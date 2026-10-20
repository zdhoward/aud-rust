//! Shared test helpers: temp trees, `$ROOT` normalization, schema validation.
#![allow(dead_code)]

use std::path::Path;

use assert_cmd::Command;
use serde_json::Value;

/// Build a temp directory containing empty files with the given names.
pub fn tree(names: &[&str]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    for name in names {
        std::fs::write(dir.path().join(name), b"x").expect("write");
    }
    dir
}

/// Replace the temp-root prefix with `$ROOT` and use `/` separators in every
/// string that starts with it (conformance README).
pub fn portable(value: &Value, root: &Path) -> Value {
    let root = root.to_string_lossy().into_owned();
    match value {
        Value::String(s) if s.starts_with(&root) => Value::String(format!(
            "$ROOT{}",
            s[root.len()..].replace(std::path::MAIN_SEPARATOR, "/")
        )),
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|v| portable(v, Path::new(&root)))
                .collect(),
        ),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), portable(v, Path::new(&root))))
                .collect(),
        ),
        other => other.clone(),
    }
}

/// Assert `doc` is valid against the embedded v1 schema.
pub fn assert_schema_valid(doc: &Value) {
    let schema = aud_core::contracts::json_schema();
    let validator = jsonschema::draft202012::new(&schema).expect("schema compiles");
    let errors: Vec<String> = validator.iter_errors(doc).map(|e| e.to_string()).collect();
    assert!(
        errors.is_empty(),
        "schema errors: {errors:?}\ndocument: {doc}"
    );
}

pub struct Output {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl Output {
    pub fn json(&self) -> Value {
        serde_json::from_str(&self.stdout)
            .unwrap_or_else(|e| panic!("stdout is not JSON ({e}): {:?}", self.stdout))
    }
}

/// Run the real `aud` binary.
pub fn aud(args: &[&str]) -> Output {
    let out = Command::cargo_bin("aud")
        .expect("aud builds")
        .args(args)
        .output()
        .expect("run aud");
    Output {
        code: out.status.code().expect("exit code"),
        stdout: String::from_utf8(out.stdout).expect("utf-8 stdout"),
        stderr: String::from_utf8(out.stderr).expect("utf-8 stderr"),
    }
}
