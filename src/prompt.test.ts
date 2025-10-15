import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { handleSendPrompt } from "./main";

// Mock the tauri api
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("Prompt Sending", () => {
  beforeEach(() => {
    // Set up the DOM elements needed for the test
    document.body.innerHTML = `
      <div id="root"></div>
      <textarea id="prompt-input"></textarea>
      <div id="response-display"></div>
    `;
  });

  it("should call invoke with the content of the prompt input", async () => {
    const promptInput = document.querySelector<HTMLTextAreaElement>("#prompt-input")!;
    promptInput.value = "This is a test prompt";

    // This will fail because handleSendPrompt is not defined yet
    await handleSendPrompt();

    expect(invoke).toHaveBeenCalledWith("send_prompt", {
      prompt: "This is a test prompt"
    });
  });
});
