use crate::{
    error::{NoteError, ResonataError},
    intervals::*,
    nope,
};

pub use crate::{note, pnote};
pub use acc::*;
pub use name::*;

pub mod acc;
pub mod name;
mod tests;
mod utils;

type Result<T> = std::result::Result<T, ResonataError>;

/// A musical accidental
/// Flats and sharps take a number to represent
/// the number of flats or sharps.
///
/// To convert an accidental to a number of semitones,
/// use the to_semitones method.
///
/// To convert a number of semitones to an accidental,
/// use the from_semitones method.
///
/// ### Examples
/// ```
/// use resonata::notes::*;
///
/// assert_eq!(Accidental::Flat(1).to_semitones(), -1);
/// assert_eq!(Accidental::Natural.to_semitones(), 0);
/// assert_eq!(Accidental::Sharp(2).to_semitones(), 2);
///
/// assert_eq!(Accidental::from_semitones(-1), Accidental::Flat(1));
/// assert_eq!(Accidental::from_semitones(0), Accidental::Natural);
/// assert_eq!(Accidental::from_semitones(2), Accidental::Sharp(2));
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Accidental {
    Flat(u8),
    Natural,
    Sharp(u8),
}

/// A musical note name
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u8)]
pub enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

/// A musical note.
///
/// Notes are represented by a note name and an accidental.
/// See the [NoteName] and [Accidental] enums for more information.
/// A note has no octave information. To represent a note with an octave,
/// use the [PitchedNote] struct.
///
/// A macro is provided to make creating notes easier:
/// note!(name)
/// note!(name, accidental)
/// note!(string)
///
/// ### Examples
/// ```
/// use resonata::notes::*;
///
/// let c = note!(NoteName::C);
/// let d_sharp = note!(NoteName::D, Accidental::Sharp(1));
/// let f_double_flat = note!("Fbb").unwrap();
///
/// assert_eq!(c, Note::new(NoteName::C, Accidental::Natural));
/// assert_eq!(d_sharp, Note::new(NoteName::D, Accidental::Sharp(1)));
/// assert_eq!(f_double_flat, Note::new(NoteName::F, Accidental::Flat(2)));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note {
    name: NoteName,
    accidental: Accidental,
}

/// A macro to create a note.
#[macro_export]
macro_rules! note {
    ($name:literal) => {
        $name.parse::<Note>()
    };
    ($name:expr) => {
        Note::from_note_name($name)
    };
    ($name:expr, $accidental:expr) => {
        Note::new($name, $accidental)
    };
}

impl Note {
    /// Creates a new note from a note name and accidental.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c = Note::new(NoteName::C, Accidental::Natural);
    /// assert_eq!(c.name(), NoteName::C);
    /// assert_eq!(c.accidental(), Accidental::Natural);
    ///
    /// let d_sharp = Note::new(NoteName::D, Accidental::Sharp(1));
    /// assert_eq!(d_sharp.name(), NoteName::D);
    /// assert_eq!(d_sharp.accidental(), Accidental::Sharp(1));
    /// ```
    pub fn new(name: NoteName, accidental: Accidental) -> Self {
        Note { name, accidental }
    }

    /// Creates a new note from a note name, with a natural accidental.
    pub fn from_note_name(name: NoteName) -> Self {
        Note { name, accidental: Accidental::Natural }
    }

    /// Returns this note with the given octave. If the resulting
    /// note is outside of the range C-1 to G9, None will be returned.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c = note!("C").unwrap();
    /// assert_eq!(c.with_octave(4).unwrap(), pnote!("C4").unwrap());
    ///
    /// let bb = note!("Bb").unwrap();
    /// assert!(bb.with_octave(9).is_err());
    /// ```
    pub fn with_octave(&self, octave: i8) -> Result<PitchedNote> {
        PitchedNote::new(self.name, self.accidental, octave)
    }

