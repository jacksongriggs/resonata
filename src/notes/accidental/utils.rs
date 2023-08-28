use std::{fmt::{self, Display, Formatter}, str::FromStr};
use super::*;
use crate::error::NoteError;

impl FromStr for Accidental {
    type Err = NoteError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut sharp_count = 0;
        let mut flat_count = 0;
    
        for c in s.chars() {
            match c {
                '#' | '♯' => sharp_count += 1,
                'x' | '𝄪' => sharp_count += 2,
                'b' | '♭' => flat_count += 1,
                '♮' => {
                    if sharp_count == 0 && flat_count == 0 {
                        return Ok(Natural)
                    } else {
                        return Err(NoteError::InvalidAccidental)
                    }
                },
                _ => return Err(NoteError::InvalidAccidental),
            }
        }
    
        if sharp_count > 0 && flat_count > 0 {
            Err(NoteError::InvalidAccidental)
        } else if flat_count > 0 {
            Ok(Flat(flat_count))
        } else {
            Ok(Sharp(sharp_count))
        }
    }    
}

impl Display for Accidental {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }   
}