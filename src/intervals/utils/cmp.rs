use super::*;

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        i8::from(*self) == i8::from(*other)
    }
}

impl Eq for Interval {}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(i8::from(*self).cmp(&i8::from(*other)))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        i8::from(*self).cmp(&i8::from(*other))
    }
}
