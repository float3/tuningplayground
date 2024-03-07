use tuning_systems::{Tone, TuningSystem};

fn main() {
    construct_et_tone(0);
    construct_et_tone(1);
    construct_et_tone(2);
    construct_et_tone(3);
    construct_et_tone(4);
    construct_et_tone(5);
    construct_et_tone(6);
    construct_et_tone(7);
    construct_et_tone(8);
    construct_et_tone(9);
    construct_et_tone(10);
    construct_et_tone(11);
    construct_et_tone(12);
    construct_et_tone(13);
    construct_et_tone(24);
    construct_et_tone(36);
    construct_et_tone(48);
    construct_et_tone(57);
    construct_et_tone(60);
    construct_et_tone(69);
    construct_et_tone(72);

    let sixty_nine = construct_et_tone(69);
    assert_eq!(sixty_nine.name, "A4");
    //assert_eq!(sixty_nine.octave(), 5);
    assert!((sixty_nine.frequency() - 440.0).abs() < 0.0001);

    let zero = construct_just_intonation_tone(0);
    assert_eq!(zero.name, "CN1");
    //assert_eq!(zero.octave(), 0);
    assert_eq!(zero.frequency(), 8.1758);

    let sixty_nine = construct_just_intonation_tone(69);
    assert_eq!(sixty_nine.name, "A4");
    //assert_eq!(sixty_nine.octave(), 5);
    assert!((sixty_nine.frequency() - 440.0).abs() < 1.5);

    let s = "-1";
    assert_eq!(-1, s.parse::<i32>().unwrap());
}

fn construct_just_intonation_tone(arg: i32) -> Tone {
    Tone::new(TuningSystem::JustIntonation, arg as usize)
}

fn construct_et_tone(arg: u32) -> Tone {
    Tone::new(TuningSystem::EqualTemperament { octave_size: 12 }, arg as usize)
}
