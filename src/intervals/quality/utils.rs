use super::super::*;

mod cmp;
mod fmt;
mod tests;

impl From<IntervalQuality> for i8 {
    fn from(iq: IntervalQuality) -> Self {
        match iq {
            Diminished(n) => -(n as i8) - 1,
            Augmented(n) => n as i8,
            Minor => -1,
            Major => 0,
            Perfect => 0,
        }
    }
}