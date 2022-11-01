use std::mem::size_of;
use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::INPUT_E_FRAME;
use windows::Win32::UI::Input::KeyboardAndMouse::{INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBD_EVENT_FLAGS, KEYBDINPUT, MOUSE_EVENT_FLAGS, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEINPUT, SendInput, VIRTUAL_KEY};
use windows::Win32::UI::WindowsAndMessaging::{GetMessageExtraInfo, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use crate::low_level_handler::VkCode;

fn keyboard_event(code: VkCode, flags: KEYBD_EVENT_FLAGS) {
    unsafe {
        let extra_info = GetMessageExtraInfo().0;
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(code),
                    wScan: 0,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: usize::try_from(extra_info).expect("Invalid extra info"),
                }
            },
        };
        let cb_size: i32 = size_of::<INPUT>() as i32;
        let arr: [INPUT; 1] = [input];
        SendInput(&arr, cb_size);
    }
}

fn mouse_event(x: i32, y: i32, flags: MOUSE_EVENT_FLAGS) {
    unsafe {
        let extra_info = GetMessageExtraInfo().0;
        let input = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: (x * 65536) / GetSystemMetrics(SM_CXSCREEN),
                    dy: (y * 65536) / GetSystemMetrics(SM_CYSCREEN),
                    mouseData: 0,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: usize::try_from(extra_info).expect("Invalid extra info"),
                }
            },
        };
        let cb_size: i32 = size_of::<INPUT>() as i32;
        let arr: [INPUT; 1] = [input];
        SendInput(&arr, cb_size);
    }
}

pub fn left_click(x: i32, y: i32) {
    mouse_event(x, y, MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE | MOUSEEVENTF_LEFTDOWN);
    thread::sleep(Duration::from_millis(5));
    mouse_event(x, y, MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE | MOUSEEVENTF_LEFTUP);
}

pub fn right_click(x: i32, y: i32) {
    mouse_event(x, y, MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE | MOUSEEVENTF_RIGHTDOWN);
    thread::sleep(Duration::from_millis(5));
    mouse_event(x, y, MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE | MOUSEEVENTF_RIGHTUP);
}

pub fn press_key(code: VkCode) {
    keyboard_event(code, KEYBD_EVENT_FLAGS(0));
}

pub fn release_key(code: VkCode) {
    keyboard_event(code, KEYBD_EVENT_FLAGS(0x0002));
}
