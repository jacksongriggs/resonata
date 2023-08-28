pub use Accidental::*;

pub mod utils;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Accidental {
    Flat(u8),
    Natural,
    Sharp(u8),
}

impl Accidental {
    pub fn to_string(&self) -> String {
        match self {
            Flat(n) => "♭".repeat(*n as usize),
            Natural => "♮".to_string(),
            Sharp(n) => "♯".repeat(*n as usize),
        }
    }

    pub fn to_semitones(&self) -> i8 {
        match self {
            Flat(n) => -(*n as i8),
            Natural => 0,
            Sharp(n) => *n as i8,
        }
    }
}