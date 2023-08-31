use std::cmp;
use super::*;

impl cmp::PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        i8::from(*self) == i8::from(*other)
    }
}

impl cmp::Eq for Interval {}

impl cmp::PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(i8::from(*self).cmp(&i8::from(*other)))
    }
}

impl cmp::Ord for Interval {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        i8::from(*self).cmp(&i8::from(*other))
    }
}