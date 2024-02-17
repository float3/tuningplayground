use std::ops::Mul;

use crate::{Fraction, A0, OCTAVE_SIZE};

use super::algorithms::equal_temperament;

pub struct Tone {
    name: String,
    fraction: Fraction,
    octave: u32,
    octave_size: u32,
    tone_index: u32,
}

impl Tone {
    pub fn new(name: &str, mut fraction: Fraction, octave: u32, tone_index: u32) -> Tone {
        fraction.numerator += 2u32.pow(octave) * fraction.denominator;
        Tone {
            name: name.to_string(),
            fraction,
            octave,
            octave_size: *OCTAVE_SIZE,
            tone_index,
        }
    }

    pub fn new_with_octave_size(
        name: &str,
        fraction: Fraction,
        octave: u32,
        octave_size: u32,
        tone_index: u32,
    ) -> Tone {
        Tone {
            name: name.to_string(),
            fraction,
            octave,
            octave_size,
            tone_index,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fraction(&self) -> Fraction {
        self.fraction
    }

    pub fn octave(&self) -> u32 {
        self.octave
    }

    pub fn octave_size(&self) -> u32 {
        self.octave_size
    }

    pub fn tone_index(&self) -> u32 {
        self.tone_index
    }

    pub fn cents(&self) -> f64 {
        let reference_freq: f64 = equal_temperament(self.tone_index, self.octave_size).into();
        let comparison_freq: f64 = self.frequency();
        1200f64 * (comparison_freq / reference_freq).log2()
    }

    pub fn frequency(&self) -> f64 {
        let ratio: f64 = self.fraction.into();
        A0.mul(ratio)
    }
}
