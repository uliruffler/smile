# Development Guide

## Running the Application

### Requirements
Ensure all system dependencies are installed before running:
```bash
sudo apt-get install python3-gi python3-gi-cairo gir1.2-gtk-3.0 xdotool
```

### Running from Source
```bash
# Make sure you're in the project directory
cd /path/to/smile

# Run directly
./smile.py

# Or use the launcher script
./smile
```

### Installing as a Package
```bash
pip install -e .
smile
```

## Testing

### Unit Tests
Run the test suite:
```bash
python3 -m unittest test_smile.py -v
```

Or with pytest (if installed):
```bash
pytest test_smile.py -v
```

### Manual Testing Checklist
When you have access to a display server:

1. **Launch**: Verify the application window appears
2. **Layout**: Check that emoticons are organized in categories
3. **Search**: 
   - Type "happy" - should filter to Happy category
   - Type "❤" - should show Love category emoticons
   - Clear search - should show all categories
4. **Recently Used**: 
   - Should be empty on first run
   - After clicking emoticons, they should appear at the top
   - Should be limited to 10 emoticons
5. **Click Behavior**:
   - Click an emoticon
   - Window should hide briefly
   - Emoticon should be typed into focused application
   - Window should reopen automatically
6. **Escape Key**: Press Escape - application should quit immediately
7. **History Persistence**: 
   - Close and reopen app
   - Recently used emoticons should persist
   - Check `~/.config/smile/history.json` exists

## Code Structure

### Files
- `smile.py` - Main application file
- `smile` - Launcher script with dependency checking
- `setup.py` - Package installation configuration
- `test_smile.py` - Unit tests
- `requirements.txt` - Python dependencies
- `.gitignore` - Git ignore patterns
- `LICENSE` - MIT license
- `README.md` - User documentation

### Key Components

#### EmoticonPicker Class
Main GTK window class that manages:
- Window layout and UI components
- Emoticon grid display
- Search functionality
- History management
- Keyboard shortcuts

#### EMOTICONS Dictionary
Global dictionary containing all emoticons organized by category.
Categories:
- Happy, Sad, Angry, Surprised, Love
- Gestures, Faces, Cool
- Symbols, Objects, Animals, Food
- Classic (text-based emoticons)

#### Configuration
- Config directory: `~/.config/smile/`
- History file: `~/.config/smile/history.json`
- Format: JSON array of emoticon strings

## Adding New Emoticons

To add emoticons, edit the `EMOTICONS` dictionary in `smile.py`:

```python
EMOTICONS = {
    "Category Name": ["emoji1", "emoji2", ...],
    # Add more categories as needed
}
```

## Troubleshooting

### Common Issues

**GTK Not Available**
```bash
sudo apt-get install python3-gi gir1.2-gtk-3.0
```

**xdotool Not Found**
```bash
sudo apt-get install xdotool
```

**Emoticons Don't Paste (Wayland)**
xdotool has limited support on Wayland. Consider:
- Running under X11
- Using alternative tools like ydotool for Wayland

**Window Doesn't Reappear**
Check the console for errors. This might be related to window manager behavior.

## Contributing

When contributing:
1. Follow PEP 8 style guidelines
2. Add tests for new features
3. Update documentation
4. Test on multiple desktop environments if possible

## Platform Notes

### Tested On
- Ubuntu 24.04 with GNOME (X11)
- Ubuntu 24.04 with GNOME (Wayland - limited)

### Known Limitations
- Wayland support is limited due to xdotool restrictions
- Requires X11 for full functionality
- Window manager behavior may vary

## Future Enhancements

Potential improvements:
- Wayland native support using alternative input methods
- Custom emoticon collections
- Keyboard shortcuts for quick access
- System tray integration
- Configurable grid layout
- Import/export emoticon collections
- Multi-language support
