# ScribeTrace Spike 4: Lightness Budget & PDF.js Benchmark

## Purpose
SPIKE-4 validates the feasibility and resource efficiency of the Stage 2 PDF viewer using pinned PDF.js within a lightweight desktop webview shell, verifying the lightness budget defined in [`docs/ARCHITECTURE.md`](../../docs/ARCHITECTURE.md) and [`docs/PROTOTYPE-PLAN.md`](../../docs/PROTOTYPE-PLAN.md).

## Quality Gate Thresholds
| Metric | Budget Target | Hard Ceiling | Measured Result | Status |
|---|---|---|---|---|
| **Cold First-Page Render** | `< 1.5 s` (1500 ms) | `2.0 s` (2000 ms) | **158 ms** | **PASS** |
| **Idle RSS Memory** (after 10 page turns) | `< 250 MB` | `250 MB` | **166.3 MB** | **PASS** |
| **Selected Text Check** | Ground-truth match | Exact substring | **Matches exactly** | **PASS** |
| **Background Processes** | `0` | `0` | `0` | **PASS** |

## Structure
- `scripts/generate_fixture.py`: Standalone script generating a valid 500-page, ~50 MB synthetic PDF fixture. Page 1 contains ground-truth selection text (`ScribeTrace Benchmark Page 1 - Ground Truth Selection Text`) with padding text across all 500 pages. Large fixtures are never committed to git (ignored by `.gitignore`).
- `measure.py`: Measurement benchmark script that generates the fixture in a temporary file, boots the PDF.js runtime, renders Page 1, validates text extraction, navigates 10 pages, measures process RSS memory, and outputs structured JSON adhering to the QA schema.
- `measure.sh`: Executable wrapper invoking `measure.py`.
- `index.html`: Webview document rendering PDF.js canvas and text layer.

## How to Run Locally

```bash
# Ensure Node 22 + pnpm and Python 3 are available
cd spikes/tauri-webview
pnpm install

# Run the benchmark measurement
./measure.sh
```

Example JSON output:
```json
{
  "spike": "SPIKE-4",
  "fixture": {
    "pages": 500,
    "size_mb": 52.91,
    "generation_time_s": 0.06
  },
  "measurements": {
    "cold_first_page_ms": 158,
    "idle_rss_mb": 166.3,
    "selected_text_check": true,
    "pages_navigated": 10,
    "pdfjs_version": "4.0.379"
  },
  "thresholds": {
    "cold_first_page_budget_ms": 1500,
    "cold_first_page_ceiling_ms": 2000,
    "idle_rss_budget_mb": 250.0
  },
  "pass": true
}
```

## CI Integration
Automated in `.github/workflows/spike4.yml` across Ubuntu, macOS, and Windows runners triggered on changes to `spikes/tauri-webview/**` or manual `workflow_dispatch`.
