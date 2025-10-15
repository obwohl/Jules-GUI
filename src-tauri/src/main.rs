// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api_client;
mod models;

use api_client::ApiClient;
use models::{
    AutomationMode, CreateSessionRequest, GithubRepoContext, ListSessionsResponse,
    ListSourcesResponse, SendPromptRequest, SendPromptResponse, Session, Source, SourceContext,
};
use tauri::{Manager, State};
use tauri_plugin_store::StoreBuilder;
use std::env;

const NO_API_CLIENT_ERROR: &str =
    "API key is not configured. Please set the JGUI_API_KEY environment variable.";

/// The application's state, containing the API client.
///
/// This struct holds the state that is shared across the application.
/// The `api_client` is optional, as it may not be available if the
/// API key is not configured.
struct AppState {
    /// The API client used to make requests to the Jules API.
    api_client: Option<ApiClient>,
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
/// The core logic for sending a prompt to the AI.
///
/// This function makes a POST request to the `/chat` endpoint of the Jules API.
/// It is separate from the `send_prompt` command to allow for easier testing.
///
/// # Arguments
///
/// * `api_client` - A reference to the `ApiClient` used to make the request.
/// * `prompt` - The prompt to send to the AI.
///
/// # Returns
///
/// A `Result` containing the AI's response string on success, or an
/// error string on failure.
async fn do_send_prompt(api_client: &ApiClient, prompt: String) -> Result<String, String> {
    let request = SendPromptRequest { prompt };
    let response = api_client
        .post::<SendPromptResponse, _>("chat", &request)
        .await?;
    Ok(response.response)
}

/// A Tauri command that sends a prompt to the AI.
///
/// This command is exposed to the frontend and can be called from TypeScript.
/// It retrieves the `ApiClient` from the application's state and calls
/// `do_send_prompt` to send the prompt.
///
/// # Arguments
///
/// * `state` - The application's state, managed by Tauri.
/// * `prompt` - The prompt to send to the AI.
///
/// # Returns
///
/// A `Result` containing the AI's response string on success, or an
/// error string on failure.
#[tauri::command]
async fn send_prompt(state: State<'_, AppState>, prompt: String) -> Result<String, String> {
    match &state.api_client {
        Some(api_client) => do_send_prompt(api_client, prompt).await,
        None => Err(NO_API_CLIENT_ERROR.to_string()),
    }
}

#[tauri::command]
async fn list_sources(state: State<'_, AppState>) -> Result<Vec<Source>, String> {
    match &state.api_client {
        Some(api_client) => get_sources(api_client).await,
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
    match &state.api_client {
        Some(api_client) => get_session(api_client, &session_name).await,
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
    match &state.api_client {
        Some(api_client) => get_sessions(api_client).await,
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
    match &state.api_client {
        Some(api_client) => {
            do_create_session(
                api_client,
                prompt,
                source_name,
                starting_branch,
                title,
            )
            .await
        }
        None => Err(NO_API_CLIENT_ERROR.to_string()),
    }
}

/// Initializes the `ApiClient` based on the `JGUI_API_KEY` environment variable.
///
/// This function checks for the `JGUI_API_KEY` environment variable and, if
/// it is present, creates a new `ApiClient` with the value of the variable.
///
/// # Returns
///
/// An `Option` containing the `ApiClient` if the environment variable is set,
/// or `None` if it is not.
fn initialize_api_client(app_handle: &tauri::AppHandle) -> Option<ApiClient> {
    create_api_client(get_api_key(app_handle))
}

#[cfg(not(test))]
fn get_api_key(app_handle: &tauri::AppHandle) -> Option<String> {
    let store_result = StoreBuilder::new(app_handle, ".settings.dat").build();
    if let Ok(store) = store_result {
        if store.reload().is_ok() {
            if let Some(key) = store.get("apiKey").and_then(|v| v.as_str().map(|s| s.to_owned())) {
                return Some(key);
            }
        }
    }

    env::var("JGUI_API_KEY").ok()
}

#[cfg(test)]
fn get_api_key(_app_handle: &tauri::AppHandle) -> Option<String> {
    env::var("JGUI_API_KEY").ok()
}

fn create_api_client(api_key: Option<String>) -> Option<ApiClient> {
    api_key.and_then(|key| ApiClient::new(key).ok())
}

#[tauri::command]
async fn save_api_key(app_handle: tauri::AppHandle, key: String) -> Result<(), String> {
    let store = StoreBuilder::new(&app_handle, ".settings.dat")
        .build()
        .map_err(|e| e.to_string())?;
    store.set("apiKey".to_string(), serde_json::Value::String(key));
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}


/// The main entry point of the application.
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let api_client = initialize_api_client(&app.handle());
            app.manage(AppState { api_client });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_prompt,
            list_sources,
            list_sessions,
            create_session,
            session_status,
            save_api_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_client::ApiClient;
    use mockito;
    use serde_json::json;
    use tokio;

    // Helper function to create a mock ApiClient
    fn create_mock_api_client(base_url: String) -> ApiClient {
        ApiClient::new_with_base_url("test_key".to_string(), base_url)
            .expect("Failed to create test API client")
    }

    #[tokio::test]
    async fn test_do_send_prompt_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/chat")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"response": "Hello from the mock AI!"}).to_string())
            .match_body(mockito::Matcher::Json(json!({
                "prompt": "Hello"
            })))
            .create();

        let api_client = create_mock_api_client(server.url());
        let result = do_send_prompt(&api_client, "Hello".to_string()).await;

        mock.assert();
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response, "Hello from the mock AI!");
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
        assert_eq!(
            result.unwrap_err(),
            "API request failed: Internal Server Error"
        );
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
        assert!(result.is_ok());
        let sessions = result.unwrap();
        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].name, "session1");
        assert_eq!(sessions[0].title, "Session One");
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
        assert_eq!(result.unwrap_err(), "API request failed: Not Found");
    }

    #[tokio::test]
    async fn test_commands_with_no_api_client() {
        // This test simulates the scenario where the AppState does not have an ApiClient.
        // While we cannot create a `tauri::State` directly, we can test the logic
        // by creating an AppState and passing it to a mock context.
        // Since the commands now have a simple match statement, this test focuses on
        // the logic that would be executed in a real application.

        let app_state = AppState { api_client: None };

        // Mocking the behavior of list_sources command
        let result_sources = match &app_state.api_client {
            Some(client) => get_sources(client).await,
            None => Err(NO_API_CLIENT_ERROR.to_string()),
        };

        assert!(result_sources.is_err());
        assert_eq!(result_sources.unwrap_err(), NO_API_CLIENT_ERROR);

        // Mocking the behavior of list_sessions command
        let result_sessions = match &app_state.api_client {
            Some(client) => get_sessions(client).await,
            None => Err(NO_API_CLIENT_ERROR.to_string()),
        };

        assert!(result_sessions.is_err());
        assert_eq!(result_sessions.unwrap_err(), NO_API_CLIENT_ERROR);
    }

    #[test]
    fn test_create_api_client_with_key() {
        let client = create_api_client(Some("test_api_key".to_string()));
        assert!(client.is_some());
    }

    #[test]
    fn test_create_api_client_without_key() {
        let client = create_api_client(None);
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