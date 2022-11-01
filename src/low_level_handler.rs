use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, DispatchMessageA, GetMessageA, HHOOK, KBDLLHOOKSTRUCT, MSG, SetWindowsHookExA, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL};
pub(crate) type VkCode = u16;
pub trait EventHandler {
    fn key_pressed(&mut self, code: VkCode) -> bool;
    fn key_released(&mut self, code: VkCode) -> bool;
}

static mut HANDLER: Option<Box<dyn EventHandler>> = None;

unsafe extern "system" fn h_func(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let data: KBDLLHOOKSTRUCT = *(lparam.0 as *const KBDLLHOOKSTRUCT);
    let result: bool;
    if data.flags.0 & 0x80 != 0 {
        result = HANDLER.as_mut().unwrap().key_released(data.vkCode as VkCode);
    } else {
        result = HANDLER.as_mut().unwrap().key_pressed(data.vkCode as VkCode);
    }
    if result {
        LRESULT(1)
    } else {
        CallNextHookEx(HHOOK(0), code, wparam, lparam)
    }
}

pub fn run(handler: Box<dyn EventHandler>) {
    unsafe {
        HANDLER = Some(handler);
        let handler_function: unsafe extern "system" fn(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT = h_func;
        let hook_result: windows::core::Result<HHOOK> = SetWindowsHookExA(WH_KEYBOARD_LL, Option::from(handler_function), HINSTANCE(0), 0);
        match hook_result {
            Ok(hook) => {
                let msg = 0 as *mut MSG;
                while GetMessageA(msg, HWND(0), 0, 0).0 != 0 {
                    TranslateMessage(msg);
                    DispatchMessageA(msg);
                }
                UnhookWindowsHookEx(hook);
            }
            Err(_) => panic!("Unable to create a hook!")
        }
        let mut old_context: Option<Box<dyn EventHandler>> = None;
        std::mem::swap(&mut old_context, &mut HANDLER);
        drop(old_context);
    }
}