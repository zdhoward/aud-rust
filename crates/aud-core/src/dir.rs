//! The directory selection facade (`aud.Dir` selection half).

use std::path::{Path, PathBuf};

use crate::error::AudError;
use crate::model::AudioFile;
use crate::selection::{DirectoryScanner, Glob, Listing, SelectionPolicy};

/// Selection settings for a [`Dir`].
#[derive(Debug, Clone, Default)]
pub struct DirOptions {
    /// Valid extensions. Empty selects nothing except allowlisted names.
    pub extensions: Vec<String>,
    /// Names that are always selected.
    pub allow_names: Vec<String>,
    /// Start-anchored regex for always-selected names.
    pub allow_regex: Option<String>,
    /// Names that are never selected by extension.
    pub deny_names: Vec<String>,
    /// Start-anchored regex for denied names.
    pub deny_regex: Option<String>,
    /// Include symlinks (resolved) instead of skipping them.
    pub follow_symlinks: bool,
    /// Glob that narrows the selection after the allowlist override.
    pub pattern: Option<String>,
}

/// A directory plus the files its selection rules choose.
#[derive(Debug)]
pub struct Dir {
    directory: PathBuf,
    files: Vec<AudioFile>,
}

impl Dir {
    /// Open `directory` and scan it once.
    ///
    /// # Errors
    /// [`AudError::File`] when `directory` is not a directory or cannot be read;
    /// [`AudError::Usage`] for an invalid regex.
    pub fn open(directory: &Path, options: &DirOptions) -> Result<Self, AudError> {
        let directory = std::path::absolute(directory)
            .map_err(|e| AudError::io(format!("Cannot resolve {}", directory.display()), e))?;
        if !directory.is_dir() {
            return Err(AudError::file(format!(
                "Not a directory: {}",
                directory.display()
            )));
        }
        let listing =
            |names: &[String], regex: &Option<String>| -> Result<Option<Listing>, AudError> {
                if names.is_empty() && regex.as_deref().is_none_or(str::is_empty) {
                    return Ok(None);
                }
                Listing::new(names, regex.as_deref()).map(Some)
            };
        let policy = SelectionPolicy::new(
            &options.extensions,
            listing(&options.allow_names, &options.allow_regex)?,
            listing(&options.deny_names, &options.deny_regex)?,
        );
        let mut files =
            DirectoryScanner::new(&directory, &policy, options.follow_symlinks).scan()?;
        if let Some(pattern) = &options.pattern {
            let glob = Glob::new(pattern);
            files.retain(|f| glob.is_match(f.name()));
        }
        Ok(Self { directory, files })
    }

    /// Absolute directory path.
    #[must_use]
    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// Selected files in order.
    #[must_use]
    pub fn files(&self) -> &[AudioFile] {
        &self.files
    }

    /// One selected name by position; negative indexes count from the end.
    ///
    /// # Errors
    /// [`AudError::Index`] when `num` is out of range.
    pub fn get_single(&self, num: i64) -> Result<String, AudError> {
        let len = self.files.len();
        let resolved = if num < 0 {
            i64::try_from(len)
                .ok()
                .and_then(|l| usize::try_from(l + num).ok())
        } else {
            usize::try_from(num).ok()
        };
        resolved
            .and_then(|i| self.files.get(i))
            .map(|f| f.name().to_owned())
            .ok_or(AudError::Index { index: num, len })
    }

    /// Selected file names in order (`Dir.get_all()`).
    #[must_use]
    pub fn get_all(&self) -> Vec<String> {
        self.files.iter().map(|f| f.name().to_owned()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Dir, DirOptions};

    fn tree(names: &[&str]) -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        for n in names {
            std::fs::write(dir.path().join(n), b"x").expect("write");
        }
        dir
    }

    fn opts(ext: &[&str]) -> DirOptions {
        DirOptions {
            extensions: ext.iter().map(|s| (*s).to_owned()).collect(),
            ..DirOptions::default()
        }
    }

    #[test]
    fn default_selects_nothing() {
        let dir = tree(&["a.wav"]);
        let d = Dir::open(dir.path(), &DirOptions::default()).expect("open");
        assert_eq!(d.get_all(), Vec::<String>::new());
    }

    #[test]
    fn pattern_narrows_after_extensions() {
        let dir = tree(&["01 a.wav", "02 b.wav", "01 c.txt"]);
        let mut o = opts(&["wav"]);
        o.pattern = Some("01*".to_owned());
        assert_eq!(
            Dir::open(dir.path(), &o).expect("open").get_all(),
            ["01 a.wav"]
        );
    }

