# Sched2-Verify 测量读数（Mac 同机，5 跑中位）

## 环境钉
- 机器：Apple M1 (arm64)，macOS 26.5.1 (Build 25F80)，外置卷 /Volumes/DATA CENTER
- 浏览器：Google Chrome 155.0.8059.85（headless=new，CDP）
- Node：v23.11.0（Homebrew /opt/homebrew/bin）
- commit 对：haxe = main 669bba04f995a560e9fb7fb731245c179a694ffb；legacy = e4af38a4b913ebbaa2b6310848c010e96f5de52d
- 语料同源：drag-responsiveness-metrics.test.ts 与 index.html 双侧 md5 一致（6cc14b0a…，byte-identical）

## 三指标（relayoutReadyTotal / totalBlockingTime / maxFrameDuration）

| side | relayout 原始 5 跑 | 中位 | TBT 原始 5 跑 | 中位 | stall 原始 5 跑 | 中位 |
|---|---|---|---|---|---|---|
| haxe main | 116,148,148,159,230 | 148 | 9108,9450,9703,9831,9887 | 9703 | 150.0,150.1,166.6,166.7,166.7 | 166.6 |
| legacy | 76,79,81,81,82 | 81 | 328,342,364,370,410 | 364 | 83.3,99.9,100.0,100.0,100.0 | 100.0 |

## 比值（haxe / legacy）
- relayout：148 / 81 = 1.83x
- TBT：9703 / 364 = 26.7x
- stall：166.6 / 100.0 = 1.67x

## 结论
预注册判据 relayout≤2x 且 TBT≤1.5x：
- relayout 1.83x≤2x ✓ 通过（残余 9x 计数口径消除）
- TBT 26.7x>1.5x ✗ 未通过
整体【未达标】。

## 下一层归因
TBT 26.7x ≈ relayout 计数 1.83x × 单次阻塞 14.6x（haxe 65.6ms/relayout vs legacy 4.5ms）。
主因 = 每 relayout 同步阻塞成本（haxe ~50-100ms 越过 50ms longtask 阈值 → 184 longtask；legacy ~17ms → 39），
非 relayout 未合并；调度时机/signature 残留仅贡献 67 次额外（1.83x 内）。
