use crate::{notes::*, error::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
    ops::{Add, AddAssign, Sub, SubAssign},
};

impl Iterator for NoteName {
    type Item = NoteName;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self {
            C => D,
            D => E,
            E => F,
            F => G,
            G => A,
            A => B,
            B => C,
        };
        *self = next;
        Some(next)
    }
}

impl From<i32> for NoteName {
    fn from(value: i32) -> Self {
        match value.abs() % 7 {
            0 => C,
            1 => D,
            2 => E,
            3 => F,
            4 => G,
            5 => A,
            6 => B,
            _ => unreachable!(),
        }
    }
}

impl Add for NoteName {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from((self + rhs as i32) as i32 % 7)
    }
}

impl Add<IntervalSize> for NoteName {
    type Output = Self;
    fn add(self, rhs: IntervalSize) -> Self::Output {
        Self::from(self + rhs as i32)
    }
}

impl Add<i32> for NoteName {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Self::from((self as i32 + rhs + 7) % 7)
    }
}

impl AddAssign<IntervalSize> for NoteName {
    fn add_assign(&mut self, rhs: IntervalSize) {
        *self += rhs as i32;
    }
}

impl AddAssign<i32> for NoteName {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl Sub for NoteName {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from((self as i32 - rhs as i32) % 7)
    }
}

impl Sub<i32> for NoteName {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        Self::from((self as i32 - rhs + 7) % 7)
    }
}

impl SubAssign<IntervalSize> for NoteName {
    fn sub_assign(&mut self, rhs: IntervalSize) {
        *self -= rhs as i32;
    }
}

impl SubAssign<i32> for NoteName {
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs;
    }
}

impl FromStr for NoteName {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "c" => Ok(C),
            "d" => Ok(D),
            "e" => Ok(E),
            "f" => Ok(F),
            "g" => Ok(G),
            "a" => Ok(A),
            "b" => Ok(B),
            _ => nope!(InvalidNoteName),
        }
    }
}

impl Display for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            A => "A",
            B => "B",
        };
        write!(f, "{}", token)
    }
}

impl Debug for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
