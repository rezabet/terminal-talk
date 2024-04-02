use bevy::input::keyboard::KeyCode;

// Notes:
// The iterator's of the below 2 types - `KeyCodeWrapper` and `Keys` - should have the order of the values they're iterating over in-sync.
// This is necessary as the implementation regarding _ depends on it.
pub struct KeyCodeWrapper(KeyCode);
impl KeyCodeWrapper {
    pub fn into_iter() -> std::array::IntoIter<KeyCode, 104> {
        [
            KeyCode::Digit0,
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
            KeyCode::KeyA,
            KeyCode::KeyB,
            KeyCode::KeyC,
            KeyCode::KeyD,
            KeyCode::KeyE,
            KeyCode::KeyF,
            KeyCode::KeyG,
            KeyCode::KeyH,
            KeyCode::KeyI,
            KeyCode::KeyJ,
            KeyCode::KeyK,
            KeyCode::KeyL,
            KeyCode::KeyM,
            KeyCode::KeyN,
            KeyCode::KeyO,
            KeyCode::KeyP,
            KeyCode::KeyQ,
            KeyCode::KeyR,
            KeyCode::KeyS,
            KeyCode::KeyT,
            KeyCode::KeyU,
            KeyCode::KeyV,
            KeyCode::KeyW,
            KeyCode::KeyX,
            KeyCode::KeyY,
            KeyCode::KeyZ,
            KeyCode::Escape,
            KeyCode::F1,
            KeyCode::F2,
            KeyCode::F3,
            KeyCode::F4,
            KeyCode::F5,
            KeyCode::F6,
            KeyCode::F7,
            KeyCode::F8,
            KeyCode::F9,
            KeyCode::F10,
            KeyCode::F11,
            KeyCode::F12,
            KeyCode::Backquote,
            KeyCode::Minus,
            KeyCode::Equal,
            KeyCode::Backspace,
            KeyCode::Tab,
            KeyCode::BracketLeft,
            KeyCode::BracketRight,
            KeyCode::Backslash,
            KeyCode::CapsLock,
            KeyCode::Semicolon,
            KeyCode::Quote,
            KeyCode::Enter,
            KeyCode::ShiftLeft,
            KeyCode::Comma,
            KeyCode::Period,
            KeyCode::Slash,
            KeyCode::ShiftRight,
            KeyCode::ControlLeft,
            KeyCode::SuperLeft,
            KeyCode::AltLeft,
            KeyCode::Space,
            KeyCode::AltRight,
            KeyCode::Fn,
            KeyCode::SuperRight,
            KeyCode::ControlRight,
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
            KeyCode::PrintScreen,
            KeyCode::ScrollLock,
            KeyCode::Pause,
            KeyCode::Insert,
            KeyCode::Delete,
            KeyCode::Home,
            KeyCode::End,
            KeyCode::PageUp,
            KeyCode::PageDown,
            KeyCode::NumLock,
            KeyCode::Numpad0,
            KeyCode::Numpad1,
            KeyCode::Numpad2,
            KeyCode::Numpad3,
            KeyCode::Numpad4,
            KeyCode::Numpad5,
            KeyCode::Numpad6,
            KeyCode::Numpad7,
            KeyCode::Numpad8,
            KeyCode::Numpad9,
            KeyCode::NumpadDivide,
            KeyCode::NumpadMultiply,
            KeyCode::NumpadSubtract,
            KeyCode::NumpadAdd,
            KeyCode::NumpadEnter,
            KeyCode::NumpadDecimal,
        ]
        .into_iter()
    }
}

pub struct Keys;
impl Keys {
    pub fn into_iter() -> std::array::IntoIter<&'static str, 104> {
        [
            "0",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            "a",
            "b",
            "c",
            "d",
            "e",
            "f",
            "g",
            "h",
            "i",
            "j",
            "k",
            "l",
            "m",
            "n",
            "o",
            "p",
            "q",
            "r",
            "s",
            "t",
            "u",
            "v",
            "w",
            "x",
            "y",
            "z",
            "esc",
            "f1",
            "f2",
            "f3",
            "f4",
            "f5",
            "f6",
            "f7",
            "f8",
            "f9",
            "f10",
            "f11",
            "f12",
            "`",
            "-",
            "=",
            "backspace",
            "tab",
            "[",
            "]",
            "\\", // Escape the escape character to have a backslash character
            "caps_lock",
            ";",
            "'",
            "enter",
            "lshift",
            ",",
            ".",
            "/",
            "rshift",
            "lctrl",
            "lsuper",
            "lalt",
            "spacebar",
            "ralt",
            "fn",
            "rsuper",
            "rctrl",
            "arrow_up",
            "arrow_down",
            "arrow_left",
            "arrow_right",
            "prtsc",
            "scrlk",
            "pause",
            "insert",
            "delete",
            "home",
            "end",
            "pgup",
            "pgdn",
            "numpad_numlock",
            "numpad_0",
            "numpad_1",
            "numpad_2",
            "numpad_3",
            "numpad_4",
            "numpad_5",
            "numpad_6",
            "numpad_7",
            "numpad_8",
            "numpad_9",
            "numpad_/",
            "numpad_*",
            "numpad_-",
            "numpad_+",
            "numpad_enter",
            "numpad_.",
        ]
        .into_iter()
    }
}
