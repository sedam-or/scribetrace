#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};

use scribetrace_core::adapters::ZoteroHttp;
use scribetrace_core::application::{SourceErrorCode, ZoteroSource};
use zotero_standin::{StandIn, StandInMode};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/fixtures/zotero")
}

#[test]
fn test_error_code_serialization_contract() {
    assert_eq!(
        serde_json::to_string(&SourceErrorCode::Refused).unwrap(),
        "\"refused\""
    );
    assert_eq!(
        serde_json::to_string(&SourceErrorCode::Forbidden).unwrap(),
        "\"forbidden\""
    );
    assert_eq!(
        serde_json::to_string(&SourceErrorCode::Timeout).unwrap(),
        "\"timeout\""
    );
    assert_eq!(
        serde_json::to_string(&SourceErrorCode::Version).unwrap(),
        "\"version\""
    );
    assert_eq!(
        serde_json::to_string(&SourceErrorCode::TooLarge).unwrap(),
        "\"too_large\""
    );
    assert_eq!(
        serde_json::to_string(&SourceErrorCode::BadAnswer).unwrap(),
        "\"bad_answer\""
    );
    assert_eq!(
        serde_json::to_string(&SourceErrorCode::Internal).unwrap(),
        "\"internal\""
    );

    assert_eq!(SourceErrorCode::Refused.as_str(), "refused");
    assert_eq!(SourceErrorCode::Forbidden.as_str(), "forbidden");
    assert_eq!(SourceErrorCode::Timeout.as_str(), "timeout");
    assert_eq!(SourceErrorCode::Version.as_str(), "version");
    assert_eq!(SourceErrorCode::TooLarge.as_str(), "too_large");
    assert_eq!(SourceErrorCode::BadAnswer.as_str(), "bad_answer");
    assert_eq!(SourceErrorCode::Internal.as_str(), "internal");
}

#[test]
fn test_refuse_off_allowlist_paths() {
    let server = StandIn::start(fixtures(), 0).unwrap();
    let client = ZoteroHttp::new(server.port());

    let off_list = [
        "/admin",
        "/api/users/0/collections",
        "/etc/passwd",
        "/api/users/0/items/../../secret",
        "/api/users/0/items/bad!key/children",
        "/api/users/0/items/H3E7BPJW/other",
    ];

    for path in off_list {
        let err = client.get(path).unwrap_err();
        assert_eq!(
            err.code(),
            SourceErrorCode::BadAnswer,
            "Path {path} must be refused as bad_answer"
        );
    }

    // Attachment key validation in fetch_marks
    assert_eq!(
        client.fetch_marks("invalid/key").unwrap_err().code(),
        SourceErrorCode::BadAnswer
    );
    assert_eq!(
        client.fetch_marks("").unwrap_err().code(),
        SourceErrorCode::BadAnswer
    );
    assert_eq!(
        client.fetch_marks("..").unwrap_err().code(),
        SourceErrorCode::BadAnswer
    );
}

#[test]
fn test_refuse_redirects() {
    let server = StandIn::start_mode(fixtures(), 0, StandInMode::Redirect).unwrap();
    let client = ZoteroHttp::new(server.port());

    let err = client.list_papers().unwrap_err();
    assert_eq!(
        err.code(),
        SourceErrorCode::BadAnswer,
        "Redirect must be refused and classified as bad_answer"
    );
    let msg = err.to_string();
    assert!(
        msg.contains("redirect refused"),
        "Error message must specify redirect refusal: {msg}"
    );
}

#[test]
fn test_refuse_timeout() {
    let server = StandIn::start_mode(fixtures(), 0, StandInMode::Timeout).unwrap();
    let client = ZoteroHttp::new(server.port());

    let err = client.list_papers().unwrap_err();
    assert_eq!(
        err.code(),
        SourceErrorCode::Timeout,
        "Timeout >5 s must be classified as SourceErrorCode::Timeout"
    );
}

#[test]
fn test_refuse_too_large() {
    let server = StandIn::start_mode(fixtures(), 0, StandInMode::TooLarge).unwrap();
    let client = ZoteroHttp::new(server.port());

    let err = client.list_papers().unwrap_err();
    assert_eq!(
        err.code(),
        SourceErrorCode::TooLarge,
        "Payload >15 MB must be classified as SourceErrorCode::TooLarge"
    );
}

#[test]
fn test_refuse_wrong_version() {
    let server = StandIn::start_mode(fixtures(), 0, StandInMode::WrongVersion).unwrap();
    let client = ZoteroHttp::new(server.port());

    let err = client.list_papers().unwrap_err();
    assert_eq!(
        err.code(),
        SourceErrorCode::Version,
        "Zotero-API-Version != 3 must be classified as SourceErrorCode::Version"
    );
}

#[test]
fn test_refuse_forbidden_api_disabled() {
    let server = StandIn::start_mode(fixtures(), 0, StandInMode::Forbidden).unwrap();
    let client = ZoteroHttp::new(server.port());

    let err = client.list_papers().unwrap_err();
    assert_eq!(
        err.code(),
        SourceErrorCode::Forbidden,
        "HTTP 403 Forbidden must be classified as SourceErrorCode::Forbidden"
    );
    assert!(
        err.to_string().contains("Local API is not enabled"),
        "Must preserve copy detail in display text: {}",
        err
    );
}

#[test]
fn test_refuse_connection_refused() {
    // Pick an unused local port
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let unused_port = listener.local_addr().unwrap().port();
    drop(listener);

    let client = ZoteroHttp::new(unused_port);
    let err = client.list_papers().unwrap_err();
    assert_eq!(
        err.code(),
        SourceErrorCode::Refused,
        "Unopened port must be classified as SourceErrorCode::Refused"
    );
}
