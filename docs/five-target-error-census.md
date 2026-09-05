# 五目标生成代码编译错误普查与修复追踪

本文记录 engine-haxe 生成代码在五种输出语言（Kotlin、TypeScript、Rust、Swift、
Dart）下的编译错误，作为跨目标行为对齐（见
[cross-target-alignment.md](cross-target-alignment.md)）的修复计划与进度追踪。
普查对象是 engine-haxe/out/ 下由 boring 从 Haxe 源码翻译出的目标语言代码目录。
原生 `engine` 模块的 `./gradlew :engine:jvmTest` 无失败，不在本文范围内。

当前状态：Kotlin 的 f32 与 f64 两个目录已完成全量普查（第 3 节）；TypeScript、
Rust、Swift、Dart 四个目录的生成当前在第一处尚未实现生成规则的 Haxe 构造上
中止（第 4 节），逐个补齐这些生成规则是四语言普查的前置条件。

## 1 更新规则

- 每个修复项只有一个复选框。满足以下四条后在复选框勾选，并在文末进度记录表
  追加一行：修复已合入 boring main（写提交号）或 tiqian（写提交号）；vendored
  副本已推进并重新生成相关目录；该错误种类在相关目录的计数都是 0；boring 验收
  链全部通过。
- 一次修复只允许勾选一次对应的复选框；不允许一次勾选多项，也不允许提前勾选。
- 计数必须来自本文「测量配方」一节的命令输出，不允许凭印象填写。
- 新暴露的错误种类在分桶表加新行，编号顺延，不允许并入既有行。

## 2 测量配方

### 2.1 Kotlin（f32 与 f64）

```shell
# 前置：vendored 副本指向要测的 boring 提交
cd /home/losses/Development/tiqian/.haxelib/boring/git && git fetch origin && git checkout <提交号>

# 从 tiqian 仓库根目录重新生成两个 Kotlin 目录
nix develop -c bash -c 'haxe engine-haxe/core-kotlin.hxml'      # f32
nix develop -c bash -c 'haxe engine-haxe/core-kotlin-f64.hxml'  # f64

# 编译普查。kotlinc 不在默认 PATH，必须用绝对路径；退出码 1 是预期，
# 错误计数来自 stderr
cd /home/losses/Development/tiqian
nix develop -c bash -c '
KOTLINC=/nix/store/rqx09a40a82di944xi6ydjyzx632av28-kotlin-2.4.10/bin/kotlinc
$KOTLINC -Xallow-kotlin-package \
  $(find engine-haxe/out/kotlin-gen engine-haxe/out/kotlin-gen-tests -name "*.kt") \
  -d /tmp/tiqian-f32.jar 2> /tmp/census-f32.log
$KOTLINC -Xallow-kotlin-package \
  $(find engine-haxe/out/kotlin-gen-f64 engine-haxe/out/kotlin-gen-f64-tests -name "*.kt") \
  -d /tmp/tiqian-f64.jar 2> /tmp/census-f64.log
grep -c " error: " /tmp/census-f32.log
grep -c " error: " /tmp/census-f64.log'
```

K4（数值类型不匹配错误）的取数命令如下，在同一批日志上执行。六条模式按顺序
对应：桶 7（运算符两侧类型）、桶 14（Int 字面量初始化给浮点字段）、
桶 22（Number 装箱值的星投影）、桶 4 的三类数值类型转换形状（Int 给浮点、Number
装箱给浮点、Long 给浮点；最后一条在 f32 日志里为 0）。桶 4 里其余形状
（可空给非空等）计入 K6，不在本命令内。2026-09-05 实测：f32 六项相加
36+15+6+58+29+0=144，f64 相加 33+17+6+66+28+2=152，与基线一致。

```shell
for LOG in /tmp/census-f32.log /tmp/census-f64.log; do
  echo "== $LOG"
  grep -c " error: operator '.*' cannot be applied to " $LOG
  grep -c " error: initializer type mismatch: expected '\(Float\|Double\)', actual 'Int'\." $LOG
  grep -c " error: receiver type '.*' contains star projection" $LOG
  grep -c " error: argument type mismatch: actual type is 'Int', but '\(Float\|Double\)' was expected\." $LOG
  grep -c " error: argument type mismatch: actual type is 'Number & Comparable<CapturedType(\*)>', but '\(Float\|Double\)' was expected\." $LOG
  grep -c " error: argument type mismatch: actual type is 'Long', but '\(Float\|Double\)' was expected\." $LOG
done
```

