#[cfg(test)]
mod willhook {
    use willhook::*;
    use willhook::hook::event::*;
    use willhook::hook::event::MouseButtonPress::*;
    use willhook::hook::event::MouseButton::*;
    use willhook::hook::event::MouseClick::*;
    use willhook::hook::event::KeyboardKey::*;
    use mki::Keyboard;
    use mki::Mouse;

    #[test]
    pub fn mixed_mouse_inputs() {            
        let h = willhook().unwrap();

        utils::fixme::vertical_wheel_backward();
        assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Backward));
        assert!(h.try_recv().is_err());

        Keyboard::A.click();
        utils::fixme::vertical_wheel_backward();
        utils::fixme::move_by(10, 15);
        utils::fixme::horizontal_wheel_forward();
        Keyboard::B.click();
        utils::fixme::click(Mouse::Middle);

        assert_eq!(h.try_recv(), utils::a_key(A, KeyPress::Down(false)));
        assert_eq!(h.try_recv(), utils::a_key(A, KeyPress::Up(false)));
        assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Backward));
        assert!(utils::is_mouse_move(h.try_recv()));
        assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Horizontal, MouseWheelDirection::Forward));
        assert_eq!(h.try_recv(), utils::a_key(B, KeyPress::Down(false)));
        assert_eq!(h.try_recv(), utils::a_key(B, KeyPress::Up(false)));
        assert_eq!(h.try_recv(), utils::a_button(Middle(SingleClick), Down));
        assert_eq!(h.try_recv(), utils::a_button(Middle(SingleClick), Up));
        assert!(h.try_recv().is_err());
    }
}