struct Fraction(i32, i32);

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

const JUST_INTONATION: [Fraction; 12] = [
    Fraction(1, 1),
    Fraction(17, 16),
    Fraction(9, 8),
    Fraction(19, 16),
    Fraction(5, 4),
    Fraction(4, 3),
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
    Fraction(4, 3),
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
