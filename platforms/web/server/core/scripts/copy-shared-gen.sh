#!/usr/bin/env bash
# Copies the shared generation output (engine-haxe/out/protocol-ts, produced
# by `nix develop -c bash -c 'haxe engine-haxe/targets/protocol-ts.hxml'` from
# the worktree root) for the html/snapshot shared modules into this package,
# rewriting the generated ".ts" import specifiers to ".js" so the tree compiles
# under tsc -p tsconfig.build.json (module NodeNext emits ".js" relative
# specifiers).
set -eu
cd "$(git rev-parse --show-toplevel)"
SRC=engine-haxe/out/protocol-ts/gen
DST=platforms/web/server/core/src/shared-gen
rm -rf "$DST"
mkdir -p "$DST/org/tiqian/shared/html" "$DST/org/tiqian/shared/snapshot"
cp "$SRC/runtime.ts" "$DST/runtime.ts"
for f in DomAttribute DomNode HtmlNameCase DomParser HtmlError HtmlParseException CompoundSelector HtmlAttributeInsertion HtmlOpeningTag HtmlScan; do
  cp "$SRC/org/tiqian/shared/html/$f.ts" "$DST/org/tiqian/shared/html/$f.ts"
done
cp "$SRC/org/tiqian/shared/snapshot/SnapshotSource.ts" "$DST/org/tiqian/shared/snapshot/SnapshotSource.ts"
grep -rl '\.ts"' "$DST" | while read -r file; do
  sed -i -E 's/\.ts"/\.js"/g' "$file"
done
echo "copied shared generation into $DST"
