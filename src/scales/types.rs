use std::fmt::{self, Display, Formatter};
pub use ScaleType::*;

/// A non-exhaustive list of musical scales
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ScaleType {
    Major,
    Minor,
    HarmonicMinor,
    MelodicMinor,
    Pentatonic,
    MinorBlues,
    MajorBlues,
    WholeTone,
    Diminished,
    Chromatic,
}

impl ScaleType {
    pub fn to_steps(&self) -> Vec<i8> {
        match self {
            Major => vec![2, 2, 1, 2, 2, 2, 1],
            Minor => vec![2, 1, 2, 2, 1, 2, 2],
            HarmonicMinor => vec![2, 1, 2, 2, 1, 3, 1],
            MelodicMinor => vec![2, 1, 2, 2, 2, 2, 1],
            Pentatonic => vec![2, 2, 3, 2, 3],
            MinorBlues => vec![3, 2, 1, 1, 3, 2],
            MajorBlues => vec![2, 1, 1, 3, 2, 3],
            WholeTone => vec![2, 2, 2, 2, 2, 2],
            Diminished => vec![2, 1, 2, 1, 2, 1, 2, 1],
            Chromatic => vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
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
            Pentatonic => "Pentatonic",
            MinorBlues => "Minor Blues",
            MajorBlues => "Major Blues",
            WholeTone => "Whole Tone",
            Diminished => "Diminished",
            Chromatic => "Chromatic",
        };

        write!(f, "{}", token)
    }
}
