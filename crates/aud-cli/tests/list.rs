//! `aud list` end to end; ports the list parts of Python `test_cli.py` and
//! `test_contracts.py`.

mod common;

use common::{assert_schema_valid, aud, tree};
use serde_json::json;

fn populated() -> tempfile::TempDir {
    tree(&["test.txt", "abc.txt", "bloop.wav", "song.wav"])
}

#[test]
fn default_selection_is_empty_and_pattern_narrows() {
    let dir = populated();
    let d = dir.path().to_str().expect("utf-8 path");
    let out = aud(&["list", d, "--json"]);
    assert_eq!((out.code, out.json()), (0, json!({"files": []})));
    let out = aud(&[
        "list",
        d,
        "--extensions",
        "wav",
        "--pattern",
        "bloop.*",
        "--json",
    ]);
    assert_eq!((out.code, out.json()), (0, json!({"files": ["bloop.wav"]})));
}

#[test]
fn versioned_document_matches_schema() {
    let dir = populated();
    let d = dir.path().to_str().expect("utf-8 path");
    let out = aud(&[
        "list",
        d,
        "--extensions",
        "wav",
        "--json",
        "--json-version",
        "1",
    ]);
    assert_eq!(out.code, 0);
    let doc = out.json();
    assert_eq!(
        doc,
        json!({"schema_version": 1, "kind": "list", "payload": {"files": ["bloop.wav", "song.wav"]}})
    );
    assert_schema_valid(&doc);
}

#[test]
fn human_output_is_indented_json_on_stdout() {
    let dir = populated();
    let d = dir.path().to_str().expect("utf-8 path");
    let out = aud(&["list", d, "--extensions", "wav"]);
    assert_eq!(out.code, 0);
    assert_eq!(
        out.stdout,
        "{\n  \"files\": [\n    \"bloop.wav\",\n    \"song.wav\"\n  ]\n}\n"
    );
    assert_eq!(out.stderr, "");
}

#[test]
fn non_ascii_names_are_escaped() {
    let dir = tree(&["caf\u{e9}.wav", "\u{1F3B5}.wav"]);
    let d = dir.path().to_str().expect("utf-8 path");
    let out = aud(&["list", d, "--extensions", "wav", "--json"]);
    assert!(out.stdout.is_ascii(), "{}", out.stdout);
    assert_eq!(
        out.json(),
        json!({"files": ["caf\u{e9}.wav", "\u{1F3B5}.wav"]})
    );
}

#[test]
fn missing_directory_is_a_schema_valid_error_document() {
    let dir = tree(&[]);
    let missing = dir.path().join("nope");
    let m = missing.to_str().expect("utf-8 path");
    let out = aud(&["list", m, "--json", "--json-version", "1"]);
    assert_eq!(out.code, 2);
    let doc = out.json();
    assert_eq!(doc["kind"], "error");
    assert_eq!(doc["payload"]["exit_code"], 2);
    assert_eq!(doc["payload"]["succeeded"], false);
    assert_eq!(doc["payload"]["result"], json!(null));
    assert!(
        doc["payload"]["error"]
            .as_str()
            .expect("text")
            .starts_with("Not a directory: ")
    );
    assert_schema_valid(&doc);
}

#[test]
fn missing_directory_without_json_reports_on_stderr() {
    let dir = tree(&[]);
    let missing = dir.path().join("nope");
    let out = aud(&["list", missing.to_str().expect("utf-8 path")]);
    assert_eq!((out.code, out.stdout.as_str()), (2, ""));
    let doc: serde_json::Value = serde_json::from_str(&out.stderr).expect("stderr is JSON");
    assert_eq!(doc["exit_code"], 2);
}

#[test]
fn json_version_requires_json() {
    let dir = populated();
    let out = aud(&[
        "list",
        dir.path().to_str().expect("utf-8 path"),
        "--json-version",
        "1",
    ]);
    assert_eq!(out.code, 2);
    assert!(out.stderr.contains("requires --json"), "{}", out.stderr);
}

#[test]
fn usage_errors_with_json_use_the_legacy_shape() {
    let dir = populated();
    let out = aud(&[
        "list",
        dir.path().to_str().expect("utf-8 path"),
        "--json",
        "--bogus",
    ]);
    assert_eq!(out.code, 2);
    let doc = out.json();
    assert_eq!(doc["succeeded"], false);
    assert_eq!(doc["exit_code"], 2);
    assert_eq!(doc["result"], json!(null));
    assert!(doc.get("schema_version").is_none());
}

#[test]
fn unsupported_json_version_is_a_usage_error() {
    let dir = populated();
    let out = aud(&[
        "list",
        dir.path().to_str().expect("utf-8 path"),
        "--json",
        "--json-version",
        "2",
    ]);
    assert_eq!(out.code, 2);
}

#[test]
fn legacy_count_form() {
    let dir = populated();
    let out = aud(&[dir.path().to_str().expect("utf-8 path")]);
    assert_eq!((out.code, out.stdout.as_str()), (0, "Found 0 files\n"));
    let out = aud(&[]);
    assert_eq!(out.code, 1);
    assert!(out.stdout.contains("Usage"));
    let missing = dir.path().join("does-not-exist");
    let out = aud(&[missing.to_str().expect("utf-8 path")]);
    assert_eq!(out.code, 1);
    assert!(out.stdout.contains("Directory does not exist"));
}

#[test]
fn accepts_parity_flags() {
    let dir = populated();
    let d = dir.path().to_str().expect("utf-8 path");
    let out = aud(&[
        "list",
        d,
        "--progress",
        "--timeout",
        "5",
        "--output-bytes",
        "10",
        "--json",
    ]);
    assert_eq!(out.code, 0);
}
