# Smile 😊

Smile is a simple emoticon picker for Gnome/Linux that makes it easy to insert emoticons into any application.

## Features

- 🎨 **Comprehensive emoticon collection** - Hundreds of emoticons organized by category
- 🔍 **Search functionality** - Quickly filter emoticons by typing
- 📝 **Recently used** - Shows your last 10 used emoticons for quick access
- ⚡ **Quick paste** - Click an emoticon to paste it and automatically reopen the picker
- ⌨️ **Keyboard shortcuts** - Press Escape to quit the application
- 🖼️ **Clean GTK3 interface** - Native Gnome look and feel

## Requirements

### System Dependencies

Smile requires the following system packages (Ubuntu/Debian):

```bash
sudo apt-get install python3-gi python3-gi-cairo gir1.2-gtk-3.0 xdotool
```

For other distributions:
- **Fedora/RHEL**: `sudo dnf install python3-gobject gtk3 python3-cairo xdotool`
- **Arch Linux**: `sudo pacman -S python-gobject gtk3 xdotool`

### Python Dependencies

```bash
pip install -r requirements.txt
```

## Installation

### Option 1: Run directly

```bash
# Clone the repository
git clone https://github.com/uliruffler/smile.git
cd smile

# Install system dependencies (Ubuntu/Debian)
sudo apt-get install python3-gi python3-gi-cairo gir1.2-gtk-3.0 xdotool

# Run the application
./smile.py
```

### Option 2: Install with pip

```bash
# Clone the repository
git clone https://github.com/uliruffler/smile.git
cd smile

# Install system dependencies first (see above)

# Install the application
pip install -e .

# Run from anywhere
smile
```

## Usage

1. **Launch Smile**: Run `./smile.py` or `smile` (if installed)
2. **Browse emoticons**: Scroll through categorized emoticons
3. **Search**: Type in the search field to filter emoticons
4. **Select**: Click any emoticon to:
   - Paste it into the currently focused application
   - Automatically reopen the picker for quick successive insertions
5. **Recently Used**: Your last 10 emoticons appear at the top for quick access
6. **Quit**: Press `Escape` to exit the application

## How It Works

When you click an emoticon:
1. The window temporarily hides
2. Focus returns to your previous application
3. The emoticon is typed using `xdotool`
4. The Smile window reappears for your next selection

This workflow allows you to quickly insert multiple emoticons without repeatedly opening and closing the application.

## Configuration

Smile stores your emoticon history in `~/.config/smile/history.json`. This file is automatically created and updated.

## Categories

Emoticons are organized into these categories:
- Happy
- Sad  
- Angry
- Surprised
- Love
- Gestures
- Faces
- Cool
- Symbols
- Objects
- Animals
- Food
- Classic (text-based emoticons like `:-)` and kaomoji)

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Feel free to:
- Add new emoticons to the collection
- Improve the UI/UX
- Fix bugs
- Add new features

## Troubleshooting

**Issue**: Emoticons don't paste
- **Solution**: Make sure `xdotool` is installed and you have X11 running (Wayland may have limitations)

**Issue**: Window doesn't appear
- **Solution**: Check that GTK3 and Python GObject bindings are properly installed

**Issue**: Search doesn't work
- **Solution**: Make sure you click in the search field or it has focus when typing
