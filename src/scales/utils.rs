use std::fmt::{self, Display, Formatter, Debug};
use super::*;

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