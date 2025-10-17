# Todoist CLI (with Interactive TUI Mode)

A fast, terminal-based task manager that syncs with your Todoist account. Manage your tasks efficiently without leaving the command line.

(_This is a WIP, but it works. Feel free to contribute!_)

## Features

- 📝 Create, edit, and delete tasks
- ✅ Mark tasks as complete
- 🔄 Sync with Todoist API
- 💾 Local SQLite cache for offline access
- ⌨️ Vim-style keyboard shortcuts
- 🎨 Clean terminal UI powered by Ratatui

## Prerequisites

- Rust 1.90.0 or later
- A Todoist account
- Your Todoist API token

## Getting Your Todoist API Token

To use this application, you'll need a Todoist API token:

1. **Visit the Todoist Developer Settings**
    - Go to [Todoist Integrations Settings](https://todoist.com/prefs/integrations)
    - Or navigate manually: Todoist → Settings → Integrations → Developer

2. **Find Your API Token**
    - Scroll to the "API token" section
    - Your token will be displayed (it looks like a long string of letters and numbers)

3. **Copy Your Token**
    - Click the "Copy API token" button or select and copy the token manually
    - Keep this token secure - it provides full access to your Todoist data

For more information about the Todoist API, visit the [official API documentation](https://developer.todoist.com/api/v1/#section/Developing-with-Todoist).

## Installation

1. **Clone the repository:**
```shell script
git clone https://github.com/yourusername/todoist-cli.git
   cd todoist-cli
```


2. **Build the project:**
```shell script
cargo build --release
```


3. **Set up your API token:**

   Export your Todoist API token as an environment variable:

   **Linux/macOS:**
```shell script
export TODOIST_TOKEN="your_api_token_here"
```


**Windows (PowerShell):**
```textmate
$env:TODOIST_TOKEN="your_api_token_here"
```


**Windows (Command Prompt):**
```textmate
set TODOIST_TOKEN=your_api_token_here
```


> **Tip:** To make this permanent, add the export command to your shell's configuration file (`~/.bashrc`, `~/.zshrc`, etc.) or use Windows environment variable settings.

## Usage

### Interactive TUI Mode

Run the application without arguments to launch the interactive terminal interface:

```shell script
cargo run
```


Or using the compiled binary:

```shell script
./target/release/todoist-cli
```


### Keyboard Shortcuts

**Normal Mode:**
- `j` / `↓` - Move down
- `k` / `↑` - Move up
- `a` - Add new task
- `i` - Edit selected task
- `d` - Delete selected task
- `t` - Toggle task completion (mark as done)
- `q` - Quit application

**Insert Mode (Add/Edit):**
- Type to enter task title
- `Enter` - Save task
- `Esc` - Cancel and return to normal mode
- `Backspace` - Delete character

### Command Line Mode

You can also use the CLI directly for quick operations:

**Add a task:**
```shell script
cargo run -- add "Buy groceries"
```


**Update a task:**
```shell script
cargo run -- update 1 "Buy groceries and cook dinner" true
```


**Delete a task:**
```shell script
cargo run -- delete 1
```


## How It Works

1. **Sync on Startup:** The app fetches your tasks from Todoist when launched
2. **Local Cache:** Tasks are cached in a local SQLite database (`tasks.db`)
3. **Real-time Updates:** All changes are immediately synced with Todoist
4. **Offline Support:** View cached tasks even without internet connection

## Project Structure

```
todoist-cli/
├── src/
│   ├── api/          # Todoist API client
│   ├── cli/          # Command-line interface
│   ├── controller/   # Application state management
│   ├── db/           # SQLite cache
│   ├── models/       # Data structures
│   ├── utils/        # Helper functions
│   └── main.rs       # Entry point
├── Cargo.toml
└── README.md
```


## Troubleshooting

**"TODOIST_TOKEN env var required" error:**
- Make sure you've set the `TODOIST_TOKEN` environment variable
- Verify the token is correct by checking your Todoist settings

**Tasks not syncing:**
- Check your internet connection
- Verify your API token is still valid
- Check the Todoist API status page

**Build errors:**
- Ensure you're using Rust 1.90.0 or later: `rustc --version`
- Try cleaning the build: `cargo clean && cargo build`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the terminal UI
- Uses the [Todoist REST API v1](https://developer.todoist.com/rest/v2/)
- Inspired by the need for a fast, keyboard-driven task manager

_Not associated with Todoist in any way._