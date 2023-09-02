use std::str::FromStr;
pub use IntervalQuality::*;

mod utils;

/// A musical interval quality
/// The interval quality is the quality of the interval, which
/// can be diminished, minor, major, perfect or augmented.
///
/// The number of diminished or augmented intervals is represented
/// by a number. For example, `Diminished(2)` would be a double diminished interval.
#[derive(Clone, Copy)]
pub enum IntervalQuality {
    Diminished(u8),
    Augmented(u8),
    Minor,
    Major,
    Perfect,
}

impl IntervalQuality {
    /// Returns an interval quality from the given string, if possible.
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    /// use IntervalQuality as IQ;
    ///
    /// assert_eq!(IQ::from_string("M").unwrap(), Major);
    /// assert_eq!(IQ::from_string("m").unwrap(), Minor);
    /// assert_eq!(IQ::from_string("P").unwrap(), Perfect);
    /// assert_eq!(IQ::from_string("A").unwrap(), Augmented(1));
    /// assert_eq!(IQ::from_string("d").unwrap(), Diminished(1));
    /// assert_eq!(IQ::from_string("AA").unwrap(), Augmented(2));
    /// assert_eq!(IQ::from_string("ddd").unwrap(), Diminished(3));
    /// ```
    pub fn from_string(s: &str) -> Option<Self> {
        match IntervalQuality::from_str(s) {
            Ok(quality) => Some(quality),
            Err(_) => None,
        }
    }

    /// Inverts the interval quality.
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    ///
    /// assert_eq!(Diminished(1).invert(), Augmented(1));
    /// assert_eq!(Augmented(3).invert(), Diminished(3));
    /// assert_eq!(Minor.invert(), Major);
    /// assert_eq!(Major.invert(), Minor);
    /// assert_eq!(Perfect.invert(), Perfect);
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
