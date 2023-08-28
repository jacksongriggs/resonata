use std::{fmt::{self, Display, Formatter}, str::FromStr, cmp, ops::{Add, Sub}};
use regex::Regex;
use super::*;

impl FromStr for Interval {
    type Err = ResonataError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?P<quality>[mMpPaAdD\+-]*)(?P<size>\d+)(?:th)?").unwrap();
        if let Some(cap) = re.captures(s) {
            let quality_expr = cap.name("quality").map_or("", |x| x.as_str());
            let size_expr = cap.name("size").map_or("", |x| x.as_str());
            println!("quality: {}, size: {}", quality_expr, size_expr);
            let quality = IntervalQuality::from_str(quality_expr)?;
            let raw_size = size_expr.parse::<u8>()
                .map_err(|_| InvalidInterval)?;
            let octaves = (raw_size - 1) / 7;
            let effective_size = (raw_size - 1) % 7 + 1;
            let size = IntervalSize::from_str(&effective_size.to_string())?;
            Interval::build(quality, size, octaves)
        } else {
            eprintln!("Invalid interval format: {}", s);
            nope!(InvalidIntervalFormat)
        }
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.semitones + other.semitones).unwrap()
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.semitones - other.semitones).unwrap()
    }
}

impl cmp::PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.semitones.partial_cmp(&other.semitones)
    }
}

impl cmp::Ord for Interval {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.semitones.cmp(&other.semitones)
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let quality = self.quality.to_string();
        let size = self.size.as_str(self.octaves);

        write!(f, "{}{}", quality, size)
    }
}