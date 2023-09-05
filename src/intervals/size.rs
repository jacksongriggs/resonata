use crate::error::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

use super::*;

/// A musical interval size
/// To get an octave or above, use the `octaves` field
/// in the `Interval` struct
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
pub enum Size {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
}

#[derive(Debug, Clone, Copy)]
pub enum PerfectSize {
    Unison = 0,
    Fourth = 5,
    Fifth = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum ImperfectSize {
    Second = 2,
    Third = 4,
    Sixth = 9,
    Seventh = 11,
}

#[derive(Debug, Clone, Copy)]
pub enum IntervalSizeType {
    Perfect(PerfectSize),
    Imperfect(ImperfectSize),
}

impl Invert for PerfectSize {
    fn invert(self) -> Self {
        match self {
            PerfectSize::Unison => PerfectSize::Unison,
            PerfectSize::Fourth => PerfectSize::Fifth,
            PerfectSize::Fifth => PerfectSize::Fourth,
        }
    }
}

impl Invert for ImperfectSize {
    fn invert(self) -> Self {
        match self {
            ImperfectSize::Second => ImperfectSize::Seventh,
            ImperfectSize::Third => ImperfectSize::Sixth,
            ImperfectSize::Sixth => ImperfectSize::Third,
            ImperfectSize::Seventh => ImperfectSize::Second,
        }
    }
}

impl Invert for IntervalSizeType {
    fn invert(self) -> Self {
        match self {
            IntervalSizeType::Perfect(interval) => IntervalSizeType::Perfect(interval.invert()),
            IntervalSizeType::Imperfect(interval) => IntervalSizeType::Imperfect(interval.invert()),
        }
    }
}

impl From<Size> for IntervalSizeType {
    fn from(size: Size) -> Self {
        match size {
            Size::Unison => IntervalSizeType::Perfect(PerfectSize::Unison),
            Size::Second => IntervalSizeType::Imperfect(ImperfectSize::Second),
            Size::Third => IntervalSizeType::Imperfect(ImperfectSize::Third),
            Size::Fourth => IntervalSizeType::Perfect(PerfectSize::Fourth),
            Size::Fifth => IntervalSizeType::Perfect(PerfectSize::Fifth),
            Size::Sixth => IntervalSizeType::Imperfect(ImperfectSize::Sixth),
            Size::Seventh => IntervalSizeType::Imperfect(ImperfectSize::Seventh),
        }
    }
}

impl From<Size> for PerfectSize {
    fn from(size: Size) -> Self {
        match size {
            Size::Unison => PerfectSize::Unison,
            Size::Fourth => PerfectSize::Fourth,
            Size::Fifth => PerfectSize::Fifth,
            _ => {
                println!("IntervalSize::from(IntervalSize) called with {:?}", size);
                unreachable!()
            }
        }
    }
}

impl From<PerfectSize> for Size {
    fn from(size: PerfectSize) -> Self {
        match size {
            PerfectSize::Unison => Size::Unison,
            PerfectSize::Fourth => Size::Fourth,
            PerfectSize::Fifth => Size::Fifth,
        }
    }
}

impl From<Size> for ImperfectSize {
    fn from(size: Size) -> Self {
        match size {
            Size::Second => ImperfectSize::Second,
            Size::Third => ImperfectSize::Third,
            Size::Sixth => ImperfectSize::Sixth,
            Size::Seventh => ImperfectSize::Seventh,
            _ => unreachable!(),
        }
    }
}

impl From<ImperfectSize> for Size {
    fn from(size: ImperfectSize) -> Self {
        match size {
            ImperfectSize::Second => Size::Second,
            ImperfectSize::Third => Size::Third,
            ImperfectSize::Sixth => Size::Sixth,
            ImperfectSize::Seventh => Size::Seventh,
        }
    }
}

impl From<IntervalSizeType> for Size {
    fn from(size: IntervalSizeType) -> Self {
        match size {
            IntervalSizeType::Perfect(size) => Size::from(size),
            IntervalSizeType::Imperfect(size) => Size::from(size),
        }
    }
}

impl From<PerfectSize> for IntervalSizeType {
    fn from(interval: PerfectSize) -> Self {
        IntervalSizeType::Perfect(interval)
    }
}

impl From<ImperfectSize> for IntervalSizeType {
    fn from(interval: ImperfectSize) -> Self {
        IntervalSizeType::Imperfect(interval)
    }
}

impl From<IntervalSizeType> for ImperfectSize {
    fn from(interval: IntervalSizeType) -> Self {
        match interval {
            IntervalSizeType::Imperfect(interval) => interval,
            _ => unreachable!(),
        }
    }
}

impl From<IntervalSizeType> for PerfectSize {
    fn from(interval: IntervalSizeType) -> Self {
        match interval {
            IntervalSizeType::Perfect(interval) => interval,
            _ => unreachable!(),
        }
    }
}

impl Size {
    /// Returns the number of semitones in the interval size, based on the C major scale
    pub fn to_semitones(&self) -> u8 {
        match self {
            Size::Unison => 0,
            Size::Second => 2,
            Size::Third => 4,
            Size::Fourth => 5,
            Size::Fifth => 7,
            Size::Sixth => 9,
            Size::Seventh => 11,
        }
    }

    /// Inverts the interval size. For example, a third inverted is a sixth.
    pub fn invert(&self) -> Self {
        match self {
            Size::Unison => Size::Unison,
            Size::Second => Size::Seventh,
            Size::Third => Size::Sixth,
            Size::Fourth => Size::Fifth,
            Size::Fifth => Size::Fourth,
            Size::Sixth => Size::Third,
            Size::Seventh => Size::Second,
        }
    }
}

impl From<i32> for Size {
    fn from(value: i32) -> Self {
        match value.abs() % 7 {
            0 => Size::Unison,
            1 => Size::Second,
            2 => Size::Third,
            3 => Size::Fourth,
            4 => Size::Fifth,
            5 => Size::Sixth,
            6 => Size::Seventh,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Size {
    fn from(value: u8) -> Self {
        match value % 7 {
            0 => Size::Unison,
            1 => Size::Second,
            2 => Size::Third,
            3 => Size::Fourth,
            4 => Size::Fifth,
            5 => Size::Sixth,
            6 => Size::Seventh,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Size {
    type Err = ResonataError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "U" => Ok(Size::Unison),
            "2" => Ok(Size::Second),
            "3" => Ok(Size::Third),
            "4" => Ok(Size::Fourth),
            "5" => Ok(Size::Fifth),
            "6" => Ok(Size::Sixth),
            "7" => Ok(Size::Seventh),
            _ => nope!(InvalidIntervalFormat),
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Size::Unison => write!(f, "U"),
            Size::Second => write!(f, "2"),
            Size::Third => write!(f, "3"),
            Size::Fourth => write!(f, "4"),
            Size::Fifth => write!(f, "5"),
            Size::Sixth => write!(f, "6"),
            Size::Seventh => write!(f, "7"),
        }
    }
}

impl Debug for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Add<u8> for Size {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        Size::from(self as u8 + rhs % 7)
    }
}

impl Sub<u8> for Size {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        Size::from((self as i8 - rhs as i8 % 7).abs() as u8)
    }
}

impl Add for Size {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs as u8
    }
}

impl Sub for Size {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs as u8
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
