use std::fmt;
use std::path::PathBuf;

/// A type in another crate that has the name of a levi-ui widget.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DuplicateHit {
    pub file: PathBuf,
    /// The line number, from 1.
    pub line: usize,
    pub name: String,
}

impl fmt::Display for DuplicateHit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = self.file.to_string_lossy().replace('\\', "/");
        write!(formatter, "{file}:{}: {} is a levi-ui widget name", self.line, self.name)
    }
}
