/// A macro to create a scale from a root note and a scale type.
#[macro_export]
macro_rules! scale {
    ($scale_type:expr) => {
        Scale::from_steps($scale_type.as_steps())
    };
    ($scale_type:expr, $rot:literal) => {
        Scale::from_steps($scale_type.as_steps()).rotated($rot)
    };
}