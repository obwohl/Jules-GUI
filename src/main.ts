import { invoke } from "@tauri-apps/api/tauri";
import "./style.css";

/**
 * Represents a source in the system.
 */
interface Source {
  /**
   * The name of the source.
   */
  name: string;
}

/**
 * Represents a session in the system.
 */
interface Session {
  /**
   * The name of the session.
   */
  name: string;
  /**
   * The title of the session.
   */
  title: string;
}

/**
 * Fetches and displays the list of available sources.
 * @returns {Promise<void>} A promise that resolves when the sources are listed.
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
 * Fetches and displays the list of available sessions.
 * @returns {Promise<void>} A promise that resolves when the sessions are listed.
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

// Add event listeners when the DOM is fully loaded.
window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#list-sources-btn")
    ?.addEventListener("click", () => listSources());
  document
    .querySelector("#list-sessions-btn")
    ?.addEventListener("click", () => listSessions());
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
    </div>
  </div>
`;