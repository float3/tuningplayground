use crate::Fraction;

use super::algorithms::equal_temperament;

pub struct Tone {
    name: String,
    ratio: Fraction,
    octave: u32,
    octave_size: u32,
    tone_index: u32,
}

impl Tone {
    pub fn new(name: &str, ratio: Fraction, octave: u32, tone_index: u32) -> Tone {
        Tone {
            name: name.to_string(),
            ratio,
            octave,
            octave_size: 12,
            tone_index,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ratio(&self) -> Fraction {
        self.ratio
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
        let ratio: f64 = self.ratio.into();
        ratio + 2f64.powf(self.octave as f64)
    }
}
