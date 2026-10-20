//! Schema validation of documents: the corpus fixtures and our own envelopes.

use aud_core::contracts::{json_document, json_schema};
use serde_json::{Value, json};

fn validator() -> jsonschema::Validator {
    jsonschema::draft202012::new(&json_schema()).expect("embedded schema compiles")
}

fn cases() -> Value {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../conformance/v1/cases.json"
    ))
    .expect("read cases.json");
    serde_json::from_str(&text).expect("cases.json parses")
}

#[test]
fn corpus_progress_and_error_fixtures_validate() {
    let v = validator();
    let cases = cases();
    for key in ["progress", "error"] {
        assert!(v.is_valid(&cases[key]), "{key} fixture should validate");
    }
}

#[test]
fn corpus_copy_documents_validate() {
    let v = validator();
    let cases = cases();
    for key in ["preview", "result"] {
        assert!(
            v.is_valid(&cases["copy"][key]),
            "copy.{key} should validate"
        );
    }
}

#[test]
fn list_documents_validate() {
    let v = validator();
    let doc = json_document("list", json!({"files": ["a.wav", "é.flac"]})).expect("doc");
    assert!(v.is_valid(&doc));
    let empty = json_document("list", json!({"files": []})).expect("doc");
    assert!(v.is_valid(&empty));
}

#[test]
fn schema_rejects_bad_documents() {
    let v = validator();
    assert!(!v.is_valid(&json!({"schema_version": 2, "kind": "list", "payload": {"files": []}})));
    assert!(!v.is_valid(&json!({"schema_version": 1, "kind": "list", "payload": {"files": [1]}})));
    assert!(
        !v.is_valid(
            &json!({"schema_version": 1, "kind": "list", "payload": {"files": [], "x": 1}})
        )
    );
    assert!(!v.is_valid(&json!({"schema_version": 1, "kind": "bogus", "payload": {}})));
    // exit_code outside {1, 2, 130}
    let bad_error = json!({"schema_version": 1, "kind": "error",
        "payload": {"succeeded": false, "error": "x", "exit_code": 3, "result": null}});
    assert!(!v.is_valid(&bad_error));
}
