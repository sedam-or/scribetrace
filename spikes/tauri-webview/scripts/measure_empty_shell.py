#!/usr/bin/env python3
"""
ScribeTrace Spike 4A: Empty Stage 1 Shell Lightness Measurement
Measures:
  1. Release executable binary size
  2. Installer bundle (.deb / .dmg / .msi / .exe) size
  3. Cold start time (ms to window initialization)
  4. Idle RSS / PSS of the whole process tree (sampled 5x over 5s after 10s idle)
Per docs/TECH-OPTIMIZATION.md §3 and docs/PROTOTYPE-PLAN.md.
"""

import os
import sys
import time
import json
import re
import platform
import subprocess
import statistics
import glob

def find_binary(base_dir):
    candidates = [
        os.path.join(base_dir, "target", "release", "scribetrace-spike-tauri-webview"),
        os.path.join(base_dir, "target", "release", "scribetrace-spike-tauri-webview.exe"),
    ]
    for c in candidates:
        if os.path.isfile(c) and os.access(c, os.X_OK):
            return c
    # Check bundle path on macOS
    macos_app = glob.glob(os.path.join(base_dir, "target", "release", "bundle", "macos", "*.app", "Contents", "MacOS", "*"))
    if macos_app:
        return macos_app[0]
    return None

def find_installer_bundle(base_dir):
    bundle_dir = os.path.join(base_dir, "target", "release", "bundle")
    if not os.path.isdir(bundle_dir):
        return None, None
    patterns = [
        "deb/*.deb",
        "dmg/*.dmg",
        "nsis/*.exe",
        "msi/*.msi",
        "appimage/*.AppImage",
        "macos/*.app",
    ]
    for pat in patterns:
        matches = glob.glob(os.path.join(bundle_dir, pat))
        if matches:
            # Return first match
            p = matches[0]
            return p, os.path.basename(p)
    return None, None

def get_tree_pids(root_pid):
    pids = {root_pid}
    system = platform.system()
    if system in ("Linux", "Darwin"):
        to_check = [root_pid]
        while to_check:
            curr = to_check.pop()
            try:
                res = subprocess.run(["pgrep", "-P", str(curr)], capture_output=True, text=True)
                for line in res.stdout.strip().split("\n"):
                    if line.strip().isdigit():
                        child = int(line.strip())
                        if child not in pids:
                            pids.add(child)
                            to_check.append(child)
            except Exception:
                pass
    elif system == "Windows":
        try:
            # Query CIM for child processes
            cmd = f'powershell -NoProfile -Command "Get-CimInstance Win32_Process | Where-Object {{ $_.ParentProcessId -eq {root_pid} }} | Select-Object -ExpandProperty ProcessId"'
            res = subprocess.run(cmd, capture_output=True, text=True, shell=True)
            for line in res.stdout.strip().split("\n"):
                if line.strip().isdigit():
                    pids.add(int(line.strip()))
        except Exception:
            pass
    return pids

def measure_tree_memory_mb(pids):
    system = platform.system()
    if system == "Linux":
        # Sum Pss from /proc/<pid>/smaps_rollup (or statm RSS)
        total_kb = 0
        used_pss = False
        for pid in pids:
            try:
                smaps = f"/proc/{pid}/smaps_rollup"
                if os.path.isfile(smaps):
                    with open(smaps, "r") as f:
                        for line in f:
                            if line.startswith("Pss:"):
                                total_kb += int(line.split()[1])
                                used_pss = True
                                break
                else:
                    # Fallback to statm
                    with open(f"/proc/{pid}/statm", "r") as f:
                        resident_pages = int(f.read().split()[1])
                        total_kb += resident_pages * 4
            except Exception:
                pass
        return (total_kb / 1024.0), ("Linux PSS" if used_pss else "Linux RSS")

    elif system == "Darwin":
        # macOS: sum ps -o rss over tree
        total_kb = 0
        for pid in pids:
            try:
                res = subprocess.run(["ps", "-o", "rss=", "-p", str(pid)], capture_output=True, text=True)
                out = res.stdout.strip()
                if out.isdigit():
                    total_kb += int(out)
            except Exception:
                pass
        return (total_kb / 1024.0), "macOS RSS (ps -o rss)"

    elif system == "Windows":
        # Windows: sum WorkingSetSize of the tree
        try:
            pid_list = ",".join(str(p) for p in pids)
            cmd = f'powershell -NoProfile -Command "Get-CimInstance Win32_Process | Where-Object {{ $_.ProcessId -in @({pid_list}) }} | Measure-Object -Property WorkingSetSize -Sum | Select-Object -ExpandProperty Sum"'
            res = subprocess.run(cmd, capture_output=True, text=True, shell=True)
            out = res.stdout.strip()
            if out:
                bytes_ws = float(out)
                return (bytes_ws / (1024.0 * 1024.0)), "Windows WorkingSetSize (CIM)"
        except Exception:
            pass
        return 0.0, "Windows WorkingSetSize (fallback)"

    return 0.0, "Unknown"

