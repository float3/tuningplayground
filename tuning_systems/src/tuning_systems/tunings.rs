use std::str::FromStr;

use crate::{
    equal_temperament, Fraction, ELEVEN_LIMIT, FIVE_LIMIT, FORTYTHREE_TONE, INDIAN_SCALE,
    INDIAN_SCALE_22, JUST_INTONATION, JUST_INTONATION_24, PYTHOGREAN_TUNING,
};

pub enum TuningSystem {
    JustIntonation,
    JustIntonation24,

    StepMethod,
    EqualTemperament,

    PythogoreanTuning,

    FiveLimit,
    ElevenLimit,

    FortyThreeTone,

    //ethnic scales
    Indian,
    IndianFull,
}

impl FromStr for TuningSystem {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JustIntonation" => Ok(TuningSystem::JustIntonation),
            "JustIntonation24" => Ok(TuningSystem::JustIntonation24),
            "StepMethod" => Ok(TuningSystem::StepMethod),
            "EqualTemperament" => Ok(TuningSystem::EqualTemperament),
            "PythogoreanTuning" => Ok(TuningSystem::PythogoreanTuning),
            "FiveLimit" => Ok(TuningSystem::FiveLimit),
            "ElevenLimit" => Ok(TuningSystem::ElevenLimit),
            "FortyThreeTone" => Ok(TuningSystem::FortyThreeTone),
            "Indian" => Ok(TuningSystem::Indian),
            "IndianFull" => Ok(TuningSystem::IndianFull),
            _ => Err(()),
        }
    }
}

pub fn get_ratio(tuning_sytem: TuningSystem, index: usize, size: Option<u32>) -> f64 {
    match tuning_sytem {
        TuningSystem::StepMethod => todo!(),
        TuningSystem::EqualTemperament => equal_temperament(
            index as u32,
            size.expect("you have to provide a size when using this tuning system"),
        )
        .into(),
        _ => get_ratio_from_table(tuning_sytem, index).into(),
    }
}

fn get_ratio_from_table(tuning_sytem: TuningSystem, index: usize) -> Fraction {
    let lut: &[Fraction] = match tuning_sytem {
        TuningSystem::JustIntonation => &JUST_INTONATION,
        TuningSystem::JustIntonation24 => &JUST_INTONATION_24,
        TuningSystem::PythogoreanTuning => &PYTHOGREAN_TUNING,
        TuningSystem::FiveLimit => &FIVE_LIMIT,
        TuningSystem::ElevenLimit => &ELEVEN_LIMIT,
        TuningSystem::FortyThreeTone => &FORTYTHREE_TONE,
        TuningSystem::Indian => &INDIAN_SCALE,
        TuningSystem::IndianFull => &INDIAN_SCALE_22,

        TuningSystem::StepMethod | TuningSystem::EqualTemperament => panic!(),
    };
    let len = lut.len();

    let octaves = (index / len) as u32;
    let mut ratio = lut[index % len];
    ratio.numerator += ratio.denominator * octaves;
    ratio
}
