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

    /// Creates a major pentatonic scale
    pub fn major_pentatonic(root: Option<Note>) -> Self {
        Self::from_steps(root, MajorPentatonic.to_steps())
    }

    /// Creates a minor pentatonic scale
    pub fn minor_pentatonic(root: Option<Note>) -> Self {
        Self::from_steps(root, MinorPentatonic.to_steps())
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
    pub fn from_steps(root: Option<Note>, steps: Vec<u8>) -> Self {
        let mut intervals = Vec::new();
        let mut semitones = 0;
        for step in steps {
            semitones += step;
            intervals.push(Interval::new(semitones));
        }

        Self {
            root,
            intervals,
        }
    }
    
    /// Returns the steps of the scale
    pub fn to_steps(&self) -> Vec<u8> {
        let mut steps = Vec::new();
        let mut previous_semitone = 0;
        for interval in &self.intervals {
            steps.push(interval.semitones() - previous_semitone);
            previous_semitone = interval.semitones();
        }
        steps
    }

    /// Creates a scale from a list of notes
    pub fn from_notes(mut notes: Vec<Note>) -> Scale {
        if notes.len() == 0 {
            return Self {
                root: None,
                intervals: Vec::new(),
            }
        }
        
        notes.sort();
        
        let mut intervals = Vec::new();
        let root = notes.remove(0);
        for note in notes {
            intervals.push(root - note);
        }
        
        Self {
            root: Some(root),
            intervals,
        }
    }

    /// Returns the notes of the scale
    /// If the scale has no root, the first note is C4
    pub fn to_notes(&self) -> Vec<Note> {
        let root = match &self.root {
            Some(root) => *root,
            None => Note::new(60),
        };

        let mut notes = Vec::new();
        notes.push(root);
        for interval in &self.intervals {
            notes.push((root + *interval).unwrap());
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