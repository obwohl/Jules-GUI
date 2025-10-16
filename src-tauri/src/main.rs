// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api_client;
mod models;

use api_client::ApiClient;
use models::{
    Activity, AutomationMode, CreateSessionRequest, GithubRepoContext, ListActivitiesResponse,
    ListSessionsResponse, ListSourcesResponse, Session, Source, SourceContext,
};
use std::env;
use std::sync::Mutex;
use tauri::Manager;
use tauri::{AppHandle, Runtime, State};
use std::fs;
use std::path::PathBuf;

const NO_API_CLIENT_ERROR: &str = "API key not found. Please configure it in the settings.";
const API_KEY_FILE: &str = ".api_key";

/// The application's state, containing the API client.
///
/// This struct holds the state that is shared across the application.
/// The `api_client` is a `Mutex` to allow for safe concurrent access,
/// and it is optional, as it may not be available if the API key is not
/// configured.
pub struct AppState {
    /// The API client used to make requests to the Jules API.
    pub api_client: Mutex<Option<ApiClient>>,
}

/// The core logic for listing available sources.
///
/// This function makes a request to the Jules API to fetch the list of
/// available sources. It is separate from the `list_sources` command to
/// allow for easier testing.
///
/// # Arguments
///
/// * `api_client` - A reference to the `ApiClient` used to make the request.
///
/// # Returns
///
/// A `Result` containing a vector of `Source` objects on success, or an
/// error string on failure.
async fn get_sources(api_client: &ApiClient) -> Result<Vec<Source>, String> {
    let response = api_client
        .get::<ListSourcesResponse>("sources")
        .await?;
    Ok(response.sources)
}

/// A Tauri command that lists the available sources.
///
/// This command is exposed to the frontend and can be called from TypeScript.
/// It retrieves the `ApiClient` from the application's state and calls
/// `get_sources` to fetch the list of sources.
///
/// # Arguments
///
/// * `state` - The application's state, managed by Tauri.
///
/// # Returns
///
/// A `Result` containing a vector of `Source` objects on success, or an
/// error string on failure.
#[tauri::command]
async fn list_sources(state: State<'_, AppState>) -> Result<Vec<Source>, String> {
    let client = state.api_client.lock().unwrap().clone();
    match client {
        Some(api_client) => get_sources(&api_client).await,
        None => Err(NO_API_CLIENT_ERROR.to_string()),
    }
}

/// The core logic for fetching a single session.
///
/// This function makes a request to the Jules API to fetch a single session by name.
///
/// # Arguments
///
/// * `api_client` - A reference to the `ApiClient` used to make the request.
/// * `session_name` - The name of the session to fetch.
///
/// # Returns
///
/// A `Result` containing the `Session` object on success, or an
/// error string on failure.
async fn get_session(api_client: &ApiClient, session_name: &str) -> Result<Session, String> {
    api_client.get(&format!("sessions/{}", session_name)).await
}

/// A Tauri command that fetches a single session.
///
/// This command is exposed to the frontend and can be called from TypeScript.
///
/// # Arguments
///
/// * `state` - The application's state, managed by Tauri.
/// * `session_name` - The name of the session to fetch.
///
/// # Returns
///
/// A `Result` containing the `Session` object on success, or an
/// error string on failure.
#[tauri::command]
async fn session_status(
    state: State<'_, AppState>,
    session_name: String,
) -> Result<Session, String> {
    let client = state.api_client.lock().unwrap().clone();
    match client {
        Some(api_client) => get_session(&api_client, &session_name).await,
        None => Err(NO_API_CLIENT_ERROR.to_string()),
    }
}

/// The core logic for listing available sessions.
///
/// This function makes a request to the Jules API to fetch the list of
/// available sessions. It is separate from the `list_sessions` command to
/// allow for easier testing.
///
/// # Arguments
///
/// * `api_client` - A reference to the `ApiClient` used to make the request.
///
/// # Returns
///
/// A `Result` containing a vector of `Session` objects on success, or an
/// error string on failure.
async fn get_sessions(api_client: &ApiClient) -> Result<Vec<Session>, String> {
    let response = api_client
        .get::<ListSessionsResponse>("sessions")
        .await?;
    Ok(response.sessions)
}

