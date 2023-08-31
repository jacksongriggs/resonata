use std::ops::{Add, AddAssign};
use super::*;

impl Add for Scale {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut scale = self;
        scale += rhs;
        scale
    }
}

impl AddAssign for Scale {
    fn add_assign(&mut self, rhs: Self) {
        self.intervals.extend(rhs.intervals);
    }
}