### 2.2 其余四目标（当前在第一处未实现构造上中止）

```shell
nix develop -c bash -c 'haxe engine-haxe/core-ts.hxml'      # TypeScript
nix develop -c bash -c 'haxe engine-haxe/core-rust.hxml'    # Rust
nix develop -c bash -c 'haxe engine-haxe/core-swift.hxml'   # Swift
nix develop -c bash -c 'haxe engine-haxe/core-dart.hxml'    # Dart
```

四条命令当前都以退出码 1 结束：boring 遇到尚未实现生成规则的 Haxe 构造时，
在第一处这样的构造上报错并中止，报错文本见第 4 节。第一处报错消除后重新
生成，才能看到下一处；全部消除、四条命令退出码为 0 后，各自的编译普查命令
（tsc、cargo check、swift build、dart analyze）补记在本节。

### 2.3 三个已知的测量错误

- kotlinc 2.4.10 的错误消息是「null cannot be a value of a non-null type」
  （cannot 为小写），按旧版消息「Null can not be」搜索会计 0，得出错误结论。
- kotlinc 与 bun test 并发运行会竞争 CPU，使 boring 仓库
  `tests/ts/package-shell.test.ts` 的 5 秒超时项失败，普查与测试不要同时运行。
- haxe 的宏阶段错误打印格式是「文件:行号 : 消息」，与警告格式相同，没有
  「Error:」前缀；判断一条输出是错误还是警告，唯一依据是命令的退出码。

## 3 Kotlin 普查结果

### 3.1 基线快照

- vendored boring：`5d7417e`（2026-09-05）
- f32 目录（out/kotlin-gen 与 kotlin-gen-tests）：3335 条错误；f64 目录
  （out/kotlin-gen-f64 与 kotlin-gen-f64-tests）：3355 条错误
- 两个目录的错误种类分布一致，差异只有 20 条，全部是同一错误里 Float 与 Double
  的类型名写法不同；另有 13 条 f64 独有错误，见下表桶 21 的注。
- 基线历史：`a75601a` 上首次普查为 3338 与 3358 条；推进到 `5d7417e` 后
  toString 遮蔽超类一桶减少 3 条（5 变 2），两目录各减少 3 条。首版分桶脚本
  把条件类 26 条错计入推断类，本表按修正后的脚本重新分桶，两目录合计数与
  `grep -c " error: "` 的总数一致。

### 3.2 完整分桶表

每条错误只归入一桶；各桶计数之和与总数用脚本核对过（f32 合计 3335，
f64 合计 3355）。

| 桶 | 错误消息种类 | f32 | f64 |
|---|---|---|---|
| 1 | null 字面量传给非空参数 | 2122 | 2122 |
| 2 | unresolved reference | 290 | 283 |
| 3 | 可空接收者直接调用方法（only safe） | 287 | 287 |
| 4 | 实参类型不匹配 | 281 | 290 |
| 5 | 无法推断类型参数 | 42 | 42 |
| 6 | 推断类型不匹配（其余） | 11 | 11 |
| 7 | 运算符两侧类型（Float 对 Int 等） | 36 | 33 |
| 8 | 可空接收者使用运算符 | 29 | 29 |
| 9 | cannot access | 28 | 28 |
| 10 | 条件类型不匹配（Boolean? 作条件） | 26 | 26 |
| 11 | 返回类型不匹配 | 27 | 27 |
| 12 | 无适用候选重载 | 20 | 20 |
| 13 | 赋值类型不匹配 | 15 | 21 |
| 14 | 初始化类型不匹配（Int 字面量给 Float） | 15 | 17 |
| 15 | 冲突声明（conflicting declarations、redeclaration） | 15 | 15 |
| 16 | 实参数过多 | 14 | 14 |
| 17 | private 与 override 互斥 | 14 | 14 |
| 18 | compareTo 缺 operator 修饰 | 10 | 11 |
| 19 | 误把函数当值引用 | 8 | 8 |
| 20 | 私有化削弱访问（getter 变 private） | 7 | 7 |
| 21 | 未归类错误（每条消息只出现一两次） | 7 | 19 |
| 22 | 星投影禁用 compareTo（Number 装箱，即值被包装进 Number 包装类） | 6 | 6 |
| 23 | 缺 get/set 数组访问操作符 | 6 | 6 |
| 24 | jvmField 对 private 属性无效 | 5 | 5 |
| 25 | return 出现在禁止位置 | 5 | 5 |
| 26 | 成员遮蔽超类缺 override 修饰 | 2 | 2 |
| 27 | when 表达式不穷尽 | 4 | 4 |
| 28 | 变量必须初始化 | 3 | 3 |

