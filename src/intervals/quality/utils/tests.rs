
#[cfg(test)]
mod tests {
    use crate::*;
    use crate::intervals::quality::IntervalQuality;
    use IntervalQuality::*;

    #[test]
    fn test_invert() {
        
        assert_eq!(Diminished(1).invert(), Augmented(1));
        assert_eq!(Augmented(1).invert(), Diminished(1));
        assert_eq!(Minor.invert(), Major);
        assert_eq!(Major.invert(), Minor);
        assert_eq!(Perfect.invert(), Perfect);
    }

    #[test]
    fn test_from_str_interval_quality() {
        use IntervalQuality as IQ;
        use IntervalQuality::*;
        assert_eq!(IQ::from_str("M"), Ok(Major));
        assert_eq!(IQ::from_str("m"), Ok(Minor));
        assert_eq!(IQ::from_str("P"), Ok(Perfect));
        assert_eq!(IQ::from_str("A"), Ok(Augmented(1)));
        assert_eq!(IQ::from_str("d"), Ok(Diminished(1)));
        assert_eq!(IQ::from_str("Q"), err!(error::InvalidIntervalQuality));
    }

    #[test]
    fn test_to_i8() {
        assert_eq!(i8::from(Diminished(3)), -4);
        assert_eq!(i8::from(Diminished(2)), -3);
        assert_eq!(i8::from(Diminished(1)), -2);
        assert_eq!(i8::from(Minor), -1);
        assert_eq!(i8::from(Major), 0);
        assert_eq!(i8::from(Perfect), 0);
        assert_eq!(i8::from(Augmented(1)), 1);
        assert_eq!(i8::from(Augmented(2)), 2);
        assert_eq!(i8::from(Augmented(3)), 3);
    }

    #[test]
    fn test_cmp() {
        assert_eq!(Diminished(3).cmp(&Diminished(3)), std::cmp::Ordering::Equal);
        assert_eq!(Diminished(2).cmp(&Diminished(1)), std::cmp::Ordering::Less);
        assert_eq!(Minor.cmp(&Diminished(4)), std::cmp::Ordering::Greater);
        assert_eq!(Minor.cmp(&Major), std::cmp::Ordering::Less);
        assert_eq!(Major.cmp(&Augmented(1)), std::cmp::Ordering::Less);
        assert_eq!(Augmented(1).cmp(&Perfect), std::cmp::Ordering::Greater);
        assert_eq!(Perfect.cmp(&Major), std::cmp::Ordering::Equal);
        assert_eq!(Diminished(3).cmp(&Augmented(2)), std::cmp::Ordering::Less);
    }
}