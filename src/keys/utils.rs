use std::ops::{Add, AddAssign, Sub, SubAssign};
use crate::{keys::*, intervals::Interval, TransposeUp, TransposeDown};

impl TransposeUp for Key {
    type Output = Self;
    fn transposed_up(&self, interval: Interval) -> Self {
        self.clone() + interval
    }
}

impl TransposeDown for Key {
    type Output = Self;
    fn transposed_down(&self, interval: Interval) -> Self {
        self.clone() - interval
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
        let notes = self
            .pitches()
            .iter()
            .map(|note| *note + rhs)
            .collect::<Vec<_>>();
        for note in notes {
            self.set_pitch(note);
        }
    }
}

impl SubAssign<Interval> for Key {
    fn sub_assign(&mut self, rhs: Interval) {
        let notes = self
            .pitches()
            .iter()
            .map(|note| *note - rhs)
            .collect::<Vec<_>>();
        for note in notes {
            self.set_pitch(note);
        }
    }
}
