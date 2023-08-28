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
    pub fn to_number(&self) -> u8 {
        match self {
            Unison => 1,
            Second => 2,
            Third => 3,
            Fourth => 4,
            Fifth => 5,
            Sixth => 6,
            Seventh => 7,
        }
    }

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

    // String representation of the interval size
    // Unison (U) and octave (8ve) are special cases, as they
    // are not represented by a number, but by the
    // word itself. All other intervals (2..7, 9.._) are represented
    // by a number, followed by the suffix "th", "st", "nd" or "rd"
    pub fn as_str(&self, octaves: u8) -> String {
        let number = self.to_number() + (octaves * 7);
        match number {
            1 => "U".to_string(),
            8 => "8ve".to_string(),
            _ => {
                let suffix = match number % 10 {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                };
                format!("{}{}", number, suffix)
            }
        }
    }
}