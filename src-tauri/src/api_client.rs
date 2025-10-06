use serde::{de::DeserializeOwned, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::time::Duration;

const API_BASE_URL: &str = "https://jules.googleapis.com/v1alpha";
const REQUEST_TIMEOUT_SECONDS: u64 = 30;

/// A client for interacting with the Jules API.
#[derive(Clone)]
pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    /// Creates a new `ApiClient` with the default base URL.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to use for authentication.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `ApiClient` or an error string.
    pub fn new(api_key: String) -> Result<Self, String> {
        Self::new_with_base_url(api_key, API_BASE_URL.to_string())
    }

    /// Creates a new `ApiClient` with a custom base URL.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to use for authentication.
    /// * `base_url` - The base URL of the API.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `ApiClient` or an error string.
    pub fn new_with_base_url(api_key: String, base_url: String) -> Result<Self, String> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("X-Goog-Api-Key", HeaderValue::from_str(&api_key).map_err(|e| e.to_string())?);

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECONDS))
            .default_headers(headers)
            .user_agent(format!("jgui/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| e.to_string())?;

        Ok(ApiClient {
            client,
            base_url,
        })
    }

    /// Sends a GET request to the specified endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response or an error string.
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, String> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await.map_err(|e| e.to_string())?;

        if response.status().is_success() {
            response.json::<T>().await.map_err(|e| e.to_string())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("API request failed: {}", error_text))
        }
    }

    /// Sends a POST request to the specified endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to.
    /// * `body` - The body of the request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response or an error string.
    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, endpoint: &str, body: &B) -> Result<T, String> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(body).send().await.map_err(|e| e.to_string())?;

        if response.status().is_success() {
            response.json::<T>().await.map_err(|e| e.to_string())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("API request failed: {}", error_text))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ListSourcesResponse, Source};
    use mockito;
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn test_new_api_client_success() {
        let api_key = "test_api_key".to_string();
        let client = ApiClient::new(api_key).unwrap();
        assert_eq!(client.base_url, API_BASE_URL);
    }

    #[tokio::test]
    async fn test_get_request_success() {
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

        let api_client =
            ApiClient::new_with_base_url("test_key".to_string(), server.url()).unwrap();
        let result = api_client.get::<ListSourcesResponse>("sources").await;

        mock.assert();
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.sources.len(), 2);
        assert_eq!(response.sources[0].name, "source1");
    }

    #[tokio::test]
    async fn test_get_request_failure() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/sources")
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let api_client =
            ApiClient::new_with_base_url("test_key".to_string(), server.url()).unwrap();
        let result = api_client.get::<ListSourcesResponse>("sources").await;

        mock.assert();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "API request failed: Internal Server Error"
        );
    }

    #[tokio::test]
    async fn test_post_request_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/sessions")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(json!({"name": "new_session", "title": "New Session"}).to_string())
            .create();

        let api_client =
            ApiClient::new_with_base_url("test_key".to_string(), server.url()).unwrap();
        let new_session_data = json!({"title": "New Session"});
        let result = api_client
            .post::<Source, _>("sessions", &new_session_data)
            .await;

        mock.assert();
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "new_session");
    }

    #[tokio::test]
    async fn test_post_request_failure() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/sessions")
            .with_status(400)
            .with_body("Bad Request")
            .create();

        let api_client =
            ApiClient::new_with_base_url("test_key".to_string(), server.url()).unwrap();
        let new_session_data = json!({"title": "New Session"});
        let result = api_client
            .post::<Source, _>("sessions", &new_session_data)
            .await;

        mock.assert();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "API request failed: Bad Request");
    }
}