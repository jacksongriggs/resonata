use super::*;
use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
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
            let semitones = match size_string.parse::<i8>() {
                Ok(n) => n,
                Err(_) => nope!(InvalidIntervalSize),
            };

            return Ok(Interval::from(semitones));
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
                            if n > 7 {
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
        Ok(Interval {
            quality,
            size,
            octaves,
        })
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let quality = self.quality.to_string();
        let size = u8::from(self.size) + self.octaves * 7;
        write!(f, "{}{}", quality, size)
    }
}

impl Debug for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
