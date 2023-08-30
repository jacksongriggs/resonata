use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr, ops::{Add, Sub}};
use regex::Regex;
use crate::err;
use lazy_static::lazy_static;

use super::*;

impl From<u8> for Note {
    fn from(value: u8) -> Self {
        let number = value % 12;
        let name = NoteName::from_chromatic_scale_degree(number);
        let accidental = Accidental::from_chromatic_scale_degree(number);
        Self { name, accidental }
    }
}

impl From<Note> for u8 {
    fn from(note: Note) -> u8 {
        let number = u8::from(note.name) as i8 + i8::from(note.accidental);
        number as u8
    }
}

impl Add<u8> for Note {
    type Output = Self;
    fn add(self, semitones: u8) -> Self::Output {
        let number = std::cmp::min(u8::from(self) as u16 + semitones as u16, 127) as u8;
        Self::from(number)
    }
}

impl Sub<u8> for Note {
    type Output = Self;
    fn sub(self, semitones: u8) -> Self::Output {
        let number = std::cmp::max(u8::from(self) as i8 - semitones as i8, 0) as u8;
        Self::from(number)
    }
}

impl Sub for Note {
    type Output = crate::Interval;
    fn sub(self, other: Self) -> Self::Output {
        crate::Interval::from(u8::from(self) as i8 - u8::from(other) as i8)
    }
}

impl Add<crate::Interval> for Note {
    type Output = Option<Self>;
    fn add(self, interval: crate::Interval) -> Self::Output {
        let number = u8::from(self) + u8::from(interval);
        if number > 127 {
            None
        } else {
            Some(Self::from(number))
        }
    }
}

impl Sub<crate::Interval> for Note {
    type Output = Option<Self>;
    fn sub(self, interval: crate::Interval) -> Self::Output {
        let number = u8::from(self) as i8 - i8::from(interval);
        if number < 0 {
            None
        } else {
            Some(Self::from(number as u8))
        }
    }
}

lazy_static! {
    static ref NOTE_RE: Regex = Regex::new("^([A-Ga-g])([#x𝄪b♯♯♭♭♮]*)$").unwrap();
}

impl FromStr for Note {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match NOTE_RE.captures(s) {
            Some(cap) => {
                let name = NoteName::from_str(&cap[1])?;
                let acc_str = &cap[2];
                let accidental = if acc_str.is_empty() {
                    Accidental::Natural
                } else {
                    Accidental::from_str(&acc_str)?
                };

                Ok(Self { name, accidental })
            }
            None => {
                eprintln!("Note: {}: {}", InvalidNoteName, s);
                err!(InvalidNoteName)
            }
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let note_name = self.name.to_string();
        let accidental = self.accidental.to_string();
        write!(f, "{}{}", note_name, accidental)
    }
}

impl Debug for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
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
    fn from(note: PitchedNote) -> Self {
        u8::from(note.note) + ((note.octave() + 1) * 12) as u8
    }
}

impl Add<u8> for PitchedNote {
    type Output = Self;
    fn add(self, semitones: u8) -> Self::Output {
        let number = std::cmp::min(u8::from(self) as u16 + semitones as u16, 127) as u8;
        Self::from(number)
    }
}

impl Sub<u8> for PitchedNote {
    type Output = Self;
    fn sub(self, semitones: u8) -> Self::Output {
        let number = std::cmp::max(u8::from(self) as i8 - semitones as i8, 0) as u8;
        Self::from(number)
    }
}

impl Sub for PitchedNote {
    type Output = crate::Interval;
    fn sub(self, other: Self) -> Self::Output {
        crate::Interval::from(u8::from(self) as i8 - u8::from(other) as i8)
    }
}

impl Add<crate::Interval> for PitchedNote {
    type Output = Option<Self>;
    fn add(self, interval: crate::Interval) -> Self::Output {
        let number = u8::from(self) + u8::from(interval);
        if number > 127 {
            None
        } else {
            Some(Self::from(number))
        }
    }
}

impl Sub<crate::Interval> for PitchedNote {
    type Output = Option<Self>;
    fn sub(self, interval: crate::Interval) -> Self::Output {
        let number = u8::from(self) as i8 - i8::from(interval);
        if number < 0 {
            None
        } else {
            Some(Self::from(number as u8))
        }
    }
}

lazy_static! {
    static ref PITCHED_NOTE_RE: Regex = Regex::new("^([A-Ga-g][#x𝄪b♯♯♭♭♮]*)(-?[0-9]*)$").unwrap();
}

impl FromStr for PitchedNote {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match PITCHED_NOTE_RE.captures(s) {
            Some(cap) => {
                let note = Note::from_str(&cap[1])?;
                let octave: i8 = cap[2].parse().unwrap_or(4);
        
                if octave < -1 || octave > 9 {
                    eprintln!("PitchedNote: {}: {}", InvalidOctave, s);
                    nope!(InvalidOctave);
                }
        
                Ok(Self { note, octave })
            }
            None => {
                eprintln!("PitchedNote: {}: {}", InvalidNoteName, s);
                nope!(InvalidNoteName)
            }
        }
    }
}

impl Display for PitchedNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let note_name = self.name().to_string();
        let accidental = self.accidental().to_string();
        let octave = self.octave.to_string();
        write!(f, "{}{}{}", note_name, accidental, octave)
    }
}

impl Debug for PitchedNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}