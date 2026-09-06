# 五目标生成代码编译错误普查与修复追踪

本文记录 engine-haxe 生成代码在五种输出语言（Kotlin、TypeScript、Rust、Swift、
Dart）下的编译错误，作为跨目标行为对齐（见
[cross-target-alignment.md](cross-target-alignment.md)）的修复计划与进度追踪。
普查对象是 engine-haxe/out/ 下由 boring 从 Haxe 源码翻译出的目标语言代码目录。
原生 `engine` 模块的 `./gradlew :engine:jvmTest` 无失败，不在本文范围内。

当前状态：Kotlin 的 f32 与 f64 两个目录已完成逐类普查（第 3 节，基线为
tiqian `105dfb30` 加 boring `4b1fec9`）；rust 的生成命令退出码为 0，首次
编译普查已完成（第 6 节：181 条错误，按消息骨架 15 类，全部在语法层）；
Swift 的生成命令自 boring `2c9257d` 起退出码为 0，首次编译普查已完成
（第 5 节：8 条错误，按消息骨架 2 类）；TypeScript 与 Dart 的生成命令仍
各停在第一处未实现的构造上报错（清单见第 4 节）。

## 1 更新规则

- 每个修复项只有一个复选框。满足以下四条后在复选框勾选，并在文末进度记录表
  追加一行：修复已合入 boring main（写提交号）或 tiqian（写提交号）；vendored
  副本已推进并重新生成相关目录；该错误种类在相关目录的计数都是 0；boring 的
  验收命令全部通过。
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
# nix shell 里重写 .dev（.dev 损坏时 haxe 报 Type not found : Intercept，
# 所以每次生成前都重写一次）
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
nix develop -c bash -c 'haxe engine-haxe/core-swift.hxml'   # Swift，自 boring 2c9257d 起退出码 0
nix develop -c bash -c 'haxe engine-haxe/core-dart.hxml'    # Dart，当前退出码 1

# Swift 重生成与首次编译普查（2026-09-06；boring 185cf02，tiqian 3240f50a）
cd /tmp/tiqian-swiftcens && nix develop -c bash -c 'printf /tmp/boring-swiftcens > .haxelib/boring/.dev; haxe engine-haxe/core-swift.hxml; echo REGEN_RC=$?'
cd /tmp/tiqian-swiftcens && nix develop /home/losses/Development/boring -c bash -c 'find engine-haxe/out/swift-gen -name "*.swift" | sort | xargs swiftc -typecheck 2> /tmp/swiftc-census.log; echo SWIFTC_RC=$?; grep -c ": error:" /tmp/swiftc-census.log'
```

boring 遇到尚未实现生成规则的 Haxe 构造时，在第一处这样的构造上报错并中止。
ts 与 dart 的生成命令当前各停在第一处报错，见第 4 节；swift 与 rust 的生成
命令退出码已为 0，两者的编译普查命令已记在本节。ts 与 dart 的生成命令退出码
为 0 之后，两者的编译普查命令（tsc、dart analyze）补记在本节。

rust 的编译普查命令如下（2026-09-06 首次运行，F4a）：

```shell
# Cargo.toml 由 boring Compiler.hx 在 PackageShell 启用时写进输出目录本身
# （core-rust.hxml 的 -D rust-output 指到 .../rust-gen/src），cargo 从该
# 目录运行。退出码 101 是预期，错误计数来自日志
nix develop -c bash -c 'cd /tmp/tiqian-census4/engine-haxe/out/rust-gen/src && cargo check' \
  2> /tmp/census-r4-rust.log; echo rc=$?

grep -a -c ": error" /tmp/census-r4-rust.log   # 错误总数

# 逐类枚举（第 6 节逐类表的产出命令）。骨架化规则与 Kotlin 相同：引号与
# 反引号内的文本替换为占位符、重复枚举项收敛、数字替换为 N；另把
# error[E####] 收敛为 [E]
grep -a ": error" /tmp/census-r4-rust.log | sed 's/^.*: error//' \
  | sed 's/^\[E[0-9]*\]:/[E]:/' \
  | sed 's/`[^`]*`/`X`/g' \
  | sed "s/'[^']*'/'X'/g" \
  | sed 's/\(, `X`\)\{2,\}/, `X`…/g' \
  | sed 's/[0-9]\+/N/g' \
  | sort | uniq -c | sort -rn
