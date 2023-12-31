use crate::{error::ResonataError, intervals::Interval, notes::*};

pub use crate::scale;
pub use types::{HarmonicMinorMode, MajorMode, MelodicMinorMode, ScaleEnumType, ScaleType};

pub mod types;
mod utils;

type Result<T> = std::result::Result<T, ResonataError>;

/// A musical scale
/// Scale are represented by an optional root note and a list of intervals
///
/// Intervals are relative to the root, which is an implied unison, so a major scale
/// would be represented by [M2, M3, P4, P5, M6, M7, P8]
///
/// A macro is provided to make creating scales easier:
///
/// scale!(string)  
///
/// scale!(type)  
///
/// scale!(type, rotation)  
///
/// ### Examples
/// ```
/// use resonata::{notes::*, scales::*};
///
/// let scale = scale!("2 2 1 2 2 2 1").unwrap();
/// assert_eq!(scale, Scale::major());
///
/// let scale = scale!("C D E F G A B C").unwrap();
/// assert_eq!(scale, Scale::major());
///
/// let scale = scale!(ScaleType::Major);
/// assert_eq!(scale.to_notes(note!("C").unwrap()), vec![
///     note!("C").unwrap(),
///     note!("D").unwrap(),
///     note!("E").unwrap(),
///     note!("F").unwrap(),
///     note!("G").unwrap(),
///     note!("A").unwrap(),
///     note!("B").unwrap(),
/// ]);
///
/// let scale = scale!(ScaleType::Major, 1);
/// assert_eq!(scale.to_notes(note!("C").unwrap()), vec![
///     note!("C").unwrap(),
///     note!("D").unwrap(),
///     note!("Eb").unwrap(),
///     note!("F").unwrap(),
///     note!("G").unwrap(),
///     note!("A").unwrap(),
///     note!("Bb").unwrap(),
/// ]);
/// ```
#[derive(PartialEq, Eq, Clone)]
pub struct Scale {
    intervals: Vec<Interval>,
}

/// A macro to create a scale from a root note and a scale type.
#[macro_export]
macro_rules! scale {
    ($str:literal) => {
        $str.parse::<Scale>()
    };
    ($scale_type:expr) => {
        Scale::from_steps($scale_type.as_steps()).unwrap()
    };
    ($scale_type:expr, $rot:expr) => {
        Scale::from_steps($scale_type.as_steps()).unwrap().rotated($rot)
    };
}

