#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

use std::collections::HashMap;
use std::time::Duration;

#[cfg(target_os = "linux")]
use linux::{poll_keys, register_hotkey};
#[cfg(windows)]
use windows::{poll_keys, register_hotkey};

pub mod modifiers {
    pub const ALT: u32 = 1;
    pub const CONTROL: u32 = 2;
    pub const SHIFT: u32 = 4;
    pub const SUPER: u32 = 8;
}

pub type ListenerID = i32;

pub struct Listener {
    poll_rate: Duration,
    handlers: HashMap<ListenerID, Box<dyn Fn()>>,
}

impl Listener {
    pub fn new() -> Listener {
        Listener {
            poll_rate: Duration::from_millis(50),
            handlers: HashMap::new(),
        }
    }

    pub fn register_hotkey<CB: 'static + Fn()>(
        &mut self,
        modifiers: u32,
        keycode: u32,
        handler: CB,
    ) -> (bool, ListenerID) {
        let id = self.handlers.len() as i32 + 1;
        self.handlers.insert(id, Box::new(handler));

        (register_hotkey(id, modifiers, keycode), id)
    }

    pub fn listen(&self) {
        poll_keys(self.poll_rate, |id| {
            match self.handlers.get(&id) {
                Some(handler) => handler(),
                None => (),
            };
        });
    }
}
