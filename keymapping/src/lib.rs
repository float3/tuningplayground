use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref GERMAN_KEYMAP: HashMap<&'static str, i32> = {
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
        m.insert("Key1", 30);
        m.insert("KeyJ", 30);
        m.insert("KeyM", 31);
        m.insert("KeyQ", 31);
        m.insert("KeyK", 32);
        m.insert("Key2", 32);
        m.insert("KeyW", 33);
        m.insert("Key,", 33);
        m.insert("KeyL", 34);
        m.insert("Key3", 34);
        m.insert("KeyE", 35);
        m.insert("Key.", 35);
        m.insert("Key-", 36);
        m.insert("KeyR", 36);
        m.insert("Key5", 37);
        m.insert("KeyÄ", 37);
        m.insert("KeyT", 38);
        m.insert("Key#", 38);
        m.insert("Key6", 39);
        m.insert("KeyZ", 40);
        m.insert("KeyU", 41);
        m.insert("Key8", 42);
        m.insert("KeyI", 43);
        m.insert("Key9", 44);
        m.insert("KeyO", 45);
        m.insert("Key0", 46);
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

    static ref US_KEYMAP: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("KeyZ", 24); // index.js:86:21
        m.insert("KeyS", 25); // index.js:86:21
        m.insert("KeyX", 26); // index.js:86:21
        m.insert("KeyD", 27); // index.js:86:21
        m.insert("KeyC", 28); // index.js:86:21
        m.insert("KeyV", 29); // index.js:86:21
        m.insert("KeyG", 30); // index.js:86:21
        m.insert("KeyB", 31); // index.js:86:21
        m.insert("KeyH", 32); // index.js:86:21
        m.insert("KeyN", 33); // index.js:86:21
        m.insert("KeyJ", 34); // index.js:86:21
        m.insert("KeyM", 35); // index.js:86:21
        m.insert("KeyQ", 36); // index.js:86:21
        m.insert("Digit2", 37); // index.js:86:21
        m.insert("KeyW", 38); // index.js:86:21
        m.insert("Digit3", 39); // index.js:86:21
        m.insert("KeyE", 40); // index.js:86:21
        m.insert("KeyR", 41); // index.js:86:21
        m.insert("Digit5", 42); // index.js:86:21
        m.insert("KeyT", 43); // index.js:86:21
        m.insert("Digit6", 44); // index.js:86:21
        m.insert("KeyY", 45); // index.js:86:21
        m.insert("Digit7", 46); // index.js:86:21
        m.insert("KeyU", 47); // index.js:86:21
        m.insert("KeyI", 48); // index.js:86:21
        m.insert("Digit9", 49); // index.js:86:21
        m.insert("KeyO", 50); // index.js:86:21
        m.insert("Digit0", 51); // index.js:86:21
        m.insert("KeyP", 52); // index.js:86:21
        m
    };
}
