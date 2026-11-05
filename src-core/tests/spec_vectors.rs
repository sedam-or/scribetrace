//! Conformance of the reference reader to `spec/FORMAT.md`: every sample line in `spec/vectors/`
//! gets the verdict the manifest promises, and the JSON Schema cannot drift from the code.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};

use scribetrace_core::domain::{classify_line, LineClass, MAX_LINE_BYTES};
use serde_json::Value;

fn spec_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../spec")
}

fn verdict(line: &[u8]) -> String {
    match classify_line(line) {
        LineClass::Valid(_) => "valid".into(),
        LineClass::Other { .. } => "other".into(),
        LineClass::Bad(b) => b.code().into(),
    }
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

#[test]
fn every_vector_gets_the_promised_verdict() {
    let dir = spec_dir().join("vectors");
    let manifest = read_json(&dir.join("manifest.json"));
    let vectors = manifest["vectors"].as_array().unwrap();
    assert!(vectors.len() >= 20, "manifest looks truncated");
    for v in vectors {
        let file = v["file"].as_str().unwrap();
        let expect = v["expect"].as_str().unwrap();
        let bytes = std::fs::read(dir.join(file)).unwrap();
        let line = bytes.strip_suffix(b"\n").unwrap_or(&bytes);
        assert_eq!(verdict(line), expect, "{file}");
    }
}

#[test]
fn manifest_lists_every_vector_file() {
    let dir = spec_dir().join("vectors");
    let manifest = read_json(&dir.join("manifest.json"));
    let listed: Vec<&str> = manifest["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["file"].as_str().unwrap())
        .collect();
    for sub in ["good", "bad"] {
        for entry in std::fs::read_dir(dir.join(sub)).unwrap() {
            let name = format!("{sub}/{}", entry.unwrap().file_name().to_string_lossy());
            assert!(
                listed.contains(&name.as_str()),
                "{name} is not in manifest.json"
            );
        }
    }
}

#[test]
fn every_bad_reason_code_has_a_vector() {
    let manifest = read_json(&spec_dir().join("vectors/manifest.json"));
    let expects: Vec<&str> = manifest["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["expect"].as_str().unwrap())
        .collect();
    // `too_long` is generated below, not stored.
    for code in [
        "not_json",
        "not_object",
        "missing_field",
        "bad_value",
        "unsupported_version",
        "bad_payload",
    ] {
        assert!(expects.contains(&code), "no vector for {code}");
    }
}

#[test]
fn a_line_over_one_mebibyte_is_too_long() {
    assert_eq!(
        MAX_LINE_BYTES, 1_048_576,
        "FORMAT.md section 2 states 1 MiB"
    );
    let long = vec![b'x'; MAX_LINE_BYTES + 1];
    assert_eq!(verdict(&long), "too_long");
    // Exactly the limit is judged on content, not rejected for size.
    let at_limit = vec![b'x'; MAX_LINE_BYTES];
    assert_eq!(verdict(&at_limit), "not_json");
}

// ---- schema drift: the schema file and the code must agree ----

fn good_line() -> Value {
    let dir = spec_dir().join("vectors/good/mark_import_minimal.jsonl");
    serde_json::from_slice(&std::fs::read(dir).unwrap()).unwrap()
}

#[test]
fn schema_required_envelope_fields_are_required_by_the_code() {
    let schema = read_json(&spec_dir().join("schema/line.schema.json"));
    let required: Vec<&str> = schema["required"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(
        required,
        ["v", "id", "device", "seq", "ts", "type", "payload"]
    );
    for field in &required {
        let mut line = good_line();
        line.as_object_mut().unwrap().remove(*field);
        assert_eq!(
            verdict(line.to_string().as_bytes()),
            "missing_field",
            "removing {field}"
        );
    }
    // `author` is optional in both.
    assert!(!schema["required"].to_string().contains("author"));
    assert_eq!(verdict(good_line().to_string().as_bytes()), "valid");
}

#[test]
fn schema_version_and_kinds_match_the_code() {
    let schema = read_json(&spec_dir().join("schema/line.schema.json"));
    assert_eq!(
        schema["properties"]["v"]["const"].as_u64(),
        Some(u64::from(scribetrace_core::domain::FORMAT_VERSION))
    );
    let kinds = schema["$defs"]["mark"]["properties"]["kind"]["enum"]
        .as_array()
        .unwrap();
    assert_eq!(kinds.len(), 5);
    for kind in kinds {
        let mut line = good_line();
        line["payload"]["kind"] = kind.clone();
        assert_eq!(verdict(line.to_string().as_bytes()), "valid", "{kind}");
    }
    let mut line = good_line();
    line["payload"]["kind"] = Value::String("sticker".into());
    assert_eq!(verdict(line.to_string().as_bytes()), "bad_payload");
}

#[test]
fn schema_defined_types_are_the_ones_the_spec_text_lists() {
    let schema = read_json(&spec_dir().join("schema/line.schema.json"));
    let spec = std::fs::read_to_string(spec_dir().join("FORMAT.md")).unwrap();
    let defs = schema["$defs"].as_object().unwrap();
    for ty in [
        "purge",
        "milestone",
        "milestone_rename",
        "milestone_restore",
    ] {
        assert!(defs.contains_key(ty), "schema lacks {ty}");
    }
    for ty in [
        "mark_import",
        "purge",
        "milestone",
        "milestone_rename",
        "milestone_restore",
    ] {
        assert!(spec.contains(&format!("`{ty}`")), "FORMAT.md lacks {ty}");
    }
}
