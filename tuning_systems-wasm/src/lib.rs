use tuning_systems::{self, TuningSystem};
use wasm_bindgen::prelude::*;

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
