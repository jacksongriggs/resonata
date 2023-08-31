use super::super::*;

mod ops;
mod fmt;
mod tests;

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