    /// Returns the chromatic scale degree of the note.
    /// The chromatic scale degree is the number of semitones
    /// from C as 0. Note that this may be outside of the range 0-11,
    /// for example, B# is 12, and Cb is -1.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let f = note!("F").unwrap();
    /// assert_eq!(f.to_chromatic_scale_degree(), 5);
    ///
    /// let d_double_sharp = note!("Dx").unwrap();
    /// assert_eq!(d_double_sharp.to_chromatic_scale_degree(), 4);
    ///
    /// let c_flat = note!("Cb").unwrap();
    /// assert_eq!(c_flat.to_chromatic_scale_degree(), -1);
    ///
    /// let b_sharp = note!("B##").unwrap();
    /// assert_eq!(b_sharp.to_chromatic_scale_degree(), 13);
    /// ```
    pub fn to_chromatic_scale_degree(&self) -> i32 {
        self.name.to_chromatic_scale_degree() as i32 + self.accidental.to_semitones()
    }

    /// Returns the note from the given chromatic scale degree.
    /// The chromatic scale degree is the number of semitones
    /// from C as 0. This will be clamped to the range 0-11,
    /// and will only return a note with a natural or sharp accidental.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let f = note!("F").unwrap();
    /// assert_eq!(Note::from_chromatic_scale_degree(5), f);
    ///
    /// let d_sharp = note!("D#").unwrap();
    /// assert_eq!(Note::from_chromatic_scale_degree(3), d_sharp);
    ///
    /// let a_sharp = note!("A#").unwrap();
    /// assert_eq!(Note::from_chromatic_scale_degree(10), a_sharp);
    /// ```
    pub fn from_chromatic_scale_degree(number: u8) -> Self {
        let name = NoteName::from_chromatic_scale_degree(number);
        let accidental =
            Accidental::from_semitones(number as i32 - name.to_chromatic_scale_degree() as i32);
        Note::new(name, accidental)
    }

    /// Returns the number of semitones from this note to the given note.
    /// The given note is assumed to be higher than this note. To find the
    /// smallest distance between two notes, use the `semitones_between` method.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c = note!("C").unwrap();
    /// let d = note!("D").unwrap();
    /// assert_eq!(c.semitones_to(&d), 2);
    /// assert_eq!(d.semitones_to(&c), 10);
    ///
    /// let c_flat = note!("Cb").unwrap();
    /// let b_sharp = note!("B#").unwrap();
    /// assert_eq!(c_flat.semitones_to(&b_sharp), 13);
    /// assert_eq!(b_sharp.semitones_to(&c_flat), -1);
    pub fn semitones_to(&self, other: &Note) -> i32 {
        let semitones = other.to_chromatic_scale_degree() - self.to_chromatic_scale_degree();
        if semitones < 0 {
            semitones + 12
        } else {
            semitones
        }
    }

    /// Returns the smallest number of semitones between this note and the given note.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c = note!("C").unwrap();
    /// let d = note!("D").unwrap();
    /// assert_eq!(c.semitones_between(&d), 2);
    /// assert_eq!(d.semitones_between(&c), 2);
    ///
    pub fn semitones_between(&self, other: &Note) -> i32 {
        let semitones = self.semitones_to(other);
        if semitones > 6 {
            (semitones - 12).abs()
        } else {
            semitones
        }
    }

    /// Returns the enharmonic equivalent of the note at the given distance.
    ///
    /// The enharmonic equivalent is the note with the same pitch,
    /// but with a different name and accidental.
    ///
    /// Distance is measured in white keys,
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c = note!("C").unwrap();
    /// let b_sharp = note!("B#").unwrap();
    /// assert_eq!(c.to_enharmonic_equivalent(-1), b_sharp);
    /// assert_eq!(b_sharp.to_enharmonic_equivalent(1), c);
    ///
    /// let c_sharp = note!("C#").unwrap();
    /// let d_flat = note!("Db").unwrap();
    /// assert_eq!(c_sharp.to_enharmonic_equivalent(1), d_flat);
    /// assert_eq!(d_flat.to_enharmonic_equivalent(-8), c_sharp);
    ///
    /// let e_dbl_sharp = note!("Ex").unwrap();
    /// let g_flat = note!("Gb").unwrap();
    /// assert_eq!(e_dbl_sharp.to_enharmonic_equivalent(2), g_flat);
    /// ```
    pub fn to_enharmonic_equivalent(&self, dist: i32) -> Self {
        let mut note = Note::new(self.name + dist, self.accidental);
        let diff = self.semitones_between(&note);
        match dist {
            n if n > 0 => note.accidental -= diff,
            n if n < 0 => note.accidental += diff,
            _ => (),
        };
        note
    }

