use serde::{de::DeserializeOwned, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use std::time::Duration;

const API_BASE_URL: &str = "https://jules.googleapis.com/v1alpha";
const REQUEST_TIMEOUT_SECONDS: u64 = 30;

#[derive(Clone)]
pub struct ApiClient {
    client: reqwest::Client,
    api_key: String,
}

impl ApiClient {
    pub fn new(api_key: String) -> Result<Self, String> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("X-Goog-Api-Key", HeaderValue::from_str(&api_key).map_err(|e| e.to_string())?);

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECONDS))
            .default_headers(headers)
            .user_agent(format!("jgui/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| e.to_string())?;

        Ok(ApiClient { client, api_key })
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, String> {
        let url = format!("{}/{}", API_BASE_URL, endpoint);
        let response = self.client.get(&url).send().await.map_err(|e| e.to_string())?;

        if response.status().is_success() {
            response.json::<T>().await.map_err(|e| e.to_string())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("API request failed: {}", error_text))
        }
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, endpoint: &str, body: &B) -> Result<T, String> {
        let url = format!("{}/{}", API_BASE_URL, endpoint);
        let response = self.client.post(&url).json(body).send().await.map_err(|e| e.to_string())?;

        if response.status().is_success() {
            response.json::<T>().await.map_err(|e| e.to_string())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("API request failed: {}", error_text))
        }
    }
}