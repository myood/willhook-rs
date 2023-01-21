#![cfg(windows)]

use std::ptr::null_mut;

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::CallNextHookEx;

enum KeyCode {
    Down,
    Up,
};

fn process_key(kc: KeyCode) {

}

fn LowLevelKeyboardProc(
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
                return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code,     win_hook_struct)
            }
        }

        match wm_key_code {
            WM_KEYDOWN => process_key(KeyCode::Down),
            WM_KEYUP => process_key(KeyCode::Up),
            WM_SYSKEYDOWN => process_key(KeyCode::Down),
            WM_SYSKEYUP => process_key(KeyCode::Up),
            _ => unsafe {
                // We don't recognize the key code. This should never happen, except something really bad is happening with the OS.
                // TODO: hhk param should be registered hook during startup
                return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code,     win_hook_struct)
            }
        }
        
        0 as LRESULT
    }
    
fn main() {
    println!("Hello, world!");
}
