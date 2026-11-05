/**
 * ScribeTrace E2E Test Harness Manager
 *
 * Spawns and supervises:
 * 1. zotero-standin (TCP loopback server on 127.0.0.1:23119 serving tests/fixtures/zotero/)
 * 2. tauri-driver (port 4444)
 * 3. Test project directories (.scribetrace/) with clean isolation
 */

import { spawn } from 'node:child_process';
import { mkdtempSync, rmSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

export class HarnessManager {
  constructor(options = {}) {
    this.projectRoot = resolve(options.projectRoot || '.');
    this.fixturesDir = resolve(options.fixturesDir || join(this.projectRoot, 'tests/fixtures/zotero'));
    this.zoteroPort = options.zoteroPort || 23119;
    this.driverPort = options.driverPort || 4444;

    this.zoteroProc = null;
    this.driverProc = null;
    this.tempProjectDir = null;
  }

  /**
   * Creates an isolated temporary directory for the project under test.
   */
  createTempProject() {
    this.tempProjectDir = mkdtempSync(join(tmpdir(), 'scribetrace-e2e-project-'));
    return this.tempProjectDir;
  }

  /**
   * Starts zotero-standin serving recorded fixtures.
   */
  async startZoteroStandin() {
    // Check if port is already active
    try {
      const probe = await fetch(`http://127.0.0.1:${this.zoteroPort}/`);
      if (probe.status) {
        console.log(`[Harness] zotero-standin already running on port ${this.zoteroPort}`);
        return;
      }
    } catch {
      // Port is free, start server
    }

    console.log(`[Harness] Starting zotero-standin serving "${this.fixturesDir}" on port ${this.zoteroPort}...`);
    this.zoteroProc = spawn('cargo', ['run', '-p', 'zotero-standin', '--', this.fixturesDir, String(this.zoteroPort)], {
      cwd: this.projectRoot,
      stdio: ['ignore', 'pipe', 'pipe']
    });

    this.zoteroProc.on('error', err => {
      console.error(`[zotero-standin error] Failed to spawn process: ${err.message}`);
    });

    this.zoteroProc.stderr.on('data', d => {
      const msg = d.toString().trim();
      if (msg) console.error(`[zotero-standin stderr] ${msg}`);
    });

    // Wait up to 10 seconds for server to be responsive
    const start = Date.now();
    while (Date.now() - start < 10000) {
      try {
        const res = await fetch(`http://127.0.0.1:${this.zoteroPort}/`);
        if (res.status === 404 || res.status === 405 || res.status === 200) {
          console.log(`[Harness] zotero-standin ready on port ${this.zoteroPort}`);
          return;
        }
      } catch {
        await new Promise(r => setTimeout(r, 200));
      }
    }
    throw new Error(`Failed to start zotero-standin on 127.0.0.1:${this.zoteroPort} within 10s`);
  }

  /**
   * Starts tauri-driver on port 4444 if not already running.
   */
  async startTauriDriver() {
    try {
      const probe = await fetch(`http://127.0.0.1:${this.driverPort}/status`);
      if (probe.status === 200) {
        console.log(`[Harness] tauri-driver already running on port ${this.driverPort}`);
        return;
      }
    } catch {
      // Driver not running, proceed to spawn
    }

    console.log(`[Harness] Spawning tauri-driver on port ${this.driverPort}...`);
    this.driverProc = spawn('tauri-driver', ['--port', String(this.driverPort)], {
      stdio: ['ignore', 'pipe', 'pipe'],
      env: {
        ...process.env,
        NO_AT_BRIDGE: '1',
        WEBKIT_DISABLE_DMABUF_RENDERER: '1',
        LIBGL_ALWAYS_SOFTWARE: '1'
      }
    });

    this.driverProc.on('error', err => {
      console.error(`[tauri-driver error] Failed to spawn tauri-driver: ${err.message}. Ensure tauri-driver is installed via 'cargo install tauri-driver'.`);
    });

    this.driverProc.stderr.on('data', d => {
      const msg = d.toString().trim();
      if (msg) console.error(`[tauri-driver stderr] ${msg}`);
    });

    const start = Date.now();
    while (Date.now() - start < 10000) {
      try {
        const res = await fetch(`http://127.0.0.1:${this.driverPort}/status`);
        if (res.status === 200) {
          console.log(`[Harness] tauri-driver ready on port ${this.driverPort}`);
          return;
        }
      } catch {
        await new Promise(r => setTimeout(r, 200));
      }
    }
    throw new Error(`Failed to connect to tauri-driver on 127.0.0.1:${this.driverPort} within 10s`);
  }

  /**
   * Cleans up all child processes and temporary directories.
   */
  teardown() {
    console.log('[Harness] Teardown: Stopping child services...');
    if (this.zoteroProc) {
      try { this.zoteroProc.kill('SIGTERM'); } catch {}
      this.zoteroProc = null;
    }
    if (this.driverProc) {
      try { this.driverProc.kill('SIGTERM'); } catch {}
      this.driverProc = null;
    }
    if (this.tempProjectDir && existsSync(this.tempProjectDir)) {
      try {
        rmSync(this.tempProjectDir, { recursive: true, force: true });
      } catch (err) {
        console.warn(`[Harness] Failed to clean up temp dir ${this.tempProjectDir}: ${err.message}`);
      }
    }
  }
}
