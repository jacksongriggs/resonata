pub use NoteName::*;

pub mod utils;

/// A musical note name
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u8)]
pub enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl NoteName {
    /// Returns the chromatic number of the note name, based on the C major scale
    pub fn to_chromatic_scale_degree(&self) -> u8 {
        match self {
            C => 0,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
            A => 9,
            B => 11,
        }
    }

    /// Returns the note name from the given chromatic number, based on the C major scale
    pub fn from_chromatic_scale_degree(number: u8) -> Self {
        match number % 12 {
            0 => C,
            1 => C,
            2 => D,
            3 => D,
            4 => E,
            5 => F,
            6 => F,
            7 => G,
            8 => G,
            9 => A,
            10 => A,
            11 => B,
            _ => unreachable!()
        }
    }
}