注：桶 21 的 f64 计数包含 13 条 f64 独有错误（`Expecting an element` 9 条、
if 表达式缺 else 分支 2 条、infix 修饰缺失 1 条、表达式不能作选择器 1 条），
f32 没有这些错误；这 13 条按 K5 单独追踪。

### 3.3 桶内明细

桶 1（86 种参数类型，前 20 种）：TextStyle 143、Float 136、
MutableList&lt;String&gt; 119、WritingMode 86、LastLineAlignment 84、
LayoutProfileId 79、ParagraphStyle 67、List&lt;DecorationSpan&gt; 64、
FontRoleClassifier 55、FallbackResolver 55、Boolean 53、QuotePairAnalyzer 51、
PunctuationSpacingCompressor 51、PunctuationAtomBuilder 51、FontMetricsNormalizer
51、FontMetricsResolver 49、List&lt;RubySpan&gt; 48、InlineObjectBoundaryAdjustment
48、List&lt;InlineBoxSpan&gt; 44、MutableList&lt;FontDecisionInfo&gt; 42；其余 66 种
类型合计 746。

桶 2（290 个符号，六个分组）：

| 分组 | 条数 | 符号 |
|---|---|---|
| compare 前缀数据类比较函数 | 41 | compareTextStyle、compareCluster、compareGlue 等 41 个符号各 1 条 |
| 类型与模块名导入缺失 | 52 | UString 29、Ic 8、TiqianNoSuchElementException 7、SortedMap 2、Type 2、NodeFileSystem 2、__functional_shim 4、haxe 1、StubFontMetricsResolver、ExplainableStubTextShaper、BuiltInClreqProfileResolver、CjkFontRoleClassifier、ScriptAwareFontMetricsNormalizer 各 1 |
| 业务标识符超出作用域 | 61 | clusterRange 24、endReason 10、adjustedWidth 5、sourceRange 5、naturalWidth 4、lineIndex 4、kind 4、hangingClusterIndices 4、trailingGlue 4、ink 前缀 7、repair 2、reason 2、openStart 2、openEnd 2、其余 15 个各 1 |
| lambda 绑定的局部名 | 30 | f 8、s 6、bi 5、start 3、left 2、right 2、top 2、bottom 2、count、char、add、anchor、bodyWidth、advance 各 1 |
| 数组与字符串方法名 | 26 | copy 6、concat 5、splice 4、pop 4、shift 2、length 2、unshift、insert、charCodeAt 各 1 |
| 浮点与数学函数 | 18 | floatToI32 7、i32ToFloat 5、pi 5、minus 3、pow 2、plus 1 |

桶 3（25 种接收者类型）：PunctuationAtom? 78、GlueBudget? 36、Boolean? 20、
RubyLineHeightDecisionInfo? 17、LineRepairDecisionInfo? 17、GlueCapacity? 17、
Cluster? 17、InlineObjectLineHeightDecisionInfo? 14、ProgressiveBreakOpportunity?
12、MutableList&lt;String&gt;? 9、RubyFontGeometry? 7、PunctuationClusterGeometry? 6、
Rect? 5、PunctuationDecisionInfo? 5、其余 11 种合计 32。

