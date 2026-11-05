#!/usr/bin/env python3
"""
Generates a valid 500-page, ~50 MB PDF fixture for ScribeTrace Spike 4 (Lightness & PDF.js).
Page 1 contains ground-truth selection text:
'ScribeTrace Benchmark Page 1 - Ground Truth Selection Text'
Followed by standard PDF text objects to reach ~100 KB per page (500 pages * 100 KB ~ 50 MB).
"""

import os
import sys

def generate_pdf(output_path, num_pages=500, target_mb=50):
    os.makedirs(os.path.dirname(os.path.abspath(output_path)), exist_ok=True)
    
    # Calculate padding needed per page to achieve target size (~100 KB per page)
    bytes_per_page = (target_mb * 1024 * 1024) // num_pages
    filler_chunk = "ScribeTrace performance benchmark filler stream data " * 10
    filler_lines = max(1, bytes_per_page // len(filler_chunk))
    filler_stream = ("\n".join([f"BT /F1 10 Tf 20 {y % 700 + 50} Td ({filler_chunk}) Tj ET" for y in range(filler_lines)])).encode("latin1")
    
    with open(output_path, "wb") as f:
        # PDF Header
        header = b"%PDF-1.4\n"
        f.write(header)
        
        xref = [0]
        
        def write_obj(content):
            offset = f.tell()
            xref.append(offset)
            idx = len(xref) - 1
            f.write(f"{idx} 0 obj\n".encode("ascii"))
            f.write(content)
            f.write(b"\nendobj\n")
            return idx

        # Object 1: Font Object
        font_id = write_obj(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>")
        
        # Object 2: Catalog
        cat_id = write_obj(b"<< /Type /Catalog /Pages 3 0 R >>")
        
        # Object 3: Pages object
        # Each page will have 2 objects:
        # Page 1: stream = 4, page = 5
        # Page p: stream = 2 + 2*p, page = 3 + 2*p
        page_ids = [3 + 2 * p for p in range(1, num_pages + 1)]
        kids_str = " ".join([f"{pid} 0 R" for pid in page_ids])
        pages_content = f"<< /Type /Pages /Kids [{kids_str}] /Count {num_pages} >>".encode("ascii")
        write_obj(pages_content)
        
        # Generate stream and page object for each page
        for p in range(1, num_pages + 1):
            if p == 1:
                p1_text = (
                    b"BT /F1 20 Tf 50 720 Td (ScribeTrace Benchmark Page 1 - Ground Truth Selection Text) Tj ET\n"
                    b"BT /F1 12 Tf 50 680 Td (Annotated highlight excerpt for anchoring verification) Tj ET\n"
                )
                stream_data = p1_text + filler_stream
            else:
                stream_data = f"BT /F1 16 Tf 50 720 Td (ScribeTrace Synthetic Page {p}) Tj ET\n".encode("latin1") + filler_stream
            
            stream_obj = write_obj(
                f"<< /Length {len(stream_data)} >>\nstream\n".encode("ascii")
                + stream_data
                + b"\nendstream"
            )
            
            page_obj = write_obj(
                f"<< /Type /Page /Parent 3 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 {font_id} 0 R >> >> /Contents {stream_obj} 0 R >>".encode("ascii")
            )
            assert page_obj == page_ids[p - 1], f"Mismatch: {page_obj} vs {page_ids[p - 1]}"

        # Write XRef table
        start_xref = f.tell()
        f.write(b"xref\n")
        f.write(f"0 {len(xref)}\n".encode("ascii"))
        f.write(b"0000000000 65535 f \n")
        for offset in xref[1:]:
            f.write(f"{offset:010d} 00000 n \n".encode("ascii"))
            
        # Trailer
        f.write(b"trailer\n")
        f.write(f"<< /Size {len(xref)} /Root 2 0 R >>\n".encode("ascii"))
        f.write(b"startxref\n")
        f.write(f"{start_xref}\n".encode("ascii"))
        f.write(b"%%EOF\n")

    size_mb = os.path.getsize(output_path) / (1024 * 1024)
    print(f"Generated {num_pages}-page PDF fixture: {output_path} ({size_mb:.2f} MB)")
    return size_mb

if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "fixtures/large_50mb_500pages.pdf"
    generate_pdf(out)
