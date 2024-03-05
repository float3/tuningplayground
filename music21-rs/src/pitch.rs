#[derive(Clone)]
pub struct Pitch {
    pub name: String,
    // pub octave: i32,
    // pub accidental: String,
    // pub frequency: f64,
}

impl Pitch {
    pub(crate) fn new(string: String) -> Pitch {
        Pitch { name: string }
    }
}
