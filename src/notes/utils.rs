use super::*;

mod fmt;
mod ops;
mod tests;

impl From<i32> for Note {
    fn from(value: i32) -> Self {
        let value = value % 12;
        let name = NoteName::from_chromatic_scale_degree(value as u8);
        let accidental = Accidental::from_chromatic_scale_degree(value);
        Note { name, accidental }
    }
}

impl From<Note> for i32 {
    fn from(note: Note) -> i32 {
        let number = u8::from(note.name) as i32 + i32::from(note.accidental);
        number % 12
    }
}

impl From<u8> for PitchedNote {
    fn from(value: u8) -> Self {
        let note = Note::from(value as i32);
        let octave = (value / 12) as i8 - 1;
        Self { note, octave }
    }
}

impl From<PitchedNote> for u8 {
    fn from(pnote: PitchedNote) -> Self {
        (i32::from(pnote.note) + ((pnote.octave() + 1) * 12) as i32) as u8
    }
}
