#!/usr/bin/env python3
"""
Smile - A simple emoticon picker for Gnome/Linux

Features:
- Shows all available emoticons in a grid
- Search field to filter emoticons
- Shows last 10 used emoticons
- Clicks paste the emoticon and reopen the window
- Escape key quits the application
"""

import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk, Gdk, GLib
import subprocess
import json
import os
from pathlib import Path

# Emoticons database - comprehensive list organized by categories
EMOTICONS = {
    "Happy": ["😀", "😃", "😄", "😁", "😆", "😊", "😇", "🙂", "🙃", "😉", "😌", "😍", "🥰", "😘"],
    "Sad": ["😢", "😭", "😿", "😔", "😞", "😟", "😥", "😰", "😨", "😧", "😦"],
    "Angry": ["😠", "😡", "🤬", "😤", "😾", "💢"],
    "Surprised": ["😮", "😯", "😲", "😳", "🤯"],
    "Love": ["❤️", "💕", "💖", "💗", "💓", "💞", "💝", "💘", "💟", "♥️"],
    "Gestures": ["👍", "👎", "👌", "✌️", "🤞", "🤘", "🤙", "👏", "🙌", "👐", "🤲", "🤝", "🙏"],
    "Faces": ["😐", "😑", "😶", "🙄", "😏", "😣", "😥", "😮", "🤐", "😯", "😪", "😫", "🥱", "😴"],
    "Cool": ["😎", "🤓", "🧐", "😺", "😸", "😹", "😻", "😼", "😽", "🙀"],
    "Symbols": ["⭐", "✨", "🌟", "💫", "🔥", "💥", "💦", "💨", "✅", "❌", "⚡", "🌈"],
    "Objects": ["🎉", "🎊", "🎈", "🎁", "🏆", "🥇", "🥈", "🥉", "🏅", "🎖️"],
    "Animals": ["🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼", "🐨", "🐯", "🦁", "🐮", "🐷", "🐸", "🐵"],
    "Food": ["🍕", "🍔", "🍟", "🌭", "🍿", "🧂", "🍰", "🎂", "🍩", "🍪", "🍫", "🍬", "🍭", "☕", "🍵"],
    "Classic": [":-)", ":)", ":(", ":-(", ";-)", ";)", ":-D", ":D", ":-P", ":P", ":-O", ":O", ":-|", ":|", 
                "<3", "</3", ":*", ":-*", "^_^", "^.^", "o_o", "O_O", "T_T", "ToT", ">_<", "-_-",
                "¯\\_(ツ)_/¯", "(╯°□°）╯︵ ┻━┻", "(ಠ_ಠ)", "(◕‿◕)", "(づ｡◕‿‿◕｡)づ", "ʕ•ᴥ•ʔ"],
}

