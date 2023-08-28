#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::*;

    #[test]
    fn test_from_steps() {
        let scale = Scale::major(None);
        let intervals = scale.intervals();
        let expected_intervals = vec![
            Interval::new(2).unwrap(),
            Interval::new(4).unwrap(),
            Interval::new(5).unwrap(),
            Interval::new(7).unwrap(),
            Interval::new(9).unwrap(),
            Interval::new(11).unwrap(),
            Interval::new(12).unwrap(),
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
            Note::new(60).unwrap(),
            Note::new(62).unwrap(),
            Note::new(64).unwrap(),
            Note::new(65).unwrap(),
            Note::new(67).unwrap(),
            Note::new(69).unwrap(),
            Note::new(71).unwrap(),
            Note::new(72).unwrap(),
        ]).unwrap();
        let expected_scale = Scale::major(Note::new(60).ok());
        println!("{}", scale);
        assert_eq!(scale, expected_scale);
    }

    #[test]
    fn test_to_notes() {
        let scale = Scale::major(Note::new(60).ok());
        let expected_notes = vec![
            Note::new(60).unwrap(),
            Note::new(62).unwrap(),
            Note::new(64).unwrap(),
            Note::new(65).unwrap(),
            Note::new(67).unwrap(),
            Note::new(69).unwrap(),
            Note::new(71).unwrap(),
            Note::new(72).unwrap(),
        ];
        assert_eq!(scale.to_notes(), expected_notes);
        println!("{}", scale);
    }

    #[test]
    fn test_known_scale_major() {
        let scale = Scale::major(None);
        let expected_scale = Scale::from_steps(None, vec![2, 2, 1, 2, 2, 2, 1]);
        assert_eq!(scale, expected_scale);
    }

    #[test]
    fn test_rotation() {
        let scale = Scale::major(None)
            .rotated(1, Direction::Up);
        let expected_scale = Scale::from_steps(None, vec![2, 1, 2, 2, 2, 1, 2]);
        assert_eq!(scale, expected_scale);
    }
}