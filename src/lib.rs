pub mod hook;

use hook::{HookBuilder, Hook};

pub fn keyboard_hook() -> Option<Hook> {
    HookBuilder::new().with_keyboard().build()
}

pub fn mouse_hook() -> Option<Hook> {
    HookBuilder::new().with_mouse().build()
}

pub fn monke_hook() -> Option<Hook> {
    HookBuilder::new().with_keyboard().with_mouse().build()
}

pub fn mouse_and_keyboard_hook() -> Option<Hook> {
    monke_hook()
}