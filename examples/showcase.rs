#![cfg(windows)]

use std::ptr::null_mut;
use std::thread::JoinHandle;

use winapi::shared::ntdef::NULL;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::{WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP, LPMSG, WH_KEYBOARD_LL};
use winapi::um::winuser::{GetMessageA, CallNextHookEx, SetWindowsHookExA, UnhookWindowsHookEx};
use once_cell::sync::Lazy;
use std::sync::mpsc::{channel, Receiver, Sender};

struct HookChannels {
    sender: Mutex<Sender<KeyCode>>,
    receiver: Mutex<Receiver<KeyCode>>,
}

impl HookChannels {
    fn new() -> HookChannels {
        let (s, r) = channel();
        HookChannels { sender: Mutex::new(s), receiver: Mutex::new(r) }
    }
}

static GLOBAL_CHANNEL: Lazy<HookChannels> = Lazy::new(|| { HookChannels::new() });

#[derive(Debug)]
enum KeyCode {
    Down,
    Up,
}

fn send_key(kc: KeyCode) {
    let sender = &GLOBAL_CHANNEL.sender.lock().unwrap();
    sender.send(kc);
}

pub unsafe extern "system" fn low_level_keyboard_procedure(
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

        let kc;
        match wm_key_code as u32 {
            WM_KEYDOWN => kc = KeyCode::Down,
            WM_KEYUP => kc = KeyCode::Up,
            WM_SYSKEYDOWN => kc = KeyCode::Down,
            WM_SYSKEYUP => kc = KeyCode::Up,
            _ => unsafe {
                // We don't recognize the key code. This should never happen, except something really bad is happening with the OS.
                // TODO: hhk param should be registered hook during startup
                return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code,     win_hook_struct)
            }
        }

        send_key(kc);
        
        CallNextHookEx(null_mut() as HHOOK, code, wm_key_code,     win_hook_struct)
    }

    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    struct KeyCodeChannel {
        hook: Hook
    }

    #[derive(Clone)]
    struct Hook {
        hook_handle: Arc<Mutex<UnsafeHook>>,
        thread_handle: Arc<Mutex<JoinHandle<()>>>,
    }

    impl Drop for Hook {
        fn drop(&mut self) {
            if self.get_raw_handle() == NULL as HHOOK {
                return
            }
            unsafe { UnhookWindowsHookEx(self.get_raw_handle()); }
        }
    }

    impl Hook {
        fn new() -> Hook {
            let unsafe_hook = Arc::new(Mutex::new(UnsafeHook::new()));
            let deferred_handle = unsafe_hook.clone();
            let deferred = Arc::new(Mutex::new(std::thread::spawn(move || {
                let hhook;
                unsafe {
                    hhook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(low_level_keyboard_procedure), NULL as HINSTANCE, NULL as DWORD);
                }
        
                if hhook as usize != 0usize {
                    if let Ok(mut exclusive) = deferred_handle.lock() {
                        exclusive.set(hhook);
                    }
                }

                // This call keeps the hook alive, it does not return. Apparently, takes control over this thread.
                let mut msg = std::mem::MaybeUninit::uninit();
                unsafe {
                    GetMessageA(msg.as_mut_ptr() as LPMSG, NULL as HWND, NULL as UINT, NULL as UINT);// {
                }
            })));

            Hook {
                hook_handle: unsafe_hook,
                thread_handle: deferred,
            }
        }

        fn get_raw_handle(&self) -> HHOOK {
            if let Ok(inner) = self.hook_handle.lock() {
                return (*inner).get() as HHOOK
            }
            return NULL as HHOOK
        }
    }

    struct UnsafeHook {
        raw_handle: HHOOK
    }

    impl UnsafeHook {
        fn new() -> UnsafeHook {
            UnsafeHook { raw_handle: NULL as HHOOK }
        }

        fn get(&self) -> HHOOK {
            return self.raw_handle as HHOOK
        }

        fn set(&mut self, v: HHOOK) {
            self.raw_handle = v;
        }
    }

    unsafe impl Send for UnsafeHook {}
    unsafe impl Sync for UnsafeHook {}


fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let _hook = Hook::new();

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        if let Ok(guard) = GLOBAL_CHANNEL.receiver.lock() {
            let keys_receiver = &(*guard);
            while let Ok(kc) = keys_receiver.try_recv() {
                println!("Key event: {:?}", kc);
            }
        }
        std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_millis(100));
    };
}

