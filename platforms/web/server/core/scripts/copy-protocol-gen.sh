#!/usr/bin/env bash
# Copies the protocol generation output (engine-haxe/out/protocol-ts, produced
# by `nix develop -c bash -c 'haxe engine-haxe/targets/protocol-ts.hxml'` from
# the worktree root) into this package, rewriting the generated ".ts" import
# specifiers to ".js" so the tree compiles under tsc -p tsconfig.build.json
# (module NodeNext emits ".js" relative specifiers).
#
# The shared HTML leaf functions (research §5.4) land in a separate directory
# (src/shared-gen) from src/protocol-gen so the two generated surfaces stay
# separated. HtmlScan.ts imports runtime.ts four levels up, so runtime.ts is
# copied into that directory too.
set -eu
cd "$(git rev-parse --show-toplevel)"
SRC=engine-haxe/out/protocol-ts/gen
DST=platforms/web/server/core/src/protocol-gen
SHARED_DST=platforms/web/server/core/src/shared-gen
rm -rf "$DST" "$SHARED_DST"
mkdir -p "$DST/haxe/crypto" "$DST/org/tiqian/protocol" "$SHARED_DST/org/tiqian/shared/html" "$SHARED_DST/org/tiqian/shared"
cp "$SRC/runtime.ts" "$DST/runtime.ts"
# Revision is flattened next to the generated tree root: the shared
# snapshot modules import it as "./protocol-gen/Revision.js".
cp "$SRC/org/tiqian/protocol/Revision.ts" "$DST/Revision.ts"
cp "$SRC/runtime.ts" "$SHARED_DST/runtime.ts"
cp "$SRC/haxe/crypto/Sha256.ts" "$DST/haxe/crypto/Sha256.ts"
for f in Canonical DecorationInput EncodeResult FontFaceRecord InlineBoxInput InlineObjectInput JsCoerce LayoutAttribute LayoutFloatShape LayoutTree LayoutVertex LineBreakSpanInput NamedError NamedErrorNames ParagraphRequest ParagraphRequestChecks ParagraphRequestException ParagraphRequestTestSupport TextSpanInput WireField WireValue; do
  cp "$SRC/org/tiqian/protocol/$f.ts" "$DST/org/tiqian/protocol/$f.ts"
done
for f in HtmlAttributeInsertion HtmlError HtmlOpeningTag HtmlParseException HtmlScan; do
  cp "$SRC/org/tiqian/shared/html/$f.ts" "$SHARED_DST/org/tiqian/shared/html/$f.ts"
done
for f in SnapshotValueStyle ValueStyleInterner; do
  cp "$SRC/org/tiqian/shared/$f.ts" "$SHARED_DST/org/tiqian/shared/$f.ts"
done
grep -rl '\.ts"' "$DST" "$SHARED_DST" | while read -r file; do
  sed -i -E 's/\.ts"/\.js"/g' "$file"
done
echo "copied protocol generation into $DST and $SHARED_DST"
