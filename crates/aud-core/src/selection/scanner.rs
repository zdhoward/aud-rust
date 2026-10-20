//! Single-directory scan: sorted, regular files only, links skipped by default.

use std::path::{Path, PathBuf};

use super::SelectionPolicy;
use crate::error::AudError;
use crate::model::AudioFile;
use crate::safety::is_link;

/// Scans one directory (not recursive).
#[derive(Debug)]
pub struct DirectoryScanner<'a> {
    directory: PathBuf,
    policy: &'a SelectionPolicy,
    follow_symlinks: bool,
}

impl<'a> DirectoryScanner<'a> {
    /// Create a scanner over `directory`.
    #[must_use]
    pub fn new(directory: &Path, policy: &'a SelectionPolicy, follow_symlinks: bool) -> Self {
        Self {
            directory: directory.to_path_buf(),
            policy,
            follow_symlinks,
        }
    }

    /// Return selected files in Python `sorted(Path)` order.
    ///
    /// # Errors
    /// [`AudError::File`] when the directory cannot be read or an entry name is
    /// not valid Unicode (ADR-0004).
    pub fn scan(&self) -> Result<Vec<AudioFile>, AudError> {
        let read = std::fs::read_dir(&self.directory)
            .map_err(|e| AudError::io(format!("Cannot read {}", self.directory.display()), e))?;
        let mut entries: Vec<(String, PathBuf)> = Vec::new();
        for entry in read {
            let entry = entry.map_err(|e| {
                AudError::io(format!("Cannot read {}", self.directory.display()), e)
            })?;
            let name = entry
                .file_name()
                .into_string()
                .map_err(|raw| AudError::not_utf8(&raw, &self.directory))?;
            entries.push((name, entry.path()));
        }
        entries.sort_by(|a, b| {
            sort_key(&a.0)
                .cmp(&sort_key(&b.0))
                .then_with(|| a.0.cmp(&b.0))
        });

        let mut files = Vec::new();
        for (name, path) in entries {
            if !self.follow_symlinks && is_link(&path) {
                continue;
            }
            if !path.is_file() {
                continue;
            }
            let audio = AudioFile::new(path.clone(), name.clone());
            if !self.policy.include(&audio) {
                continue;
            }
            if self.follow_symlinks {
                let resolved = std::fs::canonicalize(&path)
                    .map_err(|e| AudError::io(format!("Cannot resolve {}", path.display()), e))?;
                files.push(AudioFile::new(resolved, name));
            } else {
                files.push(audio);
            }
        }
        Ok(files)
    }
}

/// Python orders `Path` objects by their platform-normalized parts: lowercased
/// on Windows, as-is elsewhere. Rust `String` order is code point order (T-2).
fn sort_key(name: &str) -> String {
    if cfg!(windows) {
        name.to_lowercase()
    } else {
        name.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::DirectoryScanner;
    use crate::selection::SelectionPolicy;

    fn wav() -> SelectionPolicy {
        SelectionPolicy::new(&["wav".to_owned()], None, None)
    }

    fn touch(dir: &std::path::Path, names: &[&str]) {
        for n in names {
            std::fs::write(dir.join(n), b"x").expect("write");
        }
    }

    fn names(dir: &std::path::Path, policy: &SelectionPolicy) -> Vec<String> {
        DirectoryScanner::new(dir, policy, false)
            .scan()
            .expect("scan")
            .iter()
            .map(|f| f.name().to_owned())
            .collect()
    }

    #[test]
    fn sorts_by_code_point_including_astral() {
        let dir = tempfile::tempdir().expect("tempdir");
        // U+FFEE (BMP) must sort before U+1F3B5 (astral): code point, not UTF-16, order.
        touch(
            dir.path(),
            &["\u{1F3B5}.wav", "\u{FFEE}.wav", "b.wav", "a2.wav"],
        );
        assert_eq!(
            names(dir.path(), &wav()),
            ["a2.wav", "b.wav", "\u{FFEE}.wav", "\u{1F3B5}.wav"]
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_sort_ignores_case_like_pathlib() {
        let dir = tempfile::tempdir().expect("tempdir");
        touch(dir.path(), &["b.wav", "A.wav", "Z.wav", "a2.wav"]);
        assert_eq!(
            names(dir.path(), &wav()),
            ["A.wav", "a2.wav", "b.wav", "Z.wav"]
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn posix_sort_is_case_sensitive() {
        let dir = tempfile::tempdir().expect("tempdir");
        touch(dir.path(), &["b.wav", "A.wav", "Z.wav", "a2.wav"]);
        assert_eq!(
            names(dir.path(), &wav()),
            ["A.wav", "Z.wav", "a2.wav", "b.wav"]
        );
    }

    #[test]
    fn skips_directories_and_unselected_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        touch(dir.path(), &["a.wav", "b.txt"]);
        std::fs::create_dir(dir.path().join("sub.wav")).expect("mkdir");
        assert_eq!(names(dir.path(), &wav()), ["a.wav"]);
    }

    #[test]
    fn missing_directory_is_a_file_error() {
        let policy = wav();
        let err = DirectoryScanner::new(std::path::Path::new("no/such/dir"), &policy, false)
            .scan()
            .expect_err("missing");
        assert!(matches!(err, crate::AudError::File { .. }));
    }

    #[cfg(unix)]
    #[test]
    fn non_utf8_name_is_a_typed_error() {
        use std::os::unix::ffi::OsStrExt;
        let dir = tempfile::tempdir().expect("tempdir");
        let bad = dir.path().join(std::ffi::OsStr::from_bytes(b"bad\xff.wav"));
        if std::fs::write(&bad, b"x").is_err() {
            return; // filesystem refuses such names
        }
        let policy = wav();
        let err = DirectoryScanner::new(dir.path(), &policy, false)
            .scan()
            .expect_err("non-utf8");
        assert!(err.to_string().contains("not valid Unicode"));
    }

    #[cfg(unix)]
    #[test]
    fn links_are_skipped_unless_followed() {
        let dir = tempfile::tempdir().expect("tempdir");
        touch(dir.path(), &["real.wav"]);
        std::os::unix::fs::symlink(dir.path().join("real.wav"), dir.path().join("link.wav"))
            .expect("symlink");
        let policy = wav();
        assert_eq!(names(dir.path(), &policy), ["real.wav"]);
        let followed = DirectoryScanner::new(dir.path(), &policy, true)
            .scan()
            .expect("scan");
        let found: Vec<_> = followed.iter().map(|f| f.name().to_owned()).collect();
        assert_eq!(found, ["link.wav", "real.wav"]);
        assert!(followed.iter().all(|f| f.path().ends_with("real.wav")));
    }
}