    /// Returns the interval from this note to the given note.
    /// The given note is assumed to be higher than this note. To find the
    /// smallest interval between two notes, use the `interval_between` method.
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, intervals::*};
    ///
    /// let c = note!("C").unwrap();
    /// let d = note!("D").unwrap();
    /// assert_eq!(c.interval_to(&d), inv!("M2").unwrap());
    /// assert_eq!(d.interval_to(&c), inv!("m7").unwrap());
    ///
    /// let c_flat = note!("Cb").unwrap();
    /// let b_sharp = note!("B#").unwrap();
    /// assert_eq!(c_flat.interval_to(&b_sharp), inv!("AA7").unwrap());
    /// ```
    pub fn interval_to(&self, other: &Note) -> Interval {
        Interval::from_semitones(self.semitones_to(other))
            .unwrap()
            .as_size(Size::from((other.name - self.name + 7) as i32), 0)
            .unwrap()
    }

    pub fn name(&self) -> NoteName {
        self.name
    }

    pub fn accidental(&self) -> Accidental {
        self.accidental
    }
}

/// A musical note with an octave.
///
/// A pitched note is represented by a note name, accidental and octave.
/// See the [NoteName] and [Accidental] enums for more information.
///
/// A macro is provided to make creating pitched notes easier:
/// pnote!(name, octave)
/// pnote!(name, accidental, octave)
/// pnote!(string)
///
/// ### Examples
/// ```
/// use resonata::notes::*;
///
/// let c4 = pnote!(NoteName::C, 4).unwrap();
/// assert_eq!(c4, PitchedNote::new(NoteName::C, Accidental::Natural, 4).unwrap());
///
/// let d_sharp_2 = pnote!(NoteName::D, Accidental::Sharp(1), 2).unwrap();
/// assert_eq!(d_sharp_2, PitchedNote::new(NoteName::D, Accidental::Sharp(1), 2).unwrap());
///
/// let f_double_flat_8 = pnote!("Fbb8").unwrap();
/// assert_eq!(f_double_flat_8, PitchedNote::new(NoteName::F, Accidental::Flat(2), 8).unwrap());
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PitchedNote {
    note: Note,
    octave: i8,
}

/// A macro to create a pitched note.
#[macro_export]
macro_rules! pnote {
    ($name:literal) => {
        $name.parse::<PitchedNote>()
    };
    ($name:expr) => {
        PitchedNote::build($name, 4)
    };
    ($name:expr, $oct:literal) => {
        PitchedNote::build($name, $oct)
    };
    ($name:expr, $accidental:expr) => {
        PitchedNote::new($name, $accidental, 4)
    };
    ($name:expr, $accidental:expr, $oct:literal) => {
        PitchedNote::new($name, $accidental, $oct)
    };
}

impl PitchedNote {
    /// Creates a new note from a note name, accidental and octave. If the resulting
    /// note is outside of the range C-1 to G9, None will be returned.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c4 = PitchedNote::new(NoteName::C, Accidental::Natural, 4).unwrap();
    /// assert_eq!(c4.name(), NoteName::C);
    /// assert_eq!(c4.accidental(), Accidental::Natural);
    /// assert_eq!(c4.octave(), 4);
    ///
    /// let g_sharp_9 = PitchedNote::new(NoteName::G, Accidental::Sharp(1), 9);
    /// assert!(g_sharp_9.is_err());
    /// ```
    pub fn new(name: NoteName, accidental: Accidental, octave: i8) -> Result<Self> {
        let note = Note::new(name, accidental);

        match octave {
            -1..=8 => (),
            9 => {
                if note.to_chromatic_scale_degree() > 7 {
                    nope!(NoteError::InvalidOctave(octave));
                }
            }
            _ => nope!(NoteError::InvalidOctave(octave)),
        }

        Ok(PitchedNote { note, octave })
    }

    /// Creates a new note from a note name and octave, with a natural accidental. If the resulting
    /// note is outside of the range C-1 to G9, None will be returned.
    pub fn build(name: NoteName, octave: i8) -> Result<Self> {
        Self::new(name, Accidental::Natural, octave)
    }

