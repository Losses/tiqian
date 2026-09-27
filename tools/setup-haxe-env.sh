#!/usr/bin/env bash
# Copy the local data inputs used by Haxe generation into this checkout.
# The source must be a tiqian checkout whose inputs have been prepared and
# reviewed. Unicode files come from Unicode 17.0.0; baseline-goldens are
# recorded outputs of the original Kotlin engine. The flake provides the
# matching boring compiler and driver; this script only supplies local data.
#
# Usage: bash tools/setup-haxe-env.sh [path/to/prepared/tiqian]
set -euo pipefail

TARGET="$(git rev-parse --show-toplevel)"
SOURCE="${1:-}"

if [ -z "$SOURCE" ]; then
  SOURCE="$(git worktree list --porcelain | sed -n 's/^worktree //p' | head -1)"
fi

if [ -z "$SOURCE" ] || [ ! -d "$SOURCE" ]; then
  echo "Usage: bash tools/setup-haxe-env.sh [path/to/prepared/tiqian]" >&2
  exit 2
fi

SOURCE="$(cd "$SOURCE" && pwd -P)"

required=(
  engine-haxe/baseline-goldens/layout-dumps
  engine-haxe/baseline-goldens/layout-dumps-recorded
  engine-haxe/baseline-goldens/test-traces
  engine-haxe/baseline-goldens/shaping-evidence.json
  tools/unicode-data/DerivedCoreProperties-17.0.0.txt
  tools/unicode-data/GraphemeBreakProperty-17.0.0.txt
  tools/unicode-data/GraphemeBreakTest-17.0.0.txt
  tools/unicode-data/emoji-data-17.0.0.txt
)

for rel in "${required[@]}"; do
  if [ ! -e "$SOURCE/$rel" ]; then
    echo "setup-haxe-env: missing source input: $SOURCE/$rel" >&2
    exit 2
  fi
done

echo "setup-haxe-env: source $SOURCE"
echo "setup-haxe-env: target $TARGET"

for rel in engine-haxe/baseline-goldens tools/unicode-data; do
  if [ -e "$TARGET/$rel" ] || [ -L "$TARGET/$rel" ]; then
    echo "  = $rel (already present)"
    continue
  fi
  mkdir -p "$(dirname "$TARGET/$rel")"
  cp -pR "$SOURCE/$rel" "$TARGET/$rel"
  echo "  + $rel (copied from $SOURCE/$rel)"
done
