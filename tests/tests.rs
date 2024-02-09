#[cfg(test)]
#[test]
fn test_octave() {
    use tuning_systems::{
        fraction,
        tuning_systems::{get_ratio, TuningSystem},
    };

    let twoone = fraction::Fraction(2, 1).into();

    let ratio = get_ratio(TuningSystem::JustIntonation, 12, None);
    assert_eq!(ratio, twoone);
    let ratio = get_ratio(TuningSystem::JustIntonation24, 24, None);
    assert_eq!(ratio, twoone);
    let ratio = get_ratio(TuningSystem::EqualTemperament, 12, Some(12));
    assert_eq!(ratio, twoone);
}
