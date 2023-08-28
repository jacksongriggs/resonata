use std::{fmt::{self, Display, Formatter}, str::FromStr};
use regex::Regex;
use super::super::*;

impl FromStr for IntervalQuality {
    type Err = ResonataError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(M(?:aj(?:or)?)?|m(?:in(?:or)?)?|(?i)P(?:erf(?:ect)?)?|(?i)a(?:ug(?:mented)?)?(?:\+{2,3})?|(?i)d(?:im(?:inished)?)?(?:-{2,3})?)$").unwrap();

        if let Some(cap) = re.captures(s) {
            let quality = &cap[1];
            match quality {
                "M" | "Maj" | "Major" => Ok(Major),
                "m" | "min" | "minor" => Ok(Minor),
                s if s.to_lowercase().starts_with("p") => Ok(Perfect),
                s if s.to_lowercase().starts_with("aug") || s == "A" || s.ends_with('+') => {
                    let count = s.chars().filter(|&c| c == '+').count() as u8;
                    Ok(Augmented(count + 1))
                },
                s if s.to_lowercase().starts_with("dim") || s == "d" || s.ends_with('-') => {
                    let count = s.chars().filter(|&c| c == '-').count() as u8;
                    Ok(Diminished(count + 1))
                },
                _ => {
                    eprintln!("Invalid interval quality: {}", quality);
                    nope!(InvalidIntervalQuality)
                }
            }
        } else {
            eprintln!("Invalid interval quality: {}", s);
            nope!(InvalidIntervalQuality)
        }
    }
}

impl Display for IntervalQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            Diminished(n) => std::iter::repeat('D').take(*n as usize).collect(),
            Augmented(n) => std::iter::repeat('A').take(*n as usize).collect(),
            Minor => "m".to_string(),
            Major => "M".to_string(),
            Perfect => "P".to_string(),
        };

        write!(f, "{}", token)
    }
}