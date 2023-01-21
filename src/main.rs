#![cfg(windows)]

use std::ptr::null_mut;

use winapi::shared::ntdef::NULL;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::{MSG, LPMSG, WH_KEYBOARD_LL};
use winapi::um::winuser::{GetMessageA, TranslateMessage, DispatchMessageA, CallNextHookEx, SetWindowsHookExA, UnhookWindowsHookEx};

#[derive(Debug)]
enum KeyCode {
    Down,
    Up,
}

fn process_key(kc: KeyCode) {
    println!("Processing key: {:?}", kc);
}

pub unsafe extern "system" fn low_level_keyboard_procedure(
    code: INT,
    wm_key_code: WPARAM,
    win_hook_struct: LPARAM,
    ) -> LRESULT {
        println!("Hook invoked!");

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
            WM_KEYDOWN => { process_key(KeyCode::Down); }
            WM_KEYUP => { process_key(KeyCode::Up); }
            WM_SYSKEYDOWN => { process_key(KeyCode::Down); }
            WM_SYSKEYUP => { process_key(KeyCode::Up); }
            _ => unsafe {
                // We don't recognize the key code. This should never happen, except something really bad is happening with the OS.
                // TODO: hhk param should be registered hook during startup
                return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code,     win_hook_struct)
            }
        }
        
        0 as LRESULT
    }
    
fn main() {
    unsafe {
        let hmod = NULL as HINSTANCE;
        let thread_id = 0 as DWORD;
        let hhook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(low_level_keyboard_procedure), hmod, thread_id);
        println!("HHOOK: {:?}, GetLastError: {}", hhook, GetLastError());

        // This while loop keeps the hook
        let mut msg = std::mem::MaybeUninit::uninit();
        while FALSE == GetMessageA(msg.as_mut_ptr() as LPMSG, NULL as HWND, NULL as UINT, NULL as UINT) {
            TranslateMessage(msg.as_ptr());
            DispatchMessageA(msg.as_ptr());
        }

        if hhook != NULL as HHOOK {
            println!("OK! Destroying the hook...");
            if UnhookWindowsHookEx(hhook) != 0 {
                println!("OK! All cleaned up!");
            }
        }
    }
}
