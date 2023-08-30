#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::{*, notes::NoteName::*};

    #[test]
    fn test_from_steps() {
        let scale = Scale::major();
        let intervals = scale.intervals();
        let expected_intervals = vec![
            Interval::from(2),
            Interval::from(2),
            Interval::from(1),
            Interval::from(2),
            Interval::from(2),
            Interval::from(2),
            Interval::from(1),
        ];
        assert_eq!(*intervals, expected_intervals);
    }

    #[test]
    fn test_to_steps() {
        let steps = Scale::major().to_steps();
        let expected_steps = vec![2, 2, 1, 2, 2, 2, 1];
        assert_eq!(steps, expected_steps);
    }

    #[test]
    fn test_from_notes() {
        let notes = vec![
            Note::from(60),
            Note::from(62),
            Note::from(64),
            Note::from(65),
            Note::from(67),
            Note::from(69),
            Note::from(71),
        ];
        let expected_notes = Scale::major().to_notes(note!(C));
        assert_eq!(notes, expected_notes);
    }

    #[test]
    fn test_to_notes() {
        let scale = Scale::major();
        let expected_notes = vec![
            Note::from(60),
            Note::from(62),
            Note::from(64),
            Note::from(65),
            Note::from(67),
            Note::from(69),
            Note::from(71),
        ];
        assert_eq!(scale.to_notes(note!(C)), expected_notes);
        println!("{}", scale);
    }

    #[test]
    fn test_scale_types() {
        assert_eq!(vec![
            note!(C), 
            note!(D), 
            note!(E), 
            note!(F), 
            note!(G), 
            note!(A), 
            note!(B)], 
            Scale::major().to_notes(note!(C)));
        assert_eq!(Scale::from_steps(vec![2, 1, 2, 2, 1, 2, 2]), Scale::minor());
        assert_eq!(Scale::from_steps(vec![1; 12]), Scale::chromatic());
    }

    #[test]
    fn test_rotation() {
        let scale = Scale::major().rotated(1);
        let expected_scale = Scale::from_steps(vec![2, 1, 2, 2, 2, 1, 2]);
        assert_eq!(scale, expected_scale);
    }

    #[test]
    fn test_macros() {
        assert_eq!(Scale::major(), scale!(Major, 0));
        assert_eq!(Scale::minor(), scale!(Minor, 0));
        
    }
}