//! The one validator for a history line (see `spec/FORMAT.md`). Pure: bytes in, verdict out.
//! The app, the command-line tool and the conformance vectors all call [`classify_line`].

use serde_json::{Map, Value};

use super::{
    DeviceId, Event, EventBody, MilestonePayload, MilestoneRenamePayload, PaperMetadataPayload,
    FORMAT_VERSION, MAX_LABEL_CHARS, MAX_LINE_BYTES,
};

/// Why a line was refused. [`BadLine::code`] is the stable word the spec and the vectors use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadLine {
    /// Longer than [`MAX_LINE_BYTES`](super::MAX_LINE_BYTES) (1 MiB, SEC-12).
    TooLong,
    /// Not valid UTF-8 JSON.
    NotJson,
    /// Valid JSON, but not an object.
    NotObject,
    /// A required envelope field is absent.
    MissingField(&'static str),
    /// An envelope field has the wrong shape.
    BadValue(&'static str),
    /// `v` names a format this reader does not know.
    UnsupportedVersion,
    /// The `payload` of a known type does not fit that type.
    BadPayload,
}

impl BadLine {
    /// Stable reason code for the spec, the vectors and `check` output.
    pub fn code(self) -> &'static str {
        match self {
            Self::TooLong => "too_long",
            Self::NotJson => "not_json",
            Self::NotObject => "not_object",
            Self::MissingField(_) => "missing_field",
            Self::BadValue(_) => "bad_value",
            Self::UnsupportedVersion => "unsupported_version",
            Self::BadPayload => "bad_payload",
        }
    }
}

/// The verdict on one line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineClass {
    /// A well-formed line of a type this build understands.
    Valid(Box<Event>),
    /// A well-formed envelope whose `type` this build does not type yet (or does not know).
    /// Readers keep such lines and never count them as bad.
    Other {
        /// The `type` word.
        event_type: String,
    },
    /// A line to skip and count.
    Bad(BadLine),
}

/// Judges one line (without its newline).
pub fn classify_line(line: &[u8]) -> LineClass {
    if line.len() > MAX_LINE_BYTES {
        return LineClass::Bad(BadLine::TooLong);
    }
    let Ok(value) = serde_json::from_slice::<Value>(line) else {
        return LineClass::Bad(BadLine::NotJson);
    };
    let Value::Object(mut obj) = value else {
        return LineClass::Bad(BadLine::NotObject);
    };
    match check_envelope(&mut obj) {
        Ok(event_type) => typed(obj, event_type),
        Err(bad) => LineClass::Bad(bad),
    }
}

fn check_envelope(obj: &mut Map<String, Value>) -> Result<String, BadLine> {
    match obj.get("v") {
        None => return Err(BadLine::MissingField("v")),
        Some(v) if v.as_u64() == Some(u64::from(FORMAT_VERSION)) => {}
        Some(v) if v.as_u64().is_some() => return Err(BadLine::UnsupportedVersion),
        Some(_) => return Err(BadLine::BadValue("v")),
    }
    let id = text(obj, "id")?;
    if !is_uuid(id) {
        return Err(BadLine::BadValue("id"));
    }
    if DeviceId::new(text(obj, "device")?).is_err() {
        return Err(BadLine::BadValue("device"));
    }
    match obj.get("seq") {
        None => return Err(BadLine::MissingField("seq")),
        Some(v) if v.as_u64().is_some_and(|n| n >= 1) => {}
        Some(_) => return Err(BadLine::BadValue("seq")),
    }
    if !is_timestamp(text(obj, "ts")?) {
        return Err(BadLine::BadValue("ts"));
    }
    match obj.get("author") {
        None => {
            obj.insert("author".into(), Value::String("Me".into()));
        }
        Some(Value::String(a)) if !a.is_empty() && a.len() <= 200 => {}
        Some(_) => return Err(BadLine::BadValue("author")),
    }
    let event_type = text(obj, "type")?;
    if !is_type_word(event_type) {
        return Err(BadLine::BadValue("type"));
    }
    let event_type = event_type.to_owned();
    match obj.get("payload") {
        None => Err(BadLine::MissingField("payload")),
        Some(Value::Object(_)) => Ok(event_type),
        Some(_) => Err(BadLine::BadValue("payload")),
    }
}

