use crate::{Interval, TransposeDown, TransposeUp};

use super::*;
use crate::{err, error::*};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

impl TransposeUp for Note {
    type Output = Self;

    /// Return a new note transposed up by the given interval.
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, intervals::*, TransposeUp};
    ///
    /// let note = Note::from_string("C").unwrap();
    /// let new_note = note.transposed_up(Interval::from_string("M3").unwrap());
    /// assert_eq!(new_note, Note::from_string("E").unwrap());
    ///
    /// let note = Note::from_string("D#").unwrap();
    /// let new_note = note.transposed_up(Interval::from_string("A4").unwrap());
    /// assert_eq!(new_note, Note::from_string("G##").unwrap());
    /// ```
    fn transposed_up(&self, interval: Interval) -> Self {
        let mut new = self.clone();
        let semitones = interval.to_semitones();
        new.name += interval.size();
        new.accidental += semitones - self.interval_to(&new).to_semitones();
        new
    }
}

impl TransposeDown for Note {
    type Output = Self;

    /// Return a new note transposed down by the given interval.
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, intervals::*, TransposeDown};
    ///
    /// let note = Note::from_string("C").unwrap();
    /// let new_note = note.transposed_down(Interval::from_string("M3").unwrap());
    /// assert_eq!(new_note, Note::from_string("Ab").unwrap());
    ///
    /// let note = Note::from_string("D#").unwrap();
    /// let new_note = note.transposed_down(Interval::from_string("A4").unwrap());
    /// assert_eq!(new_note, Note::from_string("A").unwrap());
    /// ```
    fn transposed_down(&self, interval: Interval) -> Self {
        let mut new = self.clone();
        let semitones = interval.to_semitones();
        new.name -= interval.size();
        new.accidental -= semitones - new.interval_to(&self).to_semitones();
        new
    }
}

impl From<PitchedNote> for u8 {
    /// Convert a pitched note to a MIDI note number.
    fn from(pnote: PitchedNote) -> Self {
        pnote.to_midi_number()
    }
}

impl TransposeUp for PitchedNote {
    type Output = Option<Self>;

    /// Transpose the note up by the given interval.
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, intervals::*, TransposeUp};
    ///
    /// let note = pnote!("C4").unwrap().transposed_up(inv!("M3").unwrap()).unwrap();
    /// assert_eq!(note, pnote!("E4").unwrap());
    ///
    /// let note = pnote!("A4").unwrap()
    ///     .transposed_up(inv!("d3").unwrap()).unwrap();
    /// assert_eq!(note, pnote!("Cb5").unwrap());
    /// ```
    fn transposed_up(&self, interval: Interval) -> Self::Output {
        let mut new = self.moved_by(interval.to_diatonic_steps());
        match new {
            Some(ref mut new) => {
                let semitones = interval.to_semitones();
                let diff = semitones - self.interval_to(&new).to_semitones();
                new.note.accidental += diff;
                Some(new.clone())
            }
            None => None,
        }
    }
}

impl TransposeDown for PitchedNote {
    type Output = Option<Self>;

    /// Transpose the note down by the given interval.
    /// 
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, intervals::*, TransposeDown};
    /// 
    /// let note = pnote!("C4").unwrap()
    ///     .transposed_down(inv!("M3").unwrap())
    ///     .unwrap();
    /// assert_eq!(note, pnote!("Ab3").unwrap());
    /// 
    /// let note = pnote!("F#4").unwrap()
    ///     .transposed_down(inv!("A5").unwrap())
    ///     .unwrap();
    /// assert_eq!(note, pnote!("Bb3").unwrap());
    /// ```
    fn transposed_down(&self, interval: Interval) -> Self::Output {
        let mut new = self.moved_by(-interval.to_diatonic_steps());
        match new {
            Some(ref mut new) => {
                let semitones = interval.to_semitones();
                let diff = semitones - new.interval_to(&self).to_semitones();
                new.note.accidental -= diff;
                Some(new.clone())
            }
            None => None,
        }
    }
}

lazy_static! {
    static ref NOTE_RE: Regex = Regex::new("^([A-Ga-g])([#x𝄪b♯♯♭♭♮]*)$").unwrap();
}

use std::ops::{Add, AddAssign, Sub, SubAssign};

impl Add<Interval> for Note {
    type Output = Self;
    fn add(self, rhs: Interval) -> Self::Output {
        self.transposed_up(rhs)
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = self.transposed_up(rhs);
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
        self.transposed_down(rhs)
    }
}

impl SubAssign<Interval> for Note {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = self.transposed_down(rhs);
    }
}

impl Add<Interval> for PitchedNote {
    type Output = Option<Self>;
    fn add(self, rhs: Interval) -> Self::Output {
        self.transposed_up(rhs)
    }
}

impl AddAssign<Interval> for PitchedNote {
    fn add_assign(&mut self, rhs: Interval) {
        // *self = self.transposed_up(rhs);
    }
}

impl Sub for PitchedNote {
    type Output = Interval;
    fn sub(self, rhs: Self) -> Self::Output {
        rhs.interval_to(&self)
    }
}

impl Sub<Interval> for PitchedNote {
    type Output = Option<Self>;
    fn sub(self, rhs: Interval) -> Self::Output {
        self.transposed_down(rhs)
    }
}

impl SubAssign<Interval> for PitchedNote {
    fn sub_assign(&mut self, rhs: Interval) {
        // *self = self.transposed_down(rhs);
    }
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
            None => err!(InvalidNoteName),
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
                    nope!(InvalidOctave);
                }

                Ok(Self { note, octave })
            }
            None => nope!(InvalidNoteName),
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
