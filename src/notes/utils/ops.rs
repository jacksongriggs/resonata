use std::ops::{Add, Sub, AddAssign, SubAssign};
use crate::Interval;
use super::*;

impl Add<u8> for Note {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        let number = u8::from(self) + rhs % 12;
        Self::from(number)
    }
}

impl AddAssign<u8> for Note {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl Sub<u8> for Note {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        let number = std::cmp::max(u8::from(self) as i8 - rhs as i8, 0) as u8;
        Self::from(number)
    }
}

impl SubAssign<u8> for Note {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl Sub for Note {
    type Output = crate::Interval;
    fn sub(self, rhs: Self) -> Self::Output {
        crate::Interval::from(u8::from(self) as i8 - u8::from(rhs) as i8)
    }
}

impl Add<Interval> for Note {
    type Output = Option<Self>;
    fn add(self, rhs: Interval) -> Self::Output {
        let number = u8::from(self) + u8::from(rhs);
        if number > 127 {
            None
        } else {
            Some(Self::from(number))
        }
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = (*self + rhs).unwrap();
    }
}

impl Sub<Interval> for Note {
    type Output = Self;
    fn sub(self, rhs: Interval) -> Self::Output {
        let value = u8::from(self) as i8 - i8::from(rhs);
        Self::from((value % 12) as u8)
    }
}

impl SubAssign<Interval> for Note {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - rhs;
    }
}

impl Add<u8> for PitchedNote {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        let value = u8::from(self) as u16 + rhs as u16;
        Self::from((value % 127) as u8)
    }
}

impl AddAssign<u8> for PitchedNote {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl Sub<u8> for PitchedNote {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        let value = u8::from(self) as i8 - rhs as i8;
        Self::from((value % 127) as u8)
    }
}

impl SubAssign<u8> for PitchedNote {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl Sub for PitchedNote {
    type Output = crate::Interval;
    fn sub(self, rhs: Self) -> Self::Output {
        crate::Interval::from(u8::from(self) as i8 - u8::from(rhs) as i8)
    }
}

impl Add<Interval> for PitchedNote {
    type Output = Self;
    fn add(self, rhs: Interval) -> Self::Output {
        let number = u8::from(self) as u16 + u8::from(rhs) as u16;
        Self::from((number % 127) as u8)
    }
}

impl AddAssign<Interval> for PitchedNote {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

impl Sub<Interval> for PitchedNote {
    type Output = Self;
    fn sub(self, rhs: Interval) -> Self::Output {
        let value = u8::from(self) as i8 - i8::from(rhs);
        Self::from((value % 12) as u8)
    }
}

impl SubAssign<Interval> for PitchedNote {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - rhs;
    }
}