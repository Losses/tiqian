# 五目标生成代码编译错误普查与修复追踪

本文记录 engine-haxe 生成代码在五种输出语言（Kotlin、TypeScript、Rust、Swift、
Dart）下的编译错误，作为跨目标行为对齐（见
[cross-target-alignment.md](cross-target-alignment.md)）的修复计划与进度追踪。
普查对象是 engine-haxe/out/ 下由 boring 从 Haxe 源码翻译出的目标语言代码目录。
原生 `engine` 模块的 `./gradlew :engine:jvmTest` 无失败，不在本文范围内。

当前状态（2026-09-07，基线 tiqian `3f609c0f` 加 boring `e01b03b3`，工作树
/tmp/tiqian-census6）：五个目标的重生成退出码全部为 0；八格矩阵（kotlin、
swift、rust 各 f32 与 f64，ts 与 dart 单精度）编译普查全部完成，其中 rust
f32 与 swift tests 两侧为本次首测。kotlin f32 696 条 / f64 717 条（第 3
节，合并逐类表 35 行）；swift gen 侧每精度 1 条、tests 侧每精度 57 条（第
5 节）；rust 每精度 20 条、7 类，全部在语法层（第 6 节）；ts 1127 条、19
类，与 2026-09-06 首测逐类相同（第 7 节，其中测量环境 308 条、引擎侧 819
条）；dart 2606 条、33 个错误码（第 8 节）。已解决并复测确认的修复项自
2026-09-07 起从第 11 节清单删除，只留第 12 节进度行。

## 1 更新规则

- 每个修复项只有一个复选框。满足以下四条后视为完成：修复已合入 boring main
  （写提交号）或 tiqian（写提交号）；vendored 副本已推进并重新生成相关目录；
  该错误种类在相关目录的计数都是 0；boring 的验收命令全部通过。完成并在新
  基线复测确认后，把该修复项从第 11 节清单删除，只在第 12 节进度记录表留
  一行（2026-09-07 用户裁定，取代此前「勾选后保留记录」的做法）；修复项
  编号保留不复用。
- 一次修复只允许对应一个修复项；不允许一次核销多项，也不允许提前核销。
- 计数必须来自本文「测量配方」一节的命令输出，不允许凭印象填写。
- 错误按消息骨架逐行记录，每一类一行（2026-09-06 用户裁定：不设「未归类」
  「其余」聚合行，折叠会让任务量检验失真）。新暴露的错误类在逐类表加新行，
  不允许并入既有行，也不允许并入任何聚合行。每张逐类表必须附求和校验，各类
  计数之和等于总数。
- 大类内部的分解同样逐条列出：名称、模块、文件等维度的分布写全每一个名字
  与计数，不设「前 N 位」的截断，也不设把多个名字合并成一行的聚合行
  （2026-09-06 用户裁定：条目折叠省略会让工作量检验失真）。
- 修复项与 KPI 的切分必须反映修复工作量：每条修复项写明它覆盖的错误类、
  类内条目与计数，KPI 的现值写各目标逐类表的实测计数；不把多个修复位置合并
  成一个指标（2026-09-06 用户裁定）。
- 每次修复合入 boring main 后就重测受影响目标的目录：只要计数相对上一
  基线下降，立即更新对应逐类表与第 10 节 KPI 现值，并向用户交前后对照表；
  不必等计数降为 0，也不必等修复项整体完成（2026-09-07 用户裁定）。

## 2 测量配方

### 2.1 Kotlin（f32 与 f64）

```shell
# 前置：vendored 副本指向要测的 boring 提交
cd /home/losses/Development/tiqian/.haxelib/boring/git && git fetch origin && git checkout <提交号>

# 从 tiqian 仓库根目录重新生成两个 Kotlin 目录。每次 haxe 前在同一个
# nix shell 里重写 .dev（.dev 损坏时 haxe 报 Type not found : Intercept，
# 所以每次生成前都重写一次）
nix develop -c bash -c 'printf "%s" /home/losses/Development/tiqian/.haxelib/boring/git > .haxelib/boring/.dev; haxe engine-haxe/targets/kotlin-f32.hxml'  # f32
nix develop -c bash -c 'printf "%s" /home/losses/Development/tiqian/.haxelib/boring/git > .haxelib/boring/.dev; haxe engine-haxe/targets/kotlin-f64.hxml'  # f64

# 编译普查。kotlinc 不在默认 PATH，必须用绝对路径；退出码 1 是预期，
# 错误计数来自日志
cd /home/losses/Development/tiqian
nix develop -c bash -c '
KOTLINC=/nix/store/rqx09a40a82di944xi6ydjyzx632av28-kotlin-2.4.10/bin/kotlinc
$KOTLINC -Xallow-kotlin-package \
  $(find engine-haxe/out/kotlin-gen-f32 engine-haxe/out/kotlin-gen-f32-tests -name "*.kt") \
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
nix develop -c bash -c 'haxe engine-haxe/targets/ts.hxml'        # TypeScript，自 boring de11c06a 起退出码 0
nix develop -c bash -c 'haxe engine-haxe/targets/rust-f32.hxml'  # Rust（f32），当前退出码 0
nix develop -c bash -c 'haxe engine-haxe/targets/rust-f64.hxml'  # Rust（f64），当前退出码 0
nix develop -c bash -c 'haxe engine-haxe/targets/swift-f32.hxml' # Swift（f32），自 boring 2c9257d 起退出码 0
nix develop -c bash -c 'haxe engine-haxe/targets/swift-f64.hxml' # Swift（f64），自 boring 2c9257d 起退出码 0
nix develop -c bash -c 'haxe engine-haxe/targets/dart.hxml'      # Dart，自 boring `31627b5c` 起退出码 0
```

Swift 编译普查按目录运行：gen 与 tests 各自单独 typecheck 得到每目录
计数（2026-09-07 配方，基线 boring `e01b03b3`、tiqian `3f609c0f`，工作树
/tmp/tiqian-census6，日志与逐类表 /tmp/census6/swift-tests-table.txt）。
同精度的 gen 与 tests 两目录没有同名文件（2026-09-07 comm 实测），可以
合并进一次 swiftc 调用；合并测量用于区分消费侧配置与引擎侧错误：

```shell
cd /tmp/tiqian-census6/engine-haxe/out
for D in swift-gen-f64 swift-gen-f64-tests; do
  nix develop /tmp/boring-main -c bash -c "find $D -name '*.swift' | sort | xargs swiftc -typecheck" > /tmp/census6/sw-$D.log 2>&1
  echo "$D rc=$?"; grep -a -c ': error:' /tmp/census6/sw-$D.log
done   # f32 侧同形，目录名换 swift-gen-f32 与 swift-gen-f32-tests

# 合并测量（诊断用，不替代逐目录计数）：同精度 gen 与 tests 合并后跨树
# 符号可解析；语法解析错误还有剩余时语义检查不完整（实测 gen 侧的 optional
# 解包错误在合并输出中缺席），语义错误仍以逐目录计数为准
nix develop /tmp/boring-main -c bash -c "find swift-gen-f64 swift-gen-f64-tests -name '*.swift' | sort | xargs swiftc -typecheck" > /tmp/sw-combined-64.log 2>&1; echo rc=$?
grep -a -c ': error:' /tmp/sw-combined-64.log   # f32 侧同形，/tmp/sw-combined-32.log
```

boring 遇到尚未实现生成规则的 Haxe 构造时，在第一处这样的构造上报错并中止。
四个目标的生成命令退出码均已为 0，四者的编译普查命令都已记在本节。

TypeScript 重生成与首次编译普查（2026-09-06；boring `7606ff85`，tiqian
`f2517918`，工作树 /tmp/tiqian-audit，重生成日志 /tmp/audit-gen.log，
编译普查日志 /tmp/audit-tsc.log，逐类表 /tmp/audit-tsc-families.txt，
类内分解 /tmp/audit-tsc-subfamilies.txt）：

```shell
# 重生成（.dev 指向 boring 主树检出，检出位于 7606ff85）
cd /tmp/tiqian-audit && nix develop -c bash -c 'printf /home/losses/Development/boring > .haxelib/boring/.dev; haxe engine-haxe/core-ts.hxml; echo "TS_RC=$?"'

# 编译普查。tsc 用 tiqian 主树 node_modules 里的副本（5.9.3）；有错误时
# 退出码非 0 是预期，错误计数来自日志
cd /tmp/tiqian-audit/engine-haxe/out && nix develop -c bash -c 'bun /home/losses/Development/tiqian/node_modules/typescript/bin/tsc --noEmit --allowImportingTsExtensions --module esnext --moduleResolution bundler --target es2022 $(find ts-gen ts-gen-tests -name "*.ts") > /tmp/audit-tsc.log 2>&1; echo "TSC_RC=$?"; grep -c "error TS" /tmp/audit-tsc.log'

# 逐类枚举（第 7 节逐类表的产出命令）。tsc 的诊断首行含 error TSNNNN，
# 续行不含，计数只取首行。骨架化在去掉行首文件位置后的消息段上进行：
# 单引号内文本替换为 'X'，花括号与尖括号内的类型文本替换为 {…} 与 <…>
# （各两轮，处理嵌套），数字替换为 N（错误码里的数字同样被替换，输出里
# 显示为 TSN，逐类表的「错误码」列从原始行对照取回；本表 19 个骨架与
# 错误码一一对应）
grep -a 'error TS' /tmp/audit-tsc.log | sed 's/^.*error //' \
  | sed -E "s/'[^']*'/'X'/g" \
  | sed -E 's/\{[^{}]*\}/{…}/g' | sed -E 's/\{[^{}]*\}/{…}/g' \
  | sed -E 's/<[^<>]*>/<…>/g' | sed -E 's/<[^<>]*>/<…>/g' \
  | sed -E 's/[0-9]+/N/g' \
  | sort | uniq -c | sort -rn
# 求和校验：上式输出第一列求和必须等于错误总数

# 类内分解（第 7.2 节的产出命令）：对指定错误码按名字、模块名或文件抽取
# 计数，下面以 TS2304 的名称分布为例，其余错误码的抽取规则见第 7.2 节
grep -a 'error TS2304' /tmp/audit-tsc.log | sed -E "s/.*Cannot find name '([^']+)'.*/\1/" | sort | uniq -c | sort -rn
```

rust 的编译普查命令如下（2026-09-06 首次运行；2026-09-07 起两个精度
目录都测，日志 /tmp/census6/r32.log 与 r64.log）：

```shell
# Cargo.toml 由 boring Compiler.hx 在 PackageShell 启用时写进输出目录本身
# （targets/rust-f64.hxml 的 -D rust-output 指到 .../rust-gen-f64/src），
# cargo 从该目录运行。退出码 101 是预期，错误计数来自日志；必须带
# --message-format=short，原因见 2.3 节
nix develop -c bash -c 'cd /tmp/tiqian-census6/engine-haxe/out/rust-gen-f64/src && cargo check --message-format=short' \
  > /tmp/census6/r64.log 2>&1; echo rc=$?

grep -a -c ": error" /tmp/census6/r64.log   # 错误总数

# 逐类枚举（第 6 节逐类表的产出命令）。骨架化规则与 Kotlin 相同：引号与
# 反引号内的文本替换为占位符、重复枚举项收敛、数字替换为 N；另把
# error[E####] 收敛为 [E]
grep -a ": error" /tmp/census6/r64.log | sed 's/^.*: error//' \
  | sed 's/^\[E[0-9]*\]:/[E]:/' \
  | sed 's/`[^`]*`/`X`/g' \
  | sed "s/'[^']*'/'X'/g" \
  | sed 's/\(, `X`\)\{2,\}/, `X`…/g' \
  | sed 's/[0-9]\+/N/g' \
  | sort | uniq -c | sort -rn
# 求和校验：上式输出第一列求和必须等于错误总数
```

Dart 重生成与首次编译普查（2026-09-06；boring `31627b5c`，tiqian `17a646da`，
工作树 /tmp/tiqian-dartic，普查日志 /tmp/dartic-census-gen.log 与
/tmp/dartic-census-tests.log，逐类表 /tmp/dartic-code-combined.txt，类内分解
/tmp/dartic-undef-name.txt 等）：

```shell
# 重生成（消费树克隆与 .dev 都指向合并工树 /tmp/boring-darticm 的 main
# 检出 31627b5c；两处须指向同一提交，不一致时报 Type not found）
cd /tmp/tiqian-dartic && nix develop -c bash -c 'printf /tmp/boring-darticm > .haxelib/boring/.dev; rm -rf engine-haxe/out/dart-gen engine-haxe/out/dart-gen-tests; haxe engine-haxe/core-dart.hxml; echo DART_RC=$?'

# 编译普查。dart SDK 来自 boring 的 nix develop（3.13.0）；有错误时退出码
# 非 0 是预期，错误计数来自日志；两个目录分别运行
cd /tmp/tiqian-dartic/engine-haxe/out/dart-gen && nix develop /home/losses/Development/boring -c bash -c 'dart analyze --format=machine > /tmp/dartic-census-gen.log 2>&1; echo rc=$?; grep -c "^ERROR" /tmp/dartic-census-gen.log'
cd /tmp/tiqian-dartic/engine-haxe/out/dart-gen-tests && nix develop /home/losses/Development/boring -c bash -c 'dart analyze --format=machine > /tmp/dartic-census-tests.log 2>&1; echo rc=$?; grep -c "^ERROR" /tmp/dartic-census-tests.log'

# 逐类枚举（第 8 节逐类表的产出命令）。机器格式为
# 级别|类别|错误码|文件|行|列|长度|消息，错误码取第 3 列；WARNING 与 INFO
# 不计入；gen 与 tests 两目录合并计数，keys 收两侧并集
for side in gen tests; do awk -F'|' -v s=$side '/^ERROR/{print $3"\t"s}' /tmp/dartic-census-$side.log; done \
  | awk -F'\t' '{if($2=="gen") g[$1]+=1; else t[$1]+=1; keys[$1]=1} END{for (k in keys) printf "%s\t%d\t%d\t%d\n", k, g[k]+0, t[k]+0, g[k]+t[k]+0}' \
  | sort -t$'\t' -k4 -rn
# 求和校验：上式输出合计列求和必须等于错误总数 2931

# 类内分解（第 8.2 节各表的产出命令）。以 UNDEFINED_IDENTIFIER 的名称
# 分布为例，其余错误码的抽取规则见第 8.2 节
cat /tmp/dartic-census-gen.log /tmp/dartic-census-tests.log | awk -F'|' '/^ERROR/ && $3=="UNDEFINED_IDENTIFIER" {msg=$8; if (match(msg, /Undefined name '\''[^'\'']*'\''\.\''/)) print substr(msg, RSTART+16, RLENGTH-18)}' | sort | uniq -c | sort -rn
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
- rust 的 `cargo check` 默认输出多行诊断：错误行以行首 `error` 开始，没有
  `文件:行:列: error:` 前缀，本节的 `: error` 锚点一个都数不到
  （2026-09-06 实测：同一棵生成树，默认形按该锚点数得 0，short 形数得
  180）。普查命令必须带 `--message-format=short`。
- rust 的默认多行输出里，`grep -c "^error"` 会把末尾的汇总行 `error:
  could not compile ... due to N previous errors` 也计入（同一棵树 180 条
  诊断加 1 行汇总数得 181）；short 形没有汇总行，不存在这个问题。
- tsc 与 dart analyze 的诊断都打在 stdout。把 stdout 重定向到
  /dev/null（`> /dev/null 2> log`）会得到空日志而退出码仍非 0，两个目标
  被计为 0 条错误（2026-09-07 实测，ts 与 dart 两格一度误记 0）。普查命令
  必须写 `> log 2>&1`。
