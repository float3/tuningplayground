use tuning_systems::Fraction;

use crate::pitch::Pitch;

pub struct Interval {
    pitch_start: Pitch,
    pitch_end: Pitch,
    implicit_diatonic: bool,
    pub generic: GenericInterval,
    pub diatonic: DiatonicInterval,
    pub chromatic: ChromaticInterval,
    interval_type: IntervalType,
}

pub struct DiatonicInterval {
    specifier: String,
    generic: GenericInterval,
}
pub struct ChromaticInterval {
    pub semitones: i32,
    pub simple_directed: i32,
}
#[derive(Clone)]
pub struct GenericInterval {
    pub simple_directed: i32,
}

pub enum IntervalType {
    Harmonic,
    Melodic,
}

impl Interval {
    pub(crate) fn new(pitch_start: Pitch, pitch_end: Pitch) -> Option<Interval> {
        let generic = notes_to_generic(&pitch_start, &pitch_end);
        let chromatic = notes_to_chromatic(&pitch_start, &pitch_end);
        let diatonic = intervals_to_diatonic(&generic, &chromatic);

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

fn intervals_to_diatonic(g_int: &GenericInterval, c_int: &ChromaticInterval) -> DiatonicInterval {
    let specifier = get_specifier_from_generic_chromatic(g_int, c_int);
    DiatonicInterval {
        specifier,
        generic: g_int.clone(),
    }
}

fn get_specifier_from_generic_chromatic(
    g_int: &GenericInterval,
    c_int: &ChromaticInterval,
) -> String {
    todo!()
}

fn notes_to_chromatic(p1: &Pitch, p2: &Pitch) -> ChromaticInterval {
    ChromaticInterval {
        semitones: p2.ps - p1.ps,
        simple_directed: p2.diatonic_note_num - p1.diatonic_note_num,
    }
}

fn notes_to_generic(p1: &Pitch, p2: &Pitch) -> GenericInterval {
    let staff_dist = p2.diatonic_note_num - p1.diatonic_note_num;
    let gen_dist = convert_staff_distance_to_interval(staff_dist);
    return GenericInterval(gen_dist);
}

fn convert_staff_distance_to_interval(staff_dist: i32) -> _ {
    todo!()
}
