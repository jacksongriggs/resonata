use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr};
use super::*;

impl FromStr for NoteName {
    type Err = ResonataError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "c" => Ok(C),
            "d" => Ok(D),
            "e" => Ok(E),
            "f" => Ok(F),
            "g" => Ok(G),
            "a" => Ok(A),
            "b" => Ok(B),
            _ => nope!(InvalidNoteName)
        }
    }
}

impl Display for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            A => "A",
            B => "B",
        };
        write!(f, "{}", token)
    }   
}

impl Debug for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }   
}