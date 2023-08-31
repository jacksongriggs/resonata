use super::*;
use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

impl FromStr for Accidental {
    type Err = ResonataError;
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
                        return Ok(Natural);
                    } else {
                        nope!(InvalidAccidentalCombination)
                    }
                }
                _ => nope!(InvalidAccidental),
            }
        }

        if sharp_count > 0 && flat_count > 0 {
            nope!(InvalidAccidentalCombination)
        } else if flat_count > 0 {
            Ok(Flat(flat_count))
        } else {
            Ok(Sharp(sharp_count))
        }
    }
}

impl Display for Accidental {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            Flat(n) => "♭".repeat(*n as usize),
            Natural => "".to_string(),
            Sharp(n) => {
                let mut token;
                if *n % 2 == 0 {
                    token = "𝄪".repeat(*n as usize / 2);
                } else {
                    token = "♯".to_string();
                    token.push_str(&"𝄪".repeat((*n as usize - 1) / 2));
                }
                token
            }
        };
        write!(f, "{}", token)
    }
}

impl Debug for Accidental {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
