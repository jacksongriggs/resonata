use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr,};
use crate::error::IntervalError;
use super::*;

impl FromStr for Scale {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First attempt to parse as string of intervals\
        if let Ok (intervals) = s
            .split(", ")
            .map(|interval| 
                interval.parse::<Interval>()
                    .map_err(|_| IntervalError::InvalidInterval))
            .collect::<Result<Vec<Interval>, IntervalError>>() {
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
                intervals.push(12 - (notes[0] - notes[notes.len() - 1]));
                Ok(Scale { intervals })
            }
            _ => nope!(InvalidScale),
        }

    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let intervals = self.intervals
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