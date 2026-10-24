#!/usr/bin/env bash
# Sched2-Verify 测量脚本：同机 relayout/TBT/stall 三指标，N 跑（默认 5）。
# 计数点：relayoutReadyTotal = Σ 每个 tiqian-prose 根的 "tiqian:relayout-ready" DOM CustomEvent。
# 前置：demo/web 已 npm install + tsc -b（pretest）；在 demo/web 目录下执行。
# 用法：
#   export CHROME_BIN="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
#   bash sched2-verify-measure.sh 5
set -euo pipefail
RUNS="${1:-5}"
export PATH="/opt/homebrew/bin:${PATH:-/usr/bin:/bin}"
export CHROME_BIN="${CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
echo "Sched2-Verify: ${RUNS} runs, CHROME_BIN=${CHROME_BIN}"
for i in $(seq 1 "$RUNS"); do
  echo "=== RUN ${i} ==="
  node --test --test-concurrency=1 tests/drag-responsiveness-metrics.test.js 2>&1 \
    | grep -E 'Relayout-ready Total|Total Blocking Time|Max Frame Stall Duration|Max Event Loop Latency'
done
