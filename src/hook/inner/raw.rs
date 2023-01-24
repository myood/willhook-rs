use winapi::shared::windef::HHOOK;
use winapi::shared::ntdef::NULL;

pub struct RawHook {
    raw_handle: HHOOK,
}

impl RawHook {
    pub fn new() -> RawHook {
        RawHook {
            raw_handle: NULL as HHOOK,
        }
    }

    pub fn get(&self) -> HHOOK {
        return self.raw_handle as HHOOK;
    }

    pub fn set(&mut self, v: HHOOK) {
        self.raw_handle = v;
    }
}

unsafe impl Send for RawHook {}
unsafe impl Sync for RawHook {}
