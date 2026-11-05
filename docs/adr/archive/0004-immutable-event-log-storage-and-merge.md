# ADR 0004: Immutable Event Log Storage, Deterministic Merge, and Compaction

**Status:** Archived, superseded by [ADR 0007](../0007-per-device-history-log-merge-and-forget.md)  
**Date:** 2026-11-05  
**Deciders:** Principal Software Architect, Product Management  
**Consulted:** QA Lead, Systems Engineering  

---

## 1. Context and Problem Statement

ScribeTrace records the full evolution of a researcher's reading comprehension over time (Constitution Rule 3). Highlights, comments, deletions, and edits must be preserved indefinitely without data loss.

Furthermore, academic researchers frequently synchronize project folders across multiple offline devices (laptop, university workstation, lab desktop) using external sync systems such as Git, Syncthing, Dropbox, Nextcloud, or Google Drive (Requirement F5.3).

This architecture poses three core challenges:
1. **Append-Only Durability:** Plain-text, human-readable file formats must survive process crashes, power failures, and partial writes.
2. **Clock Skew & Concurrent Mutations:** Two devices editing or deleting the same annotation while disconnected will have divergent system clocks. Naive physical wall-clock timestamps cause race conditions and lost updates.
3. **Log Growth & Replay Latency:** Long-term active projects may accumulate tens of thousands of micro-events, which could degrade document load times if replayed from scratch on every open.

---

## 2. Decision: JSON Lines Sidecars with Hybrid Logical Clocks (HLC)

### 2.1 File System Storage Layout
All project metadata is sequestered inside a single root directory at the project root:

```
<project-root>/
├── .scribetrace/
│   ├── workspace.json                  <-- Workspace settings & path-to-hash index
│   ├── workspace-notes.md              <-- Project-wide synthesis notes
│   ├── workspace-reading-log.jsonl     <-- Project-wide reading session timeline
│   ├── annotations/
│   │   ├── <content-hash>.jsonl        <-- Active append-only annotation event stream
│   │   ├── <content-hash>.snapshot.json<-- Consolidated state snapshot (when compacted)
│   │   └── <content-hash>.archive.jsonl<-- Archived historical events
│   ├── notes/
│   │   ├── <content-hash>.history.jsonl<-- Canonical note edit history log
│   │   └── <content-hash>.md           <-- Projected plain Markdown note (external interop)
│   ├── reading-log/
│   │   └── <content-hash>.jsonl        <-- Per-document reading session duration log
│   ├── cache/
│   │   └── index.db                    <-- Throwaway SQLite FTS5 search & chunk cache
│   └── corrupt/                        <-- Quarantined malformed JSON lines
└── papers/
    ├── attention-is-all-you-need.pdf
    └── monograph.epub
```

### 2.2 Event Envelope Schema
Every line in `.scribetrace/annotations/<content-hash>.jsonl` is a self-contained JSON object:

```json
{
  "v": 1,
  "event_id": "019a4f2b-8c10-7000-84c1-92b0c3456789",
  "target_id": "019a4f20-31a4-7000-a29d-4819ca114521",
  "hlc": "2026-11-05T09:45:00.120Z-0001-node_a1b2c3",
  "author": "Dr. Aris Thorne",
  "action": "update",
  "payload": {
    "color": "#ffeb3b",
    "comment": "Key insight: self-attention replaces recurrence entirely."
  }
}
```

* **`v` (integer):** Explicit schema envelope version. Future versions are backwards-compatible.
* **`event_id` (UUIDv7):** Time-ordered unique identifier.
* **`target_id` (UUIDv7):** Identifier of the annotation aggregate being mutated.
* **`hlc` (string):** Hybrid Logical Clock timestamp `(physical_iso_ms - logical_counter - client_id)`.
* **`action` (enum):** `create`, `update`, `delete`, `restore`, `import`.
* **`payload` (object):** Mutation attributes. Unrecognized keys are preserved during projection.

---

## 3. Deterministic Union Merge Algorithm

