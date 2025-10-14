const MONITORING_INTERVAL_MS = 30000;

import { invoke } from "@tauri-apps/api/core";
import "./style.css";

/**
 * Represents a source in the Jules API.
 * A source typically corresponds to a code repository.
 */
interface Source {
  /**
   * The unique name of the source (e.g., "sources/github/owner/repo").
   */
  name: string;
}

/**
 * Represents a session in the Jules API.
 * A session is a single conversation or task.
 */
interface Session {
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
 * Fetches the list of available sources from the backend and displays them.
 *
 * This function calls the `list_sources` Tauri command, which in turn calls
 * the Jules API. The sources are then rendered as a list in the UI.
 * @returns {Promise<void>} A promise that resolves when the sources have been
 * fetched and displayed, or rejects if an error occurs.
 */
async function listSources() {
  const sourcesList = document.querySelector<HTMLUListElement>("#sources-list")!;
  sourcesList.innerHTML = "<li>Loading...</li>";
  try {
    const sources: Source[] = await invoke("list_sources");
    sourcesList.innerHTML = sources
      .map((source) => `<li>${source.name}</li>`)
      .join("");
  } catch (error) {
    sourcesList.innerHTML = `<li>Error: ${error}</li>`;
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
async function listSessions() {
  const sessionsList = document.querySelector<HTMLUListElement>("#sessions-list")!;
  sessionsList.innerHTML = "<li>Loading...</li>";
  try {
    const sessions: Session[] = await invoke("list_sessions");
    sessionsList.innerHTML = sessions
      .map((session) => `<li><b>${session.title}</b> (${session.name})</li>`)
      .join("");
  } catch (error) {
    sessionsList.innerHTML = `<li>Error: ${error}</li>`;
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
async function monitorSession() {
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

  const updateStatus = async () => {
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
    }
  };

  // Initial call to update status immediately
  await updateStatus();

  // Set up interval to update status every 30 seconds
  monitoringIntervalId = setInterval(updateStatus, MONITORING_INTERVAL_MS);
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
});

// Set the initial HTML content of the root element.
document.querySelector<HTMLDivElement>("#root")!.innerHTML = `
  <div class="container">
    <h1>JGUI - The Unofficial GUI for Jules</h1>
    <div class="row">
      <div class="column">
        <h2>Sources</h2>
        <button id="list-sources-btn">List Sources</button>
        <ul id="sources-list"></ul>
      </div>
      <div class="column">
        <h2>Sessions</h2>
        <button id="list-sessions-btn">List Sessions</button>
        <ul id="sessions-list"></ul>
      </div>
      <div class="column">
        <h2>Session Monitoring</h2>
        <input type="text" id="session-name-input" placeholder="Enter session name" />
        <button id="monitor-session-btn">Monitor Session</button>
        <div id="session-status-display"></div>
      </div>
    </div>
  </div>
`;