use std::ffi::CString;
use std::mem;
use std::os::raw::*;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use x11_dl::keysym;
use x11_dl::xlib;

pub fn register_hotkey(id: i32, modifiers: u32, key: u32) -> bool {
    unsafe {
        let xlib = xlib::Xlib::open().unwrap();
        let display = (xlib.XOpenDisplay)(ptr::null());
        let root = (xlib.XDefaultRootWindow)(display);

        (xlib.XGrabKey)(
            display,
            (xlib.XKeysymToKeycode)(display, keysym::XK_Y as u64) as i32,
            xlib::ControlMask | xlib::ShiftMask,
            root,
            0,
            xlib::GrabModeAsync,
            xlib::GrabModeAsync,
        );
        (xlib.XSelectInput)(display, root, xlib::KeyPressMask);

        let mut event: xlib::XEvent = mem::uninitialized();
        loop {
            println!("Waiting for event...");
            (xlib.XNextEvent)(display, &mut event);

            match event.get_type() {
                xlib::KeyPress => {
                    println!("Got keypress");
                }
                _ => (),
            }
        }
    }
    println!("Here");
    true
}

pub fn poll_keys<T>(poll_rate: Duration, cb: T)
where
    T: Fn(i32),
{
    loop {
        sleep(poll_rate);
    }
}
