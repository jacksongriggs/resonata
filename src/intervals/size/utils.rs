use std::{fmt::{self, Display, Formatter}, str::FromStr};
use super::super::*;

impl FromStr for IntervalSize {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "unison" | "u" | "1st" | "1"  => Ok(Unison),
            "second" | "2nd" | "2" => Ok(Second),
            "third" | "3rd" | "3" => Ok(Third),
            "fourth" | "4th" | "4" => Ok(Fourth),
            "fifth" | "5th" | "5" => Ok(Fifth),
            "sixth" | "6th" | "6" => Ok(Sixth),
            "seventh" | "7th" | "7" => Ok(Seventh),
            _ => {
                eprintln!("Invalid interval size: {}", s);
                nope!(InvalidIntervalSize)
            }
        }
    }
}

impl Display for IntervalSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            Unison => "U",
            Second => "2",
            Third => "3",
            Fourth => "4",
            Fifth => "5",
            Sixth => "6",
            Seventh => "7",
        };

        write!(f, "{}", token)
    }
}