桶 4（前 12 种形状）：Int 给 Float 58、Int? 给 Int 46、String? 给 String 34、
Number 装箱给 Float 29、Float? 给 Float 21、RubyKind 给 MutableList&lt;String&gt; 8、
MutableList 可空给非空 List 7、Int 给 String 6、MutableList 可空给非空 5、
ITextShaper 可空给非空 5、ClreqProfileResolver 可空给非空 5、Hyphenator 可空给
非空 4；其余 36 种形状合计 53。

桶 21（未归类错误全部列出）：f32 7 条为 ExperimentalStdlibApi 需标注 1、
super 不能作被调函数 1、局部变量被闭包捕获导致无法 smart cast 1、下划线保留名
1、iterator 二义 1、枚举当表达式引用 1；f64 另有独有 13 条（见分桶表下方关于
桶 21 的注）。

## 4 四目标生成在第一处未实现构造上中止的位置

boring 对尚未实现生成规则的 Haxe 构造，在生成阶段调用 `Context.error` 报错
并中止，不做猜测性输出。四条生成命令（第 2.2 节）当前各在第一处这样的构造
上中止；第一处之前的构造可以生成，之后的构造无法枚举，所以每消除一处报错
都要重新生成一次才知道下一处。下表是 2026-09-05 每个目标的第一处（vendored
`012ab59` 实测；Kotlin 无中止，两目录已生成）。

| 目标 | 第一处报错的文本 | boring 报错位置 | tiqian 触发点 |
|---|---|---|---|
| TypeScript | ts target: variant switch lowers at return position | `packages/compiler/reflaxe/ts/tscompiler/TsExpr.hx:1204` | `engine-haxe/src/org/tiqian/test/ShapingEvidenceJson.hx:144`（按枚举变体分派的 switch 表达式出现在赋值位置） |
| Swift | swift target: variant switch lowers at return position | `packages/compiler/reflaxe/swift/swiftcompiler/SwiftExpr.hx:1169` | 同上 `ShapingEvidenceJson.hx:144` |
| Rust | Std.string accepts scalars, enum values, records, and arrays of them only | `packages/compiler/reflaxe/rust/rustcompiler/RustExpr.hx:4052` | 七处 record 打印合成拼接了不在 `Std.string` 参数域内的值（三个普通类已修、一个 abstract `ProgressiveBreakTier`、`ParagraphLayoutPrep` 的三个函数类型字段，清单与持有者见 F0k） |
| Dart | dart target: variant switch lowers at return position | `packages/compiler/reflaxe/dart/dartcompiler/DartExpr.hx` 的 variant switch 报错处 | 同 ts 行 `ShapingEvidenceJson.hx:144`（`Math.abs` 的报错 2026-09-05 消除后，dart 的第一处与 ts、swift 是同一个构造，dart 生成器的对应规则待补，F0g 范围） |

已确认排在第一处之后的报错：

- 表达式块必须以值语句结尾（features/43）：TypeScript 目标在只编译
  `org.tiqian.core.TextRange` 加 `org.tiqian.core.TextRangeTest` 的最小集时
  命中，位置 `engine-haxe/src/org/tiqian/core/TiqianNoSuchElementException.hx:22`。
  五个生成器都有这条报错（`TsExpr.hx:529`、`SwiftExpr.hx:599`、
  `DartExpr.hx:805`、`RustExpr.hx:1064`、`KotlinExpr.hx:719`），kotlin 生成器
  已实现对应规则，所以 Kotlin 目录不受影响；Swift、Dart、Rust 是否在同样
  位置命中，待各自第一处报错消除后复测。
- `Std.string` 参数域的报错五个生成器都有（`TsExpr.hx:1778`、
  `SwiftExpr.hx:2094`、`DartExpr.hx:2020`、`RustExpr.hx:4047`、
  `KotlinExpr.hx:2104`）。参数域指 boring 规格 46 允许传入 `Std.string` 与
  字符串拼接的类型集合：标量、枚举值、record、它们的数组与 Null。tiqian 源
  的违规形态已于 2026-09-05 定位并清除一批（F0f，两个无 `toString` 的普通类
  被记录类型的打印合成拼接）；rust 目标在 fallible capacity 报错消除后又
  暴露出更深处的七处（F0k）。报错位置显示为 `(unknown)` 时的定位方法：在
  boring 报错处临时改为打印被拒类型并继续（记下结果后立即还原），可一次
  跑出全部违规类型（200 类收敛到第 53 类
  `ContextualDashEllipsisRoleResolverTest` 独立触发，2026-09-05 实测）。
