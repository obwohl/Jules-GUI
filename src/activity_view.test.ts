import { describe, it, expect, beforeEach } from "vitest";
import { JSDOM } from "jsdom";
import { renderActivities } from "./activity_view";
import { Activity, ToolOutput } from "./models";

describe("renderActivities", () => {
  let activityList: HTMLDivElement;

  beforeEach(() => {
    const dom = new JSDOM(`<!DOCTYPE html><html><body><div id="activity-list"></div></body></html>`);
    global.document = dom.window.document;
    activityList = document.querySelector<HTMLDivElement>("#activity-list")!;
  });

  it("should render a message when there are no activities", () => {
    renderActivities([]);
    expect(activityList.innerHTML).toBe("<p>No activities yet.</p>");
  });

  it("should render a list of activities", () => {
    const activities: Activity[] = [
      { name: "activity1", state: "COMPLETED", toolOutput: undefined },
      { name: "activity2", state: "IN_PROGRESS", toolOutput: undefined },
    ];
    renderActivities(activities);
    const activityItems =
      activityList.querySelectorAll<HTMLDivElement>(".activity-item");
    expect(activityItems.length).toBe(2);
    expect(activityItems[0].querySelector("p")?.innerHTML).toBe(
      "<b>Activity:</b> activity1 [COMPLETED]",
    );
    expect(activityItems[1].querySelector("p")?.innerHTML).toBe(
      "<b>Activity:</b> activity2 [IN_PROGRESS]",
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
    renderActivities(activities);
    const toolOutputDiv =
      activityList.querySelector<HTMLDivElement>(".tool-output");
    expect(toolOutputDiv).not.toBeNull();
    expect(toolOutputDiv?.querySelector("p")?.innerHTML).toBe(
      "<b>Tool:</b> test-tool",
    );
    expect(toolOutputDiv?.querySelector("pre")?.textContent).toBe(
      "Test tool output",
    );
  });

  it("should clear previous activities before rendering", () => {
    activityList.innerHTML = "<p>Old activity</p>";
    const activities: Activity[] = [
      { name: "activity1", state: "COMPLETED", toolOutput: undefined },
    ];
    renderActivities(activities);
    const activityItems =
      activityList.querySelectorAll<HTMLDivElement>(".activity-item");
    expect(activityItems.length).toBe(1);
    expect(activityList.querySelector("p")?.innerHTML).not.toBe(
      "Old activity",
    );
  });
});