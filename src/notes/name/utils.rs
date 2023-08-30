use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr, ops::{Add, Sub}};
use super::NoteName::{self, *};
use crate::error::NoteError::{self, *};

impl From<u8> for NoteName {
    fn from(value: u8) -> Self {
        match value % 7 {
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

    fn add(self, n: u8) -> Self::Output {
        Self::from(u8::from(self) + n % 6)
    }
}

impl Sub<u8> for NoteName {
    type Output = Self;

    fn sub(self, n: u8) -> Self::Output {
        Self::from((u8::from(self) as i8 - n as i8).abs() as u8 % 6)
    }
}

impl FromStr for NoteName {
    type Err = NoteError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "c" => Ok(C),
            "d" => Ok(D),
            "e" => Ok(E),
            "f" => Ok(F),
            "g" => Ok(G),
            "a" => Ok(A),
            "b" => Ok(B),
            _ => {
                eprintln!("NoteName: {}: {}", InvalidNoteName, s);
                Err(InvalidNoteName)
            }
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