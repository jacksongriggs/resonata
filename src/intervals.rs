pub use quality::IntervalQuality::{self, *};
pub use size::IntervalSize::{self, *};
pub use crate::error::IntervalError;

pub mod quality;
pub mod size;
pub mod utils;
pub mod macros;
mod tests;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Interval {
    semitones: i8,
    quality: IntervalQuality,
    size: IntervalSize,
    octaves: u8,
}

impl Interval {
    pub fn new(semitones: i8) -> Result<Interval, IntervalError> {
        if semitones < -127 {
            return Err(IntervalError::InvalidInterval);
        }

        let (quality, size) = match semitones % 12 {
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
            _ => unreachable!("Modulo 12 should never be outside of 0-11")
        };

        let octaves = (semitones / 12) as u8;

        Ok(Interval {
            semitones,
            quality,
            size,
            octaves,
        })
    }

    pub fn build(quality: IntervalQuality, size: IntervalSize, octaves: u8) -> Result<Self, IntervalError> {
        match quality {
            Major | Minor => match size {
                Unison | Fourth | Fifth => return Err(IntervalError::InvalidInterval),
                _ => {}
            },
            Perfect => match size {
                Second | Third | Sixth | Seventh => return Err(IntervalError::InvalidInterval),
                _ => {}
            },
            _ => {}
        }

        let semitones = size.to_diatonic_semitones() + quality.to_semitones();

        Ok(Self {
            semitones: semitones + (octaves as i8 * 12),
            quality,
            size,
            octaves,
        })
    }

    pub fn from_notes(note1: &crate::Note, note2: &crate::Note) -> Self {
        Self::new((note1.number() as i8 - note2.number() as i8).abs()).unwrap()
    }

    pub fn semitones(&self) -> i8 {
        self.semitones
    }
}
