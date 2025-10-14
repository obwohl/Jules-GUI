// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api_client;
mod models;

use api_client::ApiClient;
use models::{ListSessionsResponse, ListSourcesResponse, Session, Source};
use tauri::State;
use std::env;

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
#[tauri::command]
async fn list_sources(state: State<'_, AppState>) -> Result<Vec<Source>, String> {
    match &state.api_client {
        Some(api_client) => get_sources(api_client).await,
        None => Err("API key is not configured. Please set the JGUI_API_KEY environment variable.".to_string()),
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
        None => Err("API key is not configured. Please set the JGUI_API_KEY environment variable.".to_string()),
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
fn initialize_api_client() -> Option<ApiClient> {
    match env::var("JGUI_API_KEY") {
        Ok(api_key) => ApiClient::new(api_key).ok(),
        Err(_) => None,
    }
}

/// The main entry point of the application.
fn main() {
    let api_client = initialize_api_client();

    tauri::Builder::default()
        .manage(AppState { api_client })
        .invoke_handler(tauri::generate_handler![list_sources, list_sessions])
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
            None => Err(
                "API key is not configured. Please set the JGUI_API_KEY environment variable."
                    .to_string(),
            ),
        };

        assert!(result_sources.is_err());
        assert_eq!(
            result_sources.unwrap_err(),
            "API key is not configured. Please set the JGUI_API_KEY environment variable."
        );

        // Mocking the behavior of list_sessions command
        let result_sessions = match &app_state.api_client {
            Some(client) => get_sessions(client).await,
            None => Err(
                "API key is not configured. Please set the JGUI_API_KEY environment variable."
                    .to_string(),
            ),
        };

        assert!(result_sessions.is_err());
        assert_eq!(
            result_sessions.unwrap_err(),
            "API key is not configured. Please set the JGUI_API_KEY environment variable."
        );
    }

    #[test]
    fn test_initialize_api_client_with_key() {
        // Set the environment variable for this test
        env::set_var("JGUI_API_KEY", "test_api_key");
        let client = initialize_api_client();
        assert!(client.is_some());
        // Clean up the environment variable
        env::remove_var("JGUI_API_KEY");
    }

    #[test]
    fn test_initialize_api_client_without_key() {
        // Ensure the environment variable is not set
        env::remove_var("JGUI_API_KEY");
        let client = initialize_api_client();
        assert!(client.is_none());
    }
}