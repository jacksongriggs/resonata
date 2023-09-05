#[cfg(test)]
mod tests {
    use super::super::*;
    fn test_from_str() {
        let key = "Eb F G Ab Bb C D".parse::<Key>().unwrap();
        assert_eq!(key.pitch(NoteName::C).accidental(), Accidental::Natural);
        assert_eq!(key.pitch(NoteName::D).accidental(), Accidental::Natural);
        assert_eq!(key.pitch(NoteName::E).accidental(), Accidental::Flat(1));
        assert_eq!(key.pitch(NoteName::F).accidental(), Accidental::Natural);
        assert_eq!(key.pitch(NoteName::G).accidental(), Accidental::Natural);
        assert_eq!(key.pitch(NoteName::A).accidental(), Accidental::Flat(1));
        assert_eq!(key.pitch(NoteName::B).accidental(), Accidental::Flat(1));
    }
}
