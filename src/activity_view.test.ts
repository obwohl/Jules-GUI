import { describe, it, expect, beforeEach } from "vitest";
import { JSDOM } from "jsdom";
import { renderActivityList } from "./activity_view";
import { Activity, ToolOutput } from "./models";

describe("renderActivityList", () => {
  let activityList: HTMLDivElement;

  beforeEach(() => {
    const dom = new JSDOM(`<!DOCTYPE html><html><body><div id="activity-list"></div></body></html>`);
    global.document = dom.window.document;
    activityList = document.querySelector<HTMLDivElement>("#activity-list")!;
  });

  it("should render a message when there are no activities", () => {
    renderActivityList([]);
    expect(activityList.innerHTML).toBe("<p>No activities found for this session.</p>");
  });

  it("should render a list of activities", () => {
    const activities: Activity[] = [
      { name: "activity1", state: "COMPLETED", toolOutput: undefined },
      { name: "activity2", state: "IN_PROGRESS", toolOutput: undefined },
    ];
    renderActivityList(activities);
    const activityItems =
      activityList.querySelectorAll<HTMLDivElement>(".activity-item");
    expect(activityItems.length).toBe(2);
    expect(activityItems[0].querySelector("h4")?.textContent).toBe(
      "activity1 - COMPLETED",
    );
    expect(activityItems[1].querySelector("h4")?.textContent).toBe(
      "activity2 - IN_PROGRESS",
    );
  });

  it("should render tool output if it exists", () => {
    const toolOutput: ToolOutput = {
      toolName: "test-tool",
      output: "Test tool output",
    };
    const activities: Activity[] = [
      {
        name: "activity1",
        state: "COMPLETED",
        toolOutput: toolOutput,
      },
    ];
    renderActivityList(activities);
    const toolOutputPre =
      activityList.querySelector<HTMLPreElement>("pre");
    expect(toolOutputPre).not.toBeNull();
    expect(toolOutputPre?.querySelector("code")?.textContent).toBe(
      "Tool: test-tool\nOutput: Test tool output",
    );
  });

  it("should clear previous activities before rendering", () => {
    activityList.innerHTML = "<p>Old activity</p>";
    const activities: Activity[] = [
      { name: "activity1", state: "COMPLETED", toolOutput: undefined },
    ];
    renderActivityList(activities);
    const activityItems =
      activityList.querySelectorAll<HTMLDivElement>(".activity-item");
    expect(activityItems.length).toBe(1);
    expect(activityList.querySelector("p")?.innerHTML).not.toBe(
      "Old activity",
    );
  });
});