pub use Accidental::*;

pub mod utils;

/// A musical accidental
/// Flats and sharps take a number to represent
/// the number of flats or sharps. For example,
/// `Flat(2)` would be a double flat.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Accidental {
    Flat(u8),
    Natural,
    Sharp(u8),
}

impl Accidental {
    pub fn to_semitones(&self) -> i8 {
        match self {
            Flat(n) => -(*n as i8),
            Natural => 0,
            Sharp(n) => *n as i8,
        }
    }
}