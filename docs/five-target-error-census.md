# 五目标生成代码编译错误普查与修复追踪

本文记录 engine-haxe 生成代码在五种输出语言（Kotlin、TypeScript、Rust、Swift、
Dart）下的编译错误，作为跨目标行为对齐（见
[cross-target-alignment.md](cross-target-alignment.md)）的修复计划与进度追踪。
普查对象是 engine-haxe/out/ 下由 boring 从 Haxe 源码翻译出的目标语言代码目录。
原生 `engine` 模块的 `./gradlew :engine:jvmTest` 无失败，不在本文范围内。

当前状态：Kotlin 的 f32 与 f64 两个目录已完成逐类普查（第 3 节，基线为
tiqian `105dfb30` 加 boring `4b1fec9`）；rust 门重新生成退出码为 0，首次
rust 编译普查尚待运行；TypeScript、Swift、Dart 三门生成仍各在第一处未实现
构造上中止（第 4 节），逐个补齐是三语言普查的前置条件。

## 1 更新规则

- 每个修复项只有一个复选框。满足以下四条后在复选框勾选，并在文末进度记录表
  追加一行：修复已合入 boring main（写提交号）或 tiqian（写提交号）；vendored
  副本已推进并重新生成相关目录；该错误种类在相关目录的计数都是 0；boring 验收
  链全部通过。
- 一次修复只允许勾选一次对应的复选框；不允许一次勾选多项，也不允许提前勾选。
- 计数必须来自本文「测量配方」一节的命令输出，不允许凭印象填写。
- 错误按消息骨架逐行记录，每一类一行（2026-09-06 用户裁定：不设「未归类」
  「其余」聚合行，折叠会让任务量检验失真）。新暴露的错误类在逐类表加新行，
  不允许并入既有行，也不允许并入任何聚合行。每张逐类表必须附求和校验，各类
  计数之和等于总数。

## 2 测量配方

### 2.1 Kotlin（f32 与 f64）

```shell
# 前置：vendored 副本指向要测的 boring 提交
cd /home/losses/Development/tiqian/.haxelib/boring/git && git fetch origin && git checkout <提交号>

# 从 tiqian 仓库根目录重新生成两个 Kotlin 目录。每次 haxe 前在同一个
# nix shell 里重写 .dev（已知病：.dev 损坏时 haxe 报 Type not found : Intercept）
nix develop -c bash -c 'printf "%s" /home/losses/Development/tiqian/.haxelib/boring/git > .haxelib/boring/.dev; haxe engine-haxe/core-kotlin.hxml'      # f32
nix develop -c bash -c 'printf "%s" /home/losses/Development/tiqian/.haxelib/boring/git > .haxelib/boring/.dev; haxe engine-haxe/core-kotlin-f64.hxml'  # f64

# 编译普查。kotlinc 不在默认 PATH，必须用绝对路径；退出码 1 是预期，
# 错误计数来自日志
cd /home/losses/Development/tiqian
nix develop -c bash -c '
KOTLINC=/nix/store/rqx09a40a82di944xi6ydjyzx632av28-kotlin-2.4.10/bin/kotlinc
$KOTLINC -Xallow-kotlin-package \
  $(find engine-haxe/out/kotlin-gen engine-haxe/out/kotlin-gen-tests -name "*.kt") \
  -d /tmp/tiqian-f32.jar 2> /tmp/census-f32.log
$KOTLINC -Xallow-kotlin-package \
  $(find engine-haxe/out/kotlin-gen-f64 engine-haxe/out/kotlin-gen-f64-tests -name "*.kt") \
  -d /tmp/tiqian-f64.jar 2> /tmp/census-f64.log
grep -a -c " error: " /tmp/census-f32.log
grep -a -c " error: " /tmp/census-f64.log'

# 逐类枚举（第 3.2 节逐类表的产出命令）。消息骨架指把消息里的具体
# 标识符与数字替换成占位符后得到的模板：单引号内的标识符替换为 'X'，
# 数字串替换为 N，重复的枚举项收敛。输出每一行是一个错误类与其计数；
# 两列各自求和必须等于上面的总数，此校验缺失的表无效。
for LOG in /tmp/census-f32.log /tmp/census-f64.log; do
  echo "== $LOG"
  grep -a " error: " "$LOG" | sed 's/^.* error: //' \
    | sed "s/'[^']*'/'X'/g; s/\(, 'X'\)\{2,\}/, 'X'…/g; s/[0-9]\+/N/g" \
    | sort | uniq -c | sort -rn
  grep -a " error: " "$LOG" | sed 's/^.* error: //' \
    | sed "s/'[^']*'/'X'/g; s/\(, 'X'\)\{2,\}/, 'X'…/g; s/[0-9]\+/N/g" \
    | awk '{s+=$1} END{print "sum=" s}'
done

# argument type mismatch 类的形状分解（第 3.3 节的产出命令）。形状按
# 实际类型与期望类型的可空性、是否数值转换分类；「其余转换」收集还没有
# 定名的形状，下次分解时如出现新的成批形状，拆成具名形状行。
for LOG in /tmp/census-f32.log /tmp/census-f64.log; do
  echo "== $LOG"
  grep -a " error: argument type mismatch" "$LOG" \
    | sed "s/.*actual type is '\([^']*\)', but '\([^']*\)' was expected.*/\1 \2/" \
    | awk '{a=$1; e=$2
        an=(a~/\?$/); en=(e~/\?$/)
        if (an&&!en) k="可空给非空"
        else if (a=="Int"&&(e=="Float"||e=="Double")) k="Int 给浮点"
        else if (a~/^Number/) k="Number 装箱给浮点"
        else if (a=="Long"&&(e=="Float"||e=="Double")) k="Long 给浮点"
        else k="其余转换"
        c[k]++}
      END{for(k in c) printf "%5d  %s\n", c[k], k}' | sort -rn
done

# unresolved reference 类的符号分布（第 3.4 节的产出命令）
grep -a " error: unresolved reference" /tmp/census-f32.log \
  | sed "s/.*unresolved reference '\([^']*\)'.*/\1/" | sort | uniq -c | sort -rn
```

