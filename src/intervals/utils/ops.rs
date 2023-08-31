use std::ops::{Add, Sub, AddAssign, SubAssign};
use super::*;

//===========[Add]==============================================================
impl Add<i8> for Interval {
    type Output = Self;
    fn add(self, rhs: i8) -> Self::Output {
        let value = i8::from(self) as i16 + rhs as i16;
        Interval::from((value.abs() % 127) as i8)
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + i8::from(rhs)
    }
}

//===========[Sub]==============================================================
impl Sub<i8> for Interval {
    type Output = Self;
    fn sub(self, rhs: i8) -> Self::Output {
        let value = i8::from(self) as i16 - rhs as i16;
        Interval::from((value.abs() % 127) as i8)
    }
}

impl Sub<Interval> for i8 {
    type Output = Interval;
    fn sub(self, rhs: Interval) -> Self::Output {
        let value = self as i16 - i8::from(rhs) as i16;
        Interval::from((value.abs() % 127) as i8)
    }
}

impl Sub for Interval {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self - i8::from(rhs)
    }
}

//===========[AddAssign]========================================================
impl AddAssign<i8> for Interval {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl AddAssign for Interval {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

//===========[SubAssign]========================================================
impl SubAssign<i8> for Interval {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}

impl SubAssign<Interval> for i8 {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - i8::from(rhs);
    }
}

impl SubAssign for Interval {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}