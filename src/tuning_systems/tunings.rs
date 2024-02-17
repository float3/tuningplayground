use super::{algorithms, fraction, luts};
use std::str::FromStr;

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
/*
pub fn get_ratio(tuning_sytem: TuningSystem, index: usize, size: Option<u32>) -> f64 {
    match tuning_sytem {
        TuningSystem::StepMethod => todo!(),
        TuningSystem::EqualTemperament => algorithms::equal_temperament(
            index as u32,
            size.expect("you have to provide a size when using this tuning system"),
        ),
        _ => get_ratio_from_table(tuning_sytem, index).into(),
    }
}

fn get_ratio_from_table(tuning_sytem: TuningSystem, index: usize) -> Fraction {
    let lut: &[Fraction] = match tuning_sytem {
        TuningSystem::JustIntonation => &luts::JUST_INTONATION,
        TuningSystem::JustIntonation24 => &luts::JUST_INTONATION_24,
        TuningSystem::PythogoreanTuning => &luts::PYTHOGREAN_TUNING,
        TuningSystem::FiveLimit => &luts::FIVE_LIMIT,
        TuningSystem::ElevenLimit => &luts::ELEVEN_LIMIT,
        TuningSystem::FortyThreeTone => &luts::FORTYTHREE_TONE,
        TuningSystem::Indian => &luts::INDIAN_SCALE,
        TuningSystem::IndianFull => &luts::INDIAN_SCALE_FULL,

        TuningSystem::StepMethod | TuningSystem::EqualTemperament => panic!(),
    };
    let len = lut.len();

    let octaves = (index / len) as u32;
    let mut ratio = lut[index % len];
    ratio.0 += ratio.1 * octaves;
    ratio
}
*/
