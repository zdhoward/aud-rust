//! Core library for `aud`. Behavior lives here; the CLI crate only does terminal I/O.

#![forbid(unsafe_code)]

/// Version of this library, taken from the workspace manifest.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::version;

    #[test]
    fn version_matches_manifest() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
        assert_ne!(version(), "");
    }
}
