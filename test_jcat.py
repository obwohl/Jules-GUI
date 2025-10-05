"""Unit tests for the jcat CLI application.

This module contains tests for the configuration management, API client,
and command handlers in jcat.py.
"""
import unittest
from unittest.mock import patch, mock_open
import json
import os
import argparse

# Assuming jcat.py is in the same directory
import jcat

# Define a reasonable timeout for all network requests.
# This should ideally be defined in the jcat.py file alongside the ApiClient.
# We define it here to use in our tests.
REQUEST_TIMEOUT_SECONDS = 30

class TestConfigManagement(unittest.TestCase):
    """Tests for configuration management functions."""

    @patch('os.path.exists', return_value=False)
    def test_load_config_file_not_found(self, mock_exists):
        """Test loading config when the file doesn't exist.

        Args:
            mock_exists (unittest.mock.Mock): Mock for os.path.exists.
        """
        config = jcat.load_config()
        self.assertEqual(config, {})

    def test_load_config_file_found(self):
        """Test loading a valid config file."""
        mock_data = json.dumps({"api_key": "test_key"})
        with patch('builtins.open', mock_open(read_data=mock_data)) as mock_file:
            with patch('os.path.exists', return_value=True):
                config = jcat.load_config()
                self.assertEqual(config, {"api_key": "test_key"})
                # Check that the correct file path was opened
                mock_file.assert_called_once_with(jcat.CONFIG_FILE, 'r')

    @patch('builtins.open', new_callable=mock_open)
    @patch('json.dump')
    def test_save_config(self, mock_json_dump, mock_open_file):
        """Test saving a configuration.

        Args:
            mock_json_dump (unittest.mock.Mock): Mock for json.dump.
            mock_open_file (unittest.mock.Mock): Mock for builtins.open.
        """
        config_data = {"api_key": "new_key", "user": "test_user"}
        jcat.save_config(config_data)

        # Verify the file was opened in write mode
        mock_open_file.assert_called_once_with(jcat.CONFIG_FILE, 'w')

        # Verify that json.dump was called with the correct data and arguments
        mock_json_dump.assert_called_once_with(config_data, mock_open_file(), indent=4)


class TestApiClient(unittest.TestCase):
    """Tests for the ApiClient class."""

    def test_init_missing_api_key(self):
        """Test that ApiClient raises ValueError if api_key is missing."""
        with self.assertRaisesRegex(ValueError, "API key is missing"):
            jcat.ApiClient(api_key=None)

    @patch('requests.get')
    def test_get_success(self, mock_get):
        """Test a successful GET request and ensure it uses a timeout.

        Args:
            mock_get (unittest.mock.Mock): Mock for requests.get.
        """
        mock_response = mock_get.return_value
        mock_response.status_code = 200
        mock_response.json.return_value = {"data": "test"}
        mock_response.raise_for_status.return_value = None

        client = jcat.ApiClient(api_key="fake_key")
        result = client.get("some_endpoint")

        self.assertEqual(result, {"data": "test"})
        # VERIFY TIMEOUT: Ensure network requests do not hang indefinitely.
        mock_get.assert_called_once_with(
            f"{jcat.API_BASE_URL}/some_endpoint",
            headers=client.headers,
            timeout=REQUEST_TIMEOUT_SECONDS  # Check that a timeout is passed
        )

    @patch('requests.post')
    def test_post_success(self, mock_post):
        """Test a successful POST request and ensure it uses a timeout.

        Args:
            mock_post (unittest.mock.Mock): Mock for requests.post.
        """
        mock_response = mock_post.return_value
        mock_response.status_code = 200
        mock_response.text = '{"name": "new_session"}'
        mock_response.json.return_value = {"name": "new_session"}
        mock_response.raise_for_status.return_value = None

        client = jcat.ApiClient(api_key="fake_key")
        data = {"prompt": "test"}
        result = client.post("sessions", data=data)

        self.assertEqual(result, {"name": "new_session"})
        # VERIFY TIMEOUT: Ensure network requests do not hang indefinitely.
        mock_post.assert_called_once_with(
            f"{jcat.API_BASE_URL}/sessions",
            headers=client.headers,
            json=data,
            timeout=REQUEST_TIMEOUT_SECONDS # Check that a timeout is passed
        )


