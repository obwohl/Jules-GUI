const MONITORING_INTERVAL_MS = 30000;

import { invoke } from "@tauri-apps/api/core";
import { renderSessionList } from "./session_view";
import { Session, Source } from "./models";
import "./style.css";

export async function handleSendPrompt() {
  const promptInput = document.querySelector<HTMLTextAreaElement>("#prompt-input");
  const responseDisplay = document.querySelector<HTMLDivElement>("#response-display");

  if (!promptInput || !responseDisplay) {
    console.error("Could not find prompt input or response display elements.");
    return;
  }

  const prompt = promptInput.value;
  const response = await invoke("send_prompt", { prompt });
  responseDisplay.textContent = response as string;
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

// Keep track of the monitoring interval
let monitoringIntervalId: number | undefined;

/**
 * Monitors a session by periodically fetching its status.
 *
 * This function clears any existing monitoring interval, gets the session name
 * from the input field, and then sets up a new interval to call the
 * `session_status` Tauri command every 30 seconds.
 */
export async function monitorSession() {
  // Clear any existing interval
  if (monitoringIntervalId) {
    clearInterval(monitoringIntervalId);
  }

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

  const updateStatus = async (): Promise<boolean> => {
    try {
      sessionStatusDisplay.innerHTML = `Fetching status for ${sessionName}...`;
      const session: Session = await invoke("session_status", { sessionName });

      // Clear previous content
      sessionStatusDisplay.innerHTML = "";

      // Create and append elements safely
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
      return true;
    } catch (error) {
      // Clear previous content and display error safely
      sessionStatusDisplay.innerHTML = "";
      const errorP = document.createElement("p");
      errorP.style.color = "red";
      errorP.textContent = `Error: ${error}`;
      sessionStatusDisplay.appendChild(errorP);

      // Stop monitoring on error
      if (monitoringIntervalId) {
        clearInterval(monitoringIntervalId);
      }
      return false;
    }
  };

  // Initial call to update status immediately
  const success = await updateStatus();

  // Set up interval to update status every 30 seconds
  if (success) {
    monitoringIntervalId = setInterval(updateStatus, MONITORING_INTERVAL_MS);
  }
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
    ?.addEventListener("click", handleSendPrompt);
});
