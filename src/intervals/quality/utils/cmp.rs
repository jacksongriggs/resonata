use super::*;

impl PartialEq for IntervalQuality {
    fn eq(&self, other: &Self) -> bool {
        i8::from(*self) == i8::from(*other)
    }
}

impl Eq for IntervalQuality {}

impl PartialOrd for IntervalQuality {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        i8::from(*self).partial_cmp(&i8::from(*other))
    }
}

impl Ord for IntervalQuality {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        i8::from(*self).cmp(&i8::from(*other))
    }
}
