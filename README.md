# jcat: The Unofficial, Interactive CLI for Jules

`jcat` is a fast, lean, and powerful command-line interface for the Jules API. It's designed to provide the interactive, chat-focused experience that is missing from the official Jules Tools.

While the official tool can manage remote tasks, `jcat` allows you to have a full conversation with the Jules agent—creating sessions, sending messages, and following the activity feed in real-time—all without leaving your terminal.

## Features

- **Full Session Control**: Create, list, and manage sessions.
- **Real-time Activity Streaming**: Follow along with the agent's work as it happens using `jcat session follow`.
- **Direct Messaging**: Chat with the Jules agent directly from your command line.
- **Simple Configuration**: A one-time setup for your API key.
- **Lightweight & Fast**: Built in Python with minimal dependencies.

## Installation & Setup

1.  **Clone the repository:**
    ```bash
    git clone <repository_url>
    cd jules-gui
    ```

2.  **Install dependencies:**
    ```bash
    pip install -r requirements.txt
    ```

3.  **Configure your API key:**
    You only need to do this once. Get your key from the [Jules settings page](https://jules.google.com/settings#api).
    ```bash
    python jcat.py config set api_key YOUR_JULES_API_KEY
    ```

## Usage

### List available sources
```bash
python jcat.py sources list
```

### Create a new session
```bash
python jcat.py session new "My awesome new feature" --source "sources/github/your-org/your-repo"
```

### List recent sessions
```bash
python jcat.py session list
```

### Follow a session's activity feed
This is the killer feature. It streams all messages, plans, and progress updates to your terminal.
```bash
python jcat.py session follow <session_id>
```

### Send a message to a session
```bash
python jcat.py session message <session_id> "Can you also add a unit test for that?"
```
