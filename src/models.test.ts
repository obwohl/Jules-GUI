import { describe, it, expect } from "vitest";
import { Source, Session, Activity, ToolOutput } from "./models";

describe("models", () => {
  it("should correctly define the Source interface", () => {
    const source: Source = {
      name: "sources/github/owner/repo",
    };
    expect(source.name).toBe("sources/github/owner/repo");
  });

  it("should correctly define the Session interface", () => {
    const session: Session = {
      name: "sessions/session-id",
      title: "Test Session",
      state: "IN_PROGRESS",
    };
    expect(session.name).toBe("sessions/session-id");
    expect(session.title).toBe("Test Session");
    expect(session.state).toBe("IN_PROGRESS");
  });

  it("should correctly define the Activity interface", () => {
    const activity: Activity = {
      name: "activity1",
      state: "COMPLETED",
    };
    expect(activity.name).toBe("activity1");
    expect(activity.state).toBe("COMPLETED");
  });

  it("should correctly define the Activity interface with tool output", () => {
    const toolOutput: ToolOutput = {
      toolName: "test-tool",
      output: "Test tool output",
    };
    const activity: Activity = {
      name: "activity1",
      state: "COMPLETED",
      toolOutput: toolOutput,
    };
    expect(activity.toolOutput?.toolName).toBe("test-tool");
    expect(activity.toolOutput?.output).toBe("Test tool output");
  });

  it("should correctly define the ToolOutput interface", () => {
    const toolOutput: ToolOutput = {
      toolName: "test-tool",
      output: "Test tool output",
    };
    expect(toolOutput.toolName).toBe("test-tool");
    expect(toolOutput.output).toBe("Test tool output");
  });
});