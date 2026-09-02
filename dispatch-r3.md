# r3 分诊表（代码改动前建立）

类别：a=已还原；b=本轮还原；c=不合格（规格条款）；d=等 spec 39；e=等 macros/05。
本表按 Kotlin 原文闭集/扩展成员枚举；Haxe 行号在执行还原后以最终文件行号复核。

| Haxe 文件:行号 | Kotlin 原文:行号 | 类别 | 本轮动作 |
|---|---:|---|---|
| LayoutQueries.hx:99 | LayoutQueries.kt:128 | b | forEach（待还原为 rubyDecisions.forEach） |
| LayoutQueries.hx:105 | LayoutQueries.kt:129 | b | forEach（待还原为 bopomofoDecisions.forEach） |
| LayoutQueries.hx:— | LayoutQueries.kt:136 | c macros/01 position+具名引用 | 保持手工循环；嵌套 Map/keys.sorted 与 ::append |
| LayoutQueries.hx:216 | LayoutQueries.kt:206 | a | firstOrNull（r2，Array 直接位） |
| LayoutQueries.hx:230 | LayoutQueries.kt:221 | a | mapNotNull（r2，Array 直接位） |
| LayoutQueries.hx:291 | LayoutQueries.kt:316 | a | filter（r2） |
| LayoutQueries.hx:302 | LayoutQueries.kt:334 | a | filter（r2） |
| LayoutQueries.hx:— | LayoutQueries.kt:339 | c macros/01 position | groupBy 链中间位，保持循环 |
| LayoutQueries.hx:— | LayoutQueries.kt:341 | c macros/01 position | asSequence/flatMap 链中间位，保持循环 |
| LayoutQueries.hx:— | LayoutQueries.kt:342 | c macros/01 position | groupBy 链中间位，保持循环 |
| LayoutQueries.hx:— | LayoutQueries.kt:345 | c macros/01 position | map 链返回嵌套体，保持循环 |
| LayoutQueries.hx:— | LayoutQueries.kt:346 | c macros/01 position | filter 嵌套在 map 体内，保持循环 |
| LayoutQueries.hx:— | LayoutQueries.kt:414 | c macros/01 String/具名引用 | firstOrNull 具名方法引用 |
| LayoutQueries.hx:— | LayoutQueries.kt:417 | c macros/01 String/具名引用 | firstOrNull 具名方法引用 |
| LayoutQueries.hx:— | LayoutQueries.kt:434 | b | covered.forEach（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:460 | b | metrics.filter（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:463 | b | firstOrNull（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:464 | b | firstOrNull（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:490 | b | segments.map（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:493 | c macros/01 position | mapNotNull 链中间位，保持循环 |
| LayoutQueries.hx:— | LayoutQueries.kt:494 | b | firstOrNull（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:496 | c macros/01 position | mapNotNull 链中间位，保持循环 |
| LayoutQueries.hx:— | LayoutQueries.kt:593 | b | clusters.firstOrNull（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:623 | b | positioned.firstOrNull（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:638 | c macros/01 position | firstOrNull 嵌入 let |
| LayoutQueries.hx:— | LayoutQueries.kt:664 | c macros/01 position | firstOrNull 嵌入 let/块 |
| LayoutQueries.hx:— | LayoutQueries.kt:714 | b | positioned.firstOrNull（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:751 | a | filter（r2 已还原形态） |
| LayoutQueries.hx:— | LayoutQueries.kt:752 | a | associate（r2 已还原形态） |
| LayoutQueries.hx:— | LayoutQueries.kt:757 | a | filter（r2 已还原形态） |
| LayoutQueries.hx:— | LayoutQueries.kt:758 | a | associate（r2 已还原形态） |
| LayoutQueries.hx:— | LayoutQueries.kt:760 | c macros/01 position | asSequence/flatMap 链中间位 |
| LayoutQueries.hx:— | LayoutQueries.kt:761 | c macros/01 position | groupBy 链中间位 |
| LayoutQueries.hx:— | LayoutQueries.kt:764 | e macros/05 | mapIndexed 双参数，落地前不还原 |
| LayoutQueries.hx:— | LayoutQueries.kt:857 | a | filter（r2 已还原形态） |
| LayoutQueries.hx:— | LayoutQueries.kt:858 | a | associate（r2 已还原形态） |
| LayoutQueries.hx:— | LayoutQueries.kt:869 | b | rubies.filter（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:874 | b | positioned.map（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:884 | b | indices.filter（方法位数组推导式，待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:890 | b | baseIndices.map（待还原） |
| LayoutQueries.hx:— | LayoutQueries.kt:920 | e macros/05 | mapIndexed 双参数，落地前不还原 |

其余 16 个对象文件（Justifier、LineRepair、PunctuationGeometryStage、ProgressiveBreakDecisions、LineBreaker、UnicodePunctuationBoundaryResolver、PunctuationModel、QuotePairAnalyzer、KinsokuRule、FontPolicy、EastAsianSpacing、ContextualDashEllipsisRoleResolver、ClusterRoleResolution、TextShaper、BopomofoReading、ContextualQuoteRoleResolver）的全部 Kotlin 枚举站点均登记为 **c macros/01（本轮不动）**，其中 `forEachIndexed`/`mapIndexed`/`withIndex` 明确登记为 **e macros/05**；构造调用登记为 **d spec 39**。本轮动作：保持现状，不修改这 16 个文件。
