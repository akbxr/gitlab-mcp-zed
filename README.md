# GitLab MCP Server Extension for Zed

This is a Zed extension that integrates the Model Context Protocol (MCP) server for GitLab, enabling project management, file operations, and more within the Zed editor.

## Features

- **GitLab Integration**: Seamless integration with GitLab API for project management and file operations
- **Optional API URL**: Support for both gitlab.com and self-hosted GitLab instances
- **Automatic Authentication**: Handles GitLab authentication using personal access tokens

## Installation

Navigate to: **Zed** > **Extensions** Or use the command palette ([macOS](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-macos.json#L581), [Linux](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-linux.json#L459)) to search `extensions`


## Configuration

### Required Settings

You need to configure the following settings in your `settings.json`:

```json
{
  "context_servers": {
    "mcp-server-gitlab": {
      "settings": {
        "gitlab_personal_access_token": "YOUR_TOKEN"
      }
    }
  }
}
```

### Optional Settings

For self-hosted GitLab instances, you can specify a custom API URL:

```json
{
  "context_servers": {
    "mcp-server-gitlab": {
      "settings": {
        "gitlab_personal_access_token": "YOUR_TOKEN",
        "gitlab_api_url": "https://your-gitlab-instance.com/api/v4"
      }
    }
  }
}
```

## Agent Mode Configuration

If you're using Zed's agent mode, you need to enable this context server for your assistant:

1. Open Zed's Agent settings
2. Enable the GitLab MCP server. If you see that the status of the tool is a red dot, make sure you added your gitlab_personal_access_token in settings.json.
3. Enable the GitLab MCP Server in the active assistant profile. In the chat section, click on the `Write | Ask` button, then click on `tools`, then enable the GitLab MCP server.

### Personal Access Token

To create a GitLab Personal Access Token:

1. Go to GitLab User Settings > Access Tokens
2. Select the required scopes:
   - `api` for full API access
   - `read_api` for read-only access
   - `read_repository` and `write_repository` for repository operations
3. Create the token and save it securely
4. Add the token to your Zed settings as shown above

### Building from Source

1. Ensure you have Rust and Cargo installed
2. Clone the repository
3. Run `cargo build --release`

## License

Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
