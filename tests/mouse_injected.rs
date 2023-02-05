//! These tests inject programmatically mouse events and verify if willhook's hook receive them properly.
//! It is important to keep in mind, that these injected events are going through Windows OS.
//! These injected events will appear in the context of the currently focused window.
//! Hence, it is best to invoke below tests in one-time CLI environment.
//! You may observe strange behavior of your mouse if real mouse is present, beware.
//! Also, these tests do not distinguish injected or real mouse events. So running the tests while using the mouse will give random failures.

#[cfg(test)]
mod mouse_hook_tests {
    use willhook::*;
    use willhook::hook::event::*;
    use willhook::hook::event::InputEvent::*;
    use willhook::hook::event::MouseButtonPress::*;
    use willhook::hook::event::MouseButton::*;
    use willhook::hook::event::MouseClick::*;
    use willhook::hook::event::IsMouseEventInjected::*;
    use willhook::hook::event::MouseEventType::*;
    use mki::Mouse;

    fn a_button(button: MouseButton, press: MouseButtonPress) -> Result<InputEvent, std::sync::mpsc::TryRecvError> {
        Ok(Mouse(MouseEvent {
                        event: Press(MousePressEvent{
                            pressed: press,
                            button: button,
                        }),
                        is_injected: Some(Injected)}))
    }

    // The MKI implementation seems to be buggy at the current version.
    // It sends incorrect mouse events.
    // These are workarounds for this, and also a timing issue.
    mod fixme {
        pub fn delay_execution() {
            // This test is a race between the thread running the test and the background hook.
            // I don't see a good way around that for now, other then sleeping and yielding the injecter thread.
            std::thread::sleep(std::time::Duration::from_millis(100));
            std::thread::yield_now();
        }

        pub fn press(m: mki::Mouse) {
            m.release();
            delay_execution();
        }

        pub fn release(m: mki::Mouse) {
            m.click();
            delay_execution();
        }

        pub fn click(m: mki::Mouse) {
            press(m);
            release(m);
        }
    }