fn typed(obj: Map<String, Value>, event_type: String) -> LineClass {
    // Only the types with a typed payload are checked here; the others join as their slices land.
    if !matches!(
        event_type.as_str(),
        "mark_import" | "purge" | "milestone" | "milestone_rename" | "paper_metadata"
    ) {
        return LineClass::Other { event_type };
    }
    // `label` is required on milestone and milestone_rename (FORMAT §3); serde would read an
    // omitted key as `None`, so presence is checked here. An explicit `null` is still allowed.
    // The same holds for both keys of paper_metadata.
    let required_keys: &[&str] = match event_type.as_str() {
        "milestone" | "milestone_rename" => &["label"],
        "paper_metadata" => &["doi", "pdf_key"],
        _ => &[],
    };
    if !required_keys.iter().all(|key| has_key(&obj, key)) {
        return LineClass::Bad(BadLine::BadPayload);
    }
    match serde_json::from_value::<Event>(Value::Object(obj)) {
        Ok(event) => match &event.body {
            EventBody::MarkImport(mark) if mark.validate().is_err() => {
                LineClass::Bad(BadLine::BadPayload)
            }
            EventBody::Milestone(p) if !milestone_payload_ok(p) => {
                LineClass::Bad(BadLine::BadPayload)
            }
            EventBody::MilestoneRename(r) if !rename_payload_ok(r) => {
                LineClass::Bad(BadLine::BadPayload)
            }
            EventBody::PaperMetadata(p) if !paper_metadata_ok(p) => {
                LineClass::Bad(BadLine::BadPayload)
            }
            _ => LineClass::Valid(Box::new(event)),
        },
        Err(_) => LineClass::Bad(BadLine::BadPayload),
    }
}

/// `spec/FORMAT.md` §3: a DOI matches `^10\.[0-9]{4,9}/[-._;()/:A-Za-z0-9]+$`; a PDF attachment
/// key matches `^[A-Z0-9]{8}$`. `None` is always allowed.
fn paper_metadata_ok(p: &PaperMetadataPayload) -> bool {
    doi_ok(p.doi.as_deref()) && pdf_key_ok(p.pdf_key.as_deref())
}

fn doi_ok(doi: Option<&str>) -> bool {
    let Some(doi) = doi else { return true };
    let Some(rest) = doi.strip_prefix("10.") else {
        return false;
    };
    let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
    if !(4..=9).contains(&digits) {
        return false;
    }
    let Some(suffix) = rest[digits..].strip_prefix('/') else {
        return false;
    };
    !suffix.is_empty()
        && suffix
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-._;()/:".contains(&b))
}

fn pdf_key_ok(key: Option<&str>) -> bool {
    key.is_none_or(|k| {
        k.len() == 8
            && k.bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
    })
}

/// `spec/FORMAT.md` §3 and §7: every `upto` entry is at least 1; a label is at most
/// [`MAX_LABEL_CHARS`](super::MAX_LABEL_CHARS) characters.
fn milestone_payload_ok(p: &MilestonePayload) -> bool {
    p.upto.values().all(|&seq| seq >= 1) && label_ok(p.label.as_deref())
}

/// `spec/FORMAT.md` §3: the renamed milestone is named by its UUID `id`.
fn rename_payload_ok(r: &MilestoneRenamePayload) -> bool {
    is_uuid(&r.milestone) && label_ok(r.label.as_deref())
}

/// True when the payload has the key (its value may be `null`).
fn has_key(obj: &Map<String, Value>, key: &str) -> bool {
    obj.get("payload")
        .and_then(Value::as_object)
        .is_some_and(|p| p.contains_key(key))
}

fn label_ok(label: Option<&str>) -> bool {
    label.is_none_or(|l| l.chars().count() <= MAX_LABEL_CHARS)
}

fn text<'a>(obj: &'a Map<String, Value>, key: &'static str) -> Result<&'a str, BadLine> {
    match obj.get(key) {
        None => Err(BadLine::MissingField(key)),
        Some(Value::String(s)) => Ok(s),
        Some(_) => Err(BadLine::BadValue(key)),
    }
}

