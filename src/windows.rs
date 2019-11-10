use std::thread::sleep;
use std::time::Duration;
use winapi::shared::windef::HWND;
use winapi::shared::windef::POINT;
use winapi::um::winuser;

pub fn register_hotkey(id: i32, modifiers: u32, key: u32) -> bool {
    unsafe {
        let res = winuser::RegisterHotKey(0 as HWND, id, modifiers, key);
        res != 0
    }
}

pub fn poll_keys<T>(poll_rate: Duration, cb: T)
where
    T: Fn(i32),
{
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
            while winuser::GetMessageA(&mut msg, 0 as HWND, 0, 0) > 0 {
                if msg.wParam != 0 {
                    cb(msg.wParam as i32);
                }
            }

            sleep(poll_rate);
        }
    }
}
