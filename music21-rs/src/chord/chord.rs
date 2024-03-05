use crate::{note::Note, pitch::Pitch};

pub struct Chord {
    notes: Vec<Note>,
    common_name: String,
    pub pitched_common_name: String,
}

impl Chord {
    pub fn new(notes: &str) -> Result<Chord, &'static str> {
        // if let Some(notes) = &notes {
        //     if notes.iter().any(|n| {
        //         matches!(n.note_type, NoteType::General(_))
        //             && !matches!(n.note_type, NoteType::Specific(_))
        //             && !matches!(n.note_type, NoteType::Chord(_))
        //     }) {
        //         return Err("Use a PercussionChord to contain Unpitched objects");
        //     }
        // }

        // let mut chord = Chord {
        //     _notes: notes.clone().unwrap_or_else(Vec::new),
        //     // initialize other fields here...
        // };

        // if let Some(notes) = &notes {
        //     if notes
        //         .iter()
        //         .all(|n| matches!(n.note_type, NoteType::Specific(_)))
        //     {
        //         chord.simplify_enharmonics_in_place();
        //     }
        // }

        // Ok(chord)
        let mut chord = Chord {
            notes: notes
                .split(" ")
                .map(|note| Note::new(note.to_string()))
                .collect(),
            common_name: String::new(),
            pitched_common_name: String::new(),
        };

        chord.common_name = chord.pitched_common_name();
        chord.pitched_common_name = chord.pitched_common_name();

        Ok(chord)
    }

    pub fn pitches(&self) -> Vec<Pitch> {
        self.notes.iter().map(|note| note.pitch.clone()).collect()
    }

    fn common_name(&self) -> String {
        todo!()
    }

    fn pitched_common_name(&self) -> String {
        todo!()
    }
    fn simplify_enharmonics_in_place(&self) -> Chord {
        todo!()
    }
}
