# ScribeTrace history format, **unstable v0**

Status: **unstable v0**. Nothing here is promised stable until an outside tool uses it and a maintainer of that tool says so ([ADR 0010](../docs/adr/0010-open-format-spec-and-cli.md)). Reasoning behind the rules: [ADR 0007](../docs/adr/0007-per-device-history-log-merge-and-forget.md). The words MUST, MUST NOT, SHOULD and MAY are meant literally: MUST and MUST NOT are requirements, SHOULD is a strong recommendation, MAY is optional. In this document a **reader** is a program that reads these files (the app, the `scribetrace` tool, or yours), and a **writer** is a program that adds lines to them.

Everything is plain text. You can read, diff and check a history with a text editor, `git` and the `scribetrace-cli` command-line tool, which uses the same checker as the app.

## 1. Layout

```
<project folder>/
  .scribetrace/
    papers/
      <paper-id>/
        <device-id>.jsonl     one writer, append-only, never rewritten
        note.md               your own notes on the paper (not part of this spec)
```

- `paper-id`: `z_<Zotero item key>` or `h_<content hash>`.
- `device-id`: a random id, never taken from the computer's name or the user's name.
- Both match `[A-Za-z0-9_-]{1,128}`. A name that does not match is not read, so a name can never point outside the folder.
- A device writes only its own file. Two computers never write the same file, so sync tools see no conflicts.
- Files are UTF-8. One line is one JSON object, ended by `\n`. Writers MUST NOT rewrite or cut short a line that is already there. If the last line of a writer's own file has no `\n` (for example after a crash), the writer ends it with `\n` before adding the next line.

## 2. The line

| Field | Type | Rule |
|---|---|---|
| `v` | integer | `1`. Any other integer: the line is bad (`unsupported_version`). |
| `id` | string | UUID text, `8-4-4-4-12` hexadecimal digits, any case. Unique across all devices. |
| `device` | string | `device-id`; MUST equal the file name without `.jsonl`. |
| `seq` | integer | 1, 2, 3 … per device. Never 0. A writer takes the next number after the highest `seq` in its own file and never reuses or restarts one, not even after a `purge`. If its file is lost, the device starts over under a new `device-id`. |
| `ts` | string | `YYYY-MM-DDTHH:MM:SSZ` (UTC). **For display only.** Nothing is decided by it, except the two choices between simultaneous changes named in section 4. |
| `author` | string | 1 to 200 bytes. When the field is absent: `"Me"`. |
| `type` | string | `[a-z][a-z0-9_]{0,63}`, see section 3. |
| `payload` | object | Depends on `type`. |

A line is at most **1,048,576 bytes** (1 MiB) without its `\n`. Readers MUST ignore fields they do not know and SHOULD keep them when they copy a line. Newer apps read older files without migration, so there is no migration code.

All text in a line (mark text, comments, labels, author) is **untrusted input**. A reader MUST show it as plain text, never as markup or code.

## 3. Types

Defined (the payload is fixed here):

| `type` | Payload | Meaning |
|---|---|---|
| `mark_import` | a **mark** (below) | A mark seen for the first time in this paper's history. A writer checks the `key` against every line on disk, purged ones included, and writes only keys not yet there. |
| `purge` | `{ "covers": { "<device-id>": <seq>, … } }` | Forget: hides events (section 6). |
| `milestone` | `{ "label": string or null, "reason": "import" or "refresh" or "session" or "manual" or "before_restore", "upto": { "<device-id>": <seq>, … } }` | A named point in the history (section 7). |
| `milestone_rename` | `{ "milestone": "<id>", "label": string or null }` | Gives or changes a milestone's name. |

A milestone `label` (on `milestone` and `milestone_rename`) is at most **200 characters** (the same bound as an author name). A longer label, or an `upto` entry that is 0, makes the line `bad_payload`.
| `milestone_restore` | `{ "milestone": "<id>", "seen": { "<device-id>": <seq>, … } }` | Makes the current view equal to that milestone's, apart from events written after the restorer last looked (section 7). |

