use crate::pitch::Pitch;

#[derive(Clone)]
pub struct Note {
    pub pitch: Pitch,
}

impl Note {
    pub(crate) fn new(string: String) -> Note {
        let pitch = Pitch::new(string);
        Note { pitch }
    }
}

// #[derive(Clone)]
// pub enum Note {
//     Specific(Pitch),
//     General(Pitch),
//     Chord(Vec<Note>),
// }

// impl Note {
//     pub(crate) fn new(to_string: String) -> Note {
//         todo!()
//     }

//     pub fn pitches(&self) -> Vec<Pitch> {
//         match self {
//             Note::Specific(pitch) => vec![pitch.clone()],
//             Note::General(pitch) => vec![pitch.clone()],
//             Note::Chord(notes) => notes.iter().map(|note| note.pitches()).flatten().collect(),
//         }
//     }
// }
