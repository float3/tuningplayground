// #[cfg(feature = "wasm-bindgen")]
// use wasm_bindgen::prelude::*;

use crate::{
    equal_temperament, get_fraction, Fraction, TuningSystem, CN1, OCTAVE_SIZE, TWELVE_TONE_NAMES,
};

use super::tunings;

#[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
pub struct Tone {
    name: String,
    // #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(skip))]
    fraction: Fraction,
    octave: usize,
    octave_size: usize,
    tone_index: usize,
    tuning_system: TuningSystem,
}

impl Tone {
    // #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(constructor))]
    pub fn new(tuning_system: TuningSystem, tone_index: usize) -> Tone {
        Tone::new_with_octave_size(tuning_system, OCTAVE_SIZE, tone_index)
    }

    // #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(constructor))]
    pub fn new_with_octave_size(
        tuning_system: TuningSystem,
        octave_size: usize,
        tone_index: usize,
    ) -> Tone {
        let name = TWELVE_TONE_NAMES[tone_index % OCTAVE_SIZE]; // check what happens for negative tone_index
        let octave = tone_index / octave_size;
        let adjusted_octave: i32 = octave as i32 - 1;
        let name: String = if adjusted_octave < 0 {
            format!("{}N{}", name, -adjusted_octave)
        } else {
            format!("{}{}", name, adjusted_octave)
        };
        let mut fraction = get_fraction(tuning_system, tone_index);

        if fraction.base == 0 {
            fraction.numerator += (2u32.pow(octave as u32) - 1) * fraction.denominator;
        }
        Tone {
            name: name.to_string(),
            fraction,
            octave,
            octave_size,
            tone_index,
            tuning_system,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn octave(&self) -> usize {
        self.octave
    }

    pub fn octave_size(&self) -> usize {
        self.octave_size
    }

    pub fn tone_index(&self) -> usize {
        self.tone_index
    }

    pub fn cents(&self) -> f64 {
        let reference_freq: f64 = equal_temperament(self.tone_index, self.octave_size).into();
        let comparison_freq: f64 = self.frequency();
        1200f64 * (comparison_freq / reference_freq).log2()
    }

    pub fn frequency(&self) -> f64 {
        let ratio: f64 = self.fraction.into();
        ratio * CN1
    }
}