- swift 同精度的 gen 与 tests 两目录没有同名文件（2026-09-07 comm 实测），
  合并调用可行。2026-09-07 早先把合并调用只得 16 条记为 filename used
  twice 截断是误判：16 条是跨树符号解析后剩下的语法解析错误数（第
  5 节）。同名文件冲突出现在 f32 与 f64 两个精度目录之间（同名文件集
  完全相同），跨精度不能合并。
- swiftc 在语法解析错误还有剩余时语义检查不完整：合并测量里 gen 侧
  BopomofoParser.swift 的 optional 解包错误缺席，而逐目录 gen 单独测量
  该错误在（实测对照 /tmp/census6/sw-swift-gen-f64.log 与
  /tmp/sw-combined-64.log）；语义错误计数以逐目录为准。
- rust 当前全部错误在解析层（rustc 未开始类型检查）时，cargo check 约 1
  秒返回，属正常；完整性以日志末行 `due to N previous errors` 的 N 与计数
  相等核对（2026-09-07 两侧 N=20 均核过）。
- 早于 tiqian `3f609c0f`（生成入口拆进 engine-haxe/targets/）的消费树里，
  生成入口仍是旧名 `engine-haxe/core-<目标>.hxml`；在这些树上测量时以该树
  实际文件名为准。

## 3 Kotlin 普查结果

### 3.1 基线快照

- 测量基线：tiqian `3f609c0f`（本地 main）加 boring `e01b03b3`，2026-09-07
  在工作树 /tmp/tiqian-census6 实测（脚本 /tmp/census6.sh，日志
  /tmp/census6/k32.log 与 /tmp/census6/k64.log，逐类表与求和校验存于
  /tmp/census6/report.txt）。该树的 vendored 副本指向本树
  .haxelib/boring/git 检出 `e01b03b3`。
- f32 目录（out/kotlin-gen-f32 与 kotlin-gen-f32-tests）696 条错误；f64
  目录（out/kotlin-gen-f64 与 kotlin-gen-f64-tests）717 条错误；两目录
  warning 计数均为 0。
- 相对上一基线（f32 1183 / f64 1201）的下降原因：only safe 75→3 与
  operator call prohibited 60→6（knullinit 系列，合并 `23f4bf63`）；
  operator applied 35/32→2/2、return 28→17、initializer 15/17→0/2、
  argument 类 Int 给浮点 80/88→6/13 与 Long 给浮点 0/2（knumconv-r3，合并
  `7b3135a1`）；modifier incompatible 14→0、cannot weaken 7→0、cannot
  access 27→3（kgetvis，合并 `ad0990e2`）；smart cast、for-loop、infix、
  classifier companion 四类各 1→0。unresolved reference 反而 333/326→
  365/358（原始行 368/361，含 operator 形 3 条），上升 32 条的原因没有查明（疑为
  tiqian 侧源增长引入，未验证）；none of candidates 18→11、star projection
  6→4、condition 2→1 三处下降的原因同样没有查明。
- 基线历史：`a75601a` 上首次普查 f32 3338 / f64 3358；`5d7417e` 上
  f32 3335 / f64 3355；`8d17b59`（nullargs 合入）上 f32 1535 / f64 1553；
  tiqian `105dfb30` 加 boring `4b1fec9` 上 f32 1183 / f64 1201；本次
  tiqian `3f609c0f` 加 boring `e01b03b3` 上 f32 696 / f64 717。旧分桶表
  按消息种类聚合了 28 桶，2026-09-06 起作废，由第 3.2 节逐类表取代。

### 3.2 逐类全表（2026-09-07 基线；f32 32 类、f64 35 类非零，合并 35 行）

「判定」列的含义：一个错误类的修复位置（boring 生成器的机制位置，或
tiqian 源的一类写法）已经探针证实时，记修复位置；只有猜测记「假设」；都没有
记「未判定」。判定的方法见 T-attr（第 11 节第 1 组）：对类内抽样错误点读
生成代码后定性。假设不构成派发依据，证实后才开修复项。本轮新增一档
「形状证实」：探针读过该类抽样错误点的生成代码、确认错误落在生成形状上，
但还没有把生成器机制定位到文件与分支；它的可信度高于假设、低于修复位置，
不单独构成派发依据。本文用到两个修复位置名：修复位置 A 指可空接收者上方法调用
的生成机制；数值转换面指数值类型互转的生成机制。

判定来源：T-attr r1 报告 /tmp/dispatch-state/tiqian-tattr-r1.report.md（基线
boring `4b1fec9`，抽样形状）；T-attr r2 报告
/tmp/dispatch-state/boring-tattr2-r2.report.md（基线 boring `d870489c`，
unresolved reference 相关的十七类全量判定，覆盖 111 个符号 Σ=366；本次基线实测 112 个
符号 Σ=368，两个符号的计数差异原因未逐条查明，类归属按符号名沿用该报告）；
f64only r1 报告 /home/losses/Development/tiqian-f64only/F64ONLY-REPORT.md
（f64 独有类的根本原因定位）；T-attr r3 报告
/tmp/dispatch-state/boring-tattr3-r1.report.md（基线 boring `e01b03b3`，
十四类形状证实的机制定位与假设检验，报告引用的生成器行号由派发方在
e01b03b3 检出逐处复核）。r1 报告的逐类「修复位置位置」段是同一份七条引用的
重复粘贴，不构成逐类定位，本文不采信其定位、只采信其抽样形状；数值转换面
（KotlinExpr.hx:2806-2820 的 renderCallArgs 只在实参位插入转换）与 getter
降级可见性（`5628b4d` 生成 `private override`）两处机制由派发方在 4b1fec9
检出复核。

