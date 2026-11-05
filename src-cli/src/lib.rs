//! The `scribetrace-cli` command line (ADR 0010): `check` and `export` over a project folder.
//!
//! It is built on the same core as the app, so both judge a history file the same way
//! (spec/FORMAT.md). It reads only: it never writes into the folder, never edits a document
//! and never opens a network connection. Text taken from history files is untrusted, so it is
//! stripped of control characters before it reaches a terminal and escaped before it becomes
//! Markdown.

use std::collections::{BTreeMap, HashSet};
use std::io::Write;
use std::path::Path;

use scribetrace_core::adapters::FsHistoryStore;
use scribetrace_core::application::HistoryStore;
use scribetrace_core::domain::{Event, EventBody, PaperId};

/// Everything is fine.
pub const EXIT_OK: u8 = 0;
/// `check` found bad lines (the files themselves are never changed).
pub const EXIT_BAD_LINES: u8 = 1;
/// Wrong usage, or the folder could not be read.
pub const EXIT_FAILED: u8 = 2;

const USAGE: &str = "\
scribetrace-cli: read-only checks and export for a ScribeTrace project folder

usage:
  scribetrace-cli check  <folder>             judge every history line; exit 1 if any is bad
  scribetrace-cli export <folder> [paper-id]  print the current marks as Markdown
  scribetrace-cli --help | --version

check prints one line per paper:
  files   history files read
  events  lines combined by id (purge lines included)
  visible events not hidden by a purge, purge lines excluded
  hidden  events hidden by a purge
  unknown lines of a type this version does not know (kept, not a problem)
  bad     lines skipped, with a count per reason code in [brackets]

The folder is never changed. Format: spec/FORMAT.md (unstable v0).
";

