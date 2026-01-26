# Smile 😊
A simple emoticon picker for Gnome/Linux written in Rust.

## Features
- 🎨 Comprehensive emoticon collection organized by category
- 🔍 Search functionality to filter emoticons
- 📝 Recently used emoticons (last 10)
- ⚡ Quick paste via automatic typing using kernel-level uinput (works everywhere!)
- ⌨️ Escape key to quit
- 🖼️ Native GTK3 interface
- 🪟 Works on X11, Wayland, and even text consoles

## Requirements

### Runtime Dependencies
- GTK3 libraries
- Kernel module: `uinput` (usually built-in)
- User must be in the `input` group for automatic typing

### Build Dependencies
- Rust 1.70+
- GTK3 development libraries
- pkg-config

## Installation

### Ubuntu/Debian
```bash
# Install dependencies
sudo apt-get install libgtk-3-dev pkg-config

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

### Fedora/RHEL
```bash
# Install dependencies
sudo dnf install gtk3-devel pkg-config

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

1. Click an emoticon
2. The emoticon is automatically typed into your active application via kernel-level uinput
3. Window reappears for quick successive insertions
4. Press Escape to quit

**Technical Details**: Smile uses Linux's `uinput` module to create a virtual keyboard and inject key events directly at the kernel input layer. This means it works universally:
- ✅ X11 sessions
- ✅ Wayland sessions (no special tools required!)
- ✅ Text consoles (tty/VT)
- ✅ Any input consumer

**Note**: You must be in the `input` group for automatic typing to work. The emoticon is also copied to your clipboard as a fallback.
## Categories
- Happy, Sad, Angry, Surprised, Love
- Gestures, Faces, Cool, Symbols, Objects
- Animals, Food, Classic (text emoticons and kaomoji)
## License
MIT License - See LICENSE file for details
