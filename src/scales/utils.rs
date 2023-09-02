use crate::{error::*, scales::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign},
    str::FromStr,
};

impl FromStr for Scale {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First attempt to parse as string of intervals\
        if let Ok(intervals) = s
            .split(", ")
            .map(|interval| {
                interval
                    .parse::<Interval>()
                    .map_err(|_| IntervalError::InvalidInterval)
            })
            .collect::<Result<Vec<Interval>, IntervalError>>()
        {
            return Ok(Scale { intervals });
        }

        // If that fails, try parsing as a string of note names and compute the intervals
        let notes: Result<Vec<Note>, _> = s.split(", ").map(str::parse).collect();
        match notes {
            Ok(notes) if notes.len() >= 2 => {
                let mut intervals = Vec::new();
                for window in notes.windows(2) {
                    intervals.push(window[1] - window[0]);
                }
                intervals.push(notes[0] - notes[notes.len() - 1]);
                Ok(Scale { intervals })
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