| 错误消息类（骨架） | f32 | f64 | 判定与处置 |
|---|---:|---:|---|
| unresolved reference 'X'. | 365 | 358 | 修复位置已全量判定（tattr2 r2 十七类，符号到类的归属见 3.4；本行是主类计数，operator 形 3 条单列在 unresolved reference for operator 行）；修复任务 knamefix-r2（#23）由用户派出，不由本会话验收 |
| argument type mismatch: actual type is 'X', but 'X' was expected. | 135 | 144 | 形状分解见 3.3；Int 给浮点 6/13 与 Long 给浮点 0/2 是 F3j 残余；可空 82、Number 装箱 14、其余 33 待证（knulljud r1 只交付了按错误类汇总的预览计数，逐形状判定未并入本表） |
| cannot infer type for type parameter 'X'. Specify it explicitly. | 42 | 42 | 假设部分否证（tattr3 r1）：原「run 块与长实参表统一机制」假设不成立；42 条中相当部分是 unresolved `copy` 与字段错误在同一函数内的连锁（每行字段访问带一条 cannot infer），连锁部分随 F3ag 消除；独立子集集中在实参含泛型或函数值的调用（`SortedTable.mapBuilder(…)`、26 参构造，与 F3x 方法值行相关），待独立轮逐点归面；tattr3 r1 列入剩余范围的逐点判定未完成 |
| return type mismatch: expected 'X', actual 'X'. | 17 | 17 | F3j 残余（knumconv-r3 合并 `7b3135a1` 后由 28/28 降 17/17）；剩余 17 条未再抽样判定 |
| too many arguments for 'X'. | 14 | 14 | 修复位置（tattr3 r1）：typer 为 std 影子数组 indexOf 的可选 ?fromIndex 合成 null 实参，String 接收者有专支丢弃该参（KotlinExpr.hx:2910-2911），List/Array 接收者走通用实例调用分支 :2973，renderCallArgs :3052-3071 只在 DefaultArgExpander 注册过默认值时替换 null，std Array.indexOf 未注册 → `indexOf(f, null)`（PreparedParagraph.kt:50、CoreBoundaryTest.kt:58 实测）。F3v |
| conflicting declarations: | 13 | 13 | 修复位置（tattr3 r1）：typer 把数组推导 `[for …]` 展开成构建器局部 `_g`，kotlin 目标 Compiler.hx:72 的 `preventRepeatVars: false` 关闭 reflaxe 的 RepeatVariableFixer，KotlinExpr.hx localName :3299-3306 对这类展开名原样输出不做兄弟去重（只对 `` ` `` 与 `_` 生成避让名）→ 同函数两个 `val _g`（PreparedParagraphJfTest.kt:116 与 :126、LayoutQueries.kt:588 起实测；tattr3 r1 加临时 trace 实验证实展开名到达生成器，实验改动已还原）。F3w |
| type mismatch: inferred type is 'X', but 'X' was expected. | 11 | 11 | 假设（T-attr 抽样含异常第二代嵌套类引用 LayoutQueries.kt:390 `TiqianNoSuchElementException.Message`，疑与 #37 异常子类是同一机制）；tattr3 r1 列入剩余范围，逐点判定未完成；待证 |
| none of the following candidates is applicable: | 11 | 11 | 探针定位待因果验证（tattr3 r1）：`+` 两侧为 `Number & Comparable<…>` 装箱与具体 Float 时 kotlinc 列出全部候选（PunctuationGeometryLedger.kt:36、:40、:43 实测；机制位置 KotlinExpr.hx binopCore :1996 起）；与 unresolved for operator、modifier required 两行同源（装箱值仍是 Number，没有转换成具体数值类型），随 #23；由 18/18 降 11/11 的原因没有查明 |
| 'X' modifier is required on 'X'. | 9 | 10 | 探针定位待因果验证（tattr3 r1）：Number 装箱值参与比较运算，kotlinc 要求 compareTo 的 operator 修饰（Justifier.kt:85 `d > 0` 实测，d 为装箱）；与 unresolved for operator、none of candidates 两行同源（装箱值仍是 Number，没有转换成具体数值类型），随 #23；f64 由 12 降 10 的原因没有查明 |
| function invocation 'X' expected. | 8 | 8 | 修复位置（tattr3 r1）：静态方法作值使用时 KotlinExpr.hx field() 的 FStatic 分支 :2184-2187 经 staticRef 返回 `Class.method` 文本，函数类型位置需要 callable reference `Class::method` 或 lambda 包装（ContextualQuoteRoleResolverNestedAndSurrogateTest.kt:84 `Support.surrogateText`、ParseTexHyphenationPatterns.kt:55 `SortedTable.compareStrings` 实测）；8 条中 1 条（`range()`）是 Array.copy 未降级产生的连锁错误（见 ambiguous 行）。F3x |
| assignment type mismatch: actual type is 'X', but 'X' was expected. | 6 | 10 | F3j 数值形状已修（19/23 降 6/10）；剩余条目待按可空形状再判 |
| operator call is prohibited on a nullable receiver of type 'X'. Use 'X'-qualified call instead. | 6 | 6 | 修复位置 A 残余（knullinit 系列合并 `23f4bf63` 后由 60/60 降 6/6）；#22 残余 |
| no 'X' operator method providing array access. | 6 | 6 | 修复位置（tattr3 r1）：set 形 4 条为 `split` 走通用实例调用分支 KotlinExpr.hx:2973 产出只读 `List`，随后下标写 `a[i] = …` 无 set（PreparedParagraph.kt:1726/1729/1741/1745）；get 形 2 条为 stringBufMutationLines :698 以文本拼接 `part + "[0].code"` 取首字符，part 为 `"" + values[i]` 时 `[0]` 绑到 `values[i]`（TracedAssertions.kt:116）。F3y |
| 'X' is prohibited here. | 5 | 5 | 修复位置（tattr3 r1）：KotlinDecl.hx testFuncDecl :1152-1177 把测试函数体包进非 inline 的 `Test.run { … }` lambda，KotlinExpr.hx stmtLines 的无实参 TReturn 分支 :557 输出不带标签的 `return`，该位置禁止（BilingualEmphasisTest.kt:16、BopomofoLayoutTest.kt:24/96/118 实测）。F3z |
| jvmField has no effect on a private property. | 5 | 5 | 修复位置（tattr3 r1）：KotlinDecl.hx objectVarDecl :1009 对一切非 final 静态字段无条件生成 `@JvmField`，未排除 private（PreparedParagraph.kt:25/27/29/31、EnglishHyphenation.kt:4 实测）。F3aa |
| 'X' expression must be exhaustive. Add the 'X', 'X'… branches or an 'X' branch. | 4 | 4 | 修复位置（tattr3 r1）：Haxe typer 把 `case A | B:` 归并为一个 case 多值，KotlinExpr.hx switchExpression :1345-1349 只渲染 `c.values[0]`，其余值丢弃 → when 缺臂（FontMetrics.kt:15、ParagraphLayoutEngine.kt:158 实测；tattr3 r1 用 `-D dump=pretty` 实验证实归并形状）。F3ab |
| the feature "collection literals" is experimental and should be enabled explicitly. This can be done by supplying the compiler argument 'X', but note that no stability guarantees are provided. | 4 | 4 | 未判定；tattr3 r1 观察到本类与 array literals、selector 两类在同表达式重叠（RubyLayoutTest.kt:27 一条表达式同时报三码），逐点判定列入剩余范围 |
| the expression cannot be a selector (cannot occur after a dot). | 4 | 5 | 未判定；与 collection literals 行同表达式重叠（见该行）；r4 时为 f64 独有 1 条，本次两精度共有（f64 升 5 的原因没有查明） |
| receiver type 'X' contains star projection which prohibits the use of 'X'. | 4 | 4 | 假设：Number 装箱，疑与 none of candidates 行同一根本原因（LineRepair.kt:198 `shrink > 0` 实测，shrink 为装箱；tattr3 r1 未逐点验证）；由 6/6 降 4/4 的原因没有查明；待证 |
| array literals outside of annotations are unsupported. | 4 | 4 | 未判定；与 collection literals 行同表达式重叠（见该行）；r4 基线没有此类 |
| variable 'X' must be initialized. | 3 | 3 | 修复位置（tattr3 r1）：与 exhaustive 行同一机制（switchExpression 丢分组值）的连锁报错，分支缺臂路径无赋值，definite-assignment 无法证明初始化（FontMetrics.kt:21/52、ParagraphLayoutEngine.kt:164 实测）。F3ab |
| unresolved reference 'X' for operator 'X'. | 3 | 3 | 探针定位待因果验证（tattr3 r1）：SortedMapTable get 返回的 `Number` 装箱值参与 `-` 运算，`!!` 后仍为 Number（PunctuationGeometryLedger.kt:36 实测；机制位置 KotlinExpr.hx binopCore :1996 起）；与 none of candidates、modifier required 两行同源（装箱值仍是 Number，没有转换成具体数值类型），随 #23 |
| only safe (?.) or non-null asserted (!!.) calls are allowed on a nullable receiver of type 'X'. | 3 | 3 | 修复位置 A 残余（knullinit 系列合并 `23f4bf63` 后由 75/75 降 3/3）；#22 残余 |
| cannot access 'X': it is private in 'X'. | 3 | 3 | 修复位置（tattr3 r1）：kgetvis 合并 `ad0990e2` 后的残余 3 条，机制为 KotlinDecl.hx funcDecl :1103 与 objectVarDecl :1008 的可见性选择只把 `:allow` 转 internal，Haxe 的 `@:access` 授权与同模块文件私有跨类访问无映射（emptyHanging 于 LineOptimizationCoverageTest.kt:151、`@:access` 的 codePointLengthAt 与 strongScriptRole 于 ContextualQuoteRoleResolver.kt:293 起实测）。F3af |
| 'X' hides member of supertype 'X' and needs an 'X' modifier. | 2 | 2 | 修复位置（tattr3 r1）：KotlinDecl.hx funcDecl :1095-1099 的 overridesAny 只认零参 toString，手写 `hashCode` 与异常载荷 `message` 不在内（BopomofoReading.hx:27 手写 `public function hashCode():Int` 于 @:dataClass 类、TraceAssertionException 载荷 `val message` 与 sealedExceptionDecl :603-604 基类 `override val message` 冲突实测；早把本类记成 dataClass 合成 hashCode 缺 override 不属实，已更正）。F3ac |
| redeclaration: | 2 | 2 | 修复位置（tattr3 r1）：tiqian 两个模块各有一个 Haxe 文件私有类 `private class Resolution`（ContextualQuoteRoleResolver.hx:316、ContextualDashEllipsisRoleResolver.hx:223），KotlinDecl.hx classDecl :207 无条件生成 top-level `class`，同 package 冲突（ContextualDashEllipsisRoleResolver.kt:241 与 ContextualQuoteRoleResolver.kt:311 实测；早把本类记成嵌套类冲突不属实，已更正）。F3ad |
| operator 'X' cannot be applied to 'X' and 'X'. | 2 | 2 | F3j 残余（knumconv-r3 合并 `7b3135a1` 后由 35/32 降 2/2） |
| 'X' cannot be a callee. | 1 | 1 | 修复位置为 KotlinDecl 异常子类第二代的生成规则：super 调用被放进 init 块（IllegalStateException.kt:5:5，该类计数 1 即全部样本）；与 #37 同构造不同目标，修复项在 #37 完成后按修复位置开列 |
| this declaration needs opt-in. Its usage must be marked with 'X' or 'X' | 1 | 1 | 测量环境（kotlinc 2.4.10 对实验性标准库 API 的 opt-in 要求，LineRepair.kt:114；可用编译器参数消除，不派修复） |
| names _, __, ___, ... are reserved in Kotlin. | 1 | 1 | 修复位置（tattr3 r1）：KotlinDecl.hx parameterText :873 与 lambda 参数渲染只调 KotlinNameEscape.escape（:42-43，只给关键字加反引号），`_` 参数名原样输出（UnicodePunctuationBoundaryTestSupport.kt:118 `resolve(_: LayoutProfileId)` 实测）；函数体局部的 `_` 已有生成名路径（localName :3304），参数位没有。F3ae |
| method 'X' is ambiguous for this expression. Applicable candidates: | 1 | 1 | 形状记录（连锁伪影，tattr3 r1）：根本原因是 LineRepair.kt:38 `initial.copy()` 的 unresolved（Haxe Array.copy 无 Kotlin 降级，走通用实例调用 :2973 产出 List 上不存在的 copy），同函数 :41-158 的错误与 :160 的迭代候选二义全为连锁错误，无独立机制。F3ag |
| condition type mismatch: inferred type is 'X' but 'X' was expected. | 1 | 1 | 假设：修复位置 A 延伸（可空 Boolean 作条件，PunctuationGeometryLedgerCoverageTest.kt:46 实测）；由 2/2 降 1/1 的原因没有查明 |
| Expecting an element. | 0 | 9 | 修复位置已判定为 kotlin f64 浮点字面量尾点（f64only r1：KotlinExpr.hx 的 `defaultArgText` 约 158 行、`coalescingDefaultText` 约 170 行、`expr` 的 TFloat 分支约 1029 行三条 f64 路径都返回没有补数字的字符串，`0.` 形字面量破坏解析并连带产生本类与 both main 类）；修复在修复任务 knamefix-r2（#23）范围内 |
| 'X' must have both main and 'X' branches when used as an expression. | 0 | 2 | 同上一行（f64only r1 判定为同一根本原因的连锁错误） |
| initializer type mismatch: expected 'X', actual 'X'. | 0 | 2 | F3j 残余（knumconv-r3 合并后 f32 侧降为 0、f64 侧余 2） |
| 合计（求和校验） | 696 | 717 | 与 3.1 节总数相等 |
相对 r4 计数已降为 0 并从表内删除的类：modifier incompatible（14/14，kgetvis）、
cannot weaken（7/7，kgetvis）、smart cast（1/1）、for-loop non-nullable
（1/1）、infix（1/1）、classifier companion（1/1）；后四类降为 0 的提交未逐项
核对，它们发生在 boring `4b1fec9` 至 `e01b03b3` 之间的合并内。

判定进度小结（2026-09-07 基线，tattr3 r1 并入后）：修复位置 21 类（unresolved
全类经 tattr2 r2 十七类判定；return、assignment、initializer、operator
applied 四类与 argument 类内 Int 给浮点、Long 给浮点两形状是 F3j 残余；
only safe 与 operator call prohibited 是修复位置 A 残余；cannot be a callee
随 #37；tattr3 r1 把 too many、conflicting、function invocation、no operator
array access、prohibited here、jvmField、exhaustive、variable must be
initialized、hides member、redeclaration、reserved、cannot access 十二类升为
修复位置，修复项 F3v 至 F3ag 开列）；探针定位待因果验证 3 类（unresolved for
operator、none of candidates、modifier required，tattr3 r1 判为同一根本
原因：装箱值仍是 Number、没有转换成具体数值类型，随 #23）；形状记录
（连锁伪影）1 类（ambiguous，根本原因是 Haxe
Array.copy 无 Kotlin 降级，修复项 F3ag）；测量环境 1 类（opt-in）；假设
4 类（cannot infer 部分否证、type mismatch inferred、star projection、
condition）；未判定 3 类（collection literals、array literals、selector，
三码同表达式重叠）；f64 独有已判 2 类（Expecting an element、both main，
f64only r1 判为同一根本原因的连锁错误）。21 加 3 加 1 加 1 加 4 加 3 加 2
等于 35，与合并行数相等。kotlin 侧剩余的判定工作是给 3 类探针定位补因果
链（随 #23 修复时验证）、逐点判定假设 4 类与未判定 3 类、把 argument 类内
可空 82、Number 装箱 14、其余 33 归到修复位置、F3ag 合入后重数 cannot infer
的连锁部分。

### 3.3 argument type mismatch 形状分解

2026-09-07 实测（/tmp/census6/report.txt 与 k32.log、k64.log，产出本表的命令见第 2.1 节）：

| 形状 | f32 | f64 | 判定 |
|---|---:|---:|---|
| 可空给非空（actual 类型以 ? 结尾，expected 非空） | 82 | 82 | 假设：疑与修复位置 A 同源；knulljud r1 对 null 相关错误类的判定报告只预览了按类汇总的计数、未逐形状并入，待并入后再判 |
| 其余转换 | 33 | 33 | 未判定；随下一轮判定 |
| Number 装箱给浮点 | 14 | 14 | 假设：T-attr 抽样为可空两臂条件表达式（FontPolicyCoverageTest.kt:125 实测），装箱路径未定位；待证 |
| Int 给浮点 | 6 | 13 | F3j 残余（knumconv-r3 合并 `7b3135a1` 后由 80/88 降至此）；FontPolicyCoverageTest.kt:243 里 `(13).toFloat()` 与未经转换的 `0` 并存是原始形状 |
| Long 给浮点 | 0 | 2 | F3j 残余（renderCallArgs 的 isIntType 不覆盖 Long，PreparedParagraphJsonNumberTest.kt:32 实测）；f64 独有 |
| 合计（等于该类计数） | 135 | 144 | |

旧版 K4 的统计命令只覆盖数值形状（当时 f32 144 / f64 152），由本表取代；
「其余转换」行的存在不违反第 1 节的禁折叠裁定，它是对 argument type
mismatch 这一个类内部的形状分类，下次分解出现新的成批形状时拆成具名行。

### 3.4 unresolved reference 符号分布（含 tattr2 r2 十七类归属）

2026-09-07 实测（/tmp/census6/k32.log，产出命令见第 2.1 节）：去重后 112
个符号，计数合计 368，等于 unresolved 主类 365 加 unresolved for operator
类的 operator 名 3。按第 1 节裁定全量列举，每格为「计数 符号」。

| 计数 符号 | 计数 符号 | 计数 符号 | 计数 符号 | 计数 符号 | 计数 符号 |
|---|---|---|---|---|---|
| 37 kind | 32 strategyName | 29 UString | 24 clusterRange | 16 Ic | 10 endReason |
| 9 compareTo | 8 f | 7 TiqianNoSuchElementException | 7 floatToI32 | 6 s | 6 copy |
| 5 sourceRange | 5 region | 5 pi | 5 i32ToFloat | 5 concat | 5 bi |
| 5 adjustedWidth | 4 trailingGlue | 4 splice | 4 SortedMap | 4 pop | 4 naturalWidth |
| 4 lineIndex | 4 hangingClusterIndices | 4 __functional_shim | 3 text | 3 start | 3 minus |
| 3 compareRawFontMetrics | 3 compareFontMetricsRequest | 2 top | 2 shift | 2 right | 2 repair |
| 2 reason | 2 pow | 2 openStart | 2 openEnd | 2 NodeFileSystem | 2 length |
| 2 left | 2 index | 2 compareTextStyle | 2 compareLayoutFontMetrics | 2 bottom | 1 unshift |
| 1 trailingGlueInitiallyConsumed | 1 repairCandidates | 1 punctuationClass | 1 policyBodyFloor | 1 message | 1 leadingGlueInitiallyConsumed |
| 1 leadingGlue | 1 insert | 1 inkWidth | 1 inkContainmentBodyFloor | 1 inkContainmentApplied | 1 inkCenter |
| 1 inkBoundsFallback | 1 inkBounds | 1 haxe | 1 hangingClusterIndex | 1 haltValidation | 1 haltAdvance |
| 1 glyphPlacementReason | 1 glyphInlineShift | 1 geometrySource | 1 count | 1 cornerRadius | 1 compareSpacingDecisionInfo |
| 1 compareSize | 1 compareShapingEvidenceKey | 1 compareShapingDecisionInfo | 1 compareRubyDecisionInfo | 1 compareRepairCandidate | 1 compareRecordedShapingResult |
| 1 compareRecordedFontMetrics | 1 comparePunctuationWidthPolicy | 1 comparePunctuationDecisionInfo | 1 compareParagraphStyle | 1 compareMetricsEvidenceKey | 1 compareMetricDecisionInfo |
| 1 compareLineRepairCandidateInfo | 1 compareLineEdgeTrimDecisionInfo | 1 compareLineCandidate | 1 compareLineBox | 1 compareLayoutConstraints | 1 compareJustificationDecisionInfo |
| 1 compareInlineObjectSpan | 1 compareInlineObjectPunctuationAttachmentDecisionInfo | 1 compareInlineObjectDecisionInfo | 1 compareInlineBoxSpan | 1 compareInlineBoxDecisionInfo | 1 compareGlyphRun |
| 1 compareGlue | 1 compareDecorationSegmentInfo | 1 compareDecorationDecisionInfo | 1 compareClusterGeometryDecisionInfo | 1 compareCluster | 1 compareBopomofoGlyphPlacement |
| 1 compareAutoSpacePolicy | 1 compareAutoSpaceDecisionInfo | 1 compareAdjustmentStylePolicy | 1 charCodeAt | 1 char | 1 bodyWidth |
| 1 anchor | 1 advanceExpansion | 1 advance | 1 add |  |  |

f64 侧原始 361 行（主类 358 加 operator 形 3）未逐符号列举。符号到修复位置的
归属按 tattr2 r2 的十七类判定（报告
/tmp/dispatch-state/boring-tattr2-r2.report.md）：strategyName 32 属类 1
接口 getter 属性未生成；kind 37 属类 2 dataClass 默认值引用兄弟参数；
UString 29 属类 3 运行时 shim 未生成；Ic 16 属类 4 跨文件 import 未登记；
compareTo 9 属类 5 comparator 缺 nullable 分支；clusterRange 24、endReason
10、sourceRange 5、region 5、adjustedWidth 5、trailingGlue 4、naturalWidth
4、lineIndex 4、hangingClusterIndices 4 九个属性名属类 6 属性访问位的连锁错误；
compareXxx 长尾属类 17 顶层函数跨文件 import 未登记与未生成；pow 2 与
NodeFileSystem 2 的因果结论由 tattr3 r1 的结束消息给出（该轮报告文件
仍记为未定位，机制锚点未存档，随 #23 修复时验证）：pow 2 条只在 f32 侧，
幂运算被降成自由函数调用形，Kotlin 没有 `kotlin.math.pow` 自由函数，f64
侧走 `Math.pow` 不触发；NodeFileSystem 2 条为 `@:jsRequire` extern 在
Kotlin 目标没有宿主边处理被丢弃。类 6、类 16、类 17
共享同一根本原因 canEmitDataClassComparator。修复随修复任务 knamefix-r2（#23），
不按符号名另行分组派发。

## 4 五个目标的生成状态（2026-09-07 实测）

boring 对尚未实现生成规则的 Haxe 构造，在生成阶段调用 `Context.error` 报错
并中止，不做猜测性输出；每消除一处报错都要重新生成一次才知道下一处，本文把
这套循环称为逐处重跑。2026-09-07 基线（tiqian `3f609c0f` 加 boring
`e01b03b3`，工作树 /tmp/tiqian-census6）上，kotlin、swift、rust 各 f32 与
f64 加 ts、dart 共八个生成命令退出码全部为 0，产物文件数依次为 397、397、
393、393、405、405、394、393（/tmp/census6/report.txt 首段），逐处重跑
阶段结束。八个格子的编译普查见第 3、5、6、7、8 节；生成阻断期的修复历史
并入第 12 节进度表。

## 5 Swift 编译普查（2026-09-07）

2026-09-07 基线（tiqian `3f609c0f` 加 boring `e01b03b3`，工作树
/tmp/tiqian-census6）逐目录 `swiftc -typecheck` 实测（产出命令见第 2.2 节，
逐类表 /tmp/census6/swift-tests-table.txt）：两个精度的 gen 侧各 1 条、
tests 侧各 57 条，f32 与 f64 计数与分布相同。

gen 侧（每精度 1 条）：

| 错误消息类（骨架） | 条数 | 样本 | 判定与处置 |
|---|---:|---|---|
| value of optional type 'X?' must be unwrapped to a value of type 'X' | 1 | swift-gen-f64/org/tiqian/clreq/BopomofoParser.swift:21:27 | 修复任务 swiftnarrow 正在执行（提交 `49b1d52a` 修掉此缺陷但在消费树引入 6 条回归，r2 轮追加清除回归，判据为逐目录 0 条） |

tests 侧（每精度 57 条）。判定来源为 tswift1 r1 报告
/tmp/dispatch-state/boring-tswift1-r1.report.md（六类逐类三件，生成器锚点
由派发方在 e01b03b3 复核）；消费侧配置与引擎侧的区分来自合并测量
（第 2.2 节，日志 /tmp/sw-combined-64.log 与 /tmp/sw-combined-32.log，
两精度同为 16 条语法解析错误）：

| 错误消息类（骨架） | 条数 | 判定与处置 |
|---|---:|---|
| cannot find 'X' in scope | 32 | 消费侧配置：tests 树文件没有 import 头（tiqian 的 targets/swift-common.hxml 未定义 `-D swift-test-import`，boring 合同见 examples/swift.hxml:20 与 Compiler.hx fileContent 的头部条件 :422-425），且支撑类 TracedAssertions 与 TestTraceRecorder 实际生成在 gen 树 org/tiqian/test/trace/，tests 树单独 typecheck 必然找不到；合并测量下这类错误数为 0。F3u |
| static methods may only be declared on a type | 13 | 修复位置：swift 生成器把语句体闭包字面量直接嵌进外层字符串插值段（ExplainableStubParagraphLayoutEngineTest.swift:69 实测：Haxe 源的 UString.slice 调用被降级成 `let from`/`let to` 多语句闭包后原样拼进 `\(...)` 段；SwiftExpr.hx stdStringType 的 IsArray/IsSortedSet/IsSortedMap 三个分支在 e01b03b3 :1824-1836 也生成同类 `{ () -> String in … }()` 闭包，是同一修复位置的第二条触发路径）；解析器脱离 enum 作用域后，:106 起的 13 个 public static func 变成顶层声明。源头 Haxe 源 ExplainableStubParagraphLayoutEngineTest.hx:63-104 的字符串拼接。F3t |
| 'X' requires a contextual type | 9 | 消费侧配置连锁：被调方不可见时字面 nil 无上下文类型（BopomofoParserTest.swift 断言调用的第三实参）；合并测量下这类错误数为 0。随 F3u |
| unterminated string literal | 1 | F3t（同一连锁的词法症状，:69:20） |
| extraneous 'X' at top level | 1 | F3t（enum 被提前关闭后末尾 } 变多余，:315:1） |
| cannot find 'X' to match opening 'X' in string interpolation | 1 | F3t（同一连锁的插值定界症状，:69:99） |
| 合计（求和校验） | 57 | 与 tests 目录错误总数相等；其中消费侧配置 41、引擎侧（F3t）16 |

文件分布：BopomofoParserTest.swift 41 条（全部为消费侧配置两类）、
ExplainableStubParagraphLayoutEngineTest.swift 16 条（全部为 F3t 连锁；
合并测量后仅剩这 16 条，f32 侧同值同分布）。

2026-09-06 首测（boring `185cf02`、tiqian `3240f50a`）gen 侧报 8 条：浮点
字面量 `expected member name following '.'` 7 条（PunctuationGeometryStage.swift
的 139、203、255、277、409、492、493 行，对应 Haxe 源
engine-haxe/src/org/tiqian/layout/PunctuationGeometryStage.hx 的七个初始化
表达式）与控制字符 `unprintable ASCII character found in source file` 1 条
（ShapingEvidenceJson.swift:445，对应 Haxe 源 ShapingEvidenceJson.hx 约
478 行的 `buf.addChar(8)`）。两类由 boring `4c81c5fc`（合并 `e01b03b3`）
修复，本次复测均不再出现（进度见第 12 节 swiftrem-r2 行）。

## 6 rust 编译普查（2026-09-07）

2026-09-07 基线（tiqian `3f609c0f` 加 boring `e01b03b3`，工作树
/tmp/tiqian-census6，日志 /tmp/census6/r64.log 与 /tmp/census6/r32.log，
产出命令见第 2.2 节）实测：两个精度目录各 20 条错误、7 个消息骨架，
f32 侧为本次首测、与 f64 同值。这 20 条全部在语法层（rustc 还没有开始
类型检查），完整性以日志末行 due to N previous errors 的 N 等于 20 核对
（两侧均核过，见 2.3 节）；kotlin 侧的语义层错误类在 rust 侧尚未进入
测量。历史：2026-09-06 首测（boring `4b1fec9`）181 条、15 类，同日
boring `75b08ed2` 复测 180 条；rustflit 修掉浮点字面量类 124 条（合并
`d870489c`），rustf4c 修掉保留字转义类 12 条（保留字部分 boring
`dc09d773`、self 转 self_ 部分 boring `e4c24e77`，合并 `782526d7`）。
其余 44 条到本次基线变为 20 条所减少的 24 条，分摊在 `782526d7` 至
`e01b03b3` 之间的哪些合并未逐笔核对（我还没有验证）。剩余 7 类已由
rustf4c r2 判定探针全部归到 rust 生成器（报告
/tmp/dispatch-state/boring-rustf4c-r2.report.md），修复项 F3n 至 F3s
按修复位置开列。

| 错误消息类（骨架） | 计数 | 判定与处置 |
|---|---:|---|
| expected one of `X`, `X`…, or an operator, found `X`（match 臂语句位） | 10 | 修复位置为 rust 生成器 switch 臂渲染把臂模式前置于语句（RustExpr.hx 的 switch 分支约 2600-2690；layout_queries.rs:379、383、387 与 annotation_geometry_stage.rs:483、487、491、496、539、552、557）；F3n |
| [E] the name `X` is defined multiple times（INSTANCE） | 4 | 修复位置为 rust 静态成员生成在单文件多类布局下重名（RustDecl.hx 的 static 约 1000-1250 与 impl 约 2000 起；rich_text_role.rs:36、62、120、146 四处 `pub static INSTANCE`）；F3o |
| expected identifier, found `X`（分号位） | 2 | 修复位置为标识符转换产出空标识符（RustImports 的 toSnakeCase 返回空名；display_glyph_substitution_engine_test_support.rs:12:56 与 text_shaper.rs:4:56）；F3p |
| expected one of `X`, `X`…, or an operator, found `X`（layout_debug_assembly 形） | 1 | 与 match 臂语句位同一根本原因（layout_debug_assembly.rs:176:161）；F3n |
| [E] file not found for module `X` | 1 | 修复位置为 u_string 模块文件未随消费方配置写出（Compiler.hx 约 347 与 RustRuntime.hx 约 320；runtime/mod.rs:4 声明 `pub mod u_string;` 但无对应文件）；F3q |
| recursion limit reached while expanding `X`（format! 链） | 1 | 修复位置为超长 format! 链递归超限（RustExpr.hx 约 2458-2478 与 4117-4165；inline_object_decision_info.rs:78）；F3r |
| comparison operators cannot be chained | 1 | 修复位置为链式比较未降级为 `a < b && b < c`（punctuation_geometry_ledger.rs:293:30）；F3s |
| 合计（求和校验） | 20 | 与每精度错误总数相等（f32 与 f64 同值） |

判定进度小结：7 类全部有修复位置判定，修复项 F3n 至 F3s 开列在第 11 节
第 3 组；派发顺序遵循目标优先级裁定（kotlin、rust、dart、ts、swift）。

## 7 TypeScript 编译普查（2026-09-07 复测）

首测基线为 boring `7606ff85`、tiqian `f2517918`（工作树 /tmp/tiqian-audit，
编译普查日志 /tmp/audit-tsc.log，逐类表 /tmp/audit-tsc-families.txt，类内
分解 /tmp/audit-tsc-subfamilies.txt，产出命令见第 2.2 节）。2026-09-07 在
基线 tiqian `3f609c0f` 加 boring `e01b03b3`（工作树 /tmp/tiqian-census6，
日志 /tmp/census6/ts.log，逐类表 /tmp/census6/ts2-families.txt）复测：
tsc 5.9.3 报 1127 条错误，19 个消息骨架的逐类计数与首测全部相同，下表
数字两轮一致。其中测量环境条目合计 308 条：TS2307 的 bun:test 与
@tiqian 三个模块名 295 条（tsc 未配置 bun 类型）、TS2304 的 TestCore
8 条（测试支撑名字，由运行环境提供）、TS2580 的 process 3 条与 TS2307
的 node:fs、node:path 2 条（tsc 未配置 node 类型）；引擎侧错误为 1127
减 308 等于 819 条。判定来源为 tsprobe2 r1 报告
/tmp/dispatch-state/boring-tsprobe2-r1.report.md（19 类逐类判定）。

| 错误码：错误消息类（骨架） | 计数 | 判定与处置 |
|---|---:|---|
| TS2307：Cannot find module 'X' or its corresponding type declarations. | 303 | 模块分布见 7.2；297 条为测量环境；其余 6 条（相对路径 5 条与 haxe/Exception 1 条）已判引擎侧为 ts 生成器的导入路径问题，分支未逐条定位；转按修复位置派发 |
| TS2448：Block-scoped variable 'X' used before its declaration. | 215 | 已证实 ts 生成器（TsExpr 的语句融合与局部绑定生成顺序；LayoutDumpFormat.ts 单文件 214 条、ShapingEvidenceJson.ts 1 条，分布见 7.2） |
| TS2304：Cannot find name 'X'. | 183 | 已判 ts 生成器：分组判定全归生成侧（Ic 103 条源于Ic 声明只生成类型、未生成值侧声明；compare 前缀 6 条同 TS2724 的导出登记缺陷）；TestCore 8 条为测量环境；名称分布见 7.2 |
| TS2554：Expected N arguments, but got N. | 133 | 已证实 ts 生成器：默认参数与可选参数的调用实参补全机制不完整（与区间形 54 条同一原因，两形合计 187 条） |
| TS2345：Argument of type 'X' is not assignable to parameter of type 'X'. | 87 | 已判 ts 生成器：枚举载荷假设经逐类判定证伪为主因（原假设为对象字面量不能赋给枚举类型，KinsokuLevelTest.test.ts:113 样本）；TS2322、TS2420 同组 |
| TS2554：Expected N-N arguments, but got N. | 54 | 同 TS2554 第一形（默认/可选参数补全机制不完整，两形合计 187 条） |
| TS2724：'X' has no exported member named 'X'. Did you mean 'X'? | 35 | 已证实 ts 生成器：compare 函数的导出与导入登记缺陷（名称分布见 7.2，35 个比较器名全列；与 TS2305、TS2304 的 compare 条目同源） |
| TS2551：Property 'X' does not exist on type 'X'. Did you mean 'X'? | 28 | F3e 已修复待合并（28 条全部是 get_strategyName；修复任务 tsgetcal（#54）由用户派出、尚未交付；判定来源为 boring docs/specs/features/27-class-members-and-records.md 规则 5 与四目标实现 boring `5628b4d`） |
| TS2451：Cannot redeclare block-scoped variable 'X'. | 23 | 已证实 ts 生成器：局部作用域复用（alpha-renaming 的 index2 计数器；原跨文件重名假设已证伪；名称分布见 7.2） |
| TS2341：Property 'X' is private and only accessible within class 'X'. | 14 | get_ 前缀 10 条属 F3e（已修复待合并）；其余 4 条已判 ts 生成器为 private 可见性过度保留；名称与类分布见 7.2 |
| TS2339：Property 'X' does not exist on type 'X'. | 14 | 已判 ts 生成器为 Haxe Array.copy 与只读数组方法在 ts 侧的映射缺失（kind 6、copy 6、push 1、insert 1，分布见 7.2） |
| TS2322：Type 'X' is not assignable to type 'X'. | 8 | 同 TS2345 组（枚举载荷判定） |
| TS2305：Module 'X' has no exported member 'X'. | 7 | compare 函数导出/导入登记缺陷（同 TS2724；模块与名称分布见 7.2） |
| TS2420：Class 'X' incorrectly implements interface 'X'. | 6 | 同 TS2345 组（枚举载荷判定） |
| TS2367：This comparison appears to be unintentional because the types 'X' and 'X' have no overlap. | 6 | 已判 ts 生成器为枚举跨构造器相等比较未降级（Haxe 允许比较不同构造器并返回 false；PushInLineWideCapacityTestSupport.ts:30 样本） |
| TS2693：'X' only refers to a type, but is being used as a value here. | 3 | 同 TS2304 的 Ic 值位使用（已判 ts 生成器） |
| TS2869：Right operand of ?? is unreachable because the left operand is never nullish. | 3 | 已判 ts 生成器为左操作数已判非空时仍保留 ??（LineRepair.ts:456 样本） |
| TS2580：Cannot find name 'X'. Do you need to install type definitions for node? | 3 | 测量环境（process 引用，tsc 未配置 node 类型）；不派修复 |
| TS2540：Cannot assign to 'X' because it is a read-only property. | 2 | 已判 ts 生成器为只读字段生成策略错误（TestTraceStore.ts:53 与 :58 的 lines 字段） |
| 合计（求和校验） | 1127 | 与错误总数相等 |

判定进度小结（tsprobe2 r1 并入后）：19 类全部有判定结论：测量环境 2 处
（TS2580 全类 3 条与 TS2307 类内 297 条）；F3e 已修复待合并（TS2551 全类
28 条加 TS2341 类内 get_ 前缀 10 条，修复任务 tsgetcal（#54）由用户派出，
不由本会话验收）；其余 16 类与 TS2307 类内 6 条全部归到 ts 生成器，涉及修复位置
文件 TsExpr.hx、TsDecl.hx、TsImports.hx、Compiler.hx（逐类机制清单见
tsprobe2 r1 报告）。下一步是把这些判定按修复位置切分成修复项并按目标优先
级派发。

### 7.2 大类内部分解（2026-09-06 实测，产出命令见第 2.2 节）

2026-09-07 复测的逐类计数与 2026-09-06 首测全部相同（第 7 节），类内分布
未重抽，下表仍标 2026-09-06 实测值；判定列已并入 tsprobe2 r1 的结论。

TS2307 的模块分布（求和 303）：

| 计数 | 模块名 | 判定 |
|---:|---|---|
| 108 | bun:test | 测量环境（测试运行器导入，tsc 未配置 bun 类型） |
| 108 | @tiqian/runtime/test | 测量环境（同上） |
| 79 | @tiqian/runtime | 测量环境（同上） |
| 3 | ./../../../runtime/SortedTable.ts | 已判 ts 生成器为导入路径问题（随第 7 节 TS2307 行） |
| 1 | ../../../../ts-gen/runtime/SortedTable.ts | 同上 |
| 1 | ../../../../ts-gen/org/tiqian/linebreak/LiangHyphenatorTest.ts | 同上 |
| 1 | ./../../../../haxe/Exception.ts | 同上 |
| 1 | node:path | 测量环境（tsc 未配置 node 类型） |
| 1 | node:fs | 测量环境（同上） |

TS2304 的名称分布（求和 183）：

| 计数 | 名称 | 判定 |
|---:|---|---|
| 103 | Ic | 已判 ts 生成器：Ic 声明只生成类型、未生成值侧声明（与 TS2693 同源） |
| 33 | kind | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 8 | TestCore | 测量环境（测试支撑名字，由运行环境提供） |
| 5 | region | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 4 | pi | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 4 | bi | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 3 | text | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 3 | __functional_shim | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 2 | SortedMap | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 2 | org | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 2 | NodeFileSystem | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 2 | count26 | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 2 | count25 | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 2 | count11 | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 1 | count | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 1 | cornerRadius | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 1 | compareShapingEvidenceKey | 已证实 ts 生成器：compare 函数导出/导入登记缺陷（同 TS2724） |
| 1 | compareRecordedShapingResult | 同上 |
| 1 | compareRecordedFontMetrics | 同上 |
| 1 | compareMetricsEvidenceKey | 同上 |
| 1 | compareGlue | 同上 |
| 1 | compareFontMetricsRequest | 同上 |

TS2448 的文件分布（求和 215）：LayoutDumpFormat.ts 214 条（行 90 至 398
间）、ShapingEvidenceJson.ts 1 条（:546）。

TS2451 的名称分布（求和 23）：index 6、_g 5、row 4、rubyIndex 2、
parseHexCode 2、inkTop 2、inkBottom 2。

TS2341 的名称与类分布（求和 14）：

| 计数 | 属性（所属类） | 判定 |
|---:|---|---|
| 5 | get_canReplayFromControlledBytes（FontBackendCapabilityReport） | F3e 已修复待合并（修复任务 tsgetcal（#54）由用户派出，不由本会话验收） |
| 2 | get_isMixed（ScriptEvidence） | 同上 |
| 1 | get_strategyName（LineBreakerCoverage2TestCustomBreaker） | 同上 |
| 1 | get_strategyName（CustomBreaker） | 同上 |
| 1 | get_capabilityReport（CatalogImpl） | 同上 |
| 1 | strongScriptRole（ContextualQuoteRoleResolver） | 已判 ts 生成器为 private 可见性过度保留 |
| 1 | pairByOpen（ContextualQuoteRoleResolver） | 同上 |
| 1 | emptyHanging（LineCandidate） | 同上 |
| 1 | codePointLengthAt（ContextualQuoteRoleResolver） | 同上 |

TS2724 的名称分布（求和 35）：compareRawFontMetrics 2 条，其余 34 个名字
各 1 条：compareSpacingDecisionInfo、compareShapingDecisionInfo、
compareRubyLineHeightDecisionInfo、compareRubyDecisionInfo、
compareRepairCandidate、comparePunctuationWidthPolicy、
comparePunctuationDecisionInfo、compareParagraphStyle、
compareMetricDecisionInfo、compareLineSpacingDecisionInfo、
compareLineRepairDecisionInfo、compareLineRepairCandidateInfo、
compareLineLengthGridDecisionInfo、compareLineEdgeTrimDecisionInfo、
compareLineCandidate、compareLayoutFontMetrics、compareLayoutConstraints、
compareKinsokuDecisionInfo、compareJustificationDecisionInfo、
compareInlineObjectSpan、compareInlineObjectPunctuationAttachmentDecisionInfo、
compareInlineObjectLineHeightDecisionInfo、compareInlineObjectDecisionInfo、
compareInlineBoxDecisionInfo、compareFontMetricsRequest、
compareFirstLineIndentDecisionInfo、compareDecorationSegmentInfo、
compareDecorationDecisionInfo、compareClusterGeometryDecisionInfo、
compareBopomofoGlyphPlacement、compareAutoSpacePolicy、
compareAutoSpaceDecisionInfo、compareAdjustmentStylePolicy。

TS2305 的模块与名称分布（求和 7）：./TextStyle.ts 的 compareTextStyle 2
条，./Size.ts 的 compareSize、./LineBox.ts 的 compareLineBox、
./InlineBoxSpan.ts 的 compareInlineBoxSpan、./GlyphRun.ts 的
compareGlyphRun、./Cluster.ts 的 compareCluster 各 1 条。

TS2339 的名称分布（求和 14）：kind 6、copy 6、push 1、insert 1。

TS2551 的名称分布（求和 28）：get_strategyName 28 条（分布在读取
strategyName 属性的测试与支撑文件）；F3e 已修复待合并（修复任务 tsgetcal
（#54）由用户派出，不由本会话验收）。

## 8 Dart 编译普查（2026-09-07 复测）

首测 2026-09-06（boring `31627b5c`、tiqian `17a646da`，工作树
/tmp/tiqian-dartic，普查日志 /tmp/dartic-census-gen.log 与
/tmp/dartic-census-tests.log，逐类表 /tmp/dartic-code-combined.txt）：
dart-gen 目录 1748 条、dart-gen-tests 目录 1183 条，合计 2931 条、35 个
错误码。2026-09-07 复测（基线 tiqian `3f609c0f` 加 boring `e01b03b3`，
工作树 /tmp/tiqian-census6，日志 /tmp/census6/dart-gen2.log 与
dart-tests2.log，逐类表 /tmp/census6/dart2-bycode.txt，产出命令见第
2.2 节）：gen 目录 1628 条、tests 目录 978 条，合计 2606 条、33 个错误
码，求和校验相等。dart 普查没有测量环境条目：运行时与测试运行器都以
产物内相对路径引用，analyzer 在生成目录自带的 pubspec.yaml 上运行，
不依赖外部类型配置。相对首测的变化：UNCHECKED_USE_OF_NULLABLE_VALUE
420→143（knullinit 系列的 dart 侧守卫，boring `9e0537d0`，合并
`23f4bf63`；property 形 314→40，见 8.2）；UNDEFINED_GETTER 32→0 与
NON_ABSTRACT_CLASS_INHERITS_ABSTRACT_MEMBER 6→0（dartifget 合并
`14c5031d`；首测预判两码随该修复消除，复测证实，两行从表内删除）；
ARGUMENT_TYPE_NOT_ASSIGNABLE 299→290（double 74→67、num 18→16，原因未单独
查明）；INVALID_ASSIGNMENT 7→3（原因未单独查明）；UNDEFINED_METHOD 64→69
上升 5 条（新增 `toDouble @ bool` 形 5 条，原因没有查明，见 8.2）。

判定来源：T-dart r1 报告 /tmp/dispatch-state/tiqian-tdart-r1.report.md
（基线 boring `31627b5c`，35 个错误码里抽了 33 类各 3 处，另两类已有
修复位置）。本文新用一档「探针定位待因果验证」：探针引用的生成器位置经
派发方在 31627b5c 检出复核确认代码存在且与抽样形状相邻，但从源构造到
错误输出的因果链还没有逐环走通；它的可信度高于形状证实、低于修复位置，
不单独构成派发依据。该报告的引用有两处经复核降级：
REFERENCED_BEFORE_DECLARATION 引用的 Compiler.hx:216-220 实为测试函数
排序，与样本不吻合；MISSING_DEFAULT_VALUE_FOR_PARAMETER 引用的
DartDecl.hx:260-300 落在比较函数合成代码内，拟合存疑。因果链闭合的
两类开列为修复项 F3l 与 F3m。修复任务 dartguard（#64）由用户派出、尚未交付，覆盖 F3l
与 F3m 的修复，不由本会话验收。

| 错误码 | gen | tests | 合计 | 判定与处置 |
|---|---:|---:|---:|---|
| `UNDEFINED_IDENTIFIER` | 444 | 581 | 1025 | 名称分布见 8.2（79 个名字全列，与首测逐项相同）；类内 8 个类型名条目合计 705 条（WritingMode 157、LastLineAlignment 155、InlineAttachment 147、Ic 106、RubyLineHeightMode 77、RubyKind 35、LineEndReason 16、FontRole 12）已判修复位置为类型名与枚举名值位引用缺导入前缀（adjustment_style_policy.dart:14 值位不带前缀直接使用 `LineEndPunctuationStyle` 实测，同文件类型位用带前缀形；机制位置 DartExpr.hx:1466-1485 的枚举引用分支与 DartImports.hx:186-215 的前缀登记，派发方在 31627b5c 复核）；F3l，修复任务 dartguard（#64）由用户派出、尚未交付；其余名字未判定 |
| `REFERENCED_BEFORE_DECLARATION` | 314 | 96 | 410 | 形状证实：抽样含导入前缀与局部名同名冲突（cluster_role_resolution.dart:55 生成 `final cluster = cluster.Cluster(...)`）与不带前缀的类名（:64 的 `ResolvedClusterRange`）两形；探针引用的 Compiler.hx:216-220 经派发方复核是测试函数排序，与样本不吻合，已否证；layout_dump_format.dart 215 条与 ts TS2448 同源的假设保留（ts 侧 TS2448 已判生成器，本码随 T-dart 第二轮复核）；文件分布见 8.2（与首测逐项相同）；机制位置待定位；随 T-dart 第二轮 |
| `ARGUMENT_TYPE_NOT_ASSIGNABLE` | 172 | 118 | 290 | 假设：T-dart 抽样为可空 int? 给 int（codeUnitAt 闭包位），连可空守卫（F3m 同构造）；目标类型分布里 double 67 条是否数值转换待抽样；分布见 8.2（23 种全列）；随 T-dart 第二轮 |
| `UNCHECKED_USE_OF_NULLABLE_VALUE` | 121 | 22 | 143 | F3m 部分：knullinit 系列的 dart 侧守卫（boring `9e0537d0`，合并 `23f4bf63`）把 property 形从 314 修到 40，残余 143 条（property 40、method 81、operator 16、condition 6，形状分布见 8.2）；修复任务 dartguard（#64）由用户派出、尚未交付 |
| `NOT_ENOUGH_POSITIONAL_ARGUMENTS` | 71 | 34 | 105 | 探针定位待因果验证：生成调用发零参而 Haxe 源构造有参（样本 PunctuationAtomBuilder.new 实参 0 个）；机制位置 DartExpr.hx:2550-2563 的 constructorArgTexts，派发方在 31627b5c 复核存在；ts 侧 TS2554 已判生成器为默认/可选参数补全机制，本码同源假设增强；随 T-dart 第二轮 |
| `PREFIX_SHADOWED_BY_LOCAL_DECLARATION` | 32 | 42 | 74 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_METHOD` | 58 | 11 | 69 | 形状证实（T-dart r1 抽样读过生成代码）；接收类型为 List 的 23 条保持 Haxe 数组方法名生成到 dart 的 List 接收者的假设；新增 `toDouble @ bool` 形 5 条的原因没有查明（gen 侧，2026-09-07 复测新增）；方法与接收类型分布见 8.2（24 对全列）；机制位置待定位；随 T-dart 第二轮 |
| `EXPECTED_TOKEN` | 56 | 0 | 56 | 形状证实：非法 `int??` 双问号类型渲染（cjk_font_role_classifier.dart:23-24 生成 `final int?? l` 实测）；与 MISSING_ASSIGNABLE_SELECTOR、ILLEGAL_ASSIGNMENT_TO_NON_ASSIGNABLE、MISSING_IDENTIFIER、DOT_SHORTHAND_MISSING_CONTEXT 四码共享同一形状，五码文件分布重叠；期待符号分布：缺分号 49、缺右括号 4、缺冒号 2、缺右花括号 1（与首测相同）；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_FUNCTION` | 55 | 0 | 55 | 探针定位待因果验证：有序表键比较函数被调用但未在目标库生成（clreq_profile.dart:79-83 调用 compareAutoSpacePolicy 等实测）；机制位置 DartDecl.hx:221-333 的数据类比较函数合成只在数据类路径，派发方在 31627b5c 复核存在；ts 侧 TS2724/TS2305 已判生成器为 compare 函数导出/导入登记缺陷，本码同源假设增强；名称分布见 8.2（42 个名字全列，与首测逐项相同）；随 T-dart 第二轮 |
| `MISSING_ASSIGNABLE_SELECTOR` | 49 | 0 | 49 | 形状证实：非法 `int??` 双问号类型渲染（与 EXPECTED_TOKEN 共享形状，五码文件分布重叠）；机制位置待定位；随 T-dart 第二轮 |
| `ILLEGAL_ASSIGNMENT_TO_NON_ASSIGNABLE` | 49 | 0 | 49 | 形状证实：非法 `int??` 双问号类型渲染（与 EXPECTED_TOKEN 共享形状，五码文件分布重叠）；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_PREFIXED_NAME` | 20 | 22 | 42 | 探针定位待因果验证：前缀引用的名字不在目标库（clreq_profile.dart:35 经前缀引用 PunctuationGluePlacements 实测）；机制位置 DartImports.hx:186-215 的前缀登记只按模块名记录，派发方在 31627b5c 复核存在；随 T-dart 第二轮 |
| `URI_DOES_NOT_EXIST` | 25 | 11 | 36 | 已判定：运行时文件、std 影子文件与 test_host.dart 未随消费方配置写出，路径明细见 8.2（与首测相同）；修复项 F3i |
| `READ_POTENTIALLY_UNASSIGNED_FINAL` | 36 | 0 | 36 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `MISSING_IDENTIFIER` | 34 | 1 | 35 | 形状证实：非法 `int??` 双问号类型渲染（与 EXPECTED_TOKEN 共享形状，五码文件分布重叠）；机制位置待定位；随 T-dart 第二轮 |
| `INVOCATION_OF_NON_FUNCTION_EXPRESSION` | 0 | 31 | 31 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `DOT_SHORTHAND_MISSING_CONTEXT` | 27 | 0 | 27 | 形状证实：非法 `??` 双问号类型渲染（annotation_geometry_stage.dart:165 生成 `ClusterGeometryDecisionInfo?? g` 实测；与 EXPECTED_TOKEN 共享形状）；机制位置待定位；随 T-dart 第二轮 |
| `DUPLICATE_DEFINITION` | 17 | 1 | 18 | 探针定位待因果验证：同库内重复名字（ic.dart:5 的 count、layout_queries.dart:641 与 :671 的 rubyIndex 实测）；机制位置 DartDecl.hx:39-40 的顶层名登记与 :625-626，派发方在 31627b5c 复核存在；ts 侧 TS2451 已判生成器为局部作用域复用，机制位置关系待证；随 T-dart 第二轮 |
| `PREFIX_COLLIDES_WITH_TOP_LEVEL_MEMBER` | 5 | 6 | 11 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `MISSING_DEFAULT_VALUE_FOR_PARAMETER` | 11 | 0 | 11 | 形状证实：非空参数的隐式默认值为 null（ClreqProfile 可选参数实测）；探针引用的 DartDecl.hx:260-300 经派发方复核落在比较函数合成代码内，与参数签名不吻合，定位存疑；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_ENUM_CONSTANT` | 8 | 0 | 8 | 形状证实：枚举构造器名空串或保留字（unicode_punctuation_boundary_resolver.dart:208-219 生成 `Dir.final` 实测，final 是 dart 保留字）；机制位置待定位；随 T-dart 第二轮 |
| `NOT_INITIALIZED_NON_NULLABLE_INSTANCE_FIELD` | 4 | 0 | 4 | 探针定位待因果验证：非空实例字段在构造器外初始化（line_breaker.dart:28 的 _kinsoku 等实测）；机制位置 DartDecl.hx:598-621 的字段默认值路径，派发方在 31627b5c 复核存在；随 T-dart 第二轮 |
| `NON_EXHAUSTIVE_SWITCH_STATEMENT` | 4 | 0 | 4 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_OPERATOR` | 3 | 0 | 3 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `INVALID_ASSIGNMENT` | 3 | 0 | 3 | 形状证实：dart 侧赋值位没有 int 到 double 的转换（line_adjustment_stage.dart:148、:151、:159 生成 `visualWidth = (visualWidth).round()` 实测，与 kotlin F3j 的赋值位同构造；首测 7 条，本次复测 3 条的下降原因未单独查明）；机制位置待定位；随 T-dart 第二轮 |
| `IMPLICIT_THIS_REFERENCE_IN_INITIALIZER` | 3 | 0 | 3 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `RETURN_OF_INVALID_TYPE` | 2 | 0 | 2 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `LIST_ELEMENT_TYPE_NOT_ASSIGNABLE` | 0 | 2 | 2 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `RETURN_OF_INVALID_TYPE_FROM_CLOSURE` | 1 | 0 | 1 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `NON_TYPE_AS_TYPE_ARGUMENT` | 1 | 0 | 1 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `INSTANCE_MEMBER_ACCESS_FROM_STATIC` | 1 | 0 | 1 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `EXTRA_POSITIONAL_ARGUMENTS` | 1 | 0 | 1 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `CONFLICTING_METHOD_AND_FIELD` | 1 | 0 | 1 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| 合计（求和校验） | 1628 | 978 | 2606 | 与错误总数相等 |

