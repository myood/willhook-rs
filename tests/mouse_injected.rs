//! These tests inject programmatically mouse events and verify if willhook's hook receive them properly.
//! It is important to keep in mind, that these injected events are going through Windows OS.
//! These injected events will appear in the context of the currently focused window.
//! Hence, it is best to invoke below tests in one-time CLI environment.
//! You may observe strange behavior of your mouse if real mouse is present, beware.
//! Also, these tests do not distinguish injected or real mouse events. So running the tests while using the mouse will give random failures.

#[cfg(test)]
mod mouse_hook_tests {
    use willhook::*;
    use willhook::event::MouseButtonPress::*;
    use willhook::event::MouseButton::*;
    use willhook::event::MouseClick::*;
    use mki::{Keyboard, Mouse};

    mod mouse_clicks {
        use crate::mouse_hook_tests::*;

        #[test]
        fn press_one_mouse_key() {
            utils::fixme::press(Mouse::Left);

            let h = mouse_hook().unwrap();
            assert!(h.try_recv().is_err());

            utils::fixme::press(Mouse::Left);

            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Down));
            assert!(h.try_recv().is_err());
        }    
        
        #[test]
        fn release_one_mouse_key() {
            utils::fixme::release(Mouse::Left);
        
            let h = mouse_hook().unwrap();
            assert!(h.try_recv().is_err());

            utils::fixme::release(Mouse::Left);

            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Up));
            assert!(h.try_recv().is_err());
        }
        
        #[test]
        fn click_one_mouse_key() {
            let h = mouse_hook().unwrap();
            assert!(h.try_recv().is_err());

            utils::fixme::click(Mouse::Left);

            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Down));
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Up));
            assert!(h.try_recv().is_err());
        }

        #[test]
        fn hold_rmb_while_pressing_lmb() {
            let h = mouse_hook().unwrap();
            assert!(h.try_recv().is_err());

            utils::fixme::press(Mouse::Right);
            utils::fixme::click(Mouse::Left);
            utils::fixme::release(Mouse::Right);

            assert_eq!(h.try_recv(), utils::a_button(Right(SingleClick), Down));
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Down));
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Up));
            assert_eq!(h.try_recv(), utils::a_button(Right(SingleClick), Up));
            assert!(h.try_recv().is_err());
        }

        #[test]
        fn hold_rmb_while_pressing_lmb_interleaved_capture() {
            let h = mouse_hook().unwrap();
            assert!(h.try_recv().is_err());

            utils::fixme::press(Mouse::Right);
            assert_eq!(h.try_recv(), utils::a_button(Right(SingleClick), Down));
            assert!(h.try_recv().is_err());

            utils::fixme::press(Mouse::Left);
            utils::fixme::release(Mouse::Left);
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Down));
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Up));
            assert!(h.try_recv().is_err());

            utils::fixme::release(Mouse::Right);
            assert_eq!(h.try_recv(), utils::a_button(Right(SingleClick), Up));
            assert!(h.try_recv().is_err());

            assert!(h.try_recv().is_err());
        }

        #[test]
        fn multiple_buttons() {        
            utils::fixme::press(Mouse::Left);
            utils::fixme::click(Mouse::Right);
            let h = mouse_hook().unwrap();
            assert!(h.try_recv().is_err());

            utils::fixme::press(Mouse::Left);
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Down));
            assert!(h.try_recv().is_err());

            utils::fixme::press(Mouse::Right);
            utils::fixme::click(Mouse::Middle);
            utils::fixme::release(Mouse::Left);
            assert_eq!(h.try_recv(), utils::a_button(Right(SingleClick), Down));
            assert_eq!(h.try_recv(), utils::a_button(Middle(SingleClick), Down));
            assert_eq!(h.try_recv(), utils::a_button(Middle(SingleClick), Up));
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Up));
            assert!(h.try_recv().is_err());

            utils::fixme::release(Mouse::Extra);
            assert_eq!(h.try_recv(), utils::a_button(X2(SingleClick), Up));
            assert!(h.try_recv().is_err());
            
            utils::fixme::release(Mouse::Side);
            utils::fixme::click(Mouse::Left);
            utils::fixme::click(Mouse::Extra);
            assert_eq!(h.try_recv(), utils::a_button(X1(SingleClick), Up));
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Down));
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Up));
            assert_eq!(h.try_recv(), utils::a_button(X2(SingleClick), Down));
            assert_eq!(h.try_recv(), utils::a_button(X2(SingleClick), Up));
            assert!(h.try_recv().is_err());
        }

        #[test]
        fn multiple_hooks_test() {
            {
                utils::fixme::press(Mouse::Left);
                utils::fixme::click(Mouse::Left);
                let h1 = mouse_hook().unwrap();
                assert!(h1.try_recv().is_err());

                utils::fixme::press(Mouse::Left);
                assert_eq!(h1.try_recv(), utils::a_button(Left(SingleClick), Down));
                assert!(h1.try_recv().is_err());

                // These events are received by h1
                utils::fixme::press(Mouse::Left);
                utils::fixme::release(Mouse::Left);
            }

            {
                // But H press/release should not be received by h2
                let h2 = mouse_hook().unwrap();
                assert!(h2.try_recv().is_err());

                utils::fixme::click(Mouse::Left);
                assert_eq!(h2.try_recv(), utils::a_button(Left(SingleClick), Down));
                assert_eq!(h2.try_recv(), utils::a_button(Left(SingleClick), Up));
                assert!(h2.try_recv().is_err());

                utils::fixme::release(Mouse::Right);
                assert_eq!(h2.try_recv(), utils::a_button(Right(SingleClick), Up));
                assert!(h2.try_recv().is_err());

                // This J release is captured by h2, but will not be seen by h3
                utils::fixme::release(Mouse::Left);
            }
            let h3 = mouse_hook().unwrap();
            assert!(h3.try_recv().is_err());
            
            utils::fixme::click(Mouse::Left);
            utils::fixme::click(Mouse::Right);        
            assert_eq!(h3.try_recv(), utils::a_button(Left(SingleClick), Down));
            assert_eq!(h3.try_recv(), utils::a_button(Left(SingleClick), Up));
            assert_eq!(h3.try_recv(), utils::a_button(Right(SingleClick), Down));
            assert_eq!(h3.try_recv(), utils::a_button(Right(SingleClick), Up));
            assert!(h3.try_recv().is_err());
        }

        #[test]
        fn mouse_hook_does_not_capture_mouse() {
            let h1 = mouse_hook().unwrap();
            assert!(h1.try_recv().is_err());

            use mki::Keyboard;

            Keyboard::A.click();
            Keyboard::B.click();
            utils::fixme::click(Mouse::Left);    
            Keyboard::C.click();

            utils::fixme::delay_execution();

            assert_eq!(h1.try_recv(), utils::a_button(Left(SingleClick), Down));
            assert_eq!(h1.try_recv(), utils::a_button(Left(SingleClick), Up));
            assert!(h1.try_recv().is_err());
        }
    }

    mod mouse_moves {
        use crate::mouse_hook_tests::*;

        // Mouse move tests do not work properly on the GitHub CI Action.
        // I'm not sure why, because they pass locally.
        // Use `cargo test --tests -- --test-threads=1 --include-ignored` before publish.
        #[ignore]
        #[test]
        fn move_once() {
            utils::fixme::move_by(10, 10);

            let h = mouse_hook().unwrap();
            assert!(h.try_recv().is_err());

            let (new_x, new_y) = utils::fixme::move_by(10, 10);

            assert_eq!(h.try_recv(), utils::a_move(new_x, new_y));
            assert!(h.try_recv().is_err());
        }

        #[test]
        fn move_once_generates_mouse_move() {
            utils::fixme::move_by(10, 10);

            let h = mouse_hook().unwrap();
            assert!(h.try_recv().is_err());

            utils::fixme::move_by(10, 10);
            assert!(utils::is_mouse_move(h.try_recv()));
            assert!(h.try_recv().is_err());
        }

        // Mouse move tests do not work properly on the GitHub CI Action.S
        // I'm not sure why, because they pass locally.
        // Use `cargo test --tests -- --test-threads=1 --include-ignored` before publish.
        #[ignore]
        #[test]
        fn move_couple_of_times() {
            utils::fixme::move_by(10, 10);

            let h = mouse_hook().unwrap();
            assert!(h.try_recv().is_err());

            let new_pos = vec![
                utils::fixme::move_by(10, 15),
                utils::fixme::move_by(-10, 10),
                utils::fixme::move_by(-15, -15),
                utils::fixme::move_by(10, -10),
                utils::fixme::move_by(15, 0),
                utils::fixme::move_by(0, 10),
            ];

            for np in new_pos {
                let (new_x, new_y) = np;
                assert_eq!(h.try_recv(), utils::a_move(new_x, new_y));
            }
            assert!(h.try_recv().is_err());
        }

        #[test]
        fn move_couple_of_times_generates_mouse_move() {
            utils::fixme::move_by(10, 10);

            let h = mouse_hook().unwrap();
            assert!(h.try_recv().is_err());

            let new_pos = vec![
                utils::fixme::move_by(10, 15),
                utils::fixme::move_by(-10, 10),
                utils::fixme::move_by(-15, -15),
                utils::fixme::move_by(10, -10),
                utils::fixme::move_by(15, 0),
                utils::fixme::move_by(0, 10),
            ];

            // This test runs on the GitHub CI and tests only if we receive mouse move event
            // Mouse moves behave unpredictably on the GitHub CI (point values mismatch)
            for _ in new_pos {
                assert!(utils::is_mouse_move(h.try_recv()));
            }
            assert!(h.try_recv().is_err());
        }
    }

    mod mouse_wheel {
        use crate::mouse_hook_tests::*;

        #[test]
        pub fn wheels() {
            utils::fixme::horizontal_wheel_forward();
            utils::fixme::horizontal_wheel_backward();
            utils::fixme::vertical_wheel_forward();
            utils::fixme::vertical_wheel_backward();
            
            let h = willhook().unwrap();

            utils::fixme::vertical_wheel_backward();
            utils::fixme::vertical_wheel_forward();
            utils::fixme::horizontal_wheel_forward();
            utils::fixme::horizontal_wheel_backward();

            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Backward));
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Forward));
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Horizontal, MouseWheelDirection::Forward));
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Horizontal, MouseWheelDirection::Backward));
            assert!(h.try_recv().is_err());
        }

        #[test]
        pub fn wheels_interleaved_receive() {
            let h = willhook().unwrap();

            utils::fixme::vertical_wheel_backward();
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Backward));
            assert!(h.try_recv().is_err());

            utils::fixme::horizontal_wheel_forward();
            utils::fixme::horizontal_wheel_backward();
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Horizontal, MouseWheelDirection::Forward));
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Horizontal, MouseWheelDirection::Backward));
            assert!(h.try_recv().is_err());

            utils::fixme::vertical_wheel_forward();
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Forward));
            assert!(h.try_recv().is_err());
        }
    }

    mod mixed {
        use crate::mouse_hook_tests::*;

        #[test]
        pub fn mixed_mouse_inputs() {            
            let h = mouse_hook().unwrap();

            utils::fixme::vertical_wheel_backward();
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Backward));
            assert!(h.try_recv().is_err());

            Keyboard::A.click();
            utils::fixme::vertical_wheel_backward();
            utils::fixme::move_by(10, 15);
            utils::fixme::vertical_wheel_forward();
            Keyboard::B.click();
            utils::fixme::horizontal_wheel_forward();
            utils::fixme::move_by(-10, 10);
            utils::fixme::click(Mouse::Middle);
            utils::fixme::move_by(-15, -15);
            Keyboard::C.click();
            utils::fixme::move_by(10, -10);
            utils::fixme::horizontal_wheel_backward();
            utils::fixme::press(Mouse::Right);
            utils::fixme::move_by(15, 0);
            Keyboard::C.click();
            utils::fixme::release(Mouse::Right);
            utils::fixme::move_by(0, 10);
            utils::fixme::press(Mouse::Left);
            utils::fixme::release(Mouse::Left);

            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Backward));
            assert!(utils::is_mouse_move(h.try_recv()));
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Forward));
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Horizontal, MouseWheelDirection::Forward));
            assert!(utils::is_mouse_move(h.try_recv()));
            assert_eq!(h.try_recv(), utils::a_button(Middle(SingleClick), Down));
            assert_eq!(h.try_recv(), utils::a_button(Middle(SingleClick), Up));
            assert!(utils::is_mouse_move(h.try_recv()));
            assert!(utils::is_mouse_move(h.try_recv()));
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Horizontal, MouseWheelDirection::Backward));
            assert_eq!(h.try_recv(), utils::a_button(Right(SingleClick), Down));
            assert!(utils::is_mouse_move(h.try_recv()));
            assert_eq!(h.try_recv(), utils::a_button(Right(SingleClick), Up));
            assert!(utils::is_mouse_move(h.try_recv()));
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Down));
            assert_eq!(h.try_recv(), utils::a_button(Left(SingleClick), Up));
            assert!(h.try_recv().is_err());

            utils::fixme::vertical_wheel_backward();
            utils::fixme::vertical_wheel_backward();
            utils::fixme::vertical_wheel_backward();
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Backward));
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Backward));
            assert_eq!(h.try_recv(), utils::a_wheel(MouseWheel::Vertical, MouseWheelDirection::Backward));
            assert!(h.try_recv().is_err());
        }
    }
}