#[cfg(test)]
mod tests {
    use super::super::*;
    #[test]
    fn test_note_transpose_up() {
        let note = "C".parse::<Note>().unwrap();
        let new_note = note + "M3".parse::<Interval>().unwrap();
        assert_eq!(new_note, "E".parse::<Note>().unwrap());

        let note = "D#".parse::<Note>().unwrap();
        let new_note = note + "A4".parse::<Interval>().unwrap();
        assert_eq!(new_note, "G##".parse::<Note>().unwrap());

        let note = "C".parse::<Note>().unwrap();
        let new_note = note + "P8".parse::<Interval>().unwrap();
        assert_eq!(new_note, "C".parse::<Note>().unwrap());
    }

    #[test]
    fn test_note_transpose_down() {
        let note = "C".parse::<Note>().unwrap();
        let new_note = note - "M3".parse::<Interval>().unwrap();
        assert_eq!(new_note, "Ab".parse::<Note>().unwrap());

        let note = "D#".parse::<Note>().unwrap();
        let new_note = note - "A4".parse::<Interval>().unwrap();
        assert_eq!(new_note, "A".parse::<Note>().unwrap());

        let note = "C".parse::<Note>().unwrap();
        let new_note = note - "P8".parse::<Interval>().unwrap();
        assert_eq!(new_note, "C".parse::<Note>().unwrap());
    }

    #[test]
    fn test_pitched_note_transpose_up() {
        let note = pnote!("C4").unwrap() + inv!("M3").unwrap();
        assert_eq!(note.unwrap(), pnote!("E4").unwrap());

        let note = pnote!("A4").unwrap() + inv!("d3").unwrap();
        assert_eq!(note.unwrap(), pnote!("Cb5").unwrap());
    }

    #[test]
    fn test_pitched_note_transpose_down() {
        let note = pnote!("C4").unwrap() - inv!("M3").unwrap();
        assert_eq!(note.unwrap(), pnote!("Ab3").unwrap());

        let note = pnote!("F#4").unwrap() - inv!("A5").unwrap();
        assert_eq!(note.unwrap(), pnote!("Bb3").unwrap());
    }

    #[test]
    fn test_note_from_str() {
        let c = "C".parse::<Note>().unwrap();
        assert_eq!(c, note!(NoteName::C));

        let d_sharp = "D#".parse::<Note>().unwrap();
        assert_eq!(d_sharp, note!(NoteName::D, Accidental::Sharp(1)));
    }

    #[test]
    fn test_pitched_note_from_str() {
        let g = "G".parse::<PitchedNote>().unwrap();
        assert_eq!(g, pnote!(NoteName::G, 4).unwrap());

        let e_flat_2 = "Eb2".parse::<PitchedNote>().unwrap();
        assert_eq!(e_flat_2, pnote!(NoteName::E, Accidental::Flat(1), 2).unwrap());

        let resonata = "Resonata".parse::<PitchedNote>();
        assert!(resonata.is_err());
    }
}
