#![cfg(windows)]

use monke::hook::HookBuilder;
use std::sync::{Arc, atomic::{Ordering, AtomicBool}};

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

