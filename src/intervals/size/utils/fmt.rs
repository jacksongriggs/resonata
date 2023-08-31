use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr};
use super::*;

impl FromStr for IntervalSize {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if str is a cardinal number convert into u8 and then into IntervalSize
        if let Ok(number) = s.parse::<u8>() {
            return Ok(IntervalSize::from(number - 1))
        }

        // if str is an ordinal number, convert into u8 and then into IntervalSize
        if let Ok(number) = s
            .trim_end_matches("st")
            .trim_end_matches("nd")
            .trim_end_matches("rd")
            .trim_end_matches("th")
            .trim_end_matches("ve")
            .parse::<u8>() {
            return Ok(IntervalSize::from(number - 1))
        }
        
        // if str is a string, match against the string and return the corresponding IntervalSize
        match s.to_lowercase().as_str() {
            "unison" | "u" => Ok(Unison),
            "second" => Ok(Second),
            "third" => Ok(Third),
            "fourth" => Ok(Fourth),
            "fifth" => Ok(Fifth),
            "sixth" => Ok(Sixth),
            "seventh" => Ok(Seventh),
            _ => nope!(InvalidIntervalSize)
        }
    }
}

impl Display for IntervalSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // let number = *self as u8 + (octaves * 7);
        // match number {
        //     1 => "U".to_string(),
        //     8 => "8ve".to_string(),
        //     _ => {
        //         let suffix = match number % 10 {
        //             1 => "st",
        //             2 => "nd",
        //             3 => "rd",
        //             _ => "th",
        //         };
        //         format!("{}{}", number, suffix)
        //     }
        // }
        match self {
            Unison => write!(f, "U"),
            Second => write!(f, "2"),
            Third => write!(f, "3"),
            Fourth => write!(f, "4"),
            Fifth => write!(f, "5"),
            Sixth => write!(f, "6"),
            Seventh => write!(f, "7"),
        }
    }
}

impl Debug for IntervalSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}