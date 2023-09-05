use crate::Interval;

use super::*;
use crate::{err, error::*};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

impl From<PitchedNote> for u8 {
    /// Convert a pitched note to a MIDI note number.
    fn from(pnote: PitchedNote) -> Self {
        pnote.to_midi_number()
    }
}

lazy_static! {
    static ref NOTE_RE: Regex = Regex::new("^([A-Ga-g])([#x𝄪b♯♯♭♭♮]*)$").unwrap();
}

use std::ops::{Add, AddAssign, Sub, SubAssign};

impl Add<Interval> for Note {
    type Output = Self;
    fn add(self, rhs: Interval) -> Self::Output {
        let mut new = self.clone();
        
        let semitones = rhs.to_semitones() % 12;
        new.name += rhs.size();
        
        let new_semitones = self.interval_to(&new).to_semitones() % 12;
        if semitones != new_semitones {
            new.accidental += semitones - new_semitones;
        }
        new
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

impl Sub for Note {
    type Output = Interval;
    fn sub(self, rhs: Self) -> Self::Output {
        rhs.interval_to(&self)
    }
}

impl Sub<Interval> for Note {
    type Output = Self;
    fn sub(self, rhs: Interval) -> Self::Output {
        let mut new = self.clone();
        
        let semitones = rhs.to_semitones() % 12;
        new.name -= rhs.size();
        
        let new_semitones = new.interval_to(&self).to_semitones() % 12;
        if semitones != new_semitones {
            new.accidental -= semitones - new_semitones;
        }
        new
    }
}

impl SubAssign<Interval> for Note {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - rhs;
    }
}

impl Add<Interval> for PitchedNote {
    type Output = Result<Self>;
    fn add(self, rhs: Interval) -> Self::Output {
        let mut new = self.moved_by(rhs.to_diatonic_steps())?;
        let semitones = rhs.to_semitones();
        let diff = semitones - self.interval_to(&new).to_semitones();
        new.note.accidental += diff;
        Ok(new.clone())
    }
}

impl AddAssign<Interval> for PitchedNote {
    fn add_assign(&mut self, rhs: Interval) {
        *self = match *self + rhs {
            Ok(pnote) => pnote,
            Err(_) => *self,
        }
    }
}

impl Sub for PitchedNote {
    type Output = Interval;
    fn sub(self, rhs: Self) -> Self::Output {
        rhs.interval_to(&self)
    }
}

impl Sub<Interval> for PitchedNote {
    type Output = Result<Self>;
    fn sub(self, rhs: Interval) -> Self::Output {
        let mut new = self.moved_by(-rhs.to_diatonic_steps())?;
        let semitones = rhs.to_semitones();
        let diff = semitones - new.interval_to(&self).to_semitones();
        new.note.accidental -= diff;
        Ok(new.clone())
    }
}

impl SubAssign<Interval> for PitchedNote {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = match *self - rhs {
            Ok(pnote) => pnote,
            Err(_) => *self,
        }
    }
}

impl FromStr for Note {
    type Err = ResonataError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
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
            None => err!(InvalidNoteName(s.to_string())),
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

lazy_static! {
    static ref PITCHED_NOTE_RE: Regex = Regex::new("^([A-Ga-g][#x𝄪b♯♭♮]*)(-?[0-9]*)$").unwrap();
}

impl FromStr for PitchedNote {
    type Err = ResonataError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match PITCHED_NOTE_RE.captures(s) {
            Some(cap) => {
                let note = Note::from_str(&cap[1])?;
                let octave: i8 = cap[2].parse().unwrap_or(4);

                if octave < -1 || octave > 9 {
                    nope!(InvalidOctave(octave));
                }

                Ok(Self { note, octave })
            }
            None => nope!(InvalidNoteName(s.to_string())),
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
