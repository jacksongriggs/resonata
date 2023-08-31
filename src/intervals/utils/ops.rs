use std::ops::{Add, Sub, AddAssign, SubAssign};
use super::*;

impl Add<i8> for Interval {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        let value = i8::from(self) as i16 + rhs as i16;
        Interval::from((value.abs() % 127) as i8)
    }
}

impl AddAssign<i8> for Interval {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl Sub<Interval> for i8 {
    type Output = Interval;

    fn sub(self, rhs: Interval) -> Self::Output {
        Interval::from(self - i8::from(rhs))
    }
}

impl SubAssign<Interval> for i8 {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - i8::from(rhs);
    }
}

impl Sub<i8> for Interval {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        Interval::from(i8::from(self) - rhs)
    }
}

impl SubAssign<i8> for Interval {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Interval::from(((u8::from(self) as u16 + u8::from(other) as u16) % 127) as u8)
    }
}

impl AddAssign for Interval {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from((i8::from(self) - i8::from(other)).abs())
    }
}

impl SubAssign for Interval {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}