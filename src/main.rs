//! Smile - A simple emoticon picker for Gnome/Linux
//!
//! Features:
//! - Shows all available emoticons in a grid
//! - Search field to filter emoticons
//! - Shows last 10 used emoticons
//! - Clicks paste the emoticon and reopen the window
//! - Escape key quits the application

use arboard::Clipboard;
use gdk::keys::constants as key;
use glib::Propagation;
use gtk::prelude::*;
use gtk::{
    Box as GtkBox, Button, Entry, Frame, Grid, Label, Orientation, PolicyType, ScrolledWindow,
    Window, WindowPosition, WindowType,
};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

mod emoticons;
use emoticons::EMOTICONS;

mod uinput;
use uinput::UinputKeyboard;

mod settings;
use settings::Config;

lazy_static! {
    static ref CLIPBOARD: Mutex<Option<Clipboard>> = Mutex::new(Clipboard::new().ok());
}

fn get_clipboard() -> &'static Mutex<Option<Clipboard>> {
    &CLIPBOARD
}

#[cfg(test)]
mod tests;

/// Emoticon picker window state
#[derive(Clone)]
struct EmoticonPicker {
    window: Window,
    search_entry: Entry,
    emoticons_box: GtkBox,
    history: Rc<RefCell<Vec<String>>>,
    config: Rc<RefCell<Config>>,
}


impl EmoticonPicker {
    /// Create a new emoticon picker window
    fn new() -> Self {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Smile - Emoticon Picker");
        window.set_default_size(600, 500);
        window.set_position(WindowPosition::Center);
        window.set_keep_above(true);

        // Setup configuration
        let config = Config::new().expect("Failed to initialize configuration");

        // Load history
        let history = config.load_recent();

        // Main container
        let main_box = GtkBox::new(Orientation::Vertical, 10);
        main_box.set_margin_start(10);
        main_box.set_margin_end(10);
        main_box.set_margin_top(10);
        main_box.set_margin_bottom(10);

        // Search box
        let search_box = GtkBox::new(Orientation::Horizontal, 5);
        let search_label = Label::new(Some("Search:"));
        search_box.pack_start(&search_label, false, false, 0);

        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Type to filter emoticons..."));
        search_box.pack_start(&search_entry, true, true, 0);

        main_box.pack_start(&search_box, false, false, 0);

        // Scrolled window for emoticons
        let scrolled = ScrolledWindow::new(Option::<&gtk::Adjustment>::None, Option::<&gtk::Adjustment>::None);
        scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
        scrolled.set_vexpand(true);

        // Container for all emoticons
        let emoticons_box = GtkBox::new(Orientation::Vertical, 10);
        scrolled.add(&emoticons_box);

        main_box.pack_start(&scrolled, true, true, 0);

        window.add(&main_box);

        let picker = EmoticonPicker {
            window: window.clone(),
            search_entry: search_entry.clone(),
            emoticons_box: emoticons_box.clone(),
            history: Rc::new(RefCell::new(history)),
            config: Rc::new(RefCell::new(config)),
        };

        // Build the emoticon display
        picker.build_emoticons_display("");

        // Connect search changed event
        let picker_clone = picker.clone();
        search_entry.connect_changed(move |entry| {
            let filter_text = entry.text().to_string();
            picker_clone.build_emoticons_display(&filter_text);
        });

        // Connect key press event for Escape
        window.connect_key_press_event(|_, event_key| {
            if event_key.keyval() == key::Escape {
                gtk::main_quit();
                Propagation::Stop
            } else {
                Propagation::Proceed
            }
        });

        // Connect destroy event
        window.connect_destroy(|_| {
            gtk::main_quit();
        });

        // Focus on search entry
        search_entry.grab_focus();

        picker
    }

    /// Save emoticon usage history to file
    fn save_history(&self) {
        let history = self.history.borrow();
        let config = self.config.borrow();
        config.save_recent(&history).ok();
    }

    /// Add emoticon to history (most recent first)
    fn add_to_history(&self, emoticon: String) {
        let mut history = self.history.borrow_mut();

        // Remove if already exists
        if let Some(pos) = history.iter().position(|e| e == &emoticon) {
            history.remove(pos);
        }

        // Add to front
        history.insert(0, emoticon);

        // Keep only max_recent items from settings
        let max_recent = self.config.borrow().settings().max_recent;
        history.truncate(max_recent);

        drop(history);
        self.save_history();
    }

