/// A macro to create a note.
#[macro_export]
macro_rules! note {
    // For note!("C4");
    ($name:literal) => {
        Note::from_str($name)
    };
    // For note!(C);
    ($name:expr) => {
        Note::from_note_name_and_octave($name, 4)
    };
    // For note!(C, 4);
    ($name:expr, $oct:literal) => {
        Note::from_note_name_and_octave($name, $oct)
    };
    // For note!(C, Flat(1));
    ($name:expr, $accidental:expr) => {
        Note::build($name, $accidental, 4)
    };
    // For note!(C, Flat(1), 4);
    ($name:expr, $accidental:expr, $oct:literal) => {
        Note::build($name, $accidental, $oct)
    };
}