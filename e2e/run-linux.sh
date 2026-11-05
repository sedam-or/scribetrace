#!/usr/bin/env bash
set -euo pipefail

# ScribeTrace Linux E2E Real Window Runner
# Usage: ./e2e/run-linux.sh [--release | --debug]

PROFILE="release"
for arg in "$@"; do
  if [ "$arg" == "--debug" ]; then
    PROFILE="debug"
  fi
done

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "=== ScribeTrace Linux End-to-End Test Runner ==="
echo "Working directory: $ROOT_DIR"
echo "Build Profile: $PROFILE"

# 1. Locate or verify Tauri binary
BIN_PATH=""
if [ -f "$ROOT_DIR/src-tauri/target/$PROFILE/scribetrace" ]; then
  BIN_PATH="$ROOT_DIR/src-tauri/target/$PROFILE/scribetrace"
elif [ -f "$ROOT_DIR/target/$PROFILE/scribetrace" ]; then
  BIN_PATH="$ROOT_DIR/target/$PROFILE/scribetrace"
fi

if [ -n "$BIN_PATH" ]; then
  echo "Found ScribeTrace binary: $BIN_PATH"
  export SCRIBETRACE_BIN="$BIN_PATH"
else
  echo "Notice: Real ScribeTrace binary not found in target/$PROFILE."
  echo "Running harness dry-run/readiness check."
fi

# 2. Check for X11 Display or wrap with xvfb-run
if [ -z "${DISPLAY:-}" ]; then
  echo "No DISPLAY environment variable detected. Checking for xvfb-run..."
  if command -v xvfb-run >/dev/null 2>&1; then
    echo "Executing under virtual display via xvfb-run..."
    exec xvfb-run --auto-servernum --server-args="-screen 0 1280x1024x24" node "$ROOT_DIR/e2e/test_walking_skeleton.mjs"
  else
    echo "Warning: Neither DISPLAY nor xvfb-run was found on this host."
    echo "Attempting direct Node runner execution..."
    exec node "$ROOT_DIR/e2e/test_walking_skeleton.mjs"
  fi
else
  echo "Active DISPLAY found ($DISPLAY). Running directly..."
  exec node "$ROOT_DIR/e2e/test_walking_skeleton.mjs"
fi