### 2.2 其余四目标

```shell
nix develop -c bash -c 'haxe engine-haxe/core-ts.hxml'      # TypeScript，当前退出码 1
nix develop -c bash -c 'haxe engine-haxe/core-rust.hxml'    # Rust，当前退出码 0
nix develop -c bash -c 'haxe engine-haxe/core-swift.hxml'   # Swift，当前退出码 1
nix develop -c bash -c 'haxe engine-haxe/core-dart.hxml'    # Dart，当前退出码 1
```

boring 遇到尚未实现生成规则的 Haxe 构造时，在第一处这样的构造上报错并中止。
ts、swift、dart 三门当前的第一处报错见第 4 节；三门全部退出码为 0 后，各自的
编译普查命令（tsc、swiftc、dart analyze）补记在本节。rust 门已退出码 0，其
编译普查命令随首次运行（F4a）补记。

### 2.3 已知的测量错误

- kotlinc 2.4.10 的错误消息是「null cannot be a value of a non-null type」
  （cannot 为小写），按旧版消息「Null can not be」搜索会计 0，得出错误结论。
- kotlinc 诊断是多行的：候选列表等文本出现在错误行的后续行上。2026-09-06
  实测，「None of the following candidates」在一份日志里命中 384 行，全部是
  续行，错误行本身的消息是「none of the following candidates is
  applicable:」（该日志里 18 行）。因此计数一律先过滤含 ` error: ` 的行；
  对全日志直接 grep 会把续行计入，得出虚高的数。
- 消息文本必须按当前编译器版本逐字写进模式。「cannot infer type for type
  parameter」在旧版作「Not enough information to infer」，按旧文本搜索在
  kotlinc 2.4.10 上一条也匹配不到，该类会被误记为 0（2026-09-06 实测）。
- kotlinc 与 bun test 并发运行会竞争 CPU，使 boring 仓库
  `tests/ts/package-shell.test.ts` 的 5 秒超时项失败，普查与测试不要同时运行。
- haxe 的宏阶段错误打印格式是「文件:行号 : 消息」，与警告格式相同，没有
  「Error:」前缀；判断一条输出是错误还是警告，唯一依据是命令的退出码。

## 3 Kotlin 普查结果

### 3.1 基线快照

