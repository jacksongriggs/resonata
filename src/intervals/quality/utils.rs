use std::{fmt::{self, Display, Formatter, Debug}, str::FromStr};
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
        i8::from(*self).partial_cmp(&i8::from(*other))
    }
}

impl Ord for IntervalQuality {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        i8::from(*self).cmp(&i8::from(*other))
    }
}

impl FromStr for IntervalQuality {
    type Err = ResonataError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?P<quality>[#x𝄪b♯♯♭♭♮+-mMpPaAdD\+-]*)").unwrap();
        if let Some(cap) = re.captures(s) {
            let quality_expr = cap.name("quality").map_or("", |x| x.as_str());
            let mut quality = IntervalQuality::Major;
            for c in quality_expr.chars() {
                match c {
                    'm' => quality = IntervalQuality::Minor,
                    'M' => quality = IntervalQuality::Major,
                    'P' => quality = IntervalQuality::Perfect,
                    'A' => quality = IntervalQuality::Augmented(1),
                    'd' => quality = IntervalQuality::Diminished(1),
                    _ => nope!(InvalidIntervalQuality)
                }
            }
            Ok(quality)
        } else {
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