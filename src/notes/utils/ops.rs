use super::*;
use crate::Interval;
use std::ops::{Add, AddAssign, Sub, SubAssign};

//===========[Note: Add]=========================================================
impl Add<i32> for Note {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        let value = i32::from(self) + rhs % 12;
        Note::from(value)
    }
}

impl Add<Interval> for Note {
    type Output = Self;
    fn add(self, rhs: Interval) -> Self::Output {
        self + i32::from(rhs)
    }
}

//===========[Note: AddAssign]===================================================
impl AddAssign<i32> for Note {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

//===========[Note: Sub]=========================================================
impl Sub<i32> for Note {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        let value = i32::from(self) as i8 - (rhs % 12) as i8;
        Note::from(value as i32)
    }
}

impl Sub for Note {
    type Output = Interval;
    fn sub(self, rhs: Self) -> Self::Output {
        Interval::from(i32::from(self) as i8 - i32::from(rhs) as i8)
    }
}

impl Sub<Interval> for Note {
    type Output = Self;
    fn sub(self, rhs: Interval) -> Self::Output {
        self - i32::from(rhs)
    }
}

//===========[Note: SubAssign]===================================================
impl SubAssign<i32> for Note {
    fn sub_assign(&mut self, rhs: i32) {
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
        PitchedNote::from((value % 128) as u8)
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
        let value = u8::from(self) as i8 - (rhs % 128) as i8;
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