- 测量基线：tiqian `105dfb30`（本地 main）加 boring `4b1fec9`，2026-09-06
  在工树 /tmp/tiqian-census4 实测（脚本 /tmp/census-r4.sh，日志
  /tmp/census-r4-f32.log 与 /tmp/census-r4-f64.log，逐类表存于
  /tmp/census-r4-f32-families.txt 与 /tmp/census-r4-f64-families.txt）。
  主树 vendored 副本当时仍在 `012ab59`，重测前先按第 2.1 节推进。
- f32 目录（out/kotlin-gen 与 kotlin-gen-tests）1183 条错误；f64 目录
  （out/kotlin-gen-f64 与 kotlin-gen-f64-tests）1201 条错误；两目录 warning
  计数均为 0。
- 基线历史：`a75601a` 上首次普查 f32 3338 / f64 3358；`5d7417e` 上
  f32 3335 / f64 3355；`8d17b59`（nullargs 合入）上 f32 1535 / f64 1553；
  `4b1fec9`（knarrow 合入）加 tiqian 侧 onlysafe 上半（`3ce6f511`）与
  sbuf-bind（`0b152313`）后为本次的 f32 1183 / f64 1201。旧分桶表按消息
  种类聚合了 28 桶，2026-09-06 起作废，由第 3.2 节逐类表取代。

### 3.2 逐类全表（f32 36 类，f64 39 类）

「判定」列的含义：一个错误类的修复位置（boring 生成器的机制位置，或
tiqian 源的一类写法）已经探针证实时，记修复面；只有猜测记「假设」；都没有
记「未判定」。判定的方法见 T-attr（第 7 节第 1 组）：对类内抽样错误点读
生成代码后定性。假设不构成派发依据，证实后才开修复项。本文用到两个修复面
名：修复面 A 指可空接收者上方法调用的生成机制；数值转换面指数值类型互转的
生成机制（候选，待证实）。

| 错误消息类（骨架） | f32 | f64 | 判定与处置 |
|---|---:|---:|---|
| argument type mismatch: actual type is 'X', but 'X' was expected. | 407 | 416 | 形状分解见 3.3；可空形状疑与修复面 A 同源，数值形状疑为数值转换面，均待探针 |
| unresolved reference 'X'. | 333 | 326 | 未判定；符号分布见 3.4；随 T-attr |
| only safe (?.) or non-null asserted (!!.) calls are allowed on a nullable receiver of type 'X'. | 75 | 75 | 修复面 A（探针列出的三个原因见 boring-onlysafe-probe.report.md）；#22 执行中 |
| operator call is prohibited on a nullable receiver of type 'X'. Use 'X'-qualified call instead. | 60 | 60 | 假设：修复面 A 延伸（接收者类型相同）；待探针 |
| cannot infer type for type parameter 'X'. Specify it explicitly. | 42 | 42 | 未判定；随 T-attr |
| operator 'X' cannot be applied to 'X' and 'X'. | 35 | 32 | 假设：数值转换面；待探针 |
| return type mismatch: expected 'X', actual 'X'. | 28 | 28 | 未判定；随 T-attr |
| cannot access 'X': it is private in 'X'. | 27 | 27 | 未判定；随 T-attr |
| assignment type mismatch: actual type is 'X', but 'X' was expected. | 19 | 23 | 未判定；随 T-attr |
| none of the following candidates is applicable: | 18 | 18 | 未判定；随 T-attr |
| initializer type mismatch: expected 'X', actual 'X'. | 15 | 17 | 假设：数值转换面（Int 字面量给浮点为主）；待探针 |
| too many arguments for 'X'. | 14 | 14 | 未判定；随 T-attr |
| modifier 'X' is incompatible with 'X'. | 14 | 14 | 未判定；修饰符决策聚集是假设；随 T-attr |
| conflicting declarations: | 13 | 13 | 未判定；随 T-attr |
| 'X' modifier is required on 'X'. | 11 | 12 | 未判定；修饰符决策聚集是假设；随 T-attr |
| type mismatch: inferred type is 'X', but 'X' was expected. | 11 | 11 | 未判定；随 T-attr |
| function invocation 'X' expected. | 8 | 8 | 未判定；随 T-attr |
| cannot weaken access privilege private for 'X' in 'X'. | 7 | 7 | 未判定；随 T-attr |
| no 'X' operator method providing array access. | 6 | 6 | 未判定；随 T-attr |
| receiver type 'X' contains star projection which prohibits the use of 'X'. | 6 | 6 | 假设：数值转换面（Number 装箱）；待探针 |
| 'X' is prohibited here. | 5 | 5 | 未判定；随 T-attr |
| jvmField has no effect on a private property. | 5 | 5 | 未判定；随 T-attr |
| 'X' expression must be exhaustive. Add the 'X', 'X'… branches or an 'X' branch. | 4 | 4 | 未判定；随 T-attr |
| variable 'X' must be initialized. | 3 | 3 | 未判定；随 T-attr |
| unresolved reference 'X' for operator 'X'. | 3 | 3 | 未判定；随 unresolved 主类 |
| 'X' hides member of supertype 'X' and needs an 'X' modifier. | 2 | 2 | 未判定；修饰符决策聚集是假设；随 T-attr |
| redeclaration: | 2 | 2 | 未判定；随 T-attr |
| condition type mismatch: inferred type is 'X' but 'X' was expected. | 2 | 2 | 未判定；随 T-attr |
| 'X' cannot be a callee. | 1 | 1 | 未判定；随 T-attr |
| this declaration needs opt-in. Its usage must be marked with 'X' or 'X' | 1 | 1 | 未判定；随 T-attr |
| smart cast to 'X' is impossible, because 'X' is a local variable that is mutated in a capturing closure. | 1 | 1 | 未判定；随 T-attr |
| non-nullable value required to call an 'X' method in a for-loop. | 1 | 1 | 假设：修复面 A 延伸；待探针 |
| names _, __, ___, ... are reserved in Kotlin. | 1 | 1 | 未判定；随 T-attr |
| method 'X' is ambiguous for this expression. Applicable candidates: | 1 | 1 | 未判定；随 T-attr |
| infix call is prohibited on a nullable receiver of type 'X'. Use 'X'-qualified call instead. | 1 | 1 | 假设：修复面 A 延伸；待探针 |
| classifier 'X' does not have a companion object, so it cannot be used as an expression. | 1 | 1 | 未判定；随 T-attr |
| Expecting an element. | 0 | 9 | f64 独有；#25 定位任务 |
| 'X' must have both main and 'X' branches when used as an expression. | 0 | 2 | f64 独有；#25 定位任务 |
| the expression cannot be a selector (cannot occur after a dot). | 0 | 1 | f64 独有；#25 定位任务 |
| 合计（求和校验） | 1183 | 1201 | 与 3.1 节总数相等 |

