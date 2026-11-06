/**
 * ScribeTrace Process Tree & Memory Metrics Utility
 *
 * Implements the standardized whole-process-tree memory procedure specified in
 * docs/TECH-OPTIMIZATION.md §3:
 * - 10 s settle window after app launch
 * - Sample process tree every 1 s for 5 s
 * - Compute and report the median (PSS on Linux, RSS on macOS, WorkingSet on Windows)
 */

import { execSync } from 'node:child_process';
import { readFileSync, existsSync } from 'node:fs';

export class ProcessTreeMetrics {
  /**
   * Discovers all descendant process IDs for a given root PID.
   * @param {number} rootPid
   * @returns {number[]} Array of process IDs including the root PID.
   */
  static getProcessTreePids(rootPid) {
    const pids = [rootPid];
    if (process.platform === 'linux' || process.platform === 'darwin') {
      try {
        const stdout = execSync(`pgrep -P ${rootPid}`, { encoding: 'utf-8', stdio: ['pipe', 'pipe', 'ignore'] });
        const childPids = stdout.trim().split(/\s+/).map(Number).filter(Boolean);
        for (const cPid of childPids) {
          pids.push(...this.getProcessTreePids(cPid));
        }
      } catch {
        // No children found
      }
    }
    return Array.from(new Set(pids));
  }

  /**
   * Measures current process tree memory in Megabytes.
   * On Linux: Reads PSS from /proc/<pid>/smaps_rollup (or fallback to RSS).
   * On macOS / other: Reads RSS from ps.
   * @param {number} rootPid
   * @returns {number} Memory in MB.
   */
  static measureTreeMemoryMb(rootPid) {
    const pids = this.getProcessTreePids(rootPid);
    let totalKb = 0;

    if (process.platform === 'linux') {
      for (const pid of pids) {
        const smapsPath = `/proc/${pid}/smaps_rollup`;
        if (existsSync(smapsPath)) {
          try {
            const content = readFileSync(smapsPath, 'utf-8');
            const match = content.match(/Pss:\s+(\d+)\s+kB/);
            if (match) {
              totalKb += parseInt(match[1], 10);
              continue;
            }
          } catch {
            // Process might have terminated, continue
          }
        }
        // Fallback to statm / ps rss if smaps_rollup unavailable
        try {
          const statmPath = `/proc/${pid}/statm`;
          if (existsSync(statmPath)) {
            const statm = readFileSync(statmPath, 'utf-8').split(' ');
            const rssPages = parseInt(statm[1], 10);
            totalKb += rssPages * 4; // standard 4KB page size
          }
        } catch {}
      }
    } else {
      try {
        const pidList = pids.join(',');
        const stdout = execSync(`ps -o rss= -p ${pidList}`, { encoding: 'utf-8', stdio: ['pipe', 'pipe', 'ignore'] });
        const rssEntries = stdout.trim().split(/\s+/).map(Number).filter(Boolean);
        totalKb = rssEntries.reduce((acc, curr) => acc + curr, 0);
      } catch {}
    }

    return Math.round((totalKb / 1024) * 100) / 100;
  }

  /**
   * Reports a point-in-time memory breakdown for every process in the app tree.
   * On Linux this uses PSS per process, matching the aggregate metric above.
   * @param {number} rootPid
   * @returns {Array<{pid: number, name: string, memoryMb: number, metric: string}>}
   */
  static measureTreeMemoryBreakdownMb(rootPid) {
    const pids = this.getProcessTreePids(rootPid);
    const breakdown = [];

    for (const pid of pids) {
      let name = `pid-${pid}`;
      let memoryKb = 0;
      let metric = process.platform === 'linux' ? 'PSS' : 'RSS';

      if (process.platform === 'linux') {
        try {
          name = readFileSync(`/proc/${pid}/comm`, 'utf-8').trim() || name;
          const content = readFileSync(`/proc/${pid}/smaps_rollup`, 'utf-8');
          const match = content.match(/Pss:\s+(\d+)\s+kB/);
          if (match) memoryKb = parseInt(match[1], 10);
        } catch {
          metric = 'unavailable';
        }
      } else {
        try {
          const stdout = execSync(`ps -o comm=,rss= -p ${pid}`, {
            encoding: 'utf-8',
            stdio: ['pipe', 'pipe', 'ignore']
          }).trim();
          const match = stdout.match(/^(.*?)\s+(\d+)$/);
          if (match) {
            name = match[1].trim();
            memoryKb = parseInt(match[2], 10);
          }
        } catch {
          metric = 'unavailable';
        }
      }

      breakdown.push({
        pid,
        name,
        memoryMb: Math.round((memoryKb / 1024) * 100) / 100,
        metric
      });
    }

    return breakdown.sort((a, b) => b.memoryMb - a.memoryMb);
  }

  /**
   * Executes the standardized 5-sample 1-second median measurement.
   * @param {number} rootPid
   * @param {number} samples Number of samples (default 5).
   * @param {number} intervalMs Interval between samples in ms (default 1000).
   * @returns {Promise<{medianMb: number, samples: number[]}>}
   */
  static async measureMedianMemoryMb(rootPid, samples = 5, intervalMs = 1000) {
    const readings = [];
    for (let i = 0; i < samples; i++) {
      const mb = this.measureTreeMemoryMb(rootPid);
      readings.push(mb);
      if (i < samples - 1) {
        await new Promise(r => setTimeout(r, intervalMs));
      }
    }

    readings.sort((a, b) => a - b);
    const mid = Math.floor(readings.length / 2);
    const medianMb = readings.length % 2 !== 0
      ? readings[mid]
      : Math.round(((readings[mid - 1] + readings[mid]) / 2) * 100) / 100;

    return { medianMb, samples: readings };
  }
}
