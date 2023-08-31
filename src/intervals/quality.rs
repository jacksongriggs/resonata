pub use IntervalQuality::*;

mod utils;

/// A musical interval quality
/// Diminished and augmented take a number to represent
/// the number of diminished or augmented intervals. For example,
/// `Diminished(2)` would be a double diminished interval.
#[derive(Clone, Copy)]
pub enum IntervalQuality {
    Diminished(u8),
    Augmented(u8),
    Minor,
    Major,
    Perfect,
}

impl IntervalQuality {
    // Inverts the interval quality
    // Diminished becomes augmented, augmented becomes diminished
    // Minor becomes major, major becomes minor
    // Perfect stays perfect
    pub fn invert(&self) -> Self {
        match self {
            Diminished(n) => Augmented(*n),
            Augmented(n) => Diminished(*n),
            Minor => Major,
            Major => Minor,
            Perfect => Perfect,
        }
    }
}