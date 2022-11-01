use std::mem::size_of;
use windows::Win32::Foundation::INPUT_E_FRAME;
use windows::Win32::UI::Input::KeyboardAndMouse::{INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, SendInput, VIRTUAL_KEY};
use windows::Win32::UI::WindowsAndMessaging::GetMessageExtraInfo;
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

pub fn press_key(code: VkCode) {
    keyboard_event(code, KEYBD_EVENT_FLAGS(0));
}

pub fn release_key(code: VkCode) {
    keyboard_event(code, KEYBD_EVENT_FLAGS(0x0002));
}
