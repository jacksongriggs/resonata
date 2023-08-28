use super::*;
pub mod utils;

/// A musical note name
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NoteName { 
    C, D, E, F, G, A, B, 
}

impl NoteName {
    /// Returns the chromatic number of the note name, based on the C major scale
    pub fn to_chromatic_number (&self) -> u8 {
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
}

