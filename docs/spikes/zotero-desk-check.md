# SPIKE-3 Desk Check: Zotero 7 Local HTTP Interface Contract

**Document:** `docs/spikes/zotero-desk-check.md`  
**Author:** DevOps Engineer (`agent-1793867530011-yoc5o5`)  
**Date:** 2026-11-05  
**Target Specification:** [`docs/PROTOTYPE-PLAN.md`](../PROTOTYPE-PLAN.md) (SPIKE-3 Day 1 Gate), [`docs/adr/0009-zotero-integration.md`](../adr/0009-zotero-integration.md), [`docs/STRESS-TEST.md`](../STRESS-TEST.md) (ST-02).  
**Status:** Desk check complete. Live execution deferred to user's local machine (Zotero is not installed in the CI / agent sandbox).

---

## 1. Executive Summary

This desk research verifies the technical feasibility of extracting paper metadata and user annotations from Zotero 7 via its local HTTP loopback server (`127.0.0.1:23119`), as stipulated in ADR 0009.

**Key Findings:**
1. **Endpoint Exists:** Zotero 7 exposes an HTTP API on `http://127.0.0.1:23119/api/users/0/...`.
2. **Annotation Fields Present:** Annotation items expose `itemType="annotation"` with `annotationType`, `annotationText`, `annotationColor`, and `annotationPosition` (`pageIndex` and `rects`).
3. **No Auth on Loopback:** Local requests do not require an API key or auth token.
4. **Configuration Requirement:** The user must explicitly enable the setting in Zotero Preferences: *"Allow other applications on this computer to communicate with Zotero"*.

---

## 2. Interface Activation & Connection Protocol

