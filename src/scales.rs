use crate::{Note, Interval};
pub use types::ScaleType::*;
pub use crate::error::ScaleError;

pub mod types;
pub mod macros;
pub mod utils;
mod tests;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scale {
    pub root: Option<Note>,
    intervals: Vec<Interval>,
}

impl Scale {
    pub fn major(root: Option<Note>) -> Self {
        Self::from_steps(root, Major.to_steps())
    }

    pub fn minor(root: Option<Note>) -> Self {
        Self::from_steps(root, Minor.to_steps())
    }

    pub fn harmonic_minor(root: Option<Note>) -> Self {
        Self::from_steps(root, HarmonicMinor.to_steps())
    }

    pub fn melodic_minor(root: Option<Note>) -> Self {
        Self::from_steps(root, MelodicMinor.to_steps())
    }

    pub fn pentatonic(root: Option<Note>) -> Self {
        Self::from_steps(root, Pentatonic.to_steps())
    }

    pub fn minor_blues(root: Option<Note>) -> Self {
        Self::from_steps(root, MinorBlues.to_steps())
    }

    pub fn major_blues(root: Option<Note>) -> Self {
        Self::from_steps(root, MajorBlues.to_steps())
    }

    pub fn whole_tone(root: Option<Note>) -> Self {
        Self::from_steps(root, WholeTone.to_steps())
    }

    pub fn diminished(root: Option<Note>) -> Self {
        Self::from_steps(root, Diminished.to_steps())
    }

    pub fn chromatic(root: Option<Note>) -> Self {
        Self::from_steps(root, Chromatic.to_steps())
    }
    
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
    
    pub fn to_steps(&self) -> Vec<i8> {
        let mut steps = Vec::new();
        let mut previous_semitone = 0;
        for interval in &self.intervals {
            steps.push(interval.semitones() - previous_semitone);
            previous_semitone = interval.semitones();
        }
        steps
    }

    pub fn from_notes(mut notes: Vec<Note>) -> Result<Scale, ScaleError> {
        if notes.len() < 3 {
            return Err(ScaleError::InvalidScale);
        }
    
        notes.sort();
        
        let mut intervals = Vec::new();
        let root = notes.remove(0);
        for note in notes {
            intervals.push(root.interval_to(&note));
        }
        
        Ok(Self {
            root: Some(root),
            intervals,
        })
    }

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
    
    pub fn rotate(&mut self, n: usize, direction: Direction) {
        *self = self.rotated(n, direction);
    }

    pub fn rotated(&self, n: usize, direction: Direction) -> Self {
        let mut steps = self.to_steps();
        let n = n % steps.len();
        match direction {
            Direction::Up => steps.rotate_left(n),
            Direction::Down => steps.rotate_right(n),
        }
        Self::from_steps(self.root, steps)
    }

    pub fn interval(&self, n: usize) -> Interval {
        self.intervals[n]
    }

    pub fn intervals(&self) -> &Vec<Interval> {
        &self.intervals
    }
}

pub enum Direction {
    Up,
    Down,
}