判定进度小结：已判定 1 类（only-safe）；假设待证 6 类（修复面 A 延伸 3 类、
数值转换面 3 类）加 argument type mismatch 的可空与数值两类形状；f64 独有
3 类已排定位任务；其余 29 类未判定。

### 3.3 argument type mismatch 形状分解

2026-09-06 实测（r4 日志，取数命令见第 2.1 节）：

| 形状 | f32 | f64 | 判定 |
|---|---:|---:|---|
| 可空给非空（actual 类型以 ? 结尾，expected 非空） | 261 | 261 | 疑与修复面 A 同源；待探针 |
| Int 给浮点 | 80 | 88 | 疑为数值转换面；待探针 |
| Number 装箱给浮点 | 33 | 32 | 疑为数值转换面；待探针 |
| Long 给浮点 | 0 | 2 | f64 独有；疑为数值转换面；待探针 |
| 其余转换 | 33 | 33 | 未判定；随 T-attr |
| 合计（等于该类计数） | 407 | 416 | |

旧版 K4 的取数命令只覆盖数值形状（当时 f32 144 / f64 152），由本表取代；
「其余转换」行的存在不违反第 1 节的禁折叠裁定，它是对 argument type
mismatch 这一个类内部的形状分类，下次分解出现新的成批形状时拆成具名行。

### 3.4 unresolved reference 符号分布（导航用，非修复面）

2026-09-06 实测（r4 f32 日志）：去重后 111 个符号。计数前 18 位：kind 37、
UString 29、clusterRange 24、Ic 16、endReason 10、compareTo 9、f 8、
TiqianNoSuchElementException 7、floatToI32 7、s 6、copy 6、sourceRange 5、
region 5、pi 5、i32ToFloat 5、concat 5、bi 5、adjustedWidth 5。f64 侧
（326 行）分布同源，按第 2.1 节命令重取。符号名只是导航线索；哪些符号共享
一个修复面（数据类比较合成、import 登记、成员名映射、操作符生成等机制中
的哪几个）由 T-attr 探针判定，不按符号名分组派发。

