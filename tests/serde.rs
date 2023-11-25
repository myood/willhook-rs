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
    use serde_json;

    pub fn validate_serde(ie: InputEvent)
    {
        let serialized = serde_json::to_string(&ie);
        assert!(serialized.is_ok());
        let deserialized = serde_json::from_str::<InputEvent>(&serialized.unwrap());
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap(), ie);
    }

    #[test]
    pub fn serde_key() {            
        validate_serde(utils::a_key(A, KeyPress::Up(Normal)).unwrap());
    }

    #[test]
    pub fn serde_mouse_button() {            
        validate_serde(utils::a_button(Left(SingleClick), Up).unwrap());
    }

    #[test]
    pub fn serde_mouse_move() {
        let (new_x, new_y) = (100, 200);
        validate_serde(utils::a_move(new_x, new_y).unwrap());
    }
}