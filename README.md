# Smile 😊
A simple emoticon picker for Gnome/Linux written in Rust.

## Features
- 🎨 Comprehensive emoticon collection organized by category (including laughing emojis!)
- 🔍 Search functionality to filter emoticons
- 📝 Recently used emoticons (last 10)
- ⚡ Quick paste via automatic typing using kernel-level uinput (works everywhere!)
- ⌨️ Keyboard shortcuts:
  - **Enter**: Paste emoticon and close app
  - **Shift+Enter**: Paste emoticon and reopen for multiple selections
  - **Escape**: Quit application
- 🌓 Automatic dark/light theme support (follows system settings)
- 🖼️ Native GTK4 interface
- 🪟 Works on X11, Wayland, and even text consoles
## Requirements

### Runtime Dependencies
- GTK4 libraries
- Kernel module: `uinput` (usually built-in)
- User must be in the `input` group for automatic typing

### Build Dependencies
- Rust 1.70+
- GTK4 development libraries
- pkg-config

## Installation

### Ubuntu/Debian
```bash
# Install dependencies
sudo apt-get install libgtk-4-dev pkg-config

# Setup uinput permissions
sudo usermod -aG input $USER
echo 'KERNEL=="uinput", MODE="0660", GROUP="input", OPTIONS+="static_node=uinput"' | sudo tee /etc/udev/rules.d/80-uinput.rules
sudo udevadm control --reload-rules && sudo udevadm trigger
sudo modprobe uinput

# Build
cargo build --release

# The build creates a wrapper script that handles group permissions automatically
# You can use it immediately without logging out: ./target/release/smile

### Fedora/RHEL
```bash
# Install dependencies
sudo dnf install gtk4-devel pkg-config

# Setup uinput permissions
sudo usermod -aG input $USER
echo 'KERNEL=="uinput", MODE="0660", GROUP="input", OPTIONS+="static_node=uinput"' | sudo tee /etc/udev/rules.d/80-uinput.rules
sudo udevadm control --reload-rules && sudo udevadm trigger
sudo modprobe uinput

# Build
cargo build --release
```
# The build creates a wrapper script that handles group permissions automatically
# You can use it immediately without logging out: ./target/release/smile
```

### Arch Linux
```bash
# Install dependencies
sudo pacman -S gtk3 pkgconf

# Setup uinput permissions
sudo usermod -aG input $USER
echo 'KERNEL=="uinput", MODE="0660", GROUP="input", OPTIONS+="static_node=uinput"' | sudo tee /etc/udev/rules.d/80-uinput.rules
sudo udevadm control --reload-rules && sudo udevadm trigger
sudo modprobe uinput

# Build
cargo build --release

# The build creates a wrapper script that handles group permissions automatically
# You can use it immediately without logging out: ./target/release/smile
```
## Usage
```bash
# Build
make
# Run
make run-release
# Install system-wide
make install
# Uninstall
make uninstall
```
Or directly:
```bash
cargo build --release
./target/release/smile
```
## How It Works

1. Click an emoticon or press Enter when one is focused
2. The emoticon is automatically typed into your active application via kernel-level uinput
3. **Enter**: Pastes the emoticon and closes the application
4. **Shift+Enter** or **Click**: Pastes the emoticon and reopens the window for quick successive insertions
5. Press Escape to quit

**Technical Details**: Smile uses Linux's `uinput` module to create a virtual keyboard and inject key events directly at the kernel input layer. This means it works universally:
- ✅ X11 sessions
- ✅ Wayland sessions (no special tools required!)
- ✅ Text consoles (tty/VT)
- ✅ Any input consumer

**Note**: You must be in the `input` group for automatic typing to work. The emoticon is also copied to your clipboard as a fallback.

## Configuration

Smile stores its configuration in `~/.smile/`:

- **`settings.toml`**: Application settings and keyword definitions for searching emoticons
- **`recent.json`**: Recently used emoticons (automatically migrates from old location)

### Customizing Keywords

You can customize search keywords by editing `~/.smile/settings.toml`. Each category has associated keywords that make finding emoticons easier:

```toml
[keywords.happy]
terms = ["happy", "joy", "smile", "grin", "cheerful", "glad", "pleased"]
```

When searching, typing any of these keywords (e.g., "joy") will show emoticons from that category.

### Settings

```toml
max_recent = 10  # Number of recent emoticons to remember
```

For more details, see [SETTINGS.md](SETTINGS.md).

## Categories
- Happy, Sad, Angry, Surprised, Love
- Gestures, Faces, Cool, Symbols, Objects
- Animals, Food, Classic (text emoticons and kaomoji)
## License
MIT License - See LICENSE file for details
