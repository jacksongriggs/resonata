use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr, ops::{AddAssign, Add}};
use regex::Regex;
use super::super::*;

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

impl PartialEq for IntervalQuality {
    fn eq(&self, other: &Self) -> bool {
        i8::from(*self) == i8::from(*other)
    }
}

impl Eq for IntervalQuality {}

impl PartialOrd for IntervalQuality {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_i8 = i8::from(*self);
        let other_i8 = i8::from(*other);
        self_i8.partial_cmp(&other_i8)
    }
}

impl Ord for IntervalQuality {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_i8 = i8::from(*self);
        let other_i8 = i8::from(*other);
        self_i8.cmp(&other_i8)
    }
}

impl FromStr for IntervalQuality {
    type Err = ResonataError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?P<quality>[#x𝄪b♯♯♭♭♮+-mMpPaAdD\+-]*)").unwrap();
        if let Some(cap) = re.captures(s) {
            let quality_expr = cap.name("quality").map_or("", |x| x.as_str());
            let mut quality = IntervalQuality::Perfect;
            let mut count = 0;
            for c in quality_expr.chars() {
                match c {
                    'm' => {
                        if count > 0 {
                            eprintln!("IntervalQuality: {}: {}", InvalidIntervalQuality, s);
                            nope!(InvalidIntervalQuality)
                        } else {
                            quality = IntervalQuality::Minor;
                            count += 1;
                        }
                    },
                    'M' => {
                        if count > 0 {
                            eprintln!("IntervalQuality: {}: {}", InvalidIntervalQuality, s);
                            nope!(InvalidIntervalQuality)
                        } else {
                            quality = IntervalQuality::Major;
                            count += 1;
                        }
                    },
                    'P' => {
                        if count > 0 {
                            eprintln!("IntervalQuality: {}: {}", InvalidIntervalQuality, s);
                            nope!(InvalidIntervalQuality)
                        } else {
                            quality = IntervalQuality::Perfect;
                            count += 1;
                        }
                    },
                    'A' => {
                        if count > 0 {
                            eprintln!("IntervalQuality: {}: {}", InvalidIntervalQuality, s);
                            nope!(InvalidIntervalQuality)
                        } else {
                            quality = IntervalQuality::Augmented(1);
                            count += 1;
                        }
                    },
                    'd' => {
                        if count > 0 {
                            eprintln!("IntervalQuality: {}: {}", InvalidIntervalQuality, s);
                            nope!(InvalidIntervalQuality)
                        } else {
                            quality = IntervalQuality::Diminished(1);
                            count += 1;
                        }
                    },
                    _ => {
                        eprintln!("IntervalQuality: {}: {}", InvalidIntervalQuality, s);
                        nope!(InvalidIntervalQuality)
                    }
                }
            }
            Ok(quality)
        } else {
            eprintln!("IntervalQuality: {}: {}", InvalidIntervalQuality, s);
            err!(InvalidIntervalQuality)
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

impl Debug for IntervalQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}