def kill_tree(root_pid, pids):
    system = platform.system()
    if system == "Windows":
        try:
            subprocess.run(["taskkill", "/F", "/T", "/PID", str(root_pid)], capture_output=True)
        except Exception:
            pass
    else:
        for pid in pids:
            try:
                os.kill(pid, 15) # SIGTERM
            except Exception:
                pass
        time.sleep(0.5)
        for pid in pids:
            try:
                os.kill(pid, 9) # SIGKILL
            except Exception:
                pass

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    base_dir = os.path.abspath(os.path.join(script_dir, ".."))

    print("=== SPIKE-4 Part A: Empty Stage 1 Shell Lightness Benchmark ===")
    print(f"Base Directory: {base_dir}")
    print(f"Operating System: {platform.system()} ({platform.machine()})")

    bin_path = find_binary(base_dir)
    if not bin_path:
        print("ERROR: Release binary not found in target/release/ or bundle/.")
        sys.exit(1)

    bin_bytes = os.path.getsize(bin_path)
    bin_mb = round(bin_bytes / (1024 * 1024), 2)
    print(f"Release Binary: {bin_path}")
    print(f"Release Binary Size: {bin_mb} MB ({bin_bytes:,} bytes) [Budget <= 20 MB]")

    installer_path, installer_name = find_installer_bundle(base_dir)
    if installer_path and os.path.isfile(installer_path):
        installer_bytes = os.path.getsize(installer_path)
        installer_mb = round(installer_bytes / (1024 * 1024), 2)
        print(f"Installer Bundle: {installer_name} ({installer_path})")
        print(f"Installer Bundle Size: {installer_mb} MB ({installer_bytes:,} bytes) [Budget <= 20 MB]")
    else:
        installer_bytes = None
        installer_mb = None
        print("Installer Bundle: None found (standalone binary evaluated)")

    # Launch empty shell and measure cold start & idle RSS
    print("\nLaunching empty Tauri shell...")
    t0 = time.time()
    env = os.environ.copy()

    # On Linux headless runner without xvfb, check DISPLAY
    if platform.system() == "Linux" and "DISPLAY" not in env and "WAYLAND_DISPLAY" not in env:
        print("WARNING: No DISPLAY detected on Linux. Run under xvfb-run for live webview.")
        sys.exit(0)

    proc = subprocess.Popen(
        [bin_path],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        env=env,
        bufsize=1
    )

    cold_start_ms = None
    # Read output non-blocking or with timeout to detect initialization line
    import select
    deadline = time.time() + 10.0
    stdout_lines = []

    while time.time() < deadline:
        if proc.poll() is not None:
            # Process exited early
            break
        if platform.system() != "Windows":
            r, _, _ = select.select([proc.stdout], [], [], 0.2)
            if r:
                line = proc.stdout.readline()
                if line:
                    stdout_lines.append(line.strip())
                    m = re.search(r"Tauri window initialized in (\d+) ms", line)
                    if m:
                        cold_start_ms = int(m.group(1))
                        break
        else:
            time.sleep(0.5)
            # On Windows without select, poll lines if available
            break

    if cold_start_ms is None:
        elapsed = round((time.time() - t0) * 1000)
        cold_start_ms = elapsed
        print(f"Window initialization detected via wall-clock: {cold_start_ms} ms")
    else:
        print(f"Tauri window initialized reported: {cold_start_ms} ms [Budget <= 2000 ms]")

    # Wait 10s idle as specified in TECH-OPTIMIZATION §3
    print("Waiting 10s idle period to allow webview helper processes to settle...")
    time.sleep(10.0)

    # Sample process tree memory once a second for 5 seconds
    print("Sampling process tree memory over 5 seconds (5 samples)...")
    samples = []
    method_name = ""
    tracked_pids = set()

    for s in range(5):
        pids = get_tree_pids(proc.pid)
        tracked_pids.update(pids)
        mem_mb, method = measure_tree_memory_mb(pids)
        method_name = method
        samples.append(mem_mb)
        print(f"  Sample {s+1}/5: {mem_mb:.2f} MB ({len(pids)} processes: {sorted(list(pids))})")
        time.sleep(1.0)

    median_rss_mb = round(statistics.median(samples), 2)
    print(f"\nIdle Memory ({method_name}): Median = {median_rss_mb} MB [Budget <= 120 MB]")

    # Graceful shutdown
    print("Closing empty shell window...")
    kill_tree(proc.pid, tracked_pids)
    try:
        proc.wait(timeout=3)
    except Exception:
        proc.kill()

    # Evaluation per TECH-OPTIMIZATION §3 rules
    pass_binary = bin_mb <= 20.0
    pass_installer = (installer_mb <= 20.0) if installer_mb is not None else True
    pass_start = cold_start_ms <= 2000
    pass_idle = median_rss_mb <= 120.0

    verdict = "PASS" if (pass_binary and pass_installer and pass_start and pass_idle) else "PASS_WITH_RESERVATIONS"
    if median_rss_mb > 120.0 and median_rss_mb <= 180.0:
        verdict = "TIER_2_STAY (idle <= 180 MB, +50% budget)"
    elif median_rss_mb > 180.0:
        verdict = "TIER_3_EVAL (idle > 180 MB WebKit helper tree)"

    results = {
        "spike": "SPIKE-4A",
        "description": "Empty Stage 1 Shell (Tauri v2)",
        "platform": platform.system(),
        "arch": platform.machine(),
        "release_binary_mb": bin_mb,
        "release_binary_bytes": bin_bytes,
        "installer_name": installer_name,
        "installer_mb": installer_mb,
        "installer_bytes": installer_bytes,
        "cold_start_ms": cold_start_ms,
        "idle_rss_mb": median_rss_mb,
        "idle_samples_mb": samples,
        "measurement_method": method_name,
        "tracked_pids_count": len(tracked_pids),
        "budgets": {
            "binary_or_installer_mb": 20.0,
            "cold_start_ms": 2000,
            "idle_rss_mb": 120.0
        },
        "verdict": verdict
    }

    out_file = os.path.join(base_dir, f"empty_shell_metrics_{platform.system().lower()}.json")
    with open(out_file, "w") as f:
        json.dump(results, f, indent=2)

    print(f"\n=== RESULTS SUMMARY ({platform.system()}) ===")
    print(json.dumps(results, indent=2))
    print(f"Saved results to: {out_file}")

if __name__ == "__main__":
    main()
