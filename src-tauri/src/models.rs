use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub name: String,
    #[serde(default)]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSourcesResponse {
    pub sources: Vec<Source>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSessionsResponse {
    pub sessions: Vec<Session>,
}