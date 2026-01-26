//! uinput virtual keyboard implementation
//!
//! This module provides direct keyboard input at the kernel level using uinput,
//! which works on both X11 and Wayland, and even in text consoles.

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::thread;
use std::time::Duration;

// Linux input event constants
const EV_KEY: u16 = 0x01;
const EV_SYN: u16 = 0x00;
const SYN_REPORT: u16 = 0;

// Key press states
const KEY_RELEASE: i32 = 0;
const KEY_PRESS: i32 = 1;

// Common key codes from linux/input-event-codes.h
const KEY_LEFTCTRL: u16 = 29;
const KEY_LEFTSHIFT: u16 = 42;
const KEY_U: u16 = 22;
const KEY_ENTER: u16 = 28;

// Number keys for hex digits
const KEY_0: u16 = 11;
const KEY_1: u16 = 2;
const KEY_2: u16 = 3;
const KEY_3: u16 = 4;
const KEY_4: u16 = 5;
const KEY_5: u16 = 6;
const KEY_6: u16 = 7;
const KEY_7: u16 = 8;
const KEY_8: u16 = 9;
const KEY_9: u16 = 10;

// Letter keys for hex digits A-F
const KEY_A: u16 = 30;
const KEY_B: u16 = 48;
const KEY_C: u16 = 46;
const KEY_D: u16 = 32;
const KEY_E: u16 = 18;
const KEY_F: u16 = 33;

// ioctl constants
const UI_SET_EVBIT: u64 = 0x40045564;
const UI_SET_KEYBIT: u64 = 0x40045565;
const UI_DEV_CREATE: u64 = 0x5501;
const UI_DEV_DESTROY: u64 = 0x5502;

// Input event structure matching kernel's struct input_event
#[repr(C)]
struct InputEvent {
    tv_sec: i64,
    tv_usec: i64,
    type_: u16,
    code: u16,
    value: i32,
}

impl InputEvent {
    fn new(type_: u16, code: u16, value: i32) -> Self {
        InputEvent {
            tv_sec: 0,
            tv_usec: 0,
            type_,
            code,
            value,
        }
    }
}

// uinput user device setup structure
#[repr(C)]
struct UinputUserDev {
    name: [u8; 80],
    id: InputId,
    ff_effects_max: u32,
    absmax: [i32; 64],
    absmin: [i32; 64],
    absfuzz: [i32; 64],
    absflat: [i32; 64],
}

#[repr(C)]
struct InputId {
    bustype: u16,
    vendor: u16,
    product: u16,
    version: u16,
}

pub struct UinputKeyboard {
    file: File,
}

impl UinputKeyboard {
    /// Create a new virtual keyboard using uinput
    pub fn new() -> io::Result<Self> {
        // Try different possible uinput device paths
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/uinput")
            .or_else(|_| {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("/dev/input/uinput")
            })?;

        let fd = file.as_raw_fd();

        // Enable EV_KEY events
        unsafe {
            libc::ioctl(fd, UI_SET_EVBIT, EV_KEY as libc::c_int);
        }

        // Enable all key codes (KEY_ESC to KEY_MAX)
        // For simplicity, we enable a common range of keys
        for key in 1..256 {
            unsafe {
                libc::ioctl(fd, UI_SET_KEYBIT, key as libc::c_int);
            }
        }

        // Setup device
        let mut uidev = UinputUserDev {
            name: [0u8; 80],
            id: InputId {
                bustype: 0x03, // BUS_USB
                vendor: 0x1234,
                product: 0x5678,
                version: 1,
            },
            ff_effects_max: 0,
            absmax: [0; 64],
            absmin: [0; 64],
            absfuzz: [0; 64],
            absflat: [0; 64],
        };

        // Set device name
        let name = b"Smile Virtual Keyboard\0";
        let name_len = name.len().min(80);
        uidev.name[..name_len].copy_from_slice(&name[..name_len]);

        // Write device setup
        let uidev_bytes = unsafe {
            std::slice::from_raw_parts(
                &uidev as *const UinputUserDev as *const u8,
                std::mem::size_of::<UinputUserDev>(),
            )
        };

        use std::io::Write;
        let mut file_mut = &file;
        file_mut.write_all(uidev_bytes)?;

        // Create the device
        unsafe {
            libc::ioctl(fd, UI_DEV_CREATE);
        }

        // Give the system time to recognize the device
        thread::sleep(Duration::from_millis(100));

        Ok(UinputKeyboard { file })
    }

