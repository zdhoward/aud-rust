//! Link detection (trap T-6).

use std::path::Path;

/// True for symlinks and, on Windows, any reparse point (junctions included).
/// A path that does not exist is not a link.
#[must_use]
pub fn is_link(path: &Path) -> bool {
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return false;
    };
    if meta.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        if meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::is_link;

    #[test]
    fn missing_path_is_not_a_link() {
        assert!(!is_link(std::path::Path::new("definitely/not/here")));
    }

    #[test]
    fn regular_file_is_not_a_link() {
        let dir = tempfile::tempdir().expect("tempdir");
        let file = dir.path().join("a.wav");
        std::fs::write(&file, b"x").expect("write");
        assert!(!is_link(&file));
    }
}
