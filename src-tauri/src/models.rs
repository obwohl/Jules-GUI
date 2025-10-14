use serde::{Deserialize, Serialize};

/// Represents a source in the Jules API.
///
/// A source typically corresponds to a code repository or another context
/// that an agent can operate on.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Source {
    /// The unique name of the source, typically in the format `sources/{source_id}`.
    pub name: String,
}

/// Represents a session in the Jules API.
///
/// A session is a single conversation or task that an agent is working on.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Session {
    /// The unique name of the session, typically in the format `sessions/{session_id}`.
    pub name: String,
    /// The human-readable title of the session. Defaults to an empty string if
    /// not present in the API response.
    #[serde(default)]
    pub title: String,
}

/// Represents the API response for a request to list sources.
///
/// This struct is used to deserialize the JSON response from the `sources`
/// endpoint of the Jules API.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ListSourcesResponse {
    /// A vector of `Source` objects returned by the API.
    pub sources: Vec<Source>,
}

/// Represents the API response for a request to list sessions.
///
/// This struct is used to deserialize the JSON response from the `sessions`
/// endpoint of the Jules API.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ListSessionsResponse {
    /// A vector of `Session` objects returned by the API.
    pub sessions: Vec<Session>,
}