use tuning_systems::{self, TuningSystem};
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
pub fn get_ratio(tuning: &str, index: usize, size: u32) -> f64 {
    let tuning: Result<TuningSystem, _> = tuning.parse();
    match tuning {
        Ok(tuning) => tuning_systems::get_ratio(tuning, index, Some(size)),
        Err(_) => panic!("unknown tuning system"),
    }
}
