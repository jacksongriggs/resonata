use crate::{error::ResonataError, notes::*, scales::Scale};
use std::ops::Index;
mod utils;

pub use crate::key;
pub use std::collections::HashMap;

type Result<T> = std::result::Result<T, ResonataError>;

/// A musical key
///
/// A key is represented by a set of notes. Each note name is mapped to an accidental.
///
/// The key is initialized with all natural notes, then the user can specify which notes
/// are altered.
///
/// A macro is provided to make creating keys easier:
/// key!(note, ...)
///
/// ### Examples
/// ```
/// use resonata::{notes::*, keys::*, scales::*, intervals::*, TransposeUp};
///
/// let key = key!(note!("Cb").unwrap(), note!("F#").unwrap());
/// assert_eq!(key.pitch(NoteName::C).accidental(), Accidental::Flat(1));
/// assert_eq!(key.pitch(NoteName::D).accidental(), Accidental::Natural);
/// assert_eq!(key.pitch(NoteName::E).accidental(), Accidental::Natural);
/// assert_eq!(key.pitch(NoteName::F).accidental(), Accidental::Sharp(1));
/// assert_eq!(key.pitch(NoteName::G).accidental(), Accidental::Natural);
/// assert_eq!(key.pitch(NoteName::A).accidental(), Accidental::Natural);
/// assert_eq!(key.pitch(NoteName::B).accidental(), Accidental::Natural);
///
/// let key = key!("F#").unwrap();
/// assert_eq!(key.root().unwrap(), note!("G").unwrap());
///
/// let key = key.transposed_up(inv!("d3").unwrap());
/// assert_eq!(key.root().unwrap(), note!("Bbb").unwrap());
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Key {
    pitches: HashMap<NoteName, Accidental>,
}

#[macro_export]
macro_rules! key {
    ($str:literal) => {
        Key::from_string($str)
    };
    ($($note:expr),*) => {
        {
            let mut notes = Vec::new();
            $(
                notes.push($note);
            )*
            Key::new(notes)
        }
    };
}

impl Key {
    pub fn new(notes: Vec<Note>) -> Key {
        // Initialize the key with all natural notes, then set the user-specified notes.
        // This ensures that the key will have all notes, even if the user doesn't specify them all.
        use NoteName::*;
        let mut pitches = HashMap::new();
        for note_name in vec![C, D, E, F, G, A, B] {
            pitches.insert(note_name, Accidental::Natural);
        }

        // Set the user-specified notes.
        for note in notes {
            pitches.insert(note.name(), note.accidental());
        }

        Key { pitches }
    }

    /// Returns a key from the given string. The string should be a space- or comma-separated list of notes.
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, keys::*};
    ///
    /// let key = Key::from_string("Eb F G Ab Bb C D").unwrap();
    /// assert_eq!(key.pitch(NoteName::C).accidental(), Accidental::Natural);
    /// assert_eq!(key.pitch(NoteName::D).accidental(), Accidental::Natural);
    /// assert_eq!(key.pitch(NoteName::E).accidental(), Accidental::Flat(1));
    /// assert_eq!(key.pitch(NoteName::F).accidental(), Accidental::Natural);
    /// assert_eq!(key.pitch(NoteName::G).accidental(), Accidental::Natural);
    /// assert_eq!(key.pitch(NoteName::A).accidental(), Accidental::Flat(1));
    /// assert_eq!(key.pitch(NoteName::B).accidental(), Accidental::Flat(1));
    /// ```
    pub fn from_string(s: &str) -> Result<Key> {
        let s = s.replace(",", " ");
        let notes =
            s.split_whitespace().map(|s| Note::from_string(s)).collect::<Result<Vec<Note>>>()?;
        Ok(Key::new(notes))
    }

    pub fn set_pitch(&mut self, note: Note) {
        self.pitches.insert(note.name(), note.accidental());
    }

    pub fn pitch(&self, note_name: NoteName) -> Note {
        let accidental = self.pitches.get(&note_name).unwrap();
        Note::new(note_name, *accidental)
    }

    pub fn pitches(&self) -> Vec<Note> {
        use NoteName::*;
        vec![
            self.pitch(C),
            self.pitch(D),
            self.pitch(E),
            self.pitch(F),
            self.pitch(G),
            self.pitch(A),
            self.pitch(B),
        ]
    }

    /// Returns the key as a scale.
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, keys::*, scales::*};
    ///
    /// let key = key!(note!("F#").unwrap());
    /// let scale = key.to_scale();
    /// eprintln!("scale: {}", scale);
    /// assert_eq!(scale.to_notes(note!("C").unwrap()), vec![
    ///     note!("C").unwrap(),
    ///     note!("D").unwrap(),
    ///     note!("E").unwrap(),
    ///     note!("F#").unwrap(),
    ///     note!("G").unwrap(),
    ///     note!("A").unwrap(),
    ///     note!("B").unwrap(),
    /// ]);
    ///```
    pub fn to_scale(&self) -> Scale {
        let mut notes = Vec::new();
        for note in self.pitches() {
            notes.push(note);
        }
        notes.push(self.pitch(NoteName::C));
        Scale::from_notes(notes)
    }

    /// Returns the scale type of the key, if it exists.
    /// Otherwise, returns None.
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, keys::*, scales::*};
    ///
    /// let key = key!(note!("F#").unwrap());
    /// assert_eq!(key.to_scale_type(), Some(Major.into()));
    ///
    /// let key = key!(note!("G#").unwrap());
    /// assert_eq!(key.to_scale_type(), Some(HarmonicMinor.into()));
    ///
    /// let key = key!(note!("Fb").unwrap());
    /// assert!(key.to_scale_type().is_none());
    /// ```
    pub fn to_scale_type(&self) -> Option<crate::scales::types::ScaleEnumType> {
        match self.to_scale().get_parent_scale_type() {
            Some((scale_type, _)) => Some(scale_type),
            None => None,
        }
    }

    /// Returns the root note of the key, if it exists.
    /// Otherwise, returns None.
    ///
    /// ### Examples
    /// ```
    /// use resonata::{notes::*, keys::*, scales::*};
    ///
    /// let key = key!(note!("F#").unwrap());
    /// assert_eq!(key.root().unwrap(), note!("G").unwrap());
    /// assert_eq!(key.to_scale_type().unwrap(), Major.into());
    ///
    /// let key = key!(note!("G#").unwrap());
    /// assert_eq!(key.root().unwrap(), note!("A").unwrap());
    /// assert_eq!(key.to_scale_type().unwrap(), HarmonicMinor.into());
    ///
    /// let key = key!(note!("Fb").unwrap());
    /// assert!(key.root().is_none());
    /// ```
    pub fn root(&self) -> Option<Note> {
        match self.to_scale().get_parent_scale_type() {
            Some((_, root)) => Some(*self.pitches().index(root)),
            None => None,
        }
    }
}
