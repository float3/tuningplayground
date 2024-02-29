use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref GERMAN_KEYMAP: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("+", 50);
        m.insert("1", 30);
        m.insert("2", 32);
        m.insert("3", 34);
        m.insert("5", 37);
        m.insert("6", 39);
        m.insert("8", 42);
        m.insert("9", 44);
        m.insert("0", 46);
        m.insert("&lt;", 19);
        m.insert("A", 20);
        m.insert("B", 28);
        m.insert("C", 24);
        m.insert("E", 35);
        m.insert("F", 25);
        m.insert("G", 27);
        m.insert("I", 43);
        m.insert("J", 30);
        m.insert("K", 32);
        m.insert(",", 33);
        m.insert("L", 34);
        m.insert(".", 35);
        m.insert("-", 36);
        m.insert("Ä", 37);
        m.insert("#", 38);
        m.insert("M", 31);
        m.insert("N", 29);
        m.insert("O", 45);
        m.insert("P", 47);
        m.insert("Ü", 48);
        m.insert("Q", 31);
        m.insert("R", 36);
        m.insert("S", 22);
        m.insert("T", 38);
        m.insert("U", 41);
        m.insert("V", 26);
        m.insert("W", 33);
        m.insert("X", 23);
        m.insert("Y", 21);
        m.insert("Z", 40);
        m.insert("Ins", 3);
        m.insert("Del", 1);
        m.insert("Home", 8);
        m.insert("End", 6);
        m.insert("PgUp", 13);
        m.insert("PgDown", 10);
        m.insert("F1", 0);
        m.insert("F2", 2);
        m.insert("F3", 4);
        m.insert("F4", 5);
        m.insert("F5", 7);
        m.insert("F6", 9);
        m.insert("F7", 11);
        m.insert("F8", 12);
        m.insert("F9", 14);
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

// m.insert("KeyY", 45);
// m.insert("KeyS", 25);
// m.insert("KeyD", 27);
// m.insert("KeyC", 28);
// m.insert("KeyV", 29);
// m.insert("KeyG", 30);
// m.insert("KeyB", 31);
// m.insert("KeyH", 32);
// m.insert("KeyN", 33);
// m.insert("KeyJ", 34);
// m.insert("KeyM", index.js:80:13
// 35); index.js:76:17
// 36); index.js:76:17
// m.insert("KeyW", index.js:80:13
// 38); index.js:76:17
// m.insert("KeyE", index.js:80:13
// 40); index.js:76:17
// m.insert("KeyR", index.js:80:13
// 41); index.js:76:17
// m.insert("KeyT", index.js:80:13
// 43); index.js:76:17
// m.insert("KeyZ", index.js:80:13
// 24); index.js:76:17
// m.insert("KeyU", index.js:80:13
// 47); index.js:76:17
// m.insert("KeyI", index.js:80:13
// 48); index.js:76:17
// m.insert("KeyO", index.js:80:13
// 50); index.js:76:17
// m.insert("KeyP", index.js:80:13
// 52); index.js:76:17
// m.insert("", index.js:80:13
// m.insert("Equal", index.js:80:13
// m.insert("Digit3", index.js:80:13
// m.insert("Digit1", index.js:80:13
// m.insert("Digit2", index.js:80:13
// 37); index.js:76:17
// m.insert("Digit3", index.js:80:13
// 39); index.js:76:17
// m.insert("Digit4", index.js:80:13
// m.insert("Digit5", index.js:80:13
// 42); index.js:76:17
// m.insert("Digit6", index.js:80:13
// 44); index.js:76:17
// m.insert("Digit7", index.js:80:13
// 46); index.js:76:17
// m.insert("Digit8", index.js:80:13
// m.insert("Digit9", index.js:80:13
// 49); index.js:76:17
// m.insert("Digit0", index.js:80:13
// 51);
    static ref ENGLISH_KEYMAP: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("a", 8);
        m.insert("z", 9);
        m.insert("s", 10);
        m.insert("x", 11);
        m.insert("c", 12);
        m.insert("f", 13);
        m.insert("v", 14);
        m.insert("g", 15);
        m.insert("b", 16);
        m.insert("n", 17);
        m.insert("j", 18);
        m.insert("m", 19);
        m.insert("k", 20);
        m.insert(",", 21);
        m.insert("l", 22);
        m.insert(".", 23);
        m.insert("/", 24);
        m.insert("'", 25);
        m.insert("\\", 26);
        m.insert("`", 27);
        m
    };

}