class TestCommandHandlers(unittest.TestCase):
    """Tests for the command handler functions."""

    def setUp(self):
        """Set up a mock client for each test."""
        self.mock_client = unittest.mock.Mock(spec=jcat.ApiClient)

    @patch('builtins.print')
    def test_handle_sources_list(self, mock_print):
        """Test listing sources.

        Args:
            mock_print (unittest.mock.Mock): Mock for builtins.print.
        """
        self.mock_client.get.return_value = {"sources": [{"name": "source1"}, {"name": "source2"}]}
        args = None  # Not used by the function
        jcat.handle_sources_list(self.mock_client, args)

        self.mock_client.get.assert_called_once_with("sources")
        mock_print.assert_any_call("- source1")
        mock_print.assert_any_call("- source2")

    @patch('builtins.print')
    def test_handle_sources_list_no_sources(self, mock_print):
        """Test listing sources when none are found.

        Args:
            mock_print (unittest.mock.Mock): Mock for builtins.print.
        """
        self.mock_client.get.return_value = {"sources": []}
        args = None
        jcat.handle_sources_list(self.mock_client, args)

        mock_print.assert_called_with("No sources found.")

    @patch('builtins.print')
    def test_handle_session_new_success(self, mock_print):
        """Test creating a new session successfully.

        Args:
            mock_print (unittest.mock.Mock): Mock for builtins.print.
        """
        args = argparse.Namespace(
            prompt="Test prompt",
            source="github/test/repo",
            branch="develop",
            title="Test Title"
        )
        self.mock_client.post.return_value = {
            "name": "sessions/123",
            "title": "Test Title"
        }

        jcat.handle_session_new(self.mock_client, args)

        self.mock_client.post.assert_called_once()
        mock_print.assert_any_call("Session created successfully!")
        mock_print.assert_any_call("  ID: sessions/123")

    @patch('builtins.print')
    def test_handle_session_message(self, mock_print):
        """Test sending a message to a session.

        Args:
            mock_print (unittest.mock.Mock): Mock for builtins.print.
        """
        args = argparse.Namespace(
            session_id="sessions/123",
            prompt="Hello there"
        )

        jcat.handle_session_message(self.mock_client, args)

        self.mock_client.post.assert_called_once_with(
            "sessions/123:sendMessage",
            data={"prompt": "Hello there"}
        )
        mock_print.assert_called_with("Message sent successfully.")

    @patch('questionary.select')
    @patch('jcat.handle_session_follow')
    def test_handle_session_interactive_follow(self, mock_follow, mock_select):
        """Test the interactive flow for following a session.

        Args:
            mock_follow (unittest.mock.Mock): Mock for jcat.handle_session_follow.
            mock_select (unittest.mock.Mock): Mock for questionary.select.
        """
        # Simulate the user selecting a session, then the 'Follow' action
        mock_select.side_effect = [
            unittest.mock.Mock(ask=lambda: "sessions/456"), # First, select a session
            unittest.mock.Mock(ask=lambda: "Follow")         # Second, select an action
        ]

        self.mock_client.get.return_value = {
            "sessions": [{"name": "sessions/456", "title": "Interactive Session"}]
        }
        # Mock get_last_activity_summary to avoid another real API call
        with patch('jcat.get_last_activity_summary', return_value="[Summary]"):
            jcat.handle_session_interactive(self.mock_client, None)

        # Check that the follow handler was called with the correct session ID
        mock_follow.assert_called_once()
        self.assertEqual(mock_follow.call_args[0][1].session_id, "sessions/456")

    @patch('questionary.confirm')
    @patch('jcat.handle_session_approve_plan')
    def test_print_activity_prompts_for_plan_approval(self, mock_approve_handler, mock_confirm):
        """Test that print_activity prompts the user when a plan needs approval.

        Args:
            mock_approve_handler (unittest.mock.Mock): Mock for handle_session_approve_plan.
            mock_confirm (unittest.mock.Mock): Mock for questionary.confirm.
        """
        # Simulate the user confirming the prompt
        mock_confirm.return_value.ask.return_value = True

        activity = {
            "name": "sessions/123/activities/abc",
            "planGenerated": {
                "plan": {
                    "reasoning": "A plan that needs approval.",
                    "state": "NEEDS_APPROVAL",
                    "steps": [{"title": "Do something"}]
                }
            }
        }

        jcat.print_activity(activity, client=self.mock_client)

        # Verify that the user was prompted
        mock_confirm.assert_called_once_with("This plan requires your approval. Do you want to approve it?")
        # Verify the handler was called with the correct session ID
        mock_approve_handler.assert_called_once_with(self.mock_client, "sessions/123")

    @patch('questionary.confirm')
    @patch('jcat.handle_session_commit')
    def test_print_activity_prompts_for_commit(self, mock_commit_handler, mock_confirm):
        """Test that print_activity prompts the user when a session can be committed.

        Args:
            mock_commit_handler (unittest.mock.Mock): Mock for handle_session_commit.
            mock_confirm (unittest.mock.Mock): Mock for questionary.confirm.
        """
        # Simulate the user confirming the prompt
        mock_confirm.return_value.ask.return_value = True

        activity = {
            "name": "sessions/456/activities/def",
            "sessionCompleted": {
                "commitInfo": {
                    "suggestedCommitMessage": "feat: Implement new feature\n\nThis is a great feature."
                }
            }
        }

        jcat.print_activity(activity, client=self.mock_client)

        # Verify that the user was prompted
        mock_confirm.assert_called_once_with("Create this commit and open a Pull Request?")
        # Verify the handler was called with the correct session ID
        mock_commit_handler.assert_called_once()
        # The args passed to the handler are the client and a Namespace object
        self.assertEqual(mock_commit_handler.call_args[0][1].session_id, "sessions/456")