When multiple devices synchronize through external file sync tools:
1. **Deduplication:** The union of all events from both files is taken, deduplicated by `event_id`.
2. **Causal Ordering:** Events are sorted deterministically using the tuple:
   $$\text{sort\_key} = (\text{hlc.physical\_time ASC},\; \text{hlc.logical\_counter ASC},\; \text{author ASC},\; \text{event\_id ASC})$$
3. **State Reduction (Replay):**
   * An `action: "delete"` event acts as a tombstone. In the event of identical HLC timestamps between an update and a delete, `delete` always supersedes.
   * Restoring an annotation appends an `action: "restore"` event referencing the historical state.
4. **Conflict File Harvester:**
   External sync tools (Dropbox, Syncthing) create conflict duplicates when simultaneous writes occur:
   * `<hash>.sync-conflict-*.jsonl` (Syncthing)
   * `<hash> (conflicted copy *).jsonl` (Dropbox)
   The ScribeTrace file watcher detects these patterns, loads both files, executes an in-memory union merge, writes the merged result to `<hash>.jsonl`, and safely removes the conflict duplicate file.

---

## 4. Log Compaction and Snapshotting Strategy

To prevent unlimited event log growth from degrading document startup latency:
1. **Trigger Condition:** Compaction runs when active event count $> 5,000$ OR sidecar size $> 5\text{ MB}$.
2. **Snapshot Creation:**
   * Materialize the current active state of all annotations up to event watermark $W$.
   * Write materialized state atomically to `<hash>.snapshot.json.tmp` and rename to `<hash>.snapshot.json`.
3. **Log Rotation:**
   * Append events older than $W$ to `<hash>.archive.jsonl`.
   * Reset `<hash>.jsonl` to contain only events newer than watermark $W$.
4. **Document Startup:**
   $$\text{CurrentState} = \text{Replay}(\text{LoadSnapshot}(), \; \text{LoadActiveDeltaEvents}())$$
   Ensures document load time remains $< 50\text{ ms}$ indefinitely, regardless of whether a document has 100 or 100,000 historical edits.

---

## 5. Durability & Crash Recovery (Atomic Writes)

1. **JSONL Appends:** Every append operation writes to the file handle followed by an explicit operating system `fsync` / `FlushFileBuffers` before returning to the UI.
2. **State & Index Rewrites:** Workspace settings, snapshot files, and projected notes are written to temporary files in the same directory (`.tmp.XXXXXX`) and committed via atomic filesystem `rename()`.
3. **Corrupt Tail Truncation:** If a hard crash occurs during a disk write, the sidecar parser inspects the final line. If the trailing line is incomplete or malformed JSON, it is safely truncated to the last valid newline character, and the corrupt snippet is quarantined in `.scribetrace/corrupt/`.

---

## 6. Consequences

### Positive:
* Absolute data safety: zero risk of data loss or corruption during sync collisions.
* 100% offline autonomy with zero server coordinator needed.
* Deterministic state reproduction verified via property-based automated tests.
* Bounded memory and sub-second load times via snapshot compaction.

### Negative & Mitigations:
* *Archive accumulation:* Archived event files remain on disk.
  * *Mitigation:* Event archives are plain JSON Lines, highly compressible (gzip compression reduces size by > 90%), and can be manually archived or inspected by users.

---

## 7. Security Impact

* **Directory Traversal & Path Confinement:** Sidecar file paths are strictly computed by hashing document contents into hex strings (`[a-f0-9]{64}`). Filenames cannot contain relative path tokens (`../`, `..\\`), symlinks, or alternate data streams. File writes are confined strictly to `<workspace>/.scribetrace/annotations/`.
* **Hostile Sidecar Ingestion Defense:** Sidecar files arriving via git or sync services are treated as untrusted inputs. Every line must pass strict JSON schema validation. Malformed or oversized lines (> 1 MB per line) are rejected and quarantined into `.scribetrace/corrupt/`.
* **Tamper-Evident Event Logging:** Event IDs use cryptographically secure UUIDv7. An optional cryptographic hash chain (`prev_hash` linking each event to its predecessor) provides verifiable tamper-evident auditing for academic review integrity.
* **Advisory Process Locking:** Multi-process concurrency uses OS-level advisory locking (`fs2` crate) to prevent race conditions or partial file corruption during simultaneous multi-instance operations.

