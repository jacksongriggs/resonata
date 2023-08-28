pub use name::NoteName::{self, *};
pub use accidental::Accidental::{self, *};
pub use crate::{
    nope,
    error::{
        NoteError::{self, *},
        ResonataError}
};

pub mod name;
pub mod accidental;
pub mod utils;
pub mod macros;
mod tests;

/// A musical note
/// Notes are represented by a midi number
/// The midi number is the number of semitones
/// from C-1 (midi number 0) to G9 (midi number 127)
/// 
/// The note name is the name of the note, which
/// can be C, D, E, F, G, A or B.
/// 
/// The accidental is the accidental of the note, which
/// can be flat, natural or sharp.
/// Flats and sharps take a number to represent
/// the number of flats or sharps. For example,
/// `Flat(2)` would be a double flat.
/// The default accidental is natural.
/// 
/// The octave is the octave of the note.
/// The octave can be from -1 to 9.
/// The default octave is 4.
/// 
/// A macro is provided to make creating notes easier:
/// note!(<note name> <accidental> <octave>)
/// note!(<string>)
#[derive(Debug, Clone, Copy, Eq)]
pub struct Note {
    number: u8,
    note_name: NoteName,
    accidental: Accidental,
    octave: i8,
}

impl Note {
    /// Creates a new note from a midi number
    pub fn new(number: u8) -> Result<Note, ResonataError> {
        if number > 127 {
            nope!(InvalidNote);
        }
        
        let note_name = match number % 12 {
            0 => C,
            1 => C,
            2 | 3 => D,
            4 => E,
            5 | 6 => F,
            7 | 8 => G,
            9 | 10 => A,
            11 => B,
            _ => unreachable!("Modulo 12 of a u8 should never be greater than 11")
        };

        let accidental = match number % 12 {
            0 | 2 | 4 | 5 | 7 | 9 | 11 => Natural,
            1 | 3 | 6 | 8 | 10 => Sharp(1),
            _ => unreachable!("Modulo 12 of a u8 should never be greater than 11")
        };

        let octave = (number as i8 / 12) - 1;

        Ok(Note { number, note_name, accidental, octave })
    }

    /// Creates a new note from a note name, accidental and octave
    pub fn build(note_name: NoteName, accidental: Accidental, octave: i8) -> Result<Note, ResonataError> {
        if octave < -1 || octave > 9 {
            nope!(InvalidOctave);
        }
        
        let number = note_name.to_chromatic_number() as i16 + accidental.to_semitones() as i16 + 12 * (octave as i16 + 1);
        
        if number < 0 || number > 127 {
            nope!(InvalidNote);
        }
        
        Ok(Note { 
            number: number as u8, 
            note_name, 
            accidental, 
            octave 
        })
    }

    /// Returns the midi number of the note
    pub fn number(&self) -> u8 {
        self.number
    }

    /// Returns the note name of the note
    pub fn name(&self) -> NoteName {
        self.note_name
    }

    /// Returns the accidental of the note
    pub fn accidental(&self) -> Accidental {
        self.accidental
    }

    /// Returns the octave of the note
    pub fn octave(&self) -> i8 {
        self.octave
    }

    /// Returns the interval from the note to another note
    pub fn interval_to(&self, note: &Note) -> Result<crate::Interval, ResonataError> {
        crate::Interval::new((note.number - self.number) as i8)
    }
}