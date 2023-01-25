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

use winapi::shared::{
    ntdef::NULL,
    minwindef::*,
    windef::*
};
use winapi::um::winuser::{
    HOOKPROC, LPMSG,
    CallNextHookEx, GetMessageA, SetWindowsHookExA, UnhookWindowsHookEx,
    WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

static GLOBAL_CHANNEL: Lazy<HookChannels> = Lazy::new(|| HookChannels::new());
static GLOBAL_KEYBOARD_HOOK: Mutex<Option<Weak<InnerHook>>> = Mutex::new(None);
static GLOBAL_MOUSE_HOOK: Mutex<Option<Weak<InnerHook>>> = Mutex::new(None);

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

pub fn setup_mouse_hook() -> Option<Arc<InnerHook>> {
    if is_hook_present(&GLOBAL_MOUSE_HOOK) {
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
    if is_hook_present(&GLOBAL_KEYBOARD_HOOK) {
        None
    } else {
        Some(setup_hook(
            &GLOBAL_KEYBOARD_HOOK,
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_procedure),
        ))
    }
}


fn send_key(kc: KeyCode) {
    GLOBAL_CHANNEL.send_key_code(kc);
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



#[derive(Clone)]
pub struct InnerHook {
    hook_handle: Arc<Mutex<RawHook>>,
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
            return;
        }
        unsafe {
            UnhookWindowsHookEx(winapi_handle);
        }
    }
}

impl InnerHook {
    pub fn new(hook_id: INT, handler: HOOKPROC) -> InnerHook {
        let raw_hook = Arc::new(Mutex::new(RawHook::new()));
        let deferred_handle = raw_hook.clone();

        let is_started = Arc::new((Mutex::new(false), Condvar::new()));
        let set_started = is_started.clone();

        let install_hook = Arc::new(Mutex::new(std::thread::spawn(move || {
            let hhook;
            unsafe {
                hhook = SetWindowsHookExA(hook_id, handler, NULL as HINSTANCE, NULL as DWORD);
            }

            if hhook as usize != 0usize {
                if let Ok(mut exclusive) = deferred_handle.lock() {
                    exclusive.set(hhook);
                }
            }

            {
                // Notify that the hook is started
                let (start_lock, start_cvar) = &*set_started;
                let mut started = start_lock.lock().unwrap();
                *started = true;
                start_cvar.notify_one();
            }

            // This call keeps the hook alive, it does not return. Apparently, takes control over this thread.
            let mut msg = std::mem::MaybeUninit::uninit();
            unsafe {
                GetMessageA(
                    msg.as_mut_ptr() as LPMSG,
                    NULL as HWND,
                    NULL as UINT,
                    NULL as UINT,
                );
            }
        })));

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
            _thread_handle: install_hook,
        }
    }

    pub fn try_recv() -> Result<KeyCode, std::sync::mpsc::TryRecvError> {
        if !is_hook_present(&GLOBAL_MOUSE_HOOK) && !is_hook_present(&GLOBAL_KEYBOARD_HOOK) {
            Err(std::sync::mpsc::TryRecvError::Disconnected)
        } else {
            GLOBAL_CHANNEL.try_recv()
        }
    }
}
