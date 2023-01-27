
use crate::hook::event::*;
use crate::hook::inner::GLOBAL_CHANNEL;

use std::ptr::null_mut;

use winapi::{shared::{
    minwindef::*,
    windef::*
}, um::winuser::KBDLLHOOKSTRUCT, ctypes::c_void};
use winapi::um::winuser::{
    CallNextHookEx,
    WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

pub unsafe extern "system" fn keyboard_procedure(
    code: INT,
    wm_key_code: WPARAM,
    win_hook_struct: LPARAM,
) -> LRESULT {
    // If code is less than zero OR win_hook_struct is NULL,
    // then the hook procedure
    // must pass the message to the CallNextHookEx function
    // without further processing and should return the value returned by CallNextHookEx.
    if code < 0 || win_hook_struct == 0 {
        unsafe {
            // TODO: hhk param should be registered hook during startup
            return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code, win_hook_struct);
        }
    }

    let kc;
    match wm_key_code as u32 {
        WM_KEYDOWN => kc = KeyPress::Down,
        WM_KEYUP => kc = KeyPress::Up,
        WM_SYSKEYDOWN => kc = KeyPress::Down,
        WM_SYSKEYUP => kc = KeyPress::Up,
        _ => unsafe {
            // We don't recognize the key code. This should never happen, except something really bad is happening with the OS.
            // TODO: hhk param should be registered hook during startup
            return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code, win_hook_struct);
        },
    }

    let keyboard_event;
    unsafe {
        let kbd_hook_struct: *mut KBDLLHOOKSTRUCT = win_hook_struct as *mut _;
        
        keyboard_event = if kbd_hook_struct.is_null() {
            KeyboardEvent{
                pressed: KeyPress::from(wm_key_code),
                key: None,
                is_virtual: None,
            }
        } else {
            KeyboardEvent{
                pressed: KeyPress::from(wm_key_code),
                key: Some(KeyboardKey::from((*kbd_hook_struct).vkCode)),
                is_virtual: Some(IsInjected::from((*kbd_hook_struct).flags)),
            }
        };
    }

    let _ignore_error = GLOBAL_CHANNEL.send_keyboard_event(keyboard_event).is_err();

    CallNextHookEx(null_mut() as HHOOK, code, wm_key_code, win_hook_struct)
}

pub unsafe extern "system" fn mouse_procedure(
    code: INT,
    wm_key_code: WPARAM,
    win_hook_struct: LPARAM,
) -> LRESULT {
    // If code is less than zero, then the hook procedure
    // must pass the message to the CallNextHookEx function
    // without further processing and should return the value returned by CallNextHookEx.
    if code < 0 {
        unsafe {
            // TODO: hhk param should be registered hook during startup
            return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code, win_hook_struct);
        }
    }

    CallNextHookEx(null_mut() as HHOOK, code, wm_key_code, win_hook_struct)
}
