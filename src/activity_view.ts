import { Activity } from "./models";

/**
 * Renders the list of activities into the activity feed.
 *
 * This function clears the existing activity feed and then renders each
 * activity, including its name, state, and any tool output.
 *
 * @param {Activity[]} activities - The list of activities to render.
 */
export function renderActivities(activities: Activity[]): void {
  const activityList = document.querySelector<HTMLDivElement>("#activity-list");
  if (!activityList) {
    return;
  }

  // Clear previous activities
  activityList.innerHTML = "";

  if (activities.length === 0) {
    activityList.innerHTML = "<p>No activities yet.</p>";
    return;
  }

  activities.forEach((activity) => {
    const activityDiv = document.createElement("div");
    activityDiv.className = "activity-item";

    const nameP = document.createElement("p");
    nameP.innerHTML = `<b>Activity:</b> ${activity.name} [${activity.state}]`;
    activityDiv.appendChild(nameP);

    if (activity.toolOutput) {
      const toolOutputDiv = document.createElement("div");
      toolOutputDiv.className = "tool-output";

      const toolNameP = document.createElement("p");
      toolNameP.innerHTML = `<b>Tool:</b> ${activity.toolOutput.toolName}`;
      toolOutputDiv.appendChild(toolNameP);

      const outputPre = document.createElement("pre");
      outputPre.textContent = activity.toolOutput.output;
      toolOutputDiv.appendChild(outputPre);

      activityDiv.appendChild(toolOutputDiv);
    }

    activityList.appendChild(activityDiv);
  });
}