    #[test]
    fn press_one_mouse_key() {
        fixme::press(Mouse::Left);

        let h = mouse_hook().unwrap();
        assert!(h.try_recv().is_err());

        fixme::press(Mouse::Left);

        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Down));
        assert!(h.try_recv().is_err());
    }    
    
    #[test]
    fn release_one_mouse_key() {
        fixme::release(Mouse::Left);
    
        let h = mouse_hook().unwrap();
        assert!(h.try_recv().is_err());

        fixme::release(Mouse::Left);

        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Up));
        assert!(h.try_recv().is_err());
    }
    
    #[test]
    fn click_one_mouse_key() {
        let h = mouse_hook().unwrap();
        assert!(h.try_recv().is_err());

        fixme::click(Mouse::Left);

        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Down));
        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Up));
        assert!(h.try_recv().is_err());
    }

    #[test]
    fn hold_rmb_while_pressing_lmb() {
        let h = mouse_hook().unwrap();
        assert!(h.try_recv().is_err());

        fixme::press(Mouse::Right);
        fixme::click(Mouse::Left);
        fixme::release(Mouse::Right);

        assert_eq!(h.try_recv(), a_button(Right(SingleClick), Down));
        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Down));
        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Up));
        assert_eq!(h.try_recv(), a_button(Right(SingleClick), Up));
        assert!(h.try_recv().is_err());
    }

    #[test]
    fn hold_rmb_while_pressing_lmb_interleaved_capture() {
        let h = mouse_hook().unwrap();
        assert!(h.try_recv().is_err());

        fixme::press(Mouse::Right);
        assert_eq!(h.try_recv(), a_button(Right(SingleClick), Down));
        assert!(h.try_recv().is_err());

        fixme::press(Mouse::Left);
        fixme::release(Mouse::Left);
        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Down));
        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Up));
        assert!(h.try_recv().is_err());

        fixme::release(Mouse::Right);
        assert_eq!(h.try_recv(), a_button(Right(SingleClick), Up));
        assert!(h.try_recv().is_err());

        assert!(h.try_recv().is_err());
    }

    #[test]
    fn multiple_buttons() {        
        fixme::press(Mouse::Left);
        fixme::click(Mouse::Right);
        let h = mouse_hook().unwrap();
        assert!(h.try_recv().is_err());

        fixme::press(Mouse::Left);
        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Down));
        assert!(h.try_recv().is_err());

        fixme::press(Mouse::Right);
        fixme::click(Mouse::Middle);
        fixme::release(Mouse::Left);
        assert_eq!(h.try_recv(), a_button(Right(SingleClick), Down));
        assert_eq!(h.try_recv(), a_button(Middle(SingleClick), Down));
        assert_eq!(h.try_recv(), a_button(Middle(SingleClick), Up));
        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Up));
        assert!(h.try_recv().is_err());

        fixme::release(Mouse::Extra);
        assert_eq!(h.try_recv(), a_button(X2(SingleClick), Up));
        assert!(h.try_recv().is_err());
        
        fixme::release(Mouse::Side);
        fixme::click(Mouse::Left);
        fixme::click(Mouse::Extra);
        assert_eq!(h.try_recv(), a_button(X1(SingleClick), Up));
        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Down));
        assert_eq!(h.try_recv(), a_button(Left(SingleClick), Up));
        assert_eq!(h.try_recv(), a_button(X2(SingleClick), Down));
        assert_eq!(h.try_recv(), a_button(X2(SingleClick), Up));
        assert!(h.try_recv().is_err());
    }

    #[test]
    fn multiple_hooks_test() {
        {
            fixme::press(Mouse::Left);
            fixme::click(Mouse::Left);
            let h1 = mouse_hook().unwrap();
            assert!(h1.try_recv().is_err());

            fixme::press(Mouse::Left);
            assert_eq!(h1.try_recv(), a_button(Left(SingleClick), Down));
            assert!(h1.try_recv().is_err());

            // These events are received by h1
            fixme::press(Mouse::Left);
            fixme::release(Mouse::Left);
        }

        {
            // But H press/release should not be received by h2
            let h2 = mouse_hook().unwrap();
            assert!(h2.try_recv().is_err());

            fixme::click(Mouse::Left);
            assert_eq!(h2.try_recv(), a_button(Left(SingleClick), Down));
            assert_eq!(h2.try_recv(), a_button(Left(SingleClick), Up));
            assert!(h2.try_recv().is_err());

            fixme::release(Mouse::Right);
            assert_eq!(h2.try_recv(), a_button(Right(SingleClick), Up));
            assert!(h2.try_recv().is_err());

            // This J release is captured by h2, but will not be seen by h3
            fixme::release(Mouse::Left);
        }
        let h3 = mouse_hook().unwrap();
        assert!(h3.try_recv().is_err());
        
        fixme::click(Mouse::Left);
        fixme::click(Mouse::Right);        
        assert_eq!(h3.try_recv(), a_button(Left(SingleClick), Down));
        assert_eq!(h3.try_recv(), a_button(Left(SingleClick), Up));
        assert_eq!(h3.try_recv(), a_button(Right(SingleClick), Down));
        assert_eq!(h3.try_recv(), a_button(Right(SingleClick), Up));
        assert!(h3.try_recv().is_err());
    }

    #[test]
    fn mouse_hook_does_not_capture_mouse() {
        let h1 = mouse_hook().unwrap();
        assert!(h1.try_recv().is_err());

        use mki::Keyboard;

        Keyboard::A.click();
        Keyboard::B.click();
        fixme::click(Mouse::Left);    
        Keyboard::C.click();

        fixme::delay_execution();

        assert_eq!(h1.try_recv(), a_button(Left(SingleClick), Down));
        assert_eq!(h1.try_recv(), a_button(Left(SingleClick), Up));
        assert!(h1.try_recv().is_err());
    }
}