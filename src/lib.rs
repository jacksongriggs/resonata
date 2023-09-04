// TODO: pub mod chords;
pub mod error;
pub mod intervals;
pub mod keys;
pub mod notes;
pub mod scales;

// TODO: pub use chords::Chord;
pub use intervals::Interval;

pub trait TransposeUp {
    type Output;
    fn transposed_up(&self, interval: Interval) -> Self::Output;
}

pub trait TransposeDown {
    type Output;
    fn transposed_down(&self, interval: Interval) -> Self::Output;
}
