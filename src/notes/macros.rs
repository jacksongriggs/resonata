/// A macro to create a note.
#[macro_export]
macro_rules! note {
    // For note!("C");
    ($name:literal) => {
        Note::from_string($name)
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
    // For pnote!("C4");
    ($name:literal) => {
        PitchedNote::from_string($name)
    };
    // For pnote!(C);
    ($name:expr) => {
        PitchedNote::build($name, 4)
    };
    // For pnote!(C, 4);
    ($name:expr, $oct:literal) => {
        PitchedNote::build($name, $oct)
    };
    // For pnote!(C, Flat(1));
    ($name:expr, $accidental:expr) => {
        PitchedNote::new($name, $accidental, 4)
    };
    // For pnote!(C, Flat(1), 4);
    ($name:expr, $accidental:expr, $oct:literal) => {
        PitchedNote::new($name, $accidental, $oct)
    };
}
