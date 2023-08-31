use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr};
use lazy_static::lazy_static;
use regex::Regex;
use super::*;


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