use crate::error::*;
use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use super::*;

/// A musical interval quality
/// The interval quality is the quality of the interval, which
/// can be diminished, minor, major, perfect or augmented.
///
/// The number of diminished or augmented intervals is represented
/// by a number. For example, `Diminished(2)` would be a double diminished interval.
#[derive(Clone, Copy)]
pub enum Quality {
    Diminished(u8),
    Augmented(u8),
    Minor,
    Major,
    Perfect,
}

#[derive(Debug, Clone, Copy)]
pub enum ImperfectType {
    Major,
    Minor,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AlterationType {
    Diminished,
    Augmented,
}

/// A musical interval alteration. The degree specifies the degree of the alteration.
/// For example, a double diminished interval has a degree of 2. Note that while the
/// degree must be a positive integer, inclusive, using the Alteration::degree() method
/// will return a negative number if the alteration is diminished.
///
/// ### Errors
/// Returns an error if the degree is not between 1 and 127, inclusive.
///
/// ### Examples
/// ```
/// use resonata::intervals::*;
///
/// let alt = Alteration::augmented(1).unwrap();
/// assert_eq!(alt.degree(), 1);
/// assert_eq!(alt.alteration_type(), AlterationType::Augmented);
///
/// let alt = Alteration::diminished(2).unwrap();
/// assert_eq!(alt.degree(), -2);
/// assert_eq!(alt.alteration_type(), AlterationType::Diminished);
///
/// let invalid_alt = Alteration::augmented(0);
/// assert!(invalid_alt.is_err());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Alteration {
    alteration_type: AlterationType,
    degree: u8,
}

impl Alteration {
    pub fn augmented(degree: u8) -> Result<Alteration> {
        Alteration::build(AlterationType::Augmented, degree)
    }

    pub fn diminished(degree: u8) -> Result<Alteration> {
        Alteration::build(AlterationType::Diminished, degree)
    }

    pub fn build(alteration_type: AlterationType, degree: u8) -> Result<Alteration> {
        if degree >= 1 && degree <= 127 {
            Ok(Alteration { alteration_type, degree })
        } else {
            err!(IntervalError::InvalidAlterationDegree(degree))
        }
    }

    /// Returns the degree of the alteration.
    /// The degree is the number of diminished or augmented intervals.
    /// For example, a double diminished interval has a degree of 2.
    pub fn degree(&self) -> i8 {
        match self.alteration_type {
            AlterationType::Diminished => -(self.degree as i8),
            AlterationType::Augmented => self.degree as i8,
        }
    }

    /// Returns the type of the alteration.
    /// The type is either diminished or augmented.
    pub fn alteration_type(&self) -> AlterationType {
        self.alteration_type
    }
}

impl Invert for Alteration {
    fn invert(self) -> Self {
        match self.alteration_type {
            AlterationType::Diminished => {
                Alteration { alteration_type: AlterationType::Augmented, degree: self.degree }
            }
            AlterationType::Augmented => {
                Alteration { alteration_type: AlterationType::Diminished, degree: self.degree }
            }
        }
    }
}

impl From<Quality> for ImperfectType {
    fn from(quality: Quality) -> Self {
        match quality {
            Quality::Major => ImperfectType::Major,
            Quality::Minor => ImperfectType::Minor,
            _ => unreachable!(),
        }
    }
}

impl From<Quality> for AlterationType {
    fn from(quality: Quality) -> Self {
        match quality {
            Quality::Augmented(_) => AlterationType::Augmented,
            Quality::Diminished(_) => AlterationType::Diminished,
            _ => unreachable!(),
        }
    }
}

impl Quality {
    /// Inverts the interval quality.
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    ///
    /// assert_eq!(Quality::Diminished(1).invert(), Quality::Augmented(1));
    /// assert_eq!(Quality::Augmented(3).invert(), Quality::Diminished(3));
    /// assert_eq!(Quality::Minor.invert(), Quality::Major);
    /// assert_eq!(Quality::Major.invert(), Quality::Minor);
    /// assert_eq!(Quality::Perfect.invert(), Quality::Perfect);
    pub fn invert(&self) -> Self {
        match self {
            Quality::Diminished(n) => Quality::Augmented(*n),
            Quality::Augmented(n) => Quality::Diminished(*n),
            Quality::Minor => Quality::Major,
            Quality::Major => Quality::Minor,
            Quality::Perfect => Quality::Perfect,
        }
    }
}

impl From<Quality> for i8 {
    fn from(iq: Quality) -> Self {
        match iq {
            Quality::Diminished(n) => -(n as i8) - 1,
            Quality::Augmented(n) => n as i8,
            Quality::Minor => -1,
            Quality::Major => 0,
            Quality::Perfect => 0,
        }
    }
}

impl FromStr for Quality {
    type Err = ResonataError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.is_empty() {
            nope!(InvalidIntervalQuality);
        }

        match s {
            "m" => Ok(Quality::Minor),
            "M" => Ok(Quality::Major),
            "P" => Ok(Quality::Perfect),
            _ => {
                let mut chars = s.chars();
                let first_char = chars.next().unwrap();
                let mut n = 1; // One 'd' or 'A' already there in first_char

                for c in chars {
                    if c == first_char {
                        n += 1;
                    } else {
                        nope!(InvalidIntervalQuality);
                    }
                }

                match first_char {
                    'd' => Ok(Quality::Diminished(n)),
                    'A' => Ok(Quality::Augmented(n)),
                    _ => err!(InvalidIntervalQuality),
                }
            }
        }
    }
}

impl Display for Quality {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            Quality::Diminished(n) => std::iter::repeat('d').take(*n as usize).collect(),
            Quality::Augmented(n) => std::iter::repeat('A').take(*n as usize).collect(),
            Quality::Minor => "m".to_string(),
            Quality::Major => "M".to_string(),
            Quality::Perfect => "P".to_string(),
        };

        write!(f, "{}", token)
    }
}

impl Debug for Quality {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Quality {
    fn eq(&self, other: &Self) -> bool {
        i8::from(*self) == i8::from(*other)
    }
}

impl Eq for Quality {}

impl PartialOrd for Quality {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        i8::from(*self).partial_cmp(&i8::from(*other))
    }
}

impl Ord for Quality {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        i8::from(*self).cmp(&i8::from(*other))
    }
}