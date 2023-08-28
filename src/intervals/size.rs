pub use IntervalSize::*;

pub mod utils;

/// A musical interval size
/// To get an octave or above, use the `octaves` field
/// in the `Interval` struct
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    pub fn to_diatonic_semitones(&self) -> i8 {
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
}