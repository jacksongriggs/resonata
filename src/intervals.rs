pub use quality::IntervalQuality::{self, *};
pub use size::IntervalSize::{self, *};
pub use crate::{
    nope,
    error::{
        IntervalError::{self, *},
        ResonataError}
};

pub mod quality;
pub mod size;
pub mod macros;
mod utils;
mod tests;

/// A musical interval
/// The interval is represented by the number of semitones
/// between two notes.
/// 
/// The interval size is the size of the interval from unison
/// up to a seventh. If the interval is an octave or more,
/// the size will be the size of the interval modulo 7
/// (e.g. a ninth will be a second, a tenth will be a third, etc.)
/// 
/// The interval quality is the quality of the interval, which
/// can be diminished, minor, major, perfect or augmented.
/// Diminished and augmented take a number to represent
/// the number of diminished or augmented intervals. For example,
/// `Diminished(2)` would be a double diminished interval.
/// 
/// The number of octaves is the number of octaves of the interval.
/// For example, a ninth would have one octave.
/// 
/// A macro is provided to make creating intervals easier:
/// inv!(quality size octaves: u8)
/// inv!(quality size) (default octaves is 0)
/// inv!(string)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Interval {
    semitones: u8,
    quality: IntervalQuality,
    size: IntervalSize,
    octaves: u8,
}

impl Interval {
    /// Creates a new interval from a number of semitones.
    /// Numbers outside of the range 0 to 127 will be clamped
    /// to the nearest valid number, so it's best to do your
    /// own checking before creating an interval if you want
    /// to avoid this.
    pub fn new(semitones: u8) -> Self {
        let semitones = std::cmp::min(semitones, 127);

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

        Interval {
            semitones,
            quality,
            size,
            octaves,
        }
    }

    /// Creates a new interval from an interval quality, size and number of octaves
    pub fn build(quality: IntervalQuality, size: IntervalSize, octaves: u8) -> Result<Self, ResonataError> {
        match quality {
            Major | Minor => match size {
                Unison | Fourth | Fifth => nope!(InvalidInterval),
                _ => {}
            },
            Perfect => match size {
                Second | Third | Sixth | Seventh => nope!(InvalidInterval),
                _ => {}
            },
            _ => {}
        }

        let semitones = size.to_diatonic_semitones() as i8 + quality.to_semitones();
        let semitones = (semitones + octaves as i8 * 12).abs() as u8;

        match semitones {
            0..=127 => Ok (Self {
                semitones,
                quality,
                size,
                octaves,
            }),
            _ => nope!(InvalidInterval)
        }
           
    }

    /// Returns the size of the interval in semitones
    pub fn semitones(&self) -> u8 {
        self.semitones
    }

    /// Returns the quality of the interval
    pub fn quality(&self) -> IntervalQuality {
        self.quality
    }

    /// Returns the size of the interval, starting from unison
    /// up to a seventh. If the interval is an octave or more,
    /// the size will be the size of the interval modulo 7
    /// (e.g. a ninth will be a second, a tenth will be a third, etc.)
    pub fn size(&self) -> IntervalSize {
        self.size
    }

    /// Returns the number of octaves of the interval
    pub fn octaves(&self) -> u8 {
        self.octaves
    }
}
