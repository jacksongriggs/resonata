#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::*;

    #[test]
    fn test_to_u8() {
        assert_eq!(u8::from(inv!(Perfect, Unison).unwrap()), 0);
        assert_eq!(u8::from(inv!(Major, Second).unwrap()), 2);
        assert_eq!(u8::from(inv!(Augmented(1), Fourth).unwrap()), 6);
        assert_eq!(u8::from(inv!(Diminished(1), Fifth).unwrap()), 6);
    }

    #[test]
    fn test_build_valid_interval() {
        let interval = Interval::build(Perfect, Unison, 1).unwrap();
        assert_eq!(12, u8::from(interval));
        
        let interval = Interval::build(Major, Third, 0).unwrap();
        assert_eq!(4, u8::from(interval));
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
    fn test_from_str() {
        assert_eq!(inv!(Perfect, Unison).unwrap(), "P1".parse::<Interval>().unwrap());
        assert_eq!(inv!(Major, Second).unwrap(), "M2".parse::<Interval>().unwrap());
        assert_eq!(inv!(Augmented(1), Fourth).unwrap(), "A4".parse::<Interval>().unwrap());
        assert_eq!(inv!(Diminished(1), Fifth).unwrap(), "d5".parse::<Interval>().unwrap());
        assert_eq!(inv!(Augmented(1), Fourth).unwrap(), "6".parse::<Interval>().unwrap());

        let mut inv = inv!("2").unwrap();
        inv += 2;
    }

    #[test]
    fn test_interval_macros() {
        assert_eq!(inv!(Perfect, Unison), Interval::build(Perfect, Unison, 0));
        assert_eq!(inv!(Perfect, Unison, 1), Interval::build(Perfect, Unison, 1));
        assert_eq!(inv!(Major, Third), Interval::build(Major, Third, 0));
        assert_eq!(inv!(Diminished(1), Fourth, 2), Interval::build(Diminished(1), Fourth, 2));
        assert_eq!(inv!(Augmented(1), Fifth), Interval::build(Augmented(1), Fifth, 0));
    }

    #[test]
    fn test_cmp() {
        assert_eq!(inv!(Perfect, Unison), inv!(Perfect, Unison));
        assert!(inv!(Perfect, Unison).unwrap() < inv!(Major, Second).unwrap());
        assert!(inv!(Diminished(2), Fifth).unwrap() < inv!(Augmented(1), Fourth).unwrap());
        assert_eq!(inv!(Augmented(1), Fourth).unwrap(), inv!(Diminished(1), Fifth).unwrap());
    }

    #[test]
    fn test_ops() {
        assert_eq!(inv!(Perfect, Unison).unwrap() + 2, inv!(Major, Second).unwrap());
        assert_eq!(inv!(Perfect, Unison).unwrap() - inv!(Major, Second).unwrap(), inv!(Major, Second).unwrap());
        assert_eq!(inv!(Major, Third).unwrap() - inv!(Minor, Third).unwrap(), inv!(Minor, Second).unwrap());
        assert_eq!(inv!(Major, Third).unwrap() + inv!(Minor, Third).unwrap(), inv!(Perfect, Fifth).unwrap());
        assert_eq!(Interval::from(10) + inv!(Major, Second).unwrap(), inv!(Perfect, Unison, 1).unwrap());
    }

    #[test]
    fn test_inversion() {
        let interval = inv!(Major, Second).unwrap();
        assert_eq!(inv!(Minor, Seventh).unwrap(), interval.inverted());

        let interval = inv!(Augmented(1), Fourth).unwrap();
        assert_eq!(inv!(Diminished(1), Fifth).unwrap(), interval.inverted());

        let interval = inv!(Perfect, Fifth).unwrap();
        assert_eq!(inv!(Perfect, Fourth).unwrap(), interval.inverted());

        let interval = inv!(Major, Sixth, 1).unwrap();
        assert_eq!(inv!(Minor, Third, 1).unwrap(), interval.inverted());
    }
}