    /// Build or rebuild the emoticons display
    fn build_emoticons_display(&self, filter_text: &str) {
        // Clear existing content
        for child in self.emoticons_box.children() {
            self.emoticons_box.remove(&child);
        }

        let filter_lower = filter_text.to_lowercase();

        // Show last used emoticons if we have history and no filter
        let history = self.history.borrow();
        if !history.is_empty() && filter_text.is_empty() {
            let history_frame = Frame::new(Some("Recently Used"));
            let history_grid = Grid::new();
            history_grid.set_column_spacing(5);
            history_grid.set_row_spacing(5);
            history_grid.set_margin_start(10);
            history_grid.set_margin_end(10);
            history_grid.set_margin_top(10);
            history_grid.set_margin_bottom(10);

            let mut col = 0;
            let mut row = 0;
            for emoticon in history.iter() {
                let button = self.create_emoticon_button(emoticon);
                history_grid.attach(&button, col, row, 1, 1);
                col += 1;
                if col >= 10 {
                    col = 0;
                    row += 1;
                }
            }

            history_frame.add(&history_grid);
            self.emoticons_box.pack_start(&history_frame, false, false, 0);
        }
        drop(history);

        // Show categorized emoticons
        for (category, emoticons) in EMOTICONS.iter() {
            // Filter emoticons
            let filtered_emoticons: Vec<&String> = if !filter_text.is_empty() {
                let config = self.config.borrow();
                emoticons
                    .iter()
                    .filter(|e| {
                        // Match emoticon text
                        e.to_lowercase().contains(&filter_lower) ||
                        // Match category name or keywords
                        config.matches_category_keywords(category, &filter_lower)
                    })
                    .collect()
            } else {
                emoticons.iter().collect()
            };

            if filtered_emoticons.is_empty() {
                continue;
            }

            // Category frame
            let frame = Frame::new(Some(category));
            let grid = Grid::new();
            grid.set_column_spacing(5);
            grid.set_row_spacing(5);
            grid.set_margin_start(10);
            grid.set_margin_end(10);
            grid.set_margin_top(10);
            grid.set_margin_bottom(10);

            // Add emoticons to grid
            let mut col = 0;
            let mut row = 0;
            for emoticon in filtered_emoticons {
                let button = self.create_emoticon_button(emoticon);
                grid.attach(&button, col, row, 1, 1);
                col += 1;
                if col >= 10 {
                    col = 0;
                    row += 1;
                }
            }

            frame.add(&grid);
            self.emoticons_box.pack_start(&frame, false, false, 0);
        }

        self.window.show_all();
    }

    /// Create a button for an emoticon
    fn create_emoticon_button(&self, emoticon: &str) -> Button {
        let button = Button::with_label(emoticon);
        button.set_size_request(50, 40);

        let emoticon = emoticon.to_string();
        let picker = self.clone();
        button.connect_clicked(move |_| {
            picker.on_emoticon_clicked(&emoticon);
        });

        button
    }

    /// Handle emoticon button click
    fn on_emoticon_clicked(&self, emoticon: &str) {
        // Add to history
        self.add_to_history(emoticon.to_string());

        // Hide window
        self.window.hide();

        // Wait a bit for window to hide and focus to return
        let emoticon = emoticon.to_string();
        let picker = self.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            picker.paste_emoticon(&emoticon);
            glib::ControlFlow::Break
        });
    }

    /// Paste the emoticon using uinput (kernel-level input injection)
    fn paste_emoticon(&self, emoticon: &str) {
        // Copy emoticon to clipboard using arboard
        {
            let mut clipboard_guard = get_clipboard().lock().unwrap();
            if let Some(ref mut cb) = *clipboard_guard {
                if let Err(e) = cb.set_text(emoticon) {
                    eprintln!("Failed to set clipboard: {}", e);
                    return;
                }
            } else {
                eprintln!("Clipboard not available");
                return;
            }
        }

        // Wait a bit for window to fully hide and focus to return
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Use uinput to inject Ctrl+V at kernel level
        // This works on X11, Wayland, and even text consoles
        match UinputKeyboard::new() {
            Ok(mut keyboard) => {
                println!("Pasting from clipboard with Ctrl+V...");
                if let Err(e) = keyboard.paste_from_clipboard() {
                    eprintln!("Failed to paste via uinput: {}. Emoticon is in clipboard.", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to create uinput device: {}. Emoticon is in clipboard.", e);
                eprintln!("Note: uinput requires write access to /dev/uinput or /dev/input/uinput");
                eprintln!("You may need to add your user to the 'input' group or run with appropriate permissions.");
            }
        }

        // Show window again after a short delay
        let picker = self.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
            picker.reopen_window();
            glib::ControlFlow::Break
        });
    }

    /// Reopen the window
    fn reopen_window(&self) {
        self.window.show_all();
        self.window.present();
        self.search_entry.grab_focus();

        // Rebuild display to show updated history
        let filter_text = self.search_entry.text().to_string();
        self.build_emoticons_display(&filter_text);
    }

    /// Show the window
    fn show(&self) {
        self.window.show_all();
    }
}

fn main() {
    // Initialize GTK
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK.");
        std::process::exit(1);
    }

    // Create and show the emoticon picker
    let picker = EmoticonPicker::new();
    picker.show();

    // Start the GTK main event loop
    gtk::main();
}
