// #[cfg(feature = "wasm-bindgen")]
// use wasm_bindgen::prelude::*;

use crate::{equal_temperament, Fraction, TuningSystem, CN1};

#[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
pub struct Tone {
    pub name: String,
    // #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(skip))]
    pub(crate) fraction: Fraction,
    pub(crate) tone_index: usize,
    pub(crate) tuning_system: TuningSystem,
}

impl Tone {
    // #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(constructor))]
    pub fn new(tuning_system: TuningSystem, tone_index: u32) -> Tone {
        Tone::new_with_octave_size(tuning_system, tone_index)
    }

    // #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(constructor))]
    pub(crate) fn new_with_octave_size(tuning_system: TuningSystem, tone_index: u32) -> Tone {
        let name = tuning_system.get_tone_name(tone_index);
        let fraction = tuning_system.get_fraction(tone_index);

        Tone {
            name: name.to_string(),
            fraction,
            tone_index,
            tuning_system,
        }
    }

    pub(crate) fn octave(&self) -> usize {
        self.tone_index / self.tuning_system.size()
    }

    pub fn cents(&self) -> f64 {
        let reference_freq: f64 = equal_temperament(self.tone_index as u32, self.tuning_system.size() as u32).into();
        let comparison_freq: f64 = self.frequency();
        1200f64 * (comparison_freq / reference_freq).log2()
    }

    pub fn frequency(&self) -> f64 {
        self.fraction.f64() * CN1
    }
}