class TestTruncation(unittest.TestCase):
    """Tests for the output truncation logic."""

    def test_truncate_output_no_truncation_needed(self):
        """Test that output with fewer lines than max_lines is not truncated."""
        short_output = "Line 1\nLine 2\nLine 3"
        result = jcat.truncate_output(short_output, max_lines=5)
        self.assertEqual(result, short_output)

    def test_truncate_output_with_head_and_tail(self):
        """Test that long output is truncated to show head and tail."""
        # Create a 40-line string
        long_output = "\n".join([f"Line {i}" for i in range(40)])
        # Truncate with default head/tail of 10 lines each, max_lines 25
        truncated = jcat.truncate_output(long_output)

        # Check for the truncation message
        self.assertIn("... (output truncated, omitting 20 lines) ...", truncated)
        # Check that the first line (from the head) is present
        self.assertIn("Line 0", truncated)
        # Check that the last line (from the tail) is present
        self.assertIn("Line 39", truncated)
        # Check that a middle line is NOT present
        self.assertNotIn("Line 15", truncated)
        # Check that the total number of lines is correct (10 head + 10 tail + message)
        self.assertEqual(len(truncated.split('\n')), 21)

    def test_truncate_output_line_length_still_works(self):
        """Test that individual lines are still truncated if they are too long."""
        long_line = "a" * 200
        long_output = "Line 1\n" + long_line + "\nLine 3"
        truncated = jcat.truncate_output(long_output, max_line_length=150)
        self.assertIn("...[LINE TRUNCATED]...", truncated)

    def test_truncate_output_edge_case_just_over_limit(self):
        """Test truncation when the line count is just over the combined head/tail size."""
        # 26 lines, where max_lines is 25, head is 10, tail is 10.
        output = "\n".join([f"Line {i}" for i in range(26)])
        truncated = jcat.truncate_output(output, max_lines=25, head_lines=10, tail_lines=10)
        self.assertIn("... (output truncated, omitting 6 lines) ...", truncated)
        self.assertIn("Line 0", truncated)
        self.assertIn("Line 25", truncated)
        self.assertNotIn("Line 12", truncated)

    def test_truncate_output_edge_case_at_limit(self):
        """Test that no truncation occurs when line count equals max_lines."""
        output = "\n".join([f"Line {i}" for i in range(25)])
        result = jcat.truncate_output(output, max_lines=25)
        self.assertEqual(result, output)


