import os
from playwright.sync_api import sync_playwright, expect

def run(playwright):
    browser = playwright.chromium.launch()
    page = browser.new_page()
    page.goto(f"file://{os.path.abspath('dist/index.html')}")

    # Enter the API key
    page.locator("#api-key-input").fill("test-api-key")

    # Click the save button
    page.locator("#save-api-key-btn").click()

    # Check for the success message
    expect(page.locator("#api-key-status")).to_have_text("API key saved successfully!")

    # Take a screenshot
    page.screenshot(path="jules-scratch/verification/verification.png")

    browser.close()

with sync_playwright() as playwright:
    run(playwright)
