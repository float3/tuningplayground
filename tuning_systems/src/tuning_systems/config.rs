use core::panic;
use std::sync::RwLock;

use super::helpers::find_coprimes;

pub(crate) static OCTAVE_SIZE: RwLock<usize> = RwLock::new(12);
pub(crate) static STEP_SIZE: RwLock<usize> = RwLock::new(1);
//pub(crate) static TUNING_SYSTEM: RwLock<TuningSystem> = RwLock::new(TuningSystem::EqualTemperament);

pub(crate) const C4: f64 = 261.6256;
pub(crate) const C0: f64 = C4 / 16.0;
pub(crate) const CN1: f64 = C4 / 32.0;

pub(crate) const A4: f64 = 440.0;
pub(crate) const A0: f64 = A4 / 16.0;
pub(crate) const AN1: f64 = A4 / 32.0;

pub fn set_octave_size(size: usize) {
    let mut octave_size = OCTAVE_SIZE.write().unwrap();
    *octave_size = size;
    drop(octave_size);
}

pub fn set_step_size(size: usize) {
    if !find_coprimes(*OCTAVE_SIZE.read().unwrap()).contains(&size) {
        panic!("Step size must be a coprime of the octave size")
    }
    let mut step_size = STEP_SIZE.write().unwrap();
    *step_size = size;
    drop(step_size);
}