相对首测计数已降为 0 并从表内删除的错误码：UNDEFINED_GETTER（32 条，
dartifget boring `14c5031d`）、NON_ABSTRACT_CLASS_INHERITS_ABSTRACT_MEMBER
（6 条，随同修复降为 0）。

判定进度小结（2026-09-07 复测后）：修复位置 2 个整类加 1 个类内部条目：
UNCHECKED_USE_OF_NULLABLE_VALUE（F3m，残余 143，修复任务 dartguard（#64）
由用户派出、尚未交付）、URI_DOES_NOT_EXIST 36 条（F3i）、UNDEFINED_IDENTIFIER 类内 8 个
类型名条目 705 条（F3l，dartguard #64 同道）；探针定位待因果验证 5 类
（NOT_ENOUGH_POSITIONAL_ARGUMENTS、UNDEFINED_FUNCTION、
UNDEFINED_PREFIXED_NAME、DUPLICATE_DEFINITION、
NOT_INITIALIZED_NON_NULLABLE_INSTANCE_FIELD；NON_ABSTRACT 一类已随
dartifget 降为 0）；其余 25 类为形状证实或假设，其中 EXPECTED_TOKEN、
MISSING_ASSIGNABLE_SELECTOR、ILLEGAL_ASSIGNMENT_TO_NON_ASSIGNABLE、
MISSING_IDENTIFIER、DOT_SHORTHAND_MISSING_CONTEXT 五码合计 216 条共享
非法 `??` 双问号类型渲染形状，是形状证实里计数最大的一组错误码。
T-dart 第二轮的剩余工作是给 5 类探针定位待因果验证补齐因果链、给形状
证实类定位机制位置、证实或否证跨目标假设（ARGUMENT_TYPE_NOT_ASSIGNABLE
的 double 67 条是否数值转换、UNDEFINED_METHOD 新增 `toDouble @ bool` 5
条的原因查明都在其列）。

