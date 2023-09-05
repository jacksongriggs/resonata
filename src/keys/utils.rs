use crate::{intervals::Interval, keys::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign}, str::FromStr,
};

impl FromStr for Key {
    type Err = ResonataError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.replace(",", " ");
        let notes =
            s.split_whitespace().map(|s| s.parse::<Note>()).collect::<Result<Vec<Note>>>()?;
        Ok(Key::new(notes))
    }
}

impl Add<Interval> for Key {
    type Output = Self;
    fn add(self, rhs: Interval) -> Self::Output {
        let mut key = self.clone();
        key += rhs;
        key
    }
}

impl Sub<Interval> for Key {
    type Output = Self;
    fn sub(self, rhs: Interval) -> Self::Output {
        let mut key = self.clone();
        key -= rhs;
        key
    }
}

impl AddAssign<Interval> for Key {
    fn add_assign(&mut self, rhs: Interval) {
        let notes = self.pitches().iter().map(|note| *note + rhs).collect::<Vec<_>>();
        for note in notes {
            self.set_pitch(note);
        }
    }
}

impl SubAssign<Interval> for Key {
    fn sub_assign(&mut self, rhs: Interval) {
        let notes = self.pitches().iter().map(|note| *note - rhs).collect::<Vec<_>>();
        for note in notes {
            self.set_pitch(note);
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let notes =
            self.pitches().iter().map(|note| note.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "{}", notes)
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