- 2026-09-05 实测：把 rust 目标的 `Std.string` 参数域报错临时放宽为直接
  生成后，`core-rust.hxml` 退出码为 0，即 F0k 的七处是 rust 生成路径上最后
  一类中止。

## 5 分级标尺

复杂度（C）：C1 单点修复，一个生成器分支或一处源文件，改动预计不超过一百行；
C2 跨文件或跨目标，同一缺陷出现在多个目标，或需要 tiqian 源与 boring 生成器
配合；C3 新机制，需要新增降级能力。

优先级（P）：P0 阻塞项，阻塞后续测量或行为对齐验收；P1 大错误种类或已在修复
计划内的排队项；P2 影响总数但不阻塞测量；P3 单例且暂无复现路径。

严重性（S）：S0 错误出在语法层，使整个生成目录无法编译或无法生成；S1 两百条以上；
S2 二十到一百九十九条；S3 二十条以下。流程类条目不适用 S，记为 S-。

## 6 KPI

| KPI | 指标 | 基线 | 目标 | 对应桶或来源 |
|---|---|---|---|---|
| K1 | Kotlin null 传非空参数 | f32 2122 / f64 2122 | 0 | 桶 1 |
| K2 | Kotlin 可空接收者两桶合计 | f32 316 / f64 316 | 0 | 桶 3、8 |
| K3 | Kotlin unresolved reference | f32 290 / f64 283 | 0 | 桶 2 |
| K4 | Kotlin 数值类型不匹配（桶 7、14、22 加桶 4 的数值转换形状，取数命令见第 2.1 节） | f32 144 / f64 152 | 0 | 桶 7、14、22、4 部分 |
| K5 | Kotlin f64 独有语法错误 | f32 0 / f64 13 | 0 | 桶 21 注 |
| K6 | Kotlin 其余全部桶 | f32 463 / f64 469 | 0 | 桶 4 余量、5、6、9 至 21、23 至 28 |
| K7 | Kotlin 两目录错误总数 | f32 3335 / f64 3355 | 0 | 全部桶 |
| K8 | boring 验收链 | 全通过 | 每次合并后保持 | 不适用 |
| K9 | 四目标生成中止的第一处报错 | 4 个第一处（vendored `012ab59`） | 全部消除并产出四语言分桶表 | 第 4 节 |
| K10 | 修复排队项 | nullargs 在验收，features/43 未派发 | 全部完成 | 修复项清单 |

K6 与 K4 的分界：桶 4 里与可空相关的形状（Int? 给 Int 等）计入 K6，只有数值
类型转换的形状（Int 给浮点、Number 装箱给浮点、f64 独有的 Long 给 Double）
计入 K4。

## 7 修复项清单

### 第 0 组：nullargs 验收未完成项（K1 的剩余部分）

- [ ] F0a 诊断 test:kotlin 等脚本退出码 127：kotlinc 在 nix shell 的 PATH 上，
      但 bun 启动的子进程取不到；同一命令在另一工作树全部通过。对比两个工作树
      的 node_modules 与 package.json 差异。C1，P0，S-（阻塞全部 Kotlin、Swift、
      Dart 套件验证）。
- [ ] F0b 修复 RustExpr.hx 在 samples 样本上产出错误代码：rust 生成目录
      sorted_data_class_keys_ops.rs 第 152 至 179 行，引用了作用域外的 kind
      变量 4 处，Some 与 None 对 String 类型不匹配 25 处；boring main 上这些
      测试通过，错误全部来自 nullargs 任务的改动。C1，P0，S0。
- [ ] F0c 补 Dart 与 Rust 声明侧字段类型：构造器在参数为 null 时替换为默认值
      后，字段
      仍按可空类型生成，与返回非空 String 的函数不匹配。C2，P0，S1。
- [ ] F0d nullargs 修复合入 boring main，vendored 推进，两个 Kotlin 目录重新
      生成，复测 K1 计数。C1，P0，S1。