A **mark**: `key` (string, 1 to 128 bytes, the reference manager's key), `kind` (`highlight`, `note`, `underline`, `image` or `ink`), and optional `text`, `comment` (strings or null), `color` (`#rrggbb` or null), `page_index` (zero-based integer or null), `added`, `modified` (strings or null, as the source gave them).

Names reserved, payload not fixed yet in v0: `mark_update`, `mark_gone`, `note_revision`, `mark_create`, `mark_delete`, `mark_restore`. A writer MUST NOT write a reserved name until a later version of this document fixes its payload.

A line with a well-formed envelope (all the fields of section 2, correctly shaped) and any other `type`, reserved names included, is **not bad**: a reader keeps it, does not report it as a problem, and does not act on its content.

## 4. Reading

1. Split each file at `\n`. Judge each line on its own (section 5).
2. **Combine by `id`.** The set of all good lines from all devices is the history. Reading the files twice, or in any order, gives the same set. If two lines share an `id` but differ in content (a damaged or hostile copy), the line from the device whose `device-id` is first in byte order wins; within one file the earlier line wins. This needs no clock and gives every computer the same result.
3. **Display order:** sorted by `ts`, then `device`, then `seq`, then `id`. This order is for people. Only two rules below use it, both in section 7 and both only to choose between changes made at the same time on different computers: which name a milestone has, and which restore applies. Everything else depends on `seq` and `id` alone.

## 5. Bad lines

A line is **bad** and is skipped, never repaired, if it: is longer than 1 MiB (`too_long`); is not UTF-8 JSON (`not_json`); is not a JSON object (`not_object`); lacks a required field (`missing_field`); has a field of the wrong shape (`bad_value`); has an unknown `v` (`unsupported_version`); or has a `payload` that does not fit its defined `type` (`bad_payload`). A reader counts bad lines per paper and shows the count. Files are never changed because of them.

## 6. Forget (`purge`)

An event `e` is **hidden** if some purge `P` lists `e`'s device in its `covers` and `e.seq` is no larger than the number listed for that device. Hiding looks at `seq` only, so a wrong clock cannot hide or show anything. When there are several purges, take, for each device, the largest number listed. Events that the purging computer had not seen (for example written on another computer while it was offline) are not covered and stay visible. `purge` lines are never hidden: they are the "erased" marker. A purge does not reserve the `paper-id`; new events after it start a fresh history. Because `seq` never restarts (section 2), those new events are not covered by the old purge.

The format cannot delete copies that sync services or git already hold. Apps must say so.

## 7. Milestones and restore

- `milestone.upto` lists, for each device that had events at that point, the highest `seq` that belongs to the state. A device with no events is left out (an entry is never 0). The **state at a milestone** is the visible events with `seq ≤ upto[device]`; a device that is not listed contributes none. This is a cut through the set, so it needs no clock. `purge.covers` and `milestone_restore.seen` are cuts of the same kind and follow the same rule.
- An import writes **one** milestone when it changed the history: at least one new `mark_import` after the cut of the last milestone for its device. Its `reason` is `import` when the paper has no milestone yet, otherwise `refresh`. An import that changes nothing writes no milestone. The `upto` of the new milestone is the cut of every visible event at that moment.
- A name is the `label` of the milestone, replaced by the latest `milestone_rename` for its `id`, latest by display order. Renaming adds a line; no line is edited.
- `milestone_restore.seen` lists, in the same way, what the restoring computer could see when it wrote the line. The **current view** after a restore is the visible events that are in the state at the named milestone, plus the visible events with `seq > seen[device]` (or from a device not listed): the ones written after the restorer looked, for example on another computer. Events between the two cuts are not shown but stay in the files. A restore adds one line and changes none.
- If several restores exist, only the last one in display order applies; a restore always names a state in the past, so restoring a restore works: restore the `before_restore` milestone. Before writing a restore line, an app writes a milestone with `reason: "before_restore"` when there are changes since the last milestone, so restoring that milestone undoes the restore.
- A restore or rename that names a milestone that is missing or hidden has no effect.

## 8. Conformance

`spec/schema/line.schema.json` is the JSON Schema of a line (shape only; bounds on `id`, `ts` and the 1 MiB limit are in the text above). `spec/vectors/` holds sample lines and `manifest.json` says what every reader MUST decide for each one: `valid` (understood), `other` (kept, not interpreted) or the reason code of a bad line. `milestone` and `milestone_rename` are typed (slice 6 in [BUILD-PLAN](../docs/BUILD-PLAN.md)) and their samples are `valid`, with `bad_payload` samples for a 0 `upto` entry, an over-long label and a non-UUID rename target. `milestone_restore` stays `other` until its slice (10) lands. The reference implementation checks the vectors in CI (`src-core/tests/spec_vectors.rs`) and a second test fails if the schema and the code drift apart. A line over 1 MiB is generated by the test, not stored.
