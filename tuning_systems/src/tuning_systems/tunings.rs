use std::str::FromStr;

use crate::{
    equal_temperament, equal_temperament_default, Fraction, ELEVEN_LIMIT, FIVE_LIMIT, FORTYTHREE_TONE, INDIAN_SCALE,
    INDIAN_SCALE_22, INDIA_SCALE_ALT, JUST_INTONATION, JUST_INTONATION_24, OCTAVE_SIZE, PYTHAGOREAN_TUNING, SHRUTIS, SLENDRO,
    SWARAS, TWELVE_TONE_NAMES,
};
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TuningSystem {
    StepMethod,
    EqualTemperament,

    Javanese,
    Siamese,
    WholeTone,
    QuarterTone,

    JustIntonation,
    JustIntonation24,
    PythagoreanTuning,

    FiveLimit,
    ElevenLimit,

    FortyThreeTone,

    //ethnic scales
    Indian,
    IndianAlt,
    Indian22,
}

impl FromStr for TuningSystem {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stepmethod" => Ok(TuningSystem::StepMethod),
            "equaltemperament" => Ok(TuningSystem::EqualTemperament),
            "justintonation" => Ok(TuningSystem::JustIntonation),
            "justintonation24" => Ok(TuningSystem::JustIntonation24),
            "pythagoreantuning" => Ok(TuningSystem::PythagoreanTuning),
            "fivelimit" => Ok(TuningSystem::FiveLimit),
            "elevenlimit" => Ok(TuningSystem::ElevenLimit),
            "fortythreetone" => Ok(TuningSystem::FortyThreeTone),
            "indian" => Ok(TuningSystem::Indian),
            "indianalt" => Ok(TuningSystem::IndianAlt),
            "indianfull" => Ok(TuningSystem::Indian22),
            "siamese" => Ok(TuningSystem::Siamese),
            "javanese" => Ok(TuningSystem::Javanese),
            "wholetone" => Ok(TuningSystem::WholeTone),
            "quartertone" => Ok(TuningSystem::QuarterTone),
            _ => Err(()),
        }
    }
}

impl TuningSystem {
    pub fn get_fraction(&self, index: usize) -> Fraction {
        match &self {
            TuningSystem::StepMethod => todo!("StepMethod"),
            TuningSystem::EqualTemperament => equal_temperament_default(index),
            TuningSystem::Javanese => equal_temperament(index, 5),
            TuningSystem::Siamese => equal_temperament(index, 9),
            TuningSystem::WholeTone => equal_temperament(index, 6),
            TuningSystem::QuarterTone => equal_temperament(index, 24),
            TuningSystem::JustIntonation
            | TuningSystem::JustIntonation24
            | TuningSystem::PythagoreanTuning
            | TuningSystem::FiveLimit
            | TuningSystem::ElevenLimit
            | TuningSystem::FortyThreeTone
            | TuningSystem::Indian
            | TuningSystem::IndianAlt
            | TuningSystem::Indian22 => self.get_fraction_from_table(index),
        }
    }

    pub fn size(&self) -> usize {
        match &self {
            TuningSystem::JustIntonation
            | TuningSystem::JustIntonation24
            | TuningSystem::PythagoreanTuning
            | TuningSystem::FiveLimit
            | TuningSystem::ElevenLimit
            | TuningSystem::FortyThreeTone
            | TuningSystem::Indian
            | TuningSystem::IndianAlt
            | TuningSystem::Indian22 => self.get_lut_from_tuningsystem().len(),
            TuningSystem::StepMethod | TuningSystem::EqualTemperament => *OCTAVE_SIZE.read().expect("couldn't read octave size"),
            TuningSystem::Siamese => 9,
            TuningSystem::Javanese => 5,
            TuningSystem::WholeTone => 6,
            TuningSystem::QuarterTone => 24,
        }
    }