    #[test]
    fn pattern_also_narrows_allowlisted_files() {
        let dir = tree(&["keep.txt", "drop.txt"]);
        let o = DirOptions {
            allow_names: vec!["keep.txt".into(), "drop.txt".into()],
            pattern: Some("k*".into()),
            ..DirOptions::default()
        };
        assert_eq!(
            Dir::open(dir.path(), &o).expect("open").get_all(),
            ["keep.txt"]
        );
    }

    #[test]
    fn pattern_is_case_sensitive_everywhere() {
        let dir = tree(&["A.wav"]);
        let mut o = opts(&["wav"]);
        o.pattern = Some("a*".into());
        assert_eq!(
            Dir::open(dir.path(), &o).expect("open").get_all(),
            Vec::<String>::new()
        );
    }

    #[test]
    fn missing_directory_errors() {
        let dir = tree(&[]);
        let err = Dir::open(&dir.path().join("nope"), &DirOptions::default()).expect_err("missing");
        assert!(err.to_string().starts_with("Not a directory: "));
    }
}

/// Ports of the selection and API tests in Python's `test_dir_selection.py`,
/// `test_dir_config.py` and `test_dir_api.py`.
#[cfg(test)]
mod ported_tests {
    use super::{Dir, DirOptions};

    fn populated() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        for n in ["test.txt", "abc.txt", "bloop.wav", "song.wav"] {
            std::fs::write(dir.path().join(n), b"x").expect("write");
        }
        dir
    }

    fn names(dir: &tempfile::TempDir, o: &DirOptions) -> Vec<String> {
        Dir::open(dir.path(), o).expect("open").get_all()
    }

    fn s(items: &[&str]) -> Vec<String> {
        items.iter().map(|i| (*i).to_owned()).collect()
    }

    #[test]
    fn extension_filtering() {
        let o = DirOptions {
            extensions: s(&["wav"]),
            ..DirOptions::default()
        };
        assert_eq!(names(&populated(), &o), ["bloop.wav", "song.wav"]);
    }

    #[test]
    fn allowlist_overrides_extensions() {
        let o = DirOptions {
            extensions: s(&["wav"]),
            allow_names: s(&["test.txt"]),
            ..DirOptions::default()
        };
        assert_eq!(
            names(&populated(), &o),
            ["bloop.wav", "song.wav", "test.txt"]
        );
    }

    #[test]
    fn denylist_excludes() {
        let o = DirOptions {
            extensions: s(&["txt"]),
            deny_names: s(&["test.txt"]),
            ..DirOptions::default()
        };
        assert_eq!(names(&populated(), &o), ["abc.txt"]);
    }

    #[test]
    fn allowlist_overrides_denylist() {
        let o = DirOptions {
            extensions: s(&["txt"]),
            deny_names: s(&["test.txt"]),
            allow_names: s(&["test.txt"]),
            ..DirOptions::default()
        };
        assert_eq!(names(&populated(), &o), ["abc.txt", "test.txt"]);
    }

    #[test]
    fn allowlist_regex_adds_files() {
        let o = DirOptions {
            extensions: s(&["txt"]),
            allow_regex: Some("test".into()),
            ..DirOptions::default()
        };
        assert_eq!(names(&populated(), &o), ["abc.txt", "test.txt"]);
    }

    #[test]
    fn denylist_regex_removes_files() {
        let o = DirOptions {
            extensions: s(&["txt"]),
            deny_regex: Some("test".into()),
            ..DirOptions::default()
        };
        assert_eq!(names(&populated(), &o), ["abc.txt"]);
    }

    #[test]
    fn get_single_supports_positive_and_negative_indexes() {
        let dir = populated();
        let o = DirOptions {
            extensions: s(&["wav"]),
            ..DirOptions::default()
        };
        let d = Dir::open(dir.path(), &o).expect("open");
        assert_eq!(d.get_single(0).expect("0"), "bloop.wav");
        assert_eq!(d.get_single(1).expect("1"), "song.wav");
        assert_eq!(d.get_single(-1).expect("-1"), "song.wav");
        assert_eq!(d.get_single(-2).expect("-2"), "bloop.wav");
    }

    #[test]
    fn get_single_rejects_out_of_range() {
        let dir = populated();
        let o = DirOptions {
            extensions: s(&["wav"]),
            ..DirOptions::default()
        };
        let d = Dir::open(dir.path(), &o).expect("open");
        for num in [2, 3, -3, -100, i64::MIN, i64::MAX] {
            let err = d.get_single(num).expect_err("out of range");
            assert!(err.to_string().contains("out of range"), "{num}");
        }
    }

    #[test]
    fn rejects_a_file_path() {
        let dir = populated();
        let err = Dir::open(&dir.path().join("bloop.wav"), &DirOptions::default())
            .expect_err("file path");
        assert!(err.to_string().contains("Not a directory"));
    }
}