    /// Send a single input event
    fn send_event(&mut self, type_: u16, code: u16, value: i32) -> io::Result<()> {
        let event = InputEvent::new(type_, code, value);
        let event_bytes = unsafe {
            std::slice::from_raw_parts(
                &event as *const InputEvent as *const u8,
                std::mem::size_of::<InputEvent>(),
            )
        };
        self.file.write_all(event_bytes)?;
        self.file.flush()?;
        Ok(())
    }

    /// Send a key press event
    fn press_key(&mut self, keycode: u16) -> io::Result<()> {
        self.send_event(EV_KEY, keycode, KEY_PRESS)?;
        self.send_event(EV_SYN, SYN_REPORT, 0)?;
        Ok(())
    }

    /// Send a key release event
    fn release_key(&mut self, keycode: u16) -> io::Result<()> {
        self.send_event(EV_KEY, keycode, KEY_RELEASE)?;
        self.send_event(EV_SYN, SYN_REPORT, 0)?;
        Ok(())
    }


    /// Convert a hex digit character to its corresponding keycode
    fn hex_char_to_keycode(c: char) -> Option<u16> {
        match c {
            '0' => Some(KEY_0),
            '1' => Some(KEY_1),
            '2' => Some(KEY_2),
            '3' => Some(KEY_3),
            '4' => Some(KEY_4),
            '5' => Some(KEY_5),
            '6' => Some(KEY_6),
            '7' => Some(KEY_7),
            '8' => Some(KEY_8),
            '9' => Some(KEY_9),
            'a' | 'A' => Some(KEY_A),
            'b' | 'B' => Some(KEY_B),
            'c' | 'C' => Some(KEY_C),
            'd' | 'D' => Some(KEY_D),
            'e' | 'E' => Some(KEY_E),
            'f' | 'F' => Some(KEY_F),
            _ => None,
        }
    }

    /// Type a Unicode character using Ctrl+Shift+u method
    /// This is the standard GTK/Linux method for entering Unicode characters
    pub fn type_unicode_char(&mut self, c: char) -> io::Result<()> {
        // Get Unicode codepoint as hex string
        let codepoint = c as u32;
        let hex_string = format!("{:x}", codepoint);

        // Press Ctrl+Shift+u to start Unicode input mode
        self.press_key(KEY_LEFTCTRL)?;
        thread::sleep(Duration::from_millis(10));
        self.press_key(KEY_LEFTSHIFT)?;
        thread::sleep(Duration::from_millis(10));
        self.press_key(KEY_U)?;
        thread::sleep(Duration::from_millis(10));

        // Release u, Shift, and Ctrl
        self.release_key(KEY_U)?;
        thread::sleep(Duration::from_millis(10));
        self.release_key(KEY_LEFTSHIFT)?;
        thread::sleep(Duration::from_millis(10));
        self.release_key(KEY_LEFTCTRL)?;
        thread::sleep(Duration::from_millis(20));

        // Type each hex digit
        for hex_char in hex_string.chars() {
            if let Some(keycode) = Self::hex_char_to_keycode(hex_char) {
                self.press_key(keycode)?;
                thread::sleep(Duration::from_millis(10));
                self.release_key(keycode)?;
                thread::sleep(Duration::from_millis(10));
            }
        }

        // Press Enter to confirm
        self.press_key(KEY_ENTER)?;
        thread::sleep(Duration::from_millis(10));
        self.release_key(KEY_ENTER)?;
        thread::sleep(Duration::from_millis(10));

        Ok(())
    }

    /// Type a string by typing each Unicode character individually
    pub fn type_string(&mut self, text: &str) -> io::Result<()> {
        for c in text.chars() {
            self.type_unicode_char(c)?;
            // Small delay between characters
            thread::sleep(Duration::from_millis(20));
        }
        Ok(())
    }
}

impl Drop for UinputKeyboard {
    fn drop(&mut self) {
        // Destroy the virtual device
        let fd = self.file.as_raw_fd();
        unsafe {
            libc::ioctl(fd, UI_DEV_DESTROY);
        }
    }
}