# 求和校验：上式输出第一列求和必须等于错误总数
```

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
  在工作树 /tmp/tiqian-census4 实测（脚本 /tmp/census-r4.sh，日志
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
记「未判定」。判定的方法见 T-attr（第 8 节第 1 组）：对类内抽样错误点读
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
| 'X' cannot be a callee. | 1 | 1 | 修复面＝KotlinDecl 异常子类第二代的生成规则：super 调用被放进 init 块（IllegalStateException.kt:5:5，该类计数 1 即全部样本）；与 #37 同构造不同目标，修复项在 #37 完成后按修复面开列 |
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

判定进度小结：已判定 2 类（only safe 类；`'X' cannot be a callee` 类＝
KotlinDecl 异常子类第二代的生成缺陷，2026-09-06 随 #37 事实核查判定）；假设待证
6 类（修复面 A 延伸 3 类、数值转换面 3 类）加 argument type mismatch 的
可空与数值两类形状；f64 独有 3 类已排定位任务；其余 28 类未判定。

### 3.3 argument type mismatch 形状分解

2026-09-06 实测（r4 日志，产出本表的命令见第 2.1 节）：

| 形状 | f32 | f64 | 判定 |
|---|---:|---:|---|
| 可空给非空（actual 类型以 ? 结尾，expected 非空） | 261 | 261 | 疑与修复面 A 同源；待探针 |
| Int 给浮点 | 80 | 88 | 疑为数值转换面；待探针 |
| Number 装箱给浮点 | 33 | 32 | 疑为数值转换面；待探针 |
| Long 给浮点 | 0 | 2 | f64 独有；疑为数值转换面；待探针 |
| 其余转换 | 33 | 33 | 未判定；随 T-attr |
| 合计（等于该类计数） | 407 | 416 | |

旧版 K4 的统计命令只覆盖数值形状（当时 f32 144 / f64 152），由本表取代；
「其余转换」行的存在不违反第 1 节的禁折叠裁定，它是对 argument type
mismatch 这一个类内部的形状分类，下次分解出现新的成批形状时拆成具名行。

### 3.4 unresolved reference 符号分布（导航用，非修复面）

2026-09-06 实测（r4 f32 日志）：去重后 111 个符号。计数前 18 位：kind 37、
UString 29、clusterRange 24、Ic 16、endReason 10、compareTo 9、f 8、
TiqianNoSuchElementException 7、floatToI32 7、s 6、copy 6、sourceRange 5、
region 5、pi 5、i32ToFloat 5、concat 5、bi 5、adjustedWidth 5。f64 侧
（326 行）分布同源，按第 2.1 节命令重新统计。符号名只是导航线索；哪些符号共享
一个修复面（数据类比较合成、import 登记、成员名映射、操作符生成等机制中
的哪几个）由 T-attr 探针判定，不按符号名分组派发。

## 4 ts、swift、dart 三个目标的当前位置（2026-09-06 r4 实测）

boring 对尚未实现生成规则的 Haxe 构造，在生成阶段调用 `Context.error` 报错
并中止，不做猜测性输出。每消除一处报错都要重新生成一次才知道下一处；本文把
这套循环称为逐处重跑。下表是 2026-09-06 每个目标的第一处（工作树
/tmp/tiqian-census4，tiqian `105dfb30` 加 boring `4b1fec9` 实测；rust 的
生成命令退出码为 0，不在表内）。

| 目标 | 第一处报错的文本 | tiqian 触发点 | 处置 |
|---|---|---|---|
| TypeScript | `std/Type.hx:32` extern class Type 无 `@:native`/`@:jsRequire` 错误类 | `engine-haxe/src/std/Type.hx:32` | 修复进行中 |
| Swift | 重生成退出码为 0（boring `2c9257d` 修复异常超类报错后达成）；swiftc 首次编译普查见第 5 节 | — | 浮点字面量渲染、字符串控制字符转义；修复进行中 |
| Dart | `PunctuationModel.hx:264` 变体 switch 条件臂位置错误类 | `engine-haxe/src/org/tiqian/layout/PunctuationModel.hx:264` | 修复进行中 |

已越过并完成修复的阻断（保留索引）：dart 变体 switch 语句位（boring
`4e2b441`）、swift 与 dart 的 Math.pow 调用点（boring `fc0d577`）、实例字段
默认值构造器赋值（boring `81362c3`）、enum sorted keys 与 kotlin concat
（boring `556bf13`）、Math.abs 三目标（boring `7f2bced`，原 F0e）、整型容量
上界（boring `012ab59`，原 F0j）、表达式位块 features/43（boring `6251842`，
原 F0h）、StringBuf.toString 表达位（tiqian `0b152313`，源侧绑局部）、
rust 目录的 Std.string 参数域（随裁定二 A 解除：`ParagraphLayoutPrep` 移除
`@:dataClass`、`ProgressiveBreakTier` 改为真枚举，tiqian `04777e8b` 合并后
六个生成命令实测，rust 退出码为 0，原 F0k）。ts 与 swift 的变体 switch
赋值位（原 F0g）在 r4 实测中两个目标都已越过；完成该规则的提交号待核。

## 5 Swift 首次编译普查（2026-09-06）

基线为 boring `185cf02`、tiqian `3240f50a`。重生成命令退出码为 0，输出目录含 283 个 `.swift` 文件；使用 boring devshell 的 `swiftc -typecheck` 退出码为 123（xargs 聚合码），`/tmp/swiftc-census.log` 中含 8 条 `: error:`。

| 错误消息类（骨架） | 条数 | 代表样本文件:行 | 一句成因假设 |
|---|---:|---|---|
| expected member name following `.` | 7 | `org/tiqian/layout/PunctuationGeometryStage.swift:139` | Haxe 浮点字面量 `0.` 未被 Swift 渲染为有效浮点字面量 |
| unprintable ASCII character found in source file | 1 | `org/tiqian/test/ShapingEvidenceJson.swift:445` | Haxe 字符串转义 `\\b` 被生成成源文件中的控制字符 |
| 合计（求和校验） | 8 | 与 `/tmp/swiftc-census.log` 相等 | |

逐类定位：

- `expected member name following '.'`：生成位置为 `PunctuationGeometryStage.swift:139, 203, 255, 277, 409, 492, 493`；对应 Haxe 源 `engine-haxe/src/org/tiqian/layout/PunctuationGeometryStage.hx` 的 `pairWidth`、`runWidth`、`characterPen`、`totalAdvance`、`added`、`lead`、`trail` 初始化表达式（分别为 135、205、255、278、403、约 492、493 行）。可判定为已知的浮点字面量渲染错误类。
- `unprintable ASCII character found in source file`：生成位置为 `ShapingEvidenceJson.swift:445`；对应 Haxe 源 `engine-haxe/src/org/tiqian/test/ShapingEvidenceJson.hx` 的 `e == "b"` 分支（约 478 行）调用 `buf.addChar(8)` 的表达式。可判定为字符串控制字符转义错误类。

swift 错误逐行原文保存在 `/tmp/swiftc-census.log`；该节不修改 boring 或生成器。

## 6 rust 首次编译普查（2026-09-06）

基线与第 3.1 节相同（tiqian `105dfb30` 加 boring `4b1fec9`，工作树
/tmp/tiqian-census4，日志 /tmp/census-r4-rust.log，逐类表
/tmp/census-r4-rust-families.txt，产出逐类表的命令见第 2.2 节）。rust 的
生成命令退出码为 0（403 个 .rs 文件）；`cargo check` 退出码 101，报 181 条错误，按下
表 15 个消息骨架分类，求和校验相等。这 181 条全部在语法层（rustc 还没有
开始类型检查），第 3 节 Kotlin 侧的语义层错误类（可空形状、数值转换等）
在 rust 侧尚未进入测量。

| 错误消息类（骨架） | 计数 | 判定与处置 |
|---|---:|---|
| float literals must have an integer part | 124 | 修复面＝rust 浮点字面量生成规则：写成 `.25` 形，rust 语法要求 `0.25`（justifier_test.rs:311 实测样本）；生成函数定位随修复立项 |
| expected identifier, found keyword `X`（尾段重复消息） | 11 | 修复面＝rust 保留字标识符转义缺失：字段名 `type` 6 处、`match` 5 处原样输出（quote_pair_analyzer.rs:116 实测结构字段 `type:`） |
| expected one of `X`, `X`…, or an operator, found `X`（8 词消息） | 10 | 未判定；match 臂体生成在语句位（layout_queries.rs:379 实测 `=> let faces = …`）；随探针 |
| expected expression, found `X` | 10 | 未判定；记号分布 `.` 6、`+` 2、`)` 1、`=` 1；随探针 |
| expected pattern, found `X` | 7 | 未判定；记号分布 `=` 6、`:` 1；随探针 |
| `X` has been removed（box 语法位） | 4 | 未判定；box 记号位于 layout_queries_residual_coverage_test.rs:734 一带；随探针 |
| expected identifier, found `X` | 4 | 未判定；记号分布 `=` 2、`;` 2；随探针 |
| [E] the name `X` is defined multiple times（INSTANCE 四处） | 4 | 未判定；`pub static INSTANCE` 在同一文件四个类各一份（rich_text_role.rs:36、62、120、146）；rust 单文件多类布局与静态名冲突，机制随探针 |
| recursion limit reached while expanding `X`（format! 链） | 1 | 未判定；inline_object_decision_info.rs:78 超长 format! 链；随探针 |
| expected one of `X`, `X`…, or an operator, found `X`（unexpected token 尾） | 1 | 未判定；记号 `i`；随探针 |
| expected one of `X`, `X`…, or an operator, found `X`（消息两段重复形） | 1 | 未判定；layout_debug_assembly.rs:176；随探针 |
| expected identifier, found reserved keyword `X` | 1 | 修复面＝保留字转义缺失（同 keyword 行）：字段名 `virtual`（justifier.rs:140 实测） |
| expected expression, found reserved keyword `X` | 1 | 修复面＝保留字转义缺失（同 keyword 行）：`virtual`（justifier.rs:145） |
| [E] file not found for module `X` | 1 | 未判定；runtime/mod.rs:4 声明 `pub mod u_string;` 但无对应文件；随探针 |
| comparison operators cannot be chained | 1 | 未判定；punctuation_geometry_ledger.rs:293；随探针 |
| 合计（求和校验） | 181 | 与错误总数相等 |

判定进度小结：已判定 2 个修复面（浮点字面量生成 124 条；保留字标识符转义
缺失 13 条，跨上表 3 行），未判定 13 类共 44 条。两个修复面的修复项在
F4c 判定探针补齐生成函数定位后开列；派发顺序遵循目标优先级裁定
（kotlin、rust、dart、ts、swift）。

## 7 分级标尺

复杂度（C）：C1 单点修复，一个生成器分支或一处源文件，改动预计不超过一百行；
C2 跨文件或跨目标，同一缺陷出现在多个目标，或需要 tiqian 源与 boring 生成器
配合；C3 新机制，需要新增降级能力。

优先级（P）：P0 阻塞项，阻塞后续测量或行为对齐验收；P1 大错误种类或已在修复
计划内的排队项；P2 影响总数但不阻塞测量；P3 单例且暂无复现路径。

严重性（S）：S0 错误出在语法层，使整个生成目录无法编译或无法生成；S1 两百条
以上；S2 二十到一百九十九条；S3 二十条以下。流程类条目不适用 S，记为 S-。

## 8 KPI

工作量检验的方式（2026-09-06 用户裁定）：进度以第 3.2 节逐类表的行计数变化
为准，每类可单独复测；不设覆盖多类的「其余」聚合指标，聚合数只保留合计
一个完整性数字（各类求和必须等于合计）。修复项只对修复面开，不对消息类
主题开。

| KPI | 指标 | 现值 | 目标 | 对应 |
|---|---|---|---|---|
| K1 | 修复面 A：only safe 类计数 | f32 75 / f64 75 | 0 | #22 执行中 |
| K2 | 逐类判定完成度 | 已判定 2 / 假设待证 6＋两类形状 / 定位已排 3 / 未判定 28（共 39 类） | 39 类全部有判定结论 | T-attr |
| K3 | f32 与 f64 错误总数 | f32 1183 / f64 1201 | 0 | 逐类表求和（完整性数字，非派发单位） |
| K4 | 两个目录 warning 计数 | 0 / 0 | 保持 0 | 每次复测 |
| K5 | 各目标重生成退出码 | kotlin 0、kotlin-f64 0、rust 0、swift 0（自 boring `2c9257d`）；ts、dart 为 1 | 全部 0 | #37、#38 加逐处重跑 |
| K6 | boring 验收命令 | 全部通过 | 每次合并后保持 | 不适用 |
| K7 | 五目标普查覆盖 | kotlin、swift、rust 的逐类表已建（第 3、5、6 节）；ts、dart 随 K5 | 五目标各有逐类表 | F4a、F4b、F4c |

## 9 修复项清单

### 第 0 组：nullargs 收尾（已完成合入，保留记录）

- [x] F0a-F0d nullargs 验收、样本修复、字段类型补齐、合入复测：随 nullargs-r4
      并入 boring `8d17b59` 完成；19 项验收检查全部通过，r3 普查 f32 1535 /
      f64 1553 记录了复测值。原四条子项（F0a 退出码 127 诊断、F0b RustExpr
      样本修复、F0c Dart 与 Rust 字段类型、F0d 合入复测）不再单列。

### 第 0.5 组：各目标的生成阻断（K5 的前置修复）

- [x] F0e 三个生成器补 `Math.abs` 生成规则：2026-09-05 并入 boring
      `7f2bced`。
- [x] F0f `Std.string` 违规调用点定位与第一轮修复：2026-09-05 完成，tiqian
      `e0e1172d` 加 `ec285e24`。
- [x] F0g 变体 switch 赋值位生成规则：r4 实测 ts 与 swift 两个目标都已越过
      该构造；完成该规则的提交号待核，核对后补记（核对项见 F0k-核）。
- [x] F0h 表达式位块 features/43 五目标生成规则：2026-09-05 并入 boring
      `6251842`。
- [x] F0j 整型容量上界生成规则：2026-09-05 并入 boring `012ab59`。
- [x] F0k rust 目录 Std.string 参数域剩余四处：随裁定二 A 解除（tiqian
      `04777e8b`，`ParagraphLayoutPrep` 移除 `@:dataClass`、
      `ProgressiveBreakTier` 改为真枚举；合并后六个生成命令实测，rust
      退出码为 0）。此前三普通类的修复见 tiqian `26d89566`。
- [ ] F0i TypeScript extern 错误类：`std/Type.hx:32` 的 extern class Type 缺少
      `@:native`/`@:jsRequire`。修复进行中；完成后重跑 TypeScript 生成并继续
      逐处重跑。C1，P1，S0。
- [ ] F0i-逐处重跑 每消除一处当前第一位的报错后，重跑对应目标的生成命令并
      记录新出现的第一处报错，循环到 ts、dart 两个目标的生成命令退出码为
      0；Swift 已完成重生成与首次编译普查，逐类表见第 5 节。C1，P1，S-。
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
      派发任务现为第三轮（boring-onlysafe-r3，前两轮已并入），判据与
      任务书见 /tmp/dispatch-state/ 下 boring-onlysafe 各轮 brief。C2，P1，S1。

### 第 2 组：已排定位任务

- [ ] F2b f64 独有三类错误定位（#25）：`Expecting an element` 9 条、
      `'X' must have both main and 'X' branches` 2 条、`the expression
      cannot be a selector` 1 条，共 12 条（2026-09-06 现值；旧记 13 条含
      infix 一条，该条 r4 里两目录各 1 已不独有）。先取 `Expecting an
      element` 的文件与行号，定位 f64 生成路径独有分支。C1，P0，S0。

### 第 3 组：按判定结果立项（当前为空）

本组条目由 T-attr 的产出按修复面开列：一条修复项对应一个修复面，附它覆盖的
错误类与形状清单、计数、判据（对应类计数降为 0 且其余类计数不上升）。
2026-09-06 撤销原第 3 组 F3a（修饰符主题）、F3b（语句位置主题）、F3c
（推断与重载主题）、F3d（桶 4 可空形状）四条按消息主题合并的条目；各
错误类已在 3.2 节逐行可见，其中可空形状与数值形状的疑议随 T-attr 判定。
撤销理由：按主题把多个修复面合并成一个任务后，进度勾选无法与任何一个
错误类的计数核对。

### 第 4 组：五目标普查（K7）

- [x] F4a rust 首次编译普查：2026-09-06 完成（工作树 /tmp/tiqian-census4，
      基线同第 3.1 节）；`cargo check` 报 181 条错误、15 个消息骨架，逐类表
      与产出命令见第 2.2、6 节。同时判定 kotlin 表 `'X' cannot be a callee`
      类（IllegalStateException.kt:5:5，异常子类第二代的生成缺陷）。
- [ ] F4b ts、dart 两个目标的逐类表：swift 的逐类表已在第 5 节；本项随
      F0i-逐处重跑的产出补齐 ts 与 dart，填入 cross-target-alignment 的
      最终对照。C2，P1，S-。
- [ ] F4c rust 逐类判定探针：对第 6 节未判定的 13 类按类内计数降序抽样
      错误点，读生成代码，把错误类归到修复面，并补齐两个已判定修复面
      （浮点字面量生成、保留字转义缺失）的生成函数定位；产出＝第 6 节
      判定列填全，修复项随后按修复面开列。C2，P1，S-。

## 10 进度记录

| 日期 | 修复项 | 合入位置 | 复测计数 | 验收 |
|---|---|---|---|---|
| 2026-09-05 | Kotlin 基线建立 | boring `a75601a` | f32 3338 / f64 3358（分桶脚本有条件类错分） | 不适用 |
| 2026-09-05 | 基线迁移到 `5d7417e` 并修正分桶脚本 | boring `5d7417e` | f32 3335 / f64 3355，分桶合计与总数一致 | 不适用 |
| 2026-09-05 | 四目标第一处报错实测记录 | 本文档第 4 节 | ts、swift、rust、dart 均无法生成 | 不适用 |
| 2026-09-05 | F0e 三生成器补 `Math.abs` 规则 | boring `7f2bced`（vendored 已推进） | dart 生成目录该报错不再出现，dart 第一处改为 variant switch | 十条套件全部退出码 0 |
| 2026-09-05 | F0f `Std.string` 违规调用定位与修复（第一轮） | tiqian `e0e1172d`＋`ec285e24` | 当时 rust 生成目录该报错不再出现（当轮枚举两类），rust 第一处改为 fallible capacity | 移植树 gates.sh 三项全过；engine jvmTest 通过 |
| 2026-09-05 | F0j 整型容量上界生成规则 | boring `012ab59`（vendored 已推进） | rust 生成目录该报错不再出现，rust 第一处改为 F0k 的 `Std.string` 第二批（放宽实测确认是最后一类） | 十条套件＋test:consistency 全部退出码 0 |
| 2026-09-06 | 基线迁移到 tiqian `105dfb30` 加 boring `4b1fec9`（r4） | 本文档第 3 节 | f32 1183 / f64 1201，warnings 0；ts、swift、dart 首处更新为第 4 节现值 | G4-RC=0、bun pass=121 fail=0、COMPARE-RC=0（sbuf-bind 条目） |
| 2026-09-06 | 桶表改逐类表，修两条测量错误（续行计数、消息文本过期） | 本文档第 2.3、3.2 节 | f32 36 类 / f64 39 类，求和各等于 total | 不适用 |
| 2026-09-06 | KPI 重切：按修复面与判定覆盖取代主题合并，撤销 F3a-d | 本文档第 7、8 节 | 判定进度见 3.2 节小结 | 不适用 |
| 2026-09-06 | rust 首次编译普查（F4a）与 kotlin `'X' cannot be a callee` 类判定 | 本文档第 2.2、3.2、6 节 | rust 181 条、15 类，求和校验相等；kotlin 该类 f32/f64 各 1＝异常子类第二代 super 误入 init 块 | 不适用 |

## 11 已完成并合入的修复（背景）

以下修复在 r4 基线之前已合入，其效果已包含在基线数字里：names-r2 名称解析
第一批（unresolved 从 347 降到 290）、单变体异常折叠回归修复（boring
`a75601a`）、record 接口字段打印与 Rust derive 修复（boring `5d7417e`）、
Dart 的 Math.min 与 Math.max 调用点降级补臂、nullargs 五目标 null 实参的
默认值直接生成（boring `8d17b59`，null 字面量错误 2122 条降为 0）、knarrow
null 初始化位的空值处理（boring `4b1fec9`，null 残余 52 条降为 0）、
onlysafe 上半 assertFailsWith 尾端 throw 化（tiqian `3ce6f511`，only safe
95 降 75）、sbuf-bind StringBuf.toString 四站绑局部（tiqian `0b152313`，
stringbuffer 表达位错误降为 0，使 ts、swift、dart 三个目标越过该构造）。
