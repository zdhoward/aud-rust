//! Versioned JSON interchange documents (`aud.contracts`).

use std::fmt::Write as _;

use serde_json::{Map, Value};

use crate::error::AudError;

/// Document kinds defined by v1.
pub const KINDS: [&str; 7] = [
    "result",
    "preview",
    "album_preview",
    "progress",
    "list",
    "inspect",
    "error",
];

const SCHEMA_V1: &str = include_str!("../../../schemas/v1.json");

/// Wrap `payload` in a v1 envelope. `payload` is a [`Value`], which cannot hold
/// NaN or infinity, so non-finite numbers are rejected by construction.
///
/// # Errors
/// [`AudError::Usage`] for an unknown `kind` or a payload that is not an object.
pub fn json_document(kind: &str, payload: Value) -> Result<Value, AudError> {
    if !KINDS.contains(&kind) {
        return Err(AudError::Usage(format!(
            "Unknown JSON document kind: {kind}"
        )));
    }
    if !payload.is_object() {
        return Err(AudError::Usage(
            "JSON document payload must be an object".to_owned(),
        ));
    }
    let mut doc = Map::new();
    doc.insert("schema_version".to_owned(), Value::from(1));
    doc.insert("kind".to_owned(), Value::from(kind));
    doc.insert("payload".to_owned(), payload);
    Ok(Value::Object(doc))
}

/// The bundled v1 schema (no network access).
///
/// # Panics
/// Never in practice: the embedded file is validated by this crate's tests.
#[must_use]
pub fn json_schema() -> Value {
    serde_json::from_str(SCHEMA_V1).expect("embedded schemas/v1.json is valid JSON (tested)")
}

/// Render JSON like Python's `json.dumps(..., ensure_ascii=True)`: every
/// non-ASCII character becomes `\uXXXX` (surrogate pairs above the BMP).
/// `indent` selects 2-space pretty output.
#[must_use]
pub fn to_ascii_json(value: &Value, indent: bool) -> String {
    let text = if indent {
        serde_json::to_string_pretty(value)
    } else {
        serde_json::to_string(value)
    }
    // A `Value` has string keys and finite numbers only, so serialization cannot fail.
    .unwrap_or_default();
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_ascii() {
            out.push(c);
        } else {
            let mut units = [0u16; 2];
            for unit in c.encode_utf16(&mut units) {
                let _ = write!(out, "\\u{unit:04x}");
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{KINDS, json_document, json_schema, to_ascii_json};
    use serde_json::json;

    #[test]
    fn envelope_shape() {
        let doc = json_document("list", json!({"files": ["a.wav"]})).expect("doc");
        assert_eq!(
            doc,
            json!({"schema_version": 1, "kind": "list", "payload": {"files": ["a.wav"]}})
        );
    }

    #[test]
    fn rejects_unknown_kind_and_non_object() {
        assert!(json_document("nope", json!({})).is_err());
        assert!(json_document("list", json!([1])).is_err());
    }

    #[test]
    fn kinds_match_the_schema_enum() {
        let schema = json_schema();
        let listed: Vec<&str> = schema["properties"]["kind"]["enum"]
            .as_array()
            .expect("enum")
            .iter()
            .filter_map(|v| v.as_str())
            .collect();
        assert_eq!(listed, KINDS);
    }

    #[test]
    fn non_finite_numbers_are_unrepresentable() {
        assert!(serde_json::Number::from_f64(f64::NAN).is_none());
        assert!(serde_json::Number::from_f64(f64::INFINITY).is_none());
    }

    #[test]
    fn ascii_escaping_matches_python() {
        let v = json!({"files": ["é.wav", "\u{1F3B5}.wav", "tab\t\"q\""]});
        assert_eq!(
            to_ascii_json(&v, false),
            // Same text as Python json.dumps(ensure_ascii=True); @ stands for a backslash.
            r#"{"files":["@u00e9.wav","@ud83c@udfb5.wav","tab@t@"q@""]}"#.replace('@', "\\")
        );
        assert_eq!(to_ascii_json(&json!({"a": []}), true), "{\n  \"a\": []\n}");
    }
}
