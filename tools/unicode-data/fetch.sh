#!/usr/bin/env bash
# Fetches the pinned Unicode data files required by the engine-haxe build.
# The pinned version lives in packages/compiler/reflaxe/unicode/GraphemeData.hx
# in the boring compiler repository; the URL list below mirrors it. Data files
# are build inputs and stay untracked; re-run after a version bump.
set -euo pipefail
VERSION="${1:-17.0.0}"
DIR="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$DIR"
FILES=(
  "GraphemeBreakProperty|https://unicode.org/Public/${VERSION}/ucd/auxiliary/GraphemeBreakProperty.txt"
  "emoji-data|https://unicode.org/Public/${VERSION}/ucd/emoji/emoji-data.txt"
  "DerivedCoreProperties|https://unicode.org/Public/${VERSION}/ucd/DerivedCoreProperties.txt"
  "GraphemeBreakTest|https://unicode.org/Public/${VERSION}/ucd/auxiliary/GraphemeBreakTest.txt"
)
for entry in "${FILES[@]}"; do
  name="${entry%%|*}"; url="${entry#*|}"
  out="$DIR/${name}-${VERSION}.txt"
  echo "fetching ${name}-${VERSION}"
  curl -fsSL -o "$out" "$url"
  [ -s "$out" ] || { echo "empty download: $out" >&2; exit 1; }
  head -1 "$out" | grep -q '^#' || { echo "missing license header: $out" >&2; exit 1; }
done
echo "unicode ${VERSION} data ready in $DIR"
