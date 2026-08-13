#!/usr/bin/env bash
# scripts/record.sh — record an asciinema demo cast for the README.
#
# Usage:
#   ./scripts/record.sh                 # writes assets/screenshots/demo.cast
#   ./scripts/record.sh /tmp/out.cast   # custom output path
#
# Requirements:
#   - A built `rustwhy` binary on $PATH (run `cargo build --release` first)
#   - `asciinema` (https://asciinema.org/)
#
# The recording is intentionally scripted: a fixed welcome, a handful of
# representative commands, then a clean exit. The README references this
# cast in its hero block; regenerate it whenever the output changes.

set -euo pipefail

OUT="${1:-assets/screenshots/demo.cast}"

if ! command -v asciinema >/dev/null 2>&1; then
  echo "error: asciinema is not installed" >&2
  echo "  install: sudo apt install asciinema  OR  sudo pacman -S asciinema" >&2
  exit 1
fi

if ! command -v rustwhy >/dev/null 2>&1; then
  echo "warning: rustwhy not on \$PATH — using 'cargo run --' instead" >&2
  RUN=(cargo run --quiet --)
else
  RUN=(rustwhy)
fi

mkdir -p "$(dirname "$OUT")"

asciinema rec \
  --cols 100 \
  --rows 30 \
  --command "bash -c '
    set -e
    echo \$'\''\\033[1;36m🔍 rustwhy — unified Linux diagnostics\\033[0m'
    sleep 1
    echo
    echo \$'\''\\033[1m# Quick system overview\\033[0m'
    ${RUN[*]} cpu --no-color
    sleep 1
    echo
    echo \$'\''\\033[1m# Why is my disk full?\\033[0m'
    ${RUN[*]} disk --depth 2 --no-color
    sleep 1
    echo
    echo \$'\''\\033[1m# Run every applicable module at once\\033[0m'
    ${RUN[*]} all --no-color
    sleep 1
    echo
    echo \$'\''\\033[1;32m✓ done — run \`rustwhy <module> --help\` for all options\\033[0m'
  '" \
  "$OUT"

echo
echo "wrote $OUT"
echo "next: agg $OUT $(dirname "$OUT")/demo.gif"
