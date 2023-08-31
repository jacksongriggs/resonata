use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr, ops::{Add, Sub, AddAssign, SubAssign}};
use super::super::*;

impl From<i8> for Accidental {
    fn from(value: i8) -> Self {
        if value < 0 {
            Flat(value.abs() as u8)
        } else if value == 0 {
            Natural
        } else {
            Sharp(value as u8)
        }
    }
}

impl From<Accidental> for i8 {
    fn from(acc: Accidental) -> Self {
        match acc {
            Flat(n) => -(n as i8),
            Natural => 0,
            Sharp(n) => n as i8,
        }
    }
}

impl Add<i8> for Accidental {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        Self::from(i8::from(self) + rhs)
    }
}

impl AddAssign<i8> for Accidental {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl Sub<i8> for Accidental {
    type Output = Self;
    
    fn sub(self, rhs: i8) -> Self::Output {
        Self::from(i8::from(self) - rhs)
    }
}

impl SubAssign<i8> for Accidental {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}

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
                        return Ok(Natural)
                    } else {
                        nope!(InvalidAccidentalCombination)
                    }
                },
                _ => nope!(InvalidAccidental)
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