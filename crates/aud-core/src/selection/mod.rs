//! Which files in a directory are selected.
//!
//! Rule (legacy parity): `(valid_extension AND NOT denylisted) OR allowlisted`.
//! With no extensions, nothing is selected except allowlisted names.

mod fnmatch;
mod listing;
mod scanner;

pub use fnmatch::Glob;
pub use listing::Listing;
pub use scanner::DirectoryScanner;

use crate::model::AudioFile;

/// Decides whether a file is selected.
#[derive(Debug, Clone, Default)]
pub struct SelectionPolicy {
    extensions: Vec<String>,
    allow: Option<Listing>,
    deny: Option<Listing>,
}

impl SelectionPolicy {
    /// Build a policy. Extensions are lowercased and stripped of leading dots.
    #[must_use]
    pub fn new(extensions: &[String], allow: Option<Listing>, deny: Option<Listing>) -> Self {
        let extensions = extensions
            .iter()
            .map(|e| e.to_lowercase().trim_start_matches('.').to_owned())
            .collect();
        Self {
            extensions,
            allow,
            deny,
        }
    }

    /// True when `file` is selected.
    #[must_use]
    pub fn include(&self, file: &AudioFile) -> bool {
        if self.allow.as_ref().is_some_and(|a| a.matches(file.name())) {
            return true;
        }
        let valid = !self.extensions.is_empty() && self.extensions.contains(&file.extension());
        let denied = self.deny.as_ref().is_some_and(|d| d.matches(file.name()));
        valid && !denied
    }
}

#[cfg(test)]
mod tests {
    use super::{Listing, SelectionPolicy};
    use crate::model::AudioFile;
    use std::path::PathBuf;

    fn file(name: &str) -> AudioFile {
        AudioFile::new(PathBuf::from(name), name.to_owned())
    }

    fn exts(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn no_extensions_selects_nothing() {
        let policy = SelectionPolicy::new(&[], None, None);
        assert!(!policy.include(&file("a.wav")));
    }

    #[test]
    fn extensions_are_case_insensitive_and_dot_tolerant() {
        let policy = SelectionPolicy::new(&exts(&[".WAV", "flac"]), None, None);
        assert!(policy.include(&file("a.wav")));
        assert!(policy.include(&file("b.FLAC")));
        assert!(!policy.include(&file("c.mp3")));
    }

    #[test]
    fn denylist_removes_and_allowlist_overrides() {
        let deny = Listing::new(&exts(&["skip.wav"]), None).expect("listing");
        let allow = Listing::new(&exts(&["skip.wav", "note.txt"]), None).expect("listing");
        let denied = SelectionPolicy::new(&exts(&["wav"]), None, Some(deny.clone()));
        assert!(!denied.include(&file("skip.wav")));
        assert!(denied.include(&file("keep.wav")));
        let both = SelectionPolicy::new(&exts(&["wav"]), Some(allow), Some(deny));
        assert!(both.include(&file("skip.wav")));
        assert!(both.include(&file("note.txt")));
    }

    #[test]
    fn allowlist_works_without_extensions() {
        let allow = Listing::new(&exts(&["only.txt"]), None).expect("listing");
        let policy = SelectionPolicy::new(&[], Some(allow), None);
        assert!(policy.include(&file("only.txt")));
        assert!(!policy.include(&file("other.txt")));
    }
}
