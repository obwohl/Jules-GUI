# Jules API Documentation

This document provides a condensed, technical overview of the Jules API.

## Base URL

All API endpoints are relative to the following base URL:
`https://jules.googleapis.com/v1alpha`

## Authentication

All requests must include an API key in the `X-Goog-Api-Key` header.

```
X-Goog-Api-Key: YOUR_API_KEY
```

You can generate an API key from your [Jules settings page](https://jules.google.com/settings#api).

## Core Concepts

*   **Source**: Represents an input source for the agent, typically a GitHub repository.
*   **Session**: A continuous unit of work, like a chat session with the agent, initiated with a prompt and a source.
*   **Activity**: A single event or action within a session, such as a user message, an agent plan, or a progress update.

## Endpoints

### Sources

#### List Sources

*   **Endpoint**: `GET /sources`
*   **Description**: Retrieves a list of all sources (e.g., GitHub repos) connected to your account.

**Example Request:**
```bash
curl 'https://jules.googleapis.com/v1alpha/sources' \
    -H 'X-Goog-Api-Key: YOUR_API_KEY'
```

### Sessions

#### Create a Session

*   **Endpoint**: `POST /sessions`
*   **Description**: Creates a new session with a prompt and a source context.

**Example Request:**
```bash
curl 'https://jules.googleapis.com/v1alpha/sessions' \
    -X POST \
    -H "Content-Type: application/json" \
    -H 'X-Goog-Api-Key: YOUR_API_KEY' \
    -d '{
      "prompt": "Create a boba app!",
      "sourceContext": {
        "source": "sources/github/bobalover/boba",
        "githubRepoContext": {
          "startingBranch": "main"
        }
      },
      "title": "Boba App"
    }'
```

#### List Sessions

*   **Endpoint**: `GET /sessions`
*   **Description**: Retrieves a list of your sessions.

**Example Request:**
```bash
curl 'https://jules.googleapis.com/v1alpha/sessions?pageSize=5' \
    -H 'X-Goog-Api-Key: YOUR_API_KEY'
```

#### Approve a Plan

*   **Endpoint**: `POST /sessions/{sessionId}:approvePlan`
*   **Description**: Approves the latest plan for a session that requires manual approval.

**Example Request:**
```bash
curl 'https://jules.googleapis.com/v1alpha/sessions/SESSION_ID:approvePlan' \
    -X POST \
    -H "Content-Type: application/json" \
    -H 'X-Goog-Api-Key: YOUR_API_KEY'
```

### Activities

#### List Activities

*   **Endpoint**: `GET /sessions/{sessionId}/activities`
*   **Description**: Lists all activities within a specific session.

**Example Request:**
```bash
curl 'https://jules.googleapis.com/v1alpha/sessions/SESSION_ID/activities?pageSize=30' \
    -H 'X-Goog-Api-Key: YOUR_API_KEY'
```

#### Send a Message

*   **Endpoint**: `POST /sessions/{sessionId}:sendMessage`
*   **Description**: Sends a message to the agent within a session.

**Example Request:**
```bash
curl 'https://jules.googleapis.com/v1alpha/sessions/SESSION_ID:sendMessage' \
    -X POST \
    -H "Content-Type: application/json" \
    -H 'X-Goog-Api-Key: YOUR_API_KEY' \
    -d '{
      "prompt": "Can you make the app corgi themed?"
    }'
```