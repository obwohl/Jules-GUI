
from playwright.sync_api import sync_playwright, Page, expect

def verify_session_monitoring(page: Page):
    """
    This test verifies that the session monitoring UI is working correctly.
    """
    # 1. Arrange: Go to the application's URL.
    page.goto("http://localhost:1420")

    # 2. Act: Find the session name input, enter a session name, and click the monitor button.
    session_name_input = page.get_by_placeholder("Enter session name")
    session_name_input.fill("test-session")
    monitor_button = page.get_by_role("button", name="Monitor Session")
    monitor_button.click()

    # 3. Assert: Confirm that the status display shows the session information.
    # We'll wait for the "Fetching status..." message to appear first.
    expect(page.locator("#session-status-display")).to_contain_text("Fetching status for test-session...")

    # Then we wait for the actual status to be displayed.
    # In a real test, we would mock the backend call. For verification, we'll just wait a bit.
    page.wait_for_timeout(1000)

    # 4. Screenshot: Capture the final result for visual verification.
    page.screenshot(path="jules-scratch/verification/verification.png")

with sync_playwright() as p:
    browser = p.chromium.launch(headless=True)
    page = browser.new_page()
    verify_session_monitoring(page)
    browser.close()
