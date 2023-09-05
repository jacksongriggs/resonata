use crate::{
    err,
    error::{IntervalError, ResonataError},
    nope,
};

pub use crate::inv;
pub use size::*;
pub use quality::*;

pub mod size;
pub mod quality;
mod tests;
mod utils;

type Result<T> = std::result::Result<T, ResonataError>;

pub trait Invert {
    fn invert(self) -> Self;
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
                        IntervalSizeType::Imperfect(size) => match alt.alteration_type() {
                            AlterationType::Diminished => *size as i8 - 1, // Diminished intervals are two semitones smaller than major intervals
                            AlterationType::Augmented => *size as i8,
                        },
                    }
            }
        }
    }

    fn from_semitones(semitones: i32) -> Interval {
        match semitones.abs() % 12 {
            0 => SimpleInterval::Perfect(Size::Unison.into()).into(),
            1 => SimpleInterval::Imperfect(Size::Second.into(), Quality::Minor.into()).into(),
            2 => SimpleInterval::Imperfect(Size::Second.into(), Quality::Major.into()).into(),
            3 => SimpleInterval::Imperfect(Size::Third.into(), Quality::Minor.into()).into(),
            4 => SimpleInterval::Imperfect(Size::Third.into(), Quality::Major.into()).into(),
            5 => SimpleInterval::Perfect(Size::Fourth.into()).into(),
            6 => SimpleInterval::Altered(Size::Fourth.into(), Alteration::augmented(1).unwrap()).into(),
            7 => SimpleInterval::Perfect(Size::Fifth.into()).into(),
            8 => SimpleInterval::Imperfect(Size::Sixth.into(), Quality::Minor.into()).into(),
            9 => SimpleInterval::Imperfect(Size::Sixth.into(), Quality::Major.into()).into(),
            10 => SimpleInterval::Imperfect(Size::Seventh.into(), Quality::Minor.into()).into(),
            11 => SimpleInterval::Imperfect(Size::Seventh.into(), Quality::Major.into()).into(),
            _ => unreachable!(),
        }
    }

    fn quality(&self) -> Quality {
        match self {
            SimpleInterval::Perfect(_) => Quality::Perfect,
            SimpleInterval::Imperfect(_, imperfect_type) => match imperfect_type {
                ImperfectType::Major => Quality::Major,
                ImperfectType::Minor => Quality::Minor,
            },
            SimpleInterval::Altered(_, alteration) => match alteration.alteration_type() {
                AlterationType::Diminished => Quality::Diminished(alteration.degree().abs() as u8),
                AlterationType::Augmented => Quality::Augmented(alteration.degree().abs() as u8),
            },
        }
    }

    fn size(&self) -> Size {
        match self {
            SimpleInterval::Perfect(size) => Size::from(*size),
            SimpleInterval::Imperfect(size, _) => Size::from(*size),
            SimpleInterval::Altered(size, _) => Size::from(*size),
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
                IntervalSizeType::Imperfect(size) => match alt.alteration_type() {
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
/// let interval = Interval::build(Quality::Major, Size::Third, 0).unwrap();
/// assert_eq!(interval.semitones(), 4);
/// assert_eq!(interval.quality(), Quality::Major);
/// assert_eq!(interval.size(), Size::Third);
/// assert_eq!(interval.octaves(), 0);
///
/// let interval = inv!("P4").unwrap();
/// assert_eq!(interval.semitones(), 5);
/// assert_eq!(interval.quality(), Quality::Perfect);
/// assert_eq!(interval.size(), Size::Fourth);
/// assert_eq!(interval.octaves(), 0);
///
/// let interval = Interval::augmented(1).sixth().unwrap();
/// assert_eq!(interval.semitones(), 10);
/// assert_eq!(interval.quality(), Quality::Augmented(1));
/// assert_eq!(interval.size(), Size::Sixth);
/// assert_eq!(interval.octaves(), 0);
///
/// let interval = Interval::major().second().compound(1).unwrap();
/// assert_eq!(interval.semitones(), 14);
/// assert_eq!(interval.quality(), Quality::Major);
/// assert_eq!(interval.size(), Size::Second);
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
        $s.parse::<Interval>()
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
    /// assert!(Interval::is_valid_interval(Quality::Major, Size::Third));
    /// assert!(!Interval::is_valid_interval(Quality::Perfect, Size::Sixth));
    /// ```
    pub fn is_valid_interval(quality: Quality, size: Size) -> bool {
        match size {
            Size::Unison | Size::Fourth | Size::Fifth => match quality {
                Quality::Major | Quality::Minor => false,
                _ => true,
            },
            Size::Second | Size::Third | Size::Sixth | Size::Seventh => match quality {
                Quality::Perfect => false,
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
    /// assert_eq!(interval.quality(), Quality::Augmented(1));
    ///
    /// let interval = Interval::major().sixth().raised(1).unwrap();
    /// assert_eq!(interval.semitones(), 10);
    /// assert_eq!(interval.quality(), Quality::Augmented(1));
    ///
    /// let interval = Interval::perfect().fifth().raised(2).unwrap();
    /// assert_eq!(interval.semitones(), 9);
    /// assert_eq!(interval.quality(), Quality::Augmented(2));
    ///
    /// let interval = Interval::diminished(1).seventh().unwrap().raised(1).unwrap();
    /// assert_eq!(interval.semitones(), 10);
    /// assert_eq!(interval.quality(), Quality::Minor);
    ///
    /// let interval = Interval::augmented(1).second().unwrap();
    /// assert_eq!(interval.semitones(), 3);
    /// assert_eq!(interval.quality(), Quality::Augmented(1));
    ///
    /// let interval = interval.raised(1).unwrap();
    /// assert_eq!(interval.semitones(), 4);
    /// assert_eq!(interval.quality(), Quality::Augmented(2));
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
    /// assert_eq!(interval.quality(), Quality::Diminished(1));
    ///
    /// let interval = Interval::major().third().lowered(2).unwrap();
    /// assert_eq!(interval.semitones(), 2);
    /// assert_eq!(interval.quality(), Quality::Diminished(1));
    ///
    /// let interval = Interval::minor().sixth().lowered(1).unwrap();
    /// assert_eq!(interval.semitones(), 7);
    /// assert_eq!(interval.quality(), Quality::Diminished(1));
    ///
    /// let interval = Interval::augmented(1).seventh().unwrap().lowered(1).unwrap();
    /// assert_eq!(interval.semitones(), 11);
    /// assert_eq!(interval.quality(), Quality::Major);
    ///
    /// let interval = Interval::diminished(1).third().unwrap().lowered(1).unwrap();
    /// assert_eq!(interval.semitones(), 1);
    /// assert_eq!(interval.quality(), Quality::Diminished(2));
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
    pub fn quality(&self) -> Quality {
        match self {
            Interval::Simple(interval) => interval.quality(),
            Interval::Compound(interval) => interval.base_interval.quality(),
        }
    }

    /// Returns the size of the interval, ignoring the octaves if the interval is compound.
    pub fn size(&self) -> Size {
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
    /// let interval = Interval::build(Quality::Major, Size::Third, 1).unwrap();
    /// assert_eq!(interval.semitones(), 16);
    ///
    /// let interval = Interval::build(Quality::Minor, Size::Sixth, 2).unwrap();
    /// assert_eq!(interval.semitones(), 32);
    ///
    /// let interval = Interval::build(Quality::Diminished(1), Size::Unison, 1).unwrap();
    /// assert_eq!(interval.semitones(), 11);
    /// ```
    pub fn build(quality: Quality, size: Size, octaves: u8) -> Result<Self> {
        if !Interval::is_valid_interval(quality, size) {
            nope!(IntervalError::InvalidIntervalQualityAndSize(quality, size));
        }

        match quality {
            Quality::Diminished(n) => {
                Interval::diminished(n).build(size)?.compound(octaves)
            }
            Quality::Augmented(n) => Interval::augmented(n).build(size)?.compound(octaves),
            Quality::Minor => Interval::minor().build(size).compound(octaves),
            Quality::Major => Interval::major().build(size).compound(octaves),
            Quality::Perfect => Interval::perfect().build(size).compound(octaves),
        }
    }

    pub fn with_octaves(&self, octaves: u8) -> Result<Self> {
        self.compound(octaves)
    }

    /// Returns an interval from the given number of semitones.
    ///
    /// ### Examples
    /// ```
    /// use resonata::intervals::*;
    ///
    /// let major_third = inv!("M3").unwrap();
    /// assert_eq!(major_third, Interval::from_semitones(4).unwrap());
    ///
    /// let augmented_octave = inv!("A8").unwrap();
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
    /// let major_third = inv!("M3").unwrap();
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
    /// let major_third = inv!("M3").unwrap();
    /// assert_eq!(major_third.get_quality_offset(), 0);
    ///
    /// let minor_third = inv!("m3").unwrap();
    /// assert_eq!(minor_third.get_quality_offset(), -1);
    ///
    /// let diminished_third = inv!("d3").unwrap();
    /// assert_eq!(diminished_third.get_quality_offset(), -2);
    ///
    /// let augmented_second = inv!("A2").unwrap();
    /// assert_eq!(augmented_second.get_quality_offset(), 1);
    /// ```
    pub fn get_quality_offset(&self) -> i32 {
        match self.quality() {
            Quality::Perfect | Quality::Major => 0,
            Quality::Minor => -1,
            Quality::Augmented(n) => n as i32,
            Quality::Diminished(n) => match self.size() {
                Size::Unison | Size::Fourth | Size::Fifth => -(n as i32),
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
    /// let perfect_unison = inv!("PU").unwrap();
    /// let diminished_second = perfect_unison.as_size(Size::Second, 0).unwrap();
    /// assert_eq!(diminished_second, inv!("d2").unwrap());
    ///
    /// let minor_third = inv!("m3").unwrap();
    /// let augmented_second = minor_third.as_size(Size::Second, 0).unwrap();
    /// assert_eq!(augmented_second, inv!("A2").unwrap());
    ///
    /// let major_second = inv!("M2").unwrap();
    /// let diminished_third = major_second.as_size(Size::Third, 0).unwrap();
    /// assert_eq!(diminished_third, inv!("d3").unwrap());
    ///
    /// let major_seventh = inv!("M7").unwrap();
    /// let diminished_octave = major_seventh.as_size(Size::Unison, 1).unwrap();
    /// assert_eq!(diminished_octave, inv!("d8").unwrap());
    /// ```
    pub fn as_size(&self, size: Size, octaves: u8) -> Result<Self> {
        if size == self.size() {
            return Ok(*self);
        }

        let target_semitones = size.to_semitones() as i32 + 12 * octaves as i32;
        let diff = self.to_semitones() - target_semitones;

        let quality = match size {
            Size::Unison | Size::Fourth | Size::Fifth => match diff {
                d if d > 0 => Quality::Augmented(d as u8),
                d if d < 0 => Quality::Diminished(d.abs() as u8),
                _ => self.quality(),
            },
            _ => match diff {
                -1 => Quality::Minor,
                d if d > 0 => Quality::Augmented(d as u8),
                d if d < 0 => Quality::Diminished(d.abs() as u8 - 1),
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
    /// let interval = inv!("M3").unwrap();
    /// assert_eq!(interval.to_diatonic_steps(), 2);
    ///
    /// let interval = inv!("m9").unwrap();
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
