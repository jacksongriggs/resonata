use std::ops::{Add, Sub, AddAssign, SubAssign};
use super::*;

impl Add<u8> for IntervalSize {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        Self::from(u8::from(self) + (rhs % 7))
    }
}

impl AddAssign<u8> for IntervalSize {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl Sub<u8> for IntervalSize {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        Self::from(u8::from(self) + 7 - (rhs % 7))
    }
}

impl SubAssign<u8> for IntervalSize {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl Add for IntervalSize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self + u8::from(rhs)
    }
}

impl AddAssign for IntervalSize {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for IntervalSize {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self - u8::from(rhs)
    }
}

impl SubAssign for IntervalSize {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}