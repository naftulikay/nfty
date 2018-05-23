#!/usr/bin/env bash

set -e

HOOKS_BASE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOOK_TYPE="$(basename "${BASH_SOURCE[0]}")"
HOOKS_DIR="${HOOKS_BASE}/${HOOK_TYPE}.d"

mkdir -p "${HOOKS_DIR}"

find "${HOOKS_DIR}" -maxdepth 1 -type f | sort | while read hook ; do
  # execute each hook in a sorted order
  if [ ! -x "${hook}" ]; then
    continue
  fi

  if ! ${hook} $@ ; then
    rc="$?"
    echo "[ERROR] git:${HOOK_TYPE} - hook $(basename "${HOOKS_DIR}")/$(basename "${hook}") failed: $rc" >&2
    exit $rc
  fi
done