### 第 0.5 组：四目标生成阻断门（K9 的解锁项）

- [x] F0e 三个生成器补上 `Math.abs` 的生成规则：2026-09-05 完成并合入 boring
      main `7f2bced`。此前 Haxe 源调用 `Math.abs` 时，rust 与 dart 生成器没有
      对应的生成规则，生成阶段直接中止。本次在 `DartExpr.hx` 与 `RustExpr.hx`
      里分派 Math 函数的 switch 新增 abs 一条，操作数先经 `mathFloatArg`
      拓宽到浮点类型再取绝对值；swift 侧 `SwiftExpr.hx` 同日生成
      `abs(mathFloatArg(...))`（tiqian 的 swift 树生成后发现 Int32 接收者
      编译错，追加同一处拓宽）。样本 `samples/boring/MathMinMaxOps.hx` 新增
      `absValue` 与 `absOfInts`，测试 `samples/tests/MathMinMaxTests.hx` 新增
      对应用例（Int 用正值：boring 把 Haxe Int 映射 u32，负字面量出域）。
      验收十条套件（五条 stage1 加 test:kotlin-f32、test:rust-f32、
      test:swift、test:swift-f32、test:dart）全部 rc=0；vendored 已推进到
      `7f2bced`。
- [x] F0f 定位 `Std.string` 违规调用点：2026-09-05 完成第一轮。违规形态不在
      源码直接调 `Std.string` 的位置；实际触发点是 `@:dataClass` 的记录类型
      打印合成对无 `toString` 普通类字段的拼接（二元加法拼接经 `stdStringArg`
      进入同一条参数域检查）。boring 规格 46 裁定 3 明文规定无 `toString` 的
      类保持拒绝、编译器不猜文本形态，故修复位置裁定为 tiqian 侧：
      `ClreqPunctuationGlyphSubstitutor` 在 Kotlin 原件
      （`engine/src/commonMain/kotlin/org/tiqian/clreq/ClreqProfile.kt`）与
      Haxe 移植两侧同加显式 `toString`（两侧文本一致
      `ClreqPunctuationGlyphSubstitutor(policy=…)`，改前检查过无测试断言旧
      打印形态）；`AttachedInlinePunctuationBoundaryResult` 的 Kotlin 原件是
      `internal data class`，Haxe 侧补 `@:dataClass` 标记并改构造参数名与
      字段同名（延续 520f4be5 对 decision 类的先例）。本轮当时 rust 树的
      `Std.string` 报错不再出现；后续 fallible capacity 报错（F0j）消除后，
      生成范围扩大，又在更深处的七处出现同类报错（F0k，本轮补充修正此前的
      「不再出现」）。移植树验收（gates.sh 的 g4、tests、compare）全部通过
      （G4=0、trace 对比 121/121）。
- [ ] F0g 按枚举变体分派的 switch 表达式出现在赋值等其他位置时的生成规则
      （TypeScript 与 Swift 第一处报错）。boring 的报错文本是 variant switch
      lowers at return position，意思是这类 switch 只实现了在函数返回值位置
      的生成，其他位置一律中止。kotlin 生成器已支持任意位置，以此为参照移植
      到 `TsExpr.hx:1204` 与 `SwiftExpr.hx:1169`；触发文件
      `ShapingEvidenceJson.hx:144`。C2，P0，S0（ts 与 swift 目录无法生成）。
- [ ] F0h 函数体只有一个 switch 表达式、且被分派的枚举只有一个变体时的生成
      规则（features/43，连带「表达式块必须以值语句结尾」报错）。触发文件
      `TiqianNoSuchElementException.hx:22`：`describe` 函数的函数体是单个
      switch 表达式，枚举 `NoSuchElementError` 只有 `Message(text)` 一个变体，
      生成器把它展开成语句块后没有以返回值的语句结尾。kotlin 生成器已实现，
      移植到 ts、swift、dart、rust 四个生成器。C3，P0，S0。此项即任务 #21
      的派发范围，原评级 C2、P2、S3，因阻塞四语言普查升级。
- [ ] F0i 每消除一处第一位的报错后重跑对应生成命令，记录下一处，直到四条
      命令退出码 0；然后补记第 2.2 节四语言的编译普查命令与首次分桶表。
      C1，P1，S-。
