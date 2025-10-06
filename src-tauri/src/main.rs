// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api_client;
mod models;

use api_client::ApiClient;
use models::{ListSessionsResponse, ListSourcesResponse, Session, Source};
use tauri::State;
use std::env;

/// The application's state, containing the API client.
struct AppState {
    api_client: ApiClient,
}

/// Lists the available sources.
///
/// # Arguments
///
/// * `state` - The application's state.
///
/// # Returns
///
/// A `Result` containing a vector of sources or an error string.
#[tauri::command]
async fn list_sources(state: State<'_, AppState>) -> Result<Vec<Source>, String> {
    let response = state
        .api_client
        .get::<ListSourcesResponse>("sources")
        .await?;
    Ok(response.sources)
}

/// Lists the available sessions.
///
/// # Arguments
///
/// * `state` - The application's state.
///
/// # Returns
///
/// A `Result` containing a vector of sessions or an error string.
#[tauri::command]
async fn list_sessions(state: State<'_, AppState>) -> Result<Vec<Session>, String> {
    let response = state
        .api_client
        .get::<ListSessionsResponse>("sessions")
        .await?;
    Ok(response.sessions)
}

/// The main entry point of the application.
fn main() {
    let api_key = env::var("JGUI_API_KEY").expect("JGUI_API_KEY environment variable not set");
    let api_client = ApiClient::new(api_key).expect("Failed to create API client");

    tauri::Builder::default()
        .manage(AppState { api_client })
        // .plugin(tauri_plugin_window::init())
        // .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![list_sources, list_sessions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}