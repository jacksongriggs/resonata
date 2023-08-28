#[macro_export]
macro_rules! note {
    // For note!(C);
    ($name:ident) => { 
        Note::build($name, Natural, 4)
    };
    // For note!("C4");
    ($name:expr) => {
        Note::from_str($name)
    };
    // For note!(C, 4);
    ($name:ident $oct:literal) => {
        Note::build($name, Natural, $oct)
    };
    // For note!(C, Flat(1));
    ($name:ident $accidental:ident($num:literal)) => {
        Note::build($name, $accidental($num), 4)
    };
    // For note!(C, Flat(1), 4);
    ($name:ident $accidental:ident($num:literal) $oct:literal) => {
        Note::build($name, $accidental($num), $oct)
    };
}