import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { initSettingsView, showSettingsView, hideSettingsView } from "./settings_view";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core", () => ({
    invoke: vi.fn(),
}));

describe("settings_view", () => {
    beforeEach(() => {
        document.body.innerHTML = `
      <div id="settings-view" style="display: none;">
        <input id="api-key-input" type="text" />
        <button id="save-api-key"></button>
        <button id="clear-api-key"></button>
      </div>
    `;
    });

    afterEach(() => {
        vi.clearAllMocks();
    });

    it("should initialize the settings view and load the API key", async () => {
        const mockApiKey = "test-api-key";
        (invoke as vi.Mock).mockResolvedValue(mockApiKey);

        initSettingsView();

        const apiKeyInput = document.getElementById("api-key-input") as HTMLInputElement;
        await vi.waitUntil(() => apiKeyInput.value === mockApiKey);
        expect(apiKeyInput.value).toBe(mockApiKey);
    });

    it("should save the API key when the save button is clicked", async () => {
        initSettingsView();

        const apiKeyInput = document.getElementById("api-key-input") as HTMLInputElement;
        const saveButton = document.getElementById("save-api-key") as HTMLButtonElement;

        const newApiKey = "new-api-key";
        apiKeyInput.value = newApiKey;
        saveButton.click();

        expect(invoke).toHaveBeenCalledWith("set_api_key", { apiKey: newApiKey });
    });

    it("should clear the API key when the clear button is clicked", async () => {
        initSettingsView();

        const apiKeyInput = document.getElementById("api-key-input") as HTMLInputElement;
        const clearButton = document.getElementById("clear-api-key") as HTMLButtonElement;

        apiKeyInput.value = "some-api-key";
        clearButton.click();

        expect(invoke).toHaveBeenCalledWith("set_api_key", { apiKey: "" });
    });

    it("should show and hide the settings view", () => {
        initSettingsView();
        const settingsView = document.getElementById("settings-view") as HTMLElement;

        expect(settingsView.style.display).toBe("none");

        showSettingsView();
        expect(settingsView.style.display).toBe("block");

        hideSettingsView();
        expect(settingsView.style.display).toBe("none");
    });
});