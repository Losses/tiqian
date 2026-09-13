# 五目标生成代码编译错误普查与修复追踪

本文记录 engine-haxe 生成代码在五种输出语言（Kotlin、TypeScript、Rust、Swift、
Dart）下的编译错误，作为跨目标行为对齐（见
[cross-target-alignment.md](cross-target-alignment.md)）的修复计划与进度追踪。
普查对象是 engine-haxe/out/ 下由 boring 从 Haxe 源码翻译出的目标语言代码目录。
原生 `engine` 模块的 `./gradlew :engine:jvmTest` 无失败，不在本文范围内。

当前状态（2026-09-10 傍晚，基线 tiqian `3f609c0f` 加 boring `a2f2bdc2`，主树直接复测）：八个生成命令退出码全部为 0。kotlin f32 176 条 / f64 187 条；ts 337 条；rust f32 3402 条 / f64 3174 条；swift gen 侧 f32 0 条 / f64 1 条、tests 侧 f64 41 条；dart 317 条（生成侧 269 条、测试侧 48 条）。两日累计：总错误 6546 降至 4394（kotlin 减半、dart tests 减 88%、ts 减半）。已解决并复测确认的修复项自 2026-09-07 起从第 11 节清单删除，只留第 12 节进度行。

## 1 更新规则

- 每个修复项只有一个复选框。满足以下四条后视为完成：修复已合入 boring main
  （写提交号）或 tiqian（写提交号）；vendored 副本已推进并重新生成相关目录；
  该错误种类在相关目录的计数都是 0；boring 的验收命令全部通过。完成并在新
  基线复测确认后，把该修复项从第 11 节清单删除，只在第 12 节进度记录表留
  一行（2026-09-07 用户规定，取代此前「勾选后保留记录」的做法）；修复项
  编号保留不复用。
- 一次修复只允许对应一个修复项；不允许一次核销多项，也不允许提前核销。
- 计数必须来自本文「测量配方」一节的命令输出，不允许凭印象填写。
- 错误按消息骨架逐行记录，每一类一行（2026-09-06 用户规定：不设「未归类」
  「其余」聚合行，折叠会让任务量检验失真）。新暴露的错误类在逐类表加新行，
  不允许并入既有行，也不允许并入任何聚合行。每张逐类表必须附求和校验，各类
  计数之和等于总数。
- 大类内部的分解同样逐条列出：名称、模块、文件等维度的分布写全每一个名字
  与计数，不设「前 N 位」的截断，也不设把多个名字合并成一行的聚合行
  （2026-09-06 用户规定：条目折叠省略会让工作量检验失真）。
- 修复项与 KPI 的切分必须反映修复工作量：每条修复项写明它覆盖的错误类、
  类内条目与计数，KPI 的现值写各目标逐类表的实测计数；不把多个修复位置合并
  成一个指标（2026-09-06 用户规定）。
- 每次修复合入 boring main 后就重测受影响目标的目录：只要计数相对上一
  基线下降，立即更新对应逐类表与第 10 节 KPI 现值，并向用户交前后对照表；
  不必等计数降为 0，也不必等修复项整体完成（2026-09-07 用户规定）。
- 第 12 节进度表与第 11 节修复历程不复述提交内容：哪个提交改了哪个文件、
  哪个分支、加了几行，经提交号用 git show 得知，文档不重复叙述
  （2026-09-08 用户规定）。进度表修复项栏写修复项编号、修复名与覆盖的
  错误类；修复历程括注写轮次、提交号与提交号无法得知的事实（计数、
  验收结果、判定、规定、失败的定性、未提交事件）。提交号本身可以出现在
  任意栏。

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
  -d engine-haxe/out/tiqian-f32.jar 2> engine-haxe/out/census-f32.log
$KOTLINC -Xallow-kotlin-package \
  $(find engine-haxe/out/kotlin-gen-f64 engine-haxe/out/kotlin-gen-f64-tests -name "*.kt") \
  -d engine-haxe/out/tiqian-f64.jar 2> engine-haxe/out/census-f64.log
grep -a -c " error: " engine-haxe/out/census-f32.log
grep -a -c " error: " engine-haxe/out/census-f64.log'

# 逐类枚举（第 3.2 节逐类表的产出命令）。消息骨架指把消息里的具体
# 标识符与数字替换成占位符后得到的模板：单引号内的标识符替换为 'X'，
# 数字串替换为 N，重复的枚举项收敛。输出每一行是一个错误类与其计数；
# 两列各自求和必须等于上面的总数，此校验缺失的表无效。
for LOG in engine-haxe/out/census-f32.log engine-haxe/out/census-f64.log; do
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
for LOG in engine-haxe/out/census-f32.log engine-haxe/out/census-f64.log; do
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
grep -a " error: unresolved reference" engine-haxe/out/census-f32.log \
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
计数。同精度的 gen 与 tests 两目录没有同名文件，可以合并进一次 swiftc
调用；合并测量用于区分消费侧配置与引擎侧错误：

```shell
cd engine-haxe/out
for D in swift-gen-f64 swift-gen-f64-tests; do
  find "$D" -name '*.swift' | sort | xargs swiftc -typecheck > "sw-$D.log" 2>&1
  echo "$D rc=$?"; grep -a -c ': error:' "sw-$D.log"
done   # f32 侧同形，目录名换 swift-gen-f32 与 swift-gen-f32-tests

# 合并测量（诊断用，不替代逐目录计数）：同精度 gen 与 tests 合并后跨目录
# 符号可解析；语法解析错误还有剩余时语义检查不完整，语义错误仍以逐目录计数为准
find swift-gen-f64 swift-gen-f64-tests -name '*.swift' | sort | xargs swiftc -typecheck > sw-combined-64.log 2>&1; echo rc=$?
grep -a -c ': error:' sw-combined-64.log   # f32 侧同形为 sw-combined-32.log
```

boring 遇到尚未实现生成规则的 Haxe 构造时，在第一处这样的构造上报错并中止。
四个目标的生成命令退出码均已为 0，四者的编译普查命令都已记在本节。

TypeScript 重生成与编译普查：

```shell
# 重生成
nix develop -c bash -c 'printf "%s" "$PWD/.haxelib/boring/git" > .haxelib/boring/.dev; haxe engine-haxe/targets/ts.hxml; echo "TS_RC=$?"'

# 编译普查。tsc 用 tiqian 根目录 node_modules 里的副本；有错误时
# 退出码非 0 是预期，错误计数来自日志
cd engine-haxe/out && nix develop -c bash -c 'bun ../../node_modules/typescript/bin/tsc --noEmit --allowImportingTsExtensions --module esnext --moduleResolution bundler --target es2022 $(find ts-gen ts-gen-tests -name "*.ts") > audit-tsc.log 2>&1; echo "TSC_RC=$?"; grep -c "error TS" audit-tsc.log'

# 逐类枚举（第 7 节逐类表的产出命令）。tsc 的诊断首行含 error TSNNNN，
# 续行不含，计数只取首行。骨架化在去掉行首文件位置后的消息段上进行：
grep -a 'error TS' audit-tsc.log | sed 's/^.*error //' \
  | sed -E "s/'[^']*'/'X'/g" \
  | sed -E 's/\{[^{}]*\}/{…}/g' | sed -E 's/\{[^{}]*\}/{…}/g' \
  | sed -E 's/<[^<>]*>/<…>/g' | sed -E 's/<[^<>]*>/<…>/g' \
  | sed -E 's/[0-9]+/N/g' \
  | sort | uniq -c | sort -rn
# 求和校验：上式输出第一列求和必须等于错误总数

# 类内分解（第 7.2 节的产出命令）：对指定错误码按名字、模块名或文件抽取
# 计数，下面以 TS2304 的名称分布为例，其余错误码的抽取规则见第 7.2 节
grep -a 'error TS2304' audit-tsc.log | sed -E "s/.*Cannot find name '([^']+)'.*/\1/" | sort | uniq -c | sort -rn
```

带类型配置的 ts 编译普查：为消除测量环境中的类型未定义缺失，在
`engine-haxe/out` 放置 `tsconfig.json` 配置类型定义与模块映射，使用 `-p` 运行：

```json
{
  "compilerOptions": {
    "noEmit": true,
    "allowImportingTsExtensions": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "target": "es2022",
    "baseUrl": ".",
    "paths": {
      "@tiqian/runtime": ["ts-gen/runtime.ts"],
      "@tiqian/runtime/test": ["ts-gen/runtime/test.ts"]
    },
    "typeRoots": ["../../node_modules/@types"],
    "types": ["node", "bun"]
  },
  "include": ["ts-gen/**/*.ts", "ts-gen-tests/**/*.ts"]
}
```

```shell
cd engine-haxe/out && nix develop -c bash -c 'bun ../../node_modules/typescript/bin/tsc -p tsconfig.json > ts-typed.log 2>&1; echo "TSC_RC=$?"; grep -c "error TS" ts-typed.log'
# 逐类枚举与类内分解的管道与上方首测块相同，日志名换 ts-typed.log
```

rust 的编译普查命令如下：

