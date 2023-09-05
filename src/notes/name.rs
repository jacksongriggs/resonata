use crate::{error::*, notes::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

impl NoteName {
    /// Returns the chromatic number of the note name, based on the C major scale
    pub fn to_chromatic_scale_degree(&self) -> u8 {
        match self {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        }
    }

    /// Returns the note name from the given chromatic number, based on the C major scale
    pub fn from_chromatic_scale_degree(number: u8) -> Self {
        match number % 12 {
            0 => NoteName::C,
            1 => NoteName::C,
            2 => NoteName::D,
            3 => NoteName::D,
            4 => NoteName::E,
            5 => NoteName::F,
            6 => NoteName::F,
            7 => NoteName::G,
            8 => NoteName::G,
            9 => NoteName::A,
            10 => NoteName::A,
            11 => NoteName::B,
            _ => unreachable!(),
        }
    }
}

impl Iterator for NoteName {
    type Item = NoteName;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self {
            NoteName::C => NoteName::D,
            NoteName::D => NoteName::E,
            NoteName::E => NoteName::F,
            NoteName::F => NoteName::G,
            NoteName::G => NoteName::A,
            NoteName::A => NoteName::B,
            NoteName::B => NoteName::C,
        };
        *self = next;
        Some(next)
    }
}

impl From<i32> for NoteName {
    fn from(value: i32) -> Self {
        match value.abs() % 7 {
            0 => NoteName::C,
            1 => NoteName::D,
            2 => NoteName::E,
            3 => NoteName::F,
            4 => NoteName::G,
            5 => NoteName::A,
            6 => NoteName::B,
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

impl Add<Size> for NoteName {
    type Output = Self;
    fn add(self, rhs: Size) -> Self::Output {
        Self::from(self + rhs as i32)
    }
}

impl Add<i32> for NoteName {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Self::from((self as i32 + rhs + 7) % 7)
    }
}

impl AddAssign<Size> for NoteName {
    fn add_assign(&mut self, rhs: Size) {
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

impl SubAssign<Size> for NoteName {
    fn sub_assign(&mut self, rhs: Size) {
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

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "c" => Ok(NoteName::C),
            "d" => Ok(NoteName::D),
            "e" => Ok(NoteName::E),
            "f" => Ok(NoteName::F),
            "g" => Ok(NoteName::G),
            "a" => Ok(NoteName::A),
            "b" => Ok(NoteName::B),
            _ => nope!(InvalidNoteName(s.to_string())),
        }
    }
}

impl Display for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            NoteName::C => "C",
            NoteName::D => "D",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::G => "G",
            NoteName::A => "A",
            NoteName::B => "B",
        };
        write!(f, "{}", token)
    }
}

impl Debug for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
