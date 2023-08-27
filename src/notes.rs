use crate::scales::*;
use crate::intervals::*;
use std::cmp;
use regex::{Regex, Captures};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Note {
    number: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NoteError {
    InvalidNote,
    InvalidNoteName,
    InvalidAccidental,
    InvalidOctave,
}

impl Note {
    pub fn new(number: u8) -> Self {
        Self { number: Self::constrain_number(number) }
    }

    pub fn build(name: NoteName, octave: i8) -> Result<Self, NoteError> {
        if octave < -1 || octave > 9 {
            return Err(NoteError::InvalidOctave);
        }

        const ACCIDENTAL_VALUES: [i16; 5] = [-2, -1, 0, 1, 2];
        
        let (note_number, accidental) = match name {
            NoteName::C(accidental) => (0, accidental),
            NoteName::D(accidental) => (2, accidental),
            NoteName::E(accidental) => (4, accidental),
            NoteName::F(accidental) => (5, accidental),
            NoteName::G(accidental) => (7, accidental),
            NoteName::A(accidental) => (9, accidental),
            NoteName::B(accidental) => (11, accidental),
        };
        
        let accidental_value = ACCIDENTAL_VALUES[accidental as usize];
        let number: i16 = note_number + accidental_value + 12 * (octave as i16 + 1);
        
        if number < 0 || number > 127 {
            return Err(NoteError::InvalidNote);
        }
        
        Ok(Self::new(number as u8))
    }

    fn parse_captures(cap: Captures) -> Result<Note, NoteError> {
        let note_name_str = &cap[1];
        let accidental_str = &cap[2];
        let octave_str = &cap[3];

        let note_name = NoteName::from_string(note_name_str)?;
        let accidental = Accidental::from_string(accidental_str)?;
        let octave: i8 = octave_str.parse().unwrap_or(4);

        if octave < -1 || octave > 9 {
            return Err(NoteError::InvalidOctave);
        }

        match note_name {
            NoteName::C(_) => Note::build(NoteName::C(accidental), octave),
            NoteName::D(_) => Note::build(NoteName::D(accidental), octave),
            NoteName::E(_) => Note::build(NoteName::E(accidental), octave),
            NoteName::F(_) => Note::build(NoteName::F(accidental), octave),
            NoteName::G(_) => Note::build(NoteName::G(accidental), octave),
            NoteName::A(_) => Note::build(NoteName::A(accidental), octave),
            NoteName::B(_) => Note::build(NoteName::B(accidental), octave),
        }
    }

    pub fn from_string(s: &str) -> Result<Note, NoteError> {
        let re = Regex::new("^([A-Ga-g])([#x𝄪b♯♯♭♭]*)(-?[0-9]*)$").unwrap();
        match re.captures(s) {
            Some(captures) => Note::parse_captures(captures),
            None => Err(NoteError::InvalidNoteName),
        }
    }
    
    pub fn to_number(&self) -> u8 {
        self.number
    }

    pub fn add(&self, interval: &Interval) -> Self {
        Self::new(self.number + interval.to_semitones())
    }
    
    fn constrain_number(number: u8) -> u8 {
        cmp::max(cmp::min(number, 127), 0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NoteName {
    C(Accidental),
    D(Accidental),
    E(Accidental),
    F(Accidental),
    G(Accidental),
    A(Accidental),
    B(Accidental),
}

impl NoteName {
    fn from_string(s: &str) -> Result<NoteName, NoteError> {
        match s {
            "C" => Ok(NoteName::C(Accidental::Natural)),
            "D" => Ok(NoteName::D(Accidental::Natural)),
            "E" => Ok(NoteName::E(Accidental::Natural)),
            "F" => Ok(NoteName::F(Accidental::Natural)),
            "G" => Ok(NoteName::G(Accidental::Natural)),
            "A" => Ok(NoteName::A(Accidental::Natural)),
            "B" => Ok(NoteName::B(Accidental::Natural)),
            _ => Err(NoteError::InvalidNoteName),
        }   
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    fn from_string(s: &str) -> Result<Accidental, NoteError> {
        match s {
            "" | "♮" => Ok(Accidental::Natural),
            "#" | "♯" => Ok(Accidental::Sharp),
            "##" | "♯♯" | "x" | "𝄪" => Ok(Accidental::DoubleSharp),
            "b" | "♭" => Ok(Accidental::Flat),
            "bb" | "♭♭" => Ok(Accidental::DoubleFlat),
            _ => Err(NoteError::InvalidAccidental),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_note() {
        assert_eq!(
            Note::new(60).to_number(), 
            60
        );
        assert_eq!(
            Note::new(130).to_number(), 
            127
        );
    }

    #[test]
    fn test_build_valid_note() {
        assert_eq!(
            Note::build(NoteName::C(Accidental::Natural), 4).unwrap().to_number(), 
            60
        );
        assert_eq!(
            Note::build(NoteName::E(Accidental::DoubleSharp), 6).unwrap().to_number(), 
            90
        );
    }
    
    #[test]
    fn test_build_invalid_note() {
        assert_eq!(
            Note::build(NoteName::B(Accidental::DoubleSharp), 9).unwrap_err(),
            NoteError::InvalidNote // Too high
        );
        assert_eq!(
            Note::build(NoteName::C(Accidental::Flat), -2).unwrap_err(), 
            NoteError::InvalidOctave // Too low
        );
    }

    #[test]
    fn test_from_string_valid_notes() {
        // Test valid note strings
        assert_eq!(
            Note::from_string("C").unwrap(), 
            Note::build(NoteName::C(Accidental::Natural), 4).unwrap()
        );
        assert_eq!(
            Note::from_string("D5").unwrap(), 
            Note::build(NoteName::D(Accidental::Natural), 5).unwrap()
        );
        assert_eq!(
            Note::from_string("D#").unwrap(), 
            Note::build(NoteName::D(Accidental::Sharp), 4).unwrap()
        );
        assert_eq!(
            Note::from_string("F♭2").unwrap(),
            Note::build(NoteName::F(Accidental::Flat), 2).unwrap()
        );
        assert_eq!(
            Note::from_string("Gbb").unwrap(),
            Note::build(NoteName::G(Accidental::DoubleFlat), 4).unwrap()
        );
        assert_eq!(
            Note::from_string("Ax-1").unwrap(),
            Note::build(NoteName::A(Accidental::DoubleSharp), -1).unwrap()
        );
        assert_eq!(
            Note::from_string("B♯♯7").unwrap(),
            Note::build(NoteName::B(Accidental::DoubleSharp), 7).unwrap()
        );
    }

    #[test]
    fn test_from_string_invalid_notes() {
        // Test invalid note strings
        assert_eq!( 
            Note::from_string("").unwrap_err(), 
            NoteError::InvalidNoteName // Empty string
        ); 
        assert_eq!(
            Note::from_string("Xb").unwrap_err(), 
            NoteError::InvalidNoteName // Invalid note name 
        ); 
        assert_eq!(
            Note::from_string("Cbbb").unwrap_err(), 
            NoteError::InvalidAccidental // Invalid accidental
        ); 
        assert_eq!(
            Note::from_string("Cb#").unwrap_err(), 
            NoteError::InvalidAccidental // Conflicting accidentals
        ); 
        assert_eq!(
            Note::from_string("C-4").unwrap_err(), 
            NoteError::InvalidOctave // Invalid octave (too low)
        ); 
        assert_eq!(
            Note::from_string("B#10").unwrap_err(), 
            NoteError::InvalidOctave // Invalid octave (too high)
        ); 
        assert_eq!(
            Note::from_string("C-#4").unwrap_err(), 
            NoteError::InvalidNoteName // Negative 
        ); 
    }
}