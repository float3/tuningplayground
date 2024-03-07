use keymapping::US_KEYMAP;
use music21_rs::Pitch;
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

static mut OCTAVE_SIZE: usize = 12;
static mut STEP_SIZE: usize = 7;
static mut TUNING_SYSTEM: TuningSystem = TuningSystem::EqualTemperament { octave_size: 12 };

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

    let tone: Tone;
    unsafe {
        tone = Tone::new(TUNING_SYSTEM, index);
    }

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
    unsafe { TUNING_SYSTEM.size() }
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
            .map(|note| {
                let note = note.split('/').collect::<Vec<&str>>()[0];
                let pitch: Pitch = Pitch::new(note.replace("N1", "-1"));
                let octave_str = match pitch.octave {
                    Some(octave) => {
                        let diff = octave as isize - 4;
                        if diff < 0 {
                            ",".repeat(diff.unsigned_abs())
                        } else {
                            "'".repeat(diff as usize)
                        }
                    }
                    None => "".to_string(),
                };

                format!(
                    "{}{}{}",
                    pitch.accidental.replace('#', "^").replace('b', "_"),
                    pitch.name,
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
        Some(tuning_system) => unsafe {
            TUNING_SYSTEM = tuning_system;
            OCTAVE_SIZE = octave_size;
            STEP_SIZE = step_size;
        },
        None => {
            #[cfg(debug_assertions)]
            error("Invalid tuning system");
        }
    }
}
