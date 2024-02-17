use crate::{Fraction, OCTAVE_SIZE};

pub fn equal_temperament(tone: u32, octave_size: u32) -> Fraction {
    Fraction::new_with_base(tone, octave_size, 2)
}

pub fn equal_temperament_12(tone: u32) -> Fraction {
    equal_temperament(tone, 12)
}

pub fn equal_temperament_default(tone: u32) -> Fraction {
    equal_temperament(tone, *OCTAVE_SIZE)
}
