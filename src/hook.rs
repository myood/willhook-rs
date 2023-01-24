pub(super) mod inner;

use crate::hook::inner::{setup_keyboard_hook, setup_mouse_hook, InnerHook};
use std::sync::Arc;

pub struct Hook {
    keyboard_hook: Option<Arc<InnerHook>>,
    mouse_hook: Option<Arc<InnerHook>>,
}

impl Hook {
    pub fn try_recv(&self) -> Result<KeyCode, std::sync::mpsc::TryRecvError> {
        InnerHook::try_recv()
    }
}

pub struct HookBuilder {
    mouse: bool,
    keyboard: bool,
}

impl HookBuilder {
    pub fn new() -> Self {
        Self {
            mouse: false,
            keyboard: false,
        }
    }

    pub fn with_mouse(mut self) -> Self {
        self.mouse = true;
        self
    }

    pub fn with_keyboard(mut self) -> Self {
        self.keyboard = true;
        self
    }

    pub fn build(self) -> Option<Hook> {
        let kb_hook = setup_keyboard_hook();
        let m_hook = setup_mouse_hook();

        if kb_hook.is_none() || m_hook.is_none() {
            None
        } else {
            Some(Hook {
                keyboard_hook: kb_hook,
                mouse_hook: m_hook,
            })
        }
    }
}

#[derive(Debug)]
pub enum KeyCode {
    Down,
    Up,
}

