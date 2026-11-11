#!/usr/bin/env bash
# Stage the local data inputs that Haxe generation reads at compile time.
#
# Two inputs live outside git:
#   engine-haxe/baseline-goldens/  read by GoldenDataMacros.init()
#   tools/unicode-data/            read by reflaxe.unicode.GraphemeData
#
# Both have an in-repo source, so a fresh clone needs no prepared checkout:
#   engine/src/jvmTest/resources/golden/     tracked recordings of the Kotlin engine
#   .haxelib/boring/git/tools/unicode-data/  the pinned boring checkout
#
# Usage:
#   bash tools/setup-haxe-env.sh              # stage from the in-repo sources
#   bash tools/setup-haxe-env.sh <checkout>   # stage from a prepared checkout
set -euo pipefail

TARGET="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
SOURCE="${1:-}"

# The tracked recordings of the Kotlin engine. Keep this path in one place so
# moving the golden library only has to change this line.
GOLDEN_SOURCE="engine/src/jvmTest/resources/golden"
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
  stage engine-haxe/baseline-goldens "$SOURCE/engine-haxe/baseline-goldens"
  stage tools/unicode-data "$SOURCE/tools/unicode-data"
else
  echo "setup-haxe-env: source $TARGET (in-repo)"
  stage engine-haxe/baseline-goldens "$TARGET/$GOLDEN_SOURCE"
  stage tools/unicode-data "$TARGET/$UNICODE_SOURCE"
fi
