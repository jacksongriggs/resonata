use std::{
    fmt::{self, Display, Formatter},
    vec,
};
pub use ScaleType::*;
pub use MajorMode::*;
pub use HarmonicMinorMode::*;
pub use MelodicMinorMode::*;

use super::Scale;

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

impl ScaleType {
    pub fn as_steps(&self) -> Vec<i32> {
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

impl MajorMode {
    pub fn to_steps(&self) -> Vec<i32> {
        let mut mode = ScaleType::Major.as_steps();
        mode.rotate_left(*self as usize);
        mode
    }
}

impl HarmonicMinorMode {
    pub fn to_steps(&self) -> Vec<i32> {
        let mut mode = ScaleType::HarmonicMinor.as_steps();
        mode.rotate_left(*self as usize);
        mode
    }
}

type NamedScale = (ScaleEnumType, fn() -> Scale);

macro_rules! as_enum {
    ($enum_type:ident, $variant:ident) => {
        ScaleEnumType::$enum_type($enum_type::$variant)
    };
}

pub static KNOWN_SCALES: &[NamedScale] = &[
    (as_enum!(ScaleType, Major), Scale::major),
    (as_enum!(ScaleType, Minor), Scale::minor),
    (as_enum!(ScaleType, HarmonicMinor), Scale::harmonic_minor),
    (as_enum!(ScaleType, MelodicMinor), Scale::melodic_minor),
    (as_enum!(ScaleType, MajorPentatonic), Scale::major_pentatonic),
    (as_enum!(ScaleType, MinorPentatonic), Scale::minor_pentatonic),
    (as_enum!(ScaleType, MinorBlues), Scale::minor_blues),
    (as_enum!(ScaleType, MajorBlues), Scale::major_blues),
    (as_enum!(ScaleType, WholeTone), Scale::whole_tone),
    (as_enum!(ScaleType, Diminished), Scale::diminished),
    (as_enum!(ScaleType, Chromatic), Scale::chromatic),
];

pub static MAJOR_MODES: &[NamedScale] = &[
    (as_enum!(MajorMode, Ionian), Scale::major),
    (as_enum!(MajorMode, Dorian), || Scale::major().rotated(1)),
    (as_enum!(MajorMode, Phrygian), || Scale::major().rotated(2)),
    (as_enum!(MajorMode, Lydian), || Scale::major().rotated(3)),
    (as_enum!(MajorMode, Mixolydian), || Scale::major().rotated(4)),
    (as_enum!(MajorMode, Aeolian), || Scale::major().rotated(5)),
    (as_enum!(MajorMode, Locrian), || Scale::major().rotated(6)),
];

pub static HARMONIC_MINOR_MODES: &[NamedScale] = &[
    (as_enum!(HarmonicMinorMode, HarmonicMinorRoot), Scale::harmonic_minor),
    (as_enum!(HarmonicMinorMode, LocrianNat6), || Scale::harmonic_minor().rotated(1)),
    (as_enum!(HarmonicMinorMode, IonianAugmented), || Scale::harmonic_minor().rotated(2)),
    (as_enum!(HarmonicMinorMode, DorianSharp4), || Scale::harmonic_minor().rotated(3)),
    (as_enum!(HarmonicMinorMode, PhrygianDominant), || Scale::harmonic_minor().rotated(4)),
    (as_enum!(HarmonicMinorMode, LydianSharp2), || Scale::harmonic_minor().rotated(5)),
    (as_enum!(HarmonicMinorMode, SuperLocrian), || Scale::harmonic_minor().rotated(6)),
];

pub static MELODIC_MINOR_MODES: &[NamedScale] = &[
    (as_enum!(MelodicMinorMode, MelodicMinorRoot), Scale::melodic_minor),
    (as_enum!(MelodicMinorMode, DorianFlat2), || Scale::melodic_minor().rotated(1)),
    (as_enum!(MelodicMinorMode, LydianAugmented), || Scale::melodic_minor().rotated(2)),
    (as_enum!(MelodicMinorMode, LydianDominant), || Scale::melodic_minor().rotated(3)),
    (as_enum!(MelodicMinorMode, AeolianDominant), || Scale::melodic_minor().rotated(4)),
    (as_enum!(MelodicMinorMode, HalfDiminished), || Scale::melodic_minor().rotated(5)),
    (as_enum!(MelodicMinorMode, Altered), || Scale::melodic_minor().rotated(6)),
];

// The list of all things to check
pub static ALL_SCALES: &[&[NamedScale]] =
    &[KNOWN_SCALES, MAJOR_MODES, HARMONIC_MINOR_MODES, MELODIC_MINOR_MODES];
