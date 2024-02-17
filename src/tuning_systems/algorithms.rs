use crate::{Fraction, OCTAVE_SIZE};

pub fn equal_temperament(tone: u32) -> Fraction {
    Fraction::new_with_base(tone, *OCTAVE_SIZE, 2)
}
