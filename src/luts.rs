use crate::tuning_systems::TuningSystem;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fraction(pub i32, pub i32);

impl From<Fraction> for f64 {
    fn from(num: Fraction) -> f64 {
        num.0 as f64 / num.1 as f64
    }
}

impl From<(i32, i32)> for Fraction {
    fn from(nums: (i32, i32)) -> Fraction {
        Fraction(nums.0, nums.1)
    }
}

pub fn access_lut(tuning_sytem: TuningSystem, index: usize) -> Fraction {
    let lut: &[Fraction] = match tuning_sytem {
        TuningSystem::JustIntonation => &JUST_INTONATION,
        TuningSystem::JustIntonation24 => &JUST_INTONATION_24,
        TuningSystem::PythogoreanTuning => &PYTHOGREAN_TUNING,
        TuningSystem::FiveLimit => &FIVE_LIMIT,
        TuningSystem::ElevenLimit => &ELEVEN_LIMIT,
        TuningSystem::FortyThreeTone => &FORTYTHREE_TONE,
        TuningSystem::Indian => &INDIAN_SCALE,
        TuningSystem::IndianFull => &INDIAN_SCALE_FULL,

        TuningSystem::StepMethod | TuningSystem::EqualTemperament => panic!(),
    };

    lut[index % lut.len()]
}

const JUST_INTONATION: [Fraction; 12] = [
    Fraction(1, 1),
    Fraction(17, 16),
    Fraction(9, 8),
    Fraction(19, 16),
    Fraction(5, 4),
    Fraction(4, 3), // 21/16
    Fraction(45, 32),
    Fraction(3, 2),
    Fraction(51, 32),
    Fraction(27, 16),
    Fraction(57, 32),
    Fraction(15, 8),
];

const JUST_INTONATION_24: [Fraction; 24] = [
    Fraction(1, 1),
    Fraction(33, 32),
    Fraction(17, 16),
    Fraction(35, 32),
    Fraction(9, 8),
    Fraction(37, 32),
    Fraction(19, 16),
    Fraction(39, 32),
    Fraction(5, 4),
    Fraction(41, 32),
    Fraction(4, 3), // 85/64
    Fraction(11, 8),
    Fraction(45, 32),
    Fraction(93, 64),
    Fraction(3, 2),
    Fraction(99, 64),
    Fraction(51, 32),
    Fraction(105, 64),
    Fraction(27, 16),
    Fraction(111, 64),
    Fraction(57, 32),
    Fraction(117, 64),
    Fraction(15, 8),
    Fraction(31, 16),
];

const PYTHOGREAN_TUNING: [Fraction; 12] = [
    Fraction(1, 1),
    Fraction(256, 243),
    Fraction(9, 8),
    Fraction(32, 27),
    Fraction(81, 64),
    Fraction(4, 3),
    Fraction(729, 512),
    Fraction(3, 2),
    Fraction(27, 16),
    Fraction(16, 9),
    Fraction(243, 128),
    Fraction(15, 8),
];

const FIVE_LIMIT: [Fraction; 12] = [
    Fraction(1, 1),
    Fraction(16, 15),
    Fraction(9, 8),
    Fraction(6, 5),
    Fraction(5, 4),
    Fraction(4, 3),
    Fraction(64, 45),
    Fraction(3, 2),
    Fraction(8, 5),
    Fraction(5, 3),
    Fraction(16, 9),
    Fraction(15, 8),
];

const ELEVEN_LIMIT: [Fraction; 29] = [
    Fraction(1, 1),
    Fraction(12, 11),
    Fraction(11, 10),
    Fraction(10, 9),
    Fraction(9, 8),
    Fraction(8, 7),
    Fraction(7, 6),
    Fraction(6, 5),
    Fraction(11, 9),
    Fraction(5, 4),
    Fraction(14, 11),
    Fraction(9, 7),
    Fraction(4, 3),
    Fraction(11, 8),
    Fraction(7, 5),
    Fraction(10, 7),
    Fraction(16, 11),
    Fraction(3, 2),
    Fraction(14, 9),
    Fraction(11, 7),
    Fraction(8, 5),
    Fraction(18, 11),
    Fraction(5, 3),
    Fraction(12, 7),
    Fraction(7, 4),
    Fraction(16, 9),
    Fraction(9, 5),
    Fraction(20, 11),
    Fraction(11, 6),
];

const FORTYTHREE_TONE: [Fraction; 43] = [
    Fraction(1, 1),
    Fraction(81, 80),
    Fraction(33, 32),
    Fraction(21, 20),
    Fraction(16, 15),
    Fraction(12, 11),
    Fraction(11, 10),
    Fraction(10, 9),
    Fraction(9, 8),
    Fraction(8, 7),
    Fraction(7, 6),
    Fraction(32, 27),
    Fraction(6, 5),
    Fraction(11, 9),
    Fraction(5, 4),
    Fraction(14, 11),
    Fraction(9, 7),
    Fraction(21, 16),
    Fraction(4, 3),
    Fraction(27, 20),
    Fraction(11, 8),
    Fraction(7, 5),
    Fraction(10, 7),
    Fraction(16, 11),
    Fraction(40, 27),
    Fraction(3, 2),
    Fraction(23, 21),
    Fraction(14, 9),
    Fraction(11, 7),
    Fraction(8, 5),
    Fraction(18, 11),
    Fraction(5, 3),
    Fraction(27, 16),
    Fraction(12, 7),
    Fraction(7, 4),
    Fraction(16, 8),
    Fraction(9, 5),
    Fraction(20, 11),
    Fraction(11, 6),
    Fraction(15, 8),
    Fraction(40, 21),
    Fraction(64, 33),
    Fraction(160, 81),
];

const INDIAN_SCALE: [Fraction; 8] = [
    Fraction(1, 1),
    Fraction(9, 8),
    Fraction(5, 4),
    Fraction(4, 3),
    Fraction(3, 2),
    Fraction(5, 3),
    Fraction(27, 16),
    Fraction(15, 8),
];

const INDIAN_SCALE_FULL: [Fraction; 22] = [
    Fraction(1, 1),
    Fraction(256, 243),
    Fraction(16, 15),
    Fraction(10, 9),
    Fraction(9, 8),
    Fraction(32, 27),
    Fraction(6, 5),
    Fraction(5, 4),
    Fraction(81, 64),
    Fraction(4, 3),
    Fraction(27, 20),
    Fraction(45, 32),
    Fraction(729, 512),
    Fraction(3, 2),
    Fraction(128, 81),
    Fraction(8, 5),
    Fraction(5, 3),
    Fraction(27, 16),
    Fraction(16, 9),
    Fraction(9, 5),
    Fraction(15, 8),
    Fraction(243, 128),
];
