#[cfg(test)]
mod tests {
    use crate::*;
    use crate::intervals::size::IntervalSize;
    use IntervalSize::*;

    #[test]
    fn test_from_str_interval_size() {
        use IntervalSize as IS;
        assert_eq!(IS::from_str("1st"), Ok(Unison));
        assert_eq!(IS::from_str("2nd"), Ok(Second));
        assert_eq!(IS::from_str("3rd"), Ok(Third));
        assert_eq!(IS::from_str("4th"), Ok(Fourth));
        assert_eq!(IS::from_str("5"), Ok(Fifth));
        assert_eq!(IS::from_str("6th"), Ok(Sixth));
        assert_eq!(IS::from_str("7"), Ok(Seventh));
        assert_eq!(IS::from_str("8ve"), Ok(Unison));
        assert_eq!(IS::from_str("9"), Ok(Second));
        assert_eq!(IS::from_str("10th"), Ok(Third));
        assert_eq!(IS::from_str("11"), Ok(Fourth));
        assert_eq!(IS::from_str("12th"), Ok(Fifth));
        assert_eq!(IS::from_str("13"), Ok(Sixth));
        assert_eq!(IS::from_str("14"), Ok(Seventh));
        assert_eq!(IS::from_str("15"), Ok(Unison));
    }

    #[test]
    fn test_to_diatonic_semitones() {
        assert_eq!(Unison.to_diatonic_semitones(), 0);
        assert_eq!(Second.to_diatonic_semitones(), 2);
        assert_eq!(Third.to_diatonic_semitones(), 4);
        assert_eq!(Fourth.to_diatonic_semitones(), 5);
        assert_eq!(Fifth.to_diatonic_semitones(), 7);
        assert_eq!(Sixth.to_diatonic_semitones(), 9);
        assert_eq!(Seventh.to_diatonic_semitones(), 11);
    }

    #[test]
    fn test_invert() {
        assert_eq!(Unison.invert(), Unison);
        assert_eq!(Second.invert(), Seventh);
        assert_eq!(Third.invert(), Sixth);
        assert_eq!(Fourth.invert(), Fifth);
        assert_eq!(Fifth.invert(), Fourth);
        assert_eq!(Sixth.invert(), Third);
        assert_eq!(Seventh.invert(), Second);
    }

    #[test]
    fn test_cmp() {
        assert_eq!(Unison, Unison);
        assert_ne!(Unison, Second);
        assert!(Unison < Second);
    }

    #[test]
    fn test_ops() {
        assert_eq!(Unison + 1, Second);
        assert_eq!(Unison - Second, Second);
        assert_eq!(Second - Second, Unison);
        assert_eq!(Second + Second, Third);
        assert_eq!(Third + Third, Fifth);
        assert_eq!(Unison + 7, Unison);
    }
}