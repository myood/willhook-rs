pub mod hook;

use hook::{HookBuilder, Hook};

/// Return the Keyboard Hook handle. For more details see [Hook] and [HookBuilder]
pub fn keyboard_hook() -> Option<Hook> {
    HookBuilder::new().with_keyboard().build()
}

/// Return the Mouse Hook handle. For more details see [Hook] and [HookBuilder]
pub fn mouse_hook() -> Option<Hook> {
    HookBuilder::new().with_mouse().build()
}

/// Return the Mouse n' Keyboard Hook handle. For more details see [Hook] and [HookBuilder]
pub fn monke_hook() -> Option<Hook> {
    HookBuilder::new().with_keyboard().with_mouse().build()
}

/// Return the Mouse n' Keyboard Hook handle. For more details see [Hook] and [HookBuilder]
pub fn mouse_and_keyboard_hook() -> Option<Hook> {
    monke_hook()
}