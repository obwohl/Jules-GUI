/**
 * Represents a source in the Jules API.
 * A source typically corresponds to a code repository.
 */
export interface Source {
  /**
   * The unique name of the source (e.g., "sources/github/owner/repo").
   */
  name: string;
}

/**
 * Represents a session in the Jules API.
 * A session is a single conversation or task.
 */
export interface Session {
  /**
   * The unique name of the session (e.g., "sessions/session-id").
   */
  name: string;
  /**
   * The human-readable title of the session.
   */
  title: string;
  /**
   * The state of the session.
   */
  state: string;
}

/**
 * Represents a single activity in the Jules API.
 */
export interface Activity {
  name: string;
  state: string;
  toolOutput?: ToolOutput;
}

/**
 * Represents the output of a tool execution.
 */
export interface ToolOutput {
  toolName: string;
  output: string;
}
