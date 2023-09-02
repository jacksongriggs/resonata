use crate::{intervals::*, error::*};
use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
    ops::{Add, AddAssign, Sub, SubAssign},
};

impl FromStr for Interval {
    type Err = ResonataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Variables to store the parsed quality and size
        let mut quality_string = String::new();
        let mut size_string = s.to_string();

        // Split string into quality and size.
        // Handle the cases where quality is 'A', 'd', 'M', 'm' or 'P' characters
        // Iterate over the string from the start and keep adding the characters
        // to the quality until a numeric digit is encountered.
        for c in s.chars() {
            match c {
                'A' | 'd' | 'M' | 'm' | 'P' => {
                    quality_string.push(c);
                    size_string = size_string.split_off(1);
                }
                _ => break,
            }
        }

        // If the quality string is empty, then parse the size string as semitones
        if quality_string.is_empty() {
            let semitones = match size_string.parse::<i32>() {
                Ok(n) => n,
                Err(_) => nope!(InvalidIntervalSize),
            };

            match Interval::from_semitones(semitones) {
                Some(interval) => return Ok(interval),
                None => nope!(InvalidInterval),
            }
        }


        // Parse quality from the quality string
        let quality = IntervalQuality::from_str(&quality_string)?;

        // Parse size from the size string:
        let mut octaves: u8 = 0;
        let size = match size_string.as_str() {
            "U" => Unison,
            _ => match size_string.parse::<u8>() {
                Ok(n) => {
                    match n {
                        0 => nope!(InvalidIntervalSize),
                        _ => {
                            let n = n - 1;
                            if n >= 7 {
                                octaves = n / 7;
                            }
                            IntervalSize::from(n % 7)
                        }
                    }
                }
                Err(_) => nope!(InvalidIntervalSize),
            },
        };

        // Construct and return the interval
        match Interval::build(quality, size, octaves) {
            Some(interval) => Ok(interval),
            None => nope!(InvalidInterval),
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let quality = self.quality.to_string();
        let size = self.size as u8 + 1 + (self.octaves * 7);
        write!(f, "{}{}", quality, size)
    }
}

impl Debug for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


impl Add for Interval {
    type Output = Option<Interval>;
    fn add(self, rhs: Self) -> Option<Interval> {
        Interval::from_semitones(self.to_semitones() + rhs.to_semitones())
    }
}

impl Sub for Interval {
    type Output = Option<Interval>;
    fn sub(self, rhs: Self) -> Option<Interval> {
        Interval::from_semitones(self.to_semitones() - rhs.to_semitones())
    }
}

impl AddAssign for Interval {
    fn add_assign(&mut self, rhs: Self) {
        match *self + rhs {
            Some(interval) => *self = interval,
            None => panic!("Interval addition overflowed"),
        }
    }
}

impl SubAssign for Interval {
    fn sub_assign(&mut self, rhs: Self) {
        match *self - rhs {
            Some(interval) => *self = interval,
            None => panic!("Interval subtraction overflowed"),
        }
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.to_semitones() == other.to_semitones()
    }
}

impl Eq for Interval {}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_semitones().cmp(&other.to_semitones()))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_semitones().cmp(&other.to_semitones())
    }
}