use monke::hook::{HookBuilder, Hook};
use monke::{mouse_hook, keyboard_hook, monke_hook};

#[test]
fn default_build_fails() {
    assert!(HookBuilder::new().build().is_none());
}

#[test]
fn build_succeeds_with_at_least_one_hook() {
    assert!(mouse_hook().is_some())
}

#[test]
fn build_fails_if_hook_exists_1() {
    let _existing = mouse_hook();
    assert!(mouse_hook().is_none());
}
#[test]
fn build_fails_if_hook_exists_2() {
    let _existing = keyboard_hook();
    assert!(keyboard_hook().is_none());
}
#[test]
fn build_fails_if_hook_exists_3() {
    let _existing = monke_hook();
    assert!(monke_hook().is_none());
}

#[test]
fn build_fails_if_different_type_of_hook_exists_1() {
    let _existing = keyboard_hook();
    assert!(mouse_hook().is_none());
}
#[test]
fn build_fails_if_different_type_of_hook_exists_2() {
    let _existing = keyboard_hook();
    assert!(monke_hook().is_none());
}
#[test]
fn build_fails_if_different_type_of_hook_exists_3() {
    let _existing = mouse_hook();
    assert!(keyboard_hook().is_none());
}
#[test]
fn build_fails_if_different_type_of_hook_exists_4() {
    let _existing = mouse_hook();
    assert!(monke_hook().is_none());
}
#[test]
fn build_fails_if_different_type_of_hook_exists_5() {
    let _existing = monke_hook();
    assert!(mouse_hook().is_none());
}
#[test]
fn build_fails_if_different_type_of_hook_exists_6() {
    let _existing = monke_hook();
    assert!(keyboard_hook().is_none());
}

#[test]
fn building_data_race() {
    let repetitions = 1000usize;
    for _ in 0..repetitions {
        use std::sync::{Arc, Barrier};
        use std::thread;
        let racers = 10000usize;
    
        let barrier = Arc::new(Barrier::new(racers));
        let mut handles = Vec::with_capacity(racers);
    
        for _ in 0..racers {
            let c = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                c.wait();
                HookBuilder::new().with_keyboard().build()
            }));
        }
    
        let some = handles.into_iter()
            .map(|jh| jh.join().unwrap())
            .filter(|o| o.is_some())
            .count();

        // No matter the number of builders, there should be only one successful build
        assert_eq!(1, some);
    }
}