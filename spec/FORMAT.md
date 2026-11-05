# ScribeTrace history format, **unstable v0**

Status: **unstable v0**. Nothing here is promised stable until an outside tool uses it and a maintainer of that tool says so ([ADR 0010](../docs/adr/0010-open-format-spec-and-cli.md)). Reasoning behind the rules: [ADR 0007](../docs/adr/0007-per-device-history-log-merge-and-forget.md). The words MUST and MAY are meant literally.

Everything is plain text. You can read, diff and check a history with a text editor, `git` and the `scribetrace` command-line tool, which uses the same validator as the app.

## 1. Layout

```
<project folder>/
  .scribetrace/
    papers/
      <paper-id>/
        <device-id>.jsonl     one writer, append-only, never rewritten
        note.md               the reader's own note (not part of this spec)
```

- `paper-id`: `z_<Zotero item key>` or `h_<content hash>`.
- `device-id`: a random id, never taken from the machine or user name.
- Both match `[A-Za-z0-9_-]{1,128}`. A name that does not match is not read (no path tricks).
- A device writes only its own file. Two machines never write the same file, so sync tools see no conflicts.
- Files are UTF-8. One line is one JSON object, ended by `\n`. Writers MUST NOT rewrite or truncate a line that is already there. If the last line of its own file has no `\n` (a crash), a writer closes it with `\n` before the next append.

## 2. The line

| Field | Type | Rule |
|---|---|---|
| `v` | integer | `1`. Any other integer: the line is bad (`unsupported_version`). |
| `id` | string | UUID text, `8-4-4-4-12` hexadecimal digits, any case. Unique across all devices. |
| `device` | string | `device-id`; MUST equal the file name without `.jsonl`. |
| `seq` | integer | 1, 2, 3 … per device. Never 0. |
| `ts` | string | `YYYY-MM-DDTHH:MM:SSZ` (UTC). **For display only.** Nothing is decided by it. |
| `author` | string | 1 to 200 bytes. When the field is absent: `"Me"`. |
| `type` | string | `[a-z][a-z0-9_]{0,63}`, see section 3. |
| `payload` | object | Depends on `type`. |

A line is at most **1,048,576 bytes** (1 MiB) without its `\n`. Readers MUST ignore fields they do not know and SHOULD keep them when they copy a line. Newer apps read older files without migration, so there is no migration code.

All text in a line (mark text, comments, labels, author) is **untrusted input**. A reader MUST show it as plain text, never as markup or code.

## 3. Types

Defined (the payload is fixed here):

| `type` | Payload | Meaning |
|---|---|---|
| `mark_import` | a **mark** (below) | A mark seen for the first time in this paper's history. |
| `purge` | `{ "covers": { "<device-id>": <seq>, … } }` | Forget: hides events (section 6). |
| `milestone` | `{ "label": string or null, "reason": "import" or "refresh" or "session" or "manual" or "before_restore", "upto": { "<device-id>": <seq>, … } }` | A named point in the history (section 7). |
| `milestone_rename` | `{ "milestone": "<id>", "label": string or null }` | Gives or changes a milestone's name. |
| `milestone_restore` | `{ "milestone": "<id>" }` | Makes the current view equal to that milestone's (section 7). |

A **mark**: `key` (string, 1 to 128 bytes, the reference manager's key), `kind` (`highlight`, `note`, `underline`, `image` or `ink`), and optional `text`, `comment` (strings or null), `color` (`#rrggbb` or null), `page_index` (zero-based integer or null), `added`, `modified` (strings or null, as the source gave them).

Names reserved, payload not fixed yet in v0: `mark_update`, `mark_gone`, `note_revision`, `mark_create`, `mark_delete`, `mark_restore`.

A line with a well-formed envelope and any other `type` is **not bad**: a reader keeps it, does not show it as a problem, and does not interpret it.

## 4. Reading

1. Split each file at `\n`. Judge each line alone (section 5).
2. **Union by `id`.** The set of all good lines of all devices is the history. Reading it twice, or in any order, gives the same set. If two lines share an `id` with different content, the one whose canonical JSON text sorts higher wins; this keeps the result the same on every machine.
3. **Display order:** `(ts, device, seq, id)`. The order is for people. No rule below depends on it, except the choice between two names of one milestone (section 7).

## 5. Bad lines

A line is **bad** and is skipped, never repaired, if it: is longer than 1 MiB (`too_long`); is not UTF-8 JSON (`not_json`); is not a JSON object (`not_object`); lacks a required field (`missing_field`); has a field of the wrong shape (`bad_value`); has an unknown `v` (`unsupported_version`); or has a `payload` that does not fit its defined `type` (`bad_payload`). A reader counts bad lines per paper and shows the count. Files are never changed because of them.

## 6. Forget (`purge`)

An event is **hidden** if some purge `P` has `P.covers[e.device]` and `e.seq ≤ P.covers[e.device]`. Hiding uses `seq` only, so a wrong clock cannot hide or show anything. Several purges combine by taking, for each device, the largest `covers` value. Events a purger had not seen (written offline) are not covered and stay visible. `purge` lines are never hidden: they are the "erased" marker. A purge does not reserve the `paper-id`; new events start a fresh history.

The format cannot delete copies that sync services or git already hold. Apps must say so.

## 7. Milestones and restore

- `milestone.upto` lists, for each device, the highest `seq` that belongs to the state at that point. The **state at a milestone** is the visible events with `seq ≤ upto[device]`; a device that is not listed contributes none. This is a cut through the set, so it needs no clock.
- A name is the `label` of the milestone, replaced by the latest `milestone_rename` for its `id`, latest by display order. Renaming adds a line; no line is edited.
- `milestone_restore` makes the **current view** equal to the state at the named milestone, plus every event that comes after the restore line in display order. It adds one line and changes none. Before it, an app writes a milestone with `reason: "before_restore"` when there are changes since the last milestone, so restoring that one undoes the restore.
- A restore or rename that names a milestone that is missing or hidden has no effect.

## 8. Conformance

`spec/schema/line.schema.json` is the JSON Schema of a line (shape only; bounds on `id`, `ts` and the 1 MiB limit are in the text above). `spec/vectors/` holds sample lines and `manifest.json` says what every reader MUST decide for each one: `valid` (understood), `other` (kept, not interpreted) or the reason code of a bad line. Until slice 6 lands, the three `milestone*` samples are `other`. The reference implementation checks the vectors in CI (`src-core/tests/spec_vectors.rs`) and a second test fails if the schema and the code drift apart. A line over 1 MiB is generated by the test, not stored.
