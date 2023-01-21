#![cfg(windows)]

use std::ptr::null_mut;

use winapi::shared::ntdef::NULL;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::{WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP, LPMSG, WH_KEYBOARD_LL};
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

        match wm_key_code as u32 {
            WM_KEYDOWN => { 
                process_key(KeyCode::Down);
            }
            WM_KEYUP => {
                process_key(KeyCode::Up);
            }
            WM_SYSKEYDOWN => {
                process_key(KeyCode::Down);
            }
            WM_SYSKEYUP => {
                process_key(KeyCode::Up);
            }
            _ => unsafe {
                // We don't recognize the key code. This should never happen, except something really bad is happening with the OS.
                // TODO: hhk param should be registered hook during startup
                return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code,     win_hook_struct)
            }
        }
        
        0 as LRESULT
    }

    struct Droppable {
        name: &'static str,
    }
    
    // This trivial implementation of `drop` adds a print to console.
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    #[derive(Clone)]
    struct Hook {
        raw_handle: Arc<Mutex<UnsafeHook>>,
    }

    impl Hook {
        fn new() -> Hook {
            Hook {
                raw_handle: Arc::new(Mutex::new(UnsafeHook::new())),
            }
        }

        fn set(&mut self, v: HHOOK) {
            if let Ok(mut inner) = self.raw_handle.lock() {
                inner.set(v);
            }
        }

        fn get(&self) -> HHOOK {
            if let Ok(inner) = self.raw_handle.lock() {
                return (*inner).get() as HHOOK
            }
            return NULL as HHOOK
        }
    }

    unsafe impl Send for Hook {}
    unsafe impl Sync for Hook {}

    struct UnsafeHook {
        raw_handle: HHOOK
    }

    impl UnsafeHook {
        fn new() -> UnsafeHook {
            UnsafeHook { raw_handle: NULL as HHOOK }
        }

        fn set(&mut self, v: HHOOK) {
            self.raw_handle = v;
        }

        fn get(&self) -> HHOOK {
            return self.raw_handle as HHOOK
        }
    }

    unsafe impl Send for UnsafeHook {}
    unsafe impl Sync for UnsafeHook {}


fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let mut hook = Hook::new();
    let h = hook.clone();
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let _ = Droppable{name: "main"};

    let hook_thread = std::thread::spawn(move || {
        let hmod = NULL as HINSTANCE;
        let thread_id = 0 as DWORD;
        let hhook;
        unsafe {
            hhook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(low_level_keyboard_procedure), hmod, thread_id);
            println!("HHOOK: {:?}, GetLastError: {}", hhook, GetLastError());
        }

        if hhook as usize != 0usize {
            hook.set(hhook);
        }

        // This call keeps the hook alive, it does not return. Apparently, takes control over this thread.
        let mut msg = std::mem::MaybeUninit::uninit();
        unsafe {
            GetMessageA(msg.as_mut_ptr() as LPMSG, NULL as HWND, NULL as UINT, NULL as UINT);// {
        }
    });

    while !hook_thread.is_finished() && running.load(Ordering::SeqCst) {
        std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_millis(100));
    };
    let handle: HHOOK = h.get();
    unsafe {
        println!("HHOOK: {:?}, GetLastError: {}", handle as u32, GetLastError());
    }
    if handle != NULL as HHOOK {
        println!("OK! Destroying the hook...");
        unsafe {
            if UnhookWindowsHookEx(handle) != 0 {
                println!("OK! All cleaned up!");
            }
        }
    }

    println!("Outside loop");
}

