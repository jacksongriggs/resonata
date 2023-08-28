#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::super::*;
    use crate::*;

    #[test]
    fn test_new_valid_note() {
        assert_eq!(60, Note::new(60).unwrap().number());
        assert_eq!(127, Note::new(127).unwrap().number());
    }

    #[test]
    fn test_new_invalid_note() {
        assert_eq!(err!(InvalidNote), Note::new(128));
        assert_eq!(err!(InvalidNote), Note::new(255));
    }

    #[test]
    fn test_build_valid_note() {
        assert_eq!(0, Note::build(C, Natural, -1).unwrap().number());
        assert_eq!(60, Note::build(C, Natural, 4).unwrap().number());
        assert_eq!(74, Note::build(D, Natural, 5).unwrap().number());
        assert_eq!(127, Note::build(G, Natural, 9).unwrap().number());
    }
    
    #[test]
    fn test_build_invalid_note() {
        assert_eq!(err!(InvalidNote), Note::build(G, Sharp(1), 9)); // Too high
        assert_eq!(err!(InvalidNote), Note::build(C, Sharp(127), 4)); // Too high
        assert_eq!(err!(InvalidOctave), Note::build(C, Flat(1), -2)); // Too low
    }

    #[test]
    fn test_from_str_valid_notes() {
        // Test valid note strings
        assert_eq!(Note::build(C, Natural, 4), Note::from_str("C"));
        assert_eq!(Note::build(D, Natural, -1), Note::from_str("D-1"));
        assert_eq!(Note::build(E, Natural, 0), Note::from_str("E♮0"));
        assert_eq!(Note::build(F, Sharp(1), 1), Note::from_str("F#1"));
        assert_eq!(Note::build(G, Sharp(2), 2), Note::from_str("Gx2"));
        assert_eq!(Note::build(A, Flat(1), 3), Note::from_str("Ab3"));
        assert_eq!(Note::build(B, Flat(2), 4), Note::from_str("Bbb4"));
        assert_eq!(Note::build(C, Flat(3), 5), Note::from_str("cbbb5"));
        assert_eq!(Note::build(D, Sharp(3), 6), Note::from_str("d###6"));
        assert_eq!(Note::build(E, Sharp(2), 7), Note::from_str("e𝄪7"));
        assert_eq!(Note::build(F, Sharp(3), 8), Note::from_str("f#x8"));
        assert_eq!(Note::build(G, Sharp(4), 4), Note::from_str("g𝄪x"));
    }

    #[test]
    fn test_from_str_invalid_notes() {
        // Test invalid note strings
        assert_eq!(err!(InvalidNoteName), Note::from_str("")); // Empty string
        assert_eq!(err!(InvalidNoteName), Note::from_str("X")); // Invalid note name
        assert_eq!(err!(InvalidAccidental), Note::from_str("C#b")); // Conflicting accidentals
        assert_eq!(err!(InvalidOctave), Note::from_str("C-4")); // Invalid octave (too low)
        assert_eq!(err!(InvalidOctave), Note::from_str("B#10")); // Invalid octave (too high)
        assert_eq!(err!(InvalidNoteName), Note::from_str("C-#4")); // Negative
    }

    #[test]
    fn test_macros() {
        assert_eq!(note!(C), Note::build(C, Natural, 4));
        assert_eq!(note!(C, Flat(1)), Note::build(C, Flat(1), 4));
        assert_eq!(note!(C, 4), Note::build(C, Natural, 4));
        assert_eq!(note!(C, Flat(1), 4), Note::build(C, Flat(1), 4));
        assert_eq!(note!(E, Sharp(2), -1), Note::build(E, Sharp(2), -1));
        assert_eq!(note!("C4"), Note::build(C, Natural, 4));
    }
}