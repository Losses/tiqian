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
exec bun x tsc -p tsconfig.json
