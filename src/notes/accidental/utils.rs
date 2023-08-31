use super::super::*;

mod fmt;
mod ops;
mod tests;

impl From<i8> for Accidental {
    fn from(value: i8) -> Self {
        if value < 0 {
            Flat(value.abs() as u8)
        } else if value == 0 {
            Natural
        } else {
            Sharp(value as u8)
        }
    }
}

impl From<Accidental> for i8 {
    fn from(acc: Accidental) -> Self {
        match acc {
            Flat(n) => -(n as i8),
            Natural => 0,
            Sharp(n) => n as i8,
        }
    }
}