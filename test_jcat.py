import unittest
from unittest.mock import patch, mock_open
import json
import os
import argparse

# Assuming jcat.py is in the same directory
import jcat

class TestConfigManagement(unittest.TestCase):

    @patch('os.path.exists', return_value=False)
    def test_load_config_file_not_found(self, mock_exists):
        """Test loading config when the file doesn't exist."""
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
        """Test saving a configuration."""
        config_data = {"api_key": "new_key", "user": "test_user"}
        jcat.save_config(config_data)

        # Verify the file was opened in write mode
        mock_open_file.assert_called_once_with(jcat.CONFIG_FILE, 'w')

        # Verify that json.dump was called with the correct data and arguments
        mock_json_dump.assert_called_once_with(config_data, mock_open_file(), indent=4)


class TestApiClient(unittest.TestCase):

    def test_init_missing_api_key(self):
        """Test that ApiClient raises ValueError if api_key is missing."""
        with self.assertRaisesRegex(ValueError, "API key is missing"):
            jcat.ApiClient(api_key=None)

    @patch('requests.get')
    def test_get_success(self, mock_get):
        """Test a successful GET request."""
        mock_response = mock_get.return_value
        mock_response.status_code = 200
        mock_response.json.return_value = {"data": "test"}
        mock_response.raise_for_status.return_value = None

        client = jcat.ApiClient(api_key="fake_key")
        result = client.get("some_endpoint")

        self.assertEqual(result, {"data": "test"})
        mock_get.assert_called_once_with(
            f"{jcat.API_BASE_URL}/some_endpoint",
            headers=client.headers
        )

    @patch('requests.post')
    def test_post_success(self, mock_post):
        """Test a successful POST request with a JSON response."""
        mock_response = mock_post.return_value
        mock_response.status_code = 200
        mock_response.text = '{"name": "new_session"}'
        mock_response.json.return_value = {"name": "new_session"}
        mock_response.raise_for_status.return_value = None

        client = jcat.ApiClient(api_key="fake_key")
        data = {"prompt": "test"}
        result = client.post("sessions", data=data)

        self.assertEqual(result, {"name": "new_session"})
        mock_post.assert_called_once_with(
            f"{jcat.API_BASE_URL}/sessions",
            headers=client.headers,
            json=data
        )


class TestCommandHandlers(unittest.TestCase):

    def setUp(self):
        """Set up a mock client for each test."""
        self.mock_client = unittest.mock.Mock(spec=jcat.ApiClient)

    @patch('builtins.print')
    def test_handle_sources_list(self, mock_print):
        """Test listing sources."""
        self.mock_client.get.return_value = {"sources": [{"name": "source1"}, {"name": "source2"}]}
        args = None  # Not used by the function
        jcat.handle_sources_list(self.mock_client, args)

        self.mock_client.get.assert_called_once_with("sources")
        mock_print.assert_any_call("- source1")
        mock_print.assert_any_call("- source2")

    @patch('builtins.print')
    def test_handle_sources_list_no_sources(self, mock_print):
        """Test listing sources when none are found."""
        self.mock_client.get.return_value = {"sources": []}
        args = None
        jcat.handle_sources_list(self.mock_client, args)

        mock_print.assert_called_with("No sources found.")

    @patch('builtins.print')
    def test_handle_session_new_success(self, mock_print):
        """Test creating a new session successfully."""
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
        """Test sending a message to a session."""
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
        """Test the interactive flow for following a session."""
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


class TestMainFunction(unittest.TestCase):

    @patch('sys.argv', ['jcat.py', 'config', 'set', 'api_key', 'test-key'])
    @patch('jcat.handle_config_set')
    def test_main_config_command(self, mock_handler):
        """Test that main calls the config handler and does not create a client."""
        with patch('jcat.ApiClient') as mock_api_client:
            jcat.main()
            mock_handler.assert_called_once()
            # Verify ApiClient was NOT instantiated for the 'config' command
            mock_api_client.assert_not_called()

    @patch('sys.argv', ['jcat.py', 'sources', 'list'])
    @patch('os.environ.get', return_value='env_api_key')
    @patch('jcat.handle_sources_list')
    def test_main_client_command_with_env_key(self, mock_handler, mock_env):
        """Test a client command using an API key from the environment."""
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
        """Test a client command using an API key from the config file."""
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
        """Test that main prints an error if no API key is found."""
        jcat.main()
        # The error comes from ApiClient's __init__ raising a ValueError
        mock_print.assert_called_with("An error occurred: API key is missing. Please set it via the JCAT_API_KEY environment variable or by using 'jcat config set api_key YOUR_KEY'")


if __name__ == '__main__':
    unittest.main()