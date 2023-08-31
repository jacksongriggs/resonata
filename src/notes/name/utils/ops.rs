use super::*;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl Add<u8> for NoteName {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self::from(u8::from(self) + rhs % 12)
    }
}

impl AddAssign<u8> for NoteName {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl Sub<u8> for NoteName {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Self::from(((u8::from(self) as i8 - rhs as i8) % 12) as u8)
    }
}

impl SubAssign<u8> for NoteName {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}
