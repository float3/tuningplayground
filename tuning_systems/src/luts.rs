use crate::Fraction;

pub(crate) const TWELVE_TONE_NAMES: [&str; 12] = ["C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab", "A", "A#/Bb", "B"];

pub(crate) const JUST_INTONATION: [Fraction; 12] = [
    Fraction::new(1, 1),
    Fraction::new(17, 16),
    Fraction::new(9, 8),
    Fraction::new(19, 16),
    Fraction::new(5, 4),
    Fraction::new(4, 3), // 21/16
    Fraction::new(45, 32),
    Fraction::new(3, 2),
    Fraction::new(51, 32),
    Fraction::new(27, 16),
    Fraction::new(57, 32),
    Fraction::new(15, 8),
];

pub(crate) const JUST_INTONATION_24: [Fraction; 24] = [
    Fraction::new(1, 1),
    Fraction::new(33, 32),
    Fraction::new(17, 16),
    Fraction::new(35, 32),
    Fraction::new(9, 8),
    Fraction::new(37, 32),
    Fraction::new(19, 16),
    Fraction::new(39, 32),
    Fraction::new(5, 4),
    Fraction::new(41, 32),
    Fraction::new(4, 3), // 85/64
    Fraction::new(11, 8),
    Fraction::new(45, 32),
    Fraction::new(93, 64),
    Fraction::new(3, 2),
    Fraction::new(99, 64),
    Fraction::new(51, 32),
    Fraction::new(105, 64),
    Fraction::new(27, 16),
    Fraction::new(111, 64),
    Fraction::new(57, 32),
    Fraction::new(117, 64),
    Fraction::new(15, 8),
    Fraction::new(31, 16),
];

pub(crate) const PYTHAGOREAN_TUNING: [Fraction; 12] = [
    Fraction::new(1, 1),
    Fraction::new(256, 243),
    Fraction::new(9, 8),
    Fraction::new(32, 27),
    Fraction::new(81, 64),
    Fraction::new(4, 3),
    Fraction::new(729, 512),
    Fraction::new(3, 2),
    Fraction::new(27, 16),
    Fraction::new(16, 9),
    Fraction::new(243, 128),
    Fraction::new(15, 8),
];

pub(crate) const FIVE_LIMIT: [Fraction; 12] = [
    Fraction::new(1, 1),
    Fraction::new(16, 15),
    Fraction::new(9, 8),
    Fraction::new(6, 5),
    Fraction::new(5, 4),
    Fraction::new(4, 3),
    Fraction::new(64, 45),
    Fraction::new(3, 2),
    Fraction::new(8, 5),
    Fraction::new(5, 3),
    Fraction::new(16, 9),
    Fraction::new(15, 8),
];

pub(crate) const ELEVEN_LIMIT: [Fraction; 29] = [
    Fraction::new(1, 1),
    Fraction::new(12, 11),
    Fraction::new(11, 10),
    Fraction::new(10, 9),
    Fraction::new(9, 8),
    Fraction::new(8, 7),
    Fraction::new(7, 6),
    Fraction::new(6, 5),
    Fraction::new(11, 9),
    Fraction::new(5, 4),
    Fraction::new(14, 11),
    Fraction::new(9, 7),
    Fraction::new(4, 3),
    Fraction::new(11, 8),
    Fraction::new(7, 5),
    Fraction::new(10, 7),
    Fraction::new(16, 11),
    Fraction::new(3, 2),
    Fraction::new(14, 9),
    Fraction::new(11, 7),
    Fraction::new(8, 5),
    Fraction::new(18, 11),
    Fraction::new(5, 3),
    Fraction::new(12, 7),
    Fraction::new(7, 4),
    Fraction::new(16, 9),
    Fraction::new(9, 5),
    Fraction::new(20, 11),
    Fraction::new(11, 6),
];

