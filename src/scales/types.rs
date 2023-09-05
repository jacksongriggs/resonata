pub use HarmonicMinorMode::*;
pub use MajorMode::*;
pub use MelodicMinorMode::*;
pub use ScaleType::*;

pub mod utils;

use super::Scale;

impl Scale {
    /// Creates a major scale
    pub fn major() -> Self {
        Self::from_steps(Major.as_steps()).unwrap()
    }

    /// Creates a minor scale
    pub fn minor() -> Self {
        Self::from_steps(Minor.as_steps()).unwrap()
    }

    /// Creates a harmonic minor scale
    pub fn harmonic_minor() -> Self {
        Self::from_steps(HarmonicMinor.as_steps()).unwrap()
    }

    /// Creates a melodic minor scale
    pub fn melodic_minor() -> Self {
        Self::from_steps(MelodicMinor.as_steps()).unwrap()
    }

    /// Creates a major pentatonic scale
    pub fn major_pentatonic() -> Self {
        Self::from_steps(MajorPentatonic.as_steps()).unwrap()
    }

    /// Creates a minor pentatonic scale
    pub fn minor_pentatonic() -> Self {
        Self::from_steps(MinorPentatonic.as_steps()).unwrap()
    }

    /// Creates a minor blues scale
    pub fn minor_blues() -> Self {
        Self::from_steps(MinorBlues.as_steps()).unwrap()
    }

    /// Creates a major blues scale
    pub fn major_blues() -> Self {
        Self::from_steps(MajorBlues.as_steps()).unwrap()
    }

    /// Creates a whole tone scale
    pub fn whole_tone() -> Self {
        Self::from_steps(WholeTone.as_steps()).unwrap()
    }

    /// Creates a diminished scale
    pub fn diminished() -> Self {
        Self::from_steps(Diminished.as_steps()).unwrap()
    }

    /// Creates a chromatic scale
    pub fn chromatic() -> Self {
        Self::from_steps(Chromatic.as_steps()).unwrap()
    }

    /// Returns the parent scale of the scale if it matches a known scale type or mode.
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, scales::*};
    ///
    /// assert_eq!(scale!("C D E F G A B C").unwrap().get_parent_scale_type(), Some((ScaleType::Major.into(), 0)));
    /// assert_eq!(scale!("C D E F# G A B C").unwrap().get_parent_scale_type(), Some((ScaleType::Major.into(), 4)));
    /// assert_eq!(scale!("C D E F# G# A B C").unwrap().get_parent_scale_type(), Some((ScaleType::MelodicMinor.into(), 5)));
    /// ```
    pub fn get_parent_scale_type(&self) -> Option<(ScaleEnumType, usize)> {
        // Creating an all rotations of current scale as a Vec
        let all_rotations: Vec<Scale> =
            (0..self.intervals.len()).map(|i| self.rotated(i as i8)).collect();

        // Checking if any of the rotations matches a known scale
        for (name, scale) in utils::KNOWN_SCALES {
            if all_rotations.contains(&scale()) {
                return Some((*name, all_rotations.iter().position(|s| s == &scale()).unwrap()));
            }
        }
        None
    }

    /// Returns true if the scale matches a known scale type
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, scales::*};
    ///
    /// assert!(scale!("C D E F G A B C").unwrap().is_known_scale());
    /// assert!(scale!("C D E F# G A B C").unwrap().is_known_scale());
    /// assert!(scale!("C D E F# G# A B C").unwrap().is_known_scale());
    /// assert!(scale!("C D E F G A Bb C").unwrap().is_known_scale());
    /// assert!(!scale!("C D E F G A Bbb C").unwrap().is_known_scale());
    /// ```
    pub fn is_known_scale(&self) -> bool {
        self.get_known_scale_type().is_some()
    }

    /// Returns the name of the scale if it matches a known scale type or mode
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, scales::*};
    ///
    /// assert_eq!(scale!("C D E F G A B C").unwrap().get_known_scale_type(), Some(ScaleType::Major.into()));
    /// assert_eq!(scale!("C D E F# G A B C").unwrap().get_known_scale_type(), Some(MajorMode::Lydian.into()));
    /// assert_eq!(scale!("C D E F# G# A B C").unwrap().get_known_scale_type(), Some(MelodicMinorMode::LydianAugmented.into()));
    /// assert_eq!(scale!("C D E F G A Bb C").unwrap().get_known_scale_type(), Some(MajorMode::Mixolydian.into()));
    /// assert_eq!(scale!("C D E F G A Bbb C").unwrap().get_known_scale_type(), None);
    /// ```
    pub fn get_known_scale_type(&self) -> Option<ScaleEnumType> {
        for scales in utils::ALL_SCALES {
            for (name, scale) in *scales {
                if self == &scale() {
                    return Some(*name);
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ScaleEnumType {
    ScaleType(ScaleType),
    MajorMode(MajorMode),
    HarmonicMinorMode(HarmonicMinorMode),
    MelodicMinorMode(MelodicMinorMode),
}

/// A non-exhaustive list of musical scales
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ScaleType {
    Major,
    Minor,
    HarmonicMinor,
    MelodicMinor,
    MajorPentatonic,
    MinorPentatonic,
    MinorBlues,
    MajorBlues,
    WholeTone,
    Diminished,
    Chromatic,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MajorMode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HarmonicMinorMode {
    HarmonicMinorRoot = 0,
    LocrianNat6,
    IonianAugmented,
    DorianSharp4,
    PhrygianDominant,
    LydianSharp2,
    SuperLocrian,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MelodicMinorMode {
    MelodicMinorRoot = 0,
    DorianFlat2,
    LydianAugmented,
    LydianDominant,
    AeolianDominant,
    HalfDiminished,
    Altered,
}
