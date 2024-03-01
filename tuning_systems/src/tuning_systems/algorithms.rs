use crate::{Fraction, TuningSystem, OCTAVE_SIZE, STEP_SIZE};

pub(crate) fn equal_temperament(tone: usize, octave_size: usize) -> Fraction {
    Fraction::new_with_base(tone as u32, octave_size as u32, 2)
}

pub(crate) fn equal_temperament_12(tone: usize) -> Fraction {
    equal_temperament(tone, 12)
}

pub(crate) fn equal_temperament_default(tone: usize) -> Fraction {
    equal_temperament(tone, *OCTAVE_SIZE.read().expect("couldn't read octave size"))
}

fn get_ratio_from_step_algorithm(n: i32, max: i32) -> Fraction {
    let stepsize = *STEP_SIZE.read().expect("couldn't read step size");
    let ratio = TuningSystem::JustIntonation.get_fraction_from_table(stepsize);
    let n2 = n % max;
    let mut current_ratio = Fraction::new(1, 1);
    let mut current_idx = 0;
    let two = Fraction::new(2, 1);
    while current_idx != n2 {
        current_ratio *= ratio;
        current_idx += stepsize as i32;
        current_idx %= max;
        if current_ratio > two {
            current_ratio /= two;
        }
    }
    let octaves = (n / max) as f64;
    current_ratio *= Fraction::new(2u32.pow(octaves as u32), 1);
    return current_ratio;
}
