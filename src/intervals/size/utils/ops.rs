use super::*;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl Add<u8> for IntervalSize {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        IntervalSize::from(self as u8 + rhs % 7)
    }
}

impl Sub<u8> for IntervalSize {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        IntervalSize::from((self as i8 - rhs as i8 % 7).abs() as u8)
    }
}

impl Add for IntervalSize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs as u8
    }
}

impl Sub for IntervalSize {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs as u8
    }
}

impl AddAssign for IntervalSize {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for IntervalSize {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
