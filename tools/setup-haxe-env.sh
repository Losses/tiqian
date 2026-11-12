#!/usr/bin/env bash
# Stage the local data inputs that Haxe generation reads at compile time.
#
# One input lives outside git:
#   tools/unicode-data/            read by reflaxe.unicode.GraphemeData
#
# It has an in-repo source, so a fresh clone needs no prepared checkout:
#   .haxelib/boring/git/tools/unicode-data/  the pinned boring checkout
#
# Usage:
#   bash tools/setup-haxe-env.sh              # stage from the in-repo source
#   bash tools/setup-haxe-env.sh <checkout>   # stage from a prepared checkout
set -euo pipefail

TARGET="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
SOURCE="${1:-}"

# The pinned boring checkout carries the Unicode 17.0.0 data files.
UNICODE_SOURCE=".haxelib/boring/git/tools/unicode-data"

stage() {
  local rel="$1" src="$2"
  if [ -e "$TARGET/$rel" ] || [ -L "$TARGET/$rel" ]; then
    echo "  = $rel (already present)"
    return 0
  fi
  if [ ! -e "$src" ]; then
    echo "setup-haxe-env: missing source input: $src" >&2
    return 1
  fi
  mkdir -p "$(dirname "$TARGET/$rel")"
  cp -pR "$src" "$TARGET/$rel"
  echo "  + $rel (copied from $src)"
}

echo "setup-haxe-env: target $TARGET"

if [ -n "$SOURCE" ]; then
  SOURCE="$(cd "$SOURCE" && pwd -P)"
  echo "setup-haxe-env: source $SOURCE"
  stage tools/unicode-data "$SOURCE/tools/unicode-data"
else
  echo "setup-haxe-env: source $TARGET (in-repo)"
  stage tools/unicode-data "$TARGET/$UNICODE_SOURCE"
fi
