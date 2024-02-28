use tuning_systems::{Tone, TuningSystem};
use wasm_bindgen::prelude::*;

#[cfg(feature = "mini-alloc")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    debug("main");
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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
pub fn get_tone(tuning: &str, index: usize) -> JsValue {
    debug("get_tone");
    let tuning: Result<TuningSystem, _> = tuning.parse();
    let tone: Tone = match tuning {
        Ok(tuning) => Tone::new(tuning, index),
        Err(_) => panic!("unknown tuning system"),
    };

    createTone(
        index,
        tone.frequency(),
        tone.cents(),
        tone.name,
        JsValue::NULL,
    )
}

#[wasm_bindgen]
pub fn set_octave_size(size: usize) {
    debug("set_octave_size");
    tuning_systems::set_octave_size(size)
}

#[wasm_bindgen]
pub fn get_tuning_size(tuning: &str) -> usize {
    debug("get_tuning_size");
    let tuning: Result<TuningSystem, _> = tuning.parse();
    match tuning {
        Ok(tuning) => tuning.size(),
        Err(_) => panic!("unknown tuning system"),
    }
}
