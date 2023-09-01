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
    /// Returns the number of diatonic semitones in the interval, based on the C major scale
    /// For example, a third has 4 diatonic semitones (C - E)
    pub fn to_diatonic_semitones(&self) -> u8 {
        match self {
            Unison => 0,   // Perfect = 0, Augmented = 1, Diminished = 1
            Second => 2,   // Major = 2, Minor = 1, Augmented = 3, Diminished = 0
            Third => 4,    // Major = 4, Minor = 3, Augmented = 5, Diminished = 2
            Fourth => 5,   // Perfect = 5, Augmented = 6, Diminished = 4
            Fifth => 7,    // Perfect = 7, Augmented = 8, Diminished = 6
            Sixth => 9,    // Major = 9, Minor = 8, Augmented = 10, Diminished = 7
            Seventh => 11, // Major = 11, Minor = 10, Augmented = 12, Diminished = 9
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
