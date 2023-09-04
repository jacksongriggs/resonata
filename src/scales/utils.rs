use crate::{error::*, scales::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign},
    str::FromStr,
};

impl FromStr for Scale {
    type Err = ResonataError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.replace(",", " ");
        
        // First attempt to parse as a string of steps and execute the from_steps method
        let steps: std::result::Result<Vec<i32>, _> = s.split_whitespace()
            .map(str::parse)
            .collect();
        match steps {
            Ok(steps) if steps.len() >= 2 => {
                return Scale::from_steps(steps);
            }
            _ => (),
        }

        // If that fails, try parsing as a string of note names and execute the from_notes method
        let notes: std::result::Result<Vec<Note>, _> = s.split_whitespace()
            .map(str::parse)
            .collect();
        match notes {
            Ok(notes) if notes.len() >= 2 => {
                Ok(Scale::from_notes(notes))
            }
            _ => nope!(InvalidScale),
        }
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let intervals = self
            .intervals
            .iter()
            .map(|interval| interval.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", intervals)
    }
}

impl Debug for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Add for Scale {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut scale = self;
        scale += rhs;
        scale
    }
}

impl AddAssign for Scale {
    fn add_assign(&mut self, rhs: Self) {
        self.intervals.extend(rhs.intervals);
    }
}

impl From<ScaleType> for ScaleEnumType {
    fn from(scale_type: ScaleType) -> Self {
        ScaleEnumType::ScaleType(scale_type)
    }
}

impl From<MajorMode> for ScaleEnumType {
    fn from(major_mode: MajorMode) -> Self {
        ScaleEnumType::MajorMode(major_mode)
    }
}

impl From<HarmonicMinorMode> for ScaleEnumType {
    fn from(harmonic_minor_mode: HarmonicMinorMode) -> Self {
        ScaleEnumType::HarmonicMinorMode(harmonic_minor_mode)
    }
}

impl From<MelodicMinorMode> for ScaleEnumType {
    fn from(melodic_minor_mode: MelodicMinorMode) -> Self {
        ScaleEnumType::MelodicMinorMode(melodic_minor_mode)
    }
}
