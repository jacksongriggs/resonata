use crate::{error::*, intervals::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

impl From<i32> for IntervalSize {
    fn from(value: i32) -> Self {
        match value.abs() % 7 {
            0 => Unison,
            1 => Second,
            2 => Third,
            3 => Fourth,
            4 => Fifth,
            5 => Sixth,
            6 => Seventh,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for IntervalSize {
    fn from(value: u8) -> Self {
        match value % 7 {
            0 => Unison,
            1 => Second,
            2 => Third,
            3 => Fourth,
            4 => Fifth,
            5 => Sixth,
            6 => Seventh,
            _ => unreachable!(),
        }
    }
}

impl FromStr for IntervalSize {
    type Err = ResonataError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "U" => Ok(Unison),
            "2" => Ok(Second),
            "3" => Ok(Third),
            "4" => Ok(Fourth),
            "5" => Ok(Fifth),
            "6" => Ok(Sixth),
            "7" => Ok(Seventh),
            _ => nope!(InvalidIntervalFormat),
        }
    }
}

impl Display for IntervalSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Unison => write!(f, "U"),
            Second => write!(f, "2"),
            Third => write!(f, "3"),
            Fourth => write!(f, "4"),
            Fifth => write!(f, "5"),
            Sixth => write!(f, "6"),
            Seventh => write!(f, "7"),
        }
    }
}

impl Debug for IntervalSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

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
