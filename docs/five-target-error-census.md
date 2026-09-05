# 五目标生成代码编译错误普查与修复追踪

本文记录 engine-haxe 生成代码在五种输出语言（Kotlin、TypeScript、Rust、Swift、
Dart）下的编译错误，作为跨目标行为对齐（见
[cross-target-alignment.md](cross-target-alignment.md)）的修复计划与进度追踪。
普查对象是 engine-haxe/out/ 下由 boring 从 Haxe 源码翻译出的目标语言代码目录。
原生 `engine` 模块的 `./gradlew :engine:jvmTest` 无失败，不在本文范围内。

当前状态：Kotlin 的 f32 与 f64 两个目录已完成全量普查（第 3 节）；TypeScript、
Rust、Swift、Dart 四个目录的生成在 boring 侧被未实现的降级主动中止（第 4 节），
解锁这些降级是四语言普查的前置条件。

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

### 2.2 其余四目标（当前被降级门阻断）

```shell
nix develop -c bash -c 'haxe engine-haxe/core-ts.hxml'      # TypeScript
nix develop -c bash -c 'haxe engine-haxe/core-rust.hxml'    # Rust
nix develop -c bash -c 'haxe engine-haxe/core-swift.hxml'   # Swift
nix develop -c bash -c 'haxe engine-haxe/core-dart.hxml'    # Dart
```

四条命令当前都以退出码 1 结束，报错文本见第 4 节阻断门表。解锁后各自的编译
普查命令（tsc、cargo check、swift build、dart analyze）在第 4 节的门全部消除后补记。

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
  的宽度写法不同；另有 13 条 f64 独有错误，见下表桶 21 的注。
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

桶 2（290 个符号，六个子族）：

| 子族 | 条数 | 符号 |
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

## 4 四目标生成阻断门

boring 对尚未实现降级的 Haxe 构造，在生成阶段调用 `Context.error` 主动中止
编译，不做猜测性输出。四条生成命令（第 2.2 节）当前各在第一处命中门时中止，
一门之后的构造无法枚举；门清一个，下一个门才出现。下表是 2026-09-05 每个目标
的第一阻断门（vendored `5d7417e` 实测）。

| 目标 | 第一阻断门的错误文本 | boring 门位置 | tiqian 触发点 |
|---|---|---|---|
| TypeScript | ts target: variant switch lowers at return position | `packages/compiler/reflaxe/ts/tscompiler/TsExpr.hx:1204` | `engine-haxe/src/org/tiqian/test/ShapingEvidenceJson.hx:144`（变体 switch 在赋值语句位） |
| Swift | swift target: variant switch lowers at return position | `packages/compiler/reflaxe/swift/swiftcompiler/SwiftExpr.hx:1169` | 同上 `ShapingEvidenceJson.hx:144` |
| Rust | Std.string accepts scalars, enum values, records, and arrays of them only | `packages/compiler/reflaxe/rust/rustcompiler/RustExpr.hx:4047` | 位置信息缺失（宏阶段以 `(unknown)` 报出），待定位（F0f） |
| Dart | dart target: Math.abs has no direct Dart lowering; the call site lowers it | `packages/compiler/reflaxe/dart/dartcompiler/DartExpr.hx:1761` | `engine-haxe/src/org/tiqian/test/trace/TraceFormat.hx:112` 等 6 处 `Math.abs` 调用 |

已确认但排在第一门之后的门：

- 表达式块必须以值语句结尾（features/43）：TypeScript 目标在只编译
  `org.tiqian.core.TextRange` 加 `org.tiqian.core.TextRangeTest` 的最小集时命中，
  位置 `engine-haxe/src/org/tiqian/core/TiqianNoSuchElementException.hx:22`，
  该函数是单 case 捕获 switch 表达式（枚举只有一个变体 Message）。五个后端都
  有此门（`TsExpr.hx:529`、`SwiftExpr.hx:599`、`DartExpr.hx:805`、
  `RustExpr.hx:1064`、`KotlinExpr.hx:719`），Kotlin 侧已实现降级所以 Kotlin
  目录不受影响；Swift、Dart、Rust 是否在同一位点命中，待各自第一门消除后
  复测。
