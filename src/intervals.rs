pub use quality::IntervalQuality::{self, *};
pub use size::IntervalSize::{self, *};
pub use crate::{
    nope,
    err,
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
#[derive(Clone, Copy)]
pub struct Interval {
    quality: IntervalQuality,
    size: IntervalSize,
    octaves: u8,
}

impl Interval {
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

        let semitones = size.to_diatonic_semitones() as i8 + i8::from(quality);
        let semitones = (semitones + octaves as i8 * 12).abs() as u8;

        match semitones {
            0..=127 => Ok (Self {
                quality,
                size,
                octaves,
            }),
            _ => nope!(InvalidInterval)
        }
    }

    /// Inverts the interval. Resulting interval will retain
    /// the same octave number as the original interval.
    pub fn inverted(&self) -> Self {
        match Self::build(self.quality.invert(), self.size.invert(), self.octaves) {
            Ok(interval) => interval,
            Err(_) => unreachable!("Inverting an interval should never fail")
        }
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
