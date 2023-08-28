pub mod intervals;
pub mod notes;
pub mod scales;
pub mod chords;
pub mod error;

pub use intervals::Interval;
pub use notes::Note;
pub use scales::Scale;
pub use error::MTError;
// pub use chords::Chord;