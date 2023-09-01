use crate::yep;
use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use super::*;

impl FromStr for IntervalSize {
    type Err = ResonataError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Unison),
            "2" => Ok(Second),
            "3" => Ok(Third),
            "4" => Ok(Fourth),
            "5" => Ok(Fifth),
            "6" => Ok(Sixth),
            "7" => Ok(Seventh),
            _ => nope!(InvalidIntervalSize),
        }
    }
}

impl Display for IntervalSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
