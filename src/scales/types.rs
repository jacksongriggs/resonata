use std::{
    fmt::{self, Display, Formatter},
    vec,
};
pub use ScaleType::*;

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
pub enum MajorModes {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HarmonicMinorModes {
    LocrianNat6 = 1,
    IonianAugmented,
    DorianSharp4,
    PhrygianDominant,
    LydianSharp2,
    SuperLocrian,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MelodicMinorModes {
    DorianFlat2 = 1,
    LydianAugmented,
    LydianDominant,
    AeolianDominant,
    HalfDiminished,
    Altered,
}

impl ScaleType {
    pub fn as_steps(&self) -> Vec<u8> {
        match self {
            Major => vec![2, 2, 1, 2, 2, 2, 1],
            Minor => vec![2, 1, 2, 2, 1, 2, 2],
            HarmonicMinor => vec![2, 1, 2, 2, 1, 3, 1],
            MelodicMinor => vec![2, 1, 2, 2, 2, 2, 1],
            MajorPentatonic => vec![2, 2, 3, 2, 3],
            MinorPentatonic => vec![3, 2, 2, 3, 2],
            MinorBlues => vec![3, 2, 1, 1, 3, 2],
            MajorBlues => vec![2, 1, 1, 3, 2, 3],
            WholeTone => vec![2; 6],
            Diminished => vec![2, 1, 2, 1, 2, 1, 2, 1],
            Chromatic => vec![1; 12],
        }
    }
}

impl Display for ScaleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            Major => "Major",
            Minor => "Minor",
            HarmonicMinor => "Harmonic Minor",
            MelodicMinor => "Melodic Minor",
            MajorPentatonic => "Major Pentatonic",
            MinorPentatonic => "Minor Pentatonic",
            MinorBlues => "Minor Blues",
            MajorBlues => "Major Blues",
            WholeTone => "Whole Tone",
            Diminished => "Diminished",
            Chromatic => "Chromatic",
        };

        write!(f, "{}", token)
    }
}

impl MajorModes {
    pub fn to_steps(&self) -> Vec<u8> {
        let mut mode = ScaleType::Major.as_steps();
        mode.rotate_left(*self as usize);
        mode
    }
}

impl HarmonicMinorModes {
    pub fn to_steps(&self) -> Vec<u8> {
        let mut mode = ScaleType::HarmonicMinor.as_steps();
        mode.rotate_left(*self as usize);
        mode
    }
}
