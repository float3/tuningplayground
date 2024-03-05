use tuning_systems::Fraction;

use crate::pitch::Pitch;

pub struct Interval {
    note_start: Pitch,
    note_end: Pitch,
}
impl Interval {
    pub(crate) fn new(p1: Pitch, p2: Pitch) -> Interval {
        Interval {
            note_start: p1,
            note_end: p2,
        }
    }

    pub(crate) fn interval_to_pythagorean_ratio(&self) -> Fraction {
        todo!()
    }
}
