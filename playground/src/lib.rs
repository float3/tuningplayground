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
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
}

#[wasm_bindgen]
pub fn get_frequency(tuning: &str, index: usize) -> f64 {
    let tuning: Result<TuningSystem, _> = tuning.parse();
    log("get_frequency");
    match tuning {
        Ok(tuning) => Tone::new(tuning, index).frequency(),
        Err(_) => panic!("unknown tuning system"),
    }
}

#[wasm_bindgen]
pub fn set_octave_size(size: usize) {
    debug("set_octave_size");
    tuning_systems::set_octave_size(size)
}