/// 8-4-4-4-12 hexadecimal digits with hyphens, any case.
fn is_uuid(s: &str) -> bool {
    s.len() == 36
        && s.bytes().enumerate().all(|(i, b)| match i {
            8 | 13 | 18 | 23 => b == b'-',
            _ => b.is_ascii_hexdigit(),
        })
}

/// `YYYY-MM-DDTHH:MM:SSZ`, shape only: the time is for display and never decides anything.
fn is_timestamp(s: &str) -> bool {
    const SHAPE: &[u8; 20] = b"dddd-dd-ddTdd:dd:ddZ";
    s.len() == SHAPE.len()
        && s.bytes().zip(SHAPE).all(|(b, &p)| match p {
            b'd' => b.is_ascii_digit(),
            _ => b == p,
        })
}

/// `[a-z][a-z0-9_]{0,63}`.
fn is_type_word(s: &str) -> bool {
    let mut bytes = s.bytes();
    bytes.next().is_some_and(|b| b.is_ascii_lowercase())
        && s.len() <= 64
        && bytes.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD: &str = r##"{"v":1,"id":"6f1f2c1e-0a4b-4c55-9d3e-2b7a9a0c1d11","device":"dev-a","seq":1,"ts":"2026-11-06T09:00:00Z","author":"Me","type":"mark_import","payload":{"key":"AB12CD34","kind":"highlight","text":"x","comment":null,"color":"#ffd400","page_index":2,"added":null,"modified":null}}"##;

    fn class(s: &str) -> LineClass {
        classify_line(s.as_bytes())
    }

    #[test]
    fn good_line_is_valid() {
        assert!(matches!(class(GOOD), LineClass::Valid(_)));
    }

    #[test]
    fn author_defaults_to_me() {
        let line = GOOD.replace(r#""author":"Me","#, "");
        let LineClass::Valid(e) = class(&line) else {
            unreachable!("line should be valid");
        };
        assert_eq!(e.author, "Me");
    }

    #[test]
    fn unknown_type_is_kept_not_bad() {
        let line = GOOD.replace("mark_import", "future_thing");
        assert_eq!(
            class(&line),
            LineClass::Other {
                event_type: "future_thing".into()
            }
        );
    }

    #[test]
    fn unknown_fields_are_ignored() {
        let line = GOOD.replace(r#""v":1,"#, r#""v":1,"extra":[1,2],"#);
        assert!(matches!(class(&line), LineClass::Valid(_)));
    }

    #[test]
    fn each_refusal_has_its_code() {
        let cases: &[(String, &str)] = &[
            ("not json".into(), "not_json"),
            ("[1]".into(), "not_object"),
            (GOOD.replace(r#""seq":1,"#, ""), "missing_field"),
            (GOOD.replace(r#""seq":1"#, r#""seq":0"#), "bad_value"),
            (GOOD.replace(r#""v":1"#, r#""v":2"#), "unsupported_version"),
            (
                GOOD.replace(r#""kind":"highlight""#, r#""kind":"x""#),
                "bad_payload",
            ),
            (GOOD.replace(r#""key":"AB12CD34","#, ""), "bad_payload"),
            ("x".repeat(MAX_LINE_BYTES + 1), "too_long"),
        ];
        for (line, code) in cases {
            let LineClass::Bad(b) = classify_line(line.as_bytes()) else {
                unreachable!("line should be bad: {line}");
            };
            assert_eq!(b.code(), *code, "{line}");
        }
    }

    #[test]
    fn invalid_utf8_is_not_json() {
        assert_eq!(
            classify_line(&[0xff, 0xfe]),
            LineClass::Bad(BadLine::NotJson)
        );
    }

    #[test]
    fn shapes() {
        assert!(is_uuid("6F1F2C1E-0A4B-4C55-9D3E-2B7A9A0C1D11"));
        assert!(!is_uuid("6f1f2c1e0a4b4c559d3e2b7a9a0c1d11"));
        assert!(is_timestamp("2026-11-06T09:00:00Z"));
        assert!(!is_timestamp("2026-11-06 09:00:00"));
        assert!(is_type_word("mark_import"));
        assert!(!is_type_word("Mark"));
        assert!(!is_type_word(""));
    }
}