/// A Tauri command that lists the available sessions.
///
/// This command is exposed to the frontend and can be called from TypeScript.
/// It retrieves the `ApiClient` from the application's state and calls
/// `get_sessions` to fetch the list of sessions.
///
/// # Arguments
///
/// * `state` - The application's state, managed by Tauri.
///
/// # Returns
///
/// A `Result` containing a vector of `Session` objects on success, or an
/// error string on failure.
#[tauri::command]
async fn list_sessions(state: State<'_, AppState>) -> Result<Vec<Session>, String> {
    let client = state.api_client.lock().unwrap().clone();
    match client {
        Some(api_client) => get_sessions(&api_client).await,
        None => Err(NO_API_CLIENT_ERROR.to_string()),
    }
}

async fn do_create_session(
    api_client: &ApiClient,
    prompt: String,
    source_name: String,
    starting_branch: String,
    title: String,
) -> Result<Session, String> {
    let request = CreateSessionRequest {
        prompt,
        source_context: SourceContext {
            source: source_name,
            github_repo_context: GithubRepoContext { starting_branch },
        },
        automation_mode: AutomationMode::default(),
        title,
    };
    api_client.post("sessions", &request).await
}

#[tauri::command]
async fn create_session(
    state: State<'_, AppState>,
    prompt: String,
    source_name: String,
    starting_branch: String,
    title: String,
) -> Result<Session, String> {
    let client = state.api_client.lock().unwrap().clone();
    match client {
        Some(api_client) => {
            do_create_session(&api_client, prompt, source_name, starting_branch, title).await
        }
        None => Err(NO_API_CLIENT_ERROR.to_string()),
    }
}

/// Returns the path to the file where the API key is stored.
///
/// This function constructs the path to the API key file, which is located
/// in the application's data directory.
///
/// # Arguments
///
/// * `app` - A handle to the Tauri application.
///
/// # Returns
///
/// A `Result` containing the `PathBuf` to the API key file, or an error
/// if the data directory cannot be determined.
fn get_api_key_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(API_KEY_FILE);
    Ok(path)
}

/// Initializes the `ApiClient` from the stored API key.
///
/// This function first attempts to load the API key from the file system.
/// If it's not found there, it falls back to checking the `JGUI_API_KEY`
/// environment variable.
///
/// # Arguments
///
/// * `app` - A handle to the Tauri application.
///
/// # Returns
///
/// An `Option` containing the `ApiClient` if the key is found, or `None`
/// otherwise.
fn initialize_api_client<R: Runtime>(app: &AppHandle<R>) -> Option<ApiClient> {
    if let Ok(path) = get_api_key_path(app) {
        if let Ok(api_key) = fs::read_to_string(path) {
            if !api_key.is_empty() {
                return ApiClient::new(api_key).ok();
            }
        }
    }

    // Fallback to environment variable
    match env::var("JGUI_API_KEY") {
        Ok(api_key) => ApiClient::new(api_key).ok(),
        Err(_) => None,
    }
}

#[tauri::command]
fn get_api_key<R: Runtime>(app: AppHandle<R>) -> Result<Option<String>, String> {
    let path = get_api_key_path(&app)?;
    if path.exists() {
        fs::read_to_string(path)
            .map(Some)
            .map_err(|e| e.to_string())
    } else {
        Ok(None)
    }
}

#[tauri::command]
fn set_api_key<R: Runtime>(
    app: AppHandle<R>,
    app_state: State<'_, AppState>,
    api_key: String,
) -> Result<(), String> {
    let path = get_api_key_path(&app)?;
    fs::write(path, &api_key).map_err(|e| e.to_string())?;

    let mut client_guard = app_state.api_client.lock().unwrap();
    *client_guard = ApiClient::new(api_key).ok();

    Ok(())
}

/// The main entry point of the application.
fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            let api_client = initialize_api_client(&app.handle());
            app.manage(AppState {
                api_client: Mutex::new(api_client),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_sources,
            list_sessions,
            create_session,
            session_status,
            list_activities,
            get_api_key,
            set_api_key
        ])
        .run(context)
        .expect("error while running tauri application");
}

