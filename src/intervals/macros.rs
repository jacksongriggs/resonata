#[macro_export]
macro_rules! inv {
    // int!(quality size octaves) 
    // e.g. int!(Major Third 1) | int!(Perfect Fifth 2)
    // i.e. Major 10th | Perfect 17th
    ($quality:ident $size:ident $octaves:literal) => { 
        Interval::build($quality, $size, $octaves)
    };
    // int!(quality size) 
    // e.g. int!(Major Third) | int!(Perfect Fifth)
    // i.e. Major 3rd | Perfect 5th
    ($quality:ident $size:ident) => { 
        Interval::build($quality, $size, 0)
    };
    // int!(quality(num) size octaves) 
    // e.g. int!(Augmented(1) Fifth 2) | int!(Diminished(1) Fourth 1)
    // i.e. Augmented 19th | Diminished 11th
    ($quality:ident($num:literal) $size:ident $octaves:literal) => { 
        Interval::build($quality($num), $size, $octaves)
    };
    // int!(quality(num) size) 
    // e.g. int!(Augmented(1) Fifth) | int!(Diminished(1) Fourth)
    // i.e. Augmented 5th | Diminished 4th
    ($quality:ident($num:literal) $size:ident) => { 
        Interval::build($quality($num), $size, 0)
    };
    // int!(string) 
    // e.g. int!("M3") | int!("m2") | int!("P5") | int!("d7") | int!("A4")
    // i.e. Major 3rd | Minor 2nd | Perfect 5th | Diminished 7th | Augmented 4th
    ($s:literal) => { 
        Interval::from_str($s)
    };
}