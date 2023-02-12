#![allow(non_snake_case)]

use std::convert::From;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;

use crate::event::*;

impl KeyboardEvent {
    pub unsafe fn new(wm_key_code: WPARAM, kbd_hook_struct: *const KBDLLHOOKSTRUCT) -> Self {
        KeyboardEvent{
            pressed: KeyPress::from(wm_key_code),
            key: KeyboardKey::optionally_from(kbd_hook_struct),
            is_injected: IsKeyboardEventInjected::optionally_from(kbd_hook_struct),
        }
    }
}

impl IsKeyboardEventInjected {
    unsafe fn optionally_from(value: *const KBDLLHOOKSTRUCT) -> Option<Self> {
        if value.is_null() {
            None
        } else {
            Some(IsKeyboardEventInjected::from((*value).flags))
        }
    }
}

impl From<DWORD> for IsKeyboardEventInjected {
    fn from(value: DWORD) -> Self {
        let i1 = 0 != value & LLKHF_INJECTED;
        let i2 = 0 != value & LLKHF_LOWER_IL_INJECTED;
        use IsKeyboardEventInjected::*;
        match i1 || i2 {
            true => Injected,
            false => NotInjected,
        }
    }
}

impl From<WPARAM> for KeyPress {
    fn from(code: WPARAM) -> Self {
        use KeyPress::*;
        use IsSystemKeyPress::*;
        match code.try_into() {
            Ok(WM_KEYDOWN) => Down(Normal),
            Ok(WM_KEYUP) => Up(Normal),
            Ok(WM_SYSKEYDOWN) => Down(System),
            Ok(WM_SYSKEYUP) => Up(System),
            // Either we failed to convert or 
            // it is not one of our supported values
            _ => Other(code),
        }
    }
}

// impl From<KeyPress> for WPARAM {
//     fn from(code: KeyPress) -> Self {
//         use KeyPress::*;
//         match code {
//             Up(is_sys) => {
//                 match is_sys {
//                     false => WM_KEYUP as WPARAM,
//                     true => WM_SYSKEYUP as WPARAM,
//                 }
//             },
//             Down(is_sys) => {
//                 match is_sys {
//                     false => WM_KEYDOWN as WPARAM,
//                     true => WM_SYSKEYUP as WPARAM,
//                 }
//             },
//             Other(usize) => usize as WPARAM,
//         }
//     }
// }

impl From<DWORD> for KeyboardKey {
    fn from(code: DWORD) -> Self {
        use KeyboardKey::*;
        match code.try_into() {
            Ok(sc) => {
                match sc {
                    VK_BACK => BackSpace,
                    VK_TAB => Tab,
                    VK_RETURN => Enter,
                    VK_ESCAPE => Escape,
                    VK_SPACE => Space,
                    VK_PRIOR => PageUp,
                    VK_NEXT => PageDown,
                    VK_HOME => Home,
                    VK_LEFT => ArrowLeft,
                    VK_UP => ArrowUp,
                    VK_RIGHT => ArrowRight,
                    VK_DOWN => ArrowDown,
                    VK_PRINT => Print,
                    VK_SNAPSHOT => PrintScreen,
                    VK_INSERT => Insert,
                    VK_DELETE => Delete,
                    VK_0 => Number0,
                    VK_1 => Number1,
                    VK_2 => Number2,
                    VK_3 => Number3,
                    VK_4 => Number4,
                    VK_5 => Number5,
                    VK_6 => Number6,
                    VK_7 => Number7,
                    VK_8 => Number8,
                    VK_9 => Number9,
                    VK_A => A,
                    VK_B => B,
                    VK_C => C,
                    VK_D => D,
                    VK_E => E,
                    VK_F => F,
                    VK_G => G,
                    VK_H => H,
                    VK_I => I,
                    VK_J => J,
                    VK_K => K,
                    VK_L => L,
                    VK_M => M,
                    VK_N => N,
                    VK_O => O,
                    VK_P => P,
                    VK_Q => Q,
                    VK_R => R,
                    VK_S => S,
                    VK_T => T,
                    VK_U => U,
                    VK_V => V,
                    VK_W => W,
                    VK_X => X,
                    VK_Y => Y,
                    VK_Z => Z,
                    VK_LWIN => LeftWindows,
                    VK_RWIN => RightWindows,
                    VK_NUMPAD0 => Numpad0,
                    VK_NUMPAD1 => Numpad1,
                    VK_NUMPAD2 => Numpad2,
                    VK_NUMPAD3 => Numpad3,
                    VK_NUMPAD4 => Numpad4,
                    VK_NUMPAD5 => Numpad5,
                    VK_NUMPAD6 => Numpad6,
                    VK_NUMPAD7 => Numpad7,
                    VK_NUMPAD8 => Numpad8,
                    VK_NUMPAD9 => Numpad9,
                    VK_MULTIPLY => Multiply,
                    VK_ADD => Add,
                    VK_SEPARATOR => Separator,
                    VK_SUBTRACT => Subtract,
                    VK_DECIMAL => Decimal,
                    VK_DIVIDE => Divide,
                    VK_F1 => F1,
                    VK_F2 => F2,
                    VK_F3 => F3,
                    VK_F4 => F4,
                    VK_F5 => F5,
                    VK_F6 => F6,
                    VK_F7 => F7,
                    VK_F8 => F8,
                    VK_F9 => F9,
                    VK_F10 => F10,
                    VK_F11 => F11,
                    VK_F12 => F12,
                    VK_F13 => F13,
                    VK_F14 => F14,
                    VK_F15 => F15,
                    VK_F16 => F16,
                    VK_F17 => F17,
                    VK_F18 => F18,
                    VK_F19 => F19,
                    VK_F20 => F20,
                    VK_F21 => F21,
                    VK_F22 => F22,
                    VK_F23 => F23,
                    VK_F24 => F24,
                    VK_NUMLOCK => NumLock,
                    VK_SCROLL => ScrollLock,
                    VK_CAPITAL => CapsLock,
                    VK_LSHIFT => LeftShift,
                    VK_RSHIFT => RightShift,
                    VK_LCONTROL => LeftControl,
                    VK_RCONTROL => RightControl,
                    VK_LMENU => LeftAlt,
                    VK_RMENU => RightAlt,
                    VK_OEM_PERIOD => Period,
                    VK_OEM_COMMA => Comma,
                    VK_OEM_1 => SemiColon,
                    VK_OEM_2 => Slash,
                    VK_OEM_3 => Grave,
                    VK_OEM_4 => LeftBrace,
                    VK_OEM_5 => BackwardSlash,
                    VK_OEM_6 => RightBrace,
                    VK_OEM_7 => Apostrophe,
                    _ => Other(code),
                }
            }
            Err(_) => Other(code),
        }
    }
}

