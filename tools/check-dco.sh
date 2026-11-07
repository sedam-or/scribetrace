#!/usr/bin/env bash
set -euo pipefail

base_sha="${1:?Usage: check-dco.sh BASE_SHA HEAD_SHA}"
head_sha="${2:?Usage: check-dco.sh BASE_SHA HEAD_SHA}"

mapfile -t commits < <(git rev-list --reverse --no-merges "${base_sha}..${head_sha}")
if ((${#commits[@]} == 0)); then
  echo "No non-merge commits found in the pull request range."
  exit 1
fi

failed=0
for commit in "${commits[@]}"; do
  subject="$(git show -s --format=%s "$commit")"
  if git show -s --format=%B "$commit" | grep -Eq '^Signed-off-by: .+ <[^<>[:space:]]+@[^<>[:space:]]+>$'; then
    echo "PASS $commit $subject"
  else
    echo "FAIL $commit $subject (missing Signed-off-by trailer)" >&2
    failed=1
  fi
done

exit "$failed"
