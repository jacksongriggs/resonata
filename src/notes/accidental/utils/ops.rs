use super::*;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl Add<i32> for Accidental {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self::from(i32::from(self) + rhs)
    }
}

impl AddAssign<i32> for Accidental {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl Sub<i32> for Accidental {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self::from(i32::from(self) - rhs)
    }
}

impl SubAssign<i32> for Accidental {
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs;
    }
}
