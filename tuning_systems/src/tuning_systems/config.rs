use std::sync::Mutex;

use crate::TuningSystem;

pub static OCTAVE_SIZE: Mutex<usize> = Mutex::new(12);
pub static TUNING_SYSTEM: Mutex<TuningSystem> = Mutex::new(TuningSystem::EqualTemperament);

pub static C4: f64 = 261.6256;
pub static C0: f64 = C4 / 16.0;
pub static CN1: f64 = C4 / 32.0;

pub static A4: f64 = 440.0;
pub static A0: f64 = A4 / 16.0;
pub static AN1: f64 = A4 / 32.0;

pub fn set_octave_size(size: usize) {
    let mut octave_size = OCTAVE_SIZE.lock().unwrap();
    *octave_size = size;
    drop(octave_size);
}
