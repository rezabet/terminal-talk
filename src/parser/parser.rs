use bevy::{input::keyboard::KeyCode, prelude::*};

// Parsing:
//
// Keys have predetermined names in Blender. We map those to sequential ID's in Rust.
// We do this to get a range (0..104) of keys that we can iterate over.
// The alternative to a single contiguous range is a set of non-contiguous ranges (e.g., { 0..=9, a..=z, ..etc }).
// This causes a number of issues as it adds:
//      1. Complexity: How would an implementation of keys that do not implement the `Range` trait look like? (e.g., f1..=f12, arrow keys, ..etc).
//          1a. Furthermore, this causes overlapping ID values if the non-contiguous ranges use the same start value.
//      2. Code Duplication: The core logic for every key is the same. Having multiple ranges violates the DRY principle.
//
// Notes:
// The iterator's of the below 2 types - `KeyCodeWrapper` and `Keys` - should have the order of the values they're iterating over *in-sync*.
// This is necessary as we use their indices as the "common ground" to map between the blender key meshes to their respective `KeyCode` and vice-versa.
pub struct KeyCodeWrapper;
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

// A component used to identify entities that are keys
#[derive(Component)]
pub struct Key {
    pub entity: Entity,
    pub name: String,
}

pub fn mark_entities(
    mut commands: Commands,
    query: Query<(Entity, &Name), (Added<Name>, Without<Key>)>,
) {
    for (entity, blender_key) in query.iter() {
        // `any()` tests if any element of an iterator matches a predicate.
        // `any()` takes a closure that returns `true` or `false`.
        // It applies this closure to each element of the iterator, and if any of them return `true`, then so does `any()`.
        // If they all return `false`, it returns `false`.
        // `any()` is short-circuiting; in other words, it will stop processing as soon as it finds a `true`.
        // In our case, `any()` tests if any mesh in our Blender scene has a name equal to that of *our definition* of a `Key`.
        if Keys::into_iter().any(|key| blender_key.as_str() == key) {
            commands.entity(entity).insert(Key {
                entity,
                name: String::from(blender_key.as_str()),
            });
        };
    }
}