- Std.string 参数域门五个后端都有（`TsExpr.hx:1778`、`SwiftExpr.hx:2094`、
  `DartExpr.hx:2020`、`RustExpr.hx:4047`、`KotlinExpr.hx:2104`）；tiqian 源码
  有 73 个文件调用 `Std.string`，违规调用的具体清单待定位（F0f）。

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
| K4 | Kotlin 数值宽度族（桶 7、14、22，加桶 4 的 Int 给 Float 58 与 Number 装箱 29） | f32 144 / f64 152 | 0 | 桶 7、14、22、4 部分 |
| K5 | Kotlin f64 独有语法错误 | f32 0 / f64 13 | 0 | 桶 21 注 |
| K6 | Kotlin 其余全部桶 | f32 463 / f64 469 | 0 | 桶 4 余量、5、6、9 至 21、23 至 28 |
| K7 | Kotlin 两目录错误总数 | f32 3335 / f64 3355 | 0 | 全部桶 |
| K8 | boring 验收链 | 全通过 | 每次合并后保持 | 不适用 |
| K9 | 四目标生成阻断门 | 4 个第一门＋2 个已确认后续门 | 门全部消除并产出四语言分桶表 | 第 4 节 |
| K10 | 修复排队项 | nullargs 在验收，features/43 未派发 | 全部完成 | 修复项清单 |

K6 与 K4 的分界：桶 4 里与可空相关的形状（Int? 给 Int 等）计入 K6，只有宽度
转换类形状（Int 给 Float、Number 装箱给 Float）计入 K4。

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

- [ ] F0e Dart 的 `Math.abs` 调用点降级补臂：boring
      `packages/compiler/reflaxe/dart/dartcompiler/DartExpr.hx` 第 1761 位的
      fail 分支加 abs 臂，与已合入的 Math.min 与 Math.max 补臂同一写法；tiqian
      侧不改。C1，P0，S0（dart 目录无法生成）。
- [ ] F0f 定位 Std.string 违规调用点：rust 目录第一门在 `(unknown)` 位置报出，
      先在 tiqian 源 73 个 `Std.string` 调用文件里定位违规形态（消息限定参数
      只能是标量、枚举值、record、以及这些值的数组），再裁定修 boring 扩参数
      域或 tiqian 源改用 record 序列化。C1，P0，S-（定位项）。
- [ ] F0g 变体 switch 在非返回位的降级（TypeScript 与 Swift 第一门）：kotlin
      后端已实现同构造降级，以此为参照移植到 `TsExpr.hx:1204` 与
      `SwiftExpr.hx:1169`；触发样本 `ShapingEvidenceJson.hx:144`。C2，P0，S0
      （ts 与 swift 目录无法生成）。
- [ ] F0h 单 case 捕获 switch 表达式降级（features/43，含「表达式块必须以值
      语句结尾」门）：kotlin 后端已实现，移植到 ts、swift、dart、rust 四个
      后端；触发样本 `TiqianNoSuchElementException.hx:22`。C3，P0，S0。此项
      即任务 #21 的派发范围，原评级 C2、P2、S3，因阻塞四语言普查升级。
- [ ] F0i 每清一门后重跑对应生成命令，记录下一个门，直到四条命令退出码 0；
      然后补记第 2.2 节四语言的编译普查命令与首次分桶表。C1，P1，S-。

### 第 1 组：三大错误种类（Kotlin）

- [ ] F1a 可空接收者族（K2）：先取五个失败点的生成代码，判定归属为
      engine-haxe 源缺少空值判断（修 tiqian 源）或 boring 应生成 `?.` 安全
      调用（修生成器）。C2，P1，S1。
- [ ] F1b 名称解析第二批（K3），按桶 2 六个子族各派发或中央修复，每个子族
      先取一个最小失败样本写进任务书。C2，P1，S1。

### 第 2 组：数值宽度与 f64 独有错误（Kotlin）

- [ ] F2a 数值宽度族（K4）：boring 在 Float 位置为 Int 字面量与 Number 装箱值
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
| 2026-09-05 | 四目标第一阻断门实测记录 | 本文档第 4 节 | ts、swift、rust、dart 均无法生成 | 不适用 |

## 9 已完成并合入的修复（背景）

以下修复在基线建立前已合入 boring main，其效果已包含在基线数字里：
names-r2 名称解析第一批（unresolved 从 347 降到 290）、单变体异常折叠回归修复
（`a75601a`）、record 接口字段打印与 Rust derive 修复（`5d7417e`）、Dart 的
Math.min 与 Math.max 调用点降级补臂。
