use lazy_static::lazy_static;
use std::collections::HashMap;

// const keyboard: Record<string, number> = {
// 	// TODO: adjust this to match real DAW keymaps and maybe detect keymap and switch between different layouts
// 	IntlBackslash: -2,
// 	KeyA: -1,
// 	KeyZ: 0, // 24
// 	KeyS: 1,
// 	KeyX: 2,
// 	KeyC: 3,
// 	KeyF: 4,
// 	KeyV: 5,
// 	KeyG: 6,
// 	KeyB: 7,
// 	KeyN: 8,
// 	KeyJ: 9,
// 	KeyM: 10,
// 	KeyK: 11,
// 	Comma: 12,
// 	KeyL: 13,
// 	Period: 14,
// 	Slash: 15,
// 	Quote: 16,
// 	Digit1: 16,
// 	BackSlash: 17,
// 	KeyQ: 17, // 36
// 	Digit2: 18,
// 	KeyW: 19,
// 	KeyE: 20,
// 	Digit4: 21,
// 	KeyR: 22,
// 	Digit5: 23,
// 	KeyT: 24,
// 	Digit6: 25,
// 	KeyY: 26,
// 	KeyU: 27,
// 	Digit8: 28,
// 	KeyI: 29,
// 	Digit9: 30,
// 	KeyO: 31,
// 	KeyP: 32,
// 	Minus: 33,
// 	BracketLeft: 34,
// 	Equal: 35,
// 	BracketRight: 36,
// };

lazy_static! {
    pub static ref GERMAN_KEYMAP: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("KeyF1", 0);
        m.insert("KeyDel", 1);
        m.insert("KeyIns", 3);
        m.insert("KeyF2", 2);
        m.insert("KeyF3", 4);
        m.insert("KeyF4", 5);
        m.insert("KeyEnd", 6);
        m.insert("KeyF5", 7);
        m.insert("KeyHome", 8);
        m.insert("KeyF6", 9);
        m.insert("KeyPgDown", 10);
        m.insert("KeyF7", 11);
        m.insert("KeyF8", 12);
        m.insert("KeyPgUp", 13);
        m.insert("KeyF9", 14);
        m.insert("KeyF10", 15);
        m.insert("KeyF11", 16);
        m.insert("KeyF12", 17);
        m.insert("&lt;", 19);

        m.insert("KeyA", 20);
        m.insert("KeyY", 21);
        m.insert("KeyS", 22);
        m.insert("KeyX", 23);
        m.insert("KeyC", 24);
        m.insert("KeyF", 25);
        m.insert("KeyV", 26);
        m.insert("KeyG", 27);
        m.insert("KeyB", 28);
        m.insert("KeyN", 29);
        m.insert("Digit1", 30);
        m.insert("KeyJ", 30);
        m.insert("KeyM", 31);
        m.insert("KeyQ", 31);
        m.insert("KeyK", 32);
        m.insert("Digit2", 32);
        m.insert("KeyW", 33);
        m.insert("Key,", 33);
        m.insert("KeyL", 34);
        m.insert("Digit3", 34);
        m.insert("KeyE", 35);
        m.insert("Key.", 35);
        m.insert("Key-", 36);
        m.insert("KeyR", 36);
        m.insert("Digit5", 37);
        m.insert("KeyÄ", 37);
        m.insert("KeyT", 38);
        m.insert("Key#", 38);
        m.insert("Digit6", 39);
        m.insert("KeyZ", 40);
        m.insert("KeyU", 41);
        m.insert("Digit8", 42);
        m.insert("KeyI", 43);
        m.insert("Digit9", 44);
        m.insert("KeyO", 45);
        m.insert("Digit0", 46);
        m.insert("KeyP", 47);

        m.insert("KeyÜ", 48);
        m.insert("Minus", 50);

        m
    };
    // {a 8}
    // {z 9}
    // {s 10}
    // {x 11}
    // {c 12}
    // {f 13}
    // {v 14}
    // {g 15}
    // {b 16}
    // {n 17}
    // {j 18}
    // {m 19}
    // {k 20}
    // {comma 21}
    // {l 22}
    // {period 23}
    // {slash 24}
    // {apostrophe 25}
    // {backslash 26}
    // {grave 27}

    pub static ref US_KEYMAP: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("KeyZ", 24);
        m.insert("KeyS", 25);
        m.insert("KeyX", 26);
        m.insert("KeyD", 27);
        m.insert("KeyC", 28);
        m.insert("KeyV", 29);
        m.insert("KeyG", 30);
        m.insert("KeyB", 31);
        m.insert("KeyH", 32);
        m.insert("KeyN", 33);
        m.insert("KeyJ", 34);
        m.insert("KeyM", 35);
        m.insert("KeyQ", 36);
        m.insert("Digit2", 37);
        m.insert("KeyW", 38);
        m.insert("Digit3", 39);
        m.insert("KeyE", 40);
        m.insert("KeyR", 41);
        m.insert("Digit5", 42);
        m.insert("KeyT", 43);
        m.insert("Digit6", 44);
        m.insert("KeyY", 45);
        m.insert("Digit7", 46);
        m.insert("KeyU", 47);
        m.insert("KeyI", 48);
        m.insert("Digit9", 49);
        m.insert("KeyO", 50);
        m.insert("Digit0", 51);
        m.insert("KeyP", 52);
        m
    };
}