| Aspect | Specification | Source & Date | Verification Status |
|---|---|---|---|
| **Local Port & Host** | `http://127.0.0.1:23119` (loopback only) | [Zotero KB: Connector Server](https://www.zotero.org/support/kb/connector_server) (checked: 2026-11-05) | `[Documented]` |
| **Enabling Setting** | Zotero UI: `Settings` (or `Preferences`) → `Advanced` → Checkbox: *"Allow other applications on this computer to communicate with Zotero"*. | [Zotero Forum: Local API](https://forums.zotero.org/discussion/93558/local-api-access) (checked: 2026-11-05) | `[Documented]` |
| **Config Flag** | `extensions.zotero.httpServer.enabled = true` in Zotero `about:config`. | [zotero/zotero repo: Server.js](https://github.com/zotero/zotero/blob/master/chrome/content/zotero/xpcom/server.js) (checked: 2026-11-05) | `[Documented]` |
| **Ping Healthcheck** | `GET http://127.0.0.1:23119/connector/ping` returns HTTP 200 with text `"Zotero is running"`. | [Zotero Support: Connector Ping](https://www.zotero.org/support/kb/connector_server) (checked: 2026-11-05) | `[Documented]` |
| **Authentication** | None required for read operations across loopback (`127.0.0.1`). | [Zotero API: Local Server](https://www.zotero.org/support/dev/web_api/v3/basics) (checked: 2026-11-05) | `[Documented]` |
| **API Version Header** | `Zotero-API-Version: 3` recommended in request headers. | [Zotero Web API v3](https://www.zotero.org/support/dev/web_api/v3/basics) (checked: 2026-11-05) | `[Documented]` |

---

## 3. Query Endpoints & Hierarchy

Zotero models PDF annotations as **child items** belonging to a parent attachment:
```
Top-Level Item (e.g. journalArticle, key: "PAPER123")
  └── Attachment Item (e.g. PDF, key: "ATTACH456")
        └── Annotation Items (itemType: "annotation", parentItem: "ATTACH456")
```

### Endpoints:

1. **List User Top-Level Items (Papers):**
   ```http
   GET http://127.0.0.1:23119/api/users/0/items/top?format=json&limit=100
   ```
   *Claim:* Returns top-level bibliographic items.  
   *Source:* [Zotero Web API Read Requests](https://www.zotero.org/support/dev/web_api/v3/read_requests) (checked: 2026-11-05). `[Documented]`

2. **List Child Annotations of an Attachment:**
   ```http
   GET http://127.0.0.1:23119/api/users/0/items/<attachmentKey>/children?itemType=annotation&format=json&limit=100
   ```
   *Claim:* Returns all annotations attached to the specified PDF attachment.  
   *Source:* [Zotero API: Item Children](https://www.zotero.org/support/dev/web_api/v3/read_requests#item_children) (checked: 2026-11-05). `[Documented]`

3. **Global Annotations Search (Fallback):**
   ```http
   GET http://127.0.0.1:23119/api/users/0/items?itemType=annotation&format=json&limit=100
   ```
   *Claim:* Top-level query for annotations may return empty or subset depending on desktop client schema version; traversing attachment children is the canonical reliable path.  
   *Source:* [Zotero Client XPCOM Server](https://github.com/zotero/zotero) (checked: 2026-11-05). `[Documented]` / `[Unverified-live]`

---

## 4. Annotation Schema & Required Field Audit

When an item with `itemType: "annotation"` is returned, the JSON payload exposes the following fields:

| Field Name | Expected Type | Description | Source & Date | Verdict |
|---|---|---|---|---|
| `key` | `string` (8 chars, e.g. `"4ZJ6FJXV"`) | Unique identifier of the annotation item in Zotero. | [Zotero Schema](https://www.zotero.org/support/dev/web_api/v3/read_requests) (2026-11-05) | `[Documented]` |
| `itemType` | `string` | Must be strictly `"annotation"`. | [Zotero Item Types](https://www.zotero.org/support/dev/web_api/v3/basics) (2026-11-05) | `[Documented]` |
| `parentItem` | `string` (key) | Key of the parent PDF attachment. | [Zotero Schema](https://www.zotero.org/support/dev/web_api/v3/read_requests) (2026-11-05) | `[Documented]` |
| `annotationType` | `string` | Subtype: `"highlight"`, `"underline"`, `"note"`, `"image"`, `"ink"`. | [Zotero Reader Source](https://github.com/zotero/zotero/blob/master/chrome/content/zotero/xpcom/data/item.js) (2026-11-05) | `[Documented]` |
| `annotationText` | `string` | The extracted text highlighted or underlined by the user. | [Zotero Reader Source](https://github.com/zotero/zotero) (2026-11-05) | `[Documented]` |
| `annotationComment` | `string` | User's attached note or commentary on the highlight. | [Zotero Reader Source](https://github.com/zotero/zotero) (2026-11-05) | `[Documented]` |
| `annotationColor` | `string` (hex, e.g. `"#ffd400"`) | Color assigned to the mark in Zotero reader. | [Zotero Reader Source](https://github.com/zotero/zotero) (2026-11-05) | `[Documented]` |
| `annotationPosition` | `string` (JSON string) | Serialized object: `{"pageIndex": <int>, "rects": [[x1, y1, x2, y2], ...]}`. | [Zotero Annotations JSON](https://forums.zotero.org/discussion/89408/pdf-annotations-in-the-api) (2026-11-05) | `[Documented]` |
| `annotationPageLabel`| `string` (e.g. `"9"`) | Visual page label printed in the PDF document. | [Zotero Reader Source](https://github.com/zotero/zotero) (2026-11-05) | `[Documented]` |
| `dateAdded` | `string` (ISO-8601 UTC) | Creation timestamp. | [Zotero Schema](https://www.zotero.org/support/dev/web_api/v3/read_requests) (2026-11-05) | `[Documented]` |
| `dateModified` | `string` (ISO-8601 UTC) | Last update timestamp. | [Zotero Schema](https://www.zotero.org/support/dev/web_api/v3/read_requests) (2026-11-05) | `[Documented]` |

### Example Wire JSON Representation:
```json
{
  "key": "4ZJ6FJXV",
  "version": 2468,
  "itemType": "annotation",
  "parentItem": "255HN5RN",
  "annotationType": "highlight",
  "annotationAuthorName": "Me",
  "annotationText": "Deep learning models require extensive computational resources for convergence.",
  "annotationComment": "Key point for methodology rebuttal section.",
  "annotationColor": "#ffd400",
  "annotationPageLabel": "4",
  "annotationSortIndex": "00003|000412|00574",
  "annotationPosition": "{\"pageIndex\":3,\"rects\":[[68.604,246.586,174.372,267.047]]}",
  "dateAdded": "2026-11-05T08:12:00Z",
  "dateModified": "2026-11-05T08:15:30Z"
}
```

---

## 5. Pagination and Rate Limits

| Parameter / Header | Behavior | Source & Date | Status |
|---|---|---|---|
| `limit` | Controls batch size: `1` to `100` (default: `25`). Max `100`. | [Zotero API: Sorting and Paging](https://www.zotero.org/support/dev/web_api/v3/sorting_and_paging) (2026-11-05) | `[Documented]` |
| `start` | 0-indexed offset parameter for subsequent pages. | [Zotero API: Sorting and Paging](https://www.zotero.org/support/dev/web_api/v3/sorting_and_paging) (2026-11-05) | `[Documented]` |
| `Total-Results` | Response header stating the total count of matched records. | [Zotero API: Headers](https://www.zotero.org/support/dev/web_api/v3/basics) (2026-11-05) | `[Documented]` |
| `Link` | RFC 5988 header with `rel="next"`, `rel="last"` links. | [Zotero API: Headers](https://www.zotero.org/support/dev/web_api/v3/basics) (2026-11-05) | `[Documented]` |
| Local Rate Limits | No rate limits on loopback interface (`127.0.0.1:23119`). | [Zotero Server Source](https://github.com/zotero/zotero) (2026-11-05) | `[Documented]` / `[Unverified-live]` |

---

## 6. Live Verification Protocol (For the Maintainer / User)

Because the CI and development agent environments do not have a GUI or Zotero 7 installed, the live check must be run on a developer machine with Zotero 7 running:

```bash
# 1. Verify Zotero local server is active
curl -s http://127.0.0.1:23119/connector/ping
# Expected: HTTP 200 "Zotero is running"

# 2. Query top-level items
curl -s "http://127.0.0.1:23119/api/users/0/items/top?limit=1&format=json" | jq '.[0]'

# 3. Query annotations for a known attachment key
curl -s "http://127.0.0.1:23119/api/users/0/items/<ATTACHMENT_KEY>/children?itemType=annotation&format=json" | jq '.[0]'
```

### Risk Evaluation & Mitigation
- **Risk (ST-02):** If live probe against Zotero 7 reveals missing bounding boxes or disabled API, ADR 0009 requires triggering **Decision for the user #4** (e.g. user-exported annotation `.json` import or lightweight plugin bridge).
- **Desk Verdict:** The documented contract supports all required fields for ScribeTrace Stage 1 history import without modification.
