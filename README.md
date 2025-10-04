# Jules GUI

Jules GUI is a simple, web-based graphical user interface for the Jules API. It provides a more user-friendly way to interact with Jules than using `curl` commands in the terminal.

This project was created to provide a "non-shitty GUI" for Jules, allowing for easier management of sources, sessions, and agent interactions.

## Features

*   **API Key Management**: Securely save your Jules API key in your browser's local storage.
*   **Source Management**: List all your available GitHub repositories that are connected to Jules.
*   **Session Management**:
    *   Create new sessions with a title, prompt, source repository, and branch.
    *   View a list of your recent sessions.
*   **Interaction**:
    *   View session details and a real-time activity feed.
    *   Send messages to the agent within a session.
    *   Approve agent plans.

## How to Use

1.  **Clone the repository.**
2.  **Open `index.html` in your web browser.** Since this is a simple static application, you can open the file directly.
3.  **Enter your Jules API Key** in the "API Key" section and click "Save". You can get your key from the [Jules settings page](https://jules.google.com/settings#api).
4.  **Click "List Sources"** to see your available repositories.
5.  **Fill out the "Create Session"** form to start a new task with Jules.
6.  **Click on a session** in the "Sessions" list to view its details and interact with the agent.

## API Documentation

For detailed information about the Jules API itself, please see the [`documentation.md`](./documentation.md) file.