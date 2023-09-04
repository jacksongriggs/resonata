pub use Accidental::*;

pub mod utils;

/// A musical accidental
/// Flats and sharps take a number to represent
/// the number of flats or sharps.
///
/// To convert an accidental to a number of semitones,
/// use the to_semitones method.
///
/// To convert a number of semitones to an accidental,
/// use the from_semitones method.
///
/// ### Examples
/// ```
/// use resonata::notes::accidental::*;
///
/// assert_eq!(Flat(1).to_semitones(), -1);
/// assert_eq!(Natural.to_semitones(), 0);
/// assert_eq!(Sharp(2).to_semitones(), 2);
///
/// assert_eq!(Accidental::from_semitones(-1), Flat(1));
/// assert_eq!(Accidental::from_semitones(0), Natural);
/// assert_eq!(Accidental::from_semitones(2), Sharp(2));
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Accidental {
    Flat(u8),
    Natural,
    Sharp(u8),
}

impl Accidental {
    /// Returns the number of semitones from the given accidental
    /// to the natural accidental.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::accidental::*;
    ///
    /// assert_eq!(Flat(1).to_semitones(), -1);
    /// assert_eq!(Natural.to_semitones(), 0);
    /// assert_eq!(Sharp(2).to_semitones(), 2);
    /// ```
    pub fn to_semitones(&self) -> i32 {
        match self {
            Flat(n) => -(*n as i32),
            Natural => 0,
            Sharp(n) => *n as i32,
        }
    }

    /// Returns an accidental from the given number of semitones from
    /// the natural accidental. Values will be clamped to the range
    /// (-127, 127), which should be more than enough for most use cases!
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::accidental::*;
    ///
    /// assert_eq!(Accidental::from_semitones(-1), Flat(1));
    /// assert_eq!(Accidental::from_semitones(0), Natural);
    /// assert_eq!(Accidental::from_semitones(2), Sharp(2));
    /// ```
    pub fn from_semitones(semitones: i32) -> Self {
        match semitones {
            0 => Natural,
            n if n > 0 => Sharp(n.min(127) as u8),
            n if n < 0 => Flat(-n.max(-127) as u8),
            _ => unreachable!(),
        }
    }
}
