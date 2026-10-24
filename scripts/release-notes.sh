#!/usr/bin/env bash
# Prints the body of the CHANGELOG.md section for VERSION (without its heading).
# Usage: scripts/release-notes.sh 0.3.0
set -euo pipefail

version="${1:?usage: release-notes.sh VERSION}"
changelog="$(dirname "$0")/../CHANGELOG.md"

awk -v v="$version" '
	$0 ~ "^## \\[" v "\\]" { on = 1; next }
	on && /^## \[/ { exit }
	on && /^\[[^]]+\]: / { exit }
	on { print }
' "$changelog" | sed -e '/./,$!d' | sed -e ':a' -e '/^\n*$/{$d;N;ba' -e '}'
