#!/usr/bin/env bash
# Copies the shared-layer generation output into the client core sampler package.
#
#   SRC: engine-haxe/out/shared-ts/gen        (produced by "boring gen shared-ts")
#   DST: platforms/web/client/core/src/sampler
#
# The generated TypeScript rewrites ".ts" import specifiers to ".js" so the tree
# compiles under tsc -p tsconfig.json (module NodeNext). Unlike
# copy-protocol-gen.sh this script resolves the worktree root from its own
# location (five levels above scripts/) and never shells out to git.
set -eu
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
cd "$ROOT"

SRC=engine-haxe/out/shared-ts/gen
DST=platforms/web/client/core/src/sampler

rm -rf "$DST/org/tiqian/shared/fontface"
mkdir -p "$DST/org/tiqian/shared/fontface"
cp "$SRC/runtime.ts" "$DST/runtime.ts"
for name in FontFaceBoundaries FontFaceBoundaryException; do
  cp "$SRC/org/tiqian/shared/fontface/$name.ts" "$DST/org/tiqian/shared/fontface/$name.ts"
done
grep -rl '\.ts"' "$DST/org/tiqian/shared/fontface" | while read -r file; do
  sed -i -E 's/\.ts"/\.js"/g' "$file"
done
echo "copied shared fontface generation into $DST"
