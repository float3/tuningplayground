use keymapping::US_KEYMAP;
use std::collections::HashMap;

use std::sync::Mutex;
use std::sync::OnceLock;
use tuning_systems::{Tone, TuningSystem, TypeAlias};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "mini-alloc")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[cfg(feature = "console_error_panic_hook")]
pub(crate) fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

static OCTAVE_SIZE: Mutex<usize> = Mutex::new(12);
static STEP_SIZE: Mutex<usize> = Mutex::new(7);
static TUNING_SYSTEM: Mutex<TuningSystem> =
    Mutex::new(TuningSystem::EqualTemperament { octave_size: 12 });
static CHORD_NAME: Mutex<String> = Mutex::new(String::new());

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub(crate) fn main() {
    #[cfg(debug_assertions)]
    log("main");
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);

    fn createTone(
        index: usize,
        frequency: f64,
        cents: f64,
        name: String,
        tuning_system: JsValue,
    ) -> JsValue;
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_tone(index: usize) -> JsValue {
    #[cfg(debug_assertions)]
    log("get_tone");

    let tun_sys: TuningSystem = *TUNING_SYSTEM.lock().expect("couldn't lock");

    let tone: Tone = Tone::new(tun_sys, index);

    createTone(
        index,
        tone.frequency(),
        tone.cents(),
        tone.name,
        JsValue::NULL,
    )
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_tuning_size() -> TypeAlias {
    #[cfg(debug_assertions)]
    log("get_tuning_size");
    *OCTAVE_SIZE.lock().expect("couldn't lock") as TypeAlias
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn from_keymap(key: &str) -> i32 {
    #[cfg(debug_assertions)]
    log("from_keymap");
    *US_KEYMAP.get(key).unwrap_or(&-1)
}

type LUTType = HashMap<i32, String>;

static CHORD_LUT: OnceLock<LUTType> = OnceLock::new();

fn static_data() -> &'static LUTType {
    CHORD_LUT.get_or_init(|| serde_json::from_str(include_str!("../chords.json")).unwrap())
}

pub fn convert_notes_core(input: Vec<String>) -> String {
    //return "L: 1/1 \n\"C\"[C E G]".to_string();
    let mut bitmask = 0;
    let mut notes = Vec::new();
    let mut bass: String = "".to_string();
    let first: bool = true;

    input.into_iter().for_each(|note_str| {
        let mut chars = note_str.chars().peekable();
        let name = chars.next().expect("no name");

        if !('A'..='G').contains(&name) {
            panic!("Invalid note name");
        }

        let accidental = match chars.peek() {
            Some('b') => {
                chars.next();
                if chars.peek() == Some(&'b') {
                    chars.next();
                    "bb".to_string()
                } else {
                    "b".to_string()
                }
            }
            Some('#') => {
                chars.next();
                if chars.peek() == Some(&'#') {
                    chars.next();
                    "##".to_string()
                } else {
                    "#".to_string()
                }
            }
            _ => "".to_string(),
        };

        let full_name = name.to_string() + &accidental;

        if first {
            bass.clone_from(&full_name);
        }

        let index = match full_name.as_str() {
            "B#" | "C" | "Dbb" => 0,
            "B##" | "C#" | "Db" => 1,
            "C##" | "D" | "Ebb" => 2,
            "D#" | "Eb" | "Fbb" => 3,
            "D##" | "E" | "Fb" => 4,
            "E#" | "F" | "Gbb" => 5,
            "E##" | "F#" | "Gb" => 6,
            "F##" | "G" | "Abb" => 7,
            "G#" | "Ab" => 8,
            "G##" | "A" | "Bbb" => 9,
            "A#" | "Bb" | "Cbb" => 10,
            "A##" | "B" | "Cb" => 11,
            _ => panic!("Invalid note"),
        };

        bitmask |= 1 << index;

        let octave_modifier = note_str
            .replace("N1", "-1")
            .chars()
            .last()
            .unwrap_or('4')
            .to_digit(10)
            .unwrap_or(4) as isize
            - 4;

        let octave_str = if octave_modifier < 0 {
            ",".repeat(octave_modifier.unsigned_abs())
        } else {
            "'".repeat(octave_modifier as usize)
        };

        notes.push(format!(
            "{}{}{}",
            accidental.replace('#', "^").replace('b', "_"),
            name,
            octave_str
        ));
    });

    let chord: String = match static_data().get(&bitmask) {
        Some(chord) => chord.clone(),
        None => "".to_string(),
    };

    *CHORD_NAME.lock().expect("couldn't lock") = chord.clone();

    let notes = notes.join(" ");

    format!("L: 1/1 \n\"{}\"[{}]", chord, notes)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_chord_name() -> String {
    #[cfg(debug_assertions)]
    log("get_chord_name");
    CHORD_NAME.lock().expect("couldn't lock").clone()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_notes(notes: Vec<String>) -> String {
    #[cfg(debug_assertions)]
    log("convert_notes");
    convert_notes_core(notes)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn set_tuning_system(tuning_system: &str, octave_size: TypeAlias, step_size: TypeAlias) {
    #[cfg(debug_assertions)]
    log("set_tuning_system");
    let tuning_system: Option<TuningSystem> = match tuning_system.to_lowercase().as_str() {
        "stepmethod" => Some(TuningSystem::StepMethod {
            octave_size,
            step_size,
        }),
        "equaltemperament" => Some(TuningSystem::EqualTemperament { octave_size }),
        "justintonation" => Some(TuningSystem::JustIntonation),
        "justintonation24" => Some(TuningSystem::JustIntonation24),
        "pythagoreantuning" => Some(TuningSystem::PythagoreanTuning),
        "fivelimit" => Some(TuningSystem::FiveLimit),
        "elevenlimit" => Some(TuningSystem::ElevenLimit),
        "fortythreetone" => Some(TuningSystem::FortyThreeTone),
        "indian" => Some(TuningSystem::Indian),
        "indianalt" => Some(TuningSystem::IndianAlt),
        "indianfull" => Some(TuningSystem::Indian22),
        // "thai" => Some(TuningSystem::Thai),
        // "javanese" => Some(TuningSystem::Javanese),
        "wholetone" => Some(TuningSystem::WholeTone),
        "quartertone" => Some(TuningSystem::QuarterTone),
        _ => None,
    };
    match tuning_system {
        Some(tuning_system) => {
            *TUNING_SYSTEM.lock().expect("couldn't lock") = tuning_system;
            *OCTAVE_SIZE.lock().expect("couldn't lock") = octave_size;
            *STEP_SIZE.lock().expect("couldn't lock") = step_size;
        }
        None => {
            #[cfg(debug_assertions)]
            error("Invalid tuning system");
        }
    }
}
