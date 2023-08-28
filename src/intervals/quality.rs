pub use IntervalQuality::*;

pub mod utils;

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