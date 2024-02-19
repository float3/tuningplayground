use crate::{Fraction, OCTAVE_SIZE};

pub fn equal_temperament(tone: usize, octave_size: usize) -> Fraction {
    Fraction::new_with_base(tone as u32, octave_size as u32, 2)
}

pub fn equal_temperament_12(tone: usize) -> Fraction {
    equal_temperament(tone, 12)
}

pub fn equal_temperament_default(tone: usize) -> Fraction {
    equal_temperament(tone, *OCTAVE_SIZE.read().unwrap())
}
