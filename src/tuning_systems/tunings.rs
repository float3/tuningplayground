use super::{algorithms, fraction, luts};

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

fn get_ratio_from_table(tuning_sytem: TuningSystem, index: usize) -> fraction::Fraction {
    let lut: &[fraction::Fraction] = match tuning_sytem {
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

    let octaves = (index / len) as i32;
    let mut ratio = lut[index % len];
    ratio.0 += ratio.1 * octaves;
    ratio
}
