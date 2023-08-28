use thiserror::Error;

pub use IntervalError::*;
pub use NoteError::*;
pub use ScaleError::*;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum IntervalError {
    #[error("Invalid interval")]
    InvalidInterval,
    #[error("Invalid interval quality")]
    InvalidIntervalQuality,
    #[error("Invalid interval size")]
    InvalidIntervalSize,
    #[error("Invalid interval format")]
    InvalidIntervalFormat,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum NoteError {
    #[error("Invalid note")]
    InvalidNote,
    #[error("Invalid note name")]
    InvalidNoteName,
    #[error("Invalid note format")]
    InvalidAccidental,
    #[error("Invalid note format")]
    InvalidOctave,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ScaleError {
    #[error("Invalid scale")]
    InvalidScale,
    #[error("Invalid scale format")]
    NoRootSpecified,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ResonataError {
    #[error("Note error: {0}")]
    NoteError(#[from] NoteError),
    #[error("Interval error: {0}")]
    IntervalError(#[from] IntervalError),
    #[error("Scale error: {0}")]
    ScaleError(#[from] ScaleError),
}

/// A macro to create a `ResonataError` from a `NoteError`, `IntervalError` or `ScaleError`
#[macro_export]
macro_rules! err {
    ($e:expr) => {
        Err(ResonataError::from($e))
    };
}

/// A macro to retur
#[macro_export]
macro_rules! nope {
    ($e:expr) => {
        return Err(ResonataError::from($e))
    };
}