pub(crate) const FORTYTHREE_TONE: [Fraction; 43] = [
    Fraction::new(1, 1),
    Fraction::new(81, 80),
    Fraction::new(33, 32),
    Fraction::new(21, 20),
    Fraction::new(16, 15),
    Fraction::new(12, 11),
    Fraction::new(11, 10),
    Fraction::new(10, 9),
    Fraction::new(9, 8),
    Fraction::new(8, 7),
    Fraction::new(7, 6),
    Fraction::new(32, 27),
    Fraction::new(6, 5),
    Fraction::new(11, 9),
    Fraction::new(5, 4),
    Fraction::new(14, 11),
    Fraction::new(9, 7),
    Fraction::new(21, 16),
    Fraction::new(4, 3),
    Fraction::new(27, 20),
    Fraction::new(11, 8),
    Fraction::new(7, 5),
    Fraction::new(10, 7),
    Fraction::new(16, 11),
    Fraction::new(40, 27),
    Fraction::new(3, 2),
    Fraction::new(23, 21),
    Fraction::new(14, 9),
    Fraction::new(11, 7),
    Fraction::new(8, 5),
    Fraction::new(18, 11),
    Fraction::new(5, 3),
    Fraction::new(27, 16),
    Fraction::new(12, 7),
    Fraction::new(7, 4),
    Fraction::new(16, 8),
    Fraction::new(9, 5),
    Fraction::new(20, 11),
    Fraction::new(11, 6),
    Fraction::new(15, 8),
    Fraction::new(40, 21),
    Fraction::new(64, 33),
    Fraction::new(160, 81),
];

// an array of strings same length as INDIA_SCALE
pub(crate) const SWARAS: [&str; 7] = ["Sa", "Re", "Ga", "Ma", "Pa", "Dha", "Ni"];

// swaras
pub(crate) const INDIAN_SCALE: [Fraction; 7] = [
    Fraction::new(1, 1),
    Fraction::new(9, 8),
    Fraction::new(5, 4),
    Fraction::new(4, 3),
    Fraction::new(3, 2),
    Fraction::new(5, 3),
    Fraction::new(15, 8),
];

pub(crate) const INDIA_SCALE_ALT: [Fraction; 7] = [
    Fraction::new(1, 1),
    Fraction::new(9, 8),
    Fraction::new(5, 4),
    Fraction::new(4, 3),
    Fraction::new(3, 2),
    Fraction::new(27, 16),
    Fraction::new(15, 8),
];

pub(crate) const SHRUTIS: [&str; 22] = [
    "C", "D♭↓", "D♭", "D↓", "D", "E♭↓", "E♭", "E", "E↑", "F", "F↑", "F♯", "F♯↑", "G", "A♭↓", "A♭", "A", "A↑", "B♭↓", "B♭", "B",
    "B↑",
];

pub(crate) const INDIAN_SCALE_22: [Fraction; 22] = [
    Fraction::new(1, 1),
    Fraction::new(256, 243),
    Fraction::new(16, 15),
    Fraction::new(10, 9),
    Fraction::new(9, 8),
    Fraction::new(32, 27),
    Fraction::new(6, 5),
    Fraction::new(5, 4),
    Fraction::new(81, 64),
    Fraction::new(4, 3),
    Fraction::new(27, 20),
    Fraction::new(45, 32),
    Fraction::new(729, 512),
    Fraction::new(3, 2),
    Fraction::new(128, 81),
    Fraction::new(8, 5),
    Fraction::new(5, 3),
    Fraction::new(27, 16),
    Fraction::new(16, 9),
    Fraction::new(9, 5),
    Fraction::new(15, 8),
    Fraction::new(243, 128),
];

pub(crate) const SLENDRO: [&str; 5] = ["siji", "loro", "telu", "lima", "enam"];
pub(crate) const SLENDRO_SHORT: [&str; 5] = ["ji", "ro", "lu", "ma", "nam"];
pub(crate) const SLENDRO_TRAD: [&str; 5] = ["panunggal", "gulu", "dhadha", "lima", "nem"];
