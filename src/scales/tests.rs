#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::{*, notes::NoteName::*};

    #[test]
    fn test_from_steps() {
        let scale = Scale::major(None);
        let intervals = scale.intervals();
        let expected_intervals = vec![
            Interval::new(2),
            Interval::new(4),
            Interval::new(5),
            Interval::new(7),
            Interval::new(9),
            Interval::new(11),
            Interval::new(12),
        ];
        assert_eq!(*intervals, expected_intervals);
    }

    #[test]
    fn test_to_steps() {
        let steps = Scale::major(None).to_steps();
        let expected_steps = vec![2, 2, 1, 2, 2, 2, 1];
        assert_eq!(steps, expected_steps);
    }

    #[test]
    fn test_from_notes() {
        let scale = Scale::from_notes(vec![
            Note::new(60),
            Note::new(62),
            Note::new(64),
            Note::new(65),
            Note::new(67),
            Note::new(69),
            Note::new(71),
            Note::new(72),
        ]);
        let expected_scale = Scale::major(Some(note!(C)));
        println!("{}", scale);
        assert_eq!(scale, expected_scale);
    }

    #[test]
    fn test_to_notes() {
        let scale = Scale::major(Some(Note::new(60)));
        let expected_notes = vec![
            Note::new(60),
            Note::new(62),
            Note::new(64),
            Note::new(65),
            Note::new(67),
            Note::new(69),
            Note::new(71),
            Note::new(72),
        ];
        assert_eq!(scale.to_notes(), expected_notes);
        println!("{}", scale);
    }

    #[test]
    fn test_scale_types() {
        assert_eq!(vec![note!(C), note!(D), note!(E), note!(F), note!(G), note!(A), note!(B)], Scale::major(None).to_notes());
        assert_eq!(Scale::from_steps(None, vec![2, 1, 2, 2, 1, 2, 2]), Scale::minor(None));
        assert_eq!(Scale::from_steps(None, vec![2, 2, 2, 2, 2, 2, 2]), Scale::chromatic(None));
    }

    #[test]
    fn test_rotation() {
        let scale = Scale::major(None).rotated(1);
        let expected_scale = Scale::from_steps(None, vec![2, 1, 2, 2, 2, 1, 2]);
        assert_eq!(scale, expected_scale);
    }

    #[test]
    fn test_macros() {
        assert_eq!(Scale::major(Some(note!(C))), scale!(note!(C), Major, 0));
        assert_eq!(Scale::minor(Some(note!(C))), scale!(note!(C), Minor, 0));
        
    }
}