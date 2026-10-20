//! Identity of one audio file on disk. No I/O happens here.

use std::path::{Path, PathBuf};

/// An audio file the scanner selected. The name is valid Unicode (ADR-0004).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioFile {
    path: PathBuf,
    name: String,
}

impl AudioFile {
    pub(crate) fn new(path: PathBuf, name: String) -> Self {
        Self { path, name }
    }

    /// Full path, as scanned (resolved when links are followed).
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Final path component.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Lowercase extension without the dot, or `""`. Follows `pathlib.Path.suffix`:
    /// the text after the last dot, unless the dot is the first or last character.
    #[must_use]
    pub fn extension(&self) -> String {
        match self.name.rfind('.') {
            Some(i) if i > 0 && i + 1 < self.name.len() => {
                self.name[i + 1..].trim_start_matches('.').to_lowercase()
            }
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AudioFile;
    use std::path::PathBuf;

    fn ext(name: &str) -> String {
        AudioFile::new(PathBuf::from(name), name.to_owned()).extension()
    }

    #[test]
    fn extension_follows_pathlib_suffix() {
        assert_eq!(ext("a.WAV"), "wav");
        assert_eq!(ext(".wav"), "");
        assert_eq!(ext("a."), "");
        assert_eq!(ext("a.b.flac"), "flac");
        assert_eq!(ext("..wav"), "wav");
        assert_eq!(ext("noext"), "");
    }
}
