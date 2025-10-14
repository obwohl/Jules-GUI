"""A command-line interface for interacting with the Jules API.

This module provides a CLI for managing sources, sessions, and activities
with the Jules API. It supports interactive features like real-time activity
streaming and direct messaging.
"""
import argparse
import os
import json
import requests
import time
import questionary

CONFIG_FILE = os.path.expanduser("~/.jcat_config.json")
API_BASE_URL = "https://jules.googleapis.com/v1alpha"
REQUEST_TIMEOUT_SECONDS = 30

# --- Configuration Management ---

def load_config():
    """Loads the configuration from the config file.

    Returns:
        dict: The configuration dictionary, or an empty dictionary if the
              config file does not exist.
    """
    if not os.path.exists(CONFIG_FILE):
        return {}
    with open(CONFIG_FILE, 'r') as f:
        return json.load(f)

def save_config(config):
    """Saves the configuration to the config file.

    Args:
        config (dict): The configuration dictionary to save.
    """
    with open(CONFIG_FILE, 'w') as f:
        json.dump(config, f, indent=4)
    print(f"Configuration saved to {CONFIG_FILE}")

# --- API Client ---

class ApiClient:
    """A simple client for the Jules API.

    This client handles authentication and provides methods for making GET and
    POST requests to the API.

    Attributes:
        api_key (str): The API key for authenticating with the Jules API.
        headers (dict): The request headers, including the content type and
                        API key.
    """
    def __init__(self, api_key):
        """Initializes the ApiClient.

        Args:
            api_key (str): The API key for the Jules API.

        Raises:
            ValueError: If the API key is not provided.
        """
        if not api_key:
            raise ValueError("API key is missing. Please set it via the JCAT_API_KEY environment variable or by using 'jcat config set api_key YOUR_KEY'")
        self.api_key = api_key
        self.headers = {
            "Content-Type": "application/json",
            "X-Goog-Api-Key": self.api_key,
        }

    def get(self, endpoint):
        """Makes a GET request to the API.

        Args:
            endpoint (str): The API endpoint to call.

        Returns:
            dict: The JSON response from the API.
        """
        response = requests.get(
            f"{API_BASE_URL}/{endpoint}",
            headers=self.headers,
            timeout=REQUEST_TIMEOUT_SECONDS
        )
        response.raise_for_status()
        return response.json()

    def post(self, endpoint, data=None):
        """Makes a POST request to the API.

        Args:
            endpoint (str): The API endpoint to call.
            data (dict, optional): The JSON data to send in the request body.
                                   Defaults to None.

        Returns:
            dict or None: The JSON response from the API, or None if the
                          response has no content.
        """
        response = requests.post(
            f"{API_BASE_URL}/{endpoint}",
            headers=self.headers,
            json=data,
            timeout=REQUEST_TIMEOUT_SECONDS
        )
        response.raise_for_status()
        # Some POST requests return an empty body on success
        if response.status_code == 200 and response.text:
            return response.json()
        return None

# --- Command Functions ---

def handle_config_set(args):
    """Handles the 'config set' command.

    Args:
        args (argparse.Namespace): The command-line arguments, which must
                                   include `key` and `value`.
    """
    config = load_config()
    config[args.key] = args.value
    save_config(config)

def handle_sources_list(client, args):
    """Handles the 'sources list' command.

    Args:
        client (ApiClient): The API client for making requests.
        args (argparse.Namespace): The command-line arguments.
    """
    print("Fetching sources...")
    sources_data = client.get("sources")
    if not sources_data.get('sources'):
        print("No sources found.")
        return

    print("Available Sources:")
    for source in sources_data['sources']:
        print(f"- {source['name']}")

def handle_session_list(client, args):
    """Handles the 'session list' command.

    Args:
        client (ApiClient): The API client for making requests.
        args (argparse.Namespace): The command-line arguments.
    """
    print("Fetching recent sessions...")
    sessions_data = client.get("sessions")
    if not sessions_data.get('sessions'):
        print("No sessions found.")
        return

    print("Recent Sessions:")
    for session in sessions_data['sessions']:
        title = session.get('title', 'No Title')
        print(f"- {session['name']}: {title}")

