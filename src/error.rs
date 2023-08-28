use thiserror::Error;
use std::fmt::{self, Display, Formatter};
use std::error::Error;

pub use IntervalError::*;
pub use NoteError::*;
pub use ScaleError::*;

#[derive(Debug, PartialEq, Eq)]
pub enum IntervalError {
    InvalidInterval,
    InvalidIntervalQuality,
    InvalidIntervalSize,
    InvalidIntervalFormat,
}

impl Error for IntervalError {}

impl Display for IntervalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            InvalidInterval => "Invalid interval",
            InvalidIntervalQuality => "Invalid interval quality",
            InvalidIntervalSize => "Invalid interval size",
            InvalidIntervalFormat => "Invalid interval format",
        };

        write!(f, "{}", token)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NoteError {
    InvalidNote,
    InvalidNoteName,
    InvalidAccidental,
    InvalidOctave,
}

impl Error for NoteError {}

impl Display for NoteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            InvalidNote => "Invalid note",
            InvalidNoteName => "Invalid note name",
            InvalidAccidental => "Invalid accidental",
            InvalidOctave => "Invalid octave",
        };

        write!(f, "{}", token)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScaleError {
    InvalidScale,
    NoRootSpecified,
}

impl Error for ScaleError {}

impl Display for ScaleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = match self {
            InvalidScale => "Invalid scale",
            NoRootSpecified => "No root specified",
        };

        write!(f, "{}", token)
    }
}

#[derive(Debug, Error)]
pub enum MTError {
    NoteError(#[from] NoteError),
    IntervalError(#[from] IntervalError),
    ScaleError(#[from] ScaleError),
}

impl Display for MTError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use MTError::*;

        let token = match self {
            NoteError(e) => e.to_string(),
            IntervalError(e) => e.to_string(),
            ScaleError(e) => e.to_string(),
        };

        write!(f, "{}", token)
    }
}