pub use IntervalQuality::*;

pub mod utils;

/// A musical interval quality
/// Diminished and augmented take a number to represent
/// the number of diminished or augmented intervals. For example,
/// `Diminished(2)` would be a double diminished interval.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IntervalQuality {
    Diminished(u8),
    Augmented(u8),
    Minor,
    Major,
    Perfect,
}

impl IntervalQuality {
    pub fn to_semitones(&self) -> i8 {
        match self {
            Diminished(n) => -(*n as i8),
            Augmented(n) => *n as i8,
            Minor => -1,
            Major => 0,
            Perfect => 0,
        }
    }
}