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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_deserialization_with_title() {
        let json = r#"{"name": "session1", "title": "Session One"}"#;
        let session: Session = serde_json::from_str(json).unwrap();
        assert_eq!(session.name, "session1");
        assert_eq!(session.title, "Session One");
    }

    #[test]
    fn test_session_deserialization_without_title() {
        let json = r#"{"name": "session2"}"#;
        let session: Session = serde_json::from_str(json).unwrap();
        assert_eq!(session.name, "session2");
        assert_eq!(session.title, ""); // Should default to empty string
    }

    #[test]
    fn test_source_serialization() {
        let source = Source {
            name: "source1".to_string(),
        };
        let json_value = serde_json::to_value(&source).unwrap();
        let expected_value = serde_json::json!({
            "name": "source1"
        });
        assert_eq!(json_value, expected_value);
    }

    #[test]
    fn test_session_serialization() {
        let session = Session {
            name: "session1".to_string(),
            title: "Session One".to_string(),
        };
        let json_value = serde_json::to_value(&session).unwrap();
        let expected_value = serde_json::json!({
            "name": "session1",
            "title": "Session One"
        });
        assert_eq!(json_value, expected_value);
    }

    #[test]
    fn test_list_sources_response_serialization() {
        let response = ListSourcesResponse {
            sources: vec![
                Source { name: "source1".to_string() },
                Source { name: "source2".to_string() },
            ],
        };
        let json_value = serde_json::to_value(&response).unwrap();
        let expected_value = serde_json::json!({
            "sources": [
                {"name": "source1"},
                {"name": "source2"}
            ]
        });
        assert_eq!(json_value, expected_value);
    }

    #[test]
    fn test_list_sessions_response_serialization() {
        let response = ListSessionsResponse {
            sessions: vec![
                Session { name: "session1".to_string(), title: "Session One".to_string() },
                Session { name: "session2".to_string(), title: "Session Two".to_string() },
            ],
        };
        let json_value = serde_json::to_value(&response).unwrap();
        let expected_value = serde_json::json!({
            "sessions": [
                {"name": "session1", "title": "Session One"},
                {"name": "session2", "title": "Session Two"}
            ]
        });
        assert_eq!(json_value, expected_value);
    }
}