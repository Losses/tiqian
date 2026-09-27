#!/usr/bin/env bash
# T1 gate: type-check the generated TS trees under the project strict
# profile (the boring tsconfig profile). Regenerates out/ts/tsconfig.json
# because out/ is a generated tree; the profile lives here, in version
# control. Usage: engine-haxe/tools/ts-gate.sh [bun|dart] (runtime unused
# today; tsc is run through bun x).
set -euo pipefail
TI_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
OUT="$TI_ROOT/engine-haxe/out/ts"
[ -d "$OUT/gen" ] || { echo "ts-gate: $OUT/gen missing; run the bundle driver gen ts first" >&2; exit 2; }
# The profile needs @types/bun for the `types: ["bun"]` entry. Without it
# tsc silently type-checks nothing that matters; fail loudly instead of
# reporting a zero-diagnostic read nobody ran.
if [ ! -d "$TI_ROOT/node_modules/@types/bun" ]; then
  echo "ts-gate: $TI_ROOT/node_modules/@types/bun missing; run 'bun install' at the tiqian root first" >&2
  exit 2
fi

cat > "$OUT/tsconfig.json" <<'JSON'
{
  "compilerOptions": {
    "target": "esnext",
    "module": "esnext",
    "moduleResolution": "bundler",
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "noImplicitOverride": true,
    "noFallthroughCasesInSwitch": true,
    "allowImportingTsExtensions": true,
    "erasableSyntaxOnly": true,
    "types": ["bun"],
    "noEmit": true,
    "skipLibCheck": true
  },
  "include": ["gen", "gen-tests"]
}
JSON

cd "$OUT"
# Pin the repo-local tsc (typescript 5.9.3): a fetched-latest tsc drifts the
# diagnostic set between runs (observed: 744 vs 745 with 9 TS2307 flipping),
# and the gate must keep one frozen reading basis.
TSC="$TI_ROOT/node_modules/.bin/tsc"
[ -x "$TSC" ] || { echo "ts-gate: $TSC missing; run 'bun install' at the tiqian root first" >&2; exit 2; }
# Bound the run: a wedged resolver must read as a failed gate, not a hang.
exec timeout 900 "$TSC" -p tsconfig.json
