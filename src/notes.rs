pub use crate::{
    error::{
        NoteError::{self, *},
        ResonataError,
    },
    nope,
};
pub use accidental::Accidental::{self, *};
pub use name::NoteName::{self, *};

pub mod accidental;
pub mod macros;
pub mod name;
mod utils;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note {
    name: NoteName,
    accidental: Accidental,
}

impl Note {
    pub fn new(name: NoteName, accidental: Accidental) -> Self {
        Note { name, accidental }
    }

    pub fn from_note_name(name: NoteName) -> Self {
        Note { name, accidental: Natural, }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PitchedNote {
    note: Note,
    octave: i8,
}

impl PitchedNote {
    pub fn new(name: NoteName, accidental: Accidental, octave: i8) -> Self {
        Self::build(Note { name, accidental }, octave)
    }

    /// Creates a new note from a note name, accidental and octave.
    /// Octave numbers outside of the range -1 to 9 will be clamped to the nearest valid number.
    pub fn build(note: Note, octave: i8) -> Self {
        let octave = std::cmp::min(std::cmp::max(octave, -1), 9);

        PitchedNote { note, octave }
    }

    pub fn from_note_name_and_octave(name: NoteName, octave: i8) -> Self {
        Self::build(
            Note {
                name,
                accidental: Natural,
            },
            octave,
        )
    }

    /// Returns the note name of the note
    pub fn name(&self) -> NoteName {
        self.note.name
    }

    /// Returns the accidental of the note
    pub fn accidental(&self) -> Accidental {
        self.note.accidental
    }

    /// Returns the octave of the note
    pub fn octave(&self) -> i8 {
        self.octave
    }
}
