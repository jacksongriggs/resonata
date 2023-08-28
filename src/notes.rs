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

#[derive(Debug, Clone, Copy)]
pub struct Note {
    number: u8,
    note_name: NoteName,
    accidental: Accidental,
    octave: i8,
}

impl Note {
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

    pub fn number(&self) -> u8 {
        self.number
    }

    pub fn name(&self) -> NoteName {
        self.note_name
    }

    pub fn accidental(&self) -> Accidental {
        self.accidental
    }

    pub fn octave(&self) -> i8 {
        self.octave
    }

    pub fn interval_to(&self, note: &Note) -> Result<crate::Interval, ResonataError> {
        crate::Interval::new((note.number - self.number) as i8)
    }
}