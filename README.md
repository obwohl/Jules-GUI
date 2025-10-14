# JGUI: The Unofficial, Interactive GUI for Jules

`jgui` is a fast, lean, and powerful graphical user interface for the Jules API. It's designed for developers who want to interact with Jules agents directly from their desktop, providing an interactive, chat-focused experience that complements the official Jules Tools.

## Purpose

While the official `jules` CLI is excellent for orchestrating and managing remote tasks, `jgui` focuses on the conversational aspect of working with an AI agent. It allows you to have a full, real-time conversation—creating sessions, sending messages, and following the agent's activity feed—all without leaving your desktop. This makes it ideal for quick interactions, debugging, and staying in the loop with an agent's progress.

## Features

- **Session Management**: List your recent sessions with the Jules API.
- **Source Listing**: View your available code sources.
- **Cross-Platform**: Built with Tauri, `jgui` runs on Windows, macOS, and Linux.
- **Lightweight & Fast**: Built in Rust and TypeScript with minimal dependencies.

## Prerequisites

Before you begin, ensure you have the following installed:

*   [Rust](https://www.rust-lang.org/tools/install)
*   [Node.js](https://nodejs.org/)
*   [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

## Installation

1.  **Clone the repository:**
    ```bash
    git clone <repository_url>
    cd jgui
    ```

2.  **Install dependencies:**
    ```bash
    npm install
    ```

## Configuration

`jgui` needs a Jules API key to function. You can get your key from the [Jules settings page](https://jules.google.com/settings#api).

Set the `JGUI_API_KEY` environment variable. This is the most secure method and is recommended for CI/CD environments.
```bash
export JGUI_API_KEY="YOUR_JULES_API_KEY"
```
`jgui` will automatically use this key if the variable is set.

Alternatively, you can use the `jcat` CLI to set the API key in a configuration file:
```bash
python3 build/lib/jcat.py config set api_key YOUR_JULES_API_KEY
```

## Development

To run the application in development mode, use the following command:

```bash
npm run tauri dev
```

This will open the `jgui` application window. From there, you can:

- **List Sources**: Click the "List Sources" button to see your available code sources.
- **List Sessions**: Click the "List Sessions" button to see your recent sessions.

## Building for Production

To build the application for production, run:

```bash
npm run tauri build
```

The compiled application will be available in `src-tauri/target/release/`.

## Architecture

The application is built using the Tauri framework, which allows for the creation of cross-platform desktop applications using web technologies for the frontend and a Rust backend.

*   **Frontend**: The frontend is built with TypeScript and HTML/CSS. It communicates with the backend by invoking commands that are exposed by the Rust code.
*   **Backend**: The backend is written in Rust and is responsible for all communication with the Jules API. It exposes a set of commands that can be called from the frontend to fetch data from the API.
*   **API Client**: The `api_client.rs` module contains a robust client for the Jules API, which handles authentication, request signing, and response parsing.

## Project Structure

The project is organized as a standard Tauri application:

*   `src/`: Contains the frontend TypeScript, HTML, and CSS files.
*   `src-tauri/`: Contains the Rust backend code.
    *   `src/main.rs`: The main entry point for the Rust application.
    *   `src/api_client.rs`: Handles communication with the Jules API.
    *   `src/models.rs`: Defines the data structures used in the application.
*   `build/lib/jcat.py`: A standalone Python CLI for interacting with the Jules API.
*   `README.md`: This file.
*   `AGENTS.md`: Development guidelines for AI agents.

## CLI Tool (`jcat.py`)

The repository also includes a Python-based CLI tool, `jcat.py`, located in the `build/lib` directory. This tool provides a command-line interface for interacting with the Jules API, and can be used for tasks such as:

*   Listing sources and sessions
*   Creating new sessions
*   Following the activity feed of a session
*   Sending messages to a session

The `jcat.py` script is a standalone tool and is not required for the operation of the `jgui` application. It is provided as a supplementary tool for developers who prefer a command-line interface.

## Contributing

Contributions are welcome! Please follow the development guidelines outlined in `AGENTS.md`.