    fn get_fraction_from_table(&self, index: usize) -> Fraction {
        let lut = self.get_lut_from_tuningsystem();
        let len = lut.len();
        let octave = index / len;
        let index_mod = index % len;
        let mut fraction = lut[index_mod];
        // fraction.numerator += (2u32.pow(octave as u32) - 1) * fraction.denominator;
        fraction.numerator *= 2u32.pow(octave as u32);
        fraction
    }

    fn get_lut_from_tuningsystem(&self) -> &[Fraction] {
        let lut: &[Fraction] = match self {
            TuningSystem::JustIntonation => &JUST_INTONATION,
            TuningSystem::JustIntonation24 => &JUST_INTONATION_24,
            TuningSystem::PythagoreanTuning => &PYTHAGOREAN_TUNING,
            TuningSystem::FiveLimit => &FIVE_LIMIT,
            TuningSystem::ElevenLimit => &ELEVEN_LIMIT,
            TuningSystem::FortyThreeTone => &FORTYTHREE_TONE,
            TuningSystem::Indian => &INDIAN_SCALE,
            TuningSystem::IndianAlt => &INDIA_SCALE_ALT,
            TuningSystem::Indian22 => &INDIAN_SCALE_22,

            TuningSystem::StepMethod
            | TuningSystem::EqualTemperament
            | TuningSystem::Siamese
            | TuningSystem::Javanese
            | TuningSystem::QuarterTone
            | TuningSystem::WholeTone => {
                unreachable!("these tuning methods don't have LUTs. Use get_fraction instead.")
            }
        };
        lut
    }

    pub fn get_tone_name(&self, tone_index: usize) -> String {
        let octave_size = *OCTAVE_SIZE.read().expect("couldn't read octave size");
        // if indian or indianalt we want to use 7
        let name_index = tone_index % octave_size;
        let name = match self {
            TuningSystem::EqualTemperament if octave_size == 24 => TWELVE_TONE_NAMES[name_index / 2],
            TuningSystem::EqualTemperament if octave_size == 12 => TWELVE_TONE_NAMES[name_index],
            TuningSystem::EqualTemperament if octave_size == 6 => TWELVE_TONE_NAMES[name_index * 2],
            TuningSystem::WholeTone => TWELVE_TONE_NAMES[(tone_index % 6) * 2],
            TuningSystem::EqualTemperament if octave_size == 4 => TWELVE_TONE_NAMES[name_index * 3],
            TuningSystem::EqualTemperament if octave_size == 3 => TWELVE_TONE_NAMES[name_index * 4],
            TuningSystem::EqualTemperament if octave_size == 2 => TWELVE_TONE_NAMES[name_index * 6],
            TuningSystem::EqualTemperament if octave_size == 1 => TWELVE_TONE_NAMES[name_index * 12],
            TuningSystem::EqualTemperament => TWELVE_TONE_NAMES[name_index * (octave_size / 12)],

            TuningSystem::JustIntonation
            | TuningSystem::PythagoreanTuning
            | TuningSystem::FiveLimit
            | TuningSystem::StepMethod => TWELVE_TONE_NAMES[name_index],

            TuningSystem::Indian | TuningSystem::IndianAlt => SWARAS[tone_index % SWARAS.len()],
            TuningSystem::Indian22 => SHRUTIS[tone_index % SHRUTIS.len()],

            TuningSystem::Javanese => SLENDRO[tone_index % SLENDRO.len()],
            TuningSystem::Siamese => "todo",
            TuningSystem::QuarterTone => "todo",
            TuningSystem::JustIntonation24 => "todo",
            TuningSystem::ElevenLimit => "todo",
            TuningSystem::FortyThreeTone => "todo",
        };

        let octave = tone_index / octave_size;
        let adjusted_octave: i32 = octave as i32 - 1;
        if adjusted_octave < 0 {
            format!("{}N{}", name, -adjusted_octave)
        } else {
            format!("{}{}", name, adjusted_octave)
        }
    }
}
