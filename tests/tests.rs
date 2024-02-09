#[cfg(test)]
use tuning_systems::{luts, tuning_systems::TuningSystem};

#[test]
fn test_octave() {
    let ratio = luts::access_lut(TuningSystem::JustIntonation, 12);
    assert_eq!(ratio, luts::Fraction(1, 1));
}
