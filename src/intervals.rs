use std::str::FromStr;

pub use crate::inv;
pub use quality::IntervalQuality::{self, *};
pub use size::IntervalSize::{self, *};

pub mod macros;
pub mod quality;
pub mod size;
mod utils;

/// A musical interval
/// The interval is represented by the number of semitones
/// between two notes.
///
/// The interval size is the size of the interval from unison
/// up to a seventh. If the interval is an octave or more,
/// the size will be the size of the interval modulo 7
/// (e.g. a ninth will be a second, a tenth will be a third, etc.)
///
/// The interval quality is the quality of the interval, which
/// can be diminished, minor, major, perfect or augmented.
/// Diminished and augmented take a number to represent
/// the number of diminished or augmented intervals. For example,
/// `Diminished(2)` would be a double diminished interval.
///
/// The number of octaves is the number of octaves of the interval.
/// For example, a ninth would have one octave.
///
/// A macro is provided to make creating intervals easier:
/// inv!(quality size)
/// inv!(quality size octaves)
/// inv!(string)
///
/// ### Examples
/// ```
/// use resonata::intervals::*;
///
/// let major_third = inv!(Major, Third).unwrap();
/// assert_eq!(major_third.to_semitones(), 4);
///
/// let minor_sixth = inv!(Minor, Sixth).unwrap();
/// assert_eq!(minor_sixth.to_semitones(), 8);
///
/// let augmented_octave = inv!(Augmented(1), Unison, 1).unwrap();
/// assert_eq!(augmented_octave.to_semitones(), 13);
///
/// let augmented_ninth = inv!(Augmented(1), Second, 1).unwrap();
/// assert_eq!(augmented_ninth.to_semitones(), 15);
/// ```
#[derive(Clone, Copy)]
pub struct Interval {
    quality: IntervalQuality,
    size: IntervalSize,
    octaves: u8,
}

impl Interval {
    /// Creates a new interval from an interval quality, size and number of octaves.
    /// Returns `None` if the interval is invalid.
    /// 
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    /// 
    /// let major_third = inv!(Major, Third).unwrap();
    /// assert_eq!(major_third, Interval::build(Major, Third, 0).unwrap());
    /// 
    /// let augmented_octave = inv!(Augmented(1), Unison, 1).unwrap();
    /// assert_eq!(augmented_octave, Interval::build(Augmented(1), Unison, 1).unwrap());
    /// 
    /// let invalid_interval = Interval::build(Major, Fifth, 0);
    /// assert!(invalid_interval.is_none());
    /// ```
    pub fn build(quality: IntervalQuality, size: IntervalSize, octaves: u8) -> Option<Self> {
        match quality {
            Major | Minor => match size {
                Unison | Fourth | Fifth => return None,
                _ => {}
            },
            Perfect => match size {
                Second | Third | Sixth | Seventh => return None,
                _ => {}
            },
            _ => {}
        }

        let interval = Interval {
            quality,
            size,
            octaves,
        };

        match interval.to_semitones() {
            -127..=127 => Some(interval),
            _ => None,
        }
    }

    pub fn with_octaves(&self, octaves: u8) -> Option<Self> {
        Interval::build(self.quality, self.size, octaves)
    }
    
    /// Returns an interval from the given string, if possible.
    /// 
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    /// 
    /// let major_third = inv!(Major, Third).unwrap();
    /// assert_eq!(major_third, Interval::from_string("M3").unwrap());
    /// 
    /// let augmented_octave = inv!(Augmented(1), Unison, 1).unwrap();
    /// assert_eq!(augmented_octave, Interval::from_string("A8").unwrap());
    /// 
    /// let invalid_interval = Interval::from_string("P3");
    /// assert!(invalid_interval.is_none());
    /// ```
    pub fn from_string(s: &str) -> Option<Self> {
        match Interval::from_str(s) {
            Ok(interval) => Some(interval),
            Err(_) => None,
        }
    }

    /// Returns an interval from the given number of semitones.
    /// 
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    /// 
    /// let major_third = inv!(Major, Third).unwrap();
    /// assert_eq!(major_third, Interval::from_semitones(4).unwrap());
    /// 
    /// let augmented_octave = inv!(Augmented(1), Unison, 1).unwrap();
    /// assert_eq!(augmented_octave, Interval::from_semitones(13).unwrap());
    /// 
    /// let invalid_interval = Interval::from_semitones(128);
    /// assert!(invalid_interval.is_none());
    /// ```
    pub fn from_semitones(value: i32) -> Option<Self> {
        let (quality, size) = match (value % 12, value.signum()) {
            (0, _) => (Perfect, Unison),
            (1, 1) => (Minor, Second),
            (2, _) => (Major, Second),
            (3, 1) => (Minor, Third),
            (4, 1) => (Major, Third),
            (5, 1) => (Perfect, Fourth),
            (6, 1) => (Augmented(1), Fourth),
            (7, _) => (Perfect, Fifth),
            (8, 1) => (Minor, Sixth),
            (9, _) => (Major, Sixth),
            (10, 1) => (Minor, Seventh),
            (11, _) => (Major, Seventh),
            (1, -1) => (Augmented(1), Unison),
            (3, -1) => (Augmented(1), Second),
            (4, -1) => (Diminished(1), Fourth),
            (5, -1) => (Augmented(1), Third),
            (6, -1) => (Diminished(1), Fifth),
            (8, -1) => (Augmented(1), Fifth),
            (10, -1) => (Augmented(1), Sixth),
            _ => unreachable!(),
        };

        let octaves = (value / 12) as u8;

        Interval::build(quality, size, octaves)
    }

