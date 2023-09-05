use crate::{error::*, notes::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

impl Accidental {
    /// Returns the number of semitones from the given accidental
    /// to the natural accidental.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// assert_eq!(Accidental::Flat(1).to_semitones(), -1);
    /// assert_eq!(Accidental::Natural.to_semitones(), 0);
    /// assert_eq!(Accidental::Sharp(2).to_semitones(), 2);
    /// ```
    pub fn to_semitones(&self) -> i32 {
        match self {
            Accidental::Flat(n) => -(*n as i32),
            Accidental::Natural => 0,
            Accidental::Sharp(n) => *n as i32,
        }
    }

    /// Returns an accidental from the given number of semitones from
    /// the natural accidental. Values will be clamped to the range
    /// (-127, 127), which should be more than enough for most use cases!
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// assert_eq!(Accidental::from_semitones(-1), Accidental::Flat(1));
    /// assert_eq!(Accidental::from_semitones(0), Accidental::Natural);
    /// assert_eq!(Accidental::from_semitones(2), Accidental::Sharp(2));
    /// ```
    pub fn from_semitones(semitones: i32) -> Self {
        match semitones {
            0 => Accidental::Natural,
            n if n > 0 => Accidental::Sharp(n.min(127) as u8),
            n if n < 0 => Accidental::Flat(-n.max(-127) as u8),
            _ => unreachable!(),
        }
    }
}

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
                        return Ok(Accidental::Natural);
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
            Ok(Accidental::Flat(flat_count))
        } else {
            Ok(Accidental::Sharp(sharp_count))
        }
    }
}

impl Display for Accidental {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            Accidental::Flat(n) => "♭".repeat(*n as usize),
            Accidental::Natural => "".to_string(),
            Accidental::Sharp(n) => {
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