## 4 三门生成阻断的当前位置（2026-09-06 r4 实测）

boring 对尚未实现生成规则的 Haxe 构造，在生成阶段调用 `Context.error` 报错
并中止，不做猜测性输出。每消除一处报错都要重新生成一次才知道下一处（reveal
loop）。下表是 2026-09-06 每门的第一处（工树 /tmp/tiqian-census4，tiqian
`105dfb30` 加 boring `4b1fec9` 实测；rust 门退出码 0，不在表内）。

| 目标 | 第一处报错的文本 | tiqian 触发点 | 处置 |
|---|---|---|---|
| TypeScript | super class has no TypeScript lowering in the subset: org.tiqian.core.TiqianIllegalArgumentException | `engine-haxe/src/org/tiqian/core/IllegalStateException.hx:3`（异常子类继承链） | #37（boring ts/swift Decl 层） |
| Swift | super class has no Swift lowering in the subset: org.tiqian.core.TiqianIllegalArgumentException | 同上 | #37 |
| Dart | dart target: variant switch lowers at return, statement, initializer, or assign position | `engine-haxe/src/org/tiqian/layout/PunctuationModel.hx:264` | #38（先判定：源侧位置或 DartExpr 降级未覆盖处） |

已越过并完成修复的阻断（保留索引）：dart 变体 switch 语句位（boring
`4e2b441`）、swift 与 dart 的 Math.pow 调用点（boring `fc0d577`）、实例字段
默认值构造器赋值（boring `81362c3`）、enum sorted keys 与 kotlin concat
（boring `556bf13`）、Math.abs 三目标（boring `7f2bced`，原 F0e）、整型容量
上界（boring `012ab59`，原 F0j）、表达式位块 features/43（boring `6251842`，
原 F0h）、StringBuf.toString 表达位（tiqian `0b152313`，源侧绑局部）、
rust 门 Std.string 参数域（随裁定二 A 解除：`ParagraphLayoutPrep` 移除
`@:dataClass` 加 `ProgressiveBreakTier` 改真枚举，tiqian `04777e8b` 六门
实测 rust 退出码 0，原 F0k）。ts 与 swift 的变体 switch 赋值位（原 F0g）
在 r4 实测中两门均已越过，当前第一处是异常超类报错；完成该规则的提交号
待核。

## 5 分级标尺

复杂度（C）：C1 单点修复，一个生成器分支或一处源文件，改动预计不超过一百行；
C2 跨文件或跨目标，同一缺陷出现在多个目标，或需要 tiqian 源与 boring 生成器
配合；C3 新机制，需要新增降级能力。

优先级（P）：P0 阻塞项，阻塞后续测量或行为对齐验收；P1 大错误种类或已在修复
计划内的排队项；P2 影响总数但不阻塞测量；P3 单例且暂无复现路径。

严重性（S）：S0 错误出在语法层，使整个生成目录无法编译或无法生成；S1 两百条
以上；S2 二十到一百九十九条；S3 二十条以下。流程类条目不适用 S，记为 S-。

## 6 KPI

工作量检验的方式（2026-09-06 用户裁定）：进度以第 3.2 节逐类表的行计数变化
为准，每类可单独复测；不设覆盖多类的「其余」聚合指标，聚合数只保留 total
一个完整性数字（各类求和必须等于 total）。修复项只对修复面开，不对消息类
主题开。

| KPI | 指标 | 现值 | 目标 | 对应 |
|---|---|---|---|---|
| K1 | 修复面 A：only safe 类计数 | f32 75 / f64 75 | 0 | #22 执行中 |
| K2 | 逐类判定完成度 | 已判定 1 / 假设待证 6＋两类形状 / 定位已排 3 / 未判定 29（共 39 类） | 39 类全部有判定结论 | T-attr |
| K3 | f32 与 f64 错误总数 | f32 1183 / f64 1201 | 0 | 逐类表求和（完整性数字，非派发单位） |
| K4 | 两个目录 warning 计数 | 0 / 0 | 保持 0 | 每次复测 |
| K5 | 五门重生成退出码 | kotlin 0、kotlin-f64 0、rust 0；ts、swift、dart 为 1 | 全部 0 | #37、#38 加 reveal loop |
| K6 | boring 验收链 | 全通过 | 每次合并后保持 | 不适用 |
| K7 | 五目标普查覆盖 | kotlin 逐类表已建；rust 门已解锁待首跑；ts、swift、dart 随 K5 | 五目标各有逐类表 | F4a |

