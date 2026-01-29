//! Smile - A simple emoticon picker for Gnome/Linux
//!
//! Features:
//! - Shows all available emoticons in a grid
//! - Search field to filter emoticons
//! - Shows last 10 used emoticons
//! - Clicks paste the emoticon and reopen the window
//! - Enter key pastes emoticon and closes the application
//! - Shift+Enter pastes emoticon and reopens the window
//! - Escape key quits the application

use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Box, Button, Entry, EventControllerKey, FlowBox, Frame, Label, Orientation, PolicyType,
    ScrolledWindow,
};
use std::cell::RefCell;
use std::rc::Rc;

mod emoticons;
use emoticons::get_emoticons;

mod uinput;
use uinput::UinputKeyboard;

mod settings;
use settings::{Config, WindowState};

#[cfg(test)]
mod tests;


/// Emoticon picker window state
#[derive(Clone)]
struct EmoticonPicker {
    window: ApplicationWindow,
    search_entry: Entry,
    emoticons_box: Box,
    history: Rc<RefCell<Vec<String>>>,
    config: Rc<RefCell<Config>>,
    first_button: Rc<RefCell<Option<Button>>>,
}


impl EmoticonPicker {
    /// Create a new emoticon picker window
    fn new(app: &gtk::Application) -> Self {
        // Setup configuration
        let config = Config::new().expect("Failed to initialize configuration");

        // Create window with default or saved dimensions
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Smile - Emoticon Picker")
            .build();

        // Load and apply saved window state
        if let Some(state) = config.load_window_state() {
            window.set_default_size(state.width, state.height);
            // Note: GTK4 doesn't allow setting position directly for Wayland compatibility
            // Position will be managed by the window manager
        } else {
            window.set_default_size(600, 500);
        }

        // Enable system theme support (dark/light mode)
        if let Some(settings) = gtk::Settings::default() {
            // Force dark theme preference to ensure app follows system dark mode
            settings.set_gtk_application_prefer_dark_theme(true);
        }


        // Load history
        let history = config.load_recent();

        // Main container
        let main_box = Box::new(Orientation::Vertical, 10);
        main_box.set_margin_start(10);
        main_box.set_margin_end(10);
        main_box.set_margin_top(10);
        main_box.set_margin_bottom(10);

        // Search box
        let search_box = Box::new(Orientation::Horizontal, 5);
        let search_label = Label::new(Some("Search:"));
        search_box.append(&search_label);

        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Type to filter emoticons..."));
        search_entry.set_hexpand(true);
        search_box.append(&search_entry);

        main_box.append(&search_box);

        // Scrolled window for emoticons
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
        scrolled.set_vexpand(true);

        // Container for all emoticons
        let emoticons_box = Box::new(Orientation::Vertical, 10);
        scrolled.set_child(Some(&emoticons_box));

        main_box.append(&scrolled);

        window.set_child(Some(&main_box));

        let picker = EmoticonPicker {
            window: window.clone(),
            search_entry: search_entry.clone(),
            emoticons_box: emoticons_box.clone(),
            history: Rc::new(RefCell::new(history)),
            config: Rc::new(RefCell::new(config)),
            first_button: Rc::new(RefCell::new(None)),
        };

        // Build the emoticon display
        picker.build_emoticons_display("");

        // Connect search entry key press event for Down arrow navigation
        let key_controller = EventControllerKey::new();
        let picker_clone = picker.clone();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Down {
                // Focus the first button when Down is pressed
                if let Some(ref button) = *picker_clone.first_button.borrow() {
                    button.grab_focus();
                    return glib::Propagation::Stop;
                }
            }
            glib::Propagation::Proceed
        });
        search_entry.add_controller(key_controller);

        // Connect search changed event
        let picker_clone = picker.clone();
        search_entry.connect_changed(move |entry| {
            let filter_text = entry.text().to_string();
            picker_clone.build_emoticons_display(&filter_text);
        });

        // Connect key press event for Escape on window
        let window_key_controller = EventControllerKey::new();
        let picker_clone = picker.clone();
        window_key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                // Save window state before exiting
                picker_clone.save_window_state();
                std::process::exit(0);
            }
            // Auto-focus search field when typing (printable characters)
            // Check if a printable character is typed and search is not already focused
            if !picker_clone.search_entry.has_focus() {
                if let Some(ch) = key.to_unicode() {
                    // Get current text and cursor position
                    let current_text = picker_clone.search_entry.text();
                    let cursor_pos = picker_clone.search_entry.position();

                    // Insert the character at the cursor position
                    let mut new_text = current_text.to_string();
                    new_text.insert(cursor_pos as usize, ch);

                    // Focus the search entry first (before setting text to avoid selection issues)
                    picker_clone.search_entry.grab_focus();

                    // Update the search entry
                    picker_clone.search_entry.set_text(&new_text);

                    // Set cursor position and ensure no text is selected
                    let new_pos = (cursor_pos + 1) as i32;
                    picker_clone.search_entry.set_position(new_pos);
                    picker_clone.search_entry.select_region(new_pos, new_pos);

                    // Stop the event from propagating since we handled it
                    return glib::Propagation::Stop;
                }
            }
            glib::Propagation::Proceed
        });
        window.add_controller(window_key_controller);

        // Save window state when closing
        let picker_for_close = picker.clone();
        window.connect_close_request(move |_| {
            picker_for_close.save_window_state();
            glib::Propagation::Proceed
        });

        // Don't focus on search entry initially - will focus on first button later
        // search_entry.grab_focus();

        picker
    }

    /// Save emoticon usage history to file
    fn save_history(&self) {
        let history = self.history.borrow();
        let config = self.config.borrow();
        config.save_recent(&history).ok();
    }

    /// Save window state (size and position)
    fn save_window_state(&self) {
        let (width, height) = self.window.default_size();
        // GTK4 doesn't provide a reliable way to get window position on Wayland
        // We'll store position as 0,0 for now (window manager handles positioning)
        let state = WindowState {
            width,
            height,
            x: 0,
            y: 0,
        };
        let config = self.config.borrow();
        config.save_window_state(&state).ok();
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
        // Clear first button reference
        *self.first_button.borrow_mut() = None;

        // Clear existing content
        while let Some(child) = self.emoticons_box.first_child() {
            self.emoticons_box.remove(&child);
        }

        let filter_lower = filter_text.to_lowercase();

        // Show last used emoticons if we have history and no filter
        let history = self.history.borrow();
        if !history.is_empty() && filter_text.is_empty() {
            let history_frame = Frame::new(Some("Recently Used"));
            let history_flowbox = FlowBox::new();
            history_flowbox.set_selection_mode(gtk::SelectionMode::None);
            history_flowbox.set_max_children_per_line(30);
            history_flowbox.set_column_spacing(5);
            history_flowbox.set_row_spacing(5);
            history_flowbox.set_margin_start(10);
            history_flowbox.set_margin_end(10);
            history_flowbox.set_margin_top(10);
            history_flowbox.set_margin_bottom(10);
            history_flowbox.set_homogeneous(false);

            let mut is_first = true;
            for emoticon in history.iter() {
                let button = self.create_emoticon_button(emoticon);

                // Store the first button for focus navigation
                if is_first {
                    *self.first_button.borrow_mut() = Some(button.clone());
                    is_first = false;
                }

                history_flowbox.append(&button);
            }

            history_frame.set_child(Some(&history_flowbox));
            self.emoticons_box.append(&history_frame);
        }
        drop(history);

        // Show categorized emoticons
        for (category, emoticons) in get_emoticons().iter() {
            // Filter emoticons
            let filtered_emoticons: Vec<&String> = if !filter_text.is_empty() {
                let config = self.config.borrow();
                emoticons
                    .iter()
                    .filter(|e| {
                        // Match emoticon keywords or the emoticon itself
                        config.matches_emoticon_keywords(e, &filter_lower) ||
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
            let flowbox = FlowBox::new();
            flowbox.set_selection_mode(gtk::SelectionMode::None);
            flowbox.set_max_children_per_line(30);
            flowbox.set_column_spacing(5);
            flowbox.set_row_spacing(5);
            flowbox.set_margin_start(10);
            flowbox.set_margin_end(10);
            flowbox.set_margin_top(10);
            flowbox.set_margin_bottom(10);
            flowbox.set_homogeneous(false);

            // Add emoticons to flowbox
            let mut is_first = self.first_button.borrow().is_none();
            for emoticon in filtered_emoticons {
                let button = self.create_emoticon_button(emoticon);

                // Store the first button for focus navigation if not already set
                if is_first {
                    *self.first_button.borrow_mut() = Some(button.clone());
                    is_first = false;
                }

                flowbox.append(&button);
            }

            frame.set_child(Some(&flowbox));
            self.emoticons_box.append(&frame);
        }

        self.window.present();

        // Focus the first button if available (and search is empty)
        // Use a small timeout to ensure window is fully mapped and focus highlight appears
        if filter_text.is_empty() {
            let first_button = self.first_button.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(50), move || {
                if let Some(ref button) = *first_button.borrow() {
                    button.grab_focus();
                }
            });
        }
    }

    /// Create a button for an emoticon
    fn create_emoticon_button(&self, emoticon: &str) -> Button {
        let button = Button::with_label(emoticon);
        button.set_size_request(50, 40);
        button.set_hexpand(false);
        button.set_vexpand(false);
        button.set_can_focus(true);
        button.set_focus_on_click(true);

        let emoticon = emoticon.to_string();

        // Clone for the click handler
        let emoticon_for_click = emoticon.clone();
        let picker = self.clone();
        button.connect_clicked(move |_| {
            picker.on_emoticon_clicked(&emoticon_for_click, false); // Default: close app after paste
        });

        // Add key press handler for Enter and Shift+Enter
        let key_controller = EventControllerKey::new();
        let emoticon_for_key = emoticon.clone();
        let picker_for_key = self.clone();
        key_controller.connect_key_pressed(move |_, key, _, modifiers| {
            if key == gdk::Key::Return || key == gdk::Key::KP_Enter {
                if modifiers.contains(gdk::ModifierType::SHIFT_MASK) {
                    // Shift+Enter: paste and reopen
                    picker_for_key.on_emoticon_clicked(&emoticon_for_key, true);
                } else {
                    // Enter: paste and close
                    picker_for_key.on_emoticon_clicked(&emoticon_for_key, false);
                }
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        button.add_controller(key_controller);

        button
    }

    /// Handle emoticon button click
    fn on_emoticon_clicked(&self, emoticon: &str, reopen: bool) {
        // Add to history
        self.add_to_history(emoticon.to_string());

        // Hide window
        self.window.set_visible(false);

        // Paste with minimal delay (just enough to let window hide)
        let emoticon = emoticon.to_string();
        let picker = self.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(10), move || {
            picker.paste_emoticon(&emoticon, reopen);
            glib::ControlFlow::Break
        });
    }

    /// Paste the emoticon using uinput (kernel-level input injection)
    fn paste_emoticon(&self, emoticon: &str, reopen: bool) {
        let emoticon = emoticon.to_string();
        let picker = self.clone();

        // Use glib's async to avoid blocking GTK
        glib::spawn_future_local(async move {
            // Wait for modifier keys to be released before typing
            // This is important for Shift+Enter to work correctly
            let mut wait_count = 0;
            let max_wait = 10; // 10 * 50ms = 500ms max

            while wait_count < max_wait {
                glib::timeout_future(std::time::Duration::from_millis(50)).await;
                wait_count += 1;

                // Check if modifier keys are still pressed
                let mut has_modifiers = false;
                if let Some(display) = gtk::gdk::Display::default() {
                    if let Some(seat) = display.default_seat() {
                        if let Some(device) = seat.keyboard() {
                            let modifier_state = device.modifier_state();
                            has_modifiers = modifier_state.contains(gdk::ModifierType::SHIFT_MASK) ||
                                          modifier_state.contains(gdk::ModifierType::CONTROL_MASK) ||
                                          modifier_state.contains(gdk::ModifierType::ALT_MASK);
                        }
                    }
                }

                if !has_modifiers {
                    break;
                }
            }

            // Type emoticon
            match UinputKeyboard::new() {
                Ok(mut keyboard) => {
                    if let Err(e) = keyboard.type_string(&emoticon) {
                        eprintln!("Failed to type emoticon via uinput: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to create uinput device: {}", e);
                    eprintln!("Note: uinput requires write access to /dev/uinput or /dev/input/uinput");
                    eprintln!("You may need to add your user to the 'input' group or run with appropriate permissions.");
                }
            }

            // Handle reopen or exit after typing is done
            if reopen {
                picker.reopen_window();
            } else {
                std::process::exit(0);
            }
        });
    }

    /// Reopen the window
    fn reopen_window(&self) {
        self.window.set_visible(true);
        self.window.present();

        // Rebuild display to show updated history
        let filter_text = self.search_entry.text().to_string();
        self.build_emoticons_display(&filter_text);

        // Focus will be set by build_emoticons_display on the first button
    }

    /// Show the window
    fn show(&self) {
        self.window.present();
    }
}

fn main() {
    // Create GTK Application
    let app = gtk::Application::builder()
        .application_id("com.github.uliruffler.smile")
        .build();

    app.connect_activate(|app| {
        // Load custom CSS to ensure proper theme support
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_string(
            r#"
            /* Support for system dark/light theme */
            window {
                background-color: @theme_bg_color;
                color: @theme_fg_color;
            }

            entry {
                background-color: @theme_base_color;
                color: @theme_text_color;
            }

            button {
                background-color: @theme_bg_color;
                color: @theme_fg_color;
            }

            /* Ensure emoticon buttons are readable in both light and dark themes */
            button {
                border: 1px solid @borders;
            }
            "#
        );

        // Apply CSS to the default display
        if let Some(display) = gtk::gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        // Create and show the emoticon picker
        let picker = EmoticonPicker::new(app);
        picker.show();
    });

    // Run the application
    app.run();
}
