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

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    {
        let h = HookBuilder::new().with_keyboard().build();
        assert!(h.is_some());
    }

    let repetitions = 10usize;
    for k in 0..repetitions {
        println!("Repetition: {}", k);
        use std::sync::{Arc, Barrier};
        use std::thread;
        use std::sync::mpsc::channel;

        let (s, r) = channel();

        let racers = 1000usize;
    
        let barrier = Arc::new(Barrier::new(racers));
        let mut handles = Vec::with_capacity(racers);
    
        for i in 0..racers {
            println!("Racer: {}", i);
            let c = Arc::clone(&barrier);
            let s2 = s.clone();
            handles.push(thread::spawn(move || {
                c.wait();
                let h = HookBuilder::new().with_keyboard().build();
                if h.is_some() {
                    println!("Some!");
                }
                s2.send(h);
            }));
        }

        drop(s);

        println!("Joining");
        let _ = handles.into_iter()
            .for_each(|jh| { jh.join(); } );
        println!("Joined");

        let mut count = 0;
        while let Ok(v) = r.recv() {
            if v.is_some() {
                count += 1;
            }
        }

        // No matter the number of builders, there should be only one successful build
        assert_eq!(1, count);
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


// TESTS:
// create kb hook, then mouse hook, then create both hooks
// check if all threads are closed (how??)