### 8.2 大类与中类内部分解（2026-09-07 实测，产出命令见第 2.2 节）

本节各表都在 2026-09-07 复测日志（/tmp/census6/dart-gen2.log 与
dart-tests2.log）上重新抽取。UNDEFINED_IDENTIFIER、
REFERENCED_BEFORE_DECLARATION、UNDEFINED_FUNCTION、EXPECTED_TOKEN、
URI_DOES_NOT_EXIST 五张表与首测逐项相同（逐表 diff 核对过）；UNCHECKED、
ARGUMENT_TYPE_NOT_ASSIGNABLE、UNDEFINED_METHOD 三张表数字更新。

UNDEFINED_IDENTIFIER 的名称分布（求和 1025；判定列写 F3l 的名字在 tiqian
源内实测为 enum 或 abstract 声明的类型名）：

| 计数 | 名称 | 判定 |
|---:|---|---|
| 157 | `WritingMode` | 修复位置为类型名值位引用缺导入前缀（F3l） |
| 155 | `LastLineAlignment` | 修复位置为类型名值位引用缺导入前缀（F3l） |
| 147 | `InlineAttachment` | 修复位置为类型名值位引用缺导入前缀（F3l） |
| 106 | `Ic` | 修复位置为类型名值位引用缺导入前缀（F3l）；与 ts TS2304 的 Ic 103 条同一原因的假设保留 |
| 77 | `RubyLineHeightMode` | 修复位置为类型名值位引用缺导入前缀（F3l） |
| 35 | `RubyKind` | 修复位置为类型名值位引用缺导入前缀（F3l） |
| 33 | `kind` | 未判定 |
| 16 | `LineEndReason` | 修复位置为类型名值位引用缺导入前缀（F3l） |
| 15 | `PunctuationGeometryStageCoverageSupport` | 未判定 |
| 13 | `JustifierTestSupport` | 未判定 |
| 12 | `plan` | 未判定 |
| 12 | `FontRole` | 修复位置为类型名值位引用缺导入前缀（F3l） |
| 12 | `FontMetricSource` | 未判定 |
| 11 | `ink` | 未判定 |
| 11 | `cls` | 未判定 |
| 10 | `item` | 未判定 |
| 9 | `x` | 未判定 |
| 8 | `previousBudget` | 未判定 |
| 8 | `nextBudget` | 未判定 |
| 8 | `g` | 未判定 |
| 6 | `r` | 未判定 |
| 6 | `n` | 未判定 |
| 5 | `prev` | 未判定 |
| 5 | `nextChar` | 未判定 |
| 5 | `mandatory` | 未判定 |
| 5 | `InteriorPunctuationStyle` | 未判定 |
| 5 | `halt` | 未判定 |
| 5 | `CjkPunctuationGlyphPolicy` | 未判定（T-dart 抽样里有值位不带前缀直接使用的形状，源侧声明核实随第二轮） |
| 4 | `shaped` | 未判定 |
| 4 | `selectedTechnicalBreak` | 未判定 |
| 4 | `previousSpacing` | 未判定 |
| 4 | `preferredTrackingSpan` | 未判定 |
| 4 | `pi` | 未判定 |
| 4 | `pairs` | 未判定 |
| 4 | `nextSpacing` | 未判定 |
| 4 | `io` | 未判定 |
| 4 | `cp` | 未判定 |
| 4 | `candidate` | 未判定 |
| 4 | `bi` | 未判定 |
| 4 | `AutoSpaceMode` | T-dart 抽样同形状（auto_space_policy.dart:14 值位不带前缀直接使用 `AutoSpaceMode.insert` 实测）；源侧声明核实与并入 F3l 随第二轮 |
| 3 | `text` | 未判定 |
| 3 | `strongReason` | 未判定 |
| 3 | `startPrior` | 未判定 |
| 3 | `role` | 未判定 |
| 3 | `prevKind` | 未判定 |
| 3 | `naturalPrior` | 未判定 |
| 3 | `LineEndPunctuationStyle` | T-dart 抽样同形状（adjustment_style_policy.dart:14 值位不带前缀直接使用实测）；源侧声明核实与并入 F3l 随第二轮 |
| 3 | `KinsokuLevel` | 未判定 |
| 3 | `fromRepair` | 未判定 |
| 3 | `firstHanging` | 未判定 |
| 3 | `endPrior` | 未判定 |
| 3 | `boundKind` | 未判定 |
| 2 | `twoPowers` | 未判定 |
| 2 | `repairStr` | 未判定 |
| 2 | `repairName` | 未判定 |
| 2 | `repairedCurrent` | 未判定 |
| 2 | `repairDecision` | 未判定 |
| 2 | `region` | 未判定 |
| 2 | `rd` | 未判定 |
| 2 | `org` | 未判定 |
| 2 | `maxLinesDecision` | 未判定 |
| 2 | `last` | 未判定 |
| 2 | `l` | 未判定 |
| 2 | `issue` | 未判定 |
| 2 | `iod` | 未判定 |
| 2 | `inkWidth` | 未判定 |
| 2 | `HangingPunctuationStyle` | 未判定 |
| 2 | `fivePowers` | 未判定 |
| 2 | `center` | 未判定 |
| 1 | `twoPowersBuilder` | 未判定 |
| 1 | `ShrinkChannel` | 未判定 |
| 1 | `RichTextBackgroundMetricPolicy` | 未判定 |
| 1 | `MetricBox` | 未判定 |
| 1 | `LineAdjustmentStrategy` | T-dart 抽样同形状（adjustment_style_policy.dart:17 值位不带前缀直接使用 `LineAdjustmentStrategy.pushInFirst` 实测）；源侧声明核实与并入 F3l 随第二轮 |
| 1 | `InlineBoxOuterSpacing` | 未判定 |
| 1 | `fivePowersBuilder` | 未判定 |
| 1 | `enUsCache` | 未判定 |
| 1 | `count` | 未判定 |
| 1 | `BaselineClass` | 未判定 |

