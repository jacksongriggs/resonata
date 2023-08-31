use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr, ops::{Add, Sub, AddAssign, SubAssign}};
use super::super::*;

impl From<u8> for NoteName {
    fn from(value: u8) -> Self {
        match value % 12 {
            0 | 1 => C,
            2 | 3 => D,
            4 => E,
            5 | 6 => F,
            7 | 8 => G,
            9 | 10 => A,
            11 => B,
            _ => unreachable!(),
        }
    }
}

impl From<NoteName> for u8 {
    fn from(name: NoteName) -> Self {
        match name {
            C => 0,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
            A => 9,
            B => 11,
        }
    }
}

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
            _ => nope!(InvalidNoteName)
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