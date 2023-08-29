use std::{fmt::{self, Display, Formatter}, str::FromStr, cmp, ops::{Add, Sub}};
use regex::Regex;
use super::*;

impl FromStr for Note {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("^([A-Ga-g])([#x𝄪b♯♯♭♭♮]*)(-?[0-9]*)$").unwrap();
        match re.captures(s) {
            Some(cap) => {
                let note_name_str = &cap[1];
                let accidental_str = &cap[2];
                let octave_str = &cap[3];
        
                let note_name = NoteName::from_str(note_name_str)?;
                let accidental = Accidental::from_str(accidental_str)?;
                let octave: i8 = octave_str.parse().unwrap_or(4);
        
                if octave < -1 || octave > 9 {
                    nope!(InvalidOctave);
                }
        
                Ok(Self::build(note_name, accidental, octave))
            }
            None => nope!(InvalidNoteName),
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let note_name = self.note_name.to_string();
        let accidental = self.accidental.to_string();
        let octave = self.octave.to_string();
        write!(f, "{}{}{}", note_name, accidental, octave)
    }
}

impl Add<u8> for Note {
    type Output = Note;
    fn add(self, semitones: u8) -> Self::Output {
        let number = self.number as u16 + semitones as u16;
        Note::new(number as u8)
    }
}

impl Sub<u8> for Note {
    type Output = Note;
    fn sub(self, semitones: u8) -> Self::Output {
        let number = self.number as i16 - semitones as i16;
        Note::new(number as u8)
    }
}

impl Sub<Note> for Note {
    type Output = crate::Interval;
    fn sub(self, other: Note) -> Self::Output {
        let semitones = self.number as i16 - other.number as i16;
        crate::Interval::new(semitones.abs() as u8)
    }
}

impl Add<crate::Interval> for Note {
    type Output = Option<Note>;
    fn add(self, interval: crate::Interval) -> Self::Output {
        let number = self.number as i16 + interval.semitones() as i16;
        if number > 127 {
            None
        } else {
            Some(Note::new(number as u8))
        }
    }
}

impl Sub<crate::Interval> for Note {
    type Output = Option<Note>;
    fn sub(self, interval: crate::Interval) -> Self::Output {
        let number = self.number as i16 - interval.semitones() as i16;
        if number < 0 {
            None
        } else {
            Some(Note::new(number as u8))
        }
    }
}

impl cmp::PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl cmp::PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.number.partial_cmp(&other.number)
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.number.cmp(&other.number)
    }
}