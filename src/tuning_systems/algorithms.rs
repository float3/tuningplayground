use crate::OCTAVE_SIZE;

pub fn equal_temperament(tone: u32) -> f64 {
    2f64.powf(tone as f64 / OCTAVE_SIZE.clone() as f64)
}
