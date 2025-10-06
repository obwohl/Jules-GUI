use serde::{Deserialize, Serialize};

/// Represents a source in the system.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    /// The name of the source.
    pub name: String,
}

/// Represents a session in the system.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    /// The name of the session.
    pub name: String,
    /// The title of the session. Defaults to an empty string if not present.
    #[serde(default)]
    pub title: String,
}

/// Represents the response for a request to list sources.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSourcesResponse {
    /// A vector of sources.
    pub sources: Vec<Source>,
}

/// Represents the response for a request to list sessions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSessionsResponse {
    /// A vector of sessions.
    pub sessions: Vec<Session>,
}