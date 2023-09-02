use crate::notes::*;
pub use std::collections::HashMap;

mod utils;

#[derive(Clone, PartialEq, Eq)]
struct Key {
    pitches: HashMap<NoteName, Accidental>,
}

impl Key {
    pub fn new(notes: Vec<Note>) -> Key {
        // Initialize the key with all natural notes, then set the user-specified notes.
        // This ensures that the key will have all notes, even if the user doesn't specify them all.
        use NoteName::*;
        let mut pitches = HashMap::new();
        for note_name in vec![C, D, E, F, G, A, B] {
            pitches.insert(note_name, Accidental::Natural);
        }

        // Set the user-specified notes.
        for note in notes {
            pitches.insert(note.name(), note.accidental());
        }

        Key { pitches }
    }

    pub fn set_pitch(&mut self, note: Note) {
        self.pitches.insert(note.name(), note.accidental());
    }

    pub fn pitch(&self, note_name: NoteName) -> Note {
        let accidental = self.pitches.get(&note_name).unwrap();
        Note::new(note_name, *accidental)
    }

    pub fn pitches(&self) -> Vec<Note> {
        use NoteName::*;
        vec![
            self.pitch(C),
            self.pitch(D),
            self.pitch(E),
            self.pitch(F),
            self.pitch(G),
            self.pitch(A),
            self.pitch(B),
        ]
    }
}
