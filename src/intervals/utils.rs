use super::*;

mod cmp;
mod fmt;
mod ops;
mod tests;

impl From<u8> for Interval {
    fn from(value: u8) -> Self {
        let value = value % 127;
        let (quality, size) = match value % 12 {
            0 => (Perfect, Unison),
            1 => (Minor, Second),
            2 => (Major, Second),
            3 => (Minor, Third),
            4 => (Major, Third),
            5 => (Perfect, Fourth),
            6 => (Diminished(1), Fifth),
            7 => (Perfect, Fifth),
            8 => (Minor, Sixth),
            9 => (Major, Sixth),
            10 => (Minor, Seventh),
            11 => (Major, Seventh),
            _ => unreachable!("Modulo 12 should never be outside of 0-11"),
        };

        let octaves = (value / 12) as u8;

        Interval {
            quality,
            size,
            octaves,
        }
    }
}

impl From<Interval> for u8 {
    fn from(interval: Interval) -> Self {
        let value = interval.size.to_diatonic_semitones();
        let semitones = match interval.quality {
            Perfect | Major => value,
            Minor => value - 1,
            Augmented(n) => value + n,
            Diminished(n) => match interval.size {
                Unison | Fourth | Fifth => value - n,
                _ => value - n - 1,
            },
        };

        (semitones + interval.octaves * 12) % 127
    }
}

impl From<Interval> for i8 {
    fn from(value: Interval) -> Self {
        u8::from(value) as i8
    }
}

impl From<i8> for Interval {
    fn from(value: i8) -> Self {
        Interval::from(value.abs() as u8)
    }
}

impl From<Interval> for i32 {
    fn from(value: Interval) -> Self {
        u8::from(value) as i32
    }
}

impl From<i32> for Interval {
    fn from(value: i32) -> Self {
        Interval::from(value.abs() as u8)
    }
}