class EmoticonPicker(Gtk.Window):
    def __init__(self):
        super().__init__(title="Smile - Emoticon Picker")
        
        # Configuration
        self.config_dir = Path.home() / ".config" / "smile"
        self.config_file = self.config_dir / "history.json"
        self.config_dir.mkdir(parents=True, exist_ok=True)
        
        # Load history
        self.history = self.load_history()
        
        # Setup window
        self.set_default_size(600, 500)
        self.set_position(Gtk.WindowPosition.CENTER)
        self.set_keep_above(True)  # Keep window on top
        
        # Main container
        main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=10)
        main_box.set_margin_start(10)
        main_box.set_margin_end(10)
        main_box.set_margin_top(10)
        main_box.set_margin_bottom(10)
        self.add(main_box)
        
        # Search entry
        search_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=5)
        search_label = Gtk.Label(label="Search:")
        search_box.pack_start(search_label, False, False, 0)
        
        self.search_entry = Gtk.Entry()
        self.search_entry.set_placeholder_text("Type to filter emoticons...")
        self.search_entry.connect("changed", self.on_search_changed)
        search_box.pack_start(self.search_entry, True, True, 0)
        
        main_box.pack_start(search_box, False, False, 0)
        
        # Scrolled window for emoticons
        scrolled = Gtk.ScrolledWindow()
        scrolled.set_policy(Gtk.PolicyType.NEVER, Gtk.PolicyType.AUTOMATIC)
        scrolled.set_vexpand(True)
        
        # Container for all emoticons
        self.emoticons_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=10)
        scrolled.add(self.emoticons_box)
        
        main_box.pack_start(scrolled, True, True, 0)
        
        # Build the emoticon display
        self.build_emoticons_display()
        
        # Connect key press event for Escape
        self.connect("key-press-event", self.on_key_press)
        
        # Focus on search entry
        self.search_entry.grab_focus()
    
    def load_history(self):
        """Load emoticon usage history from file"""
        if self.config_file.exists():
            try:
                with open(self.config_file, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except:
                return []
        return []
    
    def save_history(self):
        """Save emoticon usage history to file"""
        try:
            with open(self.config_file, 'w', encoding='utf-8') as f:
                json.dump(self.history, f, ensure_ascii=False)
        except Exception as e:
            print(f"Error saving history: {e}")
    
    def add_to_history(self, emoticon):
        """Add emoticon to history (most recent first)"""
        # Remove if already exists
        if emoticon in self.history:
            self.history.remove(emoticon)
        # Add to front
        self.history.insert(0, emoticon)
        # Keep only last 10
        self.history = self.history[:10]
        self.save_history()
    
    def build_emoticons_display(self, filter_text=""):
        """Build or rebuild the emoticons display"""
        # Clear existing content
        for child in self.emoticons_box.get_children():
            self.emoticons_box.remove(child)
        
        filter_lower = filter_text.lower()
        
        # Show last used emoticons if we have history and no filter
        if self.history and not filter_text:
            history_frame = Gtk.Frame(label="Recently Used")
            history_grid = Gtk.Grid()
            history_grid.set_column_spacing(5)
            history_grid.set_row_spacing(5)
            history_grid.set_margin_start(10)
            history_grid.set_margin_end(10)
            history_grid.set_margin_top(10)
            history_grid.set_margin_bottom(10)
            
            col = 0
            row = 0
            for emoticon in self.history:
                button = self.create_emoticon_button(emoticon)
                history_grid.attach(button, col, row, 1, 1)
                col += 1
                if col >= 10:  # 10 columns
                    col = 0
                    row += 1
            
            history_frame.add(history_grid)
            self.emoticons_box.pack_start(history_frame, False, False, 0)
        
        # Show categorized emoticons
        for category, emoticons in EMOTICONS.items():
            # Filter emoticons
            if filter_text:
                filtered_emoticons = [e for e in emoticons 
                                    if filter_lower in e.lower() or filter_lower in category.lower()]
            else:
                filtered_emoticons = emoticons
            
            if not filtered_emoticons:
                continue
            
            # Category frame
            frame = Gtk.Frame(label=category)
            grid = Gtk.Grid()
            grid.set_column_spacing(5)
            grid.set_row_spacing(5)
            grid.set_margin_start(10)
            grid.set_margin_end(10)
            grid.set_margin_top(10)
            grid.set_margin_bottom(10)
            
            # Add emoticons to grid
            col = 0
            row = 0
            for emoticon in filtered_emoticons:
                button = self.create_emoticon_button(emoticon)
                grid.attach(button, col, row, 1, 1)
                col += 1
                if col >= 10:  # 10 columns
                    col = 0
                    row += 1
            
            frame.add(grid)
            self.emoticons_box.pack_start(frame, False, False, 0)
        
        self.show_all()
    
    def create_emoticon_button(self, emoticon):
        """Create a button for an emoticon"""
        button = Gtk.Button(label=emoticon)
        # Make button larger and more visible
        button.set_size_request(50, 40)
        button.connect("clicked", self.on_emoticon_clicked, emoticon)
        return button
    
    def on_emoticon_clicked(self, button, emoticon):
        """Handle emoticon button click"""
        # Add to history
        self.add_to_history(emoticon)
        
        # Hide window
        self.hide()
        
        # Wait a bit for window to hide and focus to return
        GLib.timeout_add(100, self.paste_emoticon, emoticon)
    
    def paste_emoticon(self, emoticon):
        """Paste the emoticon using xdotool"""
        try:
            # Use xdotool to type the emoticon
            subprocess.run(['xdotool', 'type', '--clearmodifiers', emoticon], 
                         check=False, timeout=1)
        except Exception as e:
            print(f"Error pasting emoticon: {e}")
        
        # Show window again after a short delay
        GLib.timeout_add(200, self.reopen_window)
        return False
    
    def reopen_window(self):
        """Reopen the window"""
        self.show_all()
        self.present()
        self.search_entry.grab_focus()
        # Rebuild display to show updated history
        self.build_emoticons_display(self.search_entry.get_text())
        return False
    
    def on_search_changed(self, entry):
        """Handle search text change"""
        filter_text = entry.get_text()
        self.build_emoticons_display(filter_text)
    
    def on_key_press(self, widget, event):
        """Handle key press events"""
        # Check for Escape key
        if event.keyval == Gdk.KEY_Escape:
            Gtk.main_quit()
            return True
        return False

def main():
    """Main entry point"""
    app = EmoticonPicker()
    app.connect("destroy", Gtk.main_quit)
    app.show_all()
    Gtk.main()

if __name__ == "__main__":
    main()
