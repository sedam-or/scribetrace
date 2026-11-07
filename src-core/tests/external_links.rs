#![allow(clippy::unwrap_used, clippy::expect_used)]

use scribetrace_core::adapters::parse_papers_from_items;
use scribetrace_core::application::{
    build_external_uri, external_link_availability, is_allowed_external_uri, ExternalLinkError,
    ExternalTarget, LinkAvailabilityCode, PaperLinkAvailability,
};
use scribetrace_core::domain::{PaperId, PaperMetadataPayload};
use serde_json::json;

fn paper() -> PaperId {
    PaperId::from_zotero("PAPER001").unwrap()
}

fn metadata(doi: Option<&str>, pdf_key: Option<&str>) -> PaperMetadataPayload {
    PaperMetadataPayload {
        doi: doi.map(str::to_owned),
        pdf_key: pdf_key.map(str::to_owned),
    }
}

#[test]
fn doi_uri_uses_the_fixed_https_host_and_encodes_each_path_segment() {
    let result = build_external_uri(
        ExternalTarget::Doi,
        &paper(),
        &metadata(Some("10.12345/alpha:beta/(notes);x"), Some("PDFKEY01")),
    )
    .unwrap();

    assert_eq!(
        result,
        "https://doi.org/10.12345/alpha%3Abeta/%28notes%29%3Bx"
    );
    assert!(is_allowed_external_uri(&result));
}

#[test]
fn pdf_uri_uses_the_attachment_key_and_item_uri_uses_the_parent_key() {
    let paper = paper();
    let source = metadata(Some("10.12345/example"), Some("PDFKEY01"));

    assert_eq!(
        build_external_uri(ExternalTarget::Pdf, &paper, &source).unwrap(),
        "zotero://open-pdf/library/items/PDFKEY01"
    );
    assert_eq!(
        build_external_uri(ExternalTarget::Item, &paper, &source).unwrap(),
        "zotero://select/library/items/PAPER001"
    );
}

#[test]
fn unavailable_link_targets_return_backend_codes() {
    let paper = paper();
    let no_metadata = metadata(None, None);

    assert_eq!(
        build_external_uri(ExternalTarget::Doi, &paper, &no_metadata),
        Err(ExternalLinkError::NoDoi)
    );
    assert_eq!(
        build_external_uri(ExternalTarget::Pdf, &paper, &no_metadata),
        Err(ExternalLinkError::NoPdf)
    );

    let local_paper = PaperId::new("h_0123456789abcdef").unwrap();
    assert_eq!(
        build_external_uri(ExternalTarget::Item, &local_paper, &no_metadata),
        Err(ExternalLinkError::NoZoteroKey)
    );

    assert_eq!(
        external_link_availability(&local_paper, &no_metadata),
        PaperLinkAvailability {
            doi: LinkAvailabilityCode::NoDoi,
            pdf: LinkAvailabilityCode::NoZoteroKey,
            item: LinkAvailabilityCode::NoZoteroKey,
        }
    );
}

#[test]
fn non_zotero_papers_have_no_zotero_link_targets() {
    let local_paper = PaperId::new("h_0123456789abcdef").unwrap();
    let metadata = metadata(Some("10.12345/example"), Some("PDFKEY01"));

    assert_eq!(
        external_link_availability(&local_paper, &metadata),
        PaperLinkAvailability {
            doi: LinkAvailabilityCode::Available,
            pdf: LinkAvailabilityCode::NoZoteroKey,
            item: LinkAvailabilityCode::NoZoteroKey,
        }
    );
}

#[test]
fn malformed_dois_and_attachment_keys_are_rejected() {
    let paper = paper();

    for doi in [
        "http://evil.test/x",
        "10.1234/x?host=evil",
        "10.1234/../evil",
    ] {
        assert_eq!(
            build_external_uri(ExternalTarget::Doi, &paper, &metadata(Some(doi), None)),
            Err(ExternalLinkError::InvalidMetadata),
            "invalid DOI must not become a URI: {doi}"
        );
    }

    assert_eq!(
        build_external_uri(
            ExternalTarget::Pdf,
            &paper,
            &metadata(None, Some("../../evil"))
        ),
        Err(ExternalLinkError::InvalidMetadata)
    );
    assert_eq!(
        build_external_uri(
            ExternalTarget::Item,
            &PaperId::new("z_bad").unwrap(),
            &metadata(None, None)
        ),
        Err(ExternalLinkError::NoZoteroKey)
    );
}

#[test]
fn final_uri_allowlist_rejects_unapproved_schemes_hosts_and_path_controls() {
    for uri in [
        "file:///etc/passwd",
        "javascript:alert(1)",
        "http://evil.test/10.1234/x",
        "https://evil.test/10.1234/x",
        "https://doi.org/10.1234/x?next=evil",
        "https://doi.org/10.1234/x#fragment",
        "https://doi.org/10.1234/../evil",
        "https://doi.org/10.1234/%2E%2e/evil",
        "https://doi.org/10.1234/a%2Fb",
        "https://doi.org/10.1234/x%3Fhost=evil",
        "zotero://open-pdf/library/items/PDFKEY01?query=bad",
        "zotero://select/library/items/PAPER001#fragment",
    ] {
        assert!(!is_allowed_external_uri(uri), "must refuse {uri}");
    }
}

#[test]
fn availability_uses_codes_instead_of_null_metadata_fields() {
    assert_eq!(
        external_link_availability(&paper(), &metadata(Some("10.12345/x"), Some("PDFKEY01"))),
        PaperLinkAvailability {
            doi: LinkAvailabilityCode::Available,
            pdf: LinkAvailabilityCode::Available,
            item: LinkAvailabilityCode::Available,
        }
    );
}

#[test]
fn paper_summary_exposes_display_and_availability_but_not_link_keys() {
    let items = vec![json!({
        "key": "PAPER001",
        "data": {
            "key": "PAPER001",
            "itemType": "journalArticle",
            "title": "Paper",
            "DOI": " 10.12345/example "
        },
        "links": {
            "attachment": { "href": "http://127.0.0.1/api/users/0/items/PDFKEY01" }
        }
    })];
    let papers = parse_papers_from_items(&items).unwrap();
    let summary = serde_json::to_value(&papers[0]).unwrap();

    assert_eq!(summary["doiDisplay"], "10.12345/example");
    assert_eq!(summary["linkAvailability"]["doi"], "available");
    assert_eq!(summary["linkAvailability"]["pdf"], "available");
    assert_eq!(summary["linkAvailability"]["item"], "available");
    assert!(summary.get("attachmentKey").is_none());
    assert!(summary.get("attachment_key").is_none());
    assert!(summary.get("pdf_key").is_none());
}