```shell
# Cargo.toml 由 boring Compiler.hx 在 PackageShell 启用时写进输出目录本身
# （targets/rust-f64.hxml 的 -D rust-output 指到 .../rust-gen-f64/src），
# cargo 从该目录运行。退出码 101 是预期，错误计数来自日志；必须带
# --message-format=short，原因见 2.3 节
cd engine-haxe/out/rust-gen-f64/src && cargo check --message-format=short > ../../r64.log 2>&1; echo rc=$?

grep -a -c ": error" engine-haxe/out/r64.log   # 错误总数

# 逐类枚举（第 6 节逐类表的产出命令）。骨架化规则与 Kotlin 相同：引号与
# 反引号内的文本替换为占位符、重复枚举项收敛、数字替换为 N；另把
# error[E####] 收敛为 [E]
grep -a ": error" engine-haxe/out/r64.log | sed 's/^.*: error//' \
  | sed 's/^\[E[0-9]*\]:/[E]:/' \
  | sed 's/`[^`]*`/`X`/g' \
  | sed "s/'[^']*'/'X'/g" \
  | sed 's/\(, `X`\)\{2,\}/, `X`…/g' \
  | sed 's/[0-9]\+/N/g' \
  | sort | uniq -c | sort -rn
# 求和校验：上式输出第一列求和必须等于错误总数
```

Dart 重生成与编译普查：

```shell
# 重生成
nix develop -c bash -c 'printf "%s" "$PWD/.haxelib/boring/git" > .haxelib/boring/.dev; rm -rf engine-haxe/out/dart-gen engine-haxe/out/dart-gen-tests; haxe engine-haxe/targets/dart.hxml; echo DART_RC=$?'

# 编译普查。有错误时退出码非 0 是预期，错误计数来自日志；两个目录分别运行
cd engine-haxe/out/dart-gen && dart analyze --format=machine > ../dart-gen.log 2>&1; echo rc=$?; grep -c "^ERROR" ../dart-gen.log
cd engine-haxe/out/dart-gen-tests && dart analyze --format=machine > ../dart-tests.log 2>&1; echo rc=$?; grep -c "^ERROR" ../dart-tests.log

# 逐类枚举（第 8 节逐类表的产出命令）。机器格式为
# 级别|类别|错误码|文件|行|列|长度|消息，错误码取第 3 列；WARNING 与 INFO
# 不计入；gen 与 tests 两目录合并计数，keys 收两侧并集
for side in gen tests; do awk -F'|' -v s=$side '/^ERROR/{print $3"\t"s}' engine-haxe/out/dart-$side.log; done \
  | awk -F'\t' '{if($2=="gen") g[$1]+=1; else t[$1]+=1; keys[$1]=1} END{for (k in keys) printf "%s\t%d\t%d\t%d\n", k, g[k]+0, t[k]+0, g[k]+t[k]+0}' \
  | sort -t$'\t' -k4 -rn
# 求和校验：上式输出合计列求和必须等于错误总数

# 类内分解（第 8.2 节各表的产出命令）
cat engine-haxe/out/dart-gen.log engine-haxe/out/dart-tests.log | awk -F'|' '/^ERROR/ && $3=="UNDEFINED_IDENTIFIER" {msg=$8; if (match(msg, /Undefined name '''[^''']*'''\.'''/)) print substr(msg, RSTART+16, RLENGTH-18)}' | sort | uniq -c | sort -rn
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
  （2026-09-06 实测：同一份生成目录，默认形按该锚点数得 0，short 形数得
  180）。普查命令必须带 `--message-format=short`。
- rust 的默认多行输出里，`grep -c "^error"` 会把末尾的汇总行 `error:
  could not compile ... due to N previous errors` 也计入（同一份生成目录 180 条
  诊断加 1 行汇总数得 181）；short 形没有汇总行，不存在这个问题。
- tsc 与 dart analyze 的诊断都打在 stdout。把 stdout 重定向到
  /dev/null（`> /dev/null 2> log`）会得到空日志而退出码仍非 0，两个目标
  被计为 0 条错误（2026-09-07 实测，ts 与 dart 两格一度误记 0）。普查命令
  必须写 `> log 2>&1`。
- swift 同精度的 gen 与 tests 两目录没有同名文件（2026-09-07 comm 实测），
  合并调用可行。2026-09-07 早先把合并调用只得 16 条记为 filename used
  twice 截断是误判：16 条是跨目录符号解析后剩下的语法解析错误数（第
  5 节）。同名文件冲突出现在 f32 与 f64 两个精度目录之间（同名文件集
  完全相同），跨精度不能合并。
- swiftc 在语法解析错误还有剩余时语义检查不完整：合并测量里 gen 侧
  BopomofoParser.swift 的 optional 解包错误缺席，而逐目录 gen 单独测量
  该错误在（实测对照逐目录测量日志与合并测量日志）；语义错误计数以逐目录为准。
- rust 当前全部错误在解析层（rustc 未开始类型检查）时，cargo check 约 1
  秒返回，属正常；完整性以日志末行 `due to N previous errors` 的 N 与计数
  相等核对（2026-09-07 两侧 N=20 均核过）。
- 早于 tiqian `3f609c0f`（生成入口拆进 engine-haxe/targets/）的消费方检出里，
  生成入口仍是旧名 `engine-haxe/core-<目标>.hxml`；在这些检出上测量时以该检出
  实际文件名为准。
- `_GeneratedFiles.txt` 末行没有换行符，`wc -l` 比 `grep -c .` 少计 1 行
  （2026-09-07 实测：swift gen 清单 wc -l 391、grep -c . 392）；产物文件数
  以 `find <两个产物目录> -type f | wc -l` 为准，该命令把清单文件本身也计入
  （392 个生成文件加清单等于 393）。

## 3 Kotlin 普查结果

### 3.1 基线快照

- 测量基线：tiqian `3f609c0f`（本地 main）加 boring `e01b03b3`，2026-09-07 实测。vendored 副本指向 `.haxelib/boring/git` 里的 `e01b03b3`。
- f32 目录（out/kotlin-gen-f32 与 kotlin-gen-f32-tests）696 条错误；f64
  目录（out/kotlin-gen-f64 与 kotlin-gen-f64-tests）717 条错误；两目录
  warning 计数均为 0。
- 基线血统：`a75601a` 首测 f32 3338 / f64 3358（`5d7417e` 上 3335/3355）；
  `8d17b59`（nullargs 合入）1535 / 1553；tiqian `105dfb30` 加 boring
  `4b1fec9` 1183 / 1201；本节基线 696 / 717；kf3v-r1 合并后 `b7054019`
  680 / 701（too many arguments 14/14 随 F3v 降 0 删除，）；knamefix-r8 合并后
  `3044bf91` 343 / 354（census11，两列类集首次完全相同，各 33 类；逐类
  降量、新类引入路径与未查明项见 3.2 表行内说明）；`a72f994d` 343 / 354
  （census12，见下）。旧分桶表 28 桶 2026-09-06 起作废，由第 3.2 节
  逐类表取代。
- 2026-09-08 census12 复测（boring `a72f994d`，vendored 推进到该提交，
  tiqian 主树 `4bfe817f` 重生成，退出码 0）：f32 343 / f64 354、warnings 0/0，与 census11 总数持平
  （`3044bf91..a72f994d` 区间 dartargs-r2 的 kotlin 侧改动不改变消费树
  计数）；逐类枚举本轮未做，3.2 表数值仍标 census11 复测。

### 3.2 逐类全表（2026-09-08 census11 复测；f32 与 f64 各 33 类非零，两列类集相同）

「判定」列的含义：一个错误类的修复位置（boring 生成器的机制位置，或
tiqian 源的一类写法）已经探针证实时，记修复位置；只有猜测记「假设」；都没有
记「未判定」。判定的方法见 T-attr（第 11 节第 1 组）：对类内抽样错误点读
生成代码后定性。假设不构成派发依据，证实后才开修复项。本轮新增一档
「形状证实」：探针读过该类抽样错误点的生成代码、确认错误落在生成形状上，
但还没有把生成器机制定位到文件与分支；它的可信度高于假设、低于修复位置，
不单独构成派发依据。本文用到两个修复位置名：修复位置 A 指可空接收者上方法调用
的生成机制；数值转换面指数值类型互转的生成机制。

判定来源：探针 T-attr r1（基线 boring `4b1fec9`，采信其抽样形状）；tattr2 r2（基线 boring `d870489c`，unresolved reference 相关十七类全量判定）；f64only r1（f64 独有类的根本原因）；tattr3 r1（十四类机制定位与假设检验，生成器行号在 `e01b03b3` 检出逐处复核）。数值转换面
（KotlinExpr.hx:2806-2820 的 renderCallArgs 只在实参位插入转换）与 getter
降级可见性（`5628b4d` 生成 `private override`）两处机制由派发方在 4b1fec9
检出复核。

| 错误消息类（骨架） | f32 | f64 | 判定与处置 |
|---|---:|---:|---|
| argument type mismatch: actual type is 'X', but 'X' was expected. | 131 | 138 | 形状分解见 3.3；Int 给浮点 12/12 是 F3j 残余（形状明细见 3.3）；可空 81、Number 装箱 14、其余 24/31 待证（knulljud r1 只交付了按错误类汇总的预览计数，逐形状判定未并入本表） |
| unresolved reference 'X'. | 54 | 54 | knamefix-r8 合并后的残余（365/358→54/54）；符号分布与归属见 3.4（11 个符号 Σ=57，含 operator 形 3 条）；修复任务 knamefix（#23）由本会话派出 |
| 'X' cannot be reassigned. | 24 | 24 | 新类（census11 引入，knamefix-r8）；引入路径已判定（两树同位对照）：knamefix-r8 的 counted-loop 识别改动把 `b7054019` 树的 `var i = 1; while (…)` 形改写成 `for (i in …)` 形时没有检查循环体是否写计数器（LineRepair.kt:38 加 :44 的 `i++`、LayoutQueries.kt:283 的 `index += 1` 实测；24 条分布 LineRepair.kt 12、PunctuationGeometryStage.kt 6、LayoutQueries.kt 2、其余四文件各 1）；修复位置（tattr4 r1）：循环识别的区间谓词 PolicyQueries.hx intervalCore（:800-868）与 intervalShort（:888-930）匹配 while 形计数循环时只识别声明、条件与尾部自增，不检查循环体其余位置写计数器，KotlinExpr.hx:867-870 matchInterval 命中后 :966-1000 输出只读 for 绑定（LineRepair.kt:39-44 实测）；修复为两谓词加体写检查（赋值与自增形、递归嵌套语句）返回 null 保留 while 形。F3au |
| return type mismatch: expected 'X', actual 'X'. | 17 | 21 | F3j 残余；f64 由 17 升 21 的原因没有查明 |
| none of the following candidates is applicable: | 12 | 12 | 探针定位待因果验证（tattr3 r1）：`+` 两侧为 `Number & Comparable<…>` 装箱与具体 Float 时 kotlinc 列出全部候选（PunctuationGeometryLedger.kt:36、:40、:43 实测；机制位置 KotlinExpr.hx binopCore :1996 起）；与 unresolved for operator、modifier required 两行同源（装箱值仍是 Number，没有转换成具体数值类型），随 #23；11→12 的原因没有查明 |
| conflicting declarations: | 11 | 11 | 修复位置（tattr3 r1）：typer 把数组推导 `[for …]` 展开成构建器局部 `_g`，kotlin 目标 Compiler.hx:72 的 `preventRepeatVars: false` 关闭 reflaxe 的 RepeatVariableFixer，KotlinExpr.hx localName :3299-3306 对这类展开名原样输出不做兄弟去重（只对 `` ` `` 与 `_` 生成避让名）→ 同函数两个 `val _g`（PreparedParagraphJfTest.kt:116 与 :126、LayoutQueries.kt:588 起实测；tattr3 r1 加临时 trace 实验证实展开名到达生成器，实验改动已还原）。F3w；13→11 的降幅归 knamefix-r8，未逐条核对 |
| type mismatch: inferred type is 'X', but 'X' was expected. | 8 | 8 | 假设（T-attr 抽样含异常第二代嵌套类引用 LayoutQueries.kt:390 `TiqianNoSuchElementException.Message`，疑与 #37 异常子类是同一机制）；形状证实（tattr4 r1）：抽样全部是异常构造位 `throw IllegalStateException` 与 Throwable 期望不符（DisplayGlyphSubstitutionEngineTestSupport.kt:81 等七处、TextShaper.kt:118），第二代异常引用这一组错误；声明侧分支未隔离；11→8 的降幅归 knamefix-r8，未逐条核对 |
| function invocation 'X' expected. | 7 | 7 | 修复位置（tattr3 r1）：静态方法作值使用时 KotlinExpr.hx field() 的 FStatic 分支 :2184-2187 经 staticRef 返回 `Class.method` 文本，函数类型位置需要 callable reference `Class::method` 或 lambda 包装（ContextualQuoteRoleResolverNestedAndSurrogateTest.kt:84 `Support.surrogateText`、ParseTexHyphenationPatterns.kt:55 `SortedTable.compareStrings` 实测）；8→7 即 F3ag 连锁条目 `range()` 随 knamefix-r8 消除。F3x |
| operator call is prohibited on a nullable receiver of type 'X'. Use 'X'-qualified call instead. | 6 | 6 | 修复位置 A 残余；#22 残余 |
| no 'X' operator method providing array access. | 6 | 6 | 修复位置（tattr3 r1）：set 形 4 条为 `split` 走通用实例调用分支 KotlinExpr.hx:2973 产出只读 `List`，随后下标写 `a[i] = …` 无 set（PreparedParagraph.kt:1726/1729/1741/1745）；get 形 2 条为 stringBufMutationLines :698 以文本拼接 `part + "[0].code"` 取首字符，part 为 `"" + values[i]` 时 `[0]` 绑到 `values[i]`（TracedAssertions.kt:116）。F3y |
| cannot infer type for type parameter 'X'. Specify it explicitly. | 6 | 6 | 假设部分否证（tattr3 r1）：42→6 的降幅与 F3ag 连锁消除同现（unresolved `copy` 消失后每行字段访问带的 cannot infer 连锁随之消失），tattr3 r1 的连锁判断成立；残余 6 条形状证实（tattr4 r1）：全部是 `SortedTable.mapBuilder(compareStrings)` 调用走通用调用渲染（KotlinExpr.hx:2362-2397）、未走 :3133 的类型化 map-builder 分支，K 与 V 无法推断（ParseTexHyphenationPatterns.kt:50-51 实测）；修复分支未隔离 |
| assignment type mismatch: actual type is 'X', but 'X' was expected. | 6 | 6 | F3j 数值形状已修（19/23 降 6/10）；f64 由 10 降 6 归 knamefix-r8；剩余条目待按可空形状再判 |
| 'X' is prohibited here. | 5 | 5 | 修复位置（tattr3 r1）：KotlinDecl.hx testFuncDecl :1152-1177 把测试函数体包进非 inline 的 `Test.run { … }` lambda，KotlinExpr.hx stmtLines 的无实参 TReturn 分支 :557 输出不带标签的 `return`，该位置禁止（BilingualEmphasisTest.kt:16、BopomofoLayoutTest.kt:24/96/118 实测）。F3z |
| jvmField has no effect on a private property. | 5 | 5 | 修复位置（tattr3 r1）：KotlinDecl.hx objectVarDecl :1009 对一切非 final 静态字段无条件生成 `@JvmField`，未排除 private（PreparedParagraph.kt:25/27/29/31、EnglishHyphenation.kt:4 实测）。F3aa |
| 'X' expression must be exhaustive. Add the 'X', 'X'… branches or an 'X' branch. | 4 | 4 | 修复位置（tattr3 r1）：Haxe typer 把 `case A | B:` 归并为一个 case 多值，KotlinExpr.hx switchExpression :1345-1349 只渲染 `c.values[0]`，其余值丢弃 → when 缺臂（FontMetrics.kt:15、ParagraphLayoutEngine.kt:158 实测；tattr3 r1 用 `-D dump=pretty` 实验证实归并形状）。F3ab |
| the feature "collection literals" is experimental and should be enabled explicitly. This can be done by supplying the compiler argument 'X', but note that no stability guarantees are provided. | 4 | 4 | 形状证实（tattr4 r1）：与 array literals、selector 两码为同一生成表达式 `lineExtras?.[i]`（可空接收者加数组下标，KotlinExpr.hx:1126 数组访问位）的三个解析视角，四条错误点全部同构（RubyLayoutTest.kt:26-27、LineAdjustmentStage.kt:79 两条、PreparedParagraph.kt:339）；修复分支待隔离 |
| the expression cannot be a selector (cannot occur after a dot). | 4 | 4 | 形状证实（tattr4 r1）：与 collection literals 行同一表达式 `?.[i]` 的第二个诊断视角（见该行）；f64 由 5 降 4 归 knamefix-r8 |
| receiver type 'X' contains star projection which prohibits the use of 'X'. | 4 | 4 | 假设：Number 装箱，与 none of candidates 行同一 Number 装箱机制（tattr4 r1 逐点证实）：`Number & Comparable<*>` 装箱值参与比较与 Float 调用（LineRepair.kt:187-190 `shrink > (0).toFloat()` 实测）；修复分支未隔离 |
| only safe (?.) or non-null asserted (!!.) calls are allowed on a nullable receiver of type 'X'. | 4 | 4 | 修复位置 A 残余（3→4 的原因没有查明）；#22 残余 |
| array literals outside of annotations are unsupported. | 4 | 4 | 形状证实（tattr4 r1）：与 collection literals 行同一表达式 `?.[i]` 的第二个诊断视角（见该行） |
| variable 'X' must be initialized. | 3 | 3 | 修复位置（tattr3 r1）：与 exhaustive 行同一机制（switchExpression 丢分组值）的连锁报错，分支缺臂路径无赋值，definite-assignment 无法证明初始化（FontMetrics.kt:21/52、ParagraphLayoutEngine.kt:164 实测）。F3ab |
| unresolved reference 'X' for operator 'X'. | 3 | 3 | 探针定位待因果验证（tattr3 r1）：SortedMapTable get 返回的 `Number` 装箱值参与 `-` 运算，`!!` 后仍为 Number（PunctuationGeometryLedger.kt:36 实测；机制位置 KotlinExpr.hx binopCore :1996 起）；与 none of candidates、modifier required 两行同源（装箱值仍是 Number，没有转换成具体数值类型），随 #23 |
| cannot access 'X': it is private in 'X'. | 3 | 3 | 修复位置（tattr3 r1）：kgetvis 合并 `ad0990e2` 后的残余 3 条，机制为 KotlinDecl.hx funcDecl :1103 与 objectVarDecl :1008 的可见性选择只把 `:allow` 转 internal，Haxe 的 `@:access` 授权与同模块文件私有跨类访问无映射（emptyHanging 于 LineOptimizationCoverageTest.kt:151、`@:access` 的 codePointLengthAt 与 strongScriptRole 于 ContextualQuoteRoleResolver.kt:293 起实测）。F3af |
| 'X' hides member of supertype 'X' and needs an 'X' modifier. | 2 | 2 | 修复位置（tattr3 r1）：KotlinDecl.hx funcDecl :1095-1099 的 overridesAny 只认零参 toString，手写 `hashCode` 与异常载荷 `message` 不在内（BopomofoReading.hx:27 手写 `public function hashCode():Int` 于 @:dataClass 类、TraceAssertionException 载荷 `val message` 与 sealedExceptionDecl :603-604 基类 `override val message` 冲突实测；早把本类记成 dataClass 合成 hashCode 缺 override 不属实，已更正）。F3ac |
| redeclaration: | 2 | 2 | 修复位置（tattr3 r1）：tiqian 两个模块各有一个 Haxe 文件私有类 `private class Resolution`（ContextualQuoteRoleResolver.hx:316、ContextualDashEllipsisRoleResolver.hx:223），KotlinDecl.hx classDecl :207 无条件生成 top-level `class`，同 package 冲突（ContextualDashEllipsisRoleResolver.kt:241 与 ContextualQuoteRoleResolver.kt:311 实测；早把本类记成嵌套类冲突不属实，已更正）。F3ad |
| 'X' modifier is required on 'X'. | 1 | 1 | 探针定位待因果验证（tattr3 r1）：Number 装箱值参与比较运算，kotlinc 要求 compareTo 的 operator 修饰（Justifier.kt:85 `d > 0` 实测，d 为装箱）；与 unresolved for operator、none of candidates 两行同源（装箱值仍是 Number，没有转换成具体数值类型），随 #23；9/10→1/1 归 knamefix-r8 |
| 'X' cannot be a callee. | 1 | 1 | 修复位置为 KotlinDecl 异常子类第二代的生成规则：super 调用被放进 init 块（IllegalStateException.kt:5:5，该类计数 1 即全部样本）；与 #37 同构造不同目标，修复项在 #37 完成后按修复位置开列 |
| Unexpected tokens (use 'X' to separate expressions on the same line). | 1 | 1 | 新类：Justifier.kt:151:2 的语法错误，与 overload resolution ambiguity 行的位点（Justifier.kt:149:21）同函数相邻，形状证实（tattr4 r1）：sumOf 块 lambda 表达式渲染残留（`}.toDouble() }.toFloat()` 续接，Justifier.kt:149-151 实测），与 overload resolution ambiguity 行同一位点；修复分支待隔离 |
| overload resolution ambiguity between candidates: | 1 | 1 | 新类（旧 ambiguous 类随 F3ag 消除）：Justifier.kt:149:21，与 modifier required 行的装箱 Number 位点（Justifier.kt:85）同文件；形状证实（tattr4 r1）：`ops.sumOf { … .toDouble() }.toFloat()` 的数值多重载无法解析（Justifier.kt:149-151 实测），装箱数值机制；修复分支未隔离 |
| operator 'X' cannot be applied to 'X' and 'X'. | 1 | 1 | F3j 残余 |
| null cannot be a value of a non-null type 'X'. | 1 | 1 | 形状证实（tattr4 r1）：String.lastIndexOf 单参调用降级时对非空 Int 形参插入 null 第二参（QuoteClassificationEngineTestSupport.kt:203-207 实测）；修复分支待隔离 |
| names _, __, ___, ... are reserved in Kotlin. | 1 | 1 | 修复位置（tattr3 r1）：KotlinDecl.hx parameterText :873 与 lambda 参数渲染只调 KotlinNameEscape.escape（:42-43，只给关键字加反引号），`_` 参数名原样输出（UnicodePunctuationBoundaryTestSupport.kt:118 `resolve(_: LayoutProfileId)` 实测）；函数体局部的 `_` 已有生成名路径（localName :3304），参数位没有。F3ae |
| condition type mismatch: inferred type is 'X' but 'X' was expected. | 1 | 1 | 形状证实（tattr4 r1）：可空接收者安全调用 `.has(...)` 返回 Boolean? 直接作条件（LineAdjustmentStage.kt:597 `if ((rejectedForSpan?.has(...)))` 实测），修复位置 A 延伸维持；修复分支未隔离 |
| 合计（求和校验） | 343 | 354 | 与 3.1 节 census11 总数相等 |
相对上一表（`b7054019`）计数已降为 0 并从表内删除的类：Expecting an
element（0/9）、both main（0/2）、initializer type mismatch（0/2），三类为
f64only r1 判定的 f64 浮点尾点路径，随 knamefix-r8 降 0；this declaration
needs opt-in（1/1，测量环境条目）与 method 'X' is ambiguous（1/1，F3ag）
随 knamefix-r8 降 0，F3ag 按删除制完成（进度见第 12 节）。更早降 0 删除的
类见第 12 节进度表（modifier incompatible、cannot weaken、smart cast、
for-loop non-nullable、infix、classifier companion、too many arguments）。

判定进度小结（tattr4 r1 后，33 类）：修复位置与残余挂靠修复项 20 类
（tattr4 r1 把 cannot be reassigned 升为修复位置 F3au；其余十九类同前：
unresolved 残余经 tattr2 r2 十七类判定，return、assignment、operator
applied 三类与 argument 类内 Int 给浮点形状是 F3j 残余，only safe 与
operator call prohibited 是修复位置 A 残余，cannot be a callee 随 #37，
conflicting、function invocation、no operator array access、prohibited
here、jvmField、exhaustive、variable must be initialized、hides member、
redeclaration、reserved、cannot access 十一类为 tattr3 r1 判定的修复
位置）；探针定位待因果验证 3 类（unresolved for operator、none of
candidates、modifier required，同一 Number 装箱机制，随 #23）；形状证实 10 类
（tattr4 r1：cannot infer、type mismatch inferred、star projection、
condition、overload ambiguity、Unexpected tokens、null cannot be 七类
各给生成样本与机制候选，collection literals、array literals、selector
三码 12 条为同一生成表达式 `?.[i]` 的三个解析视角）；假设与未判定
两档不再有条目。20 加 3 加 10 等于 33，与表行数相等。kotlin 侧剩余的判定工作是
给 3 类探针定位补因果链（随 #23 修复时验证）、给形状证实 10 类隔离
修复分支、把 argument 类内可空 81、Number 装箱 14、其余 24/31 归到
修复位置。

### 3.3 argument type mismatch 形状分解

2026-09-08 census11 复测（产出本表的命令见第 2.1 节）：

| 形状 | f32 | f64 | 判定 |
|---|---:|---:|---|
| 可空给非空（actual 类型以 ? 结尾，expected 非空） | 81 | 81 | 假设：疑与修复位置 A 同源；knulljud r1 对 null 相关错误类的判定报告只预览了按类汇总的计数、未逐形状并入，待并入后再判 |
| 其余转换 | 24 | 31 | 未判定；随下一轮判定 |
| Number 装箱给浮点 | 14 | 14 | 假设：T-attr 抽样为可空两臂条件表达式（FontPolicyCoverageTest.kt:125 实测），装箱路径未定位；待证 |
| Int 给浮点 | 12 | 12 | F3j 残余；FontPolicyCoverageTest.kt:243 里 `(13).toFloat()` 与未经转换的 `0` 并存是原始形状 |
| 合计（等于该类计数） | 131 | 138 | Long 给浮点形状（0/2）已随 knamefix-r8 降 0 删除 |

旧版 K4 的统计命令只覆盖数值形状（当时 f32 144 / f64 152），由本表取代；
「其余转换」行的存在不违反第 1 节的禁折叠规定，它是对 argument type
mismatch 这一个类内部的形状分类，下次分解出现新的成批形状时拆成具名行。

### 3.4 unresolved reference 符号分布（含 tattr2 r2 归属沿用）

2026-09-08 census11 实测（产出命令见第 2.1 节）：
去重后 11 个符号，计数合计 57，等于 unresolved 主类 54 加 unresolved for
operator 类的 operator 名 3。相对 `b7054019` 表的 112 个符号 Σ=368，
knamefix-r8 消灭了 101 个符号共 311 条（UString 29、clusterRange 24、
strategyName 32、Ic 16、compareXxx 长尾与全部属性名连锁在内）。f64 侧
分布与 f32 相同（主类 54 加 operator 形 3）。按第 1 节规定全量列举：

| 计数 | 符号 | 判定 |
|---:|---|---|
| 33 | kind | 修复位置（tattr2 r2 类 2）：dataClass 默认值引用兄弟参数 |
| 6 | s | 未判定（tattr2 r2 表内同名符号 6 条未单列归属） |
| 5 | region | 形状证实（rustprobe r1 跨目标对照，待 kotlin 侧探针证实）：@:dataClass 默认参数内联把构造器形参 `region` 泄漏进静态初始化器（rust 侧 clreq_profile.rs:24:433 同构造；kotlin 侧 ClreqProfile.kt:8 的 `PunctuationGluePlacements.for…` 调用点实测） |
| 3 | text | 同上假设（默认参数内联形参泄漏这一构造；kotlin 侧逐点判定未做） |
| 3 | minus | 未判定 |
| 2 | NodeFileSystem | 修复位置（tattr3 r1 结束消息）：`@:jsRequire` extern 在 Kotlin 目标没有宿主边处理被丢弃 |
| 1 | length | 未判定 |
| 1 | haxe | 未判定（疑为 haxe.Exception 引用，与 rust 侧 F4k 同名的跨目标表现，未验证） |
| 1 | count | 未判定 |
| 1 | cornerRadius | 同 region 行假设（默认参数内联形参泄漏这一构造） |
| 1 | charCodeAt | 未判定（疑与 dart F3at 的 charCodeAt 构造同源的调用点残留，未验证） |

符号到修复位置的归属沿用 tattr2 r2 十七类判定；region、text、
cornerRadius 三个符号与 rust E0425.b、swift gen 侧 org 全限定名条目、ts
F3as 的 org 名字条目是同一个 @:dataClass 默认参数内联构造在各目标的表现
（rustprobe r1 报告 3.2 节），
修复位置待跨目标统一判定后开列。修复随修复任务 knamefix（#23，由本会话
派出）的续作轮，不按符号名另行分组派发。

## 4 五个目标的生成状态（2026-09-07 实测）

boring 对尚未实现生成规则的 Haxe 构造，在生成阶段调用 `Context.error` 报错
并中止，不做猜测性输出；每消除一处报错都要重新生成一次才知道下一处，本文把
这套循环称为逐处重跑。2026-09-07 基线（tiqian `3f609c0f` 加 boring `e01b03b3`）上，kotlin、swift、rust 各 f32 与 f64 加 ts、dart 共八个生成命令退出码全部为 0，产物文件数依次为 397、397、393、393、405、405、394、393，逐处重跑阶段结束。八个格子的编译普查见第 3、5、6、7、8 节；生成阻断期的修复历史
并入第 12 节进度表。

## 5 Swift 编译普查

census27 快照（2026-09-12，boring `3b675e9e` × tiqian `d0a2ab9a`）：
strip import 后 `swiftc -typecheck -wmo` 联合检查 gen 树与 tests 树，
f64 与 f32 计数都是 0。swift 编译错误面已闭合。

历史测量记录（2026-09-08 census11 至 2026-09-12 swiftcens-r9）：
batch 模式曾把每个文件作为独立编译任务造成截断计数（f64 1262、f32 1289
的 WMO 真值发现于 swf64-r1），经 swiftinit、swf64、swiftcens 各线修复后
降为 0。逐类明细见 git 历史与本节 2026-09-12 之前的版本。

## 6 rust 编译普查

census27 快照（2026-09-12，boring `3b675e9e` × tiqian `d0a2ab9a`）：
f64 1894 / f32 1907（census20 为 2738 / 2741）。逐码四列表由
rustsurvey-r1 报告整理（完整版见 /tmp/dispatch-state/boring-rustsurvey-r1.report.md）：

| 错误码 | f64 | f32 | 代表样本（文件:行） | 机制假设 |
|---|---:|---:|---|---|
| E0308 | 1319 | 1331 | clreq/clreq_profile.rs:20（expected Vec<u32>, found [u32; 3]） | 数组字面量与 Vec、枚举与整数的类型错配 |
| E0277 | 181 | 182 | layout/prepared_paragraph.rs:113（dyn Fn 无法 shared/sent） | Mutex 包装要求 Send，捕获未满足 |
| E0599 | 71 | 71 | core/paragraph_style.rs:55（Option<Ic> 无 Display） | 可空接收者调用方法 |
| E0609 | 61 | 61 | core/layout_queries.rs:211（Option<Rect> 无字段 left） | 可空 receiver 未解包取字段 |
| E0369 | 50 | 50 | core/layout_queries.rs:187（LineBox 上用 !=） | 值类型未实现 PartialEq |
| E0425 | 39 | 39 | clreq/clreq_profile.rs:20（cannot find value region） | 标识符未生成 |
| E0382 | 35 | 35 | layout/paragraph_dp_line_breaker.rs:141（borrow of moved） | 按值传递后引用 |
| 其余 18 码 | 118 | 121 | 见 rustsurvey 报告 | 分散 |

专道分工：E0308 归 e0308 线（muse）、E0277 归 e0277 线（longcat cmd600）、
域追踪归 rustdomain 线、clone 归 F4q（已闭合）、E0433 引用类归 e0433 线
（NodeFileSystem 与 map builder 两笔已推 `b8b17665`/`553d3fd2`）。

census12 时代的逐类表（f32 4548 / f64 4558 时代）计数已整体过期，
明细见 git 历史与本节 2026-09-12 之前的版本。

## 7 TypeScript 编译普查

census27 快照（2026-09-12，boring `3b675e9e` × tiqian `d0a2ab9a`）：
`tsc --noEmit` 计数为 0。ts 编译错误面已闭合。

历史记录（2026-09-07 首测 330 起，经 ts2、ts1 各线修复，b54 三笔
narrowed variant arm、same-module class members、pipeline 展开后降为 0）：
逐类明细见 git 历史与本节 2026-09-12 之前的版本。

## 8 Dart 编译普查

census27 快照（2026-09-12，boring `3b675e9e` × tiqian `d0a2ab9a`）：
dart gen 31 / dart tests 0（census20 为 gen 96 / tests 39）。

gen 侧 31 条的逐码分布（13 码）：

| 错误码 | 条数 | 错误码 | 条数 |
|---|---:|---|---:|
| UNCHECKED_USE_OF_NULLABLE_VALUE | 5 | UNDEFINED_FUNCTION | 2 |
| UNDEFINED_IDENTIFIER | 4 | MISSING_DEFAULT_VALUE_FOR_PARAMETER | 2 |
| NOT_INITIALIZED_NON_NULLABLE_INSTANCE_FIELD | 4 | ARGUMENT_TYPE_NOT_ASSIGNABLE | 2 |
| NON_EXHAUSTIVE_SWITCH_STATEMENT | 4 | 其余六码各 1 | 6 |
| NOT_ENOUGH_POSITIONAL_ARGUMENTS | 3 | | |

文件分布无单点大户：line_breaker 4、contextual_quote_role_resolver 4、
test_trace_platform 2、unicode_emoji 2、punctuation_geometry_ledger 2、
line_break_planning_stage_coverage_test_support 2、其余散布。
dart tests 侧为 0。census12 时代的逐类表计数已整体过期，明细见 git
历史与本节 2026-09-12 之前的版本。

## 9 分级标尺

复杂度（C）：C1 单点修复，一个生成器分支或一处源文件，改动预计不超过一百行；
C2 跨文件或跨目标，同一缺陷出现在多个目标，或需要 tiqian 源与 boring 生成器
配合；C3 新机制，需要新增降级能力。

优先级（P）：P0 阻塞项，阻塞后续测量或行为对齐验收；P1 大错误种类或已在修复
计划内的排队项；P2 影响总数但不阻塞测量；P3 单例且暂无复现路径。

严重性（S）：S0 错误出在语法层，使整个生成目录无法编译或无法生成；S1 两百条
以上；S2 二十到一百九十九条；S3 二十条以下。流程类条目不适用 S，记为 S-。

## 10 KPI

工作量检验的方式（2026-09-06 用户规定）：进度以各目标逐类表的行计数变化
为准，每类可单独复测；不设覆盖多类的「其余」聚合指标，聚合数只保留合计
一个完整性数字（各类求和必须等于合计）。修复项只对修复位置开，不对消息类
主题开；每条修复项写明覆盖的错误类、类内条目与计数，KPI 现值写各目标
逐类表的实测计数，切分反映修复工作量，不合并修复位置。

| KPI | 指标 | 现值 | 目标 | 对应 |
|---|---|---|---|---|
| K1 | 修复位置 A：only safe 类计数 | f32 3 / f64 3（knullinit 系列合并 `23f4bf63` 后的残余） | 0 | #22 残余 |
| K2 | 各目标逐类判定完成度 | kotlin 33 类：修复位置或挂靠修复项 20、探针待因果 3、形状证实 10、假设 0、未判定 0（3.2 节小结，tattr4 r1）；rust 28 码：E0432、E0053、E0424、E0423、E0308-198、E0599 四子形与 r4 两组已判，E0277 仅 send 5 条已判，其余 372 条在判（第 6 节小结）；swift gen 侧 1 类已判、tests 侧 2 类已判 F3u（第 5 节）；ts 16/16 类已判（第 7 节小结）；dart 修复位置 8 码加 F3at 的 charCodeAt 组，其余 25 类形状证实或假设（第 8 节小结） | 五目标全部类有判定结论 | T-attr、T-swift、T-dart、tsprobe2、rustsem、rustprobe |
| K3 | 各目标错误总数 | kotlin f32 12 / f64 12（b1562731 补测，含 Sol 三笔与守卫笔）；rust f32 1907 / f64 1894；ts 0；dart gen 31 / tests 0；swift f32 0 / f64 0（strip import 联合 WMO）。census27 实测（2026-09-12，boring `3b675e9e` × tiqian `d0a2ab9a`）；kotlin 为 b1562731 单目标补测值 | 全部 0 | 各节逐类表求和（完整性数字，非派发单位） |
| K4 | kotlin 两个精度目录 warning 计数 | 0 / 0 | 保持 0 | 每次复测 |
| K5 | 各目标重生成退出码 | 八个生成入口全部 0（2026-09-07：kotlin、swift、rust 各 f32 与 f64，ts、dart） | 全部 0 | 逐处重跑阶段（已完成，第 4 节） |
| K6 | boring 验收命令 | 19 项 gates 统一重跑全部退出码 0：`b822afee`（dartnull 合并态）、`c38a359c`（rustcoalesce-r4 合并态）与 `4644e29b`（f4m 合并态）；`a72f994d` 合并后曾 17 项通过、3 项失败，两 rust 项失败的交互机制与修复见第 6 节 census12 段，consistency 为既有失败项、已随其后合并转绿；更早合并的 gates 结果与失败定性见第 12 节对应行 | 每次合并后保持 | 不适用 |
| K7 | 五目标普查覆盖 | 八格矩阵逐类表已建（kotlin、swift、rust 各 f32 与 f64 加 ts、dart，第 3、5、6、7、8 节） | 五目标各有逐类表 | 首次普查记录（第 12 节进度表） |

## 11 修复项清单

本清单只保留未完成或待新基线全量复测确认的修复项。已完成并在新基线复测确认的条目从本清单删除，在第 12 节进度记录表中归档；条目编号保留不复用。

### 第 1 组：判定探针（K2，先于其余修复任务的派发）

- [ ] T-attr kotlin 逐类判定探针：抽样形状已并入 3.2 与 3.3 节，unresolved reference 相关的十七类全量判定并入 3.4 节。剩余范围是将 3.2 节 14 类形状证实定位到文件与分支、证实或否证 6 类假设、判定 3 个未判定类，并将 3.3 节 argument 类内可空 82、Number 装箱 14、其余 33 归到修复位置。C2，P1，S-。
- [ ] F1a 修复位置 A 计数降为 0（#22 残余）：knullinit 系列合并 `23f4bf63` 后残余 only safe 类 f32 3 / f64 3、operator call prohibited 类 f32 6 / f64 6。C2，P1，S3。
- [ ] T-dart dart 逐类判定探针：五类探针定位已升为修复位置并入第 8 节，修复项 F3ah 至 F3ap 随之开列。剩余范围是给形状证实类定位机制位置、证实或否证其余跨目标假设（ARGUMENT_TYPE_NOT_ASSIGNABLE 的 double 67 条是否数值转换、UNDEFINED_METHOD 新增 toDouble 原因查明等）。C2，P1，S-。

### 第 3 组：按判定结果立项

本组条目按修复位置开列：一条修复项对应一个修复位置，附覆盖的错误类、条目清单、计数与判据。

- [ ] F3i dart 目标运行时与 std 影子文件的写出（残余 2 条）：dartstd-r2
      （boring `4a67d7e0`，合并 `c629b1d1`）把 URI_DOES_NOT_EXIST 从 36 条
      修到 2 条，生成侧 25 条全部消除（std 与 runtime 影子文件经
      dartcompiler/Compiler.hx 的 forceCompileModules 在消费方配置下强制
      编译写出；std/sorted_map.dart 等 extern 模块写出为仅含生成头注释的
      空壳文件，URI 条目随之消除，名字未定义转入 UNDEFINED_PREFIXED_NAME
      等类）。同笔合并新暴露 13 条进入各自错误码的判定队列（见第 8 节）。
      残余在 census11 部分回归（`3044bf91`）：dunitsurf-r3 的 `db0f713a`
      把 resident 模块并入合并 runtime 库、消费配置下不再单独写出
      runtime/ 目录，四处对 runtime/sorted_table.dart 的 import 断链
      （生成侧 liang_hyphenator.dart:3、parse_tex_hyphenation_patterns.dart:3、
      parsed_tex_hyphenation.dart:2，测试侧 liang_hyphenator_test.dart:7），
      全类 2→6（生成侧 0→3、测试侧 2→3）；目录对照证据见节首 URI 回归段。
      测试侧另有 test_host.dart（main.dart:6）与跨目录引用
      liang_hyphenator_test.dart（line_break_coverage_test.dart:8）两条
      dartstd-r2 期残余，写出条件的不一致尚未定位。修复位置随合并
      runtime 库机制重判（import 生成侧改指合并 runtime 库路径，或恢复
      单独写出），判据为 URI_DOES_NOT_EXIST 计数降为 0。C2，P1，S2。
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
- [ ] F3m dart 可空接收者守卫缺失（残余 13 条）：knullinit 系列守卫已修 property 形 314 条；修复任务
dartguard（#64）r1 至 r4 累计把残余 143 条修到 17 条、r6 后残余 13 条
      （提交号与逐轮计数见第 12 节；r4 后形状分布见 8.2）；r4 曾把 coalescing 内层构造的参数类型具化，实验 `2fe7fac9`
      使 dart 分析错误升到 1506 条量级，已回退，该方向禁用。修复位置为
      dart 生成器对可空接收者的成员访问、调用、运算
      与条件位没有生成守卫（clreq_punctuation_advance_policy.dart:27 的
      int? 接收者直接比较实测；DartExpr.hx:1315-1341 的 binop 守卫位
      派发方在 31627b5c 复核），与 kotlin 修复位置 A（#22）同构造。判据为
      该错误码计数降为 0，其余类计数不上升。C2，P1，S2。
- [ ] F4q rust Clone derive 供给与 `.clone()` 调用需求不一致：census14 实测
      该形状残余 435 条（修复未派发，判定见 rustjudge r3/r4）。修复位置
      候选两处二选一：derive 判定侧（data-class 路径对结构补 derive）与
      `.clone()` 追加侧（判 Clone 能力再追加）。判据为该形状降为 0。
      C2，P1，S2。（census14 后排首）
- [ ] F4p rust 可空类 coalescing 默认值物化与 Option 形参不匹配：census14
      实测 Option 形状 E0308 残余 389 条。修复位置为默认值物化与被调函数
      形参实际类型对齐（嵌套构造实参按形参类型包 `Some(...)`）。判据为
      该形状降为 0。C2，P1，S2。
- [ ] - [ ] - [ ] 小残余复合项（census14 实测）：F4k haxe.Exception shim（E0433 残余
      8 条、原 1 条）；F4i std.Functional shim（E0432 残余 2 条）；F4m
      dataClass 构造器 self 绑定（E0424 残余 2 条）；F3ah dart 省缺实参
      （NOT_ENOUGH_POSITIONAL 残余 3 条）。判据为各残余降为 0。
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
- [ ] F3ai dart 数据类是否生成比较函数的判定与引用侧不一致：直接覆盖第 8 节
      UNDEFINED_FUNCTION 的 compare 前缀 41 条（名称明细见 8.2，tdart2 r1）。
      修复位置为是否生成比较函数的判定 canEmitDataClassComparator（packages/compiler/
      PolicyQueries.hx:163-171）的 isDataClassFieldKey（:173-192）不接受
      Bool 与 Float 字段，而 dartcompiler/DartDecl.hx 引用侧分支
      （e01b03b3 位于 :311-312 与 :326）只复查 `:dataClass` meta、不复查
      是否生成的判定；两侧取齐（判定放宽或引用侧同判定）。同时预期消减
      UNDEFINED_METHOD 的 compareTo 形 9 条（NullableScalar 分支对非
      Comparable 实例渲染 `.compareTo`，DartDecl.hx:279-281；消减量在
      合并复测后重数，不预先承诺）。与 ts TS2724/TS2305 同源（同一
      Haxe 源构造，各目标实现独立；ts 侧机制位置待定位）。判据为 compare 前缀
      与 compareTo 形计数降为 0，其余类计数不上升。C2，P1，S2。
- [ ] F3ak dart 测试 extern 静态成员调用点不带限定名引用：覆盖第 8 节
      UNDEFINED_FUNCTION 的 mkdirSync 与 writeFileSync 各 1 条（tdart2 r1）。
      修复位置为测试 extern 的静态成员在调用点降级为不带限定名的名字
      （dartcompiler/DartExpr.hx:1605-1606 的分支形状）而定义不随 tests 产物目录
      写出。判据为两条名字计数降为 0，其余类计数不上升。C1，P3，S3。
- [ ] F3al dart coalescing 静态调用的类限定引用未随 statics-only 降级：
      覆盖第 8 节 UNDEFINED_PREFIXED_NAME 的 DefaultHyphenator 15 条与
      PunctuationGluePlacements 6 条（tdart2 r1）。修复位置为
      dartcompiler/DartExpr.hx 的 coalescingStaticCallText（e01b03b3 位于
      246-258）渲染 `前缀.类名.方法`，没有 staticRef（:1619 起）对
      statics-only 类去类名的降级路径。判据为两条名字计数降为 0，其余类
      计数不上升。C1，P2，S2。
- [ ] F3am dart 运行时 UString resident 未拉起：覆盖第 8 节
      UNDEFINED_PREFIXED_NAME 的 UString 21 条（tdart2 r1）。机制为
      std.UStringRT 成员渲染 `runtime.UString.成员`（DartExpr.hx:1607-1608
      一带），运行时模块拼接（dartcompiler/Compiler.hx:388-396）只追加
      parts 非空的 resident，tiqian 的 engine-haxe/targets/classes.hxml
      没有任何 `runtime.*` 清单条目（boring examples/dart.hxml 带
      `runtime.UString` 等条目），std.UStringRT 是 extern 不产生 parts，
      定义与引用两侧都落空。修复位置为 boring dart 编译器在配置了
      runtime-import 时强制拉起 resident（rust 侧同构造的先例是
      rustcompiler/Compiler.hx 在 runtime-import 时 `Context.getType`
      运行时类型），必要时配合 tiqian classes.hxml 补清单条目。判据为
      UString 名字计数降为 0，其余类计数不上升。C2，P1，S2。
- [ ] F3an dart 局部重名不去重：覆盖第 8 节 DUPLICATE_DEFINITION 的同名
      合成临时变量 5 条（line_geometry_stage.dart:234-246 三个 `_g`、
      line_adjustment_stage.dart:392 与 :471 的 `index`）与 Haxe 源同块
      var 重声明 2 条（layout_queries.dart:641 与 :671 的 `rubyIndex`，源
      LayoutQueries.hx:584/:597，tdart2 r1）。修复位置为
      dartcompiler/DartExpr.hx 的 localName（e01b03b3 位于 3498 起）对
      命名临时变量原样输出、同块不去重。与 ts TS2451 同源（同一源构造，
      各目标实现独立）。判据为 7 条计数降为 0，其余类计数不上升。
      C2，P2，S2。
- [ ] F3ao dart extension type 表示字段与成员同名：覆盖第 8 节
      DUPLICATE_DEFINITION 的 ic.dart:5 一条（`extension type Ic(double
      count)` 里 `count()` 与表示字段 `count` 同名，tdart2 r1）。修复
      位置为 dartcompiler/DartDecl.hx 的 valueTypeDecl（e01b03b3 位于
      422-431）取第一个构造参数名为表示字段，没有与成员名查重。判据为
      该条计数降为 0，其余类计数不上升。C1，P3，S3。
- [ ] F3ap dart coalescing 构造器守卫赋值未收集：覆盖第 8 节
      NOT_INITIALIZED_NON_NULLABLE_INSTANCE_FIELD 全类 4 条（源
      LineBreaker.hx:50-57 的 if/else 守卫赋值，tdart2 r1）。修复位置为
      dartcompiler/DartExpr.hx 的 coalescedBodyFields（e01b03b3 位于
      523-553）只扫构造函数顶层语句，分支体内的 `this.x = …` 不被收集，
      字段声明侧 DartDecl.hx:639-643 因此不标 `late`。判据为该类计数
      降为 0，其余类计数不上升。C1，P2，S3。
- [ ] F3au kotlin counted-loop 识别未查循环体写计数器：覆盖 3.2 节 cannot
      be reassigned 全类 24/24（tattr4 r1 判定为修复位置）。Haxe 源的
      while 计数循环体写计数器（LineRepair.hx:61-64 的 `i++`、
      LayoutQueries 的 `index += 1`），kotlin 的区间识别谓词
      PolicyQueries.hx intervalCore（:800-868）与 intervalShort
      （:888-930）只匹配声明与条件并识别尾部自增，不检查体内其余位置
      对计数器的写，KotlinExpr.hx:867-870 matchInterval 命中后输出只读
      for 绑定（LineRepair.kt:39-44 实测）。修复位置为两谓词加体写检查
      （赋值与自增形、递归嵌套语句）返回 null 保留 while 形；与 knamefix
      （#23）的循环识别改动同文件，随其续作轮或独立派发。判据为该类
      计数降为 0，其余类计数不上升。C2，P1，S2。（排队）
- [ ] F4e rust compare 引用侧未校验 canEmitDataClassComparator：覆盖第
      6 节 E0432 compare 前缀类 42 条（rustsem r1）。嵌套 record 的
      compare 引用生成段（RustDecl.hx:333-337、:364-366、:391-393、
      :409-411、:469-470）不校验共享谓词 canEmitDataClassComparator
      （PolicyQueries.hx:163-192），定义侧判定不生成时引用侧仍输出
      compare_X。修复位置为引用侧五段接入同一谓词；与 ts TS2724、dart
      F3ai 为同一谓词不一致的跨目标表现。判据为该类计数降为 0，其余类
      计数不上升。C2，P2，S2。（排队；曾与 knamefix 系列改动同一文件的
      冲突已解除（r7 经 gates 否决未合入，r8 已合并 `3044bf91` 且不再
      触及 PolicyQueries.hx），可开分支）
- [ ] F4f rust 接口 trait 声明与 impl 签名不对齐：覆盖第 6 节 E0053 返回
      Result 类 23 条与可变维度 5 条（rustsem r1）。trait 声明
      （RustDecl.hx:87-102）固定为 `&self`、返回类型不带 Result；impl 块
      （RustDecl.hx:291-321）经 instanceFuncDecl（:1728-1738 可抛写
      Result、:1712-1717 可变写 `&mut self`）拷贝类自身签名，两侧不一致。
      修复位置为 RustDecl.hx:291-321 的接口 impl 生成段使签名与 trait
      声明对齐，或声明与实现两侧统一承载可抛与可变信息。判据为两类
      E0053 计数降为 0，其余类计数不上升。C3，P2，S3。（排队）
- [ ] F4g rust 静态 Mutex 包装不查 Send：覆盖第 6 节 E0277 类 5 条
      （rustsem r1）。moduleStaticVarDecl 默认分支（RustDecl.hx:1154-1157）
      无条件把 mutable static 包成 `Mutex<T>`，内层 `Rc<dyn Fn…>`
      （RustType.hx:148）与无 Send 上界的 `Box<dyn Trait>`
      （RustType.hx:85-87）不满足 static 的 Sync 要求。修复位置为该分支
      区分非 Send 内容（换 Arc、加 Send 上界或改为首次使用时初始化的单例形态），配合
      RustType.hx 的类型选择。判据为该类计数降为 0，其余类计数不上升。
      C3，P3，S2。（排队）
- [ ] F4h rust 同模块 payload 自映射误跳：覆盖第 6 节 E0432 could-not-find
      类内 tiqian_no_such_element_exception 3 条（rustsem r1）与 E0433
      同名连锁 1 条（layout_queries.rs，rustprobe r1 3.4，census11 逐名
      复核仍在）。payload
      enum 与异常类同模块时 preScan（Compiler.hx:818-820）形成自映射，
      generateFilesManually 的去重跳过（Compiler.hx:250-252）把异常类
      所在模块整模块丢弃。修复位置为该两处：同模块场景不跳过异常类
      模块，或自映射时改记 owner 之外的模块。判据为该 4 条降为 0，其余
      类计数不上升。C2，P3，S3。（排队）
- [ ] F4l rust @:dataClass 默认参数内联全限定名与形参泄漏：覆盖第 6 节
      E0425 类内 org 52、region 7、count 1 共 60 条（rustprobe r1
      3.2.a/3.2.b 加派发方 census11 逐名位点复核）。org 52 为默认值展开
      保留的 Haxe 点分全限定名 `org.tiqian.core.Ic.Zero` 原样输出
      （layout_input.rs:44:150 实测）；region 7 与 count 1 为同一内联
      路径把构造器形参泄漏进调用点上下文（clreq_profile.rs:24:433 的
      pub static 初始化器与 layout_dump_format.rs:146 实测）。修复位置
      为默认参数内联路径（packages/compiler/DefaultArgExpander.hx）在
      rust 侧渲染时把全限定名解析为短名并登记 import、把形参默认值
      改绑到字段或局部绑定；与 swift gen 侧 1 条、ts F3as org 名字条目、
      kotlin 3.4 节 region/text/cornerRadius 是同一默认参数内联构造的
      各目标表现。判据为该 60 条降为 0，其余类计数不上升。C2，P1，S2。
      （排队；DefaultArgExpander.hx 与 dartargs-r2 的文件冲突已随其合并
      （`a72f994d`）解除、可以开分支；rust 消费侧回归（第 6 节）的修复
      优先于本项，回归修复合入前本项 60 条与新增 3484 条出自同一个生成树、单独
      验收无法辨认）
- [ ] F4n rust @:dataClass 继承构造器 super 调用：覆盖第 6 节 E0423
      全类 1 条（rustprobe r1 3.3）。@:dataClass 带继承的构造器
      `super(Message(message))` 被原样输出成 `super(...)`（源
      IllegalStateException.hx、生成 illegal_state_exception.rs:9 实测），
      Rust 无类继承、`super` 是父模块路径关键字。类继承加构造器 super
      调用的 Haxe 语义在 rust 目标无法直接承载，属 AGENTS.md 第 34 条
      例外情形；修复须把此类类从 @:dataClass 构造器生成路径改到手工
      impl 构造路径，任务书与报告写明所依赖的 Haxe 语义。判据为该条
      降为 0，其余类计数不上升。C3，P3，S3。（排队）
- [ ] F4o tiqian 生产模块引用测试支撑类型：覆盖第 6 节 E0433 类内
      SortedMap 6、test_core 5、NodeFileSystem 2 共 13 条（rustprobe
      r1 3.4，census11 逐名核对分布不变）。tiqian 生产模块引用只在
      测试支撑模块定义的名字（paragraph_shaping_stage.rs:61 与
      replayable_font_backend.rs:40 引 SortedMap、prepared_paragraph.rs:556
      引 test_core），测试模块被 `#[cfg(test)]` 条件编译排除后名字不再参与解析
      而暴露。修复位置在 tiqian 的 engine-haxe 源把生产模块的引用改指
      生产侧定义或把类型提升进生产模块，不在 boring 侧绕过（AGENTS.md
      第 34 条）。判据为该 13 条降为 0，其余类计数不上升。C2，P2，S2。
      （排队）

## 12 进度记录
- | 2026-09-12 | F3u swift tests import 头消费侧配置归档（swift 双精度联合 WMO 计数 0，census27） |
- | 2026-09-12 | F3as ts coalescing 静态字段路径泄漏归档（ts 计数 0，census27） |

| 日期 | 修复项 | 合入位置 | 复测计数 | 验收结论 |
|---|---|---|---|---|
| 2026-09-05 | Kotlin 基线建立 | boring `a75601a` | f32 3338 / f64 3358 | 基线记录完成 |
| 2026-09-05 | 基线迁移与分桶修正 | boring `5d7417e` | f32 3335 / f64 3355 | 分桶合计与总数一致 |
| 2026-09-05 | F0e 三生成器补 `Math.abs` 规则 | boring `7f2bced` | dart 生成目录该报错消除 | 验证套件退出码全部为 0 |
| 2026-09-05 | F0f `Std.string` 违规调用定位与修复 | tiqian `e0e1172d`＋`ec285e24` | rust 生成目录该报错消除 | 移植树验证通过，engine jvmTest 通过 |
| 2026-09-05 | F0j 整型容量上界生成规则 | boring `012ab59` | rust 生成目录该报错消除 | 验证套件退出码全部为 0 |
| 2026-09-06 | 基线迁移（r4） | tiqian `105dfb30` 加 boring `4b1fec9` | f32 1183 / f64 1201，warnings 0 | 验证测试通过 |
| 2026-09-06 | ts 第二处生成阻断 UStringException 修复 | boring `f270c670`（合并 `de11c06a`） | ts 生成退出码 0，首次全量生成 | 验证检查退出码全部为 0 |
| 2026-09-06 | getter 属性读四目标调用点修复 | boring `5628b4d`（合并 `f9f26726`） | 342 个测试六目标一致 | 验证检查退出码全部为 0 |
| 2026-09-06 | ts 首次编译普查 | boring `7606ff85` | ts 1127 条、19 类 | 逐类表求和校验相等 |
| 2026-09-06 | F0l dart 静态成员顶层重名修复与首次普查 | boring `189e01ad`（合并 `31627b5c`） | dart 生成退出码 0，analyze 2931 条 | 验证检查退出码全部为 0 |
| 2026-09-06 | F0a-F0d nullargs 等历史条目合并核销 | boring `8d17b59` 等 | null 字面量错误 2122→0，f32 1535 / f64 1553 | 验证检查退出码全部为 0 |
| 2026-09-07 | knullinit 系列合并（可空接收者守卫） | boring `3d397740`、`9e0537d0`、`b552e2a1` | kotlin only safe 75→3；dart UNCHECKED 420→143 | 验证套件通过 |
| 2026-09-07 | kparamnull 合并（可选参数 null 默认值） | boring `1f35923d`（合并 `2b78ab4b`） | kotlin 错误数持续下降 | 验证套件通过 |
| 2026-09-07 | knumconv-r3 合并（数值加宽转换扩展，F3j 主体） | boring `8aa72e16`（合并 `7b3135a1`） | 运算符与返回位错误大幅消除 | 验证套件通过 |
| 2026-09-07 | kgetvis 合并（getter 可见性与 override，F3k 关闭） | boring `6f3bc868`（合并 `ad0990e2`） | modifier incompatible 14→0、cannot access 27→3 | 验证套件通过 |
| 2026-09-07 | dartifget 合并（接口 getter 声明缺失，F3h 关闭） | boring `0d470797`（合并 `14c5031d`） | dart UNDEFINED_GETTER 32→0 | 验证套件通过 |
| 2026-09-07 | rustflit 合并（浮点字面量缺整数部分） | boring `4deac694`（合并 `d870489c`） | rust 浮点字面量类 124→0 | 验证套件通过 |
| 2026-09-07 | rustf4c 合并（保留字转义，F4c 关闭） | boring `dc09d773`＋`e4c24e77` | rust 保留字转义类 12→0 | 验证套件通过 |
| 2026-09-07 | swiftrem-r2 合并（浮点字面量与转义，F3f、F3g 关闭） | boring `4c81c5fc`（合并 `e01b03b3`） | swift gen 浮点 7 条与控制字符 1 条降为 0 | 验证套件通过 |
| 2026-09-07 | F3t swift 字符串插值内嵌语句体闭包 | boring `6ad8dc66`（合并 `d2c6b559`） | swift tests 侧每精度 57→41，F3t 四类 16→0 | 验证套件通过 |
| 2026-09-07 | rustfix-r1 合并（保留字与 switch 修复，F3n、F3o、F3q 关闭） | boring `b7054019` | rust 每精度 20→4：F3n、F3o、F3q 三类降为 0 | 验证套件通过 |
| 2026-09-07 | kf3v-r1 合并（参数数量匹配，F3v 关闭） | boring `b7500d45` | kotlin too many arguments 14/14→0 | 验证套件通过 |
| 2026-09-07 | dartguard r1 至 r4 合并（可空守卫与类型名，F3l 关闭） | boring `f42c38c9`、`18d5d8d1`、`e4952dab` | dart 分析错误 2606→1707，UNDEFINED_IDENTIFIER 1025→280 | 验证套件通过 |
| 2026-09-07 | dartstd-r2 合并（运行时与 std 影子文件） | boring `4a67d7e0`、`0162fa05`（合并 `c629b1d1`） | dart URI_DOES_NOT_EXIST 36→2 | 验证套件通过 |
| 2026-09-07 | tsgetcal-r6 合并（属性调用降级，F3e 关闭） | boring `4fa632e6` | ts 1127→1004，TS2551 与 TS2341 降为 0 | 验证套件通过 |
| 2026-09-07 | rustfix r2 至 r4 合并（F3p、F3r、F3s 关闭） | boring `e509f6df`、`d0df20db` | rust 解析层错误每精度 4→0，语义层错误显露（每精度 189） | 验证套件通过 |
| 2026-09-07 | ts 测量配置补充（类型定义与路径映射） | 无代码改动（配置校准） | ts 1004→722，环境缺失 300 条全部解决 | 验证套件通过 |
| 2026-09-08 | dartguard-r6 合并（F3m 续修） | boring `02210451` 等（合并 `b4b574b5`） | dart 分析错误 1686→1682，UNCHECKED_USE 17→13 | 验证套件通过 |
| 2026-09-08 | fph32-r1 合并（F3aj 关闭） | boring `2382140b`（合并 `014d9640`） | dart UNDEFINED_FUNCTION 58→46（floatToI32 与 i32ToFloat 降为 0） | 验证套件通过 |
| 2026-09-08 | tsforce-r1 合并（F3ar 关闭） | boring `7c2a1c02`（合并 `908305a0`） | ts 分析错误 722→714，TS2304 TestCore 8→0 | 验证套件通过 |
| 2026-09-08 | dunitsurf-r2/r3 合并（dart 运行时崩溃消除） | boring `c5cf26d8`、`db0f713a` | dart analyze 推进，消除运行时崩溃 | 验证套件通过 |
| 2026-09-08 | knamefix-r8 合并（kotlin 名字解析修复，F3ag 关闭） | boring `3044bf91` | kotlin unresolved 365/358→54/54，f32 680→343 / f64 701→354 | 验证套件通过 |
| 2026-09-08 | rustmisc-r2 合并（递归字段装箱，F4j 关闭） | boring `d4a43a33`、`b74f9da9`（合并 `746742dd`） | rust E0072 每精度 1→0 | 验证套件通过 |
| 2026-09-08 | dartargs-r2 合并（F3ah dart 侧生效） | boring `74371c5a`、`963a3ba6`（合并 `a72f994d`） | dart NOT_ENOUGH_POSITIONAL_ARGUMENTS 105→4 | 验证套件通过 |
| 2026-09-08 | tsorg-r2 合并（F3as ts 侧修复） | boring `f43d6a02..bacb2ae0`（合并 `87255540`） | 消除 abstract 类静态引用完整路径泄漏，输出对齐 | 19 项验证套件退出码全部为 0 |
| 2026-09-08 | rustjudge 探针分析（E0308 与 E0599 形状定位） | 无代码改动（探针分析） | E0308 最大形状 198 条与 E0599 四子形 569 条完成定位，开列 F4p 与 F4q | 结论并入逐类表 |
| 2026-09-08 | dartnull 合并（F3at：charCodeAt 边界转换与 rust 双 unwrap 去重） | boring `7994eceb`（合并 `b822afee`） | dart 分析错误 1702→1670（生成侧 -32 全部消除）；rust E0599 位点去重 | 19 项验证套件退出码全部为 0 |
| 2026-09-08 | boring-f4m-r1 合并（F4m：rust dataClass 关联函数 self 绑定） | boring `1d7f72e9`、`52c574ff`（合并 `4644e29b`） | 关联函数 fn new 内 this 字段改绑构造器局部，覆盖 E0424 75 条 | 19 项验证套件退出码全部为 0 |
| 2026-09-08 | rustcoalesce-r4 合并（F4p：可空类 coalescing 默认值物化） | boring `21234a8d`、`2b139721`（合并 `c38a359c`） | 解决 74371c5a 引入的跨目标默认值物化不匹配问题，覆盖 E0308 198 条 | 19 项验证套件退出码全部为 0 |
| 2026-09-09 | 夜间批次推进合并 | boring `5919f6b7..446fb874` | 合并 isCloneType 判定（F4q）、functional shim（F4i）、异常降级（F4k）、dart 嵌套可空折叠、kotlin 构造默认物化等多项目标改动 | 19 项验证套件按提交逐项验证通过 |
| 2026-09-09 | 统一基线复测（夜间合并推进） | boring `cc34c779` | swift f32 0 / f64 0、ts 677、dart gen 463 / tests 415、kotlin f32 356 / f64 367、rust f32 4063 / f64 4073 | 六目标统一基线建立，Swift 双精度错误全部消除 |
| 2026-09-10 | census13 复测（模块导入与 derive 推进） | boring `ecf140dc` × tiqian `3f609c0f` | ts 430、rust f32 3840 / f64 3850、swift f32 0 / f64 7、kotlin f32 356 / f64 367、dart gen 463 / tests 415 | ts 降 247、rust 降 223，swift 新增测试覆盖引入 7 条 |
| 2026-09-10 | census14 复测（默认参数展开、From/Fault 与迭代修复） | boring `cf31edba` × tiqian `3f609c0f` | kotlin f32 178 / f64 189、ts 337、rust f32 3402 / f64 3412、swift f32 0 / f64 1、dart gen 331 / tests 67 | 六目标错误大幅下降，dart tests 降至 67，swift f64 降至 1 |
| 2026-09-10 | census15 复测（参数名绑定与 ts 路由前基线） | boring `a2f2bdc2` × tiqian `3f609c0f` | kotlin f32 176 / f64 187、ts 337、rust f32 3402 / f64 3174、dart gen 269 / tests 48 | dart tests 67→48，rust f64 3174（redo 测量修正） |
| 2026-09-10 | census16 复测（kotlin 参数名绑定＋ts 路由＋F4q clone 后） | boring `4d91f806` × tiqian `3f609c0f` | kotlin f32 147 / f64 49、ts 330、rust f32 3493 / f64 3490、swift f32 1289 / f64 1262（WMO 测量）、dart gen 269 / tests 48 | kotlin f64 187→49；rust E0599 降至 656、E0308 升至 2019（clone 修复解锁下游暴露）；swift 改用 `-wmo` 全模块测量，旧 batch 计数为截断值 |
| 2026-09-11 | census17 复测（swift 全栈＋rust E0308 所有权批＋异常链后） | boring `f3099727` × tiqian `3f609c0f` | kotlin f32 146 / f64 49、ts 330、rust f32 3058 / f64 3055、swift f32 297 / f64 266（WMO 测量）、dart gen 260 / tests 43 | swift WMO 每精度降约 1000；rust 每精度降 435；dart gen 269→260、tests 48→43；swift 首测零值已当场复测纠正 |
| 2026-09-12 | census26 复测（rust 可空接收者守卫链、clippy deny 配置、batch44 十笔后） | boring `a0069187` × tiqian `d0a2ab9a` | kotlin f32 64 / f64 64、ts 16、rust f32 1913 / f64 1900、swift f32 0 / f64 0（strip import 联合 WMO）、dart gen 31 / tests 0 | kotlin 较 census24 的 66 降 2（nullable 守卫链与可空接收者笔贡献）；swift 双精度与 dart tests 降为 0；census.sh 首次中央全计时 18 分钟（16:50:23 bootstrap 起） |
| 2026-09-12 | batch44 十笔（dart 函数式数组、Std.string 黄金、kotlin 可空接收者、kotlin 树钉两笔、swift 五笔） | boring `b1562731` 前段 | ts、dart、kotlin、swift、rust 各面 | 套件失败名 40→34；树钉黄金债开始清偿 |
| 2026-09-12 | batch45 两笔（clippy deny 配置写入＋manifest 断言对齐） | boring `a0069187` | rust 包品质检查 | CI 的 stage1 检查纳入链验脚本 |
| 2026-09-12 | b48 三笔（rust 外部接收者扩展、Vec join 降级、断言同步） | boring `f0d5573b` | rust E0599/E0433 面 | 套件失败名 35→22 |
| 2026-09-12 | b54 三笔（ts 重写版：narrowed variant arm、same-module members、pipeline 展开） | boring `f0bb2ef8` | ts 面 | ts 普查 16→0（census27 实测降为 0）；首次推送后 CI 红（stage1 V08），守卫笔修复 |
| 2026-09-12 | b55 两笔（NodeFileSystem 降级、map builder resident 表） | boring `553d3fd2` | rust E0433 引用类错误 | E0433 15→8（rustsurvey 报告对照） |
| 2026-09-12 | b46fix2 四笔（Sol 可空实参统一处理三笔＋样本回归守卫笔） | boring `b1562731` | kotlin 可空实参类错误 | kotlin 普查 64→12（补测）；样本套件双门 31/31 绿 |
| 2026-09-12 | census27 复测 | boring `3b675e9e` × tiqian `d0a2ab9a` | 全五目标 | ts 0、swift 0/0、dart tests 0、rust 1894/1907、kotlin 64（不含当晚 b46fix2，补测 12）；census.sh 在本机实测 18 分钟 |
