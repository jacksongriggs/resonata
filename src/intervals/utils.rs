use crate::{error::*, intervals::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

impl FromStr for Interval {
    type Err = ResonataError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // Variables to store the parsed quality and size
        let mut quality_string = String::new();
        let mut size_string = s.to_string();

        // Split string into quality and size.
        // Handle the cases where quality is 'A', 'd', 'M', 'm' or 'P' characters
        // Iterate over the string from the start and keep adding the characters
        // to the quality until a numeric digit is encountered.
        for c in s.chars() {
            match c {
                'A' | 'd' | 'M' | 'm' | 'P' => {
                    quality_string.push(c);
                    size_string = size_string.split_off(1);
                }
                _ => break,
            }
        }

        // If the quality string is empty, then parse the size string as semitones
        if quality_string.is_empty() {
            let semitones = match size_string.parse::<i32>() {
                Ok(n) => n,
                Err(n) => nope!(InvalidIntervalFormat),
            };

            return Interval::from_semitones(semitones);
        }

        // Parse quality from the quality string
        let quality = IntervalQuality::from_str(&quality_string)?;

        // Parse size from the size string:
        let mut octaves: u8 = 0;
        let size = match size_string.as_str() {
            "U" => IntervalSize::Unison,
            _ => match size_string.parse::<u8>() {
                Ok(n) => match n {
                    0 => nope!(InvalidIntervalSize(0)),
                    _ => {
                        let n = n - 1;
                        if n >= 7 {
                            octaves = n / 7;
                        }
                        IntervalSize::from(n % 7)
                    }
                },
                Err(_) => nope!(InvalidIntervalFormat),
            },
        };

        Interval::build(quality, size, octaves)
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let quality = self.quality().to_string();
        let size = self.size() as u8 + 1 + (self.octaves() * 7);
        write!(f, "{}{}", quality, size)
    }
}

impl Debug for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Add for Interval {
    type Output = Result<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        Interval::from_semitones(self.to_semitones() + rhs.to_semitones())
    }
}

impl Sub for Interval {
    type Output = Result<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        Interval::from_semitones(self.to_semitones() - rhs.to_semitones())
    }
}

impl AddAssign for Interval {
    fn add_assign(&mut self, rhs: Self) {
        match *self + rhs {
            Ok(interval) => *self = interval,
            Err(err) => panic!("Interval addition overflowed: {}", err),
        }
    }
}

impl SubAssign for Interval {
    fn sub_assign(&mut self, rhs: Self) {
        match *self - rhs {
            Ok(interval) => *self = interval,
            Err(err) => panic!("Interval subtraction overflowed: {}", err),
        }
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.to_semitones() == other.to_semitones()
    }
}

impl Eq for Interval {}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_semitones().cmp(&other.to_semitones()))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_semitones().cmp(&other.to_semitones())
    }
}

impl Invert for PerfectSize {
    fn invert(self) -> Self {
        match self {
            PerfectSize::Unison => PerfectSize::Unison,
            PerfectSize::Fourth => PerfectSize::Fifth,
            PerfectSize::Fifth => PerfectSize::Fourth,
        }
    }
}

impl Invert for ImperfectSize {
    fn invert(self) -> Self {
        match self {
            ImperfectSize::Second => ImperfectSize::Seventh,
            ImperfectSize::Third => ImperfectSize::Sixth,
            ImperfectSize::Sixth => ImperfectSize::Third,
            ImperfectSize::Seventh => ImperfectSize::Second,
        }
    }
}

