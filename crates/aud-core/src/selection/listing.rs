//! Allowlist / denylist entries: exact names plus an optional start-anchored regex.

use regex::Regex;

use crate::error::AudError;

/// A set of names and an optional regex. Python used `re.match` (anchored at the
/// start only); the `regex` crate dialect is linear-time and differs (SG-2).
#[derive(Debug, Clone)]
pub struct Listing {
    names: Vec<String>,
    regex: Option<Regex>,
}

impl Listing {
    /// Build a listing.
    ///
    /// # Errors
    /// [`AudError::Usage`] when `regex` does not compile.
    pub fn new(names: &[String], regex: Option<&str>) -> Result<Self, AudError> {
        let regex = regex
            .filter(|r| !r.is_empty())
            .map(|r| {
                Regex::new(&format!(r"\A(?:{r})"))
                    .map_err(|e| AudError::Usage(format!("Invalid selection regex: {e}")))
            })
            .transpose()?;
        Ok(Self {
            names: names.to_vec(),
            regex,
        })
    }

    /// True when `name` equals an entry or the regex matches at its start.
    #[must_use]
    pub fn matches(&self, name: &str) -> bool {
        self.names.iter().any(|n| n == name)
            || self.regex.as_ref().is_some_and(|r| r.is_match(name))
    }
}

#[cfg(test)]
mod tests {
    use super::Listing;

    #[test]
    fn regex_is_anchored_at_start() {
        let listing = Listing::new(&[], Some("take_")).expect("regex");
        assert!(listing.matches("take_01.wav"));
        assert!(!listing.matches("retake_01.wav"));
    }

    #[test]
    fn invalid_regex_is_a_usage_error() {
        assert!(Listing::new(&[], Some("(")).is_err());
    }
}
