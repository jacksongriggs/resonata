use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr, ops::{Add, Sub, AddAssign, SubAssign}, cmp};
use regex::Regex;
use super::*;
use lazy_static::lazy_static;

impl From<u8> for Interval {
    fn from(value: u8) -> Self {
        let semitones = std::cmp::min(value, 127);

        let (quality, size) = match semitones % 12 {
            0 => (Perfect, Unison),
            1 => (Minor, Second),
            2 => (Major, Second),
            3 => (Minor, Third),
            4 => (Major, Third),
            5 => (Perfect, Fourth),
            6 => (Diminished(1), Fifth),
            7 => (Perfect, Fifth),
            8 => (Minor, Sixth),
            9 => (Major, Sixth),
            10 => (Minor, Seventh),
            11 => (Major, Seventh),
            _ => unreachable!("Modulo 12 should never be outside of 0-11")
        };

        let octaves = (semitones / 12) as u8;

        Interval {
            quality,
            size,
            octaves,
        }
    }
}

impl From<Interval> for u8 {
    fn from(value: Interval) -> Self {
        let semitones = value.size.to_diatonic_semitones();
        let semitones = match value.quality {
            Perfect | Major => semitones,
            Minor => semitones - 1,
            Augmented(n) => semitones + n,
            Diminished(n) => match value.size {
                Unison | Fourth | Fifth => semitones - n,
                _ => semitones - n - 1,
            }
        };

        semitones + value.octaves * 12
    }
}

impl From<Interval> for i8 {
    fn from(value: Interval) -> Self {
        u8::from(value) as i8
    }
}

impl From<Interval> for i32 {
    fn from(value: Interval) -> Self {
        u8::from(value) as i32
    }
}

impl From<i32> for Interval {
    fn from(value: i32) -> Self {
        Self::from(value.abs() as u8)
    }
}

impl From<i8> for Interval {
    fn from(value: i8) -> Self {
        Interval::from(value.abs() as u8)
    }
}

impl Add<i8> for Interval {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        Self::from(i8::from(self) + rhs.abs())
    }
}

impl AddAssign<i8> for Interval {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl Sub<Interval> for i8 {
    type Output = Interval;

    fn sub(self, rhs: Interval) -> Self::Output {
        Interval::from(self - i8::from(rhs))
    }
}

impl SubAssign<Interval> for i8 {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - i8::from(rhs);
    }
}

impl Sub<i8> for Interval {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        Self::from(i8::from(self) - rhs)
    }
}

impl SubAssign<i8> for Interval {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from(u8::from(self) + u8::from(other))
    }
}

impl AddAssign for Interval {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from((i8::from(self) - i8::from(other)).abs())
    }
}

impl SubAssign for Interval {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl cmp::PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        i8::from(*self) == i8::from(*other)
    }
}

impl cmp::Eq for Interval {}

impl cmp::PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(i8::from(*self).cmp(&i8::from(*other)))
    }
}

impl cmp::Ord for Interval {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        i8::from(*self).cmp(&i8::from(*other))
    }
}

lazy_static! {
    static ref INTERVAL_QUALITY_REGEX: Regex = Regex::new(r"^(?P<quality>[#x𝄪b♯♯♭♭♮mMpPaAdD\+-]*)(?P<size>\d+)(?:th)?$").unwrap();
}

impl FromStr for Interval {
    type Err = ResonataError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(char::is_numeric) {
            let size = s.parse::<u8>().map_err(|_| InvalidInterval)?;
            return Ok(Interval::from(size));
        }
        if let Some(cap) = INTERVAL_QUALITY_REGEX.captures(s) {
            let quality_expr = cap.name("quality").map_or("", |x| x.as_str());
            let size_expr = cap.name("size").map_or("", |x| x.as_str());
            let quality = IntervalQuality::from_str(quality_expr)?;
            let raw_size = size_expr.parse::<u8>().map_err(|_| InvalidInterval)?;
            let octaves = (raw_size - 1) / 7;
            let effective_size = (raw_size - 1) % 7 + 1;
            let size = IntervalSize::from_str(&effective_size.to_string())?;
            Interval::build(quality, size, octaves)
        } else {
            nope!(InvalidIntervalFormat)
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let quality = self.quality.to_string();
        let size = self.size.to_string(); // TODO: Fix this

        write!(f, "{}{}", quality, size)
    }
}

impl Debug for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}