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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
    #[default]
    StateUnspecified,
    Queued,
    Planning,
    AwaitingPlanApproval,
    AwaitingUserFeedback,
    InProgress,
    Paused,
    Failed,
    Completed,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// The unique name of the session, typically in the format `sessions/{session_id}`.
    pub name: String,
    /// The human-readable title of the session. Defaults to an empty string if
    /// not present in the API response.
    #[serde(default)]
    pub title: String,
    /// The state of the session.
    #[serde(default)]
    pub state: State,
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

/// Represents the request body for creating a new session.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AutomationMode {
    #[default]
    AutoCreatePr,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionRequest {
    /// The user's prompt for the new session.
    pub prompt: String,
    /// The context for the source, including the GitHub repository information.
    pub source_context: SourceContext,
    /// The automation mode for the session.
    pub automation_mode: AutomationMode,
    /// The title of the session.
    pub title: String,
}

/// Represents the source context for a new session.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceContext {
    /// The name of the source.
    pub source: String,
    /// The context for the GitHub repository.
    pub github_repo_context: GithubRepoContext,
}

/// Represents the context for a GitHub repository.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GithubRepoContext {
    /// The starting branch for the new session.
    pub starting_branch: String,
}

/// Represents a single activity in the Jules API.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    /// The unique name of the activity.
    pub name: String,
    /// The state of the activity.
    pub state: String,
    /// The content of the activity.
    #[serde(flatten)]
    pub content: Option<ActivityContent>,
}

/// Represents the content of an activity, which can be of different types.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ActivityContent {
    /// The output of a tool execution.
    ToolOutput(ToolOutput),
}

/// Represents the output of a tool execution.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ToolOutput {
    /// The name of the tool that was executed.
    pub tool_name: String,
    /// The output of the tool.
    pub output: String,
}

/// Represents the API response for a request to list activities.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListActivitiesResponse {
    /// A vector of `Activity` objects returned by the API.
    pub activities: Vec<Activity>,
    /// A token to retrieve the next page of results. If this field is empty,
    /// there are no more results.
    pub next_page_token: Option<String>,
}


/// Represents a node in the workflow graph.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// The unique identifier of the node.
    pub id: String,
    /// The type of the node (e.g., "agent", "tool").
    #[serde(rename = "type")]
    pub node_type: String,
    /// The data associated with the node, which can vary depending on the node type.
    pub data: serde_json::Value,
}

/// Represents an edge connecting two nodes in the workflow graph.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    /// The unique identifier of the edge.
    pub id: String,
    /// The identifier of the source node.
    pub source: String,
    /// The identifier of the target node.
    pub target: String,
}

/// Represents a complete workflow, consisting of nodes and edges.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    /// A vector of nodes in the workflow.
    pub nodes: Vec<Node>,
    /// A vector of edges in the workflow.
    pub edges: Vec<Edge>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_workflow_deserialization() {
        let json = r#"{
            "nodes": [
                {
                    "id": "1",
                    "type": "agent",
                    "data": { "role": "researcher" }
                }
            ],
            "edges": [
                {
                    "id": "e1-2",
                    "source": "1",
                    "target": "2"
                }
            ]
        }"#;
        let workflow: Workflow = serde_json::from_str(json).unwrap();
        assert_eq!(workflow.nodes.len(), 1);
        assert_eq!(workflow.nodes[0].id, "1");
        assert_eq!(workflow.nodes[0].node_type, "agent");
        assert_eq!(workflow.nodes[0].data, json!({ "role": "researcher" }));

        assert_eq!(workflow.edges.len(), 1);
        assert_eq!(workflow.edges[0].id, "e1-2");
        assert_eq!(workflow.edges[0].source, "1");
        assert_eq!(workflow.edges[0].target, "2");
    }

    #[test]
    fn test_activity_deserialization() {
        let json = r#"{
            "name": "activity1",
            "state": "COMPLETED",
            "toolOutput": {
                "toolName": "test-tool",
                "output": "Test tool output"
            }
        }"#;
        let activity: Activity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.name, "activity1");
        assert_eq!(activity.state, "COMPLETED");
        assert!(activity.content.is_some());
        if let Some(ActivityContent::ToolOutput(tool_output)) = activity.content {
            assert_eq!(tool_output.tool_name, "test-tool");
            assert_eq!(tool_output.output, "Test tool output");
        } else {
            panic!("Incorrect activity content type");
        }
    }

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
            state: State::InProgress,
        };
        let json_value = serde_json::to_value(&session).unwrap();
        let expected_value = serde_json::json!({
            "name": "session1",
            "title": "Session One",
            "state": "IN_PROGRESS"
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
                Session {
                    name: "session1".to_string(),
                    title: "Session One".to_string(),
                    state: State::default(),
                },
                Session {
                    name: "session2".to_string(),
                    title: "Session Two".to_string(),
                    state: State::default(),
                },
            ],
        };
        let json_value = serde_json::to_value(&response).unwrap();
        let expected_value = serde_json::json!({
            "sessions": [
                {"name": "session1", "title": "Session One", "state": "STATE_UNSPECIFIED"},
                {"name": "session2", "title": "Session Two", "state": "STATE_UNSPECIFIED"}
            ]
        });
        assert_eq!(json_value, expected_value);
    }

    #[test]
    fn test_create_session_request_serialization() {
        let request = CreateSessionRequest {
            prompt: "Test prompt".to_string(),
            source_context: SourceContext {
                source: "sources/github/test/test".to_string(),
                github_repo_context: GithubRepoContext {
                    starting_branch: "main".to_string(),
                },
            },
            automation_mode: AutomationMode::AutoCreatePr,
            title: "Test Session".to_string(),
        };
        let json_value = serde_json::to_value(&request).unwrap();
        let expected_value = serde_json::json!({
            "prompt": "Test prompt",
            "sourceContext": {
                "source": "sources/github/test/test",
                "githubRepoContext": {
                    "startingBranch": "main"
                }
            },
            "automationMode": "AUTO_CREATE_PR",
            "title": "Test Session"
        });
        assert_eq!(json_value, expected_value);
    }
}