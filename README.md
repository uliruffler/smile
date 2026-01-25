# Smile 😊
A simple emoticon picker for Gnome/Linux written in Rust.
## Features
- 🎨 Comprehensive emoticon collection organized by category
- 🔍 Search functionality to filter emoticons
- 📝 Recently used emoticons (last 10)
- ⚡ Quick paste via clipboard
- ⌨️ Escape key to quit
- 🖼️ Native GTK3 interface
## Requirements
### Runtime Dependencies
- GTK3 libraries
- xdotool
- xclip
### Build Dependencies
- Rust 1.70+
- GTK3 development libraries
- pkg-config
## Installation
### Ubuntu/Debian
```bash
sudo apt-get install libgtk-3-dev xdotool xclip pkg-config
cargo build --release
```
### Fedora/RHEL
```bash
sudo dnf install gtk3-devel xdotool xclip pkg-config
cargo build --release
```
### Arch Linux
```bash
sudo pacman -S gtk3 xdotool xclip pkgconf
cargo build --release
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
2. It's copied to clipboard and pasted into your active application
3. Window reappears for quick successive insertions
4. Press Escape to quit
## Categories
- Happy, Sad, Angry, Surprised, Love
- Gestures, Faces, Cool, Symbols, Objects
- Animals, Food, Classic (text emoticons and kaomoji)
## License
MIT License - See LICENSE file for details
