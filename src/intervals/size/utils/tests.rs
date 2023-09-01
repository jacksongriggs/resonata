#[cfg(test)]
mod tests {
    use crate::intervals::size::IntervalSize;
    use crate::*;
    use IntervalSize::*;

    #[test]
    fn test_valid_from_str() {
        assert_eq!(Unison, IntervalSize::from_str("U").unwrap());
        assert_eq!(Second, IntervalSize::from_str("2").unwrap());
        assert_eq!(Third, IntervalSize::from_str("3").unwrap());
        assert_eq!(Fourth, IntervalSize::from_str("4").unwrap());
        assert_eq!(Fifth, IntervalSize::from_str("5").unwrap());
        assert_eq!(Sixth, IntervalSize::from_str("6").unwrap());
        assert_eq!(Seventh, IntervalSize::from_str("7").unwrap());
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
