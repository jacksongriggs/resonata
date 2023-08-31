use super::*;
use crate::Interval;
use std::ops::{Add, AddAssign, Sub, SubAssign};

//===========[Note: Add]=========================================================
impl Add<u8> for Note {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        let value = u8::from(self) + rhs % 12;
        Note::from(value)
    }
}

impl Add<Interval> for Note {
    type Output = Self;
    fn add(self, rhs: Interval) -> Self::Output {
        self + u8::from(rhs)
    }
}

//===========[Note: AddAssign]===================================================
impl AddAssign<u8> for Note {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

//===========[Note: Sub]=========================================================
impl Sub<u8> for Note {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        let value = u8::from(self) as i8 - (rhs % 12) as i8;
        Note::from(value as u8)
    }
}

impl Sub for Note {
    type Output = Interval;
    fn sub(self, rhs: Self) -> Self::Output {
        Interval::from(u8::from(self) as i8 - u8::from(rhs) as i8)
    }
}

impl Sub<Interval> for Note {
    type Output = Self;
    fn sub(self, rhs: Interval) -> Self::Output {
        self - u8::from(rhs)
    }
}

//===========[Note: SubAssign]===================================================
impl SubAssign<u8> for Note {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl SubAssign<Interval> for Note {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - rhs;
    }
}

//===========[PitchedNote: Add]==================================================
impl Add<u8> for PitchedNote {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        let value = u8::from(self) as u16 + rhs as u16;
        PitchedNote::from((value % 127) as u8)
    }
}

impl Add<Interval> for PitchedNote {
    type Output = Self;
    fn add(self, rhs: Interval) -> Self::Output {
        self + u8::from(rhs)
    }
}

//===========[PitchedNote: AddAssign]=============================================
impl AddAssign<u8> for PitchedNote {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl AddAssign<Interval> for PitchedNote {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

//===========[PitchedNote: Sub]==================================================
impl Sub<u8> for PitchedNote {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        let value = u8::from(self) as i8 - (rhs % 127) as i8;
        PitchedNote::from(value as u8)
    }
}

impl Sub for PitchedNote {
    type Output = Interval;
    fn sub(self, rhs: Self) -> Self::Output {
        let self_value = u8::from(self) as i8;
        let rhs_value = u8::from(rhs) as i8;
        Interval::from(self_value - rhs_value)
    }
}

impl Sub<Interval> for PitchedNote {
    type Output = Self;
    fn sub(self, rhs: Interval) -> Self::Output {
        self - u8::from(rhs)
    }
}

//===========[PitchedNote: SubAssign]=============================================
impl SubAssign<u8> for PitchedNote {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl SubAssign<Interval> for PitchedNote {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - rhs;
    }
}