impl Scale {
    /// Creates a scale from a list of steps
    /// Steps are relative to the previous note
    /// For example, a major scale would be [2, 2, 1, 2, 2, 2, 1]
    /// Internally, this is converted to [M2, M3, P4, P5, M6, M7, P8]
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, scales::*, intervals::*};
    ///
    /// let scale = Scale::from_steps(vec![2, 2, 1, 2, 2, 2, 1]).unwrap();
    /// assert_eq!(scale.intervals(), &vec![
    ///     inv!("M2").unwrap(),
    ///     inv!("M3").unwrap(),
    ///     inv!("P4").unwrap(),
    ///     inv!("P5").unwrap(),
    ///     inv!("M6").unwrap(),
    ///     inv!("M7").unwrap(),
    ///     inv!("P8").unwrap()
    /// ]);
    /// ```
    pub fn from_steps(steps: Vec<i32>) -> Result<Self> {
        let mut intervals = Vec::new();
        let mut semitones = 0;
        for step in steps {
            semitones += step;
            intervals.push(Interval::from_semitones(semitones)?);
        }

        Ok(Self { intervals })
    }

    /// Returns the steps of the scale
    /// Steps are relative to the previous note
    /// For example, a major scale would be [2, 2, 1, 2, 2, 2, 1]
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, scales::*, intervals::*};
    ///
    /// let scale = Scale::major();
    /// assert_eq!(scale.to_steps(), vec![2, 2, 1, 2, 2, 2, 1]);
    /// ```
    pub fn to_steps(&self) -> Vec<i32> {
        let mut steps = vec![self.intervals[0].to_semitones()];
        for window in self.intervals.windows(2) {
            steps.push(window[1].to_semitones() - window[0].to_semitones());
        }

        steps
    }

    /// Creates a scale from a list of notes. The first note is the root note
    /// and the rest of the notes are used to create the intervals. The last note
    /// is used to create the interval between the last note and the root note.
    /// For example, a major scale would be [C, D, E, F, G, A, B, C]. Each note
    /// is assumed to be higher than the previous note.
    ///
    /// This is the same as using scale!("C, D, E, F, G, A, B, C").
    ///
    /// To create a scale from a list of intervals, use `Scale::from_steps()`.
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, scales::*};
    ///
    /// let scale = Scale::from_notes(vec![
    ///     note!("C").unwrap(),
    ///     note!("D").unwrap(),
    ///     note!("E").unwrap(),
    ///     note!("F").unwrap(),
    ///     note!("G").unwrap(),
    ///     note!("A").unwrap(),
    ///     note!("B").unwrap(),
    ///     note!("C").unwrap(),
    /// ]);
    /// assert_eq!(scale, Scale::major());
    ///
    /// let scale = Scale::from_notes(vec![
    ///     note!("C").unwrap(),
    ///     note!("D").unwrap(),
    ///     note!("E").unwrap(),
    ///     note!("F#").unwrap(),
    ///     note!("G").unwrap(),
    /// ]);
    /// assert_eq!(scale.to_steps(), vec![2, 2, 2, 1]);
    ///
    /// let scale = Scale::from_notes(vec![
    ///     note!("C").unwrap(),
    ///     note!("C").unwrap(),
    ///     note!("D").unwrap(),
    ///     note!("E").unwrap(),
    ///     note!("A").unwrap(),
    /// ]);
    /// assert_eq!(scale.to_steps(), vec![12, 2, 2, 5]);
    /// ```
    pub fn from_notes(mut notes: Vec<Note>) -> Scale {
        if notes.len() == 0 {
            return Self { intervals: Vec::new() };
        }

        let mut intervals = Vec::new();
        let root = notes.remove(0);
        let mut last_steps = 0;
        for note in notes {
            let mut steps = note.to_chromatic_scale_degree() - root.to_chromatic_scale_degree();
            while steps <= last_steps {
                steps += 12;
            }
            last_steps = steps;
            intervals.push(Interval::from_semitones(last_steps as i32).unwrap());
        }

        Self { intervals }
    }

    /// Returns the notes of the scale from the given root note
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, scales::*};
    ///
    /// let scale = scale!("C D E F# G A B C").unwrap();
    /// assert_eq!(scale.to_notes(note!("C").unwrap()), vec![
    ///     note!("C").unwrap(),
    ///     note!("D").unwrap(),
    ///     note!("E").unwrap(),
    ///     note!("F#").unwrap(),
    ///     note!("G").unwrap(),
    ///     note!("A").unwrap(),
    ///     note!("B").unwrap(),
    /// ]);
    /// ```
    pub fn to_notes(&self, root: Note) -> Vec<Note> {
        let mut notes = vec![root];
        for interval in self.intervals.iter().take(self.intervals.len() - 1) {
            notes.push(root + *interval);
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
        Self::from_steps(steps).unwrap()
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

    /// Adds an interval to the scale. The interval is added to the last interval
    /// in the scale. If the interval is invalid, an error is returned.
    /// If the interval is valid, the scale is updated and Ok(()) is returned.
    /// 
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, scales::*, intervals::*};
    /// 
    /// let mut scale = scale!("C D E F G").unwrap();
    /// scale.add_interval(inv!("M2").unwrap()).unwrap();
    /// assert_eq!(scale, scale!("C D E F G A").unwrap());
    /// ```
    pub fn add_interval(&mut self, interval: Interval) -> Result<()> {
        let new = *self.intervals.last().unwrap() + interval;
        self.intervals.push(new?);
        Ok(())
    }
}
