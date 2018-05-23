#!/usr/bin/env bash

set -e

git branch --merged | \
    # exclude the current branch
    grep -Pv '^\*' | \
    # strip whitespace
    awk -F ' ' '{print $1;}' | \
    # exclude branch names like master, develop, and production
    grep -Pv '\b(master|develop|production)\b' | \
    # delete the found merged branches
    xargs -r git branch -d

