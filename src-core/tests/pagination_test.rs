#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Slice 4 review item 1: the broker pages through the local API (`limit=100&start=N`)
//! instead of trusting a single page, so large libraries are never silently truncated (R-01/R-03).

use std::fs;
use std::path::Path;

use scribetrace_core::adapters::ZoteroHttp;
use scribetrace_core::application::{SourceErrorCode, ZoteroSource};
use serde_json::{json, Value};
use zotero_standin::{StandIn, StandInMode};

fn write_json(dir: &Path, rel: &str, value: &Value) {
    let path = dir.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, serde_json::to_string(value).unwrap()).unwrap();
}

fn paper(i: usize) -> Value {
    let key = format!("P{i:07}");
    json!({
        "key": key,
        "data": { "key": key, "itemType": "journalArticle", "title": format!("Paper {i}") }
    })
}

fn annotation(i: usize, parent: &str) -> Value {
    let key = format!("A{i:07}");
    json!({
        "key": key,
        "data": {
            "key": key,
            "itemType": "annotation",
            "parentItem": parent,
            "annotationType": "highlight",
            "annotationText": format!("mark {i}"),
            "annotationPosition": format!("{{\"pageIndex\":{},\"rects\":[]}}", i % 7),
            "dateAdded": "2026-11-06T00:00:00Z",
            "dateModified": "2026-11-06T00:00:00Z"
        }
    })
}

#[test]
fn list_papers_pages_past_100_items_in_stable_order_without_duplicates() {
    let dir = tempfile::tempdir().unwrap();
    // 250 papers + 1 attachment + 130 annotations on it = 381 items, i.e. four pages of 100.
    let mut items: Vec<Value> = (0..250).map(paper).collect();
    items.push(json!({
        "key": "ATTBIG01",
        "data": { "key": "ATTBIG01", "itemType": "attachment", "parentItem": "P0000007" }
    }));
    items.extend((0..130).map(|i| annotation(i, "ATTBIG01")));
    write_json(dir.path(), "api/users/0/items.json", &Value::Array(items));

    let server = StandIn::start(dir.path(), 0).unwrap();
    let papers = ZoteroHttp::new(server.port()).list_papers().unwrap();

    assert_eq!(papers.len(), 250, "every paper across all pages is listed");
    let ids: Vec<String> = papers.iter().map(|p| p.id.clone()).collect();
    let expected: Vec<String> = (0..250).map(|i| format!("z_P{i:07}")).collect();
    assert_eq!(ids, expected, "stable source order, no duplicates");

    let p7 = papers.iter().find(|p| p.id == "z_P0000007").unwrap();
    assert_eq!(p7.attachment_key.as_deref(), Some("ATTBIG01"));
    assert_eq!(
        p7.marks_count, 130,
        "annotations on later pages are counted"
    );
}

#[test]
fn fetch_marks_pages_past_100_annotations_on_one_attachment() {
    let dir = tempfile::tempdir().unwrap();
    let anns: Vec<Value> = (0..257).map(|i| annotation(i, "ATTBIG01")).collect();
    write_json(
        dir.path(),
        "api/users/0/items/ATTBIG01/children.json",
        &Value::Array(anns),
    );

    let server = StandIn::start(dir.path(), 0).unwrap();
    let marks = ZoteroHttp::new(server.port())
        .fetch_marks("ATTBIG01")
        .unwrap();

    assert_eq!(marks.len(), 257, "no mark beyond the first page is lost");
    let keys: Vec<String> = marks.iter().map(|m| m.key.clone()).collect();
    let expected: Vec<String> = (0..257).map(|i| format!("A{i:07}")).collect();
    assert_eq!(keys, expected, "stable order, no duplicates");
    assert_eq!(marks[256].page_index, Some(256 % 7));
}

#[test]
fn exactly_100_items_ends_after_an_empty_second_page() {
    let dir = tempfile::tempdir().unwrap();
    let items: Vec<Value> = (0..100).map(paper).collect();
    write_json(dir.path(), "api/users/0/items.json", &Value::Array(items));

    let server = StandIn::start(dir.path(), 0).unwrap();
    let papers = ZoteroHttp::new(server.port()).list_papers().unwrap();
    assert_eq!(papers.len(), 100);
}

#[test]
fn body_over_limit_without_content_length_is_too_large() {
    // No Content-Length header: the typed ureq BodyExceedsLimit path must still give too_large.
    let server = StandIn::start_mode(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/fixtures/zotero"),
        0,
        StandInMode::TooLargeChunked,
    )
    .unwrap();
    let err = ZoteroHttp::new(server.port()).list_papers().unwrap_err();
    assert_eq!(err.code(), SourceErrorCode::TooLarge);
}

#[test]
fn endless_pages_hit_the_page_cap_as_too_large_not_truncation() {
    // A server that ignores `start` returns full pages forever: the broker must stop at its
    // page cap with too_large rather than looping or returning a truncated list.
    let dir = tempfile::tempdir().unwrap();
    let items: Vec<Value> = (0..100).map(paper).collect();
    write_json(dir.path(), "api/users/0/items.json", &Value::Array(items));

    let server = StandIn::start_mode(dir.path(), 0, StandInMode::IgnoreStart).unwrap();
    let err = ZoteroHttp::new(server.port()).list_papers().unwrap_err();
    assert_eq!(err.code(), SourceErrorCode::TooLarge);
}
