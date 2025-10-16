const MONITORING_INTERVAL_MS = 30000;

import { invoke } from "@tauri-apps/api/core";
import { renderSessionList } from "./session_view";
import { renderActivities } from "./activity_view";
import { Activity, Session, Source } from "./models";
import "./style.css";

/**
 * Handles the creation of a new session.
 *
 * This function reads the values from the input fields, calls the
 * `create_session` Tauri command, and displays the result.
 * @returns {Promise<void>} A promise that resolves when the session has been
 * created and the result displayed, or rejects if an error occurs.
 */
export async function handleCreateSession() {
  const titleInput = document.querySelector<HTMLInputElement>("#title-input");
  const sourceNameInput = document.querySelector<HTMLInputElement>("#source-name-input");
  const startingBranchInput = document.querySelector<HTMLInputElement>("#starting-branch-input");
  const promptInput = document.querySelector<HTMLTextAreaElement>("#prompt-input");
  const responseDisplay = document.querySelector<HTMLDivElement>("#response-display");

  if (!titleInput || !sourceNameInput || !startingBranchInput || !promptInput || !responseDisplay) {
    console.error("Could not find all required input or display elements.");
    return;
  }

  const title = titleInput.value;
  const sourceName = sourceNameInput.value;
  const startingBranch = startingBranchInput.value;
  const prompt = promptInput.value;

  try {
    responseDisplay.textContent = "Creating session...";
    const response: Session = await invoke("create_session", {
      title,
      sourceName,
      startingBranch,
      prompt,
    });
    responseDisplay.textContent = `Session created: ${response.name}`;
  } catch (error) {
    responseDisplay.textContent = `Error: ${error}`;
  }
}

/**
 * Fetches the list of available sources from the backend and displays them.
 *
 * This function calls the `list_sources` Tauri command, which in turn calls
 * the Jules API. The sources are then rendered as a list in the UI.
 * @returns {Promise<void>} A promise that resolves when the sources have been
 * fetched and displayed, or rejects if an error occurs.
 */
export async function listSources() {
  const sourcesList = document.querySelector<HTMLUListElement>("#sources-list")!;
  sourcesList.innerHTML = "<li>Loading...</li>";
  try {
    const sources: Source[] = await invoke("list_sources");
    sourcesList.innerHTML = ""; // Clear loading message
    sources.forEach((source) => {
      const li = document.createElement("li");
      li.textContent = source.name;
      sourcesList.appendChild(li);
    });
  } catch (error) {
    sourcesList.innerHTML = ""; // Clear loading message
    const li = document.createElement("li");
    li.textContent = `Error: ${error}`;
    sourcesList.appendChild(li);
  }
}

/**
 * Fetches the list of available sessions from the backend and displays them.
 *
 * This function calls the `list_sessions` Tauri command, which in turn calls
 * the Jules API. The sessions are then rendered as a list in the UI.
 * @returns {Promise<void>} A promise that resolves when the sessions have been
 * fetched and displayed, or rejects if an error occurs.
 */
export async function listSessions() {
  const sessionsList = document.querySelector<HTMLDivElement>("#sessions-list")!;
  sessionsList.innerHTML = "<p>Loading...</p>";
  try {
    const sessions: Session[] = await invoke("list_sessions");
    renderSessionList(sessions);
  } catch (error) {
    sessionsList.innerHTML = `<p>Error: ${error}</p>`;
  }
}

// Keep track of the monitoring timeout and state
let monitoringTimeoutId: number | undefined;
let isMonitoring = false;

/**
 * Monitors a session by periodically fetching its status.
 *
 * This function clears any existing monitoring timeout, gets the session name
 * from the input field, and then sets up a new timeout to call the
 * `session_status` Tauri command.
 */
export function monitorSession() {
  // Stop any previous monitoring
  if (monitoringTimeoutId) {
    clearTimeout(monitoringTimeoutId);
  }
  isMonitoring = false;

  const sessionNameInput =
    document.querySelector<HTMLInputElement>("#session-name-input")!;
  const sessionStatusDisplay = document.querySelector<HTMLDivElement>(
    "#session-status-display",
  )!;
  const sessionName = sessionNameInput.value.trim();

  if (!sessionName) {
    sessionStatusDisplay.innerHTML = "Please enter a session name.";
    return;
  }

  isMonitoring = true;

  const updateStatus = async () => {
    if (!isMonitoring) {
      return;
    }

    try {
      sessionStatusDisplay.innerHTML = `Fetching status for ${sessionName}...`;
      const session: Session = await invoke("session_status", { sessionName });

      // If monitoring was stopped while we were fetching, do not update the UI
      if (!isMonitoring) return;

      sessionStatusDisplay.innerHTML = "";
      const sessionP = document.createElement("p");
      const sessionB = document.createElement("b");
      sessionB.textContent = "Session:";
      sessionP.appendChild(sessionB);
      sessionP.append(` ${session.name}`);
      sessionStatusDisplay.appendChild(sessionP);

      const titleP = document.createElement("p");
      const titleB = document.createElement("b");
      titleB.textContent = "Title:";
      titleP.appendChild(titleB);
      titleP.append(` ${session.title}`);
      sessionStatusDisplay.appendChild(titleP);

      const stateP = document.createElement("p");
      const stateB = document.createElement("b");
      stateB.textContent = "State:";
      stateP.appendChild(stateB);
      stateP.append(` ${session.state}`);
      sessionStatusDisplay.appendChild(stateP);

      // If monitoring was stopped, do not fetch activities
      if (!isMonitoring) return;

      const activities: Activity[] = await invoke("list_activities", {
        sessionName,
      });
      renderActivities(activities);
    } catch (error) {
      if (!isMonitoring) return; // Don't show error if monitoring was cancelled

      sessionStatusDisplay.innerHTML = "";
      const errorP = document.createElement("p");
      errorP.style.color = "red";
      errorP.textContent = `Error: ${error}`;
      sessionStatusDisplay.appendChild(errorP);

      isMonitoring = false; // Stop monitoring on error
    } finally {
      if (isMonitoring) {
        monitoringTimeoutId = setTimeout(updateStatus, MONITORING_INTERVAL_MS);
      }
    }
  };

  // Initial call to update status immediately
  updateStatus();
}

// Add event listeners when the DOM is fully loaded.
window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#list-sources-btn")
    ?.addEventListener("click", () => listSources());
  document
    .querySelector("#list-sessions-btn")
    ?.addEventListener("click", () => listSessions());
  document
    .querySelector("#monitor-session-btn")
    ?.addEventListener("click", () => monitorSession());
  document
    .querySelector("#send-button")
    ?.addEventListener("click", handleCreateSession);
});
