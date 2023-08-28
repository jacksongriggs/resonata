pub use types::ScaleType::*;
pub use crate::{
    Note,
    Interval, 
    nope,
    error::{
        ScaleError::{self, *}, 
        ResonataError}
};

pub mod types;
pub mod macros;
mod utils;
mod tests;

/// A musical scale
/// Scale are represented by an optional root note and a list of intervals
/// 
/// Intervals are relative to the root note, so a major scale would be
/// [Unison, MajorSecond, MajorThird, PerfectFourth, PerfectFifth, MajorSixth, MajorSeventh]
/// 
/// A macro is provided to make creating scales easier:
/// scale!(note scale_type)
/// scale!(scale_type)
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scale {
    pub root: Option<Note>,
    intervals: Vec<Interval>,
}

impl Scale {
    /// Creates a major scale
    pub fn major(root: Option<Note>) -> Self {
        Self::from_steps(root, Major.to_steps())
    }

    /// Creates a minor scale
    pub fn minor(root: Option<Note>) -> Self {
        Self::from_steps(root, Minor.to_steps())
    }

    /// Creates a harmonic minor scale
    pub fn harmonic_minor(root: Option<Note>) -> Self {
        Self::from_steps(root, HarmonicMinor.to_steps())
    }

    /// Creates a melodic minor scale
    pub fn melodic_minor(root: Option<Note>) -> Self {
        Self::from_steps(root, MelodicMinor.to_steps())
    }

    /// Creates a pentatonic scale
    pub fn pentatonic(root: Option<Note>) -> Self {
        Self::from_steps(root, Pentatonic.to_steps())
    }

    /// Creates a minor blues scale
    pub fn minor_blues(root: Option<Note>) -> Self {
        Self::from_steps(root, MinorBlues.to_steps())
    }

    /// Creates a major blues scale
    pub fn major_blues(root: Option<Note>) -> Self {
        Self::from_steps(root, MajorBlues.to_steps())
    }

    /// Creates a whole tone scale
    pub fn whole_tone(root: Option<Note>) -> Self {
        Self::from_steps(root, WholeTone.to_steps())
    }

    /// Creates a diminished scale
    pub fn diminished(root: Option<Note>) -> Self {
        Self::from_steps(root, Diminished.to_steps())
    }

    /// Creates a chromatic scale
    pub fn chromatic(root: Option<Note>) -> Self {
        Self::from_steps(root, Chromatic.to_steps())
    }
    
    /// Creates a scale from a root note and a list of steps
    /// Steps are relative to the previous note
    pub fn from_steps(root: Option<Note>, steps: Vec<i8>) -> Self {
        let mut intervals = Vec::new();
        let mut semitones = 0;
        for step in steps {
            semitones += step;
            intervals.push(Interval::new(semitones).unwrap());
        }

        Self {
            root,
            intervals,
        }
    }
    
    /// Returns the steps of the scale
    pub fn to_steps(&self) -> Vec<i8> {
        let mut steps = Vec::new();
        let mut previous_semitone = 0;
        for interval in &self.intervals {
            steps.push(interval.semitones() - previous_semitone);
            previous_semitone = interval.semitones();
        }
        steps
    }

    /// Creates a scale from a list of notes
    pub fn from_notes(mut notes: Vec<Note>) -> Result<Scale, ResonataError> {
        if notes.len() < 3 {
            nope!(InvalidScale);
        }
    
        notes.sort();
        
        let mut intervals = Vec::new();
        let root = notes.remove(0);
        for note in notes {
            intervals.push(root.interval_to(&note)?);
        }
        
        Ok(Self {
            root: Some(root),
            intervals,
        })
    }

    /// Returns the notes of the scale
    /// If the scale has no root, the first note is C4
    pub fn to_notes(&self) -> Vec<Note> {
        let root = match &self.root {
            Some(root) => *root,
            None => Note::new(60).unwrap(),
        };

        let mut notes = Vec::new();
        notes.push(root);
        for interval in &self.intervals {
            notes.push((root + *interval).unwrap());
        }

        notes
    }
    
    /// Rotates the scale by n steps in the given direction
    pub fn rotate(&mut self, n: usize, direction: Direction) {
        *self = self.rotated(n, direction);
    }

    /// Returns a rotated scale by n steps in the given direction
    pub fn rotated(&self, n: usize, direction: Direction) -> Self {
        let mut steps = self.to_steps();
        let n = n % steps.len();
        match direction {
            Direction::Up => steps.rotate_left(n),
            Direction::Down => steps.rotate_right(n),
        }
        Self::from_steps(self.root, steps)
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

/// The direction of a rotation
pub enum Direction {
    Up,
    Down,
}