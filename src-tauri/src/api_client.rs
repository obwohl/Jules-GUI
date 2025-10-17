use serde::{de::DeserializeOwned, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::time::Duration;

const API_BASE_URL: &str = "https://jules.googleapis.com/v1alpha";
const REQUEST_TIMEOUT_SECONDS: u64 = 30;

/// A client for interacting with the Jules API.
///
/// This struct provides a convenient way to make authenticated requests to the
/// Jules API. It handles setting the required headers and provides methods
/// for making GET and POST requests.
#[derive(Clone)]
pub struct ApiClient {
    /// The underlying `reqwest::Client` used to make HTTP requests.
    client: reqwest::Client,
    /// The base URL of the Jules API.
    base_url: String,
}

impl ApiClient {
    /// Creates a new `ApiClient` with the default base URL.
    ///
    /// This is the standard constructor for the `ApiClient`. It uses the
    /// default API base URL and configures the client with the provided
    /// API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to use for authentication. This is a `String`
    ///   that will be sent in the `X-Goog-Api-Key` header.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `ApiClient` on success, or an error
    /// string on failure.
    pub fn new(api_key: String) -> Result<Self, String> {
        Self::new_with_base_url(api_key, API_BASE_URL.to_string())
    }

    /// Creates a new `ApiClient` with a custom base URL.
    ///
    /// This constructor is useful for testing or for targeting a different
    /// version of the API. It allows you to specify a custom base URL for
    /// all requests made by the client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to use for authentication.
    /// * `base_url` - The base URL of the API.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `ApiClient` on success, or an error
    /// string on failure.
    pub fn new_with_base_url(api_key: String, base_url: String) -> Result<Self, String> {
        Self::new_with_timeout(
            api_key,
            base_url,
            Duration::from_secs(REQUEST_TIMEOUT_SECONDS),
        )
    }

    /// Creates a new `ApiClient` with a custom timeout.
    ///
    /// This private constructor is used in tests to simulate timeouts without
    /// waiting for the full duration. It allows for setting a custom timeout
    /// for all requests made by the client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to use for authentication.
    /// * `base_url` - The base URL of the API.
    /// * `timeout` - The request timeout duration.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `ApiClient` on success, or an error
    /// string on failure.
    fn new_with_timeout(
        api_key: String,
        base_url: String,
        timeout: Duration,
    ) -> Result<Self, String> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "X-Goog-Api-Key",
            HeaderValue::from_str(&api_key).map_err(|e| e.to_string())?,
        );

        let client = reqwest::Client::builder()
            .timeout(timeout)
            .default_headers(headers)
            .user_agent(format!("jgui/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| e.to_string())?;

        Ok(ApiClient { client, base_url })
    }

    /// Sends a GET request to the specified endpoint.
    ///
    /// This method constructs the full URL from the base URL and the endpoint,
    /// and sends a GET request. It then deserializes the response into the
    /// specified type `T`.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to (e.g., "sources").
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response of type `T` on success,
    /// or an error string on failure.
    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        query: Option<&Vec<(String, String)>>,
    ) -> Result<T, String> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut request_builder = self.client.get(&url);
        if let Some(query_params) = query {
            request_builder = request_builder.query(query_params);
        }
        let response = request_builder.send().await.map_err(|e| e.to_string())?;

        if response.status().is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| format!("error decoding response body: {}", e))
        } else {
            let status = response.status();
            let error_bytes = response.bytes().await.unwrap_or_default();
            let error_text = String::from_utf8(error_bytes.to_vec())
                .unwrap_or_else(|_| "Unknown error".to_string());

            if error_text.is_empty() {
                let reason = status.canonical_reason().unwrap_or("Unknown Status");
                Err(format!(
                    "API request failed: {} {}",
                    status.as_u16(),
                    reason
                ))
            } else {
                Err(format!("API request failed: {}", error_text))
            }
        }
    }

    /// Sends a POST request to the specified endpoint.
    ///
    /// This method constructs the full URL, serializes the request body, and
    /// sends a POST request. It then deserializes the response into the
    /// specified type `T`.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to.
    /// * `body` - The body of the request, which must implement `Serialize`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response of type `T` on success,
    /// or an error string on failure. If the response is `204 No Content`,
    /// it returns `T::default()`.
    pub async fn post<T: DeserializeOwned + Default, B: Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T, String> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(body).send().await.map_err(|e| e.to_string())?;
        let status = response.status();

        if status.is_success() {
            if status == reqwest::StatusCode::NO_CONTENT {
                Ok(T::default())
            } else {
                response
                    .json::<T>()
                    .await
                    .map_err(|e| format!("error decoding response body: {}", e))
            }
        } else {
            let error_bytes = response.bytes().await.unwrap_or_default();
            let error_text = String::from_utf8(error_bytes.to_vec())
                .unwrap_or_else(|_| "Unknown error".to_string());

            if error_text.is_empty() {
                let reason = status.canonical_reason().unwrap_or("Unknown Status");
                Err(format!(
                    "API request failed: {} {}",
                    status.as_u16(),
                    reason
                ))
            } else {
                Err(format!("API request failed: {}", error_text))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ListSourcesResponse, Source};
    use mockito;
    use serde_json::json;
    use std::time::Duration;
    use tokio;

    #[tokio::test]
    async fn test_new_api_client_success() {
        let api_key = "test_api_key".to_string();
        let client = ApiClient::new(api_key).unwrap();
        assert_eq!(client.base_url, API_BASE_URL);
    }

    #[tokio::test]
    async fn test_new_api_client_headers() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .match_header("X-Goog-Api-Key", "test_api_key")
            .match_header("User-Agent", &*format!("jgui/{}", env!("CARGO_PKG_VERSION")))
            .create();

        let client = ApiClient::new_with_base_url("test_api_key".to_string(), server.url()).unwrap();
        let _ = client.get::<serde_json::Value>("test", None).await;

        mock.assert();
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
        let result = api_client
            .get::<ListSourcesResponse>("sources", None)
            .await;

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
        let result = api_client
            .get::<ListSourcesResponse>("sources", None)
            .await;

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

    #[tokio::test]
    async fn test_get_request_timeout() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/slow-endpoint")
            .with_chunked_body(|w| {
                // Simulate a delay by sleeping before writing the response
                std::thread::sleep(Duration::from_millis(150));
                let _ = w.write_all(b"should not reach here");
                Ok(())
            })
            .create();

        let api_client = ApiClient::new_with_timeout(
            "test_key".to_string(),
            server.url(),
            Duration::from_millis(50), // Set a short timeout
        )
        .unwrap();

        let result = api_client
            .get::<serde_json::Value>("slow-endpoint", None)
            .await;

        mock.assert();
        assert!(result.is_err());
        // The error message should indicate a timeout
        assert!(result.unwrap_err().contains("timed out"));
    }

    #[tokio::test]
    async fn test_get_request_malformed_json() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/malformed")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("this is not valid json")
            .create();

        let api_client =
            ApiClient::new_with_base_url("test_key".to_string(), server.url()).unwrap();
        let result = api_client
            .get::<ListSourcesResponse>("malformed", None)
            .await;

        mock.assert();
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("error decoding response body"));
    }

    #[tokio::test]
    async fn test_get_request_failure_empty_body() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("GET", "/sources").with_status(500).create();

        let api_client =
            ApiClient::new_with_base_url("test_key".to_string(), server.url()).unwrap();
        let result = api_client
            .get::<ListSourcesResponse>("sources", None)
            .await;

        mock.assert();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "API request failed: 500 Internal Server Error"
        );
    }

    #[tokio::test]
    async fn test_post_request_failure_empty_body() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("POST", "/sessions").with_status(400).create();

        let api_client =
            ApiClient::new_with_base_url("test_key".to_string(), server.url()).unwrap();
        let new_session_data = json!({"title": "New Session"});
        let result = api_client
            .post::<Source, _>("sessions", &new_session_data)
            .await;

        mock.assert();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "API request failed: 400 Bad Request"
        );
    }

    #[tokio::test]
    async fn test_get_request_failure_invalid_utf8() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/invalid-utf8")
            .with_status(500)
            .with_body(vec![0, 159, 146, 150]) // Invalid UTF-8 sequence
            .create();

        let api_client =
            ApiClient::new_with_base_url("test_key".to_string(), server.url()).unwrap();
        let result = api_client
            .get::<serde_json::Value>("invalid-utf8", None)
            .await;

        mock.assert();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "API request failed: Unknown error");
    }

    #[tokio::test]
    async fn test_post_request_success_no_content() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/no-content")
            .with_status(204)
            .create();

        let api_client =
            ApiClient::new_with_base_url("test_key".to_string(), server.url()).unwrap();

        let result = api_client
            .post::<(), _>("no-content", &serde_json::Value::Null)
            .await;

        mock.assert();
        assert!(result.is_ok());
    }
}