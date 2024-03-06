use keymapping::{GERMAN_KEYMAP, US_KEYMAP};
use music21_rs::Pitch;
use tuning_systems::{Tone, TuningSystem, TypeAlias};
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

#[wasm_bindgen(start)]
pub(crate) fn main() {
    #[cfg(debug_assertions)]
    debug("main");
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
}

#[wasm_bindgen]
extern "C" {
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);

    fn createTone(
        index: usize,
        frequency: f64,
        cents: f64,
        name: String,
        tuning_system: JsValue,
    ) -> JsValue;
}

#[wasm_bindgen]
pub fn get_tone(index: usize) -> JsValue {
    #[cfg(debug_assertions)]
    debug("get_tone");

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

#[wasm_bindgen]
pub fn get_tuning_size() -> TypeAlias {
    #[cfg(debug_assertions)]
    debug("get_tuning_size");
    unsafe { TUNING_SYSTEM.size() }
}

#[wasm_bindgen]
pub fn from_keymap(key: &str) -> i32 {
    #[cfg(debug_assertions)]
    debug("from_keymap");
    US_KEYMAP.get(key).unwrap_or(&-1).clone()
}

#[wasm_bindgen]
pub fn convert_notes(notes: Vec<JsValue>) -> String {
    format!(
        "L: 1/1 \n[{}]",
        notes
            .iter()
            .map(|note| {
                // TODO: use music21 pitch struct here instead of regex
                let pitch: Pitch = Pitch::new(note.as_string().unwrap().replace("N1", "-1"));
                let octave_str;
                match pitch.octave {
                    Some(octave) => {
                        if octave == -1 {
                            let octave_number = octave;
                            if octave_number == 4 {
                                octave_str = "".to_string()
                            } else if octave_number < 4 {
                                octave_str = ",".repeat((4 - octave_number) as usize)
                            } else if octave_number > 4 {
                                octave_str = "'".repeat((octave_number - 4) as usize)
                            } else {
                                octave_str = "".to_string()
                            }
                        } else {
                            octave_str = ",,,,,,".to_string()
                        }
                    }
                    None => panic!("didn't supply octave"),
                }
                format!(
                    "{}{}{}",
                    pitch.accidental.replace("#", "^").replace("b", "_"),
                    pitch.name,
                    octave_str
                )

                // let note = note.as_string().unwrap();
                // let re = Regex::new(r"([A-G])([#b]*)(N1|\d+)").unwrap();
                // if let Some(cap) = re.captures(&note) {
                //     let pitch = &cap[1];
                //     let mut accidental = cap[2].to_string();
                //     let octave_str = &cap[3];
                //     accidental = accidental.replace("#", "^").replace("b", "_");
                //     let mut formatted_octave = String::new();
                //     if octave_str != "N1" {
                //         let octave_number = octave_str.parse::<i32>().unwrap();
                //         if octave_number == 4 {
                //             formatted_octave = "".to_string();
                //         } else if octave_number < 4 {
                //             formatted_octave = ",".repeat((4 - octave_number) as usize);
                //         } else if octave_number > 4 {
                //             formatted_octave = "'".repeat((octave_number - 4) as usize);
                //         }
                //     } else {
                //         formatted_octave = ",,,,,,".to_string();
                //     }
                //     return format!("{}{}{}", accidental, pitch, formatted_octave);
                // }
                // note
            })
            .collect::<Vec<String>>()
            .join("")
    )
}

#[wasm_bindgen]
pub fn set_tuning_system(tuning_system: &str, octave_size: TypeAlias, step_size: TypeAlias) {
    #[cfg(debug_assertions)]
    debug("set_tuning_system");
    match TuningSystem::new(tuning_system, octave_size, step_size) {
        Some(tuning_system) => unsafe {
            TUNING_SYSTEM = tuning_system;
            OCTAVE_SIZE = octave_size;
            STEP_SIZE = step_size;
        },
        None => {}
    }
}
