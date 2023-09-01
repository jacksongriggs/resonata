#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::*;

    #[test]
    fn test_new_valid_note() {
        assert_eq!(60, u8::from(PitchedNote::from(60)));
        assert_eq!(127, u8::from(PitchedNote::from(127)));
    }

    #[test]
    fn test_build_valid_note() {
        assert_eq!(0, u8::from(PitchedNote::new(C, Natural, -1)));
        assert_eq!(60, u8::from(PitchedNote::new(C, Natural, 4)));
        assert_eq!(74, u8::from(PitchedNote::new(D, Natural, 5)));
        assert_eq!(127, u8::from(PitchedNote::new(G, Natural, 9)));
    }

    #[test]
    fn test_from_str_valid_notes() {
        // Test valid note strings
        assert_eq!(
            PitchedNote::new(C, Natural, 4),
            PitchedNote::from_str("C").unwrap()
        );
        assert_eq!(
            PitchedNote::new(D, Natural, -1),
            PitchedNote::from_str("D-1").unwrap()
        );
        assert_eq!(
            PitchedNote::new(E, Natural, 0),
            PitchedNote::from_str("E♮0").unwrap()
        );
        assert_eq!(
            PitchedNote::new(F, Sharp(1), 1),
            PitchedNote::from_str("F#1").unwrap()
        );
        assert_eq!(
            PitchedNote::new(G, Sharp(2), 2),
            PitchedNote::from_str("Gx2").unwrap()
        );
        assert_eq!(
            PitchedNote::new(A, Flat(1), 3),
            PitchedNote::from_str("Ab3").unwrap()
        );
        assert_eq!(
            PitchedNote::new(B, Flat(2), 4),
            PitchedNote::from_str("Bbb4").unwrap()
        );
        assert_eq!(
            PitchedNote::new(C, Flat(3), 5),
            PitchedNote::from_str("cbbb5").unwrap()
        );
        assert_eq!(
            PitchedNote::new(D, Sharp(3), 6),
            PitchedNote::from_str("d###6").unwrap()
        );
        assert_eq!(
            PitchedNote::new(E, Sharp(2), 7),
            PitchedNote::from_str("e𝄪7").unwrap()
        );
        assert_eq!(
            PitchedNote::new(F, Sharp(3), 8),
            PitchedNote::from_str("f#x8").unwrap()
        );
        assert_eq!(
            PitchedNote::new(G, Sharp(4), 4),
            PitchedNote::from_str("g𝄪x").unwrap()
        );
    }

    #[test]
    fn test_from_str_invalid_notes() {
        // Test invalid note strings
        assert_eq!(err!(InvalidNoteName), PitchedNote::from_str("")); // Empty string
        assert_eq!(err!(InvalidNoteName), PitchedNote::from_str("X")); // Invalid note name
        assert_eq!(
            err!(InvalidAccidentalCombination),
            PitchedNote::from_str("C#b")
        ); // Conflicting accidentals
        assert_eq!(err!(InvalidOctave), PitchedNote::from_str("C-4")); // Invalid octave (too low)
        assert_eq!(err!(InvalidOctave), PitchedNote::from_str("B#10")); // Invalid octave (too high)
        assert_eq!(err!(InvalidNoteName), PitchedNote::from_str("C-#4")); // Invalid syntax
    }

    #[test]
    fn test_macros() {
        assert_eq!(note!(C), Note::new(C, Natural));
        assert_eq!(note!(C, Flat(1)), Note::new(C, Flat(1)));
        assert_eq!(pnote!(C, 4), PitchedNote::new(C, Natural, 4));
        assert_eq!(pnote!(C, Flat(1), 4), PitchedNote::new(C, Flat(1), 4));
        assert_eq!(pnote!(E, Sharp(2), -1), PitchedNote::new(E, Sharp(2), -1));
        assert_eq!(pnote!("C4").unwrap(), PitchedNote::new(C, Natural, 4));
        assert_eq!(pnote!(C, -1), PitchedNote::from(0));
        assert_eq!(pnote!(G, 9), PitchedNote::from(127));
    }

    #[test]
    fn test_cmp() {
        assert_eq!(pnote!(C), pnote!(C));
        assert_ne!(pnote!(C), pnote!(D));
        assert!(pnote!(C) < pnote!(D));
        assert!(pnote!(C) <= pnote!(D));
        assert!(pnote!(D) > pnote!(C));
        assert!(pnote!(D) >= pnote!(C));
    }

    #[test]
    fn test_ops() {
        use crate::intervals::IntervalQuality::*;
        use crate::intervals::IntervalSize::*;
        assert_eq!(pnote!(C, 4) + 2, pnote!(D, 4));
        assert_eq!(pnote!(C, 4) - pnote!(D, 4), inv!(Major, Second).unwrap());
        assert_eq!(
            pnote!(C, 4) - pnote!(F, Sharp(1), 4),
            inv!(Augmented(1), Fourth).unwrap()
        );
        assert_eq!(pnote!(C, -1) - pnote!(G, 9), Interval::from(127));
    }
}
