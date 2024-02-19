use tuning_systems::{self, Tone, TuningSystem};
use wasm_bindgen::prelude::*;

#[cfg(feature = "mini-alloc")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[allow(dead_code)]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_frequency(tuning: &str, index: usize) -> f64 {
    let tuning: Result<TuningSystem, _> = tuning.parse();
    match tuning {
        Ok(tuning) => Tone::new(tuning, index).frequency(),
        Err(_) => panic!("unknown tuning system"),
    }
}

#[wasm_bindgen]
pub fn set_octave_size(size: usize) {
    let mut octave_size = tuning_systems::OCTAVE_SIZE.lock().unwrap();
    *octave_size = size;
}
