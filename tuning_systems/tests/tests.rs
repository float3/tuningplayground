extern crate tuning_systems;

use tuning_systems::{Fraction, Tone, TuningSystem};

const EQ: TuningSystem = TuningSystem::EqualTemperament { octave_size: 12 };

#[cfg(test)]
#[test]
fn test_octave() {
    let twoone: f64 = Fraction::new(2, 1).into();
    let ratio: f64 = TuningSystem::JustIntonation.get_fraction(12).into();
    assert_eq!(ratio, twoone);
    let ratio: f64 = TuningSystem::JustIntonation24.get_fraction(24).into();
    assert_eq!(ratio, twoone);
    let ratio: f64 = EQ.get_fraction(12).into();
    assert_eq!(ratio, twoone);
}

#[cfg(test)]
#[test]
fn test_et() {
    let zero = construct_et_tone(0);
    assert_eq!(zero.name, "CN1");
    //assert_eq!(zero.octave(), 0);
    assert_eq!(zero.frequency(), 8.1758);

    let sixty_nine = construct_et_tone(69);
    assert_eq!(sixty_nine.name, "A4");
    //assert_eq!(sixty_nine.octave(), 5);
    assert!((sixty_nine.frequency() - 440.0).abs() < 0.0001);
}

fn construct_et_tone(index: u32) -> Tone {
    Tone::new(EQ, index as usize)
}

#[test]
fn test_just_intonation() {
    let zero = construct_just_intonation_tone(0);
    assert_eq!(zero.name, "CN1");
    //assert_eq!(zero.octave(), 0);
    assert_eq!(zero.frequency(), 8.1758);

    let sixty_nine = construct_just_intonation_tone(69);
    assert_eq!(sixty_nine.name, "A4");
    //assert_eq!(sixty_nine.octave(), 5);
    assert!((sixty_nine.frequency() - 440.0).abs() < 1.5);
}

fn construct_just_intonation_tone(arg: i32) -> Tone {
    Tone::new(TuningSystem::JustIntonation, arg as usize)
}
