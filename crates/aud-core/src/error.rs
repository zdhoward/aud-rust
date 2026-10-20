//! Typed library errors. Variants mirror the Python exception families that
//! the CLI maps to exit codes.

use std::path::PathBuf;

/// Everything the library can report.
#[derive(Debug, thiserror::Error)]
pub enum AudError {
    /// A filesystem problem: missing directory, unreadable entry, non-UTF-8 name.
    #[error("{message}")]
    File {
        /// Human-readable description.
        message: String,
        /// Underlying I/O error, when there is one.
        #[source]
        source: Option<std::io::Error>,
    },
    /// Invalid caller input: bad option, bad pattern, bad document request.
    #[error("{0}")]
    Usage(String),
    /// A selection index outside the selection (Python `IndexError`).
    #[error("Index {index} is out of range for a selection of {len} files")]
    Index {
        /// Requested index.
        index: i64,
        /// Number of selected files.
        len: usize,
    },
}

impl AudError {
    pub(crate) fn file(message: impl Into<String>) -> Self {
        Self::File {
            message: message.into(),
            source: None,
        }
    }

    pub(crate) fn io(message: impl Into<String>, source: std::io::Error) -> Self {
        Self::File {
            message: message.into(),
            source: Some(source),
        }
    }

    pub(crate) fn not_utf8(entry: &std::ffi::OsStr, directory: &std::path::Path) -> Self {
        let lossy = PathBuf::from(entry).to_string_lossy().into_owned();
        Self::file(format!(
            "File name is not valid Unicode: {lossy:?} in {}",
            directory.display()
        ))
    }
}
