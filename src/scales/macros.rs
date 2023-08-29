/// A macro to create a scale from a root note and a scale type.
#[macro_export]
macro_rules! scale {
    ($scale_type:expr, $rot:literal) => {
        Scale::from_steps(None, $scale_type.to_steps().rotate_left($rot))
    };
    ($root:expr, $scale_type:expr, $rot:literal) => {
        Scale::from_steps(Some($root), $scale_type.to_steps()).rotated($rot)
    };
}