## 7 修复项清单

### 第 0 组：nullargs 收尾（已完成合入，保留记录）

- [x] F0a-F0d nullargs 验收、样本修复、字段类型补齐、合入复测：随 nullargs-r4
      并入 boring `8d17b59` 完成；19 项验收检查全部通过，r3 普查 f32 1535 /
      f64 1553 记录了复测值。原四条子项（F0a 退出码 127 诊断、F0b RustExpr
      样本修复、F0c Dart 与 Rust 字段类型、F0d 合入复测）不再单列。

### 第 0.5 组：三门生成阻断（K5 的解锁项）

- [x] F0e 三个生成器补 `Math.abs` 生成规则：2026-09-05 并入 boring
      `7f2bced`。
- [x] F0f `Std.string` 违规调用点定位与第一轮修复：2026-09-05 完成，tiqian
      `e0e1172d` 加 `ec285e24`。
- [x] F0g 变体 switch 赋值位生成规则：r4 实测 ts 与 swift 两门已越过该
      构造（当前第一处是异常超类报错）；完成该规则的提交号待核，核对后补记。
- [x] F0h 表达式位块 features/43 五目标生成规则：2026-09-05 并入 boring
      `6251842`。
- [x] F0j 整型容量上界生成规则：2026-09-05 并入 boring `012ab59`。
- [x] F0k rust 门 Std.string 参数域剩余四处：随裁定二 A 解除（tiqian
      `04777e8b`，`ParagraphLayoutPrep` 移除 `@:dataClass`、
      `ProgressiveBreakTier` 改真枚举；合并态六门实测 rust 退出码 0）。
      此前三普通类的修复见 tiqian `26d89566`。
- [ ] F0i 每消除一处第一位的报错后重跑对应生成命令，记录下一处，直到 ts、
      swift、dart 三门退出码 0；然后补记第 2.2 节三语言的编译普查命令与
      首次逐类表（与 F4a 合并执行）。C1，P1，S-。
- [ ] F0k-核 变体 switch 赋值位的修复提交号核对（见 F0g 条）。C1，P3，S-。

### 第 1 组：判定探针（K2，先于其余修复任务的派发）

- [ ] T-attr 逐类判定探针：对第 3.2 节判定列为「未判定」或「假设」的每个
      错误类，按类内计数降序抽样错误点（每类 3 至 5 处），读对应生成代码，
      把错误类归到修复面（boring 生成器的机制位置，写明文件与分支；或
      tiqian 源的一类写法）。产出＝3.2 节判定列填全，每个新修复面在此开
      一条修复项（附错误类清单与计数）。方法与 #22 上半的探针相同（参照
      boring-onlysafe-probe.report.md）。抽样顺序：argument 可空形状 261、
      unresolved 336 / 329、operator call prohibited 60、cannot infer 42、
      argument 数值形状 113 / 122、return 28、cannot access 27，其余按表
      降序。假设的证实或否证（修复面 A 延伸 3 类、数值转换面 3 类）属于
      本项产出。C2，P1，S-。
- [ ] F1a 修复面 A 计数降为 0（#22）：only safe 类两个目录；执行中的
      派发任务 boring-onlysafe-r1，判据与任务书见
      boring-onlysafe-r1.brief.md。C2，P1，S1。

### 第 2 组：已排定位任务

- [ ] F2b f64 独有三类错误定位（#25）：`Expecting an element` 9 条、
      `'X' must have both main and 'X' branches` 2 条、`the expression
      cannot be a selector` 1 条，共 12 条（2026-09-06 现值；旧记 13 条含
      infix 一条，该条 r4 里两目录各 1 已不独有）。先取 `Expecting an
      element` 的文件与行号，定位 f64 生成路径独有分支。C1，P0，S0。

### 第 3 组：按判定结果立项（当前为空）