UNCHECKED_USE_OF_NULLABLE_VALUE 的形状分布（求和 143；消息里的名字以 X
代替；knullinit 系列的 dart 侧守卫（boring `9e0537d0`，合并 `23f4bf63`）
已修掉 property 形的 314 条，残余全类的修复位置是dart 可空接收者守卫缺失
（F3m，与 kotlin 修复位置 A 同构造），修复任务 dartguard（#64）由用户派出、尚未交付）：

| 计数 | 形状 | 判定 |
|---:|---|---|
| 81 | The method 'X' can't be unconditionally invoked because the receiver can be 'null'. | F3m 部分（修复任务 dartguard（#64）由用户派出，不由本会话验收） |
| 40 | The property 'X' can't be unconditionally accessed because the receiver can be 'null'. | 同上；首测 314 条已由 boring `9e0537d0` 修复 |
| 16 | The operator 'X' can't be unconditionally invoked because the receiver can be 'null'. | 同上 |
| 6 | A nullable expression can't be used as a condition. | 同上 |

REFERENCED_BEFORE_DECLARATION 的文件分布（求和 410）：

| 计数 | 文件 | 判定 |
|---:|---|---|
| 215 | `layout_dump_format.dart` | 与 ts TS2448 的 LayoutDumpFormat.ts 214 条同一原因的假设（同一源文件的两个目标侧产物，顶层声明顺序未按依赖排序） |
| 78 | `justifier_jf_test.dart` | 未判定 |
| 64 | `justifier_coverage_test.dart` | 未判定 |
| 23 | `punctuation_geometry_stage_coverage_test.dart` | 未判定 |
| 6 | `layout_queries_test.dart` | 未判定 |
| 5 | `line_repair.dart` | 未判定 |
| 4 | `paragraph_shaping_stage.dart` | 未判定 |
| 3 | `cluster_role_resolution.dart` | 未判定 |
| 2 | `punctuation_model.dart` | 未判定 |
| 2 | `justifier_compression_test.dart` | 未判定 |
| 1 | `width_independent_annotation_cache_coverage_test_support.dart` | 未判定 |
| 1 | `unicode_emoji17_rgi_role_audit_test_support.dart` | 未判定 |
| 1 | `text_shaper.dart` | 未判定 |
| 1 | `shaping_evidence_json.dart` | 未判定 |
| 1 | `shaping_evidence.dart` | 未判定 |
| 1 | `punctuation_geometry_stage.dart` | 未判定 |
| 1 | `line_optimization_coverage_test.dart` | 未判定 |
| 1 | `annotation_geometry_stage_coverage_test_support.dart` | 未判定 |

ARGUMENT_TYPE_NOT_ASSIGNABLE 的目标类型分布（求和 290，24 行全列；消息
形如 The argument type 'X' can't be assigned to the parameter type 'Y'，
本表按 Y 计；T-dart 抽样为可空 int? 给 int，连可空守卫 F3m；double 67 条
是否数值转换待第二轮抽样；相对首测 double 74→67、num 18→16，下降原因未单独
查明）：

| 计数 | 目标类型 | 判定 |
|---:|---|---|
| 70 | `List<Cluster>` | 未判定 |
| 67 | `double` | 未判定 |
| 40 | `List<EastAsianSpacingEdges>` | 未判定 |
| 37 | `int` | 未判定 |
| 24 | `String` | 未判定 |
| 16 | `num` | 未判定 |
| 8 | `List<String>?` | 未判定 |
| 6 | `Cluster` | 未判定 |
| 5 | `SortedSetTable<int>` | 未判定 |
| 3 | `SortedMapTable<String, double>` | 未判定 |
| 2 | `KinsokuLevel` | 未判定 |
| 1 | `UnbreakableRanges` | 未判定 |
| 1 | `SortedMapTable<TextRange, SortedSetTable<int>>` | 未判定 |
| 1 | `SortedMapTable<TextRange, ClusterMetricDecision>` | 未判定 |
| 1 | `SortedMapTable<String, String>` | 未判定 |
| 1 | `SortedMapTable<int, ProgressiveBreakOpportunity>` | 未判定 |
| 1 | `SortedMapTable<int, InlineObjectSpan>` | 未判定 |
| 1 | `SortedMapTable<int, InlineObjectPreferredStretch>` | 未判定 |
| 1 | `List<ShrinkOpportunity>` | 未判定 |
| 1 | `List<int>` | 未判定 |
| 1 | `List<Glyph>` | 未判定 |
| 1 | `Iterable<int>` | 未判定 |
| 1 | `HangingPunctuationStyle` | 未判定 |

UNDEFINED_METHOD 的方法与接收类型分布（求和 69，24 对全列，记法为方法 @
接收类型；相对首测新增 `toDouble` @ `bool` 形 5 条，gen 侧，原因没有查明）：

| 计数 | 方法与接收类型 | 判定 |
|---:|---|---|
| 13 | `Cluster` @ `Function` | 未判定 |
| 10 | `emptyF` @ `PunctuationGeometryLedger` | 未判定 |
| 6 | `copy` @ `List` | Haxe 数组方法名生成到 dart 的 List 接收者的假设 |
| 5 | `toDouble` @ `bool` | 未判定（2026-09-07 复测新增形，原因没有查明） |
| 5 | `concat` @ `List` | Haxe 数组方法名生成到 dart 的 List 接收者的假设 |
| 4 | `splice` @ `List` | Haxe 数组方法名生成到 dart 的 List 接收者的假设 |
| 4 | `pop` @ `List` | Haxe 数组方法名生成到 dart 的 List 接收者的假设 |
| 3 | `Cluster` @ `Cluster` | 未判定 |
| 2 | `shift` @ `List` | Haxe 数组方法名生成到 dart 的 List 接收者的假设 |
| 2 | `Ic` @ `Function` | 未判定 |
| 2 | `emptyHanging` @ `LineCandidate` | 未判定 |
| 1 | `unshift` @ `List` | Haxe 数组方法名生成到 dart 的 List 接收者的假设 |
| 1 | `reverse` @ `List` | Haxe 数组方法名生成到 dart 的 List 接收者的假设 |
| 1 | `Rect` @ `Rect` | 未判定 |
| 1 | `Glyph` @ `Glyph` | 未判定 |
| 1 | `compareTo` @ `RubyLineHeightDecisionInfo` | 未判定 |
| 1 | `compareTo` @ `MaxLinesDecisionInfo` | 未判定 |
| 1 | `compareTo` @ `LineSpacingDecisionInfo` | 未判定 |
| 1 | `compareTo` @ `LineRepairDecisionInfo` | 未判定 |
| 1 | `compareTo` @ `LineLengthGridDecisionInfo` | 未判定 |
| 1 | `compareTo` @ `LineCandidate` | 未判定 |
| 1 | `compareTo` @ `KinsokuDecisionInfo` | 未判定 |
| 1 | `compareTo` @ `InlineObjectLineHeightDecisionInfo` | 未判定 |
| 1 | `compareTo` @ `FirstLineIndentDecisionInfo` | 未判定 |

UNDEFINED_FUNCTION 的名称分布（求和 55，消息全部为 The function 'X'
isn't defined.）：

| 计数 | 名称 | 判定 |
|---:|---|---|
| 7 | `floatToI32` | 未判定（Haxe 浮点位转换函数名，tiqian 源 org/tiqian/test/TestHelpers.hx 等处使用） |
| 5 | `i32ToFloat` | 未判定（Haxe 浮点位转换函数名，tiqian 源 org/tiqian/test/TestHelpers.hx 等处使用） |
| 2 | `compareTextStyle` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 2 | `compareFontMetricsRequest` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 2 | `compareRawFontMetrics` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareAutoSpacePolicy` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareAdjustmentStylePolicy` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `comparePunctuationWidthPolicy` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareBopomofoGlyphPlacement` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareMetricDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareClusterGeometryDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareAutoSpaceDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareRubyDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareShapingDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `comparePunctuationDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareSpacingDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareJustificationDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareLineEdgeTrimDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareDecorationDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareDecorationSegmentInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareInlineBoxDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareInlineObjectDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareInlineObjectPunctuationAttachmentDecisionInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareParagraphStyle` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareLayoutConstraints` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareInlineBoxSpan` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareInlineObjectSpan` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareSize` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareCluster` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareGlyphRun` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareLineBox` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareLineRepairCandidateInfo` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareLayoutFontMetrics` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareLineCandidate` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareRepairCandidate` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareGlue` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareShapingEvidenceKey` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareRecordedShapingResult` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareMetricsEvidenceKey` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `compareRecordedFontMetrics` | 有序表键比较函数未导出假设（同 ts TS2724） |
| 1 | `mkdirSync` | 未判定 |
| 1 | `writeFileSync` | 未判定 |

EXPECTED_TOKEN 的期待符号分布（求和 56）：';' 49 条、')' 4 条、':' 2 条、
'}' 1 条（全部未判定）。

URI_DOES_NOT_EXIST 的引用路径分布（求和 36；目录列 gen 指 dart-gen 内
的文件、tests 指 dart-gen-tests 内的文件；已判定 F3i）：

| 计数 | 引用路径 | 目录 |
|---:|---|---|
| 5 | `../../../std/u_string_fault.dart` | gen |
| 5 | `../../../std/u_string_exception.dart` | gen |
| 4 | `../../../../std/u_string_fault.dart` | gen |
| 4 | `../../../../std/u_string_exception.dart` | gen |
| 4 | `../../../../dart-gen/lib/std/u_string_fault.dart` | tests |
| 4 | `../../../../dart-gen/lib/std/u_string_exception.dart` | tests |
| 3 | `../../../runtime/sorted_table.dart` | gen |
| 2 | `../../../std/sorted_map.dart` | gen |
| 1 | `test_host.dart` | tests |
| 1 | `../../../std/functional.dart` | gen |
| 1 | `../../../../haxe/exception.dart` | gen |
| 1 | `../../../../dart-gen/lib/runtime/sorted_table.dart` | tests |
| 1 | `../../../../dart-gen/lib/org/tiqian/linebreak/liang_hyphenator_test.dart` | tests |

## 9 分级标尺

复杂度（C）：C1 单点修复，一个生成器分支或一处源文件，改动预计不超过一百行；
C2 跨文件或跨目标，同一缺陷出现在多个目标，或需要 tiqian 源与 boring 生成器
配合；C3 新机制，需要新增降级能力。

