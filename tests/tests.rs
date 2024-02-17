extern crate tuning_systems;

use tuning_systems::{equal_temperament_12, get_ratio, Fraction, Tone, TuningSystem};

#[cfg(test)]
#[test]
fn test_octave() {
    let twoone = Fraction::new(2, 1).into();
    let ratio = get_ratio(TuningSystem::JustIntonation, 12, None);
    assert_eq!(ratio, twoone);
    let ratio = get_ratio(TuningSystem::JustIntonation24, 24, None);
    assert_eq!(ratio, twoone);
    let ratio = get_ratio(TuningSystem::EqualTemperament, 12, Some(12));
    assert_eq!(ratio, twoone);
}

#[test]
fn test_et() {
    let zero = construct_et_tone(0);

    assert_eq!(zero.name(), "CN1");
    assert_eq!(zero.octave(), 0);
    assert_eq!(zero.frequency(), 8.1758);

    let sixty_nine = construct_et_tone(69);
    assert_eq!(sixty_nine.name(), "A4");
    assert_eq!(sixty_nine.octave(), 5);
    assert!((sixty_nine.frequency() - 440.0).abs() < 0.0001);
}

fn construct_et_tone(index: u32) -> Tone {
    let frac: Fraction = equal_temperament_12(index);
    let tone = Tone::new(frac, index);
    tone
}