/// The core logic for listing activities for a session.
///
/// # Arguments
///
/// * `api_client` - A reference to the `ApiClient` used to make the request.
/// * `session_name` - The name of the session to fetch activities for.
///
/// # Returns
///
/// A `Result` containing a vector of `Activity` objects on success, or an
/// error string on failure.
async fn get_activities(
    api_client: &ApiClient,
    session_name: &str,
) -> Result<Vec<Activity>, String> {
    let response = api_client
        .get::<ListActivitiesResponse>(&format!("sessions/{}/activities", session_name))
        .await?;
    Ok(response.activities)
}

/// A Tauri command that lists the activities for a session.
///
/// # Arguments
///
/// * `state` - The application's state, managed by Tauri.
/// * `session_name` - The name of the session to fetch activities for.
///
/// # Returns
///
/// A `Result` containing a vector of `Activity` objects on success, or an
/// error string on failure.
#[tauri::command]
async fn list_activities(
    state: State<'_, AppState>,
    session_name: String,
) -> Result<Vec<Activity>, String> {
    let client = state.api_client.lock().unwrap().clone();
    match client {
        Some(api_client) => get_activities(&api_client, &session_name).await,
        None => Err(NO_API_CLIENT_ERROR.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_client::ApiClient;
    use mockito;
    use serde_json::json;
    use tauri::test::{mock_builder, mock_context, MockRuntime};
    use tokio;

    const MOCK_API_KEY: &str = "test_api_key";

    // Helper function to create a mock ApiClient
    fn create_mock_api_client(base_url: String) -> ApiClient {
        ApiClient::new_with_base_url("test_key".to_string(), base_url)
            .expect("Failed to create test API client")
    }

    /// Sets up a mock application context.
    fn setup_test_handle() -> tauri::AppHandle<MockRuntime> {
        let app = mock_builder()
            .manage(AppState {
                api_client: Mutex::new(None),
            })
            .build(mock_context(tauri::test::noop_assets()))
            .unwrap();
        app.handle().clone()
    }

    #[test]
    fn test_set_and_get_api_key() {
        let handle = setup_test_handle();
        let app_state = handle.state::<AppState>();

        // Set a new key
        let new_key = "new_test_key";
        set_api_key(handle.clone(), app_state.clone(), new_key.to_string()).unwrap();

        // Verify the key was set in the store
        let stored_key = get_api_key(handle.clone()).unwrap();
        assert_eq!(stored_key, Some(new_key.to_string()));

        // Verify the key was updated in the app state
        let client_guard = app_state.api_client.lock().unwrap();
        assert!(client_guard.is_some());
    }

    #[test]
    fn test_initialize_api_client_from_file() {
        let handle = setup_test_handle();
        let app_state = handle.state::<AppState>();

        // Set a key to be read by initialize_api_client
        set_api_key(
            handle.clone(),
            app_state.clone(),
            MOCK_API_KEY.to_string(),
        )
        .unwrap();

        let client = initialize_api_client(&handle);
        assert!(client.is_some());
    }

    #[test]
    fn test_get_api_key_command_empty() {
        let handle = setup_test_handle();
        // Ensure the key file doesn't exist from a previous run
        if let Ok(path) = get_api_key_path(&handle) {
            let _ = fs::remove_file(path);
        }
        // Do not set a key.
        let result = get_api_key(handle.clone()).unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_get_activities_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/sessions/session1/activities")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "activities": [
                        {
                            "name": "activity1",
                            "state": "COMPLETED",
                            "toolOutput": {
                                "toolName": "test-tool",
                                "output": "Test tool output"
                            }
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let api_client = create_mock_api_client(server.url());
        let result = get_activities(&api_client, "session1").await;

        mock.assert();
        assert!(result.is_ok());
        let activities = result.unwrap();
        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0].name, "activity1");
    }

    #[tokio::test]
    async fn test_get_sources_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/sources")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "sources": [{"name": "source1"}, {"name": "source2"}]
                })
                .to_string(),
            )
            .create();

        let api_client = create_mock_api_client(server.url());
        let result = get_sources(&api_client).await;

        mock.assert();
        assert!(result.is_ok());
        let sources = result.unwrap();
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0].name, "source1");
    }

    #[tokio::test]
    async fn test_get_sources_failure() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/sources")
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let api_client = create_mock_api_client(server.url());
        let result = get_sources(&api_client).await;

        mock.assert();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_sessions_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/sessions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "sessions": [
                        {"name": "session1", "title": "Session One"},
                        {"name": "session2", "title": "Session Two"}
                    ]
                })
                .to_string(),
            )
            .create();

        let api_client = create_mock_api_client(server.url());
        let result = get_sessions(&api_client).await;

        mock.assert();
        let sessions = result.unwrap();
        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].name, "session1");
        assert_eq!
        (sessions[0].title, "Session One");
    }

    #[tokio::test]
    async fn test_get_sessions_failure() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/sessions")
            .with_status(404)
            .with_body("Not Found")
            .create();

        let api_client = create_mock_api_client(server.url());
        let result = get_sessions(&api_client).await;

        mock.assert();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_commands_with_no_api_client() {
        let handle = setup_test_handle();
        let state: State<AppState> = handle.state();

        let result_sources = list_sources(state.clone()).await;
        assert!(result_sources.is_err());
        assert_eq!(result_sources.unwrap_err(), NO_API_CLIENT_ERROR);

        let result_sessions = list_sessions(state.clone()).await;
        assert!(result_sessions.is_err());
        assert_eq!(result_sessions.unwrap_err(), NO_API_CLIENT_ERROR);

        let result_status = session_status(state.clone(), "test".to_string()).await;
        assert!(result_status.is_err());
        assert_eq!(result_status.unwrap_err(), NO_API_CLIENT_ERROR);

        let result_activities = list_activities(state.clone(), "test".to_string()).await;
        assert!(result_activities.is_err());
        assert_eq!(result_activities.unwrap_err(), NO_API_CLIENT_ERROR);
    }

    #[test]
    fn test_initialize_api_client_without_key_in_store_or_env() {
        let handle = setup_test_handle();
        // Ensure the key file doesn't exist from a previous run
        if let Ok(path) = get_api_key_path(&handle) {
            let _ = fs::remove_file(path);
        }
        env::remove_var("JGUI_API_KEY");

        let client = initialize_api_client(&handle);
        assert!(client.is_none());
    }

    #[tokio::test]
    async fn test_create_session_success() {
        const PROMPT: &str = "Create a new boba app";
        const SOURCE_NAME: &str = "sources/github/bobalover/boba";
        const STARTING_BRANCH: &str = "main";
        const TITLE: &str = "New Test Session";
        const SESSION_NAME: &str = "sessions/new-session-123";

        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/sessions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "name": SESSION_NAME,
                    "title": TITLE
                })
                .to_string(),
            )
            .match_body(mockito::Matcher::Json(json!({
                "prompt": PROMPT,
                "sourceContext": {
                    "source": SOURCE_NAME,
                    "githubRepoContext": {
                        "startingBranch": STARTING_BRANCH
                    }
                },
                "automationMode": "AUTO_CREATE_PR",
                "title": TITLE
            })))
            .create();

        let api_client = create_mock_api_client(server.url());
        let result = do_create_session(
            &api_client,
            PROMPT.to_string(),
            SOURCE_NAME.to_string(),
            STARTING_BRANCH.to_string(),
            TITLE.to_string(),
        )
        .await;

        mock.assert();
        assert!(result.is_ok());
        let session = result.unwrap();
        assert_eq!(session.name, SESSION_NAME);
        assert_eq!(session.title, TITLE);
    }

    #[tokio::test]
    async fn test_create_session_failure() {
        const PROMPT: &str = "Create a new boba app";
        const SOURCE_NAME: &str = "sources/github/bobalover/boba";
        const STARTING_BRANCH: &str = "main";
        const TITLE: &str = "New Test Session";

        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/sessions")
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let api_client = create_mock_api_client(server.url());
        let result = do_create_session(
            &api_client,
            PROMPT.to_string(),
            SOURCE_NAME.to_string(),
            STARTING_BRANCH.to_string(),
            TITLE.to_string(),
        )
        .await;

        mock.assert();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_session_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/sessions/session1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "name": "sessions/session1",
                    "title": "Session One",
                    "state": "IN_PROGRESS"
                })
                .to_string(),
            )
            .create();

        let api_client = create_mock_api_client(server.url());
        let result = get_session(&api_client, "session1").await;

        mock.assert();
        assert!(result.is_ok());
        let session = result.unwrap();
        assert_eq!(session.name, "sessions/session1");
        assert_eq!(session.title, "Session One");
        assert!(matches!(session.state, models::State::InProgress));
    }
}