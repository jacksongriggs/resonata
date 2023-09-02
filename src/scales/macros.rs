/// A macro to create a scale from a root note and a scale type.
#[macro_export]
macro_rules! scale {
    ($str:literal) => {
        Scale::from_string($str)
    };
    ($scale_type:expr) => {
        Scale::from_steps($scale_type.as_steps()).unwrap()
    };
    ($scale_type:expr, $rot:expr) => {
        Scale::from_steps($scale_type.as_steps()).unwrap().rotated($rot)
    };
}
