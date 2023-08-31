use super::super::*;

mod ops;
mod fmt;

impl From<u8> for NoteName {
    fn from(value: u8) -> Self {
        match value % 12 {
            0 | 1 => C,
            2 | 3 => D,
            4 => E,
            5 | 6 => F,
            7 | 8 => G,
            9 | 10 => A,
            11 => B,
            _ => unreachable!(),
        }
    }
}

impl From<NoteName> for u8 {
    fn from(name: NoteName) -> Self {
        match name {
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