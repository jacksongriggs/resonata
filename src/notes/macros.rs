/// A macro to create a note.
#[macro_export]
macro_rules! note {
    // For note!("C");
    ($name:literal) => {
        Note::from_str($name)
    };
    // For note!(C);
    ($name:expr) => {
        Note::from_note_name($name)
    };
    // For note!(C, Flat(1));
    ($name:expr, $accidental:expr) => {
        Note::new($name, $accidental)
    };
}


/// A macro to create a pitched note.
#[macro_export]
macro_rules! pnote {
    // For note!("C4");
    ($name:literal) => {
        PitchedNote::from_str($name)
    };
    // For note!(C);
    ($name:expr) => {
        PitchedNote::from_note_name_and_octave($name, 4)
    };
    // For note!(C, 4);
    ($name:expr, $oct:literal) => {
        PitchedNote::from_note_name_and_octave($name, $oct)
    };
    // For note!(C, Flat(1));
    ($name:expr, $accidental:expr) => {
        PitchedNote::new($name, $accidental, 4)
    };
    // For note!(C, Flat(1), 4);
    ($name:expr, $accidental:expr, $oct:literal) => {
        PitchedNote::new($name, $accidental, $oct)
    };
}