impl KeyboardKey {
    pub unsafe fn optionally_from(value: *const KBDLLHOOKSTRUCT) -> Option<Self> {
        if value.is_null() {
            None
        } else {
            Some(KeyboardKey::from((*value).vkCode))
        }
    }
}

impl MouseEvent {
    pub unsafe fn new(wm_mouse_param: WPARAM, ms_ll_hook_struct: *const MSLLHOOKSTRUCT) -> Self {
        use MouseEventType::*;
        MouseEvent{
            is_injected: IsMouseEventInjected::optionally_from(ms_ll_hook_struct),
            event: match wm_mouse_param as u32 {
                // Mouse press
                WM_LBUTTONDOWN | WM_LBUTTONUP | WM_LBUTTONDBLCLK => Press(MousePressEvent::new(wm_mouse_param, ms_ll_hook_struct)),
                WM_RBUTTONDOWN | WM_RBUTTONUP | WM_RBUTTONDBLCLK => Press(MousePressEvent::new(wm_mouse_param, ms_ll_hook_struct)),
                WM_MBUTTONDOWN | WM_MBUTTONUP | WM_MBUTTONDBLCLK => Press(MousePressEvent::new(wm_mouse_param, ms_ll_hook_struct)),
                WM_XBUTTONDOWN | WM_XBUTTONUP | WM_XBUTTONDBLCLK => Press(MousePressEvent::new(wm_mouse_param, ms_ll_hook_struct)),
                
                // Mouse move
                WM_MOUSEMOVE => Move(MouseMoveEvent::new(ms_ll_hook_struct)),

                // Wheel move
                WM_MOUSEWHEEL | WM_MOUSEHWHEEL => Wheel(MouseWheelEvent::new(wm_mouse_param, ms_ll_hook_struct)),

                _ => Other(wm_mouse_param),
            }
        }
    }
}

impl MousePressEvent {
    pub unsafe fn new(wm_mouse_param: WPARAM, ms_ll_hook_struct: *const MSLLHOOKSTRUCT) -> MousePressEvent {
        MousePressEvent { 
            pressed: MouseButtonPress::from(wm_mouse_param),
            button: MouseButton::from(wm_mouse_param, ms_ll_hook_struct),
        }
    }
}

impl From<POINT> for Point {
    fn from(value: POINT) -> Self {
        Point { x: value.x, y: value.y }
    }
}

impl IsMouseEventInjected {
    unsafe fn optionally_from(value: *const MSLLHOOKSTRUCT) -> Option<Self> {
        if value.is_null() {
            None
        } else {
            Some(IsMouseEventInjected::from((*value).flags))
        }
    }
}

impl From<DWORD> for IsMouseEventInjected {
    fn from(value: DWORD) -> Self {
        let i1 = 0 != value & LLMHF_INJECTED;
        let i2 = 0 != value & LLMHF_LOWER_IL_INJECTED;
        use IsMouseEventInjected::*;
        match i1 || i2 {
            true => Injected,
            false => NotInjected,
        }
    }
}