def handle_session_new(client, args):
    """Handles the 'session new' command.

    Args:
        client (ApiClient): The API client for making requests.
        args (argparse.Namespace): The command-line arguments, which must
                                   include `prompt`, `source`, and `branch`.
    """
    print("Creating new session...")
    body = {
        "prompt": args.prompt,
        "sourceContext": {
            "source": args.source,
            "githubRepoContext": {
                "startingBranch": args.branch
            }
        },
        "title": args.title if args.title else args.prompt
    }

    new_session = client.post("sessions", data=body)
    if new_session and new_session.get('name'):
        print("Session created successfully!")
        print(f"  ID: {new_session['name']}")
        print(f"  Title: {new_session.get('title', 'N/A')}")
    else:
        print("Failed to create session.")

def handle_session_follow(client, args):
    """Handles the 'session follow' command.

    This function continuously polls the API for new activities in the specified
    session and prints them to the console.

    Args:
        client (ApiClient): The API client for making requests.
        args (argparse.Namespace): The command-line arguments, which must
                                   include `session_id`.
    """
    print(f"Following session: {args.session_id}. Press Ctrl+C to exit.")
    seen_activity_names = set()

    while True:
        try:
            activities_data = client.get(f"{args.session_id}/activities")
            if activities_data and 'activities' in activities_data:
                for activity in sorted(activities_data['activities'], key=lambda x: x['createTime']):
                    if activity['name'] not in seen_activity_names:
                        print("-" * 20)
                        if 'message' in activity:
                            role = activity['message'].get('role', 'unknown').upper()
                            content = activity['message'].get('content', '')
                            print(f"[{role}] {content}")
                        elif 'plan' in activity:
                            reasoning = activity['plan'].get('reasoning', 'No reasoning provided.')
                            print(f"[PLAN] {reasoning}")
                        elif 'progress' in activity:
                            message = activity['progress'].get('message', 'No message.')
                            print(f"[PROGRESS] {message}")
                        else:
                            # Fallback for unknown activity types
                            print(f"[UNKNOWN ACTIVITY]\n{json.dumps(activity, indent=2)}")

                        seen_activity_names.add(activity['name'])

            time.sleep(5) # Poll every 5 seconds
        except KeyboardInterrupt:
            print("\nExiting follow mode.")
            break
        except Exception as e:
            print(f"\nAn error occurred: {e}")
            time.sleep(10) # Wait longer after an error

def get_last_activity_summary(client, session_name):
    """Fetches the last activity for a session and returns a summary string.

    Args:
        client (ApiClient): The API client for making requests.
        session_name (str): The name of the session to fetch the activity for.

    Returns:
        str: A summary of the last activity, or an error message if the
             activity could not be fetched.
    """
    try:
        # The API should return the most recent activities first.
        activities_data = client.get(f"{session_name}/activities?pageSize=1")
        if not activities_data.get('activities'):
            return "[No activity found]"

        activity = activities_data['activities'][0]
        summary = ""
        if 'message' in activity:
            role = activity['message'].get('role', 'unknown').upper()
            content = activity['message'].get('content', '').split('\n')[0] # First line only
            summary = f"[{role}] {content}"
        elif 'plan' in activity:
            reasoning = activity['plan'].get('reasoning', 'No reasoning provided.').split('\n')[0]
            summary = f"[PLAN] {reasoning}"
        elif 'progress' in activity:
            message = activity['progress'].get('message', 'No message.').split('\n')[0]
            summary = f"[PROGRESS] {message}"
        else:
            summary = "[UNKNOWN ACTIVITY]"

        # Truncate for display
        if len(summary) > 70:
            summary = summary[:67] + "..."
        return summary

    except Exception:
        return "[Error fetching activity]"

def handle_session_interactive(client, args):
    """Handles the 'session interactive' command.

    This function displays an interactive list of recent sessions and allows
    the user to choose an action (follow or send a message).

    Args:
        client (ApiClient): The API client for making requests.
        args (argparse.Namespace): The command-line arguments.
    """
    print("Fetching recent sessions...")
    sessions_data = client.get("sessions")
    if not sessions_data.get('sessions'):
        print("No sessions found.")
        return

    choices = []
    for session in sessions_data['sessions']:
        title = session.get('title', 'No Title')
        last_activity = get_last_activity_summary(client, session['name'])
        # Add a space for alignment to make it look nicer
        choice_text = f"{title:<50} {last_activity}"
        choices.append(questionary.Choice(title=choice_text, value=session['name']))

    if not choices:
        print("No sessions to display.")
        return

    try:
        selected_session_id = questionary.select(
            "Select a session:",
            choices=choices,
            use_indicator=True
        ).ask()

        if not selected_session_id:
            return # User cancelled

        action = questionary.select(
            f"What do you want to do with session {selected_session_id}?",
            choices=["Follow", "Send Message", "Cancel"]
        ).ask()

        if action == "Follow":
            # We need to create a mock 'args' object for the handler
            follow_args = argparse.Namespace(session_id=selected_session_id)
            handle_session_follow(client, follow_args)
        elif action == "Send Message":
            message_prompt = questionary.text("Enter your message:").ask()
            if message_prompt:
                # We need a mock 'args' object here too
                message_args = argparse.Namespace(session_id=selected_session_id, prompt=message_prompt)
                handle_session_message(client, message_args)
        else:
            print("Operation cancelled.")

    except KeyboardInterrupt:
        print("\nOperation cancelled by user.")


