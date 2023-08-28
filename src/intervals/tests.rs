#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::super::*;
    use crate::*;

    #[test]
    fn test_from_str_interval_quality() {
        use IntervalQuality as IQ;
        assert_eq!(IQ::from_str("M"), Ok(Major));
        assert_eq!(IQ::from_str("m"), Ok(Minor));
        assert_eq!(IQ::from_str("P"), Ok(Perfect));
        assert_eq!(IQ::from_str("A"), Ok(Augmented(1)));
        assert_eq!(IQ::from_str("d"), Ok(Diminished(1)));
        assert_eq!(IQ::from_str("Maj"), Ok(Major));
        assert_eq!(IQ::from_str("min"), Ok(Minor));
        assert_eq!(IQ::from_str("perf"), Ok(Perfect));
        assert_eq!(IQ::from_str("aug"), Ok(Augmented(1)));
        assert_eq!(IQ::from_str("dim"), Ok(Diminished(1)));
        assert_eq!(IQ::from_str("A++"), Ok(Augmented(3)));
        assert_eq!(IQ::from_str("d--"), Ok(Diminished(3)));
        assert_eq!(IQ::from_str("Aug"), Ok(Augmented(1)));
        assert_eq!(IQ::from_str("Dim"), Ok(Diminished(1)));
        assert_eq!(IQ::from_str("Major"), Ok(Major));
        assert_eq!(IQ::from_str("minor"), Ok(Minor));
        assert_eq!(IQ::from_str("Perfect"), Ok(Perfect));
        assert_eq!(IQ::from_str("augmented"), Ok(Augmented(1)));
        assert_eq!(IQ::from_str("diminished"), Ok(Diminished(1)));
        assert_eq!(IQ::from_str("Q"), err!(InvalidIntervalQuality));
    }

    #[test]
    fn test_build_valid_interval() {
        let interval = Interval::build(Perfect, Unison, 1).unwrap();
        assert_eq!(12, interval.semitones());
        
        let interval = Interval::build(Major, Third, 0).unwrap();
        assert_eq!(4, interval.semitones());
    }

    #[test]
    fn test_build_invalid_interval() {
        assert_eq!(err!(InvalidInterval), Interval::build(Perfect, Third, 0));
        assert_eq!(err!(InvalidInterval), Interval::build(Major, Unison, 0));
        assert_eq!(err!(InvalidInterval), Interval::build(Minor, Fourth, 0));
        assert_eq!(err!(InvalidInterval), Interval::build(Major, Fifth, 0));
        assert_eq!(err!(InvalidInterval), Interval::build(Perfect, Sixth, 0));
    }

    #[test]
    fn test_interval_macros() {
        assert_eq!(inv!(Perfect Unison), Interval::build(Perfect, Unison, 0));
        assert_eq!(inv!(Perfect Unison 1), Interval::build(Perfect, Unison, 1));
        assert_eq!(inv!(Major Third), Interval::build(Major, Third, 0));
        assert_eq!(inv!(Diminished(1) Fourth 2), Interval::build(Diminished(1), Fourth, 2));
        assert_eq!(inv!(Augmented(1) Fifth), Interval::build(Augmented(1), Fifth, 0));
    }

    #[test]
    fn test_valid_interval_from_str() {
        // Test building a valid interval from string
        let min_2nd = Interval::from_str("m2").unwrap();
        assert_eq!(min_2nd.quality, Minor);
        assert_eq!(min_2nd.size, Second);
        assert_eq!(min_2nd.octaves, 0);

        let dbl_aug_4th = Interval::from_str("++4").unwrap();
        assert_eq!(dbl_aug_4th.quality, Augmented(2));
        assert_eq!(dbl_aug_4th.size, Fourth);
        assert_eq!(dbl_aug_4th.octaves, 0);

        let perf_8ve = Interval::from_str("P8").unwrap();
        assert_eq!(perf_8ve.quality, Perfect);
        assert_eq!(perf_8ve.size, Unison);
        assert_eq!(perf_8ve.octaves, 1);

        let min_9th = Interval::from_str("m9").unwrap();
        assert_eq!(min_9th.quality, Minor);
        assert_eq!(min_9th.size, Second);
        assert_eq!(min_9th.octaves, 1);

        let dim_13th = Interval::from_str("d13").unwrap();
        assert_eq!(dim_13th.quality, Diminished(1));
        assert_eq!(dim_13th.size, Sixth);
        assert_eq!(dim_13th.octaves, 1);

        let perf_22nd = Interval::from_str("P22nd").unwrap();
        assert_eq!(perf_22nd.quality, Perfect);
        assert_eq!(perf_22nd.size, Unison);
        assert_eq!(perf_22nd.octaves, 3);
    }
}