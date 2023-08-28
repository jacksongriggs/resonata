#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::super::*;
    use crate::*;

    #[test]
    fn test_from_str_interval_quality() {
        assert_eq!(IntervalQuality::from_str("M"), Ok(IntervalQuality::Major));
        assert_eq!(IntervalQuality::from_str("m"), Ok(IntervalQuality::Minor));
        assert_eq!(IntervalQuality::from_str("P"), Ok(IntervalQuality::Perfect));
        assert_eq!(IntervalQuality::from_str("A"), Ok(IntervalQuality::Augmented(1)));
        assert_eq!(IntervalQuality::from_str("d"), Ok(IntervalQuality::Diminished(1)));
        assert_eq!(IntervalQuality::from_str("Maj"), Ok(IntervalQuality::Major));
        assert_eq!(IntervalQuality::from_str("min"), Ok(IntervalQuality::Minor));
        assert_eq!(IntervalQuality::from_str("perf"), Ok(IntervalQuality::Perfect));
        assert_eq!(IntervalQuality::from_str("aug"), Ok(IntervalQuality::Augmented(1)));
        assert_eq!(IntervalQuality::from_str("dim"), Ok(IntervalQuality::Diminished(1)));
        assert_eq!(IntervalQuality::from_str("A++"), Ok(IntervalQuality::Augmented(3)));
        assert_eq!(IntervalQuality::from_str("d--"), Ok(IntervalQuality::Diminished(3)));
        assert_eq!(IntervalQuality::from_str("Aug"), Ok(IntervalQuality::Augmented(1)));
        assert_eq!(IntervalQuality::from_str("Dim"), Ok(IntervalQuality::Diminished(1)));
        assert_eq!(IntervalQuality::from_str("Major"), Ok(IntervalQuality::Major));
        assert_eq!(IntervalQuality::from_str("minor"), Ok(IntervalQuality::Minor));
        assert_eq!(IntervalQuality::from_str("Perfect"), Ok(IntervalQuality::Perfect));
        assert_eq!(IntervalQuality::from_str("augmented"), Ok(IntervalQuality::Augmented(1)));
        assert_eq!(IntervalQuality::from_str("diminished"), Ok(IntervalQuality::Diminished(1)));
        assert_eq!(IntervalQuality::from_str("Q"), Err(IntervalError::InvalidIntervalQuality));
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
        assert_eq!(IntervalError::InvalidInterval, Interval::build(Perfect, Third, 0).unwrap_err());
        assert_eq!(IntervalError::InvalidInterval, Interval::build(Major, Unison, 0).unwrap_err());
        assert_eq!(IntervalError::InvalidInterval, Interval::build(Minor, Fourth, 0).unwrap_err());
        assert_eq!(IntervalError::InvalidInterval, Interval::build(Major, Fifth, 0).unwrap_err());
        assert_eq!(IntervalError::InvalidInterval, Interval::build(Perfect, Sixth, 0).unwrap_err());
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
        assert_eq!(min_2nd.quality, IntervalQuality::Minor);
        assert_eq!(min_2nd.size, IntervalSize::Second);
        assert_eq!(min_2nd.octaves, 0);

        let dbl_aug_4th = Interval::from_str("++4").unwrap();
        assert_eq!(dbl_aug_4th.quality, IntervalQuality::Augmented(2));
        assert_eq!(dbl_aug_4th.size, IntervalSize::Fourth);
        assert_eq!(dbl_aug_4th.octaves, 0);

        let perf_8ve = Interval::from_str("P8").unwrap();
        assert_eq!(perf_8ve.quality, IntervalQuality::Perfect);
        assert_eq!(perf_8ve.size, IntervalSize::Unison);
        assert_eq!(perf_8ve.octaves, 1);

        let min_9th = Interval::from_str("m9").unwrap();
        assert_eq!(min_9th.quality, IntervalQuality::Minor);
        assert_eq!(min_9th.size, IntervalSize::Second);
        assert_eq!(min_9th.octaves, 1);

        let dim_13th = Interval::from_str("d13").unwrap();
        assert_eq!(dim_13th.quality, IntervalQuality::Diminished(1));
        assert_eq!(dim_13th.size, IntervalSize::Sixth);
        assert_eq!(dim_13th.octaves, 1);

        let perf_22nd = Interval::from_str("P22nd").unwrap();
        assert_eq!(perf_22nd.quality, IntervalQuality::Perfect);
        assert_eq!(perf_22nd.size, IntervalSize::Unison);
        assert_eq!(perf_22nd.octaves, 3);
    }
}