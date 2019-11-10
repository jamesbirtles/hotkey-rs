use std::collections::HashMap;
use std::mem;
use std::os::raw::c_ulong;
use std::ptr;
use winapi::shared::windef::{HWND, POINT};
use winapi::um::winuser;

pub mod modifiers {
    use winapi::um::winuser;
    pub const ALT: u32 = winuser::MOD_ALT;
    pub const CONTROL: u32 = winuser::MOD_CONTROL;
    pub const SHIFT: u32 = winuser::MOD_SHIFT;
    pub const SUPER: u32 = winuser::MOD_WIN;
}

pub type ListenerID = i32;

pub struct Listener {
    handlers: HashMap<ListenerID, Box<dyn Fn()>>,
}

impl Listener {
    pub fn new() -> Listener {
        Listener {
            last_id: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn register_hotkey<CB: 'static + Fn()>(
        &mut self,
        modifiers: u32,
        key: u32,
        handler: CB,
    ) -> Result<ListenerID, String> {
        unsafe {
            let id = self.last_id += 1;
            let result = winuser::RegisterHotKey(0 as HWND, id, modifiers, key);
            if result == 0 {
                return Err("Failed to register hotkey".to_string());
            }

            self.handlers.insert(id, Box::new(handler));
            Ok(id)
        }
    }

    pub fn listen(self) {
        unsafe {
            loop {
                let mut msg = winuser::MSG {
                    hwnd: 0 as HWND,
                    message: 0,
                    wParam: 0,
                    lParam: 0,
                    time: 0,
                    pt: POINT { x: 0, y: 0 },
                };
                while winuser::GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
                    if msg.wParam != 0 {
                        if let Some(handler) = self.handlers.get(&(msg.wParam as i32)) {
                            handler();
                        }
                    }
                }
            }
        }
    }
}
