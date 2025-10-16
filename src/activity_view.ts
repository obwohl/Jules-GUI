import { Activity } from "./models";

/**
 * Renders the list of activities in a container.
 * @param activities - The list of activities to render.
 */
export function renderActivityList(activities: Activity[]) {
  const activityList = document.querySelector<HTMLDivElement>("#activity-list");

  if (!activityList) {
    return;
  }

  // Clear any existing content
  activityList.innerHTML = "";

  if (activities.length === 0) {
    const p = document.createElement("p");
    p.textContent = "No activities found for this session.";
    activityList.appendChild(p);
    return;
  }

  activities.forEach(activity => {
    const activityDiv = document.createElement("div");
    activityDiv.className = "activity-item";

    const title = document.createElement("h4");
    title.textContent = `${activity.name} - ${activity.state}`;
    activityDiv.appendChild(title);

    if (activity.toolOutput) {
      const toolOutputPre = document.createElement("pre");
      const toolOutputCode = document.createElement("code");
      toolOutputCode.textContent = `Tool: ${activity.toolOutput.toolName}\nOutput: ${activity.toolOutput.output}`;
      toolOutputPre.appendChild(toolOutputCode);
      activityDiv.appendChild(toolOutputPre);
    }

    activityList.appendChild(activityDiv);
  });
}