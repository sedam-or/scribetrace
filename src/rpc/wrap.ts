import type { ZoteroErrorCode } from "./types.ts";
import { ZoteroError } from "./types.ts";

const KNOWN_CODES: ReadonlySet<string> = new Set([
  "refused",
  "forbidden",
  "timeout",
  "version",
  "too_large",
  "bad_answer",
  "internal",
]);

/**
 * Maps any IPC rejection to a typed ZoteroError (SEC-39). Only a known `{ code }` passes through;
 * plain strings, Tauri's own rejections and unknown codes become "internal", so no raw text from
 * the OS, Tauri or Zotero ever reaches the UI.
 */
export function wrap<T>(promise: Promise<T>): Promise<T> {
  return promise.catch((err: unknown) => {
    if (err && typeof err === "object" && "code" in err) {
      const code = String((err as { code: unknown }).code);
      if (KNOWN_CODES.has(code)) throw new ZoteroError(code as ZoteroErrorCode);
    }
    throw new ZoteroError("internal");
  });
}
