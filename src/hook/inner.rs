pub(super) mod raw;
pub(super) mod channels;

use crate::hook::{
    KeyCode,
    inner::{raw::RawHook, channels::HookChannels}
};

use std::{
    ptr::null_mut,
    thread::JoinHandle,
    sync::{Arc, Weak, Condvar, Mutex}
};

use once_cell::sync::Lazy;

use winapi::{shared::{
    ntdef::NULL,
    minwindef::*,
    windef::*
}};
use winapi::um::{
        processthreadsapi::GetCurrentThreadId,
    winuser::{
    HOOKPROC, LPMSG,
    CallNextHookEx, SetWindowsHookExA, UnhookWindowsHookEx, GetMessageA, PostThreadMessageA,
    WM_QUIT,
    WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
    }
};

pub fn setup_mouse_hook() -> Option<Arc<InnerHook>> {
    if is_any_hook_present() {
        None
    } else {
        Some(setup_hook(
            &GLOBAL_MOUSE_HOOK,
            WH_MOUSE_LL,
            Some(low_level_keyboard_procedure),
        ))
    }
}

pub fn setup_keyboard_hook() -> Option<Arc<InnerHook>> {
    if is_any_hook_present() {
        None
    } else {
        Some(setup_hook(
            &GLOBAL_KEYBOARD_HOOK,
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_procedure),
        ))
    }
}

static GLOBAL_CHANNEL: Lazy<HookChannels> = Lazy::new(|| HookChannels::new());
static GLOBAL_KEYBOARD_HOOK: Mutex<Option<Weak<InnerHook>>> = Mutex::new(None);
static GLOBAL_MOUSE_HOOK: Mutex<Option<Weak<InnerHook>>> = Mutex::new(None);

fn is_any_hook_present() -> bool {
    is_hook_present(&GLOBAL_MOUSE_HOOK) || is_hook_present(&GLOBAL_KEYBOARD_HOOK)
}

fn is_hook_present(global: &Mutex<Option<Weak<InnerHook>>>) -> bool {
    let mut kb_hook_guard = global.lock().unwrap();
    if kb_hook_guard.is_some() {
        let weak_kb_hook = kb_hook_guard.as_mut();
        if let Some(weak_kb_hook) = weak_kb_hook {
            if weak_kb_hook.upgrade().is_some() {
                return true;
            }
        }
    }

    return false;
}

fn setup_hook(
    global: &Mutex<Option<Weak<InnerHook>>>,
    hook_id: INT,
    handler: HOOKPROC,
) -> Arc<InnerHook> {
    let mut kb_hook_guard = global.lock().unwrap();
    if kb_hook_guard.is_some() {
        let weak_kb_hook = kb_hook_guard.as_mut();
        if let Some(weak_kb_hook) = weak_kb_hook {
            if let Some(arc) = weak_kb_hook.upgrade() {
                return arc;
            }
        }
    }

    let hook = Arc::new(InnerHook::new(hook_id, handler));
    *kb_hook_guard = Some(Arc::downgrade(&hook));
    hook
}


fn send_key(kc: KeyCode) {
    let _ignore_result = GLOBAL_CHANNEL.send_key_code(kc);
}

unsafe extern "system" fn low_level_keyboard_procedure(
    code: INT,
    wm_key_code: WPARAM,
    win_hook_struct: LPARAM,
) -> LRESULT {
    // If code is less than zero, then the hook procedure
    // must pass the message to the CallNextHookEx function
    // without further processing and should return the value returned by CallNextHookEx.
    if code < 0 || !is_hook_present(&GLOBAL_KEYBOARD_HOOK) {
        unsafe {
            // TODO: hhk param should be registered hook during startup
            return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code, win_hook_struct);
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
            return CallNextHookEx(null_mut() as HHOOK, code, wm_key_code, win_hook_struct);
        },
    }

    send_key(kc);

    CallNextHookEx(null_mut() as HHOOK, code, wm_key_code, win_hook_struct)
}

pub struct InnerHook {
    hook_handle: Arc<Mutex<RawHook>>,
    thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl Drop for InnerHook {
    fn drop(&mut self) {
        let (winapi_handle, thread_id) = if let Ok(inner) = self.hook_handle.lock() {
            ((*inner).raw_handle as HHOOK, (*inner).thread_id)
        } else {
            (NULL as HHOOK, NULL as DWORD)
        };

        if winapi_handle == NULL as HHOOK {
            return;
        }
        unsafe {
            UnhookWindowsHookEx(winapi_handle);
        }

        if thread_id == NULL as DWORD {
            return;
        }

        let res;
        unsafe {
            // We don't really care about the result, because there's not much we can do about it.
            res = PostThreadMessageA(thread_id, WM_QUIT, NULL as WPARAM, NULL as LPARAM);
        }

        // We successfully sent a message to the background thread, so let's join with it.
        if res != NULL as i32 {
            // Below ridiculous chain of calls is "necessary" to move a value out of a mutex.
            // See : https://stackoverflow.com/questions/30573188/cannot-move-data-out-of-a-mutex
            if self.thread_handle.lock().unwrap().take().unwrap().join().is_err() {
                panic!("Failed to join with the thread :(");
            }
        }
    }
}

impl InnerHook {
    pub fn new(hook_id: INT, handler: HOOKPROC) -> InnerHook {
        let raw_hook = Arc::new(Mutex::new(RawHook::new()));
        let deferred_handle = raw_hook.clone();

        let is_started = Arc::new((Mutex::new(false), Condvar::new()));
        let set_started = is_started.clone();

        let install_hook = Arc::new(Mutex::new(Some(std::thread::spawn(move || {
            let hhook;
            unsafe {
                hhook = SetWindowsHookExA(hook_id, handler, NULL as HINSTANCE, NULL as DWORD);
            }

            if hhook as usize != 0usize {
                if let Ok(mut exclusive) = deferred_handle.lock() {
                    exclusive.raw_handle = hhook;
                    exclusive.thread_id = unsafe { GetCurrentThreadId() };
                }
            }

            {
                // Notify that the hook is started
                let (start_lock, start_cvar) = &*set_started;
                let mut started = start_lock.lock().unwrap();
                *started = true;
                start_cvar.notify_one();
            }

            // This call keeps the hook alive until the InnerHook is dropped.
            // GetMessageA waits for a message to this thread, blocking thread from quiting.
            // InnerHook's Drop implementation sends the message to this thread making GetMessageA return the value.
            // At the moment the message is received, the underlying low-level Windows hook is already "unhooked",
            // so we simply quit and let the InnerHook's Drop implementation join with this thread.
            let mut msg = std::mem::MaybeUninit::uninit();
            unsafe {
                GetMessageA(
                    msg.as_mut_ptr() as LPMSG,
                    -1isize as HWND,
                    NULL as UINT,
                    NULL as UINT,
                );
            }
        }))));

        {
            // Wait for the hook to start
            let (start_lock, start_cvar) = &*is_started;
            let mut started = start_lock.lock().unwrap();
            while !*started {
                started = start_cvar.wait(started).unwrap();
            }
        }

        InnerHook {
            hook_handle: raw_hook,
            thread_handle: install_hook,
        }
    }

    pub fn try_recv() -> Result<KeyCode, std::sync::mpsc::TryRecvError> {
        if is_any_hook_present() {
            GLOBAL_CHANNEL.try_recv()
        } else {
            // This actually should never happen
            Err(std::sync::mpsc::TryRecvError::Disconnected)
        }
    }
}
