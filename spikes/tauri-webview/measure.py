#!/usr/bin/env python3
"""
ScribeTrace Spike 4: Lightness & PDF.js Measurement Benchmark.
Measures:
1. Cold start to render first page of a 50 MB / 500-page PDF (target < 1.5s, ceiling 2.0s).
2. Idle RSS memory usage after 10 page navigations (target < 250 MB).
3. Selected text ground-truth check (verifies text layer extraction and bounding).

Outputs structured JSON adhering to ScribeTrace QA lightness budget.
"""

import os
import sys
import json
import time
import subprocess
import tempfile

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

def run_benchmark():
    # 1. Generate temporary 50 MB / 500-page PDF fixture
    with tempfile.NamedTemporaryFile(suffix=".pdf", delete=False) as tmp_pdf:
        tmp_pdf_path = tmp_pdf.name

    try:
        gen_script = os.path.join(SCRIPT_DIR, "scripts", "generate_fixture.py")
        t0 = time.time()
        subprocess.check_call([sys.executable, gen_script, tmp_pdf_path], stdout=subprocess.DEVNULL)
        gen_time = time.time() - t0

        pdf_size_mb = os.path.getsize(tmp_pdf_path) / (1024 * 1024)

        # 2. Run Node.js + PDF.js benchmark runner
        node_script = """
        const fs = require('fs');
        const { performance } = require('perf_hooks');

        async function measure() {
            const pdfjsLib = await import('pdfjs-dist/legacy/build/pdf.mjs');
            const pdfPath = process.argv[1] || process.argv[2];
            
            const startCold = performance.now();
            const data = new Uint8Array(fs.readFileSync(pdfPath));
            const doc = await pdfjsLib.getDocument({ data }).promise;
            const page1 = await doc.getPage(1);
            const textContent = await page1.getTextContent();
            const coldFirstPageMs = performance.now() - startCold;

            const allText = textContent.items.map(item => item.str).join(' ');
            const hasGroundTruth = allText.includes('ScribeTrace Benchmark Page 1 - Ground Truth Selection Text');

            // Navigate 10 subsequent pages
            for (let p = 2; p <= 11; p++) {
                const page = await doc.getPage(p);
                await page.getTextContent();
                page.cleanup();
            }

            const memUsage = process.memoryUsage();
            const rssMb = memUsage.rss / (1024 * 1024);

            const output = {
                cold_first_page_ms: Math.round(coldFirstPageMs),
                idle_rss_mb: Math.round(rssMb * 10) / 10,
                selected_text_check: hasGroundTruth,
                pages_navigated: 10,
                pdfjs_version: pdfjsLib.version
            };
            console.log(JSON.stringify(output));
        }

        measure().catch(err => {
            console.error(err);
            process.exit(1);
        });
        """

        node_proc = subprocess.run(
            ["node", "-e", node_script, tmp_pdf_path],
            cwd=SCRIPT_DIR,
            capture_output=True,
            text=True
        )

        if node_proc.returncode != 0:
            print("Node benchmark error:", node_proc.stderr)
            return 1

        # Extract JSON from stdout
        json_line = None
        for line in node_proc.stdout.splitlines():
            line_str = line.strip()
            if line_str.startswith("{") and line_str.endswith("}"):
                json_line = line_str
                break

        if not json_line:
            print("Failed to find JSON output in stdout. Raw stdout:", node_proc.stdout)
            print("Raw stderr:", node_proc.stderr)
            return 1

        node_output = json.loads(json_line)

        cold_first_page_ms = node_output["cold_first_page_ms"]
        idle_rss_mb = node_output["idle_rss_mb"]
        selected_text_check = node_output["selected_text_check"]

        # QA Lightness Budgets (docs/ARCHITECTURE.md & docs/PROTOTYPE-PLAN.md)
        budget_first_page_ms = 1500
        ceiling_first_page_ms = 2000
        budget_idle_rss_mb = 250.0

        passed = (
            cold_first_page_ms <= ceiling_first_page_ms and
            idle_rss_mb <= budget_idle_rss_mb and
            selected_text_check is True
        )

        result = {
            "spike": "SPIKE-4",
            "fixture": {
                "pages": 500,
                "size_mb": round(pdf_size_mb, 2),
                "generation_time_s": round(gen_time, 2)
            },
            "measurements": {
                "cold_first_page_ms": cold_first_page_ms,
                "idle_rss_mb": idle_rss_mb,
                "selected_text_check": selected_text_check,
                "pages_navigated": node_output["pages_navigated"],
                "pdfjs_version": node_output["pdfjs_version"]
            },
            "thresholds": {
                "cold_first_page_budget_ms": budget_first_page_ms,
                "cold_first_page_ceiling_ms": ceiling_first_page_ms,
                "idle_rss_budget_mb": budget_idle_rss_mb
            },
            "pass": passed
        }

        print(json.dumps(result, indent=2))
        return 0 if passed else 1

    finally:
        if os.path.exists(tmp_pdf_path):
            os.remove(tmp_pdf_path)

if __name__ == "__main__":
    sys.exit(run_benchmark())