class TestMainFunction(unittest.TestCase):
    """Tests for the main function and argument parsing."""

    @patch('sys.argv', ['jcat.py', 'config', 'set', 'api_key', 'test-key'])
    @patch('jcat.handle_config_set')
    def test_main_config_command(self, mock_handler):
        """Test that main calls the config handler and does not create a client.

        Args:
            mock_handler (unittest.mock.Mock): Mock for jcat.handle_config_set.
        """
        with patch('jcat.ApiClient') as mock_api_client:
            jcat.main()
            mock_handler.assert_called_once()
            # Verify ApiClient was NOT instantiated for the 'config' command
            mock_api_client.assert_not_called()

    @patch('sys.argv', ['jcat.py', 'sources', 'list'])
    @patch('os.environ.get', return_value='env_api_key')
    @patch('jcat.handle_sources_list')
    def test_main_client_command_with_env_key(self, mock_handler, mock_env):
        """Test a client command using an API key from the environment.

        Args:
            mock_handler (unittest.mock.Mock): Mock for jcat.handle_sources_list.
            mock_env (unittest.mock.Mock): Mock for os.environ.get.
        """
        with patch('jcat.ApiClient') as mock_api_client_class:
            # Make the ApiClient constructor return a mock instance
            mock_client_instance = unittest.mock.Mock()
            mock_api_client_class.return_value = mock_client_instance

            jcat.main()

            # Verify ApiClient was initialized with the key from the environment
            mock_api_client_class.assert_called_once_with(api_key='env_api_key')
            # Verify the correct handler was called with the client instance
            mock_handler.assert_called_once()
            # The handler is called as handler(client, args), so client is the first arg.
            self.assertIs(mock_handler.call_args[0][0], mock_client_instance)

    @patch('sys.argv', ['jcat.py', 'session', 'list'])
    @patch('os.environ.get', return_value=None) # No env key
    @patch('jcat.load_config', return_value={'api_key': 'config_api_key'})
    @patch('jcat.handle_session_list')
    def test_main_client_command_with_config_key(self, mock_handler, mock_load_config, mock_env):
        """Test a client command using an API key from the config file.

        Args:
            mock_handler (unittest.mock.Mock): Mock for jcat.handle_session_list.
            mock_load_config (unittest.mock.Mock): Mock for jcat.load_config.
            mock_env (unittest.mock.Mock): Mock for os.environ.get.
        """
        with patch('jcat.ApiClient') as mock_api_client_class:
            mock_client_instance = unittest.mock.Mock()
            mock_api_client_class.return_value = mock_client_instance

            jcat.main()

            mock_api_client_class.assert_called_once_with(api_key='config_api_key')
            mock_handler.assert_called_once()
            # The handler is called as handler(client, args), so client is the first arg.
            self.assertIs(mock_handler.call_args[0][0], mock_client_instance)

    @patch('sys.argv', ['jcat.py', 'sources', 'list'])
    @patch('os.environ.get', return_value=None)
    @patch('jcat.load_config', return_value={}) # No key in config
    @patch('builtins.print')
    def test_main_missing_api_key_error(self, mock_print, mock_load_config, mock_env):
        """Test that main prints an error if no API key is found.

        Args:
            mock_print (unittest.mock.Mock): Mock for builtins.print.
            mock_load_config (unittest.mock.Mock): Mock for jcat.load_config.
            mock_env (unittest.mock.Mock): Mock for os.environ.get.
        """
        jcat.main()
        # The error comes from ApiClient's __init__ raising a ValueError
        mock_print.assert_called_with("An error occurred: API key is missing. Please set it via the JCAT_API_KEY environment variable or by using 'jcat config set api_key YOUR_KEY'")


if __name__ == '__main__':
    unittest.main()
