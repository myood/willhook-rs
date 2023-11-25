#[cfg(feature = "serde")]
#[cfg(test)]
mod feature_test_willhook {
    use willhook::*;
    use MouseButtonPress::*;
    use willhook::InputEvent;
    use willhook::MouseButton::*;
    use willhook::MouseClick::*;
    use willhook::KeyboardKey::*;
    use willhook::IsSystemKeyPress::*;
    use mki::Keyboard;
    use mki::Mouse;
    use serde_json;

    #[test]
    pub fn serde_key() {            
        let h = willhook().unwrap();

        let key_event = utils::a_key(A, KeyPress::Up(Normal)).unwrap();
        let serialized = serde_json::to_string(&key_event);
        assert!(serialized.is_ok());
        let deserialized = serde_json::from_str::<InputEvent>(&serialized.unwrap());
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap(), key_event);
    }
}