本组条目由 T-attr 的产出逐面开列：一条修复项对应一个修复面，附它覆盖的
错误类与形状清单、计数、判据（对应类计数降为 0 且其余类计数不上升）。
2026-09-06 撤销原第 3 组 F3a（修饰符主题）、F3b（语句位置主题）、F3c
（推断与重载主题）、F3d（桶 4 可空形状）四条按消息主题合并的条目；各
错误类已在 3.2 节逐行可见，其中可空形状与数值形状的疑议随 T-attr 判定。
撤销理由：按主题把多个修复面合并成一个任务后，进度勾选无法与任何一个
错误类的计数核对。

### 第 4 组：五目标普查（K7）

- [ ] F4a rust 门首次编译普查：门已退出码 0，跑 cargo 侧编译检查并产出
      rust 逐类表，命令与结果补记第 2.2 节。C1，P1，S-。
- [ ] F4b ts、swift、dart 三门逐类表：随 F0i 的门清产出，填入
      cross-target-alignment 的最终对照。C2，P1，S-。

## 8 进度记录

| 日期 | 修复项 | 合入位置 | 复测计数 | 验收链 |
|---|---|---|---|---|
| 2026-09-05 | Kotlin 基线建立 | boring `a75601a` | f32 3338 / f64 3358（分桶脚本有条件类错分） | 不适用 |
| 2026-09-05 | 基线迁移到 `5d7417e` 并修正分桶脚本 | boring `5d7417e` | f32 3335 / f64 3355，分桶合计与总数一致 | 不适用 |
| 2026-09-05 | 四目标第一处报错实测记录 | 本文档第 4 节 | ts、swift、rust、dart 均无法生成 | 不适用 |
| 2026-09-05 | F0e 三生成器补 `Math.abs` 规则 | boring `7f2bced`（vendored 已推进） | dart 树该报错不再出现，dart 第一处改为 variant switch | 十条套件全部退出码 0 |
| 2026-09-05 | F0f `Std.string` 违规调用定位与修复（第一轮） | tiqian `e0e1172d`＋`ec285e24` | 当时 rust 树该报错不再出现（当轮枚举两类），rust 第一处改为 fallible capacity | 移植树 gates.sh 三项全过；engine jvmTest 通过 |
| 2026-09-05 | F0j 整型容量上界生成规则 | boring `012ab59`（vendored 已推进） | rust 树该报错不再出现，rust 第一处改为 F0k 的 `Std.string` 第二批（放宽实测确认是最后一类） | 十条套件＋test:consistency 全部退出码 0 |
| 2026-09-06 | 基线迁移到 tiqian `105dfb30` 加 boring `4b1fec9`（r4） | 本文档第 3 节 | f32 1183 / f64 1201，warnings 0；ts、swift、dart 首处更新为第 4 节现值 | G4-RC=0、bun pass=121 fail=0、COMPARE-RC=0（sbuf-bind 条目） |
| 2026-09-06 | 桶表改逐类表，修两条测量错误（续行计数、消息文本过期） | 本文档第 2.3、3.2 节 | f32 36 类 / f64 39 类，求和各等于 total | 不适用 |
| 2026-09-06 | KPI 重切：按修复面与判定覆盖取代主题合并，撤销 F3a-d | 本文档第 6、7 节 | 判定进度见 3.2 节小结 | 不适用 |

## 9 已完成并合入的修复（背景）

以下修复在 r4 基线之前已合入，其效果已包含在基线数字里：names-r2 名称解析
第一批（unresolved 从 347 降到 290）、单变体异常折叠回归修复（boring
`a75601a`）、record 接口字段打印与 Rust derive 修复（boring `5d7417e`）、
Dart 的 Math.min 与 Math.max 调用点降级补臂、nullargs 五目标 null 实参的
默认值直接生成（boring `8d17b59`，null 字面量错误 2122 条降为 0）、knarrow
null 初始化位的空值处理（boring `4b1fec9`，null 残余 52 条降为 0）、
onlysafe 上半 assertFailsWith 尾端 throw 化（tiqian `3ce6f511`，only safe
95 降 75）、sbuf-bind StringBuf.toString 四站绑局部（tiqian `0b152313`，
stringbuffer 表达位错误降为 0 并解锁 ts、swift、dart 三门越过该构造）。