def handle_session_message(client, args):
    """Handles the 'session message' command.

    Args:
        client (ApiClient): The API client for making requests.
        args (argparse.Namespace): The command-line arguments, which must
                                   include `session_id` and `prompt`.
    """
    print(f"Sending message to session: {args.session_id}...")
    body = {"prompt": args.prompt}
    client.post(f"{args.session_id}:sendMessage", data=body)
    print("Message sent successfully.")

def main():
    """The main entry point for the jcat CLI application.

    This function sets up the command-line argument parser, parses the
    arguments, and calls the appropriate handler function based on the
    command-line input.
    """
    parser = argparse.ArgumentParser(description="A fast and lean CLI for interacting with the Jules API.", prog="jcat")
    subparsers = parser.add_subparsers(dest='command', required=True)

    # Config command
    config_parser = subparsers.add_parser('config', help='Manage configuration')
    config_subparsers = config_parser.add_subparsers(dest='config_command', required=True)
    config_set_parser = config_subparsers.add_parser('set', help='Set a configuration value')
    config_set_parser.add_argument('key', choices=['api_key'], help='The configuration key to set')
    config_set_parser.add_argument('value', help='The value to set')
    config_set_parser.set_defaults(func=lambda args: handle_config_set(args))

    # Sources command
    sources_parser = subparsers.add_parser('sources', help='Manage sources')
    sources_subparsers = sources_parser.add_subparsers(dest='sources_command', required=True)
    sources_list_parser = sources_subparsers.add_parser('list', help='List available sources')
    # We will pass the client to the function
    sources_list_parser.set_defaults(func=lambda args, client: handle_sources_list(client, args))

    # Session command
    session_parser = subparsers.add_parser('session', help='Manage sessions')
    session_subparsers = session_parser.add_subparsers(dest='session_command', required=True)

    session_list_parser = session_subparsers.add_parser('list', help='List recent sessions')
    session_list_parser.set_defaults(func=lambda args, client: handle_session_list(client, args))

    session_new_parser = session_subparsers.add_parser('new', help='Create a new session')
    session_new_parser.add_argument('prompt', help='The initial prompt for the session')
    session_new_parser.add_argument('--source', required=True, help='The source to use (e.g., sources/github/owner/repo)')
    session_new_parser.add_argument('--branch', default='main', help='The starting branch (defaults to main)')
    session_new_parser.add_argument('--title', help='An optional title for the session')
    session_new_parser.set_defaults(func=lambda args, client: handle_session_new(client, args))

    session_follow_parser = session_subparsers.add_parser('follow', help='Follow the activity feed of a session')
    session_follow_parser.add_argument('session_id', help='The ID of the session to follow')
    session_follow_parser.set_defaults(func=lambda args, client: handle_session_follow(client, args))

    session_message_parser = session_subparsers.add_parser('message', help='Send a message to a session')
    session_message_parser.add_argument('session_id', help='The ID of the session to send a message to')
    session_message_parser.add_argument('prompt', help='The message to send')
    session_message_parser.set_defaults(func=lambda args, client: handle_session_message(client, args))

    session_interactive_parser = session_subparsers.add_parser('interactive', help='Select a session from an interactive list')
    session_interactive_parser.set_defaults(func=lambda args, client: handle_session_interactive(client, args))

    args = parser.parse_args()

    try:
        # For commands that don't need an API client (like config)
        if args.command == 'config':
            args.func(args)
            return

        # For all other commands, create an API client
        api_key = os.environ.get('JCAT_API_KEY')
        if not api_key:
            config = load_config()
            api_key = config.get('api_key')
        client = ApiClient(api_key=api_key)

        if hasattr(args, 'func'):
            args.func(args, client)
        else:
            print("Command logic not yet implemented.")

    except Exception as e:
        print(f"An error occurred: {e}")


if __name__ == "__main__":
    main()