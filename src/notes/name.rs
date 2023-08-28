use super::*;
pub mod utils;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NoteName { 
    C, D, E, F, G, A, B, 
}

impl NoteName {
    pub fn to_string(&self) -> String {
        match self {
            C => "C".to_string(),
            D => "D".to_string(),
            E => "E".to_string(),
            F => "F".to_string(),
            G => "G".to_string(),
            A => "A".to_string(),
            B => "B".to_string(),
        }
    }

    pub fn to_chromatic_number (&self) -> u8 {
        match self {
            C => 0,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
            A => 9,
            B => 11,
        }
    }
}