- [x] F0j 无错误枚举函数里的整型容量表达式生成规则（rust 第一处报错，接
      F0f 之后）：2026-09-05 完成并合入 boring main `012ab59`。此前 Haxe
      源里 `new Array<T>()` 后紧跟按下标填满的循环时，rust 生成器会生成
      `Vec::with_capacity`，但循环上界既不是常量也不是 `.length` 时，要求
      函数声明错误枚举，`ParagraphShapingStage.hx:1182`（上界 `rej.size()`）
      因此中止。本次在该生成分支新增：上界是 Haxe `Int` 且函数无错误枚举时，
      按 T3 下标形式生成 `usize::try_from(上界).unwrap_or(0)`（u32 到 usize
      在所有支持的目标上必成功，`unwrap_or(0)` 分支不可达；容量只决定提前
      预留多少存储，预留 0 不影响数组正确性）。规格 06 增补「Capacity bounds
      on the Rust target」
      一节记录四类上界各自的生成形态。样本 `samples/boring/ClusterTags.hx`
      新增 `setScores` 与 `describeScores`，测试 `samples/tests/SortedKeyDomainTests.hx`
      与 `tests/ts/sorted-key-domains.test.ts` 各新增对应用例。验收十条套件
      与 test:consistency 全部 rc=0；vendored 已推进到 `012ab59`，rust 树
      该报错不再出现。
- [~] F0k rust 树 `Std.string` 参数域报错第二批（七处，F0j 消除后暴露，
      rust 生成路径最后一类中止）。本轮修正一处此前记错的事实：三个函数
      类型字段的持有者是 `ParagraphLayoutPrep`（Haxe 侧
      `engine-haxe/src/org/tiqian/layout/LineBreakPlanningStage.hx:70` 标了
      `@:dataClass`，`styleAt`、`fontSizeAt`、`bopomofoFontWeightAt` 在第 75
      至 77 行；Kotlin 原件 `LineBreakPlanningStage.kt:122` 是未标 data 的
      `internal class`）；此前误记为 `WidthIndependentParagraphAnnotation`，
      本轮已改正。一个
      abstract 是 `ProgressiveBreakTier`，把它作为记录字段持有的类只有
      `ProgressiveBreakOpportunity`（Haxe 侧 `ProgressiveBreakDecisions.hx:24`
      标了 `@:dataClass`，对应 Kotlin 原件 `ProgressiveBreakDecisions.kt:17`
      的 `data class`；除此之外无其他记录持有该类型的字段或数组）。三个
      普通类（`PunctuationClusterGeometry`、`GlueBudget`、`ClreqKinsokuRule`）
      已按 F0f 的两条修法由 tiqian-f0k 派发任务修复并合并 main（`26d89566`：
      前两个 Kotlin 原件是 data class，Haxe 侧补 `@:dataClass` 标记；
      `ClreqKinsokuRule` 两侧同加文本一致的显式 `toString`）。中央复验：
      `:engine:jvmTest` 通过，工树 gates 三项全部通过（G4-RC=0、121/121、
      COMPARE-RC=0），`core-rust.hxml` 的报错面不再出现三个类名。剩余四处
      （一个 abstract 加 `ParagraphLayoutPrep` 的三个函数字段）的处置选项
      已报用户裁定，裁定前不动。裁定补充事实（2026-09-05 实测）：
      `ParagraphLayoutPrep` 无排序键使用（`SortedMap<ParagraphLayoutPrep`、
      `SortedSet<ParagraphLayoutPrep`、`compareParagraphLayoutPrep` 三种模式
      在 engine-haxe 源与 kotlin 生成树全部零命中）；`ProgressiveBreakTier`
      的类型名在 kotlin 生成树整体零命中，`@:enum abstract` 折叠成 `Int`
      （`ProgressiveBreakOpportunity` 生成为 `val tier: Int`，比较生成为
      `o.tier == 4` 一类 Int 字面量；Kotlin 原件
      `ProgressiveBreakDecisions.kt:8` 是 `enum class
      ProgressiveBreakTier(val priority: Int)`，移植后该字段不再持有枚举
      类型而持有 Int）。C2，P0，S0。

