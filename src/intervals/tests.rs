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
    fn test_from_str() {
        let major_third = inv!(Quality::Major, Size::Third).unwrap();
        assert_eq!(major_third, "M3".parse::<Interval>().unwrap());

        let augmented_octave = inv!(Quality::Augmented(1), Size::Unison, 1).unwrap();
        assert_eq!(augmented_octave, "A8".parse::<Interval>().unwrap());
        
        let invalid_interval = "P3".parse::<Interval>();
        assert!(invalid_interval.is_err());
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

    #[test]
    fn test_interval_quality_from_str() {
        assert_eq!("M".parse::<Quality>().unwrap(), Quality::Major);
        assert_eq!("m".parse::<Quality>().unwrap(), Quality::Minor);
        assert_eq!("P".parse::<Quality>().unwrap(), Quality::Perfect);
        assert_eq!("A".parse::<Quality>().unwrap(), Quality::Augmented(1));
        assert_eq!("d".parse::<Quality>().unwrap(), Quality::Diminished(1));
        assert_eq!("AA".parse::<Quality>().unwrap(), Quality::Augmented(2));
        assert_eq!("ddd".parse::<Quality>().unwrap(), Quality::Diminished(3));
    }

    #[test]
    fn test_interval_quality_invert() {
        assert_eq!(Quality::Diminished(1).invert(), Quality::Augmented(1));
        assert_eq!(Quality::Augmented(3).invert(), Quality::Diminished(3));
        assert_eq!(Quality::Minor.invert(), Quality::Major);
        assert_eq!(Quality::Major.invert(), Quality::Minor);
        assert_eq!(Quality::Perfect.invert(), Quality::Perfect);
    }

    #[test]
    fn test_interval_quality_to_string() {
        assert_eq!(Quality::Major.to_string(), "M");
        assert_eq!(Quality::Minor.to_string(), "m");
        assert_eq!(Quality::Perfect.to_string(), "P");
        assert_eq!(Quality::Augmented(1).to_string(), "A");
        assert_eq!(Quality::Diminished(1).to_string(), "d");
        assert_eq!(Quality::Augmented(2).to_string(), "AA");
        assert_eq!(Quality::Diminished(3).to_string(), "ddd");
    }
}
