use std::ops::{Add, Sub, AddAssign, SubAssign};
use super::*;

impl Add<i8> for Accidental {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        Self::from(i8::from(self) + rhs)
    }
}

impl AddAssign<i8> for Accidental {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl Sub<i8> for Accidental {
    type Output = Self;
    
    fn sub(self, rhs: i8) -> Self::Output {
        Self::from(i8::from(self) - rhs)
    }
}

impl SubAssign<i8> for Accidental {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}