### 第 1 组：三大错误种类（Kotlin）

- [ ] F1a 可空接收者错误（K2）：先取五个失败点的生成代码，判定归属为
      engine-haxe 源缺少空值判断（修 tiqian 源）或 boring 应生成 `?.` 安全
      调用（修生成器）。C2，P1，S1。
- [ ] F1b 名称解析第二批（K3），按桶 2 的六个分组各派发或中央修复，每个分组
      先取一个最小失败样本写进任务书。C2，P1，S1。

### 第 2 组：数值类型不匹配与 f64 独有错误（Kotlin）

- [ ] F2a 数值类型不匹配（K4）：boring 在 Float 位置为 Int 字面量与 Number 装箱值
      生成显式转换，样本库增加覆盖。C2，P1，S2。
- [ ] F2b f64 独有语法错误（K5）：先取 9 条 `Expecting an element` 的文件与
      行号，定位 f64 生成路径独有的分支。C1，P0，S0。

### 第 3 组：其余错误种类（Kotlin K6，按生成器归组）

- [ ] F3a 可见性与修饰组：桶 17（14）、桶 18（10）、桶 20（7）、桶 24（5）、
      桶 26（5），合计 41 条，同一类生成器修饰符决策。C1，P2，S2。
- [ ] F3b 语句与声明位置组：桶 25（5）、桶 23（6）、桶 16（14）、桶 15（15）、
      桶 27（4）、桶 28（3），合计 47 条。C2，P2，S2。
- [ ] F3c 推断与重载组：桶 5（42）、桶 6（37）、桶 12（20）、桶 11（27）、
      桶 10（26）、桶 19（8）、桶 9（28），合计 188 条。C2，P2，S2。
- [ ] F3d 桶 4 可空形状余量（194 条 f32）：与 K2 由同一缺陷引起，可空值传给
      非空参数，随 F1a 的判定结论一并处理。C2，P1，S1。

### 第 4 组：四语言普查（K9）

- [ ] F4a 门全清后，四个目录分别跑 tsc、cargo check、swift build、
      dart analyze，产出与第 3 节同规格的分桶表，填入 cross-target-alignment
      的最终对照。C2，P1，S-。

## 8 进度记录

| 日期 | 修复项 | 合入位置 | 复测计数 | 验收链 |
|---|---|---|---|---|
| 2026-09-05 | Kotlin 基线建立 | boring `a75601a` | f32 3338 / f64 3358（分桶脚本有条件类错分） | 不适用 |
| 2026-09-05 | 基线迁移到 `5d7417e` 并修正分桶脚本 | boring `5d7417e` | f32 3335 / f64 3355，分桶合计与总数一致 | 不适用 |
| 2026-09-05 | 四目标第一处报错实测记录 | 本文档第 4 节 | ts、swift、rust、dart 均无法生成 | 不适用 |
| 2026-09-05 | F0e 三生成器补 `Math.abs` 规则 | boring `7f2bced`（vendored 已推进） | dart 树该报错不再出现，dart 第一处改为 variant switch | 十条套件全 rc=0 |
| 2026-09-05 | F0f `Std.string` 违规调用定位与修复（第一轮） | tiqian `e0e1172d`＋`ec285e24` | 当时 rust 树该报错不再出现（当轮枚举两类），rust 第一处改为 fallible capacity | 移植树 gates.sh 三项全过；engine jvmTest 通过 |
| 2026-09-05 | F0j 整型容量上界生成规则 | boring `012ab59`（vendored 已推进） | rust 树该报错不再出现，rust 第一处改为 F0k 的 `Std.string` 第二批（放宽实测确认是最后一类） | 十条套件＋test:consistency 全 rc=0 |

## 9 已完成并合入的修复（背景）

以下修复在基线建立前已合入 boring main，其效果已包含在基线数字里：
names-r2 名称解析第一批（unresolved 从 347 降到 290）、单变体异常折叠回归修复
（`a75601a`）、record 接口字段打印与 Rust derive 修复（`5d7417e`）、Dart 的
Math.min 与 Math.max 调用点降级补臂。
