use super::*;

mod ops;
mod fmt;
mod tests;

impl From<u8> for Note {
    fn from(value: u8) -> Self {
        let value = value % 12;
        let name = NoteName::from_chromatic_scale_degree(value);
        let accidental = Accidental::from_chromatic_scale_degree(value);
        Self { name, accidental }
    }
}

impl From<Note> for u8 {
    fn from(note: Note) -> u8 {
        let number = u8::from(note.name) as i8 + i8::from(note.accidental);
        (number % 12) as u8
    }
}

impl From<u8> for PitchedNote {
    fn from(value: u8) -> Self {
        let note = Note::from(value);
        let octave = (value / 12) as i8 - 1;
        Self { note, octave }
    }
}

impl From<PitchedNote> for u8 {
    fn from(pnote: PitchedNote) -> Self {
        u8::from(pnote.note) + ((pnote.octave() + 1) * 12) as u8
    }
}