/// Runs one command line (without the program name). Returns the exit code.
pub fn run(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> u8 {
    match dispatch(args, out) {
        Ok(code) => code,
        Err(message) => {
            let _ = writeln!(err, "scribetrace-cli: {message}");
            let _ = writeln!(err, "try: scribetrace-cli --help");
            EXIT_FAILED
        }
    }
}

fn io_err(e: std::io::Error) -> String {
    e.to_string()
}

fn dispatch(args: &[String], out: &mut dyn Write) -> Result<u8, String> {
    let mut it = args.iter().map(String::as_str);
    match it.next() {
        None | Some("--help" | "-h" | "help") => {
            out.write_all(USAGE.as_bytes()).map_err(io_err)?;
            Ok(EXIT_OK)
        }
        Some("--version" | "-V") => {
            writeln!(out, "scribetrace-cli {}", env!("CARGO_PKG_VERSION")).map_err(io_err)?;
            Ok(EXIT_OK)
        }
        Some("check") => {
            let folder = it.next().ok_or("check needs a folder")?;
            if it.next().is_some() {
                return Err("check takes exactly one folder".into());
            }
            check(folder, out)
        }
        Some("export") => {
            let folder = it.next().ok_or("export needs a folder")?;
            let paper = it.next();
            if it.next().is_some() {
                return Err("export takes a folder and at most one paper id".into());
            }
            export(folder, paper, out)
        }
        Some(other) => Err(format!("unknown command {:?}", clean_line(other))),
    }
}

fn open_store(folder: &str) -> Result<FsHistoryStore, String> {
    let root = Path::new(folder);
    if !root.join(".scribetrace").join("papers").is_dir() {
        return Err(format!(
            "no .scribetrace/papers folder in {:?}",
            clean_line(folder)
        ));
    }
    Ok(FsHistoryStore::new(root))
}

fn check(folder: &str, out: &mut dyn Write) -> Result<u8, String> {
    let store = open_store(folder)?;
    let papers = store.list_papers().map_err(|e| e.to_string())?;
    let mut total_bad = 0usize;
    for paper in &papers {
        let report = store.read_report(paper).map_err(|e| e.to_string())?;
        let mut line = format!(
            "{paper}: files {} events {} visible {} hidden {} unknown {} bad {}",
            report.files_scanned,
            report.merged_events.len(),
            report.visible_events.len(),
            report.hidden_count,
            report.other_lines,
            report.skipped_lines,
            paper = paper.as_str(),
        );
        if !report.bad_lines.is_empty() {
            // BTreeMap: reason codes in a fixed order, so the output is stable.
            let by_code: BTreeMap<&String, &usize> = report.bad_lines.iter().collect();
            let parts: Vec<String> = by_code.iter().map(|(c, n)| format!("{c} {n}")).collect();
            line.push_str(&format!(" [{}]", parts.join(", ")));
        }
        writeln!(out, "{line}").map_err(io_err)?;
        total_bad += report.skipped_lines;
    }
    writeln!(
        out,
        "checked {} papers, {total_bad} bad lines",
        papers.len()
    )
    .map_err(io_err)?;
    Ok(if total_bad == 0 {
        EXIT_OK
    } else {
        EXIT_BAD_LINES
    })
}

fn export(folder: &str, paper: Option<&str>, out: &mut dyn Write) -> Result<u8, String> {
    let store = open_store(folder)?;
    let known = store.list_papers().map_err(|e| e.to_string())?;
    let papers: Vec<PaperId> = match paper {
        Some(id) => {
            let id = PaperId::new(id).map_err(|e| e.to_string())?;
            if !known.contains(&id) {
                return Err(format!("no such paper {:?}", id.as_str()));
            }
            vec![id]
        }
        None => known,
    };
    for (n, paper) in papers.iter().enumerate() {
        let report = store.read_report(paper).map_err(|e| e.to_string())?;
        if n > 0 {
            writeln!(out).map_err(io_err)?;
        }
        out.write_all(render_paper(paper, &report.visible_events).as_bytes())
            .map_err(io_err)?;
    }
    Ok(EXIT_OK)
}

/// One paper as Markdown. A mark seen on several devices appears once (the first in display
/// order); everything that came from a file is escaped.
fn render_paper(paper: &PaperId, visible: &[Event]) -> String {
    let mut seen: HashSet<&str> = HashSet::new();
    let mut body = String::new();
    let mut count = 0usize;
    for ev in visible {
        let EventBody::MarkImport(mark) = &ev.body else {
            continue;
        };
        if !seen.insert(mark.key.as_str()) {
            continue;
        }
        count += 1;
        let place = match mark.page_index {
            Some(i) => format!(", page {}", u64::from(i) + 1),
            None => String::new(),
        };
        body.push_str(&format!("## {}{place}\n\n", mark.kind));
        if let Some(text) = &mark.text {
            body.push_str(&quote(text));
            body.push('\n');
        }
        if let Some(comment) = &mark.comment {
            body.push_str(&format!("Comment: {}\n\n", paragraph(comment)));
        }
        body.push_str(&format!(
            "_{} · {} · {}_\n\n",
            md_escape(&clean_line(&mark.key)),
            md_escape(&clean_line(&ev.ts)),
            md_escape(&clean_line(&ev.author)),
        ));
    }
    let heading = format!("# Paper {}\n\n", paper.as_str());
    if count == 0 {
        format!("{heading}No marks.\n")
    } else {
        format!("{heading}{body}").trim_end().to_owned() + "\n"
    }
}

/// Drops what could drive a terminal or spoof the reading order: control characters (a tab
/// becomes a space, `\n` is kept) and the Unicode bidirectional overrides.
fn clean(text: &str) -> String {
    text.chars()
        .filter_map(|c| match c {
            '\n' => Some('\n'),
            '\t' => Some(' '),
            c if c.is_control() => None,
            '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' | '\u{200E}' | '\u{200F}' => None,
            c => Some(c),
        })
        .collect()
}

/// Like [`clean`], on one line.
fn clean_line(text: &str) -> String {
    clean(text).replace('\n', " ")
}

/// Backslash-escapes what Markdown would read as markup, so text stays text.
fn md_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if matches!(
            c,
            '\\' | '`' | '*' | '_' | '[' | ']' | '<' | '>' | '&' | '#' | '|' | '~' | '!'
        ) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// A block quote; every line is escaped on its own.
fn quote(text: &str) -> String {
    let mut out = String::new();
    for line in clean(text).lines() {
        if line.trim().is_empty() {
            out.push_str(">\n");
        } else {
            out.push_str(&format!("> {}\n", md_escape(line)));
        }
    }
    out
}

/// A paragraph; line breaks become Markdown hard breaks.
fn paragraph(text: &str) -> String {
    let lines: Vec<String> = clean(text).lines().map(md_escape).collect();
    lines.join("  \n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_removes_terminal_escapes_and_bidi_overrides() {
        assert_eq!(clean("a\u{1b}[31mb\u{202e}c\td\ne"), "a[31mbc d\ne");
        assert_eq!(clean_line("x\ny"), "x y");
    }

    #[test]
    fn md_escape_neutralises_markup() {
        assert_eq!(
            md_escape("<b>*x*</b> & [l](u)"),
            r"\<b\>\*x\*\</b\> \& \[l\](u)"
        );
    }
}
