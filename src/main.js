const MONITORING_INTERVAL_MS = 30000;
import { invoke } from "@tauri-apps/api/core";
import { renderSessionList } from "./session_view";
import { renderActivityList } from "./activity_view";
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
    const titleInput = document.querySelector("#title-input");
    const sourceNameInput = document.querySelector("#source-name-input");
    const startingBranchInput = document.querySelector("#starting-branch-input");
    const promptInput = document.querySelector("#prompt-input");
    const responseDisplay = document.querySelector("#response-display");
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
        const response = await invoke("create_session", {
            title,
            sourceName,
            startingBranch,
            prompt,
        });
        responseDisplay.textContent = `Session created: ${response.name}`;
    }
    catch (error) {
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
    const sourcesList = document.querySelector("#sources-list");
    if (!sourcesList)
        return;
    // Clear previous content and show loading message
    sourcesList.innerHTML = "";
    const loadingLi = document.createElement("li");
    loadingLi.textContent = "Loading...";
    sourcesList.appendChild(loadingLi);
    try {
        const sources = await invoke("list_sources");
        sourcesList.innerHTML = ""; // Clear loading message
        if (sources.length === 0) {
            const li = document.createElement("li");
            li.textContent = "No sources found.";
            sourcesList.appendChild(li);
        }
        else {
            sources.forEach((source) => {
                const li = document.createElement("li");
                li.textContent = source.name;
                sourcesList.appendChild(li);
            });
        }
    }
    catch (error) {
        sourcesList.innerHTML = ""; // Clear loading message
        const li = document.createElement("li");
        li.textContent = `Error: ${error}`;
        li.style.color = "red";
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
    const sessionsList = document.querySelector("#sessions-list");
    if (!sessionsList)
        return;
    // Clear previous content and show loading message
    sessionsList.innerHTML = "";
    const loadingP = document.createElement("p");
    loadingP.textContent = "Loading...";
    sessionsList.appendChild(loadingP);
    try {
        const sessions = await invoke("list_sessions");
        renderSessionList(sessions); // This function now handles clearing
    }
    catch (error) {
        sessionsList.innerHTML = ""; // Clear loading message
        const errorP = document.createElement("p");
        errorP.textContent = `Error: ${error}`;
        errorP.style.color = "red";
        sessionsList.appendChild(errorP);
    }
}
// Keep track of the monitoring interval
let monitoringIntervalId;
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
    const sessionNameInput = document.querySelector("#session-name-input");
    const sessionStatusDisplay = document.querySelector("#session-status-display");
    if (!sessionNameInput || !sessionStatusDisplay)
        return;
    const sessionName = sessionNameInput.value.trim();
    // Clear previous content
    sessionStatusDisplay.innerHTML = "";
    if (!sessionName) {
        const p = document.createElement("p");
        p.textContent = "Please enter a session name.";
        sessionStatusDisplay.appendChild(p);
        return;
    }
    const updateStatus = async () => {
        try {
            sessionStatusDisplay.innerHTML = ""; // Clear previous content
            const loadingP = document.createElement("p");
            loadingP.textContent = `Fetching status for ${sessionName}...`;
            sessionStatusDisplay.appendChild(loadingP);
            const session = await invoke("session_status", { sessionName });
            // Clear loading message
            sessionStatusDisplay.innerHTML = "";
            // Helper function to create <p><b>Key:</b> Value</p>
            const createDetailP = (key, value) => {
                const p = document.createElement("p");
                const b = document.createElement("b");
                b.textContent = `${key}:`;
                p.appendChild(b);
                p.append(` ${value}`);
                return p;
            };
            // Create and append elements safely
            sessionStatusDisplay.appendChild(createDetailP("Session", session.name));
            sessionStatusDisplay.appendChild(createDetailP("Title", session.title));
            sessionStatusDisplay.appendChild(createDetailP("State", session.state));
            // Fetch and render activities
            const activities = await invoke("list_activities", { sessionName });
            renderActivityList(activities);
            return true;
        }
        catch (error) {
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
    window.renderActivityList = renderActivityList;
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