impl MouseWheel {
    pub fn new(wm_mouse_param: WPARAM) -> MouseWheel {
        use MouseWheel::*;
        match wm_mouse_param.try_into() {
            Ok(param_u32) => {
                match param_u32 {
                    WM_MOUSEWHEEL => Vertical,
                    WM_MOUSEHWHEEL => Horizontal,
                    _ => Unknown(wm_mouse_param),
                }
            },
            _ => Unknown(wm_mouse_param),
        }
    }
}

impl MouseWheelEvent {
    pub unsafe fn new(wm_mouse_param: WPARAM, ms_ll_hook_struct: *const MSLLHOOKSTRUCT) -> MouseWheelEvent {
        MouseWheelEvent { 
            wheel: MouseWheel::new(wm_mouse_param), 
            direction: MouseWheelDirection::optionally_from(ms_ll_hook_struct)
        }
        
    }
}

impl MouseWheelDirection {
    pub unsafe fn optionally_from(ms_ll_hook_struct: *const MSLLHOOKSTRUCT) -> Option<MouseWheelDirection>{
        if ms_ll_hook_struct.is_null() {
            None
        } else {
            Some(MouseWheelDirection::new(&*ms_ll_hook_struct))
        }
    }
    
    fn new(ms_ll_hook_struct: &MSLLHOOKSTRUCT) -> MouseWheelDirection {
        use MouseWheelDirection::*;
        let delta = GET_WHEEL_DELTA_WPARAM(ms_ll_hook_struct.mouseData as WPARAM);
        match delta {
            _ if delta > 0 => Forward,
            _ if delta < 0 => Backward,
            _ => Unknown(ms_ll_hook_struct.mouseData),
        }
    }
}

impl MouseMoveEvent {
    pub unsafe fn new(ms_ll_hook_struct: *const MSLLHOOKSTRUCT) -> MouseMoveEvent {
        if ms_ll_hook_struct.is_null() {
            MouseMoveEvent{ point: None }
        } else {
            let msll = &*ms_ll_hook_struct;
            let pt = msll.pt;
            MouseMoveEvent{ point: Some(pt.into()) }
        }
    }
}

impl From<WPARAM> for MouseButtonPress {
    fn from(value: WPARAM) -> Self {
        use MouseButtonPress::*;
        match value.try_into() {
            Ok(uv) => {
                match uv {
                    WM_LBUTTONDOWN| WM_RBUTTONDOWN| WM_MBUTTONDOWN | WM_XBUTTONDOWN => Down,
                    WM_RBUTTONUP | WM_LBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => Up,
                    _ => Other(value),
                }
            }
            Err(_) => Other(value),
        }
    }
}

impl From<WPARAM> for MouseClick {
    fn from(value: WPARAM) -> Self {
        use MouseClick::*;
        match value.try_into() {
            Ok (uv) => {
                match uv {
                    WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => SingleClick,
                    WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => SingleClick,
                    WM_LBUTTONDBLCLK | WM_RBUTTONDBLCLK | WM_MBUTTONDBLCLK | WM_XBUTTONDBLCLK => DoubleClick,
                    _ => Other(value as u32),

                }
            },
            Err(_) => Other(value as u32),
        }    
    }
}

impl MouseButton {
    pub unsafe fn from(wm_mouse_param: WPARAM, ms_ll_hook_struct: *const MSLLHOOKSTRUCT) -> Self {
        let click = MouseClick::from(wm_mouse_param);

        use MouseButton::*;
        match wm_mouse_param.try_into() {
            Ok(param) => {
                match param {
                    WM_LBUTTONDOWN | WM_LBUTTONUP | WM_LBUTTONDBLCLK => Left(click),
                    WM_RBUTTONDOWN | WM_RBUTTONUP | WM_RBUTTONDBLCLK => Right(click),
                    WM_MBUTTONDOWN | WM_MBUTTONUP | WM_MBUTTONDBLCLK=> Middle(click),
                    WM_XBUTTONDOWN | WM_XBUTTONUP | WM_XBUTTONDBLCLK => {
                        if ms_ll_hook_struct.is_null() {
                            UnkownX(click)
                        } else {
                            Self::into_extra(click, &*ms_ll_hook_struct)
                        }
                    },
                    // Value out of expected set
                    _ => Other(wm_mouse_param)
                }
            },
            // Conversion error
            Err(_) => Other(wm_mouse_param),
        }
    }

    fn into_extra(click: MouseClick, ms_ll_hook_struct: &MSLLHOOKSTRUCT) -> Self {
        use MouseButton::*;
        match GET_XBUTTON_WPARAM(ms_ll_hook_struct.mouseData.try_into().expect("")) {
            XBUTTON1 => X1(click),
            XBUTTON2 => X2(click),
            _ => UnkownX(click),
        }
    }
}



