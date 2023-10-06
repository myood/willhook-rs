//! These tests inject programmatically keyboard events and verify if willhook's hook receive them properly.
//! It is important to keep in mind, that these injected events are going through Windows OS.
//! These injected events will appear in the context of the currently focused window.
//! Hence, it is best to invoke below tests in one-time CLI environment.
//! You may observe it by letters appearing in CLI input after test run finishes, like so:
//! PS C:\workspace\willhook> cfgklaa
//! It can become real mess especially if you run tests from GUI program

#[cfg(test)]
mod keyboard_hook_tests {
    use willhook::*;
    use willhook::event::KeyPress::*;
    use willhook::event::KeyboardKey::*;
    use willhook::event::IsSystemKeyPress::*;
    use mki::Keyboard;

    #[test]
    fn press_one_keyboard_key_blocking() {
        let h = keyboard_hook().unwrap();
        Keyboard::A.press();
        assert_eq!(h.recv(), utils::a_key_blocking(A, Down(Normal)));
    }

    #[test]
    fn press_one_keyboard_key() {
        Keyboard::A.press();

        let h = keyboard_hook().unwrap();
        assert!(h.try_recv().is_err());

        Keyboard::A.press();

        assert_eq!(h.try_recv(), utils::a_key(A, Down(Normal)));
        assert!(h.try_recv().is_err());
    }    
    
    #[test]
    fn release_one_keyboard_key() {
        Keyboard::B.release();
    
        let h = keyboard_hook().unwrap();
        assert!(h.try_recv().is_err());

        Keyboard::B.release();

        assert_eq!(h.try_recv(), utils::a_key(B, Up(Normal)));
        assert!(h.try_recv().is_err());
    }
    
    #[test]
    fn click_one_keyboard_key() {
        let h = keyboard_hook().unwrap();
        assert!(h.try_recv().is_err());

        Keyboard::C.click();

        assert_eq!(h.try_recv(), utils::a_key(C, Down(Normal)));
        assert_eq!(h.try_recv(), utils::a_key(C, Up(Normal)));
        assert!(h.try_recv().is_err());
    }

    #[test]
    fn click_one_system_key() {
        let h = keyboard_hook().unwrap();
        assert!(h.try_recv().is_err());

        Keyboard::LeftAlt.press();
        Keyboard::D.click();
        Keyboard::LeftAlt.release();

        assert_eq!(h.try_recv(), utils::a_key(LeftAlt, Down(System)));
        assert_eq!(h.try_recv(), utils::a_key(D, Down(System)));
        assert_eq!(h.try_recv(), utils::a_key(D, Up(System)));
        assert_eq!(h.try_recv(), utils::a_key(LeftAlt, Up(Normal)));
        assert!(h.try_recv().is_err());
    }

    #[test]
    fn click_one_system_key_receive_interleaving() {
        let h = keyboard_hook().unwrap();
        assert!(h.try_recv().is_err());

        Keyboard::LeftAlt.press();
        assert_eq!(h.try_recv(), utils::a_key(LeftAlt, Down(System)));
        assert!(h.try_recv().is_err());

        Keyboard::E.press();
        Keyboard::E.release();
        assert_eq!(h.try_recv(), utils::a_key(E, Down(System)));
        assert_eq!(h.try_recv(), utils::a_key(E, Up(System)));
        assert!(h.try_recv().is_err());

        Keyboard::LeftAlt.release();
        assert_eq!(h.try_recv(), utils::a_key(LeftAlt, Up(Normal)));
        assert!(h.try_recv().is_err());

        assert!(h.try_recv().is_err());
    }

    #[test]
    fn multiple_keys() {        
        Keyboard::F.press();
        Keyboard::G.click();
        let h = keyboard_hook().unwrap();
        assert!(h.try_recv().is_err());

        Keyboard::LeftAlt.press();
        assert_eq!(h.try_recv(), utils::a_key(LeftAlt, Down(System)));
        assert!(h.try_recv().is_err());

        Keyboard::H.press();
        Keyboard::I.click();
        Keyboard::H.release();
        assert_eq!(h.try_recv(), utils::a_key(H, Down(System)));
        assert_eq!(h.try_recv(), utils::a_key(I, Down(System)));
        assert_eq!(h.try_recv(), utils::a_key(I, Up(System)));
        assert_eq!(h.try_recv(), utils::a_key(H, Up(System)));
        assert!(h.try_recv().is_err());

        Keyboard::LeftAlt.release();
        assert_eq!(h.try_recv(), utils::a_key(LeftAlt, Up(Normal)));
        assert!(h.try_recv().is_err());
        
        Keyboard::J.release();
        Keyboard::K.click();
        Keyboard::L.click();        
        assert_eq!(h.try_recv(), utils::a_key(J, Up(Normal)));
        assert_eq!(h.try_recv(), utils::a_key(K, Down(Normal)));
        assert_eq!(h.try_recv(), utils::a_key(K, Up(Normal)));
        assert_eq!(h.try_recv(), utils::a_key(L, Down(Normal)));
        assert_eq!(h.try_recv(), utils::a_key(L, Up(Normal)));
        assert!(h.try_recv().is_err());
    }

    #[test]
    fn multiple_hooks_test() {
        {
            Keyboard::F.press();
            Keyboard::G.click();
            let h1 = keyboard_hook().unwrap();
            assert!(h1.try_recv().is_err());

            Keyboard::LeftAlt.press();
            assert_eq!(h1.try_recv(), utils::a_key(LeftAlt, Down(System)));
            assert!(h1.try_recv().is_err());

            // These events are received by h1
            Keyboard::H.press();
            Keyboard::H.release();
        }

        {
            // But H press/release should not be received by h2
            let h2 = keyboard_hook().unwrap();
            assert!(h2.try_recv().is_err());

            Keyboard::I.click();
            assert_eq!(h2.try_recv(), utils::a_key(I, Down(System)));
            assert_eq!(h2.try_recv(), utils::a_key(I, Up(System)));
            assert!(h2.try_recv().is_err());

            Keyboard::LeftAlt.release();
            assert_eq!(h2.try_recv(), utils::a_key(LeftAlt, Up(Normal)));
            assert!(h2.try_recv().is_err());

            // This J release is captured by h2, but will not be seen by h3
            Keyboard::J.release();
        }
        let h3 = keyboard_hook().unwrap();
        assert!(h3.try_recv().is_err());
        
        Keyboard::K.click();
        Keyboard::L.click();        
        assert_eq!(h3.try_recv(), utils::a_key(K, Down(Normal)));
        assert_eq!(h3.try_recv(), utils::a_key(K, Up(Normal)));
        assert_eq!(h3.try_recv(), utils::a_key(L, Down(Normal)));
        assert_eq!(h3.try_recv(), utils::a_key(L, Up(Normal)));
        assert!(h3.try_recv().is_err());
    }

    #[test]
    fn keyboard_hook_does_not_capture_mouse() {
        let h1 = keyboard_hook().unwrap();
        assert!(h1.try_recv().is_err());

        use mki::Mouse;

        Mouse::Left.click();
        Mouse::Right.click();
        Keyboard::K.click();    
        Mouse::Left.click();
        assert_eq!(h1.try_recv(), utils::a_key(K, Down(Normal)));
        assert_eq!(h1.try_recv(), utils::a_key(K, Up(Normal)));
        assert!(h1.try_recv().is_err());
    }
}