use tuning_systems::Fraction;

use crate::{pitch::Pitch, PitchOrNote};

pub struct Interval {
    pitch_start: Pitch,
    pitch_end: Pitch,
    implicit_diatonic: bool,
    pub generic: Option<GenericInterval>,
    pub diatonic: Option<DiatonicInterval>,
    pub chromatic: Option<ChromaticInterval>,
    interval_type: IntervalType,
}

pub struct DiatonicInterval {
    generic: GenericInterval,
}
pub struct ChromaticInterval {
    pub semitones: i32,
    pub simple_directed: i32,
}

pub struct GenericInterval {
    pub simple_directed: i32,
}

pub enum IntervalType {
    Harmonic,
    Melodic,
}

impl Interval {
    pub(crate) fn new(pitch_start: PitchOrNote, pitch_end: PitchOrNote) -> Option<Interval> {
        let generic = notes_to_generic(pitch_start.clone(), pitch_end.clone());
        let chromatic = notes_to_chromatic(pitch_start.clone(), pitch_end.clone());
        let diatonic = intervals_to_diatonic(generic, chromatic);

        Some(Interval {
            pitch_start: pitch_start.into(),
            pitch_end: pitch_end.into(),
            implicit_diatonic: false,
            generic,
            diatonic,
            chromatic,
            interval_type: todo!(),
        })
    }

    pub(crate) fn interval_to_pythagorean_ratio(&self) -> Fraction {
        todo!()
    }
}

fn intervals_to_diatonic(
    generic_interval: GenericInterval,
    chromatic_new: ChromaticInterval,
) -> DiatonicInterval {
    todo!()
}

fn notes_to_chromatic(p1: PitchOrNote, p2: PitchOrNote) -> ChromaticInterval {
    todo!()
}

fn notes_to_generic(p1: PitchOrNote, p2: PitchOrNote) -> GenericInterval {
    let staff_dist = p2.diatonic_note_num - p1.diatonic_note_num;
    let gen_dist = convert_staff_distance_to_interval(staff_dist);
    return GenericInterval(gen_dist);
}
