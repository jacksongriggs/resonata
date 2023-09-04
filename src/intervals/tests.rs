#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_boundary_values() {
        // Lower boundary
        assert_eq!(Interval::from_semitones(0).unwrap().semitones(), 0);
        // Upper boundary
        assert_eq!(Interval::from_semitones(127).unwrap().semitones(), 127);

        assert!(Interval::from_semitones(128).is_err());
    }

    #[test]
    fn test_invalid_inputs() {
        // Invalid values for Diminished and Augmented degrees
        assert!(Interval::diminished(128).unison().is_err());
        assert!(Interval::augmented(0).unison().is_err());

        // Invalid values for octaves
        assert!(Interval::augmented(1).fifth().unwrap().compound(10).is_err());
    }

    #[test]
    fn test_compound_scenarios() {
        let dim_5 = Interval::diminished(1).fifth().unwrap();
        assert_eq!(dim_5.semitones(), 6);

        let aug_5 = Interval::augmented(1).fifth().unwrap();
        assert_eq!(aug_5.semitones(), 8);

        let compound_dim_5 = dim_5.compound(1).unwrap();
        assert_eq!(compound_dim_5.semitones(), 18);

        let compound_aug_5 = aug_5.compound(2).unwrap();
        assert_eq!(compound_aug_5.semitones(), 32);
    }

    #[test]
    fn test_random() {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let semitones = rng.gen_range(0..128);
            let interval = Interval::from_semitones(semitones);

            match interval {
                Ok(interval) => assert_eq!(interval.semitones() as i32, semitones),
                Err(_) => assert_eq!(semitones, 128),
            }
        }
    }

    #[test]
    fn extra() {
        let aug_2 = Interval::augmented(1).second().unwrap();
        let min_3 = Interval::minor().third();
        assert_eq!(aug_2.semitones(), min_3.semitones());

        let dim_2 = Interval::diminished(1).second().unwrap();
        let unis = Interval::perfect().unison();
        assert_eq!(dim_2.semitones(), unis.semitones());
    }

    use IntervalQuality as IQ;

    #[test]
    fn test_from_string() {
        assert_eq!(IQ::from_string("M").unwrap(), Major);
        assert_eq!(IQ::from_string("m").unwrap(), Minor);
        assert_eq!(IQ::from_string("P").unwrap(), Perfect);
        assert_eq!(IQ::from_string("A").unwrap(), Augmented(1));
        assert_eq!(IQ::from_string("d").unwrap(), Diminished(1));
        assert_eq!(IQ::from_string("AA").unwrap(), Augmented(2));
        assert_eq!(IQ::from_string("ddd").unwrap(), Diminished(3));
    }

    #[test]
    fn test_invert() {
        assert_eq!(Diminished(1).invert(), Augmented(1));
        assert_eq!(Augmented(3).invert(), Diminished(3));
        assert_eq!(Minor.invert(), Major);
        assert_eq!(Major.invert(), Minor);
        assert_eq!(Perfect.invert(), Perfect);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Major.to_string(), "M");
        assert_eq!(Minor.to_string(), "m");
        assert_eq!(Perfect.to_string(), "P");
        assert_eq!(Augmented(1).to_string(), "A");
        assert_eq!(Diminished(1).to_string(), "d");
        assert_eq!(Augmented(2).to_string(), "AA");
        assert_eq!(Diminished(3).to_string(), "ddd");
    }
}