优先级（P）：P0 阻塞项，阻塞后续测量或行为对齐验收；P1 大错误种类或已在修复
计划内的排队项；P2 影响总数但不阻塞测量；P3 单例且暂无复现路径。

严重性（S）：S0 错误出在语法层，使整个生成目录无法编译或无法生成；S1 两百条
以上；S2 二十到一百九十九条；S3 二十条以下。流程类条目不适用 S，记为 S-。

## 10 KPI

工作量检验的方式（2026-09-06 用户裁定）：进度以各目标逐类表的行计数变化
为准，每类可单独复测；不设覆盖多类的「其余」聚合指标，聚合数只保留合计
一个完整性数字（各类求和必须等于合计）。修复项只对修复位置开，不对消息类
主题开；每条修复项写明覆盖的错误类、类内条目与计数，KPI 现值写各目标
逐类表的实测计数，切分反映修复工作量，不合并修复位置。

| KPI | 指标 | 现值 | 目标 | 对应 |
|---|---|---|---|---|
| K1 | 修复位置 A：only safe 类计数 | f32 3 / f64 3（knullinit 系列合并 `23f4bf63` 后的残余） | 0 | #22 残余 |
| K2 | 各目标逐类判定完成度 | kotlin 35 合并行中 32 行有判定（修复位置 21、探针定位待因果验证 3、形状记录（连锁伪影）1、假设 4、测量环境 1、f64 独有已判 2；未判定 3；tattr3 r1 并入后形状证实类全部升级或归档）；rust 7 / 7 类（rustf4c r2）；ts 19 / 19 类（tsprobe2 r1；含测量环境 2 处）；dart 33 类中修复位置 3 类加类内 705 条、探针定位 5 类、其余 25 类形状证实或假设；swift tests 侧 6 / 6 类（tswift1 r1：消费侧配置 2 类、修复位置 F3t 覆盖 4 类） | 五目标全部类有判定结论 | T-attr、T-swift、T-dart、tsprobe2、rustf4c r2 |
| K3 | 各目标错误总数 | kotlin f32 696 / f64 717；swift gen 1 加 tests 57（每精度同值；tests 内消费侧配置 41、引擎侧 16）；rust 20（每精度同值）；ts 1127（引擎侧 819）；dart 2606 | 全部 0 | 各节逐类表求和（完整性数字，非派发单位） |
| K4 | kotlin 两个精度目录 warning 计数 | 0 / 0 | 保持 0 | 每次复测 |
| K5 | 各目标重生成退出码 | 八个生成入口全部 0（2026-09-07，/tmp/census6/report.txt：kotlin、swift、rust 各 f32 与 f64，ts、dart） | 全部 0 | 逐处重跑阶段（已完成，第 4 节） |
| K6 | boring 验收命令 | 2026-09-06 合并 `f9f26726` 与 `31627b5c` 后 19 项检查退出码全部为 0（当时 bun test 672 pass / 3 fail，三个既有名目）；其后至 `e01b03b3` 的 8 次合并只经各修复任务报告内的验收命令，19 项检查未在合并后统一重跑（我还没有验证） | 每次合并后保持 | 不适用 |
| K7 | 五目标普查覆盖 | 八格矩阵逐类表已建（kotlin、swift、rust 各 f32 与 f64 加 ts、dart，第 3、5、6、7、8 节） | 五目标各有逐类表 | 首次普查记录（第 12 节进度表） |

## 11 修复项清单

本清单只留未完成条目。已完成并在新基线复测确认的条目自 2026-09-07 起从
本清单删除，只在第 12 节进度表留行；条目编号保留不复用（第 1 节更新规则）。
已删除的条目：第 0 组 F0a-F0d、第 0.5 组 F0e 至 F0l（含 F0i-逐处重跑与
F0k-核）、第 2 组 F2b、F3f、F3g、F3h、F3k、T-ts、第 4 组 F4a 至 F4c、
第 1 组 T-swift（tswift1 r1 交付，判定并入第 5 节，F3t 与 F3u 随之开列）；
各条的提交号与复测计数移入第 12 节对应行。

### 第 1 组：判定探针（K2，先于其余修复任务的派发）

- [ ] T-attr kotlin 逐类判定探针（第二轮）：r1 于 2026-09-06 交付（报告
      /tmp/dispatch-state/tiqian-tattr-r1.report.md，34 节全量），抽样形状
      已并入 3.2 与 3.3 节；r2 于 2026-09-07 交付（报告
      /tmp/dispatch-state/boring-tattr2-r2.report.md），unresolved reference 相关的十七类
      全量判定并入 3.4 节。剩余范围是把 3.2 节 14 类形状证实定位到文件与
      分支、证实或否证 6 类假设、判定 3 个未判定类（collection literals、
      array literals、selector）、把 3.3 节 argument 类内可空 82、Number
      装箱 14、其余 33 归到修复位置（knulljud r1 只交付了按错误类汇总的预览计数，逐
      形状判定未并入）、对 3.4 节 pow 2 与 NodeFileSystem 2 两条补因果
      验证。C2，P1，S-。
- [ ] F1a 修复位置 A 计数降为 0（#22 残余）：knullinit 系列合并 `23f4bf63`
      后残余 only safe 类 f32 3 / f64 3、operator call prohibited 类
      f32 6 / f64 6。C2，P1，S3。
- [ ] T-dart dart 逐类判定探针（第二轮）：r1 于 2026-09-06 交付（报告
      /tmp/dispatch-state/tiqian-tdart-r1.report.md，33 类全量），抽样与
      生成器引用经派发方在 31627b5c 复核后并入第 8 节，F3l、F3m 随之
      开列，两处不吻合的引用已降级并记录在第 8 节判定来源段。修复任务
      dartguard（#64）由用户派出、尚未交付，覆盖 F3l 与 F3m 的修复，不由本会话验收。第二轮范围是给 5 类探针定位待因果验证补齐因果链
      （NOT_ENOUGH_POSITIONAL_ARGUMENTS、UNDEFINED_FUNCTION、
      UNDEFINED_PREFIXED_NAME、DUPLICATE_DEFINITION、
      NOT_INITIALIZED_NON_NULLABLE_INSTANCE_FIELD）、给形状证实类定位
      机制位置、证实或否证跨目标假设（ARGUMENT_TYPE_NOT_ASSIGNABLE 的
      double 67 条与 UNDEFINED_METHOD 新增 `toDouble @ bool` 5 条的原因查明
      在其列）。C2，P1，S-。

### 第 3 组：按判定结果立项

本组条目按修复位置开列：一条修复项对应一个修复位置，附它覆盖的错误类与
类内条目清单、计数、判据（对应计数降为 0 且其余类计数不上升）。条目的
判定来源写明：kotlin 侧由 T-attr 产出；swift 与 ts 侧的下列三条来自各
目标首次编译普查时已可定性的错误类。2026-09-06 撤销原第 3 组 F3a（修饰符
主题）、F3b（语句位置主题）、F3c（推断与重载主题）、F3d（桶 4 可空形状）
四条按消息主题合并的条目；各错误类已在 3.2 节逐行可见，其中可空形状与
数值形状的疑议随 T-attr 判定。撤销理由：按主题把多个修复位置合并成一个
任务后，进度勾选无法与任何一个错误类的计数核对。2026-09-06 T-attr r1
与 T-dart r1 的判定并入后，本组新增 F3j 至 F3m 四条：F3j、F3k 来自
kotlin 侧（机制由派发方在 boring `4b1fec9` 复核后开列），F3l、F3m 来自
dart 侧（引用由派发方在 boring `31627b5c` 复核后开列）。2026-09-07 复测
后，F3f 与 F3g（合并 `e01b03b3`）、F3h（合并 `14c5031d`）、F3k（合并
`ad0990e2`，cannot access 残余 3 / 3 即判据写明的基数）完成并删除；
rustf4c r2 判定并入第 6 节后新增 F3n 至 F3s 六条；tswift1 r1 判定并入
第 5 节后开列 F3t 与 F3u；tattr3 r1 判定并入第 3.2 节后开列 F3v 至 F3ag
十二条。

- [ ] F3e ts 目标 getter-only 属性调用点：状态为已修复待合并（修复任务
      tsgetcal（#54）由用户派出，不由本会话验收；第 7 节计数待合并复测后
      降为 0）。覆盖 TS2551 全类 28 条（全部
      get_strategyName）加 TS2341 的 get_ 前缀 10 条，合计 38 条。修复
      面为 ts 生成器实例成员读取与零参调用两条路径上补 getter-only 属性
      判定，参照 dart 的 DartExpr.hx:1516 与 :2267 两处及 boring `5628b4d`
      在 kotlin、swift、rust 的对应实现（boring 仓库
      docs/specs/features/27-class-members-and-records.md 规则 5）。判据为
      TS2551 计数与 TS2341 的 get_ 前缀条目计数降为 0，其余类计数不上升。
      C1，P1，S2。
- [ ] F3i dart 目标运行时与 std 影子文件的写出：覆盖 URI_DOES_NOT_EXIST
      全类 36 条（路径明细见第 8.2 节：runtime/sorted_table、
      std/sorted_map、std/functional、std/u_string_exception、
      std/u_string_fault、haxe/exception、tests 侧的 test_host 与跨目录
      测试引用）。判定来源为2026-09-06 磁盘对照：boring 自身样本生成树
      含 lib/std、lib/haxe 与 test_host.dart，tiqian 消费树 out/ 下这些
      文件均不存在而生成代码以相对路径引用它们。修复位置为 dart 目标这些
      文件在消费方配置下的写出条件。判据为 URI_DOES_NOT_EXIST 计数降为
      0，其余类计数不上升。C2，P1，S2。
- [ ] F3j kotlin 数值转换位扩展（残余）：knumconv-r3（boring `8aa72e16`，
      合并 `7b3135a1`）已修掉主体；残余为 operator applied 全类（f32 2 /
      f64 2）、return 全类（17 / 17）、initializer f64 侧（0 / 2）、
      assignment 类内待再分形状（6 / 10，数值形状已修、残余疑为可空）与
      3.3 节 Int 给浮点（6 / 13）、Long 给浮点（0 / 2）两形状。修复位置为
      KotlinExpr.hx 的 renderCallArgs（boring `4b1fec9` 时位于 2806-2820，
      派发方复核）只在实参位对 isIntType 的接收值插入
      `(x).toFloat()/.toDouble()` 转换，运算、return、默认值、赋值四个
      位置没有同等转换，isIntType 也不覆盖 Long；修复方案是把这四个位置与
      Long 纳入同一转换机制。判据为 operator applied、return、initializer
      三类与 Int 给浮点、Long 给浮点两形状计数降为 0，assignment 剩余
      条目全部为可空形状，其余类计数不上升。C2，P1，S2。
- [ ] F3l dart 类型名与枚举名值位引用缺导入前缀：修复任务 dartguard（#64）
      由用户派出，不由本会话验收。覆盖 UNDEFINED_IDENTIFIER
      类内 8 个类型名条目合计 705 条（WritingMode 157、LastLineAlignment
      155、InlineAttachment 147、Ic 106、RubyLineHeightMode 77、RubyKind
      35、LineEndReason 16、FontRole 12，分布见 8.2）。修复位置为
      DartExpr.hx:1466-1485 的枚举引用分支对值位引用输出不带前缀的名字、
      DartImports.hx:186-215 的前缀登记没有覆盖这类引用
      （adjustment_style_policy.dart:14 值位不带前缀直接使用 `LineEndPunctuationStyle`
      实测，同文件类型位用带前缀形；两处位置派发方在 31627b5c 复核）。
      判据为上述 8 个名字的计数降为 0，其余名字计数不上升。C2，P1，S1。
- [ ] F3m dart 可空接收者守卫缺失（残余 143 条）：knullinit 系列的 dart
      侧守卫（boring `9e0537d0`，合并 `23f4bf63`）已把 property 形从 314
      修到 40；残余覆盖 UNCHECKED_USE_OF_NULLABLE_VALUE 全类 143 条
      （property 40、method 81、operator 16、condition 6，分布见 8.2）。
      修复任务 dartguard（#64）由用户派出，不由本会话验收。修复位置为 dart
      生成器对可空接收者的成员访问、调用、运算与条件位没有生成守卫
      （clreq_punctuation_advance_policy.dart:27 的 int? 接收者直接比较
      实测；DartExpr.hx:1315-1341 的 binop 守卫位派发方在 31627b5c
      复核），与 kotlin 修复位置 A（#22）同构造。判据为该错误码计数降为
      0，其余类计数不上升。C2，P1，S2。
- [ ] F3n rust switch 臂语句位渲染：覆盖第 6 节 match 臂语句位 10 条与
      layout_debug_assembly 形 1 条。修复位置为 rust 生成器 switch 臂渲染把
      臂模式前置于语句（RustExpr.hx 的 switch 分支约 2600-2690；样本
      layout_queries.rs:379、383、387 与 annotation_geometry_stage.rs:483、
      487、491、496、539、552、557，layout_debug_assembly.rs:176:161）。
      判据为两类计数降为 0，其余类计数不上升。C2，P1，S2。
- [ ] F3o rust 静态成员 INSTANCE 重名：覆盖第 6 节 defined multiple
      times 4 条。修复位置为 rust 静态成员生成在单文件多类布局下重名
      （RustDecl.hx 的 static 约 1000-1250 与 impl 约 2000 起；
      rich_text_role.rs:36、62、120、146 四处 `pub static INSTANCE`）。
      判据为该类计数降为 0，其余类计数不上升。C1，P1，S3。
- [ ] F3p rust 标识符转换产出空标识符：覆盖第 6 节 expected identifier
      2 条。修复位置为 RustImports 的 toSnakeCase 返回空名
      （display_glyph_substitution_engine_test_support.rs:12:56 与
      text_shaper.rs:4:56）。判据为该类计数降为 0，其余类计数不上升。
      C1，P2，S3。
- [ ] F3q rust u_string 模块文件未写出：覆盖第 6 节 file not found for
      module 1 条。修复位置为 u_string 模块文件未随消费方配置写出
      （Compiler.hx 约 347 与 RustRuntime.hx 约 320；runtime/mod.rs:4
      声明 `pub mod u_string;` 但无对应文件）。判据为该类计数降为 0，
      其余类计数不上升。C2，P1，S3。
- [ ] F3r rust 超长 format! 链递归超限：覆盖第 6 节 recursion limit 1 条。
      修复位置为超长 format! 链的宏递归超限（RustExpr.hx 约 2458-2478 与
      4117-4165；inline_object_decision_info.rs:78）。判据为该类计数降为
      0，其余类计数不上升。C1，P2，S3。
- [ ] F3s rust 链式比较未降级：覆盖第 6 节 comparison operators cannot
      be chained 1 条。修复位置为链式比较未降级为 `a < b && b < c`
      （punctuation_geometry_ledger.rs:293:30）。判据为该类计数降为 0，
      其余类计数不上升。C1，P2，S3。
- [ ] F3t swift 字符串插值内嵌语句体闭包：覆盖第 5 节 tests 侧
      static methods 13、unterminated string 1、extraneous 1、string
      interpolation 1，每精度 16 条（f32 与 f64 同值）。修复位置为 swift
      生成器字符串拼接合成把语句体闭包字面量嵌进插值段：实测样本是
      多语句表达式降级成的闭包（ExplainableStubParagraphLayoutEngineTest.swift:69，
      `let from`/`let to` 两条声明在 `\(...)` 段内）；SwiftExpr.hx
      stdStringType 的 IsArray/IsSortedSet/IsSortedMap 分支（e01b03b3
      :1824-1836）生成同类闭包，构成第二条触发路径。两条路径出自同一
      修复位置（插值段不得含语句体闭包），修复需同时覆盖；正确输出先把
      闭包结果绑定到局部常量再插值。判据为四类计数降为 0，其余类计数
      不上升（合并测量下 gen 侧语义错误不上升）。C2，P1，S2。