    /// Returns the number of semitones of the interval
    /// 
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    /// 
    /// let major_third = inv!(Major, Third).unwrap();
    /// assert_eq!(major_third.to_semitones(), 4);
    /// ```
    pub fn to_semitones(&self) -> i32 {
        let value = self.size.to_semitones() as i32 + self.get_quality_offset();
        value + self.octaves as i32 * 12
    }

    /// Returns the semitone offset of the interval quality.
    /// 
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    /// 
    /// let major_third = inv!(Major, Third).unwrap();
    /// assert_eq!(major_third.get_quality_offset(), 0);
    /// 
    /// let minor_third = inv!(Minor, Third).unwrap();
    /// assert_eq!(minor_third.get_quality_offset(), -1);
    /// 
    /// let diminished_third = inv!(Diminished(1), Third).unwrap();
    /// assert_eq!(diminished_third.get_quality_offset(), -2);
    /// 
    /// let augmented_second = inv!(Augmented(1), Second).unwrap();
    /// assert_eq!(augmented_second.get_quality_offset(), 1);
    /// ```
    pub fn get_quality_offset(&self) -> i32 {
        match self.quality {
            Perfect | Major => 0,
            Minor => -1,
            Augmented(n) => n as i32,
            Diminished(n) => match self.size {
                Unison | Fourth | Fifth => -(n as i32),
                _ => -(n as i32 + 1),
            },
        }
    }

    /// Returns this interval with the given size. The resulting interval
    /// will be the closest interval to the original interval with the given size.
    /// The resulting interval will have the same number of semitones.
    /// 
    /// ### Failures
    /// Returns `None` if the resulting interval is invalid.
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    ///
    /// let perfect_unison = inv!(Perfect, Unison).unwrap();
    /// let diminished_second = perfect_unison.as_size(Second, 0).unwrap();
    /// assert_eq!(diminished_second, inv!(Diminished(1), Second).unwrap());
    ///
    /// let minor_third = inv!(Minor, Third).unwrap();
    /// let augmented_second = minor_third.as_size(Second, 0).unwrap();
    /// assert_eq!(augmented_second, inv!(Augmented(1), Second).unwrap());
    /// 
    /// let major_second = inv!(Major, Second).unwrap();
    /// let diminished_third = major_second.as_size(Third, 0).unwrap();
    /// assert_eq!(diminished_third, inv!(Diminished(1), Third).unwrap());
    ///
    /// let major_seventh = inv!(Major, Seventh).unwrap();
    /// let diminished_octave = major_seventh.as_size(Unison, 1).unwrap();
    /// assert_eq!(diminished_octave, inv!(Diminished(1), Unison, 1).unwrap());
    /// ```
    pub fn as_size(&self, size: IntervalSize, octaves: u8) -> Option<Self> {
        if size == self.size {
            return Some(*self);
        }

        let target_semitones = size.to_semitones() as i32 + 12 * octaves as i32;
        let diff = self.to_semitones() - target_semitones;

        let quality = match size {
            Unison | Fourth | Fifth => match diff {
                d if d > 0 => Augmented(d as u8),
                d if d < 0 => Diminished(d.abs() as u8),
                _ => self.quality,
            },
            _ => match diff {
                -1 => Minor,
                d if d > 0 => Augmented(d as u8),
                d if d < 0 => Diminished(d.abs() as u8 - 1),
                _ => self.quality,
            },
        };

        Interval::build(quality, size, octaves)
    }

    /// Inverts the interval. Resulting interval will retain
    /// the same octave number as the original interval.
    pub fn inverted(&self) -> Self {
        let size = self.size.invert();
        let quality = self.quality.invert();
        let octaves = self.octaves;

        Interval {
            quality,
            size,
            octaves,
        }
    }

    /// Returns the quality of the interval
    pub fn quality(&self) -> IntervalQuality {
        self.quality
    }

    /// Returns the size of the interval. This is the size of the interval
    /// from unison up to a seventh. If the interval is an octave or more,
    /// the size will be the size of the interval modulo 7. To get the total
    /// size of the interval, use `to_semitones()` or 'to_diatonic_steps()'.
    pub fn size(&self) -> IntervalSize {
        self.size
    }

    /// Returns the number of octaves of the interval
    pub fn octaves(&self) -> u8 {
        self.octaves
    }

    /// Returns the number of diatonic steps of the interval, starting from 0.
    /// 
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    /// 
    /// let interval = inv!(Major, Third).unwrap();
    /// assert_eq!(interval.to_diatonic_steps(), 2);
    /// 
    /// let interval = inv!(Minor, Second, 1).unwrap();
    /// assert_eq!(interval.to_diatonic_steps(), 8);
    /// ```
    pub fn to_diatonic_steps(&self) -> i32 {
        self.size as i32 + self.octaves as i32 * 7
    }
}
