pub(super) mod details;

/// Main event sent by the hook to the client thread.
#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum InputEvent {
    /// It is keyboard event, the inner value contains the details. See [KeyboardEvent].
    Keyboard(KeyboardEvent),
    /// It is mouse event, the inner value contains the details. See [MouseEvent].
    Mouse(MouseEvent),
    /// Unexpected data was received by the hook, the event type is stored for reference as inner value.
    Other(u32),
}

/// Indicates if the keyboard event was injected by the software, see this crate integration tests for example.
#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum IsEventInjected {
    /// Event was injected by software
    Injected,
    /// Input comes from the user input (real hardware)
    NotInjected,
}

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub struct KeyboardEvent {
    pub pressed: KeyPress,
    pub key: Option<KeyboardKey>,
    pub is_injected: Option<IsEventInjected>,
}

/// Enum to distinguish system key press from normal key press.
#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum IsSystemKeyPress {
    /// System key is basically any key pressed while ALT is also pressed.
    System,
    /// Indicates that key input event occured while ALT key was NOT pressed.
    Normal,
}

/// Indicates whether the [KeyboardKey] was pressed [KeyPress::Down] or [KeyPress::Up].
#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum KeyPress {
    Down(IsSystemKeyPress),
    Up(IsSystemKeyPress),
    Other(usize),
}

/// Indicates what key on the keyboard was pressed.
#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum KeyboardKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    LeftAlt,
    RightAlt,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    BackSpace,
    Tab,
    Enter,
    Escape,
    Space,
    PageUp,
    PageDown,
    Home,
    ArrowLeft,
    ArrowUp,
    ArrowRight,
    ArrowDown,
    Print,
    PrintScreen,
    Insert,
    Delete,
    LeftWindows,
    RightWindows,
    /// , (with shift <)
    Comma,         
    /// . (with shift >)
    Period,        
    /// / (with shift ?)
    Slash,         
    /// ; (with shift :)
    SemiColon,     
    /// ' (with shift ")
    Apostrophe,    
    /// [ (with shift {)
    LeftBrace,     
    /// \ (with shift |)
    BackwardSlash, 
    /// ] (with shift })
    RightBrace,    
    /// ` (with shift ~)
    Grave,         
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    NumLock,
    ScrollLock,
    CapsLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Multiply,
    Add,
    Separator,
    Subtract,
    Decimal,
    Divide,
    Other(u32),
    /// Invalid input received from the OS
    InvalidKeyCodeReceived,
}

// those dont have defines.
const VK_0: i32 = 0x30;
const VK_1: i32 = 0x31;
const VK_2: i32 = 0x32;
const VK_3: i32 = 0x33;
const VK_4: i32 = 0x34;
const VK_5: i32 = 0x35;
const VK_6: i32 = 0x36;
const VK_7: i32 = 0x37;
const VK_8: i32 = 0x38;
const VK_9: i32 = 0x39;
const VK_A: i32 = 0x41;
const VK_B: i32 = 0x42;
const VK_C: i32 = 0x43;
const VK_D: i32 = 0x44;
const VK_E: i32 = 0x45;
const VK_F: i32 = 0x46;
const VK_G: i32 = 0x47;
const VK_H: i32 = 0x48;
const VK_I: i32 = 0x49;
const VK_J: i32 = 0x4A;
const VK_K: i32 = 0x4B;
const VK_L: i32 = 0x4C;
const VK_M: i32 = 0x4D;
const VK_N: i32 = 0x4E;
const VK_O: i32 = 0x4F;
const VK_P: i32 = 0x50;
const VK_Q: i32 = 0x51;
const VK_R: i32 = 0x52;
const VK_S: i32 = 0x53;
const VK_T: i32 = 0x54;
const VK_U: i32 = 0x55;
const VK_V: i32 = 0x56;
const VK_W: i32 = 0x57;
const VK_X: i32 = 0x58;
const VK_Y: i32 = 0x59;
const VK_Z: i32 = 0x5A;

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub struct MouseEvent {
    pub event: MouseEventType,
    pub is_injected: Option<IsEventInjected>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum MouseEventType {
    Press(MousePressEvent),
    Move(MouseMoveEvent),
    Wheel(MouseWheelEvent),
    Other(usize)
}

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub struct MousePressEvent {
    pub pressed: MouseButtonPress,
    pub button: MouseButton,
}

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum MouseWheel {
    Horizontal,
    Vertical,
    Unknown(usize),
}

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum MouseWheelDirection {
    Forward,
    Backward,
    Unknown(u32),
}
#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub struct MouseWheelEvent {
    pub wheel: MouseWheel,
    pub direction: Option<MouseWheelDirection>

}

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub struct MouseMoveEvent {
    pub point: Option<Point>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum MouseButtonPress {
    Down,
    Up,
    Other(usize),
}

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum MouseClick {
    SingleClick,
    DoubleClick,
    Other(u32),
}

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum MouseButton {
    Left(MouseClick),
    Right(MouseClick),
    Middle(MouseClick),
    /// XBUTTON1
    X1(MouseClick), 
    /// XBUTTON2
    X2(MouseClick), 
    /// Either XBUTTON1 or XBUTTON2
    UnkownX(MouseClick),  
    /// Unexpected mouse button. Raw code stored for reference, see MSDN documentation about low-level hooks.
    Other(usize),
}
