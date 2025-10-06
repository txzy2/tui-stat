# tui-stat

`tui-stat` is a beautiful terminal-based application that displays real-time system information, weather data, and a persistent TODO list - all in an elegant TUI (Terminal User Interface).

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![TUI](https://img.shields.io/badge/TUI-ncurses-blue?style=for-the-badge)](https://en.wikipedia.org/wiki/Text-based_user_interface)
[![MIT License](https://img.shields.io/github/license/txzy2/tui-stat?style=for-the-badge)](https://github.com/txzy2/tui-stat/blob/main/LICENSE)

## ✨ Features

- 🖥️ **System Information Dashboard** - Monitor CPU, RAM, disk usage, and system load in real-time
- 🌤️ **Weather Display** - Get current weather based on your IP location
- 🌍 **IP Geolocation** - Shows your public IP and approximate geographical location
- 📝 **Persistent TODO List** - Manage tasks with a built-in database (SQLite)
- 🎨 **Beautiful UI** - Elegant terminal interface built with Ratatui
- ⚡ **Lightweight** - Minimal resource usage and fast performance

## 🚀 Installation

### Quick Install (Linux/macOS)

To install the latest version automatically:

```bash
curl -fsSL https://raw.githubusercontent.com/txzy2/tui-stat/main/install.sh | bash
```

Or download and run the install script manually:

```bash
curl -LO https://raw.githubusercontent.com/txzy2/tui-stat/main/install.sh
chmod +x install.sh
./install.sh
```

**Note**: If the installation runs without root privileges, the binary will be installed to `~/.local/bin`. Make sure this directory is in your PATH by adding it to your shell configuration:

For zsh (default on macOS):
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

For bash:
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### From Source

To build from source:

```bash
git clone https://github.com/txzy2/tui-stat.git
cd tui-stat
cargo build --release
./target/release/tui_stat
```

### Pre-built Binaries

You can download pre-built binaries from the [Releases](https://github.com/txzy2/tui-stat/releases) page.
Each release includes binaries for Linux, macOS, and Windows.

#### Prerequisites

The installation script requires the following tools to be available on your system:
- `curl` - for downloading assets
- `tar` - for extracting .tar.gz archives (most Linux/macOS systems have this by default)
- `unzip` - for extracting .zip archives (required for Windows binaries)

## ⚙️ Configuration

To get weather information, you need to set up an API key from [OpenWeatherMap](https://openweathermap.org/api):

1. Sign up for a free account at [OpenWeatherMap](https://openweathermap.org/api)
2. Get your API key from the account dashboard
3. Create a `.env` file in your home directory or in the project root:

```env
WEATHER_API_KEY=your_api_key_here
```

## 🎮 Usage

Run the application with:

```bash
tui-stat
```

### Controls

- `q` or `Ctrl+C` - Quit the application
- `j` / `k` - Navigate through the TODO list
- `A` - Add a new TODO item
- `D` - Delete the selected TODO item
- `E` - Edit the selected TODO item
- `T` - Toggle TODO status (Todo → Active → Done → Cancelled → Todo)
- `?` - Show help

## 📦 Built With

- [**Rust**](https://www.rust-lang.org/) - Systems programming language focused on safety and performance
- [**Ratatui**](https://ratatui.rs/) - Terminal user interface library for Rust
- [**Crossterm**](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation library
- [**SysInfo**](https://github.com/GuillaumeGomez/sysinfo) - Cross-platform Rust library to fetch system information
- [**SQLite**](https://www.sqlite.org/) - Lightweight database for storing TODO items
- [**Reqwest**](https://github.com/seanmonstar/reqwest) - HTTP client for API calls
- [**Tokio**](https://tokio.rs/) - Asynchronous runtime for Rust

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🐛 Issues & Support

If you encounter any bugs or have feature requests, please [open an issue](https://github.com/txzy2/tui-stat/issues) on GitHub.

## ⭐ Acknowledgments

- Inspired by the [Ratatui](https://ratatui.rs/) ecosystem and examples
- Uses [simple template](https://github.com/ratatui/templates/tree/main/simple) for quick setup
- API services: IP geolocation via ipapi.co, weather data via OpenWeatherMap API
