use std::str::FromStr;

use crate::{
    equal_temperament_default, Fraction, ELEVEN_LIMIT, FIVE_LIMIT, FORTYTHREE_TONE, INDIAN_SCALE, INDIAN_SCALE_22, INDIAN_SCALE_NAMES,
    INDIA_SCALE_ALT, JUST_INTONATION, JUST_INTONATION_24, OCTAVE_SIZE, PYTHOGREAN_TUNING, TWELVE_TONE_NAMES,
};
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TuningSystem {
    StepMethod,
    EqualTemperament,

    JustIntonation,
    JustIntonation24,
    PythogoreanTuning,

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
        match s {
            "StepMethod" => Ok(TuningSystem::StepMethod),
            "EqualTemperament" => Ok(TuningSystem::EqualTemperament),
            "JustIntonation" => Ok(TuningSystem::JustIntonation),
            "JustIntonation24" => Ok(TuningSystem::JustIntonation24),
            "PythogoreanTuning" => Ok(TuningSystem::PythogoreanTuning),
            "FiveLimit" => Ok(TuningSystem::FiveLimit),
            "ElevenLimit" => Ok(TuningSystem::ElevenLimit),
            "FortyThreeTone" => Ok(TuningSystem::FortyThreeTone),
            "Indian" => Ok(TuningSystem::Indian),
            "IndianAlt" => Ok(TuningSystem::IndianAlt),
            "IndianFull" => Ok(TuningSystem::Indian22),
            _ => Err(()),
        }
    }
}

impl TuningSystem {
    pub fn get_fraction(&self, index: usize) -> Fraction {
        match &self {
            TuningSystem::StepMethod | TuningSystem::EqualTemperament => equal_temperament_default(index),
            TuningSystem::JustIntonation
            | TuningSystem::JustIntonation24
            | TuningSystem::PythogoreanTuning
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
            | TuningSystem::PythogoreanTuning
            | TuningSystem::FiveLimit
            | TuningSystem::ElevenLimit
            | TuningSystem::FortyThreeTone
            | TuningSystem::Indian
            | TuningSystem::IndianAlt
            | TuningSystem::Indian22 => self.get_lut_from_tuningsystem().len(),
            TuningSystem::StepMethod | TuningSystem::EqualTemperament => *OCTAVE_SIZE.read().unwrap(),
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
            TuningSystem::PythogoreanTuning => &PYTHOGREAN_TUNING,
            TuningSystem::FiveLimit => &FIVE_LIMIT,
            TuningSystem::ElevenLimit => &ELEVEN_LIMIT,
            TuningSystem::FortyThreeTone => &FORTYTHREE_TONE,
            TuningSystem::Indian => &INDIAN_SCALE,
            TuningSystem::IndianAlt => &INDIA_SCALE_ALT,
            TuningSystem::Indian22 => &INDIAN_SCALE_22,

            TuningSystem::StepMethod | TuningSystem::EqualTemperament => panic!(),
        };
        lut
    }

    pub fn get_tone_name(&self, tone_index: usize) -> String {
        let octave_size = *OCTAVE_SIZE.read().unwrap();
        let name_index = tone_index % octave_size;
        let name = match self {
            TuningSystem::EqualTemperament if octave_size == 12 => TWELVE_TONE_NAMES[name_index],
            TuningSystem::EqualTemperament if octave_size == 6 => TWELVE_TONE_NAMES[name_index * 2],
            TuningSystem::EqualTemperament if octave_size == 4 => TWELVE_TONE_NAMES[name_index * 3],
            TuningSystem::EqualTemperament if octave_size == 3 => TWELVE_TONE_NAMES[name_index * 4],
            TuningSystem::EqualTemperament if octave_size == 2 => TWELVE_TONE_NAMES[name_index * 6],
            TuningSystem::EqualTemperament if octave_size == 1 => TWELVE_TONE_NAMES[name_index * 12],
            TuningSystem::JustIntonation | TuningSystem::PythogoreanTuning | TuningSystem::FiveLimit => TWELVE_TONE_NAMES[name_index],
            TuningSystem::Indian | TuningSystem::IndianAlt => INDIAN_SCALE_NAMES[name_index],
            _ => "TODO",
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
