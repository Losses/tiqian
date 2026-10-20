#!/usr/bin/env bash
# tq-vendor-prepared-dom.sh — 把 protocol-ts 束的 org.tiqian.shared.prepared_dom 产物
# 挂载到 platforms/web/client/core/src/sampler/snapshot/prepared-dom-gen。
#
# 驱动产出：engine-haxe/out/protocol-ts/gen（先跑 boring gen protocol-ts --project boring.json）。
# 三步约定（与 scripts/cutover/tq-vendor-check.sh 登记表 prepared-dom-ts-client 行的判定一致）：
#   1) 相对 .ts 说明符改写成 .js（NodeNext 包内的 ESM 解析）；
#   2) 仅导入类型的 import { X } 写成 import type { X }
#      （client/core 开了 verbatimModuleSyntax，类型导入必须写成 type-only）；
#   3) 测试支撑类与测试用解析器不进挂载（各 *TestSupport、DeclarationLengthStyleClass、
#      FixedEmphasisDotColor、ClusterThreeDotColor、FixedStyleClassResolver、PrefixedStyleClassResolver）。
# 用法：bash scripts/cutover/tq-vendor-prepared-dom.sh [tiqian 根，默认当前目录]
set -euo pipefail

ROOT="$1"
if [ -z "$ROOT" ]; then ROOT="."; fi
GEN="$ROOT/engine-haxe/out/protocol-ts/gen"
DST="$ROOT/platforms/web/client/core/src/sampler/snapshot/prepared-dom-gen"
[ -d "$GEN" ] || { echo "找不到 $GEN：先跑 boring gen protocol-ts --project boring.json" >&2; exit 2; }

rm -rf "$DST"
mkdir -p "$DST/org/tiqian/shared/prepared_dom"
for f in "$GEN"/org/tiqian/shared/prepared_dom/*.ts; do
  case "$(basename "$f")" in
    EvidenceTestSupport.ts|PreparedArtifactTestSupport.ts|PreparedDomTestSupport.ts) continue;;
    DeclarationLengthStyleClass.ts|FixedEmphasisDotColor.ts|ClusterThreeDotColor.ts) continue;;
    FixedStyleClassResolver.ts|PrefixedStyleClassResolver.ts) continue;;
  esac
  cp "$f" "$DST/org/tiqian/shared/prepared_dom/"
done
cp "$GEN/runtime.ts" "$DST/runtime.ts"

find "$DST" -name "*.ts" -print0 | xargs -0 sed -i -E 's|(")(\.\.?/[^"]*)\.ts\1|\1\2.js\1|g'

python3 - "$DST" <<'PY'
import pathlib, re, sys
mount = pathlib.Path(sys.argv[1])
exports = {}
for p in mount.rglob("*.ts"):
    s = p.read_text()
    kinds = {}
    for m in re.finditer(r"^export\s+(?:interface|type)\s+(\w+)", s, re.M):
        kinds.setdefault(m.group(1), "type")
    for m in re.finditer(r"^export\s+(?:const|function|class|let|var|enum)\s+(\w+)", s, re.M):
        kinds[m.group(1)] = "value"
    exports[p.resolve()] = kinds
IMPORT_RE = re.compile(r'^import \{ ([^}]+) \} from "([^"]+)";$', re.M)
rewritten = 0
for p in mount.rglob("*.ts"):
    s = p.read_text()
    def repl(m):
        global rewritten
        names = [n.strip() for n in m.group(1).split(",") if n.strip()]
        spec = m.group(2)
        target_name = spec[:-3] + ".ts" if spec.endswith(".js") else spec
        kinds = exports.get((p.parent / target_name).resolve())
        if not kinds:
            return m.group(0)
        if all(kinds.get(n) == "type" for n in names):
            rewritten += 1
            return "import type { " + m.group(1) + ' } from "' + spec + '";'
        return m.group(0)
    out = IMPORT_RE.sub(repl, s)
    if out != s:
        p.write_text(out)
print("import type rewritten:", rewritten)
PY

echo "vendored prepared_dom -> $DST"
