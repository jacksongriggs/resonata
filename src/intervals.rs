use crate::{
    err,
    error::{IntervalError, ResonataError},
    nope,
};
use quality::IntervalQuality::*;
use size::IntervalSize::*;
use std::str::FromStr;

pub use crate::inv;
pub use quality::IntervalQuality;
pub use size::IntervalSize;

pub mod quality;
pub mod size;
mod tests;
mod utils;

type Result<T> = std::result::Result<T, ResonataError>;

pub trait Invert {
    fn invert(self) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub enum PerfectSize {
    Unison = 0,
    Fourth = 5,
    Fifth = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum ImperfectSize {
    Second = 2,
    Third = 4,
    Sixth = 9,
    Seventh = 11,
}

#[derive(Debug, Clone, Copy)]
pub enum IntervalSizeType {
    Perfect(PerfectSize),
    Imperfect(ImperfectSize),
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

    fn build(alteration_type: AlterationType, degree: u8) -> Result<Alteration> {
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

#[derive(Debug, Clone, Copy)]
pub enum SimpleInterval {
    Perfect(PerfectSize),
    Imperfect(ImperfectSize, ImperfectType),
    Altered(IntervalSizeType, Alteration),
}

impl SimpleInterval {
    fn to_semitones(&self) -> i8 {
        match self {
            SimpleInterval::Perfect(size) => *size as i8,
            SimpleInterval::Imperfect(size, imperfect_type) => match imperfect_type {
                ImperfectType::Major => *size as i8,
                ImperfectType::Minor => *size as i8 - 1, // Minor intervals are one semitone smaller than major intervals
            },
            SimpleInterval::Altered(size, alt) => {
                alt.degree()
                    + match size {
                        IntervalSizeType::Perfect(size) => *size as i8,
                        IntervalSizeType::Imperfect(size) => match alt.alteration_type {
                            AlterationType::Diminished => *size as i8 - 1, // Diminished intervals are two semitones smaller than major intervals
                            AlterationType::Augmented => *size as i8,
                        },
                    }
            }
        }
    }

    fn from_semitones(semitones: i32) -> Interval {
        match semitones.abs() % 12 {
            0 => SimpleInterval::Perfect(Unison.into()).into(),
            1 => SimpleInterval::Imperfect(Second.into(), Minor.into()).into(),
            2 => SimpleInterval::Imperfect(Second.into(), Major.into()).into(),
            3 => SimpleInterval::Imperfect(Third.into(), Minor.into()).into(),
            4 => SimpleInterval::Imperfect(Third.into(), Major.into()).into(),
            5 => SimpleInterval::Perfect(Fourth.into()).into(),
            6 => SimpleInterval::Altered(Fourth.into(), Alteration::augmented(1).unwrap()).into(),
            7 => SimpleInterval::Perfect(Fifth.into()).into(),
            8 => SimpleInterval::Imperfect(Sixth.into(), Minor.into()).into(),
            9 => SimpleInterval::Imperfect(Sixth.into(), Major.into()).into(),
            10 => SimpleInterval::Imperfect(Seventh.into(), Minor.into()).into(),
            11 => SimpleInterval::Imperfect(Seventh.into(), Major.into()).into(),
            _ => unreachable!(),
        }
    }

    fn quality(&self) -> IntervalQuality {
        match self {
            SimpleInterval::Perfect(_) => IntervalQuality::Perfect,
            SimpleInterval::Imperfect(_, imperfect_type) => match imperfect_type {
                ImperfectType::Major => IntervalQuality::Major,
                ImperfectType::Minor => IntervalQuality::Minor,
            },
            SimpleInterval::Altered(_, alteration) => match alteration.alteration_type {
                AlterationType::Diminished => IntervalQuality::Diminished(alteration.degree),
                AlterationType::Augmented => IntervalQuality::Augmented(alteration.degree),
            },
        }
    }

    fn size(&self) -> IntervalSize {
        match self {
            SimpleInterval::Perfect(size) => IntervalSize::from(*size),
            SimpleInterval::Imperfect(size, _) => IntervalSize::from(*size),
            SimpleInterval::Altered(size, _) => IntervalSize::from(*size),
        }
    }

    fn altered(self, amount: i8) -> Result<SimpleInterval> {
        if amount == 0 {
            return Ok(self);
        }

        match self {
            SimpleInterval::Perfect(size) => Self::alter_perfect(size, amount),
            SimpleInterval::Imperfect(size, maj_or_min) => match maj_or_min {
                ImperfectType::Major => Self::alter_imperfect(size, amount),
                ImperfectType::Minor => Self::alter_imperfect(size, amount - 1),
            },
            SimpleInterval::Altered(size, alt) => match size {
                IntervalSizeType::Perfect(size) => Self::alter_perfect(size, amount + alt.degree()),
                IntervalSizeType::Imperfect(size) => match alt.alteration_type {
                    AlterationType::Diminished => {
                        Self::alter_imperfect(size, amount + alt.degree() - 1)
                    }
                    AlterationType::Augmented => Self::alter_imperfect(size, amount + alt.degree()),
                },
            },
        }
    }

    fn alter_perfect(size: PerfectSize, amount: i8) -> Result<SimpleInterval> {
        if amount == 0 {
            Ok(Self::Perfect(size))
        } else if amount > 0 {
            Ok(Self::Altered(size.into(), Alteration::augmented(amount as u8)?))
        } else {
            Ok(Self::Altered(size.into(), Alteration::diminished(amount.abs() as u8)?))
        }
    }

    fn alter_imperfect(size: ImperfectSize, amount: i8) -> Result<SimpleInterval> {
        if amount == 0 {
            Ok(Self::Imperfect(size, ImperfectType::Major))
        } else if amount == -1 {
            Ok(Self::Imperfect(size, ImperfectType::Minor))
        } else if amount > 0 {
            Ok(Self::Altered(size.into(), Alteration::augmented(amount as u8)?))
        } else {
            Ok(Self::Altered(size.into(), Alteration::diminished(amount.abs() as u8 - 1)?))
        }
    }
}

pub struct PerfectBuilder;

impl PerfectBuilder {
    pub fn unison(self) -> Interval {
        self.build(PerfectSize::Unison)
    }

    pub fn fourth(self) -> Interval {
        self.build(PerfectSize::Fourth)
    }

    pub fn fifth(self) -> Interval {
        self.build(PerfectSize::Fifth)
    }

    fn build(self, size: impl Into<PerfectSize>) -> Interval {
        SimpleInterval::Perfect(size.into()).into()
    }
}

pub struct ImperfectBuilder {
    imperfect_type: ImperfectType,
}

impl ImperfectBuilder {
    pub fn second(self) -> Interval {
        self.build(ImperfectSize::Second)
    }

    pub fn third(self) -> Interval {
        self.build(ImperfectSize::Third)
    }

    pub fn sixth(self) -> Interval {
        self.build(ImperfectSize::Sixth)
    }

    pub fn seventh(self) -> Interval {
        self.build(ImperfectSize::Seventh)
    }

    fn build(self, size: impl Into<ImperfectSize>) -> Interval {
        SimpleInterval::Imperfect(size.into(), self.imperfect_type).into()
    }
}

pub struct AlteredBuilder {
    alteration_type: AlterationType,
    degree: u8,
}

impl AlteredBuilder {
    pub fn unison(self) -> Result<Interval> {
        self.build(PerfectSize::Unison)
    }

    pub fn second(self) -> Result<Interval> {
        self.build(ImperfectSize::Second)
    }

    pub fn third(self) -> Result<Interval> {
        self.build(ImperfectSize::Third)
    }

    pub fn fourth(self) -> Result<Interval> {
        self.build(PerfectSize::Fourth)
    }

    pub fn fifth(self) -> Result<Interval> {
        self.build(PerfectSize::Fifth)
    }

    pub fn sixth(self) -> Result<Interval> {
        self.build(ImperfectSize::Sixth)
    }

    pub fn seventh(self) -> Result<Interval> {
        self.build(ImperfectSize::Seventh)
    }

    fn build(self, interval: impl Into<IntervalSizeType>) -> Result<Interval> {
        Alteration::build(self.alteration_type, self.degree)
            .map(|alt| SimpleInterval::Altered(interval.into(), alt).into())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CompoundInterval {
    base_interval: SimpleInterval,
    octaves: u8,
}

/// A musical interval.
///
/// Intervals are represented by a quality, size and number of octaves.
///
/// Intervals can be simple or compound. Simple intervals are intervals that are less than an octave.
/// Compound intervals are intervals that are greater than an octave.
///
/// Intervals can be built using the Interval::build() method, or using the inv! macro.
/// Intervals can also be created using the builder methods.
///
/// ### Examples
/// ```
/// use resonata::intervals::*;
///
/// let interval = Interval::build(IntervalQuality::Major, IntervalSize::Third, 0).unwrap();
/// assert_eq!(interval.semitones(), 4);
/// assert_eq!(interval.quality(), IntervalQuality::Major);
/// assert_eq!(interval.size(), IntervalSize::Third);
/// assert_eq!(interval.octaves(), 0);
///
/// let interval = inv!("P4").unwrap();
/// assert_eq!(interval.semitones(), 5);
/// assert_eq!(interval.quality(), IntervalQuality::Perfect);
/// assert_eq!(interval.size(), IntervalSize::Fourth);
/// assert_eq!(interval.octaves(), 0);
///
/// let interval = Interval::augmented(1).sixth().unwrap();
/// assert_eq!(interval.semitones(), 10);
/// assert_eq!(interval.quality(), IntervalQuality::Augmented(1));
/// assert_eq!(interval.size(), IntervalSize::Sixth);
/// assert_eq!(interval.octaves(), 0);
///
/// let interval = Interval::major().second().compound(1).unwrap();
/// assert_eq!(interval.semitones(), 14);
/// assert_eq!(interval.quality(), IntervalQuality::Major);
/// assert_eq!(interval.size(), IntervalSize::Second);
/// assert_eq!(interval.octaves(), 1);
/// ```
#[derive(Clone, Copy)]
pub enum Interval {
    Simple(SimpleInterval),
    Compound(CompoundInterval),
}

/// A macro for creating intervals.
#[macro_export]
macro_rules! inv {
    ($s:literal) => {
        Interval::from_string($s)
    };
    ($quality:expr, $size:expr) => {
        Interval::build($quality, $size, 0)
    };
    ($quality:expr, $size:expr, $octaves:literal) => {
        Interval::build($quality, $size, $octaves)
    };
}

impl Interval {
    /// Returns whether the given size and quality are a valid interval.
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    ///
    /// assert!(Interval::is_valid_interval(IntervalQuality::Major, IntervalSize::Third));
    /// assert!(!Interval::is_valid_interval(IntervalQuality::Perfect, IntervalSize::Sixth));
    /// ```
    pub fn is_valid_interval(quality: IntervalQuality, size: IntervalSize) -> bool {
        match size {
            Unison | Fourth | Fifth => match quality {
                Major | Minor => false,
                _ => true,
            },
            Second | Third | Sixth | Seventh => match quality {
                Perfect => false,
                _ => true,
            },
        }
    }

    /// Returns a perfect unison interval.
    pub fn unison() -> Interval {
        PerfectBuilder.unison()
    }

    /// Returns a major second interval.
    pub fn second() -> Interval {
        ImperfectBuilder { imperfect_type: ImperfectType::Major }.second()
    }

    /// Returns a major third interval.
    pub fn third() -> Interval {
        ImperfectBuilder { imperfect_type: ImperfectType::Major }.third()
    }

    /// Returns a perfect fourth interval.
    pub fn fourth() -> Interval {
        PerfectBuilder.fourth()
    }

    /// Returns a perfect fifth interval.
    pub fn fifth() -> Interval {
        PerfectBuilder.fifth()
    }

    /// Returns a major sixth interval.
    pub fn sixth() -> Interval {
        ImperfectBuilder { imperfect_type: ImperfectType::Major }.sixth()
    }

    /// Returns a major seventh interval.
    pub fn seventh() -> Interval {
        ImperfectBuilder { imperfect_type: ImperfectType::Major }.seventh()
    }

    /// Returns a major interval builder.
    pub fn major() -> ImperfectBuilder {
        ImperfectBuilder { imperfect_type: ImperfectType::Major }
    }

    /// Returns a minor interval builder.
    pub fn minor() -> ImperfectBuilder {
        ImperfectBuilder { imperfect_type: ImperfectType::Minor }
    }

    /// Returns a perfect interval builder.
    pub fn perfect() -> PerfectBuilder {
        PerfectBuilder
    }

    /// Returns a diminished interval builder.
    pub fn diminished(degree: u8) -> AlteredBuilder {
        AlteredBuilder { alteration_type: AlterationType::Diminished, degree }
    }

    /// Returns an augmented interval builder.
    pub fn augmented(degree: u8) -> AlteredBuilder {
        AlteredBuilder { alteration_type: AlterationType::Augmented, degree }
    }

    /// Raises the interval by the given number of semitones.
    /// The intervals size is not changed.
    ///
    /// i.e.
    /// Unison | Fourth | Fifth: Dim -> Perfect -> Aug
    /// Second | Third | Sixth | Seventh: Dim -> Minor -> Major -> Aug
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    ///
    /// let interval = Interval::minor().third().raised(2).unwrap();
    /// assert_eq!(interval.semitones(), 5);
    /// assert_eq!(interval.quality(), IntervalQuality::Augmented(1));
    ///
    /// let interval = Interval::major().sixth().raised(1).unwrap();
    /// assert_eq!(interval.semitones(), 10);
    /// assert_eq!(interval.quality(), IntervalQuality::Augmented(1));
    ///
    /// let interval = Interval::perfect().fifth().raised(2).unwrap();
    /// assert_eq!(interval.semitones(), 9);
    /// assert_eq!(interval.quality(), IntervalQuality::Augmented(2));
    ///
    /// let interval = Interval::diminished(1).seventh().unwrap().raised(1).unwrap();
    /// assert_eq!(interval.semitones(), 10);
    /// assert_eq!(interval.quality(), IntervalQuality::Minor);
    ///
    /// let interval = Interval::augmented(1).second().unwrap();
    /// assert_eq!(interval.semitones(), 3);
    /// assert_eq!(interval.quality(), IntervalQuality::Augmented(1));
    ///
    /// let interval = interval.raised(1).unwrap();
    /// assert_eq!(interval.semitones(), 4);
    /// assert_eq!(interval.quality(), IntervalQuality::Augmented(2));
    /// ```
    pub fn raised(self, semitones: u8) -> Result<Self> {
        let interval = match self {
            Interval::Simple(interval) => interval,
            Interval::Compound(interval) => interval.base_interval,
        };

        interval.altered(semitones as i8).map(|i| i.into())
    }

    /// Lowers the interval by the given number of semitones.
    /// The intervals size is not changed.
    ///
    /// i.e.
    /// Unison | Fourth | Fifth: Aug -> Perfect -> Dim
    /// Second | Third | Sixth | Seventh: Aug -> Major -> Minor -> Dim
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    ///
    /// let interval = Interval::perfect().fourth().lowered(1).unwrap();
    /// assert_eq!(interval.semitones(), 4);
    /// assert_eq!(interval.quality(), IntervalQuality::Diminished(1));
    ///
    /// let interval = Interval::major().third().lowered(2).unwrap();
    /// assert_eq!(interval.semitones(), 2);
    /// assert_eq!(interval.quality(), IntervalQuality::Diminished(1));
    ///
    /// let interval = Interval::minor().sixth().lowered(1).unwrap();
    /// assert_eq!(interval.semitones(), 7);
    /// assert_eq!(interval.quality(), IntervalQuality::Diminished(1));
    ///
    /// let interval = Interval::augmented(1).seventh().unwrap().lowered(1).unwrap();
    /// assert_eq!(interval.semitones(), 11);
    /// assert_eq!(interval.quality(), IntervalQuality::Major);
    ///
    /// let interval = Interval::diminished(1).third().unwrap().lowered(1).unwrap();
    /// assert_eq!(interval.semitones(), 1);
    /// assert_eq!(interval.quality(), IntervalQuality::Diminished(2));
    /// ```
    pub fn lowered(self, semitones: u8) -> Result<Self> {
        let interval = match self {
            Interval::Simple(interval) => interval,
            Interval::Compound(interval) => interval.base_interval,
        };

        interval.altered(-(semitones as i8)).map(|i| i.into())
    }

    /// Returns the number of semitones in the interval.
    /// The number of semitones is the distance between the two notes.
    /// For example, a major third has 4 semitones.
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    ///
    /// let interval = Interval::major().third();
    /// assert_eq!(interval.semitones(), 4);
    ///
    /// let interval = Interval::minor().sixth();
    /// assert_eq!(interval.semitones(), 8);
    ///
    /// let interval = Interval::perfect().fifth().compound(1).unwrap();
    /// assert_eq!(interval.semitones(), 19);
    ///
    /// let interval = Interval::diminished(1).seventh().unwrap();
    /// assert_eq!(interval.semitones(), 9);
    ///
    /// let interval = Interval::augmented(1).second().unwrap();
    /// assert_eq!(interval.semitones(), 3);
    /// ```
    pub fn semitones(&self) -> i8 {
        match self {
            Interval::Simple(interval) => interval.to_semitones(),
            Interval::Compound(interval) => {
                interval.base_interval.to_semitones() + (interval.octaves * 12) as i8
            }
        }
    }

    /// Returns a compound interval from this interval.
    /// A compound interval is an interval that is greater than an octave.
    /// For example, a major tenth is a major third with one octave added.
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    ///
    /// let interval = Interval::major().third().compound(1).unwrap();
    /// assert_eq!(interval.semitones(), 16);
    ///
    /// let interval = Interval::minor().sixth().compound(2).unwrap();
    /// assert_eq!(interval.semitones(), 32);
    ///
    /// let interval = Interval::diminished(1).unison().unwrap().compound(1).unwrap();
    /// assert_eq!(interval.semitones(), 11);
    /// ```
    pub fn compound(self, octaves: u8) -> Result<Self> {
        let semitones = self.semitones() as i16 + (octaves * 12) as i16;
        if semitones > 127 || semitones < -128 {
            err!(IntervalError::InvalidIntervalSize(semitones))
        } else {
            Ok(CompoundInterval { base_interval: self.into(), octaves }.into())
        }
    }

    /// Returns the interval quality.
    pub fn quality(&self) -> IntervalQuality {
        match self {
            Interval::Simple(interval) => interval.quality(),
            Interval::Compound(interval) => interval.base_interval.quality(),
        }
    }

    /// Returns the size of the interval, ignoring the octaves if the interval is compound.
    pub fn size(&self) -> IntervalSize {
        match self {
            Interval::Simple(interval) => interval.size(),
            Interval::Compound(interval) => interval.base_interval.size(),
        }
    }

    /// Builds an interval from the given quality, size and number of octaves.
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    ///
    /// let interval = Interval::build(IntervalQuality::Major, IntervalSize::Third, 1).unwrap();
    /// assert_eq!(interval.semitones(), 16);
    ///
    /// let interval = Interval::build(IntervalQuality::Minor, IntervalSize::Sixth, 2).unwrap();
    /// assert_eq!(interval.semitones(), 32);
    ///
    /// let interval = Interval::build(IntervalQuality::Diminished(1), IntervalSize::Unison, 1).unwrap();
    /// assert_eq!(interval.semitones(), 11);
    /// ```
    pub fn build(quality: IntervalQuality, size: IntervalSize, octaves: u8) -> Result<Self> {
        if !Interval::is_valid_interval(quality, size) {
            nope!(IntervalError::InvalidIntervalQualityAndSize(quality, size));
        }

        match quality {
            IntervalQuality::Diminished(n) => {
                Interval::diminished(n).build(size)?.compound(octaves)
            }
            IntervalQuality::Augmented(n) => Interval::augmented(n).build(size)?.compound(octaves),
            IntervalQuality::Minor => Interval::minor().build(size).compound(octaves),
            IntervalQuality::Major => Interval::major().build(size).compound(octaves),
            IntervalQuality::Perfect => Interval::perfect().build(size).compound(octaves),
        }
    }

    pub fn with_octaves(&self, octaves: u8) -> Result<Self> {
        self.compound(octaves)
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
    /// assert!(invalid_interval.is_err());
    /// ```
    pub fn from_string(s: &str) -> Result<Self> {
        Interval::from_str(s)
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
    /// assert!(invalid_interval.is_err());
    /// ```
    pub fn from_semitones(semitones: i32) -> Result<Self> {
        let octaves = semitones / 12;
        SimpleInterval::from_semitones(semitones % 12).compound(octaves as u8)
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
        let value = self.size().to_semitones() as i32 + self.get_quality_offset();
        value + self.octaves() as i32 * 12
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
        match self.quality() {
            Perfect | Major => 0,
            Minor => -1,
            Augmented(n) => n as i32,
            Diminished(n) => match self.size() {
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
    pub fn as_size(&self, size: IntervalSize, octaves: u8) -> Result<Self> {
        if size == self.size() {
            return Ok(*self);
        }

        let target_semitones = size.to_semitones() as i32 + 12 * octaves as i32;
        let diff = self.to_semitones() - target_semitones;

        let quality = match size {
            Unison | Fourth | Fifth => match diff {
                d if d > 0 => Augmented(d as u8),
                d if d < 0 => Diminished(d.abs() as u8),
                _ => self.quality(),
            },
            _ => match diff {
                -1 => Minor,
                d if d > 0 => Augmented(d as u8),
                d if d < 0 => Diminished(d.abs() as u8 - 1),
                _ => self.quality(),
            },
        };

        Interval::build(quality, size, octaves)
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
        self.size() as i32 + self.octaves() as i32 * 7
    }

    pub fn octaves(&self) -> u8 {
        match self {
            Interval::Simple(_) => 0,
            Interval::Compound(interval) => interval.octaves,
        }
    }
}
