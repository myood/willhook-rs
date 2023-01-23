#![cfg(windows)]

use std::ptr::null_mut;
use std::thread::JoinHandle;

use winapi::shared::ntdef::NULL;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::HOOKPROC;
use winapi::um::winuser::{WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP, LPMSG, WH_KEYBOARD_LL, WH_MOUSE_LL};
use winapi::um::winuser::{GetMessageA, CallNextHookEx, SetWindowsHookExA, UnhookWindowsHookEx};
use once_cell::sync::Lazy;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::sync::Weak;

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
static GLOBAL_KEYBOARD_HOOK: Mutex<Option<Weak<InnerHook>>> = Mutex::new(None);
static GLOBAL_MOUSE_HOOK: Mutex<Option<Weak<InnerHook>>> = Mutex::new(None);

struct Hook {
    keyboard_hook: Option<Arc<InnerHook>>,
    mouse_hook: Option<Arc<InnerHook>>,
}

impl Hook {
    fn try_recv(&self) -> Result<KeyCode, std::sync::mpsc::TryRecvError> {
        if let Ok(guard) = GLOBAL_CHANNEL.receiver.lock() {
            let keys_receiver = &(*guard);
            keys_receiver.try_recv()
        } else {
            Err(std::sync::mpsc::TryRecvError::Disconnected)
        }
    }
}

struct HookBuilder {
    mouse: bool,
    keyboard: bool,
}

impl HookBuilder {
    fn new() -> Self {
        Self{mouse: false, keyboard: false}
    }

    fn with_mouse(mut self) -> Self {
        self.mouse = true;
        self
    }

    fn with_keyboard(mut self) -> Self {
        self.keyboard = true;
        self
    }

    fn is_hook_present(global: &Mutex<Option<Weak<InnerHook>>>) -> bool {
        let mut kb_hook_guard = global.lock().unwrap();
        if kb_hook_guard.is_some() {
            let weak_kb_hook = kb_hook_guard.as_mut();
            if let Some(weak_kb_hook) = weak_kb_hook {
                if let Some(arc) = weak_kb_hook.upgrade() {
                    return true
                }
            }
        }
        return false
    }

    fn setup_hook(global: &Mutex<Option<Weak<InnerHook>>>, hook_id: INT, handler: HOOKPROC) -> Arc<InnerHook> {
        let mut kb_hook_guard = global.lock().unwrap();
        if kb_hook_guard.is_some() {
            let weak_kb_hook = kb_hook_guard.as_mut();
            if let Some(weak_kb_hook) = weak_kb_hook {
                if let Some(arc) = weak_kb_hook.upgrade() {
                    return arc
                }
            }
        }

        let hook = Arc::new(InnerHook::new(hook_id, handler));
        *kb_hook_guard = Some(Arc::downgrade(&hook));
        hook
    }

    fn setup_mouse_hook() -> Arc<InnerHook> {
        Self::setup_hook(&GLOBAL_MOUSE_HOOK, WH_MOUSE_LL, Some(low_level_keyboard_procedure))
    }

    fn setup_keyboard_hook() -> Arc<InnerHook> {
        Self::setup_hook(&GLOBAL_KEYBOARD_HOOK, WH_KEYBOARD_LL, Some(low_level_keyboard_procedure))
    }

    fn build(self) -> Option<Hook> {
        if Self::is_hook_present(&GLOBAL_KEYBOARD_HOOK) || Self::is_hook_present(&GLOBAL_MOUSE_HOOK) {
            return None
        }
        Some(Hook {
            keyboard_hook: {
                if self.keyboard {
                    Some(Self::setup_keyboard_hook())
                } else {
                    None
                }
            },
            mouse_hook: {
                if self.mouse {
                    Some(Self::setup_mouse_hook())
                } else {
                    None
                }
            }
        })
    }
}

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

    use std::sync::Mutex;
    use std::sync::Condvar;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    #[derive(Clone)]
    struct InnerHook {
        hook_handle: Arc<Mutex<UnsafeHook>>,
        _thread_handle: Arc<Mutex<JoinHandle<()>>>,
    }

    impl Drop for InnerHook {
        fn drop(&mut self) {
            let winapi_handle: HHOOK = if let Ok(inner) = self.hook_handle.lock() {
                (*inner).get() as HHOOK
            } else {
                NULL as HHOOK
            };
            
            if winapi_handle == NULL as HHOOK {
                return
            }
            unsafe { UnhookWindowsHookEx(winapi_handle); }
        }
    }

    impl InnerHook {
        fn new(hook_id: INT, handler: HOOKPROC) -> InnerHook {
            let unsafe_hook = Arc::new(Mutex::new(UnsafeHook::new()));
            let deferred_handle = unsafe_hook.clone();

            let is_started = Arc::new((Mutex::new(false), Condvar::new()));
            let set_started = is_started.clone();

            let deferred = Arc::new(Mutex::new(std::thread::spawn(move || {
                let hhook;
                unsafe {
                    hhook = SetWindowsHookExA(hook_id, handler, NULL as HINSTANCE, NULL as DWORD);
                }
        
                if hhook as usize != 0usize {
                    if let Ok(mut exclusive) = deferred_handle.lock() {
                        exclusive.set(hhook);
                    }
                }

                {  // Notify that the hook is started
                    let (start_lock, start_cvar) = &*set_started;
                    let mut started = start_lock.lock().unwrap();
                    *started = true;
                    start_cvar.notify_one();
                }

                // This call keeps the hook alive, it does not return. Apparently, takes control over this thread.
                let mut msg = std::mem::MaybeUninit::uninit();
                unsafe {
                    GetMessageA(msg.as_mut_ptr() as LPMSG, NULL as HWND, NULL as UINT, NULL as UINT);// {
                }
            })));

            {  // Wait for the hook to start
                let (start_lock, start_cvar) = &*is_started;
                let mut started = start_lock.lock().unwrap();
                while !*started {
                    started = start_cvar.wait(started).unwrap();
                }
            }

            InnerHook {
                hook_handle: unsafe_hook,
                _thread_handle: deferred,
            }
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

    fn black_box<T>(dummy: T) -> T {
        unsafe {
            std::ptr::read_volatile(&dummy)
        }
    }

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
        let h = h.unwrap();
        let h2 = HookBuilder::new().with_mouse().build();
        assert!(h2.is_none());
        let h3 = HookBuilder::new().with_keyboard().with_mouse().build();
        assert!(h3.is_none());
    }

    let h = HookBuilder::new().with_keyboard().build().unwrap();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        if let Ok(kc) = h.try_recv() {
            println!("Key event: {:?}", kc);
        }
        std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_millis(100));
    };
}

