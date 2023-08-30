pub use Accidental::*;

pub mod utils;
mod tests;

/// A musical accidental
/// Flats and sharps take a number to represent
/// the number of flats or sharps. For example,
/// `Flat(2)` would be a double flat.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Accidental {
    Flat(u8),
    Natural,
    Sharp(u8),
}

impl Accidental {
    pub fn from_chromatic_scale_degree(number: u8) -> Self {
        match number % 12 {
            0 | 2 | 4 | 5 | 7 | 9 | 11 => Natural,
            1 | 3 | 6 | 8 | 10 => Sharp(1),
            _ => unreachable!(),
        }
    }
}