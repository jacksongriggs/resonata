/// A macro for creating intervals.
#[macro_export]
macro_rules! inv {
    ($s:literal) => { 
        Interval::from_str($s)
    };
    ($quality:expr, $size:expr) => { 
        Interval::build($quality, $size, 0)
    };
    ($quality:expr, $size:expr, $octaves:literal) => { 
        Interval::build($quality, $size, $octaves)
    };
}