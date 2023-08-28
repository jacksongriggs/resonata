use std::fmt::{self, Display, Formatter};
use super::*;

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let root = match &self.root {
            Some(root) => root.to_string(),
            None => String::from(""),
        };
        let intervals = self.intervals
            .iter()
            .map(|interval| interval.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}: {}", root, intervals)
    }
}