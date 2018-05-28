#!/usr/bin/env bash

if ! which git-lfs &>/dev/null ; then
  echo "ERROR [$0] git-lfs is not installed." >&2
  exit 1
fi

git-lfs post-commit "$@"
