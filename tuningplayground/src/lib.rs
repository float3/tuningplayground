use std::sync::Mutex;

use keymapping::US_KEYMAP;
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

pub fn convert_notes_core(notes: Vec<String>) -> String {
    format!(
        "L: 1/1 \n[{}]",
        notes
            .iter()
            .map(|note_str| {
                let mut chars = note_str.chars().peekable();
                let name = chars.next().expect("no name");

                if !('A'..='G').contains(&name) {
                    panic!("Invalid note name");
                }

                let accidental;

                match chars.peek() {
                    Some('b') => {
                        chars.next();
                        if chars.peek() == Some(&'b') {
                            chars.next();
                            accidental = "bb".to_string();
                        } else {
                            accidental = "b".to_string();
                        }
                    }
                    Some('#') => {
                        chars.next();
                        if chars.peek() == Some(&'#') {
                            chars.next();
                            accidental = "##".to_string();
                        } else {
                            accidental = "#".to_string();
                        }
                    }
                    _ => {
                        accidental = "".to_string();
                    }
                }

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

                format!(
                    "{}{}{}",
                    accidental.replace('#', "^").replace('b', "_"),
                    name,
                    octave_str
                )
            })
            .collect::<Vec<String>>()
            .join(" ")
    )
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
