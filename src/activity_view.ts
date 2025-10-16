import { Activity } from "./models";

/**
 * Renders the list of activities into the activity feed.
 *
 * This function clears the existing activity feed and then renders each
 * activity, including its name, state, and any tool output.
 *
 * @param {Activity[]} activities - The list of activities to render.
 */
export function renderActivityList(activities: Activity[]): void {
  const activityList = document.querySelector<HTMLDivElement>("#activity-list");
  if (!activityList) {
    return;
  }

  // Clear previous activities
  activityList.innerHTML = "";

  if (activities.length === 0) {
    activityList.innerHTML = "<p>No activities found for this session.</p>";
    return;
  }

  activities.forEach((activity) => {
    const activityDiv = document.createElement("div");
    activityDiv.className = "activity-item";

    const nameH4 = document.createElement("h4");
    nameH4.textContent = `${activity.name} - ${activity.state}`;
    activityDiv.appendChild(nameH4);

    if (activity.toolOutput) {
      const toolOutputDiv = document.createElement("div");
      toolOutputDiv.className = "tool-output";

      const outputPre = document.createElement("pre");
      const outputCode = document.createElement("code");
      outputCode.textContent = `Tool: ${activity.toolOutput.toolName}\nOutput: ${activity.toolOutput.output}`;
      outputPre.appendChild(outputCode);
      toolOutputDiv.appendChild(outputPre);

      activityDiv.appendChild(toolOutputDiv);
    }

    activityList.appendChild(activityDiv);
  });
}