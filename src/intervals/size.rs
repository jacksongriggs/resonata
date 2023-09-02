use std::str::FromStr;

pub use IntervalSize::*;

mod utils;

/// A musical interval size
/// To get an octave or above, use the `octaves` field
/// in the `Interval` struct
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
pub enum IntervalSize {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
}

impl IntervalSize {
    /// Returns the interval size from the given string, if the string is valid.
    /// Valid strings are "U", "2", "3", "4", "5", "6", "7".
    pub fn from_string(s: &str) -> Option<Self> {
        match IntervalSize::from_str(s) {
            Ok(size) => Some(size),
            Err(_) => None,
        }
    }

    /// Returns the number of semitones in the interval size, based on the C major scale
    pub fn to_semitones(&self) -> u8 {
        match self {
            Unison => 0,
            Second => 2,
            Third => 4,
            Fourth => 5,
            Fifth => 7,
            Sixth => 9,
            Seventh => 11,
        }
    }

    /// Inverts the interval size. For example, a third inverted is a sixth.
    pub fn invert(&self) -> Self {
        match self {
            Unison => Unison,
            Second => Seventh,
            Third => Sixth,
            Fourth => Fifth,
            Fifth => Fourth,
            Sixth => Third,
            Seventh => Second,
        }
    }
}
