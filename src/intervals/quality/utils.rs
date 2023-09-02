use crate::{intervals::*, error::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

impl From<IntervalQuality> for i8 {
    fn from(iq: IntervalQuality) -> Self {
        match iq {
            Diminished(n) => -(n as i8) - 1,
            Augmented(n) => n as i8,
            Minor => -1,
            Major => 0,
            Perfect => 0,
        }
    }
}

impl FromStr for IntervalQuality {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            nope!(InvalidIntervalQuality);
        }

        match s {
            "m" => Ok(Minor),
            "M" => Ok(Major),
            "P" => Ok(Perfect),
            _ => {
                let mut chars = s.chars();
                let first_char = chars.next().unwrap();
                let mut n = 1; // One 'd' or 'A' already there in first_char

                for c in chars {
                    if c == first_char {
                        n += 1;
                    } else {
                        nope!(InvalidIntervalQuality);
                    }
                }

                match first_char {
                    'd' => Ok(Diminished(n)),
                    'A' => Ok(Augmented(n)),
                    _ => err!(InvalidIntervalQuality),
                }
            }
        }
    }
}

impl Display for IntervalQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            Diminished(n) => std::iter::repeat('d').take(*n as usize).collect(),
            Augmented(n) => std::iter::repeat('A').take(*n as usize).collect(),
            Minor => "m".to_string(),
            Major => "M".to_string(),
            Perfect => "P".to_string(),
        };

        write!(f, "{}", token)
    }
}

impl Debug for IntervalQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for IntervalQuality {
    fn eq(&self, other: &Self) -> bool {
        i8::from(*self) == i8::from(*other)
    }
}

impl Eq for IntervalQuality {}

impl PartialOrd for IntervalQuality {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        i8::from(*self).partial_cmp(&i8::from(*other))
    }
}

impl Ord for IntervalQuality {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        i8::from(*self).cmp(&i8::from(*other))
    }
}
