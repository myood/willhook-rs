#![cfg(windows)]

use monke::hook::HookBuilder;
use std::sync::{Arc, atomic::{Ordering, AtomicBool}};

fn main() {
    let is_running = Arc::new(AtomicBool::new(true));
    let set_running = is_running.clone();

    let h = HookBuilder::new().with_keyboard().build().unwrap();

    ctrlc::set_handler(move || {
        set_running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while is_running.load(Ordering::SeqCst) {
        if let Ok(kc) = h.try_recv() {
            println!("Key event: {:?}", kc);
        }
        std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_millis(100));
    };
}
