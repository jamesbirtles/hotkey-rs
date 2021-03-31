use std::{collections::HashMap, ptr::null_mut};
use std::mem;
use winapi::shared::windef::HWND;
use winapi::um::winuser;

pub mod modifiers {
    use winapi::um::winuser;
    pub const ALT: u32 = winuser::MOD_ALT as u32;
    pub const CONTROL: u32 = winuser::MOD_CONTROL as u32;
    pub const SHIFT: u32 = winuser::MOD_SHIFT as u32;
    pub const SUPER: u32 = winuser::MOD_WIN as u32;
}

pub mod keys {
    use winapi::um::winuser;
    pub const BACKSPACE: i32 = winuser::VK_BACK;
    pub const TAB: i32 = winuser::VK_TAB;
    pub const ENTER: i32 = winuser::VK_RETURN;
    pub const CAPS_LOCK: i32 = winuser::VK_CAPITAL;
    pub const ESCAPE: i32 = winuser::VK_ESCAPE;
    pub const SPACEBAR: i32 = winuser::VK_SPACE;
    pub const PAGE_UP: i32 = winuser::VK_PRIOR;
    pub const PAGE_DOWN: i32 = winuser::VK_NEXT;
    pub const END: i32 = winuser::VK_END;
    pub const HOME: i32 = winuser::VK_HOME;
    pub const ARROW_LEFT: i32 = winuser::VK_LEFT;
    pub const ARROW_RIGHT: i32 = winuser::VK_RIGHT;
    pub const ARROW_UP: i32 = winuser::VK_UP;
    pub const ARROW_DOWN: i32 = winuser::VK_DOWN;
    pub const PRINT_SCREEN: i32 = winuser::VK_SNAPSHOT;
    pub const INSERT: i32 = winuser::VK_INSERT;
    pub const DELETE: i32 = winuser::VK_DELETE;
}

pub type ListenerID = i32;

pub struct Listener {
    last_id: i32,
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
            self.last_id += 1;
            let id = self.last_id;
            let result = winuser::RegisterHotKey(0 as HWND, id, modifiers, key);
            if result == 0 {
                return Err("Failed to register hotkey".to_string());
            }

            self.handlers.insert(id, Box::new(handler));
            Ok(id)
        }
    }

    /// Sends [`WM_QUIT`](winapi::um::winuser::WM_QUIT) siqnal to interupt [`listen`](#listen) infinite loop.
    pub fn post_quit_message() {
        unsafe { winuser::PostQuitMessage(0) }
    }

    /// Runs blocking infinite loop to listen for events.
    ///
    /// *Note:* callbacks are beeing called from current thread in "blocking" fashion. Make sure you aren't blocking
    /// main thread for ever if you want to dispatch hotkey presses.
    ///
    /// You can execute [`PostQuitMessage`](winapi::um::winuser::PostQuitMessage) or call
    /// [`Listener::post_quit_message`](#post_quit_message)
    /// to interupt this loop.
    pub fn listen(self) {
        unsafe {
            let mut msg = mem::MaybeUninit::uninit().assume_init();
            loop {
                match winuser::GetMessageW(&mut msg, null_mut(), 0, 0) {
                    0 => break,
                    ret if msg.wParam != 0 && ret != -1 => {
                        if let Some(handler) = self.handlers.get(&(msg.wParam as i32)) {
                            handler();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
