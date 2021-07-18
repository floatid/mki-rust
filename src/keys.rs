use std::str::FromStr;

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum Keyboard {
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
    Left,
    Up,
    Right,
    Down,
    Print,
    PrintScreen,
    Insert,
    Delete,
    LeftWindows,
    RightWindows,
    Comma,         // ,<
    Period,        // .>
    Slash,         // /?
    SemiColon,     // ;:
    Apostrophe,    // '"
    LeftBrace,     // [{
    BackwardSlash, // \|
    RightBrace,    // ]}
    Grave,         // `~
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
    Other(i32),
}

impl FromStr for Keyboard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = match s {
            "A" => Keyboard::A,
            "B" => Keyboard::B,
            "C" => Keyboard::C,
            "D" => Keyboard::D,
            "E" => Keyboard::E,
            "F" => Keyboard::F,
            "G" => Keyboard::G,
            "H" => Keyboard::H,
            "I" => Keyboard::I,
            "J" => Keyboard::J,
            "K" => Keyboard::K,
            "L" => Keyboard::L,
            "M" => Keyboard::M,
            "N" => Keyboard::N,
            "O" => Keyboard::O,
            "P" => Keyboard::P,
            "Q" => Keyboard::Q,
            "R" => Keyboard::R,
            "S" => Keyboard::S,
            "T" => Keyboard::T,
            "U" => Keyboard::U,
            "V" => Keyboard::V,
            "W" => Keyboard::W,
            "X" => Keyboard::X,
            "Y" => Keyboard::Y,
            "Z" => Keyboard::Z,
            "0" => Keyboard::Number0,
            "1" => Keyboard::Number1,
            "2" => Keyboard::Number2,
            "3" => Keyboard::Number3,
            "4" => Keyboard::Number4,
            "5" => Keyboard::Number5,
            "6" => Keyboard::Number6,
            "7" => Keyboard::Number7,
            "8" => Keyboard::Number8,
            "9" => Keyboard::Number9,
            "LeftAlt" => Keyboard::LeftAlt,
            "RightAlt" => Keyboard::RightAlt,
            "LeftShift" => Keyboard::LeftShift,
            "RightShift" => Keyboard::RightShift,
            "LeftControl" => Keyboard::LeftControl,
            "RightControl" => Keyboard::RightControl,
            "BackSpace" => Keyboard::BackSpace,
            "Tab" | "	" => Keyboard::Tab,
            "Enter" | "\n" => Keyboard::Enter,
            "Escape" => Keyboard::Escape,
            "Space" | " " => Keyboard::Space,
            "PageUp" => Keyboard::PageUp,
            "PageDown" => Keyboard::PageDown,
            "Home" => Keyboard::Home,
            "Left" => Keyboard::Left,
            "Up" => Keyboard::Up,
            "Right" => Keyboard::Right,
            "Down" => Keyboard::Down,
            "Print" => Keyboard::Print,
            "PrintScreen" => Keyboard::PrintScreen,
            "Insert" => Keyboard::Insert,
            "Delete" => Keyboard::Delete,
            "LeftWindows" => Keyboard::LeftWindows,
            "RightWindows" => Keyboard::RightWindows,
            "Comma" | "," => Keyboard::Comma,
            "Period" | "." => Keyboard::Period,
            "Slash" | "/" => Keyboard::Slash,
            "SemiColon" | ";" | ":" => Keyboard::SemiColon,
            "Apostrophe" | "'" | "\"" => Keyboard::Apostrophe,
            "LeftBrace" | "[" => Keyboard::LeftBrace,
            "BackwardSlash" | "\\" => Keyboard::BackwardSlash,
            "RightBrace" | "]" => Keyboard::RightBrace,
            "Grave" | "`" => Keyboard::Grave,
            "F1" => Keyboard::F1,
            "F2" => Keyboard::F2,
            "F3" => Keyboard::F3,
            "F4" => Keyboard::F4,
            "F5" => Keyboard::F5,
            "F6" => Keyboard::F6,
            "F7" => Keyboard::F7,
            "F8" => Keyboard::F8,
            "F9" => Keyboard::F9,
            "F10" => Keyboard::F10,
            "F11" => Keyboard::F11,
            "F12" => Keyboard::F12,
            "F13" => Keyboard::F13,
            "F14" => Keyboard::F14,
            "F15" => Keyboard::F15,
            "F16" => Keyboard::F16,
            "F17" => Keyboard::F17,
            "F18" => Keyboard::F18,
            "F19" => Keyboard::F19,
            "F20" => Keyboard::F20,
            "F21" => Keyboard::F21,
            "F22" => Keyboard::F22,
            "F23" => Keyboard::F23,
            "F24" => Keyboard::F24,
            "NumLock" => Keyboard::NumLock,
            "ScrollLock" => Keyboard::ScrollLock,
            "CapsLock" => Keyboard::CapsLock,
            "Numpad0" => Keyboard::Numpad0,
            "Numpad1" => Keyboard::Numpad1,
            "Numpad2" => Keyboard::Numpad2,
            "Numpad3" => Keyboard::Numpad3,
            "Numpad4" => Keyboard::Numpad4,
            "Numpad5" => Keyboard::Numpad5,
            "Numpad6" => Keyboard::Numpad6,
            "Numpad7" => Keyboard::Numpad7,
            "Numpad8" => Keyboard::Numpad8,
            "Numpad9" => Keyboard::Numpad9,
            "Multiply" => Keyboard::Multiply,
            "Add" => Keyboard::Add,
            "Separator" => Keyboard::Separator,
            "Subtract" => Keyboard::Subtract,
            "Decimal" => Keyboard::Decimal,
            "Divide" => Keyboard::Divide,
            _ => return Err(()),
        };
        Ok(parsed)
    }
}
