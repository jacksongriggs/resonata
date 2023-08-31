use super::*;
use std::ops::{Add, AddAssign, Sub, SubAssign};

//===========[Add]==============================================================
impl Add<u8> for IntervalSize {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        Self::from(u8::from(self) + (rhs % 7))
    }
}

impl Add for IntervalSize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self + u8::from(rhs)
    }
}

//===========[Sub]==============================================================
impl Sub<u8> for IntervalSize {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        Self::from(u8::from(self) + 7 - (rhs % 7))
    }
}

impl Sub for IntervalSize {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self - u8::from(rhs)
    }
}

//===========[AddAssign]========================================================
impl AddAssign for IntervalSize {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<u8> for IntervalSize {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

//===========[SubAssign]========================================================
impl SubAssign for IntervalSize {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<u8> for IntervalSize {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}