- [ ] F3u swift 测试树 import 头的消费侧配置：覆盖第 5 节 tests 侧
      cannot find 32 与 contextual type 9，每精度 41 条（合并测量下这类错误数为 0，
      证明符号本身可解析）。修复位置为 tiqian 的
      engine-haxe/targets/swift-common.hxml 补 `-D swift-test-import=<模块名>`
      （并与 package-shell 配置一起评估，当前为 none；boring 合同见
      examples/swift.hxml:20-21 与 Compiler.hx:422-425）。判据为生成的
      tests 文件带 import 头且 SwiftPM 构建下测试目标符号解析通过。
      C1，P2，S-。
- [ ] F3v kotlin std 数组 indexOf 可选参合成 null 未丢弃：覆盖 3.2 节 too
      many arguments 全类 14/14。修复位置为 KotlinExpr.hx call() 的通用实例
      调用分支（:2973）与 renderCallArgs（:3052-3071）：typer 为
      `?fromIndex` 合成的 null 实参在 String 接收者的专支（:2910-2911）
      被丢弃，List/Array 接收者没有同等处理，std Array.indexOf 也未在
      DefaultArgExpander 注册默认值（PreparedParagraph.kt:50
      `fs!!.indexOf(f, null)` 实测，tattr3 r1）。判据为该类计数降为 0，
      其余类计数不上升。C1，P1，S2。
- [ ] F3w kotlin 同函数局部变量名不去重：覆盖 3.2 节 conflicting
      declarations 全类 13/13。修复位置为 KotlinExpr.hx localName
      （:3299-3306）只对 `` ` `` 与 `_` 生成避让名、对 typer 展开数组推导
      产出的 `_g` 等名字原样输出，且 kotlin 目标 Compiler.hx:72
      `preventRepeatVars: false` 关闭了 reflaxe 的 RepeatVariableFixer
      （PreparedParagraphJfTest.kt:116/126 两个 `val _g` 实测，tattr3 r1）。
      判据为该类计数降为 0，其余类计数不上升。C2，P1，S2。
- [ ] F3x kotlin 静态方法值缺 callable reference：覆盖 3.2 节 function
      invocation 全类 8/8（其中 1 条为 Array.copy 未降级产生的连锁错误，随 F3ag 消除）。
      修复位置为 KotlinExpr.hx field() 的 FStatic 分支（:2184-2187）对函数
      类型位置的静态成员返回 `Class.method` 文本，需要 `Class::method` 或
      lambda 包装（ContextualQuoteRoleResolverNestedAndSurrogateTest.kt:84、
      ParseTexHyphenationPatterns.kt:55 实测，tattr3 r1）。判据为该类计数
      降为 0，其余类计数不上升。C1，P1，S2。
- [ ] F3y kotlin 数组下标访问两条降级缺失：覆盖 3.2 节 no operator array
      access 全类 6/6（set 形 4：`split` 经 KotlinExpr.hx:2973 产出只读
      List 后下标写，PreparedParagraph.kt:1726 起；get 形 2：
      stringBufMutationLines :698 文本拼接 `part + "[0].code"` 未给 part
      加括号，TracedAssertions.kt:116 实测，tattr3 r1）。判据为该类计数
      降为 0，其余类计数不上升。C2，P2，S2。
- [ ] F3z kotlin 测试 lambda 内无标签 return：覆盖 3.2 节 prohibited here
      全类 5/5。修复位置为 KotlinDecl.hx testFuncDecl（:1152-1177）把测试
      函数体包进非 inline 的 `Test.run { … }`，而 KotlinExpr.hx stmtLines
      无实参 TReturn 分支（:557）输出不带标签的 `return`（BilingualEmphasisTest.kt:16
      实测，tattr3 r1）。判据为该类计数降为 0，其余类计数不上升。
      C1，P2，S3。
- [ ] F3aa kotlin @JvmField 未排除 private：覆盖 3.2 节 jvmField 全类
      5/5。修复位置为 KotlinDecl.hx objectVarDecl（:1009）对非 final 静态
      字段无条件生成 `@JvmField`，未考虑可见性（PreparedParagraph.kt:25
      实测，tattr3 r1）。判据为该类计数降为 0，其余类计数不上升。
      C1，P2，S3。
- [ ] F3ab kotlin when 臂分组多值丢弃：覆盖 3.2 节 exhaustive 全类 4/4
      与 variable must be initialized 全类 3/3（同一机制连锁，分支缺臂
      路径无赋值）。修复位置为 KotlinExpr.hx switchExpression（:1345-1349）
      只渲染每个 case 的 `c.values[0]`，Haxe typer 归并的 `case A | B:`
      其余值被丢弃（FontMetrics.kt:15 与 :21 实测，tattr3 r1）；
      KotlinDecl.hx collectMessageCases（:672 起）同形，需同步检查。
      判据为两类计数降为 0，其余类计数不上升。C1，P1，S3。
- [ ] F3ac kotlin hashCode 与异常载荷 message 缺 override：覆盖 3.2 节
      hides member 全类 2/2。修复位置为 KotlinDecl.hx funcDecl（:1095-1099）
      的 overridesAny 只认零参 toString，与 sealedExceptionDecl（:603-604）
      载荷字段与基类 `override val message` 同名时无 override
      （BopomofoReading.hx:27 手写 hashCode、TraceAssertionException 载荷
      message 实测，tattr3 r1）。判据为该类计数降为 0，其余类计数不上升。
      C1，P2，S3。
- [ ] F3ad kotlin Haxe 文件私有类映射缺失：覆盖 3.2 节 redeclaration
      全类 2/2。修复位置为 KotlinDecl.hx classDecl（:207）对一切类无条件
      生成 top-level `class`，Haxe 文件私有类（仅本文件可见）需要 Kotlin
      private top-level 映射（ContextualQuoteRoleResolver.hx:316 与
      ContextualDashEllipsisRoleResolver.hx:223 各一个 `private class
      Resolution`，生成后同 package 冲突，tattr3 r1）。判据为该类计数
      降为 0，其余类计数不上升。C2，P2，S3。
- [ ] F3ae kotlin 下划线参数名原样输出：覆盖 3.2 节 reserved 全类 1/1。
      修复位置为 KotlinDecl.hx parameterText（:873）与 lambda 参数渲染只调
      KotlinNameEscape.escape（:42-43 只给关键字加反引号），`_` 参数需要
      与函数体局部一致的生成名路径（UnicodePunctuationBoundaryTestSupport.kt:118
      实测，tattr3 r1）。判据为该类计数降为 0，其余类计数不上升。
      C1，P2，S3。
- [ ] F3af kotlin @:access 与模块可见性无映射：覆盖 3.2 节 cannot access
      全类 3/3（kgetvis 合并 `ad0990e2` 后的残余基数）。修复位置为
      KotlinDecl.hx funcDecl（:1103）与 objectVarDecl（:1008）的可见性
      选择只把 `:allow` 转 internal，`@:access` 授权类跨类访问与 Haxe
      同模块文件私有访问没有对应映射（LineOptimizationCoverageTest.kt:151、
      ContextualQuoteRoleResolver.kt:293 起实测，tattr3 r1）。判据为该类
      计数降为 0，其余类计数不上升。C2，P2，S3。
- [ ] F3ag kotlin Array.copy 无降级：直接覆盖 3.2 节 ambiguous 全类 1/1
      （根本原因：LineRepair.kt:38 `initial.copy()` 为 Haxe Array.copy 走
      KotlinExpr.hx:2973 通用实例调用，Kotlin List 无 copy 成员，同函数
      其后错误全为连锁错误，tattr3 r1）；同时预期消减 cannot infer 行的连锁
      部分与 function invocation 行的 `range()` 连锁条目（消减量在合并
      复测后重数，不预先承诺）。判据为 ambiguous 计数降为 0，其余类计数
      不上升。C1，P2，S3。

## 12 进度记录

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
| 2026-09-06 | KPI 重切：按修复位置与判定覆盖取代主题合并，撤销 F3a-d | 本文档第 9、10 节 | 判定进度见 3.2 节小结 | 不适用 |
| 2026-09-06 | rust 首次编译普查（F4a）与 kotlin `'X' cannot be a callee` 类判定 | 本文档第 2.2、3.2、6 节 | rust 181 条、15 类，求和校验相等；kotlin 该类 f32/f64 各 1 条，为异常子类第二代 super 误入 init 块 | 不适用 |
| 2026-09-06 | ts 第二处生成阻断 UStringException 值引用修复（修复任务 ustr） | boring `f270c670`（合并 `de11c06a`，已推送） | ts 生成退出码 0，首次全量生成 | 19 项验收检查退出码全部为 0；合并后 boring main 的 bun test 670 pass / 3 fail（三个既有名目） |
| 2026-09-06 | getter 属性读四目标调用点修复（修复任务 getprop） | boring `5628b4d`（合并 `f9f26726`，已推送） | 一致性检查 342 个测试六目标一致（含新增的 PublicGetterPropertyTests） | 19 项验收检查退出码全部为 0 |
| 2026-09-06 | ts 首次编译普查（F4b 的 ts 半项）；swift 与 ts 并入 KPI；文档条目全量列举与格式统一 | 本文档第 1、2.2、3.4、4、7 至 11 节 | ts 1127 条、19 类，求和校验相等；测量环境 308 条、引擎侧 819 条；dart 第一处更新为 Units.hx 静态重名（F0l） | 不适用 |
| 2026-09-06 | F0l dart 静态成员顶层重名修复（修复任务 dartic）与 dart 首次编译普查（F4b dart 半项）；dart 并入 KPI | boring `189e01ad`（合并 `31627b5c`，已推送） | dart 生成退出码 0；dart analyze 报 2931 条、35 类，求和校验相等；无测量环境条目 | 19 项验收检查在修复工树与合并工树各全部退出码 0；合并后 bun test 672 pass / 3 fail（三个既有名目） |
| 2026-09-06 | rust 普查复测（基线推进到 boring `75b08ed2`）；补两条 rust 测量错误记录 | 本文档第 2.2、2.3、6 节 | rust 180 条、15 类，求和校验相等；相对 4b1fec9 首测的 181 条少一条保留字转义类（`type` 6 处降 5 处）；浮点字面量类 124 条不变 | 不适用 |
| 2026-09-06 | T-attr r1 与 T-dart r1 判定并入普查文档；新增修复项 F3j、F3k、F3l、F3m | 本文档第 3.2、3.3、3.4、8、8.2、10、11 节 | 判定进度见 3.2 与第 8 节小结；错误计数未复测，基线不变 | 不适用 |
| 2026-09-06 | 补记已完成并自第 11 节删除的条目：F0a-F0d nullargs（boring `8d17b59`，null 字面量错误 2122→0，合并后普查 f32 1535 / f64 1553）、F0g 变体 switch 赋值位（ts `73c076d8`、swift `ab1d7882`，样本 `2b0189c9`）、F0h 表达式位块 features/43（boring `6251842`）、F0k Std.string 第二批（tiqian `26d89566`；裁定解除 `04777e8b`）、F0i 第一处 std/Type extern（boring `47d6cea`，合并 `a0b7416e`）与逐处重跑收尾 | 见左列 | 效果已含在 r4 与本次基线数字内 | nullargs 19 项验收检查退出码 0；F0k 合并后六个生成命令实测，rust 退出码 0 |
| 2026-09-06 | 原第 13 节背景条目并入本表后该节删除：names-r2 第一批（unresolved 347→290）、单变体异常折叠回归（boring `a75601a`）、record 接口字段打印与 Rust derive（boring `5d7417e`）、Dart Math.min/max 调用点降级补臂、knarrow null 初始化位（boring `4b1fec9`，52→0）、onlysafe 上半（tiqian `3ce6f511`，only safe 95→75）、sbuf-bind（tiqian `0b152313`，stringbuffer 表达位错误→0） | 见左列 | 效果已含在各基线数字内 | 不适用 |
| 2026-09-07 | knullinit 系列合并（kotlin 可空接收者守卫与 dart 侧同机制守卫） | boring `3d397740`、`9e0537d0`、`b552e2a1`（合并 `23f4bf63`） | kotlin only safe 75→3、operator call prohibited 60→6；dart UNCHECKED 420→143（property 314→40） | 修复任务报告内验收命令通过；合并后 19 项检查未整体重跑（我还没有验证） |
| 2026-09-07 | kparamnull 合并（kotlin 可选参数 null 默认值） | boring `1f35923d`（合并 `2b78ab4b`） | 效果含在第 3.1 节本次基线内，原因未逐类查明 | 同上 |
| 2026-09-07 | knumconv-r3 合并（kotlin 数值加宽转换扩展，F3j 主体） | boring `8aa72e16`（合并 `7b3135a1`） | operator applied 35/32→2/2、return 28/28→17/17、initializer 15/17→0/2、assignment 19/23→6/10、Int 给浮点 80/88→6/13 | 同上 |
| 2026-09-07 | kgetvis 合并（kotlin getter 可见性与 override 组合，F3k 关闭） | boring `6f3bc868`（合并 `ad0990e2`） | modifier incompatible 14→0、cannot weaken 7→0、cannot access 27→3（3 即判据写明的私有成员跨作用域基数） | 同上 |
| 2026-09-07 | dartifget 合并（dart 接口 getter 声明缺失，F3h 关闭） | boring `0d470797`（合并 `14c5031d`） | dart UNDEFINED_GETTER 32→0、NON_ABSTRACT_CLASS_INHERITS_ABSTRACT_MEMBER 6→0 | 同上 |
| 2026-09-07 | rustflit 合并（rust 浮点字面量缺整数部分） | boring `4deac694`（合并 `d870489c`） | rust 浮点字面量类 124→0 | 同上 |
| 2026-09-07 | rustf4c 合并（rust 保留字转义；r2 探针判定 7 类，F4c 关闭、F3n 至 F3s 开列） | boring `dc09d773`＋`e4c24e77`（合并 `782526d7`）；r2 报告 /tmp/dispatch-state/boring-rustf4c-r2.report.md | rust 保留字转义类 12→0；本次复测 20 条、7 类全部有修复位置判定 | 同上 |
| 2026-09-07 | swiftrem-r2 合并（swift 浮点字面量与控制字符转义，F3f、F3g 关闭） | boring `4c81c5fc`（合并 `e01b03b3`） | swift gen 侧浮点 7 条与控制字符 1 条均→0 | 同上 |
| 2026-09-07 | 八格矩阵重测与新判定并入（tattr2 r2、tsprobe2 r1 即 T-ts 关闭、f64only r1 即 F2b 关闭、knulljud r1 的按类汇总预览、rustf4c r2）；第 10、11、12 节按 2026-09-07 用户裁定改为删除制 | 本文档第 3、5、6、7、8、10、11 节 | kotlin f32 696 / f64 717；swift gen 1 加 tests 57（每精度）；rust 20（每精度）；ts 1127（与首测逐类相同）；dart 2606、33 类 | 重生成八入口退出码 0；各逐类表求和校验相等 |
| 2026-09-07 | tswift1 r1 探针交付（swift tests 六类判定）＋合并测量补进配方 | 本文档第 2.2、2.3、5、10、11 节 | tests 57 分解为消费侧配置 41 与引擎侧 16（F3t 连锁四类）；早先把合并 16 条记为截断系误判，已更正 | 不适用（判定轮，无代码改动） |
| 2026-09-07 | tattr3 r1 探针交付（kotlin 十二类升修复位置、假设检验、连锁错误归类）＋修复项 F3v 至 F3ag 开列 | 本文档第 3.2、3.4、10、11 节 | 修复位置 9→21 类、形状证实 14→0（12 类升修复位置、1 类判为连锁伪影、1 类升探针定位）；探针定位 3 类随 #23；pow 与 NodeFileSystem 因果结论在探针轮结束消息里（报告文件未及更新，锚点未存档） | 不适用（判定轮，无代码改动；探针工树 git status 干净核实，报告引用的生成器行号逐处复核） |