    /// Returns the midi note number of the note.
    /// The midi note number is the number of semitones from C-1 (0).
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c4 = pnote!("C4").unwrap();
    /// assert_eq!(c4.to_midi_number(), 60);
    ///
    /// let c_neg1 = pnote!("C-1").unwrap();
    /// assert_eq!(c_neg1.to_midi_number(), 0);
    ///
    /// let gbb9 = pnote!("Gbb9").unwrap();
    /// assert_eq!(gbb9.to_midi_number(), 125);
    /// ```
    pub fn to_midi_number(&self) -> u8 {
        let degree = self.note.to_chromatic_scale_degree() as i32;
        let octave = self.octave as i32;
        (degree + (octave + 1) * 12) as u8
    }

    /// Returns the note at the given midi note number.
    /// The midi note number is the number of semitones from C-1 (0).
    ///
    /// The resulting note will be either a natural or sharp note.
    /// You can convert this to an enharmonic equivalent using the `to_enharmonic_equivalent` method.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c4 = pnote!("C4").unwrap();
    /// assert_eq!(PitchedNote::from_midi_number(60).unwrap(), c4);
    ///
    /// let gbb9 = pnote!("Gbb9").unwrap();
    /// assert_eq!(PitchedNote::from_midi_number(125).unwrap()
    ///     .to_enharmonic_equivalent(1).unwrap(), gbb9);
    ///
    /// let c_neg1 = pnote!("C-1").unwrap();
    /// assert_eq!(PitchedNote::from_midi_number(0).unwrap(), c_neg1);
    ///
    /// let c9 = pnote!("C9").unwrap();
    /// assert_eq!(PitchedNote::from_midi_number(120).unwrap(), c9);
    /// ```
    pub fn from_midi_number(number: u8) -> Result<Self> {
        let octave = (number / 12) as i8 - 1;
        let note = Note::from_chromatic_scale_degree(number % 12);
        PitchedNote::new(note.name, note.accidental, octave)
    }

    /// Returns the number of semitones from this note to the given note.
    /// If the given note is lower than this note, the result will be negative.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c4 = pnote!("C4").unwrap();
    /// let d4 = pnote!("D4").unwrap();
    /// assert_eq!(c4.semitones_to(&d4), 2);
    /// assert_eq!(d4.semitones_to(&c4), -2);
    ///
    /// let c_neg1 = pnote!("C-1").unwrap();
    /// let c0 = pnote!("C0").unwrap();
    /// assert_eq!(c_neg1.semitones_to(&c0), 12);
    /// assert_eq!(c0.semitones_to(&c_neg1), -12);
    /// ```
    pub fn semitones_to(&self, other: &PitchedNote) -> i32 {
        other.to_midi_number() as i32 - self.to_midi_number() as i32
    }

    /// Returns the number of semitones between this note and the given note.
    /// The result will always be positive.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c4 = pnote!("C4").unwrap();
    /// let d4 = pnote!("D4").unwrap();
    /// assert_eq!(c4.semitones_between(&d4), 2);
    /// assert_eq!(d4.semitones_between(&c4), 2);
    /// ```
    pub fn semitones_between(&self, other: &PitchedNote) -> i32 {
        self.semitones_to(other).abs()
    }

    /// Returns the number of note names from this note to the given note.
    /// If the given note is lower than this note, the result will be negative.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let c4 = pnote!("C4").unwrap();
    /// let g4 = pnote!("G4").unwrap();
    /// assert_eq!(c4.diatonic_distance_to(&g4), 4);
    /// assert_eq!(g4.diatonic_distance_to(&c4), -4);
    ///
    /// let c_sharp_3 = pnote!("C#3").unwrap();
    /// let f_flat_4 = pnote!("Fb4").unwrap();
    /// assert_eq!(c_sharp_3.diatonic_distance_to(&f_flat_4), 10);
    /// assert_eq!(f_flat_4.diatonic_distance_to(&c_sharp_3), -10);
    /// ```
    pub fn diatonic_distance_to(&self, other: &PitchedNote) -> i32 {
        let self_note_name = self.note.name;
        let self_octave = self.octave;

        let other_note_name = other.note.name;
        let other_octave = other.octave;

        other_note_name as i32 - self_note_name as i32 + 7 * (other_octave - self_octave) as i32
    }