impl Invert for IntervalSizeType {
    fn invert(self) -> Self {
        match self {
            IntervalSizeType::Perfect(interval) => IntervalSizeType::Perfect(interval.invert()),
            IntervalSizeType::Imperfect(interval) => IntervalSizeType::Imperfect(interval.invert()),
        }
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

impl Invert for SimpleInterval {
    fn invert(self) -> Self {
        match self {
            SimpleInterval::Perfect(size) => SimpleInterval::Perfect(size.invert()),
            SimpleInterval::Imperfect(size, major_minor_type) => match major_minor_type {
                ImperfectType::Major => {
                    SimpleInterval::Imperfect(size.invert(), ImperfectType::Minor)
                }
                ImperfectType::Minor => {
                    SimpleInterval::Imperfect(size.invert(), ImperfectType::Major)
                }
            },
            SimpleInterval::Altered(interval_type, modification) => {
                SimpleInterval::Altered(interval_type.invert(), modification.invert())
            }
        }
    }
}

impl Invert for Interval {
    fn invert(self) -> Self {
        match self {
            Interval::Simple(interval) => Interval::Simple(interval.invert()),
            Interval::Compound(interval) => Interval::Simple(interval.base_interval.invert())
                .compound(interval.octaves)
                .unwrap(),
        }
    }
}

impl From<IntervalSize> for IntervalSizeType {
    fn from(size: IntervalSize) -> Self {
        match size {
            IntervalSize::Unison => IntervalSizeType::Perfect(PerfectSize::Unison),
            IntervalSize::Second => IntervalSizeType::Imperfect(ImperfectSize::Second),
            IntervalSize::Third => IntervalSizeType::Imperfect(ImperfectSize::Third),
            IntervalSize::Fourth => IntervalSizeType::Perfect(PerfectSize::Fourth),
            IntervalSize::Fifth => IntervalSizeType::Perfect(PerfectSize::Fifth),
            IntervalSize::Sixth => IntervalSizeType::Imperfect(ImperfectSize::Sixth),
            IntervalSize::Seventh => IntervalSizeType::Imperfect(ImperfectSize::Seventh),
        }
    }
}

impl From<IntervalSize> for PerfectSize {
    fn from(size: IntervalSize) -> Self {
        match size {
            IntervalSize::Unison => PerfectSize::Unison,
            IntervalSize::Fourth => PerfectSize::Fourth,
            IntervalSize::Fifth => PerfectSize::Fifth,
            _ => {
                println!("IntervalSize::from(IntervalSize) called with {:?}", size);
                unreachable!()
            }
        }
    }
}

impl From<PerfectSize> for IntervalSize {
    fn from(size: PerfectSize) -> Self {
        match size {
            PerfectSize::Unison => IntervalSize::Unison,
            PerfectSize::Fourth => IntervalSize::Fourth,
            PerfectSize::Fifth => IntervalSize::Fifth,
        }
    }
}

impl From<IntervalSize> for ImperfectSize {
    fn from(size: IntervalSize) -> Self {
        match size {
            IntervalSize::Second => ImperfectSize::Second,
            IntervalSize::Third => ImperfectSize::Third,
            IntervalSize::Sixth => ImperfectSize::Sixth,
            IntervalSize::Seventh => ImperfectSize::Seventh,
            _ => unreachable!(),
        }
    }
}

impl From<ImperfectSize> for IntervalSize {
    fn from(size: ImperfectSize) -> Self {
        match size {
            ImperfectSize::Second => IntervalSize::Second,
            ImperfectSize::Third => IntervalSize::Third,
            ImperfectSize::Sixth => IntervalSize::Sixth,
            ImperfectSize::Seventh => IntervalSize::Seventh,
        }
    }
}

impl From<IntervalSizeType> for IntervalSize {
    fn from(size: IntervalSizeType) -> Self {
        match size {
            IntervalSizeType::Perfect(size) => IntervalSize::from(size),
            IntervalSizeType::Imperfect(size) => IntervalSize::from(size),
        }
    }
}

impl From<PerfectSize> for IntervalSizeType {
    fn from(interval: PerfectSize) -> Self {
        IntervalSizeType::Perfect(interval)
    }
}

impl From<ImperfectSize> for IntervalSizeType {
    fn from(interval: ImperfectSize) -> Self {
        IntervalSizeType::Imperfect(interval)
    }
}

impl From<IntervalQuality> for ImperfectType {
    fn from(quality: IntervalQuality) -> Self {
        match quality {
            IntervalQuality::Major => ImperfectType::Major,
            IntervalQuality::Minor => ImperfectType::Minor,
            _ => unreachable!(),
        }
    }
}

impl From<IntervalQuality> for AlterationType {
    fn from(quality: IntervalQuality) -> Self {
        match quality {
            IntervalQuality::Augmented(_) => AlterationType::Augmented,
            IntervalQuality::Diminished(_) => AlterationType::Diminished,
            _ => unreachable!(),
        }
    }
}

impl From<IntervalSizeType> for ImperfectSize {
    fn from(interval: IntervalSizeType) -> Self {
        match interval {
            IntervalSizeType::Imperfect(interval) => interval,
            _ => unreachable!(),
        }
    }
}

impl From<IntervalSizeType> for PerfectSize {
    fn from(interval: IntervalSizeType) -> Self {
        match interval {
            IntervalSizeType::Perfect(interval) => interval,
            _ => unreachable!(),
        }
    }
}

impl From<SimpleInterval> for Interval {
    fn from(interval: SimpleInterval) -> Self {
        Interval::Simple(interval)
    }
}

impl From<CompoundInterval> for Interval {
    fn from(interval: CompoundInterval) -> Self {
        Interval::Compound(interval)
    }
}

impl From<Interval> for SimpleInterval {
    fn from(interval: Interval) -> Self {
        match interval {
            Interval::Simple(interval) => interval,
            Interval::Compound(interval) => interval.base_interval,
        }
    }
}
