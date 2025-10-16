from playwright.sync_api import Page, expect

def test_session_monitoring(page: Page):
    """
    This test verifies that the session monitoring feature is working correctly.
    """
    # 1. Arrange: Go to the application.
    page.goto("http://localhost:1420")

    # 2. Act: Enter a session name and click the "Monitor Session" button.
    session_name_input = page.locator("#session-name-input")
    session_name_input.fill("test-session")
    monitor_session_button = page.locator("#monitor-session-btn")
    monitor_session_button.click()

    # 3. Assert: Confirm that the session status is being displayed.
    session_status_display = page.locator("#session-status-display")
    expect(session_status_display).to_contain_text("Fetching status for test-session...")

    # 4. Screenshot: Capture the final result for visual verification.
    page.screenshot(path="jules-scratch/verification/verification.png")