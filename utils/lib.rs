use willhook::event::*;
use willhook::event::InputEvent::*;
use willhook::event::MouseEventType::*;

pub fn a_key(key: KeyboardKey, press: KeyPress) -> Result<InputEvent, std::sync::mpsc::TryRecvError> {
    Ok(Keyboard(KeyboardEvent {
                    pressed: press,
                    key: Some(key),
                    is_injected: Some(IsEventInjected::Injected)}))
}

pub fn a_button(button: MouseButton, press: MouseButtonPress) -> Result<InputEvent, std::sync::mpsc::TryRecvError> {
    Ok(Mouse(MouseEvent {
                    event: Press(MousePressEvent{
                        pressed: press,
                        button: button,
                    }),
                    is_injected: Some(IsEventInjected::Injected)}))
}

pub fn a_move(an_x: i32, an_y: i32) -> Result<InputEvent, std::sync::mpsc::TryRecvError> {
    Ok(Mouse(MouseEvent {
        event: Move(MouseMoveEvent{
            point: Some(Point{x: an_x, y: an_y}),
        }),
        is_injected: Some(IsEventInjected::Injected)}))
}

pub fn is_mouse_move(r: Result<InputEvent, std::sync::mpsc::TryRecvError>) -> bool {
    if let Ok(ie) = r {
        if let Mouse(me) = ie {
            if let Move(_) = me.event {
                return true
            }
        }
        // Assertion to print out the actual value in tests
        assert_eq!(ie, InputEvent::Other(0));
    }
    // Assertion to print out that error was returned
    assert!(r.is_ok());
    false
}

pub fn a_wheel(wheel: MouseWheel, wheel_direction: MouseWheelDirection) -> Result<InputEvent, std::sync::mpsc::TryRecvError> {
    Ok(Mouse(MouseEvent {
        event: Wheel(MouseWheelEvent {
                wheel: wheel, direction: Some(wheel_direction),
            }),
        is_injected: Some(IsEventInjected::Injected) }))
}

// The MKI implementation seems to be buggy at the current version.
// It sends incorrect mouse events.
// These are workarounds for this, and also a timing issue.
pub mod fixme {
    use winapi::shared::windef::POINT;
    use winapi::ctypes::c_int;
    use winapi::um::winuser::{
        SendInput, GetCursorPos,
        MOUSEEVENTF_MOVE, INPUT_MOUSE, MOUSEEVENTF_WHEEL, MOUSEEVENTF_HWHEEL,
        LPINPUT, INPUT, INPUT_u, MOUSEINPUT, 
    };

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

    pub fn move_by(x: i32, y: i32) -> (i32, i32) {
        unsafe {
            let mut current_pos = POINT{ x: 0, y: 0, };
            GetCursorPos(&mut current_pos);

            let mut inner_input: INPUT_u = std::mem::zeroed();
            *inner_input.mi_mut() = MOUSEINPUT {
                dx: x,
                dy: y,
                mouseData: 0,
                time: 0,
                dwFlags: MOUSEEVENTF_MOVE,
                dwExtraInfo: 0,
            };
            let mut input = INPUT {
                type_: INPUT_MOUSE,
                u: inner_input,
            };
    
            SendInput(1, &mut input as LPINPUT, std::mem::size_of::<INPUT>() as c_int);

            delay_execution();
            (x + current_pos.x, y + current_pos.y)
        }
    }

    pub fn vertical_wheel_forward() {
        wheel(MOUSEEVENTF_WHEEL,1);
    }

    pub fn vertical_wheel_backward() {
        wheel(MOUSEEVENTF_WHEEL, -1);
    }

    pub fn horizontal_wheel_forward() {
        wheel(MOUSEEVENTF_HWHEEL,1);
    }

    pub fn horizontal_wheel_backward() {
        wheel(MOUSEEVENTF_HWHEEL, -1);
    }

    fn wheel(w: u32, d: i16) {
        unsafe {
            let mut current_pos = POINT{ x: 0, y: 0, };
            GetCursorPos(&mut current_pos);

            let mut inner_input: INPUT_u = std::mem::zeroed();
            *inner_input.mi_mut() = MOUSEINPUT {
                dx: current_pos.x,
                dy: current_pos.y,
                mouseData: ((d as i32) << 16) as u32,
                time: 0,
                dwFlags: w,
                dwExtraInfo: 0,
            };
            let mut input = INPUT {
                type_: INPUT_MOUSE,
                u: inner_input,
            };
    
            SendInput(1, &mut input as LPINPUT, std::mem::size_of::<INPUT>() as c_int);

            delay_execution();
        }
    }

    pub fn click(m: mki::Mouse) {
        press(m);
        release(m);
    }
}

