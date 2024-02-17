use crate::Fraction;

pub struct Tone {
    name: String,
    ratio: Fraction,
    octave: i32,
    cents: f64,
    frequency: f64,
}

impl Tone {
    pub fn new(name: &str, ratio: Fraction, octave: i32, cents: f64, frequency: f64) -> Tone {
        Tone {
            name: name.to_string(),
            ratio,
            octave,
            cents,
            frequency,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ratio(&self) -> Fraction {
        self.ratio
    }

    pub fn octave(&self) -> i32 {
        self.octave
    }

    pub fn cents(&self) -> f64 {
        self.cents
    }

    pub fn frequency(&self) -> f64 {
        self.frequency
    }
}
