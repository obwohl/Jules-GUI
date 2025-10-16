import { invoke } from "@tauri-apps/api/core";

/**
 * The HTML element that contains the settings view.
 */
let settingsView: HTMLElement | null;

/**
 * The input field for the API key.
 */
let apiKeyInput: HTMLInputElement | null;

/**
 * Initializes the settings view.
 *
 * This function finds the necessary HTML elements and sets up event listeners.
 */
export function initSettingsView() {
    settingsView = document.getElementById("settings-view");
    apiKeyInput = document.getElementById("api-key-input") as HTMLInputElement;
    const saveButton = document.getElementById("save-api-key");
    const clearButton = document.getElementById("clear-api-key");

    if (saveButton) {
        saveButton.addEventListener("click", saveApiKey);
    }

    if (clearButton) {
        clearButton.addEventListener("click", clearApiKey);
    }

    loadApiKey();
}

/**
 * Loads the API key from the backend and populates the input field.
 */
async function loadApiKey() {
    if (!apiKeyInput) return;
    try {
        const apiKey = await invoke<string>("get_api_key");
        if (apiKey) {
            apiKeyInput.value = apiKey;
        }
    } catch (error) {
        console.error("Failed to load API key:", error);
    }
}

/**
 * Saves the API key to the backend.
 *
 * This function reads the value from the API key input field and calls the
 * `set_api_key` Tauri command.
 */
/**
 * The notification element for the settings view.
 */
let notificationElement: HTMLElement | null;

/**
 * Shows a notification message in the settings view.
 *
 * @param message The message to display.
 * @param isError Whether the message is an error.
 */
function showNotification(message: string, isError = false) {
    if (!notificationElement) {
        notificationElement = document.getElementById("settings-notification");
    }
    if (notificationElement) {
        notificationElement.textContent = message;
        notificationElement.style.color = isError ? "red" : "green";
        notificationElement.style.display = "block";
        setTimeout(() => {
            notificationElement!.style.display = "none";
        }, 3000);
    }
}

async function saveApiKey() {
    if (!apiKeyInput) return;
    const apiKey = apiKeyInput.value;
    try {
        await invoke("set_api_key", { apiKey });
        showNotification("API key saved successfully!");
    } catch (error) {
        console.error("Failed to save API key:", error);
        showNotification("Failed to save API key.", true);
    }
}

/**
 * Clears the API key from the backend and the input field.
 */
async function clearApiKey() {
    if (!apiKeyInput) return;
    try {
        await invoke("set_api_key", { apiKey: "" });
        apiKeyInput.value = "";
        showNotification("API key cleared successfully!");
    } catch (error) {
        console.error("Failed to clear API key:", error);
        showNotification("Failed to clear API key.", true);
    }
}

/**
 * Shows the settings view.
 */
export function showSettingsView() {
    if (settingsView) {
        settingsView.style.display = "block";
    }
}

/**
 * Hides the settings view.
 */
export function hideSettingsView() {
    if (settingsView) {
        settingsView.style.display = "none";
    }
}