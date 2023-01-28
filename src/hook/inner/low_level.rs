
use crate::hook::event::*;
use crate::hook::inner::GLOBAL_CHANNEL;

use std::ptr::null_mut;

use winapi::{shared::{minwindef::*, windef::*}, um::winuser::{CallNextHookEx, KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT}};

pub unsafe extern "system" fn keyboard_procedure(
    code: INT,
    wm_key_code: WPARAM,
    win_hook_struct: LPARAM,
) -> LRESULT {
    // If code is less than zero OR win_hook_struct is NULL,
    // then the hook procedure
    // must pass the message to the CallNextHookEx function
    // without further processing and should return the value returned by CallNextHookEx.
    if code < 0 {
        // TODO: hhk param should be registered hook during startup
        return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code, win_hook_struct);
    }

    let kbd_hook_struct: *mut KBDLLHOOKSTRUCT = win_hook_struct as *mut _;        
    let keyboard_event = KeyboardEvent::new(wm_key_code, kbd_hook_struct);

    let _ignore_error = GLOBAL_CHANNEL.send_keyboard_event(keyboard_event).is_err();

    CallNextHookEx(null_mut() as HHOOK, code, wm_key_code, win_hook_struct)
}

pub unsafe extern "system" fn mouse_procedure(
    code: INT,
    wm_mouse_param: WPARAM,
    win_hook_struct: LPARAM,
) -> LRESULT {
    // If code is less than zero, then the hook procedure
    // must pass the message to the CallNextHookEx function
    // without further processing and should return the value returned by CallNextHookEx.
    if code < 0 {
        unsafe {
            // TODO: hhk param should be registered hook during startup
            return CallNextHookEx(null_mut() as HHOOK, code, wm_mouse_param, win_hook_struct);
        }
    }

    let mice_hook_struct: *const MSLLHOOKSTRUCT = win_hook_struct as *mut _;
    let mouse_event = MouseEvent::new(wm_mouse_param, mice_hook_struct);
    let _ignore_error = GLOBAL_CHANNEL.send_mouse_event(mouse_event).is_err();

    CallNextHookEx(null_mut() as HHOOK, code, wm_mouse_param, win_hook_struct)
}
