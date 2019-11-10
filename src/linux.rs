use std::collections::HashMap;
use std::mem;
use std::os::raw::c_ulong;
use std::ptr;
use x11_dl::xlib;

pub mod modifiers {
    use x11_dl::xlib;
    pub const ALT: u32 = xlib::Mod1Mask;
    pub const CONTROL: u32 = xlib::ControlMask;
    pub const SHIFT: u32 = xlib::ShiftMask;
    pub const SUPER: u32 = xlib::Mod4Mask;
}

pub type ListenerID = (i32, u32);

pub struct Listener {
    display: *mut xlib::Display,
    root: c_ulong,
    xlib: xlib::Xlib,
    handlers: HashMap<ListenerID, Box<dyn Fn()>>,
}

impl Listener {
    pub fn new() -> Listener {
        let xlib = xlib::Xlib::open().unwrap();
        unsafe {
            let display = (xlib.XOpenDisplay)(ptr::null());

            // Only trigger key release at end of repeated keys
            let mut supported_rtrn: i32 = mem::MaybeUninit::uninit().assume_init();
            (xlib.XkbSetDetectableAutoRepeat)(display, 1, &mut supported_rtrn);

            Listener {
                display: display,
                root: (xlib.XDefaultRootWindow)(display),
                xlib,
                handlers: HashMap::new(),
            }
        }
    }

    pub fn register_hotkey<CB: 'static + Fn()>(
        &mut self,
        modifiers: u32,
        key: u32,
        handler: CB,
    ) -> Result<ListenerID, String> {
        unsafe {
            let keycode = (self.xlib.XKeysymToKeycode)(self.display, key as u64) as i32;
            let result = (self.xlib.XGrabKey)(
                self.display,
                keycode,
                modifiers,
                self.root,
                0,
                xlib::GrabModeAsync,
                xlib::GrabModeAsync,
            );

            if result == 0 {
                return Err("Failed to register hotkey".to_string());
            }

            let id = (keycode, modifiers);
            self.handlers.insert(id, Box::new(handler));
            Ok(id)
        }
    }

    pub fn listen(self) {
        unsafe {
            (self.xlib.XSelectInput)(self.display, self.root, xlib::KeyReleaseMask);
            let mut event: xlib::XEvent = mem::MaybeUninit::uninit().assume_init();
            loop {
                (self.xlib.XNextEvent)(self.display, &mut event);
                match event.get_type() {
                    xlib::KeyRelease => {
                        if let Some(handler) = self
                            .handlers
                            .get(&(event.key.keycode as i32, event.key.state))
                        {
                            handler();
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
