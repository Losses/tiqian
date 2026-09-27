#!/usr/bin/env bash
# Copies the protocol generation output (engine-haxe/out/protocol-ts, produced
# by `nix develop -c bash -c 'haxe engine-haxe/targets/protocol-ts.hxml'` from
# the worktree root) into this package, rewriting the generated ".ts" import
# specifiers to ".js" so the tree compiles under tsc -p tsconfig.build.json
# (module NodeNext emits ".js" relative specifiers).
set -eu
cd "$(git rev-parse --show-toplevel)"
SRC=engine-haxe/out/protocol-ts/gen
DST=platforms/web/server/core/src/protocol-gen
rm -rf "$DST"
mkdir -p "$DST/haxe/crypto" "$DST/org/tiqian/protocol"
cp "$SRC/runtime.ts" "$DST/runtime.ts"
cp "$SRC/haxe/crypto/Sha256.ts" "$DST/haxe/crypto/Sha256.ts"
for f in Canonical EncodeResult JsCoerce WireField WireValue; do
  cp "$SRC/org/tiqian/protocol/$f.ts" "$DST/org/tiqian/protocol/$f.ts"
done
grep -rl '\.ts"' "$DST" | while read -r file; do
  sed -i -E 's/\.ts"/\.js"/g' "$file"
done
echo "copied protocol generation into $DST"
