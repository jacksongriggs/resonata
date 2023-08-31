pub use crate::{
    error::{
        ResonataError,
        ScaleError::{self, *},
    },
    nope, Interval, Note, PitchedNote,
};
pub use types::ScaleType::*;

pub mod macros;
pub mod types;
mod utils;

/// A musical scale
/// Scale are represented by an optional root note and a list of intervals
///
/// Intervals are relative to the root note, so a major scale would be
/// [Unison, MajorSecond, MajorThird, PerfectFourth, PerfectFifth, MajorSixth, MajorSeventh]
///
/// A macro is provided to make creating scales easier:
/// scale!(note scale_type)
/// scale!(scale_type)
#[derive(PartialEq, Eq, Clone)]
pub struct Scale {
    intervals: Vec<Interval>,
}

impl Scale {
    /// Creates a major scale
    pub fn major() -> Self {
        Self::from_steps(Major.as_steps())
    }

    /// Creates a minor scale
    pub fn minor() -> Self {
        Self::from_steps(Minor.as_steps())
    }

    /// Creates a harmonic minor scale
    pub fn harmonic_minor() -> Self {
        Self::from_steps(HarmonicMinor.as_steps())
    }

    /// Creates a melodic minor scale
    pub fn melodic_minor() -> Self {
        Self::from_steps(MelodicMinor.as_steps())
    }

    /// Creates a major pentatonic scale
    pub fn major_pentatonic() -> Self {
        Self::from_steps(MajorPentatonic.as_steps())
    }

    /// Creates a minor pentatonic scale
    pub fn minor_pentatonic() -> Self {
        Self::from_steps(MinorPentatonic.as_steps())
    }

    /// Creates a minor blues scale
    pub fn minor_blues() -> Self {
        Self::from_steps(MinorBlues.as_steps())
    }

    /// Creates a major blues scale
    pub fn major_blues() -> Self {
        Self::from_steps(MajorBlues.as_steps())
    }

    /// Creates a whole tone scale
    pub fn whole_tone() -> Self {
        Self::from_steps(WholeTone.as_steps())
    }

    /// Creates a diminished scale
    pub fn diminished() -> Self {
        Self::from_steps(Diminished.as_steps())
    }

    /// Creates a chromatic scale
    pub fn chromatic() -> Self {
        Self::from_steps(Chromatic.as_steps())
    }

    /// Creates a scale from a list of steps
    /// Steps are relative to the previous note
    /// For example, a major scale would be [2, 2, 1, 2, 2, 2, 1]
    pub fn from_steps(steps: Vec<u8>) -> Self {
        let mut intervals = Vec::new();
        for step in steps {
            intervals.push(Interval::from(step));
        }

        Self { intervals }
    }

    /// Returns the steps of the scale
    /// Steps are relative to the previous note
    /// For example, a major scale would be [2, 2, 1, 2, 2, 2, 1]
    pub fn to_steps(&self) -> Vec<u8> {
        let mut steps = Vec::new();
        for interval in &self.intervals {
            steps.push(u8::from(*interval));
        }

        steps
    }

    /// Creates a scale from a list of notes
    pub fn from_notes(mut notes: Vec<Note>) -> Scale {
        if notes.len() == 0 {
            return Self {
                intervals: Vec::new(),
            };
        }

        notes.sort();

        let mut intervals = Vec::new();
        let root = notes.remove(0);
        for note in notes {
            intervals.push(root - note);
        }

        Self { intervals }
    }

    /// Returns the notes of the scale from the given root note
    pub fn to_notes(&self, root: Note) -> Vec<Note> {
        let mut notes = Vec::new();
        let mut last_note = root;
        for interval in &self.intervals {
            notes.push(last_note);
            last_note += *interval;
        }
        notes
    }

    /// Rotates the scale by n steps in the given direction
    /// The root note is retained, so for example rotating a major scale up by 1 step
    /// will result in a dorian scale
    pub fn rotate(&mut self, n: i8) {
        *self = self.rotated(n);
    }

    /// Returns a rotated scale by n steps in the given direction
    /// The root note is kept, so for example rotating a major scale up by 1 step
    /// will result in a dorian scale
    pub fn rotated(&self, n: i8) -> Self {
        let mut steps = self.to_steps();
        let rotate_left = n > 0;
        let n = n % steps.len() as i8;
        match rotate_left {
            true => steps.rotate_left(n as usize),
            false => steps.rotate_right(n.abs() as usize),
        }
        Self::from_steps(steps)
    }

    /// Returns the interval at the given index
    /// If the index is out of bounds, the last interval is returned
    pub fn interval(&self, n: usize) -> Interval {
        let n = std::cmp::min(n, self.intervals.len() - 1);
        self.intervals[n]
    }

    /// Returns the intervals in the scale
    pub fn intervals(&self) -> &Vec<Interval> {
        &self.intervals
    }
}
