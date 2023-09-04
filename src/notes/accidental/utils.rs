use crate::{error::*, notes::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

impl From<i32> for Accidental {
    fn from(value: i32) -> Self {
        Accidental::from_semitones(value)
    }
}

impl From<Accidental> for i32 {
    fn from(acc: Accidental) -> Self {
        acc.to_semitones()
    }
}

impl Add<i32> for Accidental {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self::from(i32::from(self) + rhs)
    }
}

impl AddAssign<i32> for Accidental {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl Sub<i32> for Accidental {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self::from(i32::from(self) - rhs)
    }
}

impl SubAssign<i32> for Accidental {
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs;
    }
}

impl FromStr for Accidental {
    type Err = ResonataError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
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
                        nope!(InvalidAccidentalCombination(s.to_string()))
                    }
                }
                _ => nope!(InvalidAccidental(s.to_string())),
            }
        }

        if sharp_count > 0 && flat_count > 0 {
            nope!(InvalidAccidentalCombination(s.to_string()))
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
