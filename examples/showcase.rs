#![cfg(windows)]

use willhook::willhook;
use std::sync::{Arc, atomic::{Ordering, AtomicBool}};

fn main() {
    let is_running = Arc::new(AtomicBool::new(true));
    let set_running = is_running.clone();

    let h = willhook().unwrap();

    ctrlc::set_handler(move || {
        set_running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while is_running.load(Ordering::SeqCst) {
        if let Ok(ie) = h.try_recv() {
            match ie {
                willhook::event::InputEvent::Keyboard(ke) => println!("{:?}", ke),
                willhook::event::InputEvent::Mouse(me) => println!("{:?}", me),
                _ => println!("Input event: {:?}", ie),
            }
        } else {
            std::thread::yield_now();   
        }
    };
}