    /// Returns this note moved by the given number of diatonic steps.
    /// The number of diatonic steps is the number of note names moved.
    /// For example, moving C4 by +2 diatonic steps would result in E4.
    /// The resulting note will have the same accidental as this note.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let note = pnote!("C4").unwrap();
    /// assert_eq!(note.moved_by(2).unwrap(), pnote!("E4").unwrap());
    /// assert_eq!(note.moved_by(-2).unwrap(), pnote!("A3").unwrap());
    /// ```
    pub fn moved_by(&self, steps: i32) -> Result<Self> {
        let position = self.note.name as i32 + steps;
        let mut octave = self.octave;
        match position {
            n if n < 0 => octave -= 1,
            n if n > 6 => octave += 1,
            _ => (),
        }

        PitchedNote::new(NoteName::from(7 + position), self.accidental(), octave)
    }

    /// Returns the enharmonic equivalent of the note at the given distance.
    ///
    /// The enharmonic equivalent is the note with the same pitch,
    /// but with a different name and accidental.
    ///
    /// Distance is measured in note names ignoring the accidental, and can be negative,
    /// i.e. moving from C to B is a distance of -1, and moving from C to E is a distance of 2.
    ///
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    ///
    /// let b_sharp_3 = pnote!("B#3").unwrap();
    /// let c_4 = pnote!("C4").unwrap();
    /// assert_eq!(b_sharp_3.to_enharmonic_equivalent(1).unwrap(), c_4);
    /// assert_eq!(c_4.to_enharmonic_equivalent(-1).unwrap(), b_sharp_3);
    ///
    /// let e_dbl_sharp_2 = pnote!("E##2").unwrap();
    /// let g_flat_2 = pnote!("Gb2").unwrap();
    /// assert_eq!(e_dbl_sharp_2.to_enharmonic_equivalent(2).unwrap(), g_flat_2);
    /// assert_eq!(g_flat_2.to_enharmonic_equivalent(-2).unwrap(), e_dbl_sharp_2);
    /// ```
    pub fn to_enharmonic_equivalent(&self, dist: i32) -> Result<Self> {
        let equivalent = self.note.to_enharmonic_equivalent(dist);

        if dist > 0 && equivalent.name < self.note.name {
            equivalent.with_octave(self.octave + 1)
        } else if dist < 0 && equivalent.name > self.note.name {
            equivalent.with_octave(self.octave - 1)
        } else {
            equivalent.with_octave(self.octave)
        }
    }

    /// Returns the interval from this note to the given note.
    ///
    /// ### Examples
    /// ```
    /// use resonata::notes::*;
    /// use resonata::intervals::*;
    ///
    /// let c4 = pnote!("C4").unwrap();
    /// let d4 = pnote!("D4").unwrap();
    /// assert_eq!(c4.interval_to(&d4), inv!("M2").unwrap());
    ///
    /// let cb4 = pnote!("Cb4").unwrap();
    /// let b3 = pnote!("B3").unwrap();
    /// assert_eq!(cb4.interval_to(&b3), inv!("d2").unwrap());
    ///
    /// let c_triple_sharp_3 = pnote!("C#x3").unwrap();
    /// let d_double_flat_3 = pnote!("Dbb3").unwrap();
    /// assert_eq!(c_triple_sharp_3.interval_to(&d_double_flat_3), inv!("A2").unwrap());
    /// ```
    pub fn interval_to(&self, other: &PitchedNote) -> Interval {
        let diatonic_distance = self.diatonic_distance_to(other);
        Interval::from_semitones(self.semitones_between(other))
            .unwrap()
            .as_size(
                Size::from(diatonic_distance as u8),
                (diatonic_distance / 7) as u8,
            )
            .unwrap()
    }

    /// Returns the note name of the note
    pub fn name(&self) -> NoteName {
        self.note.name
    }

    /// Returns the accidental of the note
    pub fn accidental(&self) -> Accidental {
        self.note.accidental
    }

    /// Returns the octave of the note
    pub fn octave(&self) -> i8 {
        self.octave
    }
}
