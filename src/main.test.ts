import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { JSDOM } from "jsdom";
import { listSources, listSessions, monitorSession } from "./main";
import { renderSessionList } from "./session_view";

// Mock the tauri api
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("listSources", () => {
  beforeEach(() => {
    const dom = new JSDOM(`<!DOCTYPE html><html><body><ul id="sources-list"></ul></body></html>`);
    global.document = dom.window.document;
  });

  it("should fetch and display sources", async () => {
    const mockSources = [{ name: "Source 1" }, { name: "Source 2" }];
    vi.mocked(invoke).mockResolvedValue(mockSources);

    await listSources();

    const sourcesList = document.querySelector("#sources-list");
    expect(sourcesList.innerHTML).toBe("<li>Source 1</li><li>Source 2</li>");
  });

  it("should display an error message on failure", async () => {
    const errorMessage = "Failed to fetch sources";
    vi.mocked(invoke).mockRejectedValue(errorMessage);

    await listSources();

    const sourcesList = document.querySelector("#sources-list");
    expect(sourcesList.innerHTML).toBe(`<li>Error: ${errorMessage}</li>`);
  });

  it("should escape HTML in source names to prevent XSS", async () => {
    const mockSources = [{ name: "Source 1" }, { name: "<img src=x onerror=alert(1)>" }];
    vi.mocked(invoke).mockResolvedValue(mockSources);

    await listSources();

    const sourcesList = document.querySelector("#sources-list");
    // Using textContent to check for the rendered text, not the HTML structure
    const listItems = sourcesList.querySelectorAll("li");
    expect(listItems.length).toBe(2);
    expect(listItems[0].textContent).toBe("Source 1");
    expect(listItems[1].textContent).toBe("<img src=x onerror=alert(1)>");
    // Also check innerHTML to be sure it's not being rendered as an element
    expect(sourcesList.innerHTML).not.toContain("<img");
  });
});

describe("listSessions", () => {
  beforeEach(() => {
    const dom = new JSDOM(`<!DOCTYPE html><html><body><div id="sessions-list"></div></body></html>`);
    global.document = dom.window.document;
    vi.mock("./session_view", () => ({
      renderSessionList: vi.fn(),
    }));
  });

  it("should fetch and render sessions", async () => {
    const mockSessions = [{ name: "Session 1", title: "First Session", state: "COMPLETED" }];
    vi.mocked(invoke).mockResolvedValue(mockSessions);

    await listSessions();

    expect(renderSessionList).toHaveBeenCalledWith(mockSessions);
  });

  it("should display an error message on failure", async () => {
    const errorMessage = "Failed to fetch sessions";
    vi.mocked(invoke).mockRejectedValue(errorMessage);

    await listSessions();

    const sessionsList = document.querySelector("#sessions-list");
    expect(sessionsList.innerHTML).toBe(`<p>Error: ${errorMessage}</p>`);
  });
});

describe("monitorSession", () => {
  beforeEach(() => {
    const dom = new JSDOM(`
      <!DOCTYPE html>
      <html>
        <body>
          <input id="session-name-input" />
          <div id="session-status-display"></div>
        </body>
      </html>
    `);
    global.document = dom.window.document;
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("should display a message if session name is empty", async () => {
    await monitorSession();
    const display = document.querySelector("#session-status-display");
    expect(display.innerHTML).toBe("Please enter a session name.");
  });

  it("should fetch and display session status", async () => {
    const sessionNameInput = document.querySelector<HTMLInputElement>("#session-name-input");
    sessionNameInput.value = "test-session";

    const mockSession = { name: "test-session", title: "Test Session", state: "IN_PROGRESS" };
    vi.mocked(invoke).mockResolvedValue(mockSession);

    await monitorSession();

    const display = document.querySelector("#session-status-display");
    expect(display.textContent).toContain("Session: test-session");
    expect(display.textContent).toContain("Title: Test Session");
    expect(display.textContent).toContain("State: IN_PROGRESS");
  });

  it("should periodically update session status", async () => {
    const sessionNameInput = document.querySelector<HTMLInputElement>("#session-name-input");
    sessionNameInput.value = "test-session";

    const initialSession = { name: "test-session", title: "Test Session", state: "IN_PROGRESS" };
    const updatedSession = { name: "test-session", title: "Test Session", state: "COMPLETED" };
    vi.mocked(invoke).mockResolvedValueOnce(initialSession).mockResolvedValueOnce(updatedSession);

    await monitorSession();

    const display = document.querySelector("#session-status-display");
    expect(display.textContent).toContain("State: IN_PROGRESS");

    // Advance timers to trigger the interval
    await vi.advanceTimersByTimeAsync(30000);

    expect(display.textContent).toContain("State: COMPLETED");
  });

  it("should handle errors and stop monitoring", async () => {
    const sessionNameInput = document.querySelector<HTMLInputElement>("#session-name-input");
    sessionNameInput.value = "test-session";

    const errorMessage = "Session not found";
    vi.mocked(invoke).mockRejectedValue(errorMessage);

    await monitorSession();

    const display = document.querySelector("#session-status-display");
    expect(display.textContent).toContain(`Error: ${errorMessage}`);

    const setIntervalSpy = vi.spyOn(global, "setInterval");
    await monitorSession();
    expect(setIntervalSpy).not.toHaveBeenCalled();
  });
});
