use std::{fmt::{self, Display, Formatter, Debug}, ops::Add, ops::{Sub, AddAssign, SubAssign}, str::FromStr};
use super::super::*;

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
            _ => unreachable!("Result should always be in the range 0..6"),
        }
    }
}

impl From<IntervalSize> for u8 {
    fn from(is: IntervalSize) -> Self {
        match is {
            Unison => 0,
            Second => 1,
            Third => 2,
            Fourth => 3,
            Fifth => 4,
            Sixth => 5,
            Seventh => 6,
        }
    }
}

impl Add<u8> for IntervalSize {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        Self::from(u8::from(self) + (rhs % 7))
    }
}

impl AddAssign<u8> for IntervalSize {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl Sub<u8> for IntervalSize {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        Self::from(u8::from(self) + 7 - (rhs % 7))
    }
}

impl SubAssign<u8> for IntervalSize {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl Add for IntervalSize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self + u8::from(rhs)
    }
}

impl AddAssign for IntervalSize {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for IntervalSize {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self - u8::from(rhs)
    }
}

impl SubAssign for IntervalSize {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl FromStr for IntervalSize {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if str is a cardinal number convert into u8 and then into IntervalSize
        if let Ok(number) = s.parse::<u8>() {
            return Ok(IntervalSize::from(number - 1))
        }

        // if str is an ordinal number, convert into u8 and then into IntervalSize
        if let Ok(number) = s
            .trim_end_matches("st")
            .trim_end_matches("nd")
            .trim_end_matches("rd")
            .trim_end_matches("th")
            .trim_end_matches("ve")
            .parse::<u8>() {
            return Ok(IntervalSize::from(number - 1))
        }
        
        // if str is a string, match against the string and return the corresponding IntervalSize
        match s.to_lowercase().as_str() {
            "unison" | "u" => Ok(Unison),
            "second" => Ok(Second),
            "third" => Ok(Third),
            "fourth" => Ok(Fourth),
            "fifth" => Ok(Fifth),
            "sixth" => Ok(Sixth),
            "seventh" => Ok(Seventh),
            _ => nope!(InvalidIntervalSize)
        }
    }
}

impl Display for IntervalSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // let number = *self as u8 + (octaves * 7);
        // match number {
        //     1 => "U".to_string(),
        //     8 => "8ve".to_string(),
        //     _ => {
        //         let suffix = match number % 10 {
        //             1 => "st",
        //             2 => "nd",
        //             3 => "rd",
        //             _ => "th",
        //         };
        //         format!("{}{}", number, suffix)
        //     }
        // }
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