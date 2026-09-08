# 五目标生成代码编译错误普查与修复追踪

本文记录 engine-haxe 生成代码在五种输出语言（Kotlin、TypeScript、Rust、Swift、
Dart）下的编译错误，作为跨目标行为对齐（见
[cross-target-alignment.md](cross-target-alignment.md)）的修复计划与进度追踪。
普查对象是 engine-haxe/out/ 下由 boring 从 Haxe 源码翻译出的目标语言代码目录。
原生 `engine` 模块的 `./gradlew :engine:jvmTest` 无失败，不在本文范围内。

当前状态（2026-09-08，基线 tiqian main `4bfe817f` 加 boring `a72f994d`
（rustmisc-r2 与 dartargs-r2 两笔合并之后），在 tiqian 主树直接复测，
日志与逐类表 /tmp/census12/）：八个生成命令退出码全部为 0。kotlin f32
343 条 / f64 354 条（第 3 节；相对 census11 的 343/354 持平，dartargs-r2
的 kotlin 侧改动不改变消费树计数）；rust 出现合并引入的消费侧回归：f32
4548 条、f64 4558 条、28 个错误码（第 6 节；相对 census11 的每精度 247
上升 4301/4311，回归来自 `3044bf91..a72f994d` 区间两笔合并的 rust 侧
改动，boring 自身验收命令在同一提交除 consistency 外全部通过，样本没有
覆盖消费树的这些形状，逐码来源判定与机制候选见第 6 节）；ts 706 条、16 类（第 7 节；
该区间未触及 ts 编译器，沿用 census11 复测值）；dart 1594 条（生成侧
1193、测试侧 401；第 8 节；相对 census11 的 1695 降 101，全部在
NOT_ENOUGH_POSITIONAL_ARGUMENTS 105→4，F3ah 的修复经 dartargs-r2 生效）；swift gen 侧每精度
1 条（第 5 节；该区间未触及 swift 编译器，沿用 census11 复测值）、tests
侧计数无法产出（第 5 节；测试树的 `import TiqianEngine` 使逐目录 swiftc
在模块加载处中止，gen 侧降为 0 并产出模块前没有替代配方）。已解决并复测
确认的修复项自 2026-09-07 起从第 11 节清单删除，只留第 12 节进度行。

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
- 第 12 节进度表与第 11 节修复历程不复述提交内容：哪个提交改了哪个文件、
  哪个分支、加了几行，经提交号用 git show 得知，文档不重复叙述
  （2026-09-08 用户裁定）。进度表修复项栏写修复项编号、修复名与覆盖的
  错误类；修复历程括注写轮次、提交号与提交号无法得知的事实（计数、
  验收结果、判定、裁定、失败的定性、未提交事件）。提交号本身可以出现在
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
计数（2026-09-07 配方，基线 boring `e01b03b3`、tiqian `3f609c0f`，检出目录
/tmp/tiqian-census6，日志与逐类表 /tmp/census6/swift-tests-table.txt）。
同精度的 gen 与 tests 两目录没有同名文件（2026-09-07 comm 实测），可以
合并进一次 swiftc 调用；合并测量用于区分消费侧配置与引擎侧错误：

```shell
cd /tmp/tiqian-census6/engine-haxe/out
for D in swift-gen-f64 swift-gen-f64-tests; do
  nix develop /tmp/boring-main -c bash -c "find $D -name '*.swift' | sort | xargs swiftc -typecheck" > /tmp/census6/sw-$D.log 2>&1
  echo "$D rc=$?"; grep -a -c ': error:' /tmp/census6/sw-$D.log
done   # f32 侧同形，目录名换 swift-gen-f32 与 swift-gen-f32-tests

# 合并测量（诊断用，不替代逐目录计数）：同精度 gen 与 tests 合并后跨目录
# 符号可解析；语法解析错误还有剩余时语义检查不完整（实测 gen 侧的 optional
# 解包错误在合并输出中缺席），语义错误仍以逐目录计数为准
nix develop /tmp/boring-main -c bash -c "find swift-gen-f64 swift-gen-f64-tests -name '*.swift' | sort | xargs swiftc -typecheck" > /tmp/sw-combined-64.log 2>&1; echo rc=$?
grep -a -c ': error:' /tmp/sw-combined-64.log   # f32 侧同形，/tmp/sw-combined-32.log
```

boring 遇到尚未实现生成规则的 Haxe 构造时，在第一处这样的构造上报错并中止。
四个目标的生成命令退出码均已为 0，四者的编译普查命令都已记在本节。

TypeScript 重生成与首次编译普查（2026-09-06；boring `7606ff85`，tiqian
`f2517918`，检出目录 /tmp/tiqian-audit，重生成日志 /tmp/audit-gen.log，
编译普查日志 /tmp/audit-tsc.log，逐类表 /tmp/audit-tsc-families.txt，
类内分解 /tmp/audit-tsc-subfamilies.txt）：

```shell
# 重生成（.dev 指向 boring 主仓库检出，检出位于 7606ff85）
cd /tmp/tiqian-audit && nix develop -c bash -c 'printf /home/losses/Development/boring > .haxelib/boring/.dev; haxe engine-haxe/core-ts.hxml; echo "TS_RC=$?"'

# 编译普查。tsc 用 tiqian 主仓库 node_modules 里的副本（5.9.3）；有错误时
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

2026-09-07 起 ts 编译普查命令补类型配置（动机：首测与复测各有 308 条
测量环境条目，tsc 未配置 bun 与 node 的类型定义、未把 `@tiqian/runtime`
模块名映射到生成树内的文件；同树复测见第 7 节第三轮，日志
/tmp/census7/ts-typed.log）。类型支撑目录一次安装：

```shell
# 类型支撑目录（独立于 tiqian 与 boring 的 node_modules；@types/bun 的
# index.d.ts 只有一行对 bun-types 的 reference，两个包都要装；@types/node
# 作为 bun-types 的依赖自动装入）
mkdir -p /tmp/census7/tsconfig-support && cd /tmp/census7/tsconfig-support \
  && npm init -y >/dev/null && npm install @types/bun bun-types --no-audit --no-fund
```

测量树 out 目录内放 `tsconfig.json`（生成产物目录，不入库）后用 `-p`
运行。编译选项与首轮命令行逐项相同（noEmit、allowImportingTsExtensions、
module esnext、moduleResolution bundler、target es2022），新增四项：
`paths` 把 `@tiqian/runtime` 与 `@tiqian/runtime/test` 映射到生成树内的
runtime.ts 与 runtime/test.ts；`typeRoots` 加 `types` 把 node 与 bun 的
类型定义装入全局（bun:test 由 bun-types/test.d.ts 的
`declare module "bun:test"` 提供）：

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
    "typeRoots": ["/tmp/census7/tsconfig-support/node_modules/@types"],
    "types": ["node", "bun"]
  },
  "include": ["ts-gen/**/*.ts", "ts-gen-tests/**/*.ts"]
}
```

```shell
cd /tmp/tiqian-census6/engine-haxe/out && nix develop -c bash -c 'bun /home/losses/Development/tiqian/node_modules/typescript/bin/tsc -p tsconfig.json > /tmp/census7/ts-typed.log 2>&1; echo "TSC_RC=$?"; grep -c "error TS" /tmp/census7/ts-typed.log'
# 逐类枚举与类内分解的管道与上方首测块相同，日志名换 ts-typed.log
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
检出目录 /tmp/tiqian-dartic，普查日志 /tmp/dartic-census-gen.log 与
/tmp/dartic-census-tests.log，逐类表 /tmp/dartic-code-combined.txt，类内分解
/tmp/dartic-undef-name.txt 等）：

```shell
# 重生成（消费方检出与 .dev 都指向合并检出 /tmp/boring-darticm 的 main
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
  该错误在（实测对照 /tmp/census6/sw-swift-gen-f64.log 与
  /tmp/sw-combined-64.log）；语义错误计数以逐目录为准。
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

- 测量基线：tiqian `3f609c0f`（本地 main）加 boring `e01b03b3`，2026-09-07
  在检出目录 /tmp/tiqian-census6 实测（脚本 /tmp/census6.sh，日志
  /tmp/census6/k32.log 与 /tmp/census6/k64.log，逐类表与求和校验存于
  /tmp/census6/report.txt）。该检出的 vendored 副本指向本检出
  .haxelib/boring/git 里的 `e01b03b3`。
- f32 目录（out/kotlin-gen-f32 与 kotlin-gen-f32-tests）696 条错误；f64
  目录（out/kotlin-gen-f64 与 kotlin-gen-f64-tests）717 条错误；两目录
  warning 计数均为 0。
- 基线血统：`a75601a` 首测 f32 3338 / f64 3358（`5d7417e` 上 3335/3355）；
  `8d17b59`（nullargs 合入）1535 / 1553；tiqian `105dfb30` 加 boring
  `4b1fec9` 1183 / 1201；本节基线 696 / 717；kf3v-r1 合并后 `b7054019`
  680 / 701（too many arguments 14/14 随 F3v 降 0 删除，日志
  /tmp/census6/k32-b7054019.log 与 k64-b7054019.log）；knamefix-r8 合并后
  `3044bf91` 343 / 354（census11，两列类集首次完全相同，各 33 类；逐类
  降量、新类引入路径与未查明项见 3.2 表行内说明）；`a72f994d` 343 / 354
  （census12，见下）。旧分桶表 28 桶 2026-09-06 起作废，由第 3.2 节
  逐类表取代。
- 2026-09-08 census12 复测（boring `a72f994d`，vendored 推进到该提交，
  tiqian 主树 `4bfe817f` 重生成，退出码 0，日志 /tmp/census12/k32.log 与
  k64.log）：f32 343 / f64 354、warnings 0/0，与 census11 总数持平
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

判定来源：T-attr r1（/tmp/dispatch-state/tiqian-tattr-r1.report.md，基线
boring `4b1fec9`，只采信其抽样形状，其逐类定位段为同一份引用的重复粘贴、
不采信）；tattr2 r2（/tmp/dispatch-state/boring-tattr2-r2.report.md，基线
boring `d870489c`，unresolved reference 相关十七类全量判定，基线实测
112 个符号 Σ=368 与该报告 111/366 有两符号差异，类归属按符号名沿用）；
f64only r1（f64 独有类的根本原因）；tattr3 r1
（/tmp/dispatch-state/boring-tattr3-r1.report.md，十四类机制定位与假设
检验，生成器行号由派发方在 e01b03b3 检出逐处复核）。数值转换面
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

2026-09-08 census11 复测（/tmp/census11/k32.log 与 k64.log，产出本表的
命令见第 2.1 节）：

| 形状 | f32 | f64 | 判定 |
|---|---:|---:|---|
| 可空给非空（actual 类型以 ? 结尾，expected 非空） | 81 | 81 | 假设：疑与修复位置 A 同源；knulljud r1 对 null 相关错误类的判定报告只预览了按类汇总的计数、未逐形状并入，待并入后再判 |
| 其余转换 | 24 | 31 | 未判定；随下一轮判定 |
| Number 装箱给浮点 | 14 | 14 | 假设：T-attr 抽样为可空两臂条件表达式（FontPolicyCoverageTest.kt:125 实测），装箱路径未定位；待证 |
| Int 给浮点 | 12 | 12 | F3j 残余；FontPolicyCoverageTest.kt:243 里 `(13).toFloat()` 与未经转换的 `0` 并存是原始形状 |
| 合计（等于该类计数） | 131 | 138 | Long 给浮点形状（0/2）已随 knamefix-r8 降 0 删除 |

旧版 K4 的统计命令只覆盖数值形状（当时 f32 144 / f64 152），由本表取代；
「其余转换」行的存在不违反第 1 节的禁折叠裁定，它是对 argument type
mismatch 这一个类内部的形状分类，下次分解出现新的成批形状时拆成具名行。

### 3.4 unresolved reference 符号分布（含 tattr2 r2 归属沿用）

2026-09-08 census11 实测（/tmp/census11/k32.log，产出命令见第 2.1 节）：
去重后 11 个符号，计数合计 57，等于 unresolved 主类 54 加 unresolved for
operator 类的 operator 名 3。相对 `b7054019` 表的 112 个符号 Σ=368，
knamefix-r8 消灭了 101 个符号共 311 条（UString 29、clusterRange 24、
strategyName 32、Ic 16、compareXxx 长尾与全部属性名连锁在内）。f64 侧
分布与 f32 相同（主类 54 加 operator 形 3）。按第 1 节裁定全量列举：

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

符号到修复位置的归属沿用 tattr2 r2 十七类判定（报告
/tmp/dispatch-state/boring-tattr2-r2.report.md）；region、text、
cornerRadius 三个符号与 rust E0425.b、swift gen 侧 org 全限定名条目、ts
F3as 的 org 名字条目是同一个 @:dataClass 默认参数内联构造在各目标的表现
（rustprobe r1 报告 3.2 节，/tmp/dispatch-state/boring-rustprobe.report.md），
修复位置待跨目标统一判定后开列。修复随修复任务 knamefix（#23，由本会话
派出）的续作轮，不按符号名另行分组派发。

## 4 五个目标的生成状态（2026-09-07 实测）

boring 对尚未实现生成规则的 Haxe 构造，在生成阶段调用 `Context.error` 报错
并中止，不做猜测性输出；每消除一处报错都要重新生成一次才知道下一处，本文把
这套循环称为逐处重跑。2026-09-07 基线（tiqian `3f609c0f` 加 boring
`e01b03b3`，检出目录 /tmp/tiqian-census6）上，kotlin、swift、rust 各 f32 与
f64 加 ts、dart 共八个生成命令退出码全部为 0，产物文件数依次为 397、397、
393、393、405、405、394、393（/tmp/census6/report.txt 首段），逐处重跑
阶段结束。八个格子的编译普查见第 3、5、6、7、8 节；生成阻断期的修复历史
并入第 12 节进度表。

## 5 Swift 编译普查（2026-09-08 tests 侧计数阻断中）

2026-09-08 census11 复测现值（boring `3044bf91`，日志
/tmp/census11/sw-swift-gen-f32.log 等四份）：gen 侧每精度 1 条（错误在
census11 换成另一组：swiftnarrow 记录的 optional 解包错误消失、修复归
knamefix-r8 期间的合并，未逐笔核对；现存 1 条见下表）；tests 侧计数无法
产出：两个精度的 tests 目录逐目录 swiftc 与合并调用都在第一条
`no such module 'TiqianEngine'` 处中止（BopomofoParserTest.swift:1:8，
rc=123，计数 1 不是实际错误数）；测试文件头的 `import TiqianEngine` 在
census6 基线树同样存在（该树重跑日志 /tmp/census11/sw-census6tests-recheck.log
为 57 条），census6 能计数是因为它当时的语法解析错误使 swiftc 在解析阶段
中止、未到模块加载；本轮 gen 侧语法错误减少后 swiftc 走到模块加载被
import 卡住。gen 侧计数降为 0 并经 `-emit-module -module-name TiqianEngine`
产出模块前，tests 侧没有替代测量配方，下表 tests 侧仍标 `d2c6b559`
复测值。轮次历史：census6 基线（tiqian `3f609c0f` 加 boring `e01b03b3`，
检出 /tmp/tiqian-census6，逐类表 /tmp/census6/swift-tests-table.txt）gen
每精度 1、tests 每精度 57；swiftstr 修复合并（boring `6ad8dc66`，合并
`d2c6b559`）后 tests 每精度降 41（gen 侧不变，消费方检出 /tmp/tiqian-swiftstr）；
首测（2026-09-06，boring `185cf02`）gen 侧浮点字面量 7 条与控制字符 1 条
两类由 swiftrem-r2（boring `4c81c5fc`，合并 `e01b03b3`）修复。

gen 侧（每精度 1 条）：

| 错误消息类（骨架） | 条数 | 样本 | 判定与处置 |
|---|---:|---|---|
| cannot find 'X' in scope | 1 | swift-gen-f64/org/tiqian/core/LayoutInput.swift:22:255（f32 同位） | 已判生成器：@:dataClass 默认参数内联把 Haxe 点分全限定名 `org.tiqian.core.Ic.Zero` 原样输出（生成行 `_ paragraphStyle: ParagraphStyle = ParagraphStyle(…, org.tiqian.core.Ic.Zero, …)` 实测），与 rust E0425.a、ts F3as org 名字条目、kotlin 3.4 节 region 形参泄漏是同一构造的 swift 表现；修复随跨目标统一判定开列 |

tests 侧（每精度 41 条，复测值）。判定来源 tswift1 r1 报告
/tmp/dispatch-state/boring-tswift1-r1.report.md（六类逐类三件，锚点经派发方
复核）；消费侧配置与引擎侧的区分来自合并测量（第 2.2 节，两精度同为 16 条
语法解析错误）。引擎侧 F3t 四类（每精度 16 条，全部集中在
ExplainableStubParagraphLayoutEngineTest.swift:69 的多语句闭包嵌进插值段
连锁）已随 swiftstr 合并 `d2c6b559` 消除，对应四行已从表内删除：

| 错误消息类（骨架） | 条数 | 判定与处置 |
|---|---:|---|
| cannot find 'X' in scope | 32 | 消费侧配置：tests 目录文件没有 import 头（tiqian 的 targets/swift-common.hxml 未定义 `-D swift-test-import`，boring 合同见 examples/swift.hxml:20 与 Compiler.hx fileContent 的头部条件 :422-425），且支撑类 TracedAssertions 与 TestTraceRecorder 实际生成在 gen 目录 org/tiqian/test/trace/，tests 目录单独 typecheck 必然找不到；合并测量下这类错误数为 0。F3u |
| 'X' requires a contextual type | 9 | 消费侧配置连锁：被调方不可见时字面 nil 无上下文类型（BopomofoParserTest.swift 断言调用的第三实参）；合并测量下这类错误数为 0。随 F3u |
| 合计（求和校验） | 41 | 与 tests 目录错误总数相等（全部为消费侧配置 F3u；引擎侧 F3t 四类已消除） |

文件分布（复测）：BopomofoParserTest.swift 41 条（全部为消费侧配置
两类，即 F3u）；ExplainableStubParagraphLayoutEngineTest.swift 0 条
（修复前 16 条全部为 F3t 连锁，随合并 `d2c6b559` 消除；f32 侧同值
同分布）。


## 6 rust 编译普查（2026-09-08 census12 复测：f32 4548 / f64 4558，回归来源与逐码判定见逐类表）

首测与早期历史：2026-09-06 首测（boring `4b1fec9`）181 条、15 类；
rustflit 与 rustf4c 修掉浮点字面量 124 条与保留字转义 12 条（合并
`d870489c` 与 `782526d7`），rustf4c r2 判定探针把当时剩余 7 类归到 rust
生成器（F3n 至 F3s 开列）；`e01b03b3` 复测每精度 20 条、7 类（全语法层，
完整性以日志末行 N 与计数核对相等）；rustfix-r1 合并 `b7054019` 后每
精度 4 条、3 类（F3n、F3o、F3q 降 0；F3p、F3r、F3s 现场形状与 rustf4c
r2 记录不符，当轮更正）；rustfix r2 至 r4 合并后（`d0df20db`）解析层
三类计数降为 0、完成删除，rustc 进入类型检查，语义层错误首次进入测量：每精度
189 条、8 个消息骨架，f32 与 f64 逐类相同（两侧 N=189 均核过，见 2.3
节）。boring 自身的 stage1:rust 与 rust-f32 在 `d0df20db` 仍为 0：该
错误类只在 lib 单独构建下出现。

2026-09-08 boring `d38459ab` 第四次复测（日志 /tmp/census10/，两精度
同值）：F4d 修复生效（pub use 行错误类 108→0，E0432 全类 154→46，求和
校验相等），同次复测新增四个错误码 174 条（E0425 84、E0424 75、E0433
净增 14、E0423 1），进入区间经提交拓扑缩小为 `d0df20db..d38459ab`。
rustprobe r1 只读探针（报告 /tmp/dispatch-state/boring-rustprobe.report.md）
A/B 判定：新增四码是 `0629b847` 把测试模块加 `#[cfg(test)]` 条件编译排除
后、类型检查首次到达非测试模块既有错误的暴露（控制实验删光全部
cfg(test) 标注后计数精确回到 189；`908305a0` 精确复现基线 189 且其非
测试 .rs 文件与 `d38459ab` 逐字节相同）；派发方假设的两个候选
（`39054311` 经 `c332897a`、`7c2a1c02` 经 `908305a0`）都被实验否定，
`c332897a` 生成整体崩溃属 knamefix-r2 自身回归。

2026-09-08 census11 复测（boring `3044bf91`，日志 /tmp/census11/r32.log 与
r64.log）：每精度 247 条、8 个错误码（逐码 E0425 76、E0424 75、E0432 46、
E0053 28、E0433 15、E0277 5、E0423 1、E0072 1，求和校验相等），相对
`d38459ab` 的 255 降 8，唯一变动 E0425（84→76，名字 `pi` 4 条与 `bi`
4 条消失，归 knamefix-r8 期间的合并，未逐笔核对），其余七个错误码计数不变。

2026-09-08 census12 复测（boring `a72f994d`，日志 /tmp/census12/r32.log 与
r64.log）：f32 4548 条、f64 4558 条（两精度逐码差异只在 E0689，93 对 83），
相对 census11 的每精度 247 上升 4301/4311，为 `3044bf91..a72f994d` 区间
仅有的两笔 rust 侧合并（`746742dd` rustmisc-r2 与 `a72f994d` dartargs-r2）
引入的消费侧回归，boring 样本没有覆盖消费树的这些形状。boring 验收命令
在合并后统一重跑 17 项通过、3 项失败（/tmp/gates-postmerge12-rc.txt 为
修复前记录）：stage1:rust 与 rust-f32 的失败是两笔改动的交互（dartargs
的 fieldInits 优先分支消费了 rustmisc 装箱分支要处理的同名形参记录），
派发方修复 RustExpr.hx constructorBody 的同名形参不记录规则并入
`a72f994d` 后两套件单独复测通过；consistency 为既有失败项。抽样核对
一致的机制候选（经 rustjudge r2/r3 逐环验证，逐码判定与形状明细见
下表）：E0308 计数最大的形状 198 条与嵌套构造实参缺 Some 包装一致
（错误来源判定 `74371c5a`，F4p）；E0277 条数 5→834 的扩大与调用点
写出默认值相关（F4g 仅覆盖 send 5 条）；E0599 的 470 条 no method 形与
`.clone()` 调用点落在没有 Clone 派生的类上一致（F4q）；E0689 与无类型
后缀整数字面量上的方法调用一致；E0061 与构造器和函数签名减参后调用点
未补实参一致。E0072 1→0（F4j 修复生效）；E0433 15→14（census11 段落
记 15、表行记 14 的两处不一致以本轮实测 14 为准，消失的一条未逐名
核对）；其余六个 census11 错误码计数不变。

| 错误消息类（骨架） | 计数 | 判定与处置 |
|---|---:|---|
| cannot find value `X` in this scope 等 E0425 全部消息骨架 | 76 | 判定分四支（rustprobe r1 加派发方 census11 逐名位点复核；名字分布全列：org 52、UStringException 8、region 7、compare 前缀函数 6〔compare_font_metrics_request、compare_glue、compare_shaping_evidence_key、compare_recorded_shaping_result、compare_metrics_evidence_key、compare_recorded_font_metrics 各 1〕、SORTED_TABLE_COMPARE_STRINGS 2、count 1）：org 52 为 @:dataClass 默认参数内联把 Haxe 点分全限定名 `org.tiqian.core.Ic.Zero` 原样输出（rustprobe r1 3.2.a，layout_input.rs:44:150 实测；位点分布 35 个文件，core 侧 layout_input.rs、paragraph_style.rs、rich_text_paint.rs、rich_text_background_paint.rs 各 1，其余 48 条在 layout 目录的 test_support 类文件，最大 ruby_layout_test_support.rs 6）；region 7 与 count 1 为同一内联路径把构造器形参泄漏进调用点上下文（clreq_profile.rs:24:433 的 pub static 初始化器与 layout_dump_format.rs:146 的 `Ic(count)` 实测，rustprobe r1 3.2.a/3.2.b）；compare 前缀 6 条落在比较函数合成体内（font_metrics.rs:114、punctuation_model.rs:159、shaping_evidence.rs:39/41/66/68 实测），是引用了未生成的兄弟比较函数，与 E0432 compare 前缀 42 条同一谓词不一致（F4e 修复项；rustprobe r1 3.2.b 把这六条记入默认参数内联的修复范围，与位点形状不符，按位点改判）；UStringException 8（text_shaper.rs:176/185 等 Return 位 `Result<…, UStringException>`）与 SORTED_TABLE_COMPARE_STRINGS 2（parse_tex_hyphenation_patterns.rs:54/55 引 crate::runtime::sorted_table 常量）为模块未生成或未导出形态，假设与 F4k/F4i 同类（ts F3aq 与 dart F3am 记录同名 UString 问题），待证。修复项 F4l（org 加 region 加 count 60 条）；compare 6 条随 F4e；UStringException 加 SORTED_TABLE 10 条待证 |
| expected value, found module `self`（E0424） | 75 | 修复位置（rustprobe r1 3.1）：@:dataClass 构造器体里的 `this.<field>` 被输出成关联函数 `fn new` 内的 `self.<field>`，而关联函数没有 `self` 绑定（源 InlineObjectBoundaryAdjustment.hx:18-22 的构造器校验、生成 inline_object_boundary_adjustment.rs:21 实测；受害 Haxe 源另有 LayoutConstraints.hx、RichTextBackgroundPaint.hx、RichTextPaint.hx、LineBreaker.hx、LineBreakPlanningStage.hx、LineOptimization.hx、ParagraphDpLineBreaker.hx、ProgressiveBreakDecisions.hx、ContextualDashEllipsisRoleResolver.hx、ContextualQuoteRoleResolver.hx、TestTraceRecorder.hx 十一个，清单见报告 3.1）。修复项 F4m |
| unresolved import `X`（compare 前缀函数） | 42 | 判定完成（rustsem r1）：嵌套 record 的 compare 引用侧未校验共享谓词 canEmitDataClassComparator：引用生成段在 RustDecl.hx:333-337、:364-366、:391-393、:409-411、:469-470，谓词定义在 PolicyQueries.hx:163-192；定义侧判定不生成时引用侧仍输出 compare_X（样本 layout_input.rs:17 引 compare_paragraph_style、layout_result.rs:10 引 compare_line_box）。与 ts TS2724、dart F3ai 为同一谓词不一致的跨目标表现。修复项 F4e（排队） |
| method `X` has an incompatible type for trait: expected `X`, found `X` | 23 | 判定完成（rustsem r1）：接口 trait 声明（RustDecl.hx:87-102，:96 固定为 `&self`、返回类型不带 Result）与 impl 块签名（RustDecl.hx:291-321 经 instanceFuncDecl :1728-1738 按方法体可抛性写 `Result<…>`）不对齐；Haxe 无 checked exception，trait 声明没有可抛信息（display_glyph_substitution_engine_test_support.rs:265 等 23 处实测）。修复项 F4f（排队） |
| `X` cannot be sent between threads safely: `X` cannot be sent between threads safely | 834 | 仅 send 形状 5 条判定完成（rustsem r1）：moduleStaticVarDecl 默认分支（RustDecl.hx:1154-1157）无条件把 mutable static 包成 `Mutex<T>`，内层 `Rc<dyn Fn(&K,&K)->i32>`（RustType.hx:148）与无 Send 上界的 `Box<dyn Trait>`（RustType.hx:85-87）不满足 static 的 Sync 要求（english_hyphenation.rs:9 一条、prepared_paragraph.rs:37-40 四条实测）。修复项 F4g（排队，覆盖该 5 条）。census12 条数 5→834，早先「新增条目与该机制相同」的说法经 rustjudge r2 全量形状分解证伪：834 条分 18 种消息形状（couldn't convert the error 494、闭包 `?` 算子 137、trait bound 未满足 62、can't compare 两形 57＋34、not an iterator 13、cannot divide 12、send 5、其余 11 形合计 20，Σ=834，报告 /tmp/dispatch-state/boring-rustjudge-r2.report.md 第 5 节），F4g 只覆盖 send 5 条，其余 829 条的逐形状判定由 rustjudge r3 执行 |
| method `X` has an incompatible type for trait: types differ in mutability | 5 | 判定完成（rustsem r1）：与 23 条同一生成路径的可变维度：isMethodMutating（RustDecl.hx:1857-1869）检测到方法体写自身字段后 impl 侧输出 `&mut self`（:1712-1717），trait 声明固定为 `&self`（:96）。修复项 F4f（与 23 条同一修复位置，排队） |
| unresolved import `X`: could not find `X` in `X`（tiqian_no_such_element_exception） | 3 | 判定完成（rustsem r1）：payload enum 与异常类声明在同一 Haxe 模块（TiqianNoSuchElementException.hx）时，preScan（Compiler.hx:818-820）形成自映射，generateFilesManually 的去重跳过（Compiler.hx:250-252）把异常类自己的模块整模块丢弃，文件不写、mod.rs 不登记。修复项 F4h（排队） |
| unresolved import `X`: could not find `X` in `X`（crate::std::functional） | 1 | 判定完成（rustsem r1）：std.Functional 是 extern 不产生模块（Compiler.hx:84），rust 目标 shim 清单（Compiler.hx:319-325 与 RustImports.hx:9-21）无对应项，sumOfFloat/forEach 也未进惯用展开层（RustExpr.hx:3896-3913 与 :4332；PipelineExpander.hx:852-855 只有 sortedBy 就地展开）。修复项 F4i（排队） |
| cannot find `X` in `X`（E0433 新增条目） | 14 | 判定分两支（rustprobe r1 3.4，census11 逐名核对分布不变：SortedMap 6、test_core 5、NodeFileSystem 2、tiqian_no_such_element_exception 1）：SortedMap 加 test_core 加 NodeFileSystem 13 条为 tiqian 生产模块引用只在测试支撑模块定义的名字（paragraph_shaping_stage.rs:61 与 replayable_font_backend.rs:40 引 SortedMap、prepared_paragraph.rs:556 引 test_core），测试模块被 `#[cfg(test)]` 条件编译排除后名字不再参与解析而暴露；修复位置在 tiqian 的 engine-haxe 源把生产模块的引用改指生产侧定义或把类型提升进生产模块，不在 boring 侧绕过，修复项 F4o；tiqian_no_such_element_exception 1 条（layout_queries.rs）与 E0432 同名 3 条（F4h）是同一模块未写出的连锁，随 F4h |
| expected function, found module `super`（E0423） | 1 | 判定完成（rustprobe r1 3.3）：@:dataClass 带继承的构造器 `super(Message(message))` 被原样输出成 `super(...)`（源 IllegalStateException.hx、生成 illegal_state_exception.rs:9 实测），Rust 无类继承、`super` 是父模块路径关键字，类继承加构造器 super 调用的 Haxe 语义在 rust 目标无法直接承载，属 AGENTS.md 第 34 条例外情形；修复须把此类类从 @:dataClass 构造器生成路径改到手工 impl 构造路径，任务书与报告写明所依赖的 Haxe 语义。修复项 F4n |
| mismatched types: expected `X`, found `X`（E0308） | 2083 | 计数最大的形状 198 条判定完成（rustjudge r2 第 3.1 节，档位修复位置）：Haxe 源 `LayoutInput.hx:21` 的 `textStyle == null ? new TextStyle() : textStyle` 经 `RustExpr.hx:503` coalescingNormalizationLines 与 `:201` coalescingDefaultText 物化默认构造，嵌套构造实参列表的生成路径（`RustExpr.hx:246`）不传既有的 asOption 形参，实参不带 Some 包装直接写出（生成 `layout_input.rs:43`），对 `text_style.rs:17` 的全 `Option<...>` 形参逐个不匹配；错误来源判定为 `74371c5a`（git show hunk 与该形状直接相交），VNull 双包装与递归装箱两候选证伪。修复项 F4p，修复任务 rustcoalesce-r1 已派发（2026-09-08，terra）。其余形状：expected `f64` found integer 222、expected `u32` found `i32` 87 与反向 69、expected `Option<String>` found `String` 70、语句位收 `Result` 56＋55＋55、接口位收具体类缺 `Box::new` 55×3（clreq_profile.rs:24、bopomofo_parser.rs:22 实测）；类内全量形状分解的测量已由 rustjudge r1 交付并经派发方验收（209 种形状 Σ=2083，报告
/tmp/dispatch-state/boring-rustjudge-r1.report.md 第 2 节），逐形状判定待 F4p 合并后 census13 重测再按新形状空间执行（当前判定会随修复失效） |
| no method or associated item named `X` found（E0599） | 701 | 四个消息子形 569 条已判（rustjudge r3 第 3 节，2026-09-08 验收）：clone 459 条修复位置（错误来源判定为 `74371c5a`：该提交在调用点对非 Copy 值读取统一追加 `.clone()`（`RustExpr.hx:3873-3882`），而这些结构没有拿到 `#[derive(Clone)]`，要求与供给不一致（r4 前置问题回答更正了 r3 的「放宽 derive 触发条件」表述：`RustDecl.hx:216-223` 的 `final hasCoalescingClone = true` 只把非 data-class 分支的来源记录条件换成常真，`@:dataClass` 结构走 `:223-225` 第二分支、仍被 `isAllClone(varFields)`（`:2274-2300`）拦住，见 F4q），`LayoutResult` 87、`LayoutInput` 77、`LineSolution` 54、`LineCandidate` 48、`LineBox` 39 等结构无 Clone impl，位点如 `layout_queries.rs:147:96`；`d4a43a33` 递归装箱与 `b74f9da9` VNull 双包装两候选证伪），开列修复项 F4q；关联常量 51 条（`BuiltInLayoutProfiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL` 等，`RustExpr.hx:3644-3660` FStatic 静态字段映射成关联常量而实际声明是模块静态 `pub static`，built_in_layout_profiles.rs:5 实测）、`as_deref` on `Option<f64>`/`Option<u32>` 36 条（`RustExpr.hx:950-966` 字符串化分支对 null 类型统一发 `.as_deref().unwrap_or("")`，glyph.rs:40 实测）、Option 上 `to_string` 23 条（`RustExpr.hx:4167-4173` 把条件 null 分支直接字符串化成 `None.to_string()`，justifier.rs:168 实测）三组均为既有暴露（生成分支在 `3044bf91..a72f994d` 区间外已存在，错误位点由 `74371c5a` 的 coalescing 物化新引入；逐轮计数核对：census10 `rust-f64-check.log`/`rust-f32-check.log` 与 census11 `r64.log`/`r32.log` 的 E0599 计数均为 0，census12 为 701，显现全部为 census12 新增）。rustjudge r4 续判（2026-09-08 验收，诚实部分交付 26/398）：u32 上双重 `unwrap_or` 12 条（机制经派发方 2026-09-08 在 dartnull 检出 `9f0b52df` 逐行复核更正：两次追加都发生在局部声明渲染路径，`RustExpr.hx:611` 调用的 renderValueForType（定义 `:6281` 起）对标注 `Int` 局部经其 charCodeAt 桥追加第一个 `.unwrap_or(0)`，`:625`-`:631` 声明分支再追加第二个，不标注 `Null<Int>` 局部只吃声明分支一次、输出正确；`:3346`-`:3353` 算术 operand 分支只处理内联调用形、产出单 unwrap，不在此缺陷链上，rustjudge r4 早先把第二个追加记到该分支不属实；bopomofo_parser.rs:22 双后缀实测；判定为既有暴露，`3044bf91..a72f994d` 区间三笔 RustExpr 提交的 hunk 均不触及这些分支）、关联常量 `AUTO_SPACE_POLICY_DEFAULT` 14 条（`AutoSpacePolicy.hx:24` 的 `public static final Default` 经 `RustDecl.hx:1116-1170` 的 moduleStaticVarDecl 写成模块静态 `pub static LazyLock`，调用点却经 `RustExpr.hx:3640-3660` 的 FStatic 分支按 `AutoSpacePolicy::` 关联常量访问；与 51 条组同一机制，该组扩为 65 条；判定为既有暴露）两组并入。剩余 106 条（其余方法/trait 94、关联常量 12）未判 |
| this function takes N arguments but N was supplied（E0061） | 341 | 未判定（census12 新增）。data 表函数声明两个参数而调用只给一个参数（east_asian_spacing_data.rs:374 实测）。与构造器和函数签名因 coalescing 降级而减少参数、调用点未补实参相关，待判定 |
| can't call method `X` on ambiguous numeric type（E0689） | 93 | 未判定（census12 新增），rustjudge r2 已定位候选机制：`RustExpr.hx:201` coalescingDefaultText 的 CFloat 分支 f32 追加 `f32` 后缀、f64 不追加，无后缀整数字面量上调用 `to_ne_bytes`、`is_nan`（layout_queries.rs:538、annotation_geometry_stage.rs:305 实测）；f32 侧 83，f64 比 f32 多 10 条的文件分布（line_breaker.rs 多 2、paragraph_dp_line_breaker.rs 多 1、annotation_geometry_stage.rs 多 1、line_geometry_stage.rs 20 对 14）已核对，四文件 f64/f32 同位生成行对照由 rustjudge r3 完成后写入本行 |
| binary operation `==` cannot be applied to type `X`（E0369） | 69 | 未判定（census12 新增）。`==` 作用于 `TextRange`、`Fill` 等缺 PartialEq 实现（layout_queries.rs:1249、rich_text_background_paint.rs:58 实测），待判定 |
| no field `X` on type `Option<X>`（E0609） | 61 | 未判定（census12 新增）。`Option<Rect>` 上取 `.left`、`.top`（layout_queries.rs:181 实测），待判定 |
| cannot assign to `X`, which is behind a `&` reference（E0594） | 31 | 未判定（census12 新增）。`&` 引用后赋值 `self.call_count`（annotation_geometry_stage_coverage_test_support.rs:126 实测），待判定 |
| type `X` cannot be dereferenced（E0614） | 21 | 未判定（census12 新增）。对枚举 `ClreqStrictness` 解引用（justifier_engine_test_support.rs:74 实测），待判定 |
| use of moved value: `X`（E0382） | 13 | 未判定（census12 新增）。循环内使用已被移动的值（cluster_role_resolution.rs:89 实测），待判定 |
| use of unstable library feature `str_as_str`（E0658） | 11 | 未判定（census12 新增）。`str_as_str` 非 stable（layout_debug_assembly.rs:246 实测），待判定 |
| cannot index into a value of type `Option<Vec<f64>>`（E0608） | 11 | 未判定（census12 新增）。对 `Option<Vec<f64>>` 直接索引（layout_queries.rs:889 实测），待判定 |
| cannot borrow `X` as mutable, as it is not declared as mutable（E0596） | 11 | 未判定（census12 新增）。非 mut 声明被可变借用（explainable_stub_paragraph_layout_engine_test_support.rs:56 实测），待判定 |
| cannot move out of `X` which is behind a shared reference（E0507） | 9 | 未判定（census12 新增）。共享引用后移动 `self.points`（hyphenator.rs:70 实测），待判定 |
| method `X` is private（E0624） | 8 | 未判定（census12 新增）。`len` 为私有方法（layout_queries.rs:901 实测），待判定 |
| cannot apply unary operator `-` to type `u32`（E0600） | 5 | 未判定（census12 新增）。对 `u32` 取负（prepared_paragraph.rs:1686 实测），待判定 |
| non-exhaustive patterns: `X` not covered（E0004） | 5 | 未判定（census12 新增）。match 不穷尽（font_metrics.rs:141 实测），待判定 |
| type annotations needed（E0282） | 4 | 未判定（census12 新增）。`Option<T>` 无法推断（line_break_planning_stage.rs:371 实测），待判定 |
| cannot call non-const associated function in statics（E0015） | 4 | 未判定（census12 新增）。static 初始化器调用 `Fill::new`（rich_text_background_draw_style.rs:10 实测），待判定 |
| `f64` is a primitive type and therefore doesn't have fields（E0610） | 2 | 未判定（census12 新增）。对 `f64` 取字段（punctuation_geometry_ledger.rs:183 实测），待判定 |
| struct `X` has no field named `X`（E0560） | 1 | 未判定（census12 新增）。`CatalogImpl` 无 `faces` 字段（replayable_font_backend_coverage_test.rs:177 实测），待判定 |
| 合计（求和校验） | 4558 | 与 f64 错误总数相等；f32 4548（E0689 83，其余逐码与 f64 相同）。本行为 census12 复测值（`a72f994d`，/tmp/census12/r32.log 与 r64.log）；census11 复测值 247（`3044bf91`，/tmp/census11）；`d38459ab` 复测值 255（/tmp/census10）；`d0df20db` 复测值 189（/tmp/census7） |

相对上一表已降为 0 并删除的行：unresolved import `X`: use of unresolved
module or unlinked crate `X`（mod.rs 对 `X_test` 模块的 pub use 行，F4d，
`d38459ab` 复测 108→0，census11 复核仍为 0，按删除制移除，进度见第 12 节）；
recursive type `X` has infinite size（E0072，F4j，rustmisc-r2 递归字段装箱
合并 `746742dd` 后 census12 复测 1→0，按删除制移除，进度见第 12 节）。

判定进度小结（2026-09-08 census12 复测，28 个错误码：既有 7 码中 6 码
判定保持、E0277 判定范围缩小；新增 20 码中 E0308 计数最大的形状与 E0599
四个消息子形已判，其余判定中）：解析层已清空（F3n 至 F3s 六项全部完成
删除）。既有错误码：E0432（46，compare 前缀 42 加
tiqian_no_such_element_exception 3 加 crate::std::functional 1）与 E0053
（28，即表中 23 与 5 两行）由 rustsem r1 判定为修复位置；E0277（834）
经 rustjudge r2 全量形状分解改为仅 send 形状 5 条属 F4g 机制，其余 829
条分 17 种消息形状（couldn't convert 494、闭包 `?` 137、trait bound 62
等，全表见该报告第 5 节）待逐形状判定；E0425（76）分四支：F4l 60 条、
随 F4e 6 条、假设待证 10 条；E0424（75）修复位置 F4m（修复任务
boring-f4m-r1 已交付并合并 `4644e29b`）；E0423（1）判定完成 F4n（条款 34 例外）；E0433
（14）分 F4o 13 条、F4h 连锁 1 条；E0072 已由 rustmisc-r2 修掉并删除。
census12 新增 20 码共 3484 条：E0308 计数最大的形状 198 条判定为修复
位置（rustjudge r2，错误来源判定 `74371c5a`，修复项 F4p 的修复任务
rustcoalesce r1 至 r4 已交付、末轮合并 `c38a359c`），其余形状待 census13
重测后按新形状空间再判（当前判定会随修复失效）；E0599 四个消息子形
569 条已判（rustjudge r3：clone 459 修复位置开列 F4q、关联常量 51、
`as_deref` 36、Option `to_string` 23 三组 110 条既有暴露）；r4 续判交付
26 条（u32 双重 `unwrap_or` 12 与关联常量 `AUTO_SPACE_POLICY_DEFAULT`
14，均既有暴露，见 E0599 行）；E0061、E0277、E0689 三码形状空间随 F4p
修复合并变化，判定排除待 census13。剩余未判 372 条（E0599 余 106、
E0369 69、E0609 61、E0594 31、十三个小码 105，Σ=372；早记「十三个
小码 167」为求和笔误，本轮更正）待续轮。rustjudge r1 至 r4 的验收历程
见第 12 节对应行。剩余既有判定工作是给 UStringException 8 加
SORTED_TABLE_COMPARE_STRINGS 2 共 10 条假设补因果（疑与 F4k/F4i 同为
模块未生成或未导出形态，可与 #93 的 haxe.Exception 跨目标立项合并
评估）。F4e 至 F4i、F4l 至 F4o 排队；F4l 要改的 DefaultArgExpander.hx
与 dartargs-r2 的文件冲突已随其合并解除、可以开分支，但 rust 侧回归的
复测（census13）优先于 F4l（回归修复合入前 F4l 的 60 条与新增 3484 条
出自同一个生成树，单独验收无法辨认）。

## 7 TypeScript 编译普查（2026-09-07 第三次复测）

轮次历史：首测（boring `7606ff85`、tiqian `f2517918`，检出 /tmp/tiqian-audit，
日志 /tmp/audit-tsc.log，逐类表 /tmp/audit-tsc-families.txt）1127 条、
19 类（其中测量环境条目 308 条：TS2307 的 bun:test 与 @tiqian 模块名、
TS2580 的 process 等，引擎侧 819 条）；census6 复测（`e01b03b3`，日志
/tmp/census6/ts.log）1127 条与首测逐类相同；`d0df20db` 复测 1004 条、
16 类（F3e 覆盖的 TS2551 28 条与 TS2341 get_ 前缀 10 条降 0，TS2420、
TS2693 两类降 0，TS2345 87→14、TS2322 8→2；TS2304 类内 Ic 103 条消失、
org 2→108 条为新形态即 F3as）；同一棵 `d0df20db` 生成树按第 2.2 节新
配方配置类型定义与模块映射后第三次复测 722 条（日志
/tmp/census7/ts-typed.log）：测量环境 300 条全部解析、TS2580 整类删除，
新暴露引擎侧 19 条：TS2305 新增 18 条（生成代码从 `@tiqian/runtime`
导入 UString 8、floatToI32 6、i32ToFloat 4，而生成树 runtime.ts 只导出
12 个名字、不含这三个，为 ts 生成器的 runtime 模块导出缺失即 F3aq；
其中 UString 与 dart F3al 是同一个 UString 问题），TS2304 类内
compareFontMetricsRequest 1 条换骨架为 TS2552；TestCore 8 条由测量环境
改判引擎侧：ts 目标的初始化没有对 runtime.TestCore 的强制类型化
（kotlin 目标有，kotlincompiler/Compiler.hx:58），该模块未进编译、追加
段为空，全生成树无 TestCore 声明；tsforce-r1 合并（`7c2a1c02`，合并
`908305a0`）后第四次复测 714 条（日志 /tmp/census9/ts-typed.log），逐名
对照唯一变化 TestCore 8 条降 0（生成树 runtime/test.ts 现含
`export class TestCore` 声明）；census11 复测（`3044bf91`，日志
/tmp/census11/ts-typed.log）706 条、16 类不变，逐名对照的两处变化都在
名称维度（TS2304 177→171：`pi` 4 与 `bi` 4 消失、count 系列重编号；
TS2451 23→21：`rubyIndex` 2 消失；与 kotlin conflicting 13→11、dart
DUPLICATE_DEFINITION 17→16 同为 Haxe 源构造 LayoutQueries.hx 重声明的
三目标联动），归于 knamefix-r8 的循环识别这一组提交，其余十四类计数
不变。本轮 706 条全部为引擎侧。

| 错误码：错误消息类（骨架） | 计数 | 判定与处置 |
|---|---:|---|
| TS2307：Cannot find module 'X' or its corresponding type declarations. | 6 | 模块分布见 7.2（第三次复测后测量环境的 bun:test、@tiqian、node 模块全部解析，本类只剩引擎侧条目）；6 条（相对路径 5 条与 haxe/Exception 1 条）已判引擎侧为 ts 生成器的导入路径问题，分支未逐条定位；转按修复位置派发 |
| TS2448：Block-scoped variable 'X' used before its declaration. | 215 | 已证实 ts 生成器（TsExpr 的语句融合与局部绑定生成顺序；LayoutDumpFormat.ts 单文件 214 条、ShapingEvidenceJson.ts 1 条，分布见 7.2） |
| TS2304：Cannot find name 'X'. | 171 | 已判 ts 生成器：org 名称 108 条为完整限定路径 `org.tiqian.…` 引用未导入（F3as，机制与位点见 7.2）；compare 前缀 6 条同 TS2724 的导出登记缺陷（compareFontMetricsRequest 1 条换骨架为 TS2552，见该行）；TestCore 8 条已由 tsforce-r1 修复降为 0（依据见节首）；名称分布见 7.2 |
| TS2554：Expected N arguments, but got N. | 133 | 已证实 ts 生成器：默认参数与可选参数的调用实参补全机制不完整（与区间形 54 条同一原因，两形合计 187 条） |
| TS2345：Argument of type 'X' is not assignable to parameter of type 'X'. | 14 | 已判 ts 生成器：枚举载荷假设经逐类判定证伪为主因（原假设为对象字面量不能赋给枚举类型，KinsokuLevelTest.test.ts:113 样本）；`d0df20db` 复测由 87 降为 14，余量在 PreparedParagraphJfTest.test.ts 7、KinsokuLevelTest.test.ts 4、LineOptimizationCoverageTest.test.ts 3，下降与 TS2304 类内枚举值改发限定路径同现，机制未逐条验证；TS2322 同组 |
| TS2554：Expected N-N arguments, but got N. | 54 | 同 TS2554 第一形（默认/可选参数补全机制不完整，两形合计 187 条） |
| TS2724：'X' has no exported member named 'X'. Did you mean 'X'? | 35 | 已证实 ts 生成器：compare 函数的导出与导入登记缺陷（名称分布见 7.2，35 个比较器名全列；与 TS2305、TS2304 的 compare 条目同源；dart UNDEFINED_FUNCTION 的 compare 前缀 41 条与本病同源已由 tdart2 r1 双侧证实，dart 侧机制见第 8 节 F3ai，ts 侧机制位置待定位） |
| TS2451：Cannot redeclare block-scoped variable 'X'. | 21 | 已证实 ts 生成器：局部作用域复用（alpha-renaming 的 index2 计数器；原跨文件重名假设已证伪；名称分布见 7.2；dart DUPLICATE_DEFINITION 的 Haxe 源同块 var 重声明 2 条与本病同源已由 tdart2 r1 双侧证实，dart 侧机制见 F3an）；census11 相对第四次复测 23→21（rubyIndex 2 条消失，归 knamefix-r8，见节首） |
| TS2341：Property 'X' is private and only accessible within class 'X'. | 4 | get_ 前缀 10 条属 F3e，`d0df20db` 复测降为 0（F3e 完成删除）；其余 4 条已判 ts 生成器为 private 可见性过度保留；名称与类分布见 7.2 |
| TS2339：Property 'X' does not exist on type 'X'. | 14 | 已判 ts 生成器为 Haxe Array.copy 与只读数组方法在 ts 侧的映射缺失（kind 6、copy 6、push 1、insert 1，分布见 7.2） |
| TS2322：Type 'X' is not assignable to type 'X'. | 2 | 同 TS2345 组（枚举载荷判定；余 2 条在 LineOptimizationCoverageTest.test.ts:26 与 :28） |
| TS2305：Module 'X' has no exported member 'X'. | 25 | compare 函数导出/导入登记缺陷（同 TS2724，7 条）加第三次复测新暴露的 runtime 模块导出缺失 18 条（`@tiqian/runtime` 的 UString 8、floatToI32 6、i32ToFloat 4，依据见节首；模块与名称分布见 7.2） |
| TS2367：This comparison appears to be unintentional because the types 'X' and 'X' have no overlap. | 6 | 已判 ts 生成器为枚举跨构造器相等比较未降级（Haxe 允许比较不同构造器并返回 false；PushInLineWideCapacityTestSupport.ts:30 样本） |
| TS2869：Right operand of ?? is unreachable because the left operand is never nullish. | 3 | 已判 ts 生成器为左操作数已判非空时仍保留 ??（LineRepair.ts:456 样本） |
| TS2552：Cannot find name 'X'. Did you mean 'X'? | 1 | compare 函数导出/导入登记缺陷（compareFontMetricsRequest @ FontMetrics.ts:84，同 TS2724；第二次复测时报 TS2304，类型定义装入后 tsc 给出拼写建议换为本骨架） |
| TS2540：Cannot assign to 'X' because it is a read-only property. | 2 | 已判 ts 生成器为只读字段生成策略错误（TestTraceStore.ts:53 与 :58 的 lines 字段） |
| 合计（求和校验） | 706 | 与错误总数相等（本轮为 census11 复测值（`3044bf91`，日志 /tmp/census11/ts-typed.log）；前四轮为 1127、1004、722 与 714） |

判定进度小结（census11 复测后，706 条全部为引擎侧）：测量环境条目已随
第 2.2 节配方全部解析，现存 16 类全部归到 ts 生成器，涉及修复位置文件
TsExpr.hx、TsDecl.hx、TsImports.hx、Compiler.hx、TsRuntime.hx（逐类机制
清单见 tsprobe2 r1 报告）。已按修复位置开列的修复项：F3aq（runtime 模块
F3as（org 108 条，org108 r1 判定；修复任务 tsorg r2 已合并
`87255540`，条目待新基线复测后删除）。
谓词不一致构造的跨目标表现（tdart2 r1 与 rustsem r1 分别在两侧证实）。
剩余工作是按目标优先级派发 F3aq 与其余判定类的修复位置。

### 7.2 大类内部分解（产出命令见第 2.2 节）

2026-09-07 晚 `d0df20db` 复测（日志 /tmp/census7/ts.log）重抽了 TS2304 与
TS2341 两张名称分布表并更新为本次值；TS2448、TS2451、TS2724、TS2339
各表本轮逐项核对与 2026-09-06 首测相同，仍标首测值；判定列已并入
tsprobe2 r1 的结论。同日第三次复测（日志 /tmp/census7/ts-typed.log）重抽
TS2307、TS2304、TS2305 三表并更新为本次值（测量环境模块名全部解析后，
TS2307 只剩引擎侧条目）。

TS2307 的模块分布（求和 6，第三次复测值）：

| 计数 | 模块名 | 判定 |
|---:|---|---|
| 3 | ./../../../runtime/SortedTable.ts | 已判 ts 生成器为导入路径问题（随第 7 节 TS2307 行） |
| 1 | ../../../../ts-gen/runtime/SortedTable.ts | 同上 |
| 1 | ../../../../ts-gen/org/tiqian/linebreak/LiangHyphenatorTest.ts | 同上 |
| 1 | ./../../../../haxe/Exception.ts | 同上 |

TS2304 的名称分布（求和 171，census11 复测值；Ic 103 条在 `d0df20db` 消失、
org 2→108 条为该轮新形态；compareFontMetricsRequest 1 条第三次复测起
换骨架为 TS2552；pi 4 条与 bi 4 条在 census11 消失、count 系列重编号，
归 knamefix-r8，见节首）：

| 计数 | 名称 | 判定 |
|---:|---|---|
| 108 | org | 已判 ts 生成器并定位机制（org108 探针，2026-09-08）：默认值展开保留的完整静态路径 `org.tiqian.core.Ic.Zero` 在 `TsExpr.hx:179` 进入 coalescingStaticFieldText（`TsExpr.hx:244-258`），`Context.getType` 对 abstract `Ic` 不返回 TInst，异常与非 TInst 都被吞掉后在 `:258` 原样返回完整路径，且该分支不调用 `imports.value`（正常静态引用路径在 `TsExpr.hx:1478-1479` 既输出短名又登记 import），生成文件没有 `org` 绑定，TS2304 落在首段 `org`；抽样四位点（AnnotationGeometryStageCoverageTest.test.ts:65、FontInstanceMetricsRequestTest.test.ts:31、LineBreakPlanningStageCoverageTestSupport.ts:42、LineBreakRepairEngineTestSupport.ts:46）全部同一机制，源头是 ParagraphStyle.hx:69 的 `blockIndent == null ? Ic.Zero : blockIndent` 默认值；形态切换的引入提交在 `7606ff85..d0df20db` 区间内检索没有找到对应提交（修复项 F3as） |
| 33 | kind | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 5 | region | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定；与 rust E0425 的 region、kotlin 3.4 节 region 同为 @:dataClass 默认参数内联形参泄漏这一构造，rustprobe r1 跨目标对照） |
| 3 | text | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定；同 region 行的跨目标同一构造假设） |
| 3 | __functional_shim | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 2 | SortedMap | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 2 | NodeFileSystem | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 2 | count7 | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定；census11 重编号新名） |
| 2 | count27 | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定；census11 重编号新名，原 count25） |
| 2 | count26 | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 2 | count12 | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定；census11 重编号新名，原 count11） |
| 1 | count | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定） |
| 1 | cornerRadius | 已判 ts 生成器（随第 7 节 TS2304 行的分组判定；同 region 行的跨目标同一构造假设） |
| 1 | compareShapingEvidenceKey | 已证实 ts 生成器：compare 函数导出/导入登记缺陷（同 TS2724） |
| 1 | compareRecordedShapingResult | 同上 |
| 1 | compareRecordedFontMetrics | 同上 |
| 1 | compareMetricsEvidenceKey | 同上 |
| 1 | compareGlue | 同上 |

TS2448 的文件分布（求和 215）：LayoutDumpFormat.ts 214 条（行 90 至 398
间）、ShapingEvidenceJson.ts 1 条（:546）。

TS2451 的名称分布（求和 21，census11 复测值）：index 6、_g 5、row 4、
parseHexCode 2、inkTop 2、inkBottom 2。rubyIndex 2 条在 census11 消失
（归 knamefix-r8，见节首）；TS2448、TS2724、TS2339 三表
计数在 census11 不变，仍标旧轮值。

TS2341 的名称与类分布（求和 4，`d0df20db` 复测值；get_ 前缀 10 条属 F3e，
已随修复降为 0，对应五行删除）：

| 计数 | 属性（所属类） | 判定 |
|---:|---|---|
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

TS2305 的模块与名称分布（求和 25，第三次复测值）：`@tiqian/runtime` 的
UString 8、floatToI32 6、i32ToFloat 4（runtime 模块导出缺失，依据见第
7 节节首）；./TextStyle.ts 的 compareTextStyle 2 条，./Size.ts 的
compareSize、./LineBox.ts 的 compareLineBox、./InlineBoxSpan.ts 的
compareInlineBoxSpan、./GlyphRun.ts 的 compareGlyphRun、./Cluster.ts 的
compareCluster 各 1 条。

TS2339 的名称分布（求和 14）：kind 6、copy 6、push 1、insert 1。

TS2551 全类 28 条（全部 get_strategyName）已随 F3e 修复降为 0，名称分布
表删除（进度见第 12 节 tsgetcal-r6 行）。

## 8 Dart 编译普查（2026-09-08 census12 复测）

轮次历史（gen 与 tests 合计）：首测 2026-09-06（boring `31627b5c`、tiqian
`17a646da`，检出 /tmp/tiqian-dartic，日志 /tmp/dartic-census-gen.log 与
dart-census-tests.log，逐类表 /tmp/dartic-code-combined.txt）2931 条、35 个
错误码；`e01b03b3` 复测（/tmp/tiqian-census6，日志 dart-gen2.log 与
dart-tests2.log）2606 条、33 码（knullinit 守卫把 UNCHECKED 420→143、
dartifget 两码降 0）。判定来源：T-dart r1
（/tmp/dispatch-state/tiqian-tdart-r1.report.md，35 码抽 33 类，两处引用
经派发方复核降级）与 tdart2 r1
（/tmp/dispatch-state/boring-tdart2-r1.report.md，五类升修复位置、三处
跨目标同源证实，引用与样本经派发方逐处复核；接收方两处未声明插桩由
派发方还原）。修复轮：dartguard r1 至 r6（F3l 全部、F3m 至残余 13 条，
`f42c38c9`、`18d5d8d1`、`e4952dab`、`b4b574b5`）、dartstd-r2 合并
`c629b1d1`（URI_DOES_NOT_EXIST 36→2，同笔新暴露 13 条进入各自错误码
判定队列）、fph32-r1 合并 `014d9640`（UNDEFINED_FUNCTION 58→46）、
dunitsurf-r2 合并 `fc2b53b6`（dart 运行时崩溃消除、test:dart 链首次走到
analyze 步；净增 32 条为 charCodeAt 可空结果流入非空 Int 语境：
ARGUMENT_TYPE +17、UNCHECKED +14、RETURN_OF_INVALID +1，开列 F3at）、
census11（`3044bf91`）1695 条、census12（`a72f994d`）1594 条（唯一变动
NOT_ENOUGH_POSITIONAL_ARGUMENTS 105→4，F3ah 经 dartargs-r2 修复生效，
残余 4 条的按构造器名分布未逐条抽取）。

F3at 的修复与 A/B 验证：dartnull-r1 与 r2 死于渠道端点挂起零交付；r3
（terra）交付 dart 侧修复 `819f6d3a`（报告
/tmp/dispatch-state/boring-dartnull-r3.report.md），派发方在消费树 A/B
实测（vendored 由 `a72f994d` 换 `819f6d3a` 重生成，日志
/tmp/dartnull-ab-base.ml 与 dartnull-ab-fix.ml）三个错误码 170→153、
26→12、2→1、生成侧总量 1193→1161，差值与 F3at 的 census12 新增数一致，
其他错误码零扰动；r3 样本未复现三个错误码的原因判定为 r1 任务书规格
缺陷（样本局部不标注，消费树触发错误的是标注 `Int` 的局部）。r4（terra）
补 kotlin、swift、dart 测试侧四处漏补非空断言（`c25cc3c1`、`17b3d3a8`、
`9f0b52df`），rust 越界断言裁定为断言形态问题、非缺陷掩蔽；r5（terra）
交付 rust 双 unwrap_or 去重 `7994eceb`。任务分支六笔经合并 `b822afee`
进入 boring main 并已推送，条目待 census13 复测删除。

dunitsurf-r3 的 URI 回归（census11 实测明细）：`db0f713a` 把 resident
模块并入合并 runtime 库，消费配置下不再单独写出 runtime/ 目录（census10
树 dart-gen/lib/runtime/ 实测有 sorted_table.dart 与 string_tools.dart，
census11 树只剩 haxe、org、std 三目录），四处对 runtime/sorted_table.dart
的 import 断链报 URI_DOES_NOT_EXIST（生成侧 liang_hyphenator.dart:3、
parse_tex_hyphenation_patterns.dart:3、parsed_tex_hyphenation.dart:2，
测试侧 liang_hyphenator_test.dart:7）；已写出文件内的 `_codePointAt` 4
条随文件不再写出而消失；parse_tex_hyphenation_patterns.dart:99 的两条
与 liang_hyphenator_test.dart:56 的返回位一条因 import 不可解析、类型
检查未到达错误位而不再报（掩蔽，import 恢复后错误会回来）。`bi`、
`rubyIndex`、可空 for-in 迭代子新条归 knamefix-r8 的循环识别（未逐笔
核对）。F3i 修复位置随合并 runtime 库机制重判。

| 错误码 | gen | tests | 合计 | 判定与处置 |
|---|---:|---:|---:|---|
| `UNDEFINED_IDENTIFIER` | 212 | 64 | 276 | 名称分布见 8.2（58 个名字全列，census11 复测）；类内全部类型名条目（2026-09-07 复测判定的 8 个名字合计 705 条，与其余类型名条目 FontMetricSource 12、InteriorPunctuationStyle 5、CjkPunctuationGlyphPolicy 5、AutoSpaceMode 4、LineEndPunctuationStyle 3、KinsokuLevel 3、HangingPunctuationStyle 2 和六个单条名字）已随 F3l（dartguard r1 至 r6，提交号见第 12 节）降为 0，逐笔提交与名字的对应未逐条验证；残余 276 条全部为局部名与测试支持类名，未判定 |
| `REFERENCED_BEFORE_DECLARATION` | 314 | 96 | 410 | 形状证实：抽样含导入前缀与局部名同名冲突（cluster_role_resolution.dart:55 生成 `final cluster = cluster.Cluster(...)`）与不带前缀的类名（:64 的 `ResolvedClusterRange`）两形；探针引用的 Compiler.hx:216-220 经派发方复核是测试函数排序，与样本不吻合，已否证；layout_dump_format.dart 215 条与 ts TS2448 同源的假设保留（ts 侧 TS2448 已判生成器，本码随 T-dart 第二轮复核）；文件分布见 8.2（与首测逐项相同）；机制位置待定位；随 T-dart 第二轮 |
| `ARGUMENT_TYPE_NOT_ASSIGNABLE` | 170 | 118 | 288 | 假设：可空 int? 给 int（codeUnitAt 闭包位），连可空守卫（F3m 同构造）；目标类型分布见 8.2（23 种全列，该表数值取自首测日志）；F3at 的 charCodeAt 结果 `int?` 给 `int` 形参 17 条（clreq_punctuation_policies.dart:35:37 实测，修复已合并待 census13 复测）；dartstd-r2 期新增的 SortedMapTable 两条现为 import 断链掩蔽（见节首 URI 回归段）；double 67 条是否数值转换待抽样；随 T-dart 第二轮 |
| `UNCHECKED_USE_OF_NULLABLE_VALUE` | 26 | 2 | 28 | 构成三部分：F3m 残余 13 条（修复任务 dartguard r1 至 r6 的提交号见第 12 节；残余逐文件分布：生成侧 ParagraphStyle 1、Justifier 3、LineBreakPlanningStageCoverageTestSupport 2、LineRepair 2、PunctuationGeometryLedger 3，测试侧 FontPolicyCoverageTest 1、TextShaperCoverageTest 1，r6 报告 /tmp/dispatch-state/boring-dartguard-r6.report.md）、F3at 的 charCodeAt 可空结果比较运算 14 条（clreq_punctuation_advance_policy.dart:27:12 实测，修复已合并待 census13 复测）、可空 for-in 迭代子 1 条（line_repair.dart:79，knamefix-r8 循环改写后新暴露、待归面）；r6 同期实验 `cd34ca32` 扩大断言面使总数回归 1755，已整笔回退，不得原样重试 |
| `NOT_ENOUGH_POSITIONAL_ARGUMENTS` | 1 | 3 | 4 | 修复位置（tdart2 r1）：coalescing 内层构造的省略实参没有补全。调用点经 `DartExpr.hx` 的 `completeCoalescingCallArgs`（e01b03b3 位于 235-244，派发方复核），它传给 `omittedCallDefaults` 的只有模块路径与方法名，内层构造的类名被丢弃；`omittedCallDefaults`（packages/compiler/DefaultArgExpander.hx:1534-1578）用 `Context.getType(modulePath)` 解析且只认 TInst，模块无同名主类型（PunctuationModel.hx 的 PunctuationAtomBuilder）或主类型是接口（GreedyLineBreaker、LruWidthIndependentAnnotationCache）时返回 null，省略实参不填充，调用点渲染零参而构造声明要求参数（声明侧 DartDecl.hx:838-888 只把带默认值的参数排进可选组，且按字段名匹配的判定对构造参数名不成立）。修复项 F3ah（修复任务 dartargs 经 r1 与 r2 交付，合并 `a72f994d`；census12 复测全类 105→4（生成侧 71→1、测试侧 34→3），census11 的按构造器名计数 PunctuationAtomBuilder.new 56、GreedyLineBreaker.new 39、LruWidthIndependentAnnotationCache.new 10 对应修复前分布，残余 4 条的按构造器名分布未逐条抽取） |
| `PREFIX_SHADOWED_BY_LOCAL_DECLARATION` | 32 | 42 | 74 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_METHOD` | 58 | 11 | 69 | 形状证实（T-dart r1 抽样读过生成代码）；接收类型为 List 的 23 条保持 Haxe 数组方法名生成到 dart 的 List 接收者的假设；新增 `toDouble @ bool` 形 5 条的原因没有查明（gen 侧，2026-09-07 复测新增）；dartstd-r2 合并（`c629b1d1`）后生成侧 62，新增 4 条为新写出的 runtime/sorted_table.dart 内 `_codePointAt` @ `SortedTable`（未判定）；census11 生成侧 62→58，`_codePointAt` 4 条随 runtime/sorted_table.dart 不再写出而消失（dunitsurf-r3 的 `db0f713a`，见节首 URI 回归段）；方法与接收类型分布见 8.2（24 对全列，census11 复测）；机制位置待定位；随 T-dart 第二轮 |
| `EXPECTED_TOKEN` | 56 | 0 | 56 | 形状证实：非法 `int??` 双问号类型渲染（cjk_font_role_classifier.dart:23-24 生成 `final int?? l` 实测）；与 MISSING_ASSIGNABLE_SELECTOR、ILLEGAL_ASSIGNMENT_TO_NON_ASSIGNABLE、MISSING_IDENTIFIER、DOT_SHORTHAND_MISSING_CONTEXT 四码共享同一形状，五码文件分布重叠；期待符号分布：缺分号 49、缺右括号 4、缺冒号 2、缺右花括号 1（与首测相同）；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_FUNCTION` | 46 | 0 | 46 | 修复位置分三个分支（tdart2 r1，名称分布见 8.2）：compare 前缀 41 条，是否生成比较函数的判定与引用侧不一致：判定函数 `canEmitDataClassComparator`（packages/compiler/PolicyQueries.hx:163-171）经 `isDataClassFieldKey`（:173-192）只认 Int、enum、String、dataClass 与其可空/只读数组包装，含 Bool 或 Float 字段的数据类不生成比较函数，而引用侧 DartDecl.hx 的 NullableArray 与 PlainField 分支（e01b03b3 位于 :311-312 与 :326）只复查 `:dataClass` meta、不复查是否生成的判定，引用照发（修复项 F3ai；与 ts TS2724/TS2305 同源证实）；mkdirSync 与 writeFileSync 各 1 条，测试 extern 的静态成员在调用点降级为不带限定名的名字（DartExpr.hx:1605-1606 走 fail 分支的调用点形状），定义不随 tests 产物目录写出（修复项 F3ak）；floatToI32 7 条加 i32ToFloat 5 条已随 F3aj（fph32-r1，见第 12 节）降为 0；dartstd-r2 期新增 justifier.dart 的 `sumOfFloat` 1 与 `forEach` 2（未判定，8.2 表已补两行） |
| `MISSING_ASSIGNABLE_SELECTOR` | 49 | 0 | 49 | 形状证实：非法 `int??` 双问号类型渲染（与 EXPECTED_TOKEN 共享形状，五码文件分布重叠）；机制位置待定位；随 T-dart 第二轮 |
| `ILLEGAL_ASSIGNMENT_TO_NON_ASSIGNABLE` | 49 | 0 | 49 | 形状证实：非法 `int??` 双问号类型渲染（与 EXPECTED_TOKEN 共享形状，五码文件分布重叠）；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_PREFIXED_NAME` | 22 | 22 | 44 | 修复位置分两个分支（tdart2 r1；名称分布按 dartstd-r2 合并态实测：UString 21（生成侧 6、测试侧 15）、DefaultHyphenator 15（生成侧 8、测试侧 7）、PunctuationGluePlacements 6、SortedMap 2）：DefaultHyphenator 加 PunctuationGluePlacements 21 条，`DartExpr.hx` 的 `coalescingStaticCallText`（e01b03b3 位于 246-258，派发方复核）把静态调用渲染成类限定引用，没有 `staticRef`（:1619 起）对 statics-only 类去类名的降级路径（修复项 F3al）；UString 21 条，std.UStringRT 的成员走运行时限定渲染 `runtime.UString.成员`（DartExpr.hx:1607-1608 一带），但运行时模块的拼接（dartcompiler/Compiler.hx:388-396）只追加 parts 非空的 resident，tiqian 的 engine-haxe/targets/classes.hxml 没有任何 `runtime.*` 清单条目（boring examples/dart.hxml 带 `runtime.UString` 等条目），std.UStringRT 又是 extern 不产生 parts，定义与引用两侧都落空（修复项 F3am；dartstd-r2 合并 `c629b1d1` 后 runtime.dart 已写出但其中 UString 出现 0 次，extern 不产生 parts 的落空机制不变，修复位置不变）；SortedMap 2 条为 dartstd-r2 合并后新增（paragraph_shaping_stage.dart:49 与 replayable_font_backend.dart:40 经前缀 sorted_map 引用），std/sorted_map.dart 已写出但为仅含生成头注释的空壳文件（实测 1 行），名字仍未定义，与 F3am 的 extern 空壳构造同类（未逐条验证因果） |
| `URI_DOES_NOT_EXIST` | 3 | 3 | 6 | F3i（dartstd-r2 主体修复，提交号见第 12 节）在 census11 部分回归后的现状：生成侧 3 条与测试侧 sorted_table 1 条为合并 runtime 库机制的 import 断链（见节首 URI 回归段）；测试侧 test_host.dart（main.dart:6）与跨目录 liang_hyphenator_test.dart（line_break_coverage_test.dart:8）两条为 dartstd-r2 期残余，写出条件的不一致尚未定位；路径明细见 8.2（census11 重抽）；修复位置随合并 runtime 库机制重判 |
| `READ_POTENTIALLY_UNASSIGNED_FINAL` | 36 | 0 | 36 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `MISSING_IDENTIFIER` | 27 | 0 | 27 | 形状证实：非法 `int??` 双问号类型渲染（与 EXPECTED_TOKEN 共享形状，五码文件分布重叠）；dartguard 合并后 27|0，较首测 34|1 少 8 条，减少的 8 条与哪笔修复对应未逐条查明；机制位置待定位；随 T-dart 第二轮 |
| `INVOCATION_OF_NON_FUNCTION_EXPRESSION` | 0 | 31 | 31 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `DOT_SHORTHAND_MISSING_CONTEXT` | 27 | 0 | 27 | 形状证实：非法 `??` 双问号类型渲染（annotation_geometry_stage.dart:165 生成 `ClusterGeometryDecisionInfo?? g` 实测；与 EXPECTED_TOKEN 共享形状）；机制位置待定位；随 T-dart 第二轮 |
| `DUPLICATE_DEFINITION` | 16 | 1 | 17 | 修复位置分四个分支（tdart2 r1）：int?? 双问号类型的连锁错误 10 条，并入第 8 节既有 `??` 双问号五个错误码的连锁条目，不独立立项；同名合成临时变量 5 条（line_geometry_stage.dart:234-246 三个 `_g`、line_adjustment_stage.dart:392 与 :471 的 `index`，`DartExpr.hx` 的 localName（e01b03b3 位于 3498 起）对命名临时变量原样输出、同块不去重）加 Haxe 源同块 var 重声明 2 条（layout_queries.dart:641 与 :671 的 `rubyIndex`，源 LayoutQueries.hx:584/:597，TS2451 同源证实）合计 7 条（修复项 F3an）；extension type 的表示字段与成员同名 1 条（ic.dart:5 `extension type Ic(double count)` 里 `count()` 与表示字段 `count` 同名，DartDecl.hx 的 valueTypeDecl 取第一个构造参数名为表示字段，e01b03b3 位于 422-431）（修复项 F3ao；`rubyIndex` 两条降一条归 knamefix-r8，三目标联动见节首） |
| `PREFIX_COLLIDES_WITH_TOP_LEVEL_MEMBER` | 5 | 6 | 11 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `MISSING_DEFAULT_VALUE_FOR_PARAMETER` | 11 | 0 | 11 | 形状证实：非空参数的隐式默认值为 null（ClreqProfile 可选参数实测）；探针引用的 DartDecl.hx:260-300 经派发方复核落在比较函数合成代码内，与参数签名不吻合，定位存疑；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_ENUM_CONSTANT` | 8 | 0 | 8 | 形状证实：枚举构造器名空串或保留字（unicode_punctuation_boundary_resolver.dart:208-219 生成 `Dir.final` 实测，final 是 dart 保留字）；机制位置待定位；随 T-dart 第二轮 |
| `NOT_INITIALIZED_NON_NULLABLE_INSTANCE_FIELD` | 4 | 0 | 4 | 修复位置（tdart2 r1）：`DartExpr.hx` 的 `coalescedBodyFields`（e01b03b3 位于 523-553，派发方复核）只扫构造函数顶层语句里的 `this.x = …` 赋值，if/else 分支体里的守卫赋值（line_breaker.dart:28 的 `_kinsoku` 等四字段，源 LineBreaker.hx:50-57）不被收集，字段声明侧 DartDecl.hx:639-643 因此不标 `late`。修复项 F3ap |
| `NON_EXHAUSTIVE_SWITCH_STATEMENT` | 4 | 0 | 4 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_OPERATOR` | 3 | 0 | 3 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `INVALID_ASSIGNMENT` | 3 | 0 | 3 | 形状证实：dart 侧赋值位没有 int 到 double 的转换（line_adjustment_stage.dart:148、:151、:159 生成 `visualWidth = (visualWidth).round()` 实测，与 kotlin F3j 的赋值位同构造；首测 7 条，本次复测 3 条的下降原因未单独查明）；机制位置待定位；随 T-dart 第二轮 |
| `IMPLICIT_THIS_REFERENCE_IN_INITIALIZER` | 3 | 0 | 3 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `RETURN_OF_INVALID_TYPE` | 2 | 0 | 2 | 形状证实（T-dart r1 抽样读过生成代码）；F3at 的 charCodeAt 结果 int? 处于返回 Int 的返回位 1 条（修复已合并待 census13 复测）；测试侧 liang_hyphenator_test.dart:56 的 `SortedMapTable<String, dynamic>` 返回位 1 条现为 import 断链掩蔽（见节首 URI 回归段）；机制位置待定位；随 T-dart 第二轮 |
| `LIST_ELEMENT_TYPE_NOT_ASSIGNABLE` | 0 | 2 | 2 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `RETURN_OF_INVALID_TYPE_FROM_CLOSURE` | 1 | 0 | 1 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `NON_TYPE_AS_TYPE_ARGUMENT` | 1 | 0 | 1 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `INSTANCE_MEMBER_ACCESS_FROM_STATIC` | 1 | 0 | 1 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `EXTRA_POSITIONAL_ARGUMENTS` | 1 | 0 | 1 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `CONFLICTING_METHOD_AND_FIELD` | 1 | 0 | 1 | 形状证实（T-dart r1 抽样读过生成代码）；机制位置待定位；随 T-dart 第二轮 |
| `UNDEFINED_CLASS` | 1 | 0 | 1 | 形状证实：dartstd-r2 合并（`c629b1d1`）后新增，traced_assertions.dart:651 引用类名 'Exception'（生成代码对 haxe.Exception 的 dart 侧引用）；机制位置待定位；随 T-dart 第二轮 |
| 合计（求和校验） | 1193 | 401 | 1594 | 与错误总数相等；本行为 census12 复测值（`a72f994d`，日志 /tmp/census12/dart-gen.log 与 dart-tests.log）；census11 复测值 1695（`3044bf91`，/tmp/census11）；`d38459ab` 复测值为 1702（/tmp/census10），fph32-r1 合并（`2382140b`，合并提交 `014d9640`）后为 1670，dartguard-r6 合并态（`b4b574b5`）为 1682，dartstd-r2 合并态（`c629b1d1`）为 1686，dartguard r4 合并态（`e4952dab`）为 1707，r3 合并态为 1739，首测与 2026-09-07 复测值为 2931 与 2606 |

相对首测计数已降为 0 并从表内删除的错误码：UNDEFINED_GETTER（32 条，
dartifget boring `14c5031d`）、NON_ABSTRACT_CLASS_INHERITS_ABSTRACT_MEMBER
（6 条，随同修复降为 0）。

判定进度小结（2026-09-08 census12 复测；8.2 各表仍标各自标注轮次）：
定位到修复位置的错误码 8 个：UNCHECKED_USE_OF_NULLABLE_VALUE（F3m 残余
13 条；另有 F3at 的 14 条与 census11 新增可空 for-in 迭代子 1 条归
knamefix-r8 循环改写、待归面；dartguard r1 至 r6 已交付合并）、
URI_DOES_NOT_EXIST 6 条（F3i，修复位置随合并 runtime 库机制重判，见
节首 URI 回归段）、NOT_ENOUGH_POSITIONAL_ARGUMENTS 4 条（F3ah 残余）、
UNDEFINED_FUNCTION 46 条（F3ai 41、F3ak 2、dartstd-r2 后新增未判定 3；
F3aj 的 floatToI32 7 加 i32ToFloat 5 已由 fph32-r1 修复降 0）、
UNDEFINED_PREFIXED_NAME 44 条（F3al 21、F3am 21、SortedMap 2 未归类）、
DUPLICATE_DEFINITION 17 条（10 条并入 `??` 双问号连锁条目、F3an 6、
F3ao 1）、NOT_INITIALIZED_NON_NULLABLE_INSTANCE_FIELD 4 条（F3ap）；
ARGUMENT_TYPE_NOT_ASSIGNABLE 与 RETURN_OF_INVALID_TYPE 各自的
charCodeAt 新增部分（+17 与 +1）为 F3at，修复已合并 `b822afee`、待
census13 复测删除；UNDEFINED_IDENTIFIER 类内类型名条目（F3l）已由
dartguard 降为 0，该错误码残余 276 条全部为局部名与测试支持类名，回到
未判定档；UNDEFINED_METHOD 的 compareTo 形 9 条由 NullableScalar 分支
渲染 `.compareTo` 解释，预期随 F3ai 消除，不预先承诺计数。探针定位待
因果验证这一档已没有条目；其余 25 类为形状证实或假设，其中
EXPECTED_TOKEN、MISSING_ASSIGNABLE_SELECTOR、
ILLEGAL_ASSIGNMENT_TO_NON_ASSIGNABLE、MISSING_IDENTIFIER、
DOT_SHORTHAND_MISSING_CONTEXT 五码合计 208 条共享非法 `??` 双问号类型
渲染形状，是形状证实里计数最大的一组。T-dart 第二轮的剩余工作是给
形状证实类定位机制位置、证实或否证其余跨目标假设
（ARGUMENT_TYPE_NOT_ASSIGNABLE 的 double 67 条是否数值转换、
UNDEFINED_METHOD 新增 `toDouble @ bool` 5 条的原因查明都在其列）。

### 8.2 大类与中类内部分解（census11 部分重抽，产出命令见第 2.2 节）

UNDEFINED_IDENTIFIER、UNDEFINED_METHOD 与 URI_DOES_NOT_EXIST 三张表按
census11 复测日志（/tmp/census11/dart-dart-gen.log 与
dart-dart-gen-tests.log）重新抽取（2026-09-08）：UNDEFINED_IDENTIFIER
删去名字 `bi` 的行（4 条随 knamefix-r8 消失），其余 58 个名字与上一轮
逐项相同；UNDEFINED_METHOD 删去 `_codePointAt` @ `SortedTable` 行（4 条
随 runtime/sorted_table.dart 不再写出而消失），其余 24 对逐项相同；URI
表为断链后的 6 条现值。UNCHECKED_USE_OF_NULLABLE_VALUE 表仍标
dartguard r3 合并态（boring `f9b446df`，日志 /tmp/dartguard3-final-gen.log
与 dartguard3-final-tests.log，31 条未重新按形状分桶；census11 现值 28 条
见第 8 节该行，含 F3at 的 14 条与 for-in 新增 1 条）。其余各表仍基于
2026-09-07 复测日志（/tmp/census6/dart-gen2.log 与 dart-tests2.log，
boring `e01b03b3` 态）：其中 ARGUMENT_TYPE_NOT_ASSIGNABLE 表（23 种全列）
对应生成侧首测值 172，其后的下降未重新逐条抽取。
UNDEFINED_PREFIXED_NAME 的名称分布仍标 dartstd-r2 合并态（`c629b1d1`）。

UNDEFINED_IDENTIFIER 的名称分布（求和 276，census11 复测值；dartguard 把
类型名条目降为 0 后，残余全部为局部名与测试支持类名，判定列全部未判定；
带「测试侧」标记的四个名字合计 64 条来自测试日志，其余 54 个名字合计
212 条来自生成日志；`bi` 4 条在 census11 消失，归 knamefix-r8 循环
识别这一组提交）：

| 计数 | 名称 | 判定 |
|---:|---|---|
| 33 | `kind` | 未判定（测试侧） |
| 15 | `PunctuationGeometryStageCoverageSupport` | 未判定（测试侧） |
| 13 | `JustifierTestSupport` | 未判定（测试侧） |
| 12 | `plan` | 未判定 |
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
| 5 | `halt` | 未判定 |
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
| 3 | `text` | 未判定（测试侧） |
| 3 | `strongReason` | 未判定 |
| 3 | `startPrior` | 未判定 |
| 3 | `role` | 未判定 |
| 3 | `prevKind` | 未判定 |
| 3 | `naturalPrior` | 未判定 |
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
| 2 | `fivePowers` | 未判定 |
| 2 | `center` | 未判定 |
| 1 | `twoPowersBuilder` | 未判定 |
| 1 | `fivePowersBuilder` | 未判定 |
| 1 | `enUsCache` | 未判定 |
| 1 | `count` | 未判定 |

UNCHECKED_USE_OF_NULLABLE_VALUE 的形状分布（求和 31，生成侧 28 条与
测试侧 3 条合并，r3 合并态计数；消息里的名字以 X 代替；knullinit 系列
的 dart 侧守卫（boring `9e0537d0`，合并 `23f4bf63`）已修掉 property 形的
314 条，修复任务 dartguard 的可空接收者守卫（`b5bed1e3`，合并
`f42c38c9`）又修掉 112 条，残余 31 条的修复位置仍是 dart 可空接收者守卫
缺失（F3m，与 kotlin 修复位置 A 同构造），逐条三件套由 dartguard r4
补齐；r4 合并（`e4952dab`）后残余 17 条，见第 8 节该行）：

| 计数 | 形状 | 判定 |
|---:|---|---|
| 6 | The method 'X' can't be unconditionally invoked because the receiver can be 'null'. | F3m（生成侧 5、测试侧 1；r3 合并态计数） |
| 3 | The property 'X' can't be unconditionally accessed because the receiver can be 'null'. | 同上（生成侧 1、测试侧 2；首测 314 条已由 boring `9e0537d0` 修复） |
| 16 | The operator 'X' can't be unconditionally invoked because the receiver can be 'null'. | 同上（全部生成侧） |
| 6 | A nullable expression can't be used as a condition. | 同上（全部生成侧） |

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
是否数值转换待第二轮抽样；double 与 num 两形状相对首测的下降原因未单独
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

UNDEFINED_METHOD 的方法与接收类型分布（求和 69，24 对全列，census11
复测值，记法为方法 @ 接收类型；`_codePointAt` @ `SortedTable` 4 条随
runtime/sorted_table.dart 不再写出而消失（dunitsurf-r3 的 `db0f713a`），
其余各对与 dartstd-r2 期逐项相同；`toDouble` @ `bool` 形 5 条为
2026-09-07 复测新增，原因没有查明；compareTo 形 9 条由 NullableScalar
分支对非 Comparable 数据类实例渲染 `.compareTo` 解释
（DartDecl.hx:279-281 一带，tdart2 r1），预期随 F3ai 的修复消除，计数在
合并复测后重数）：

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
| 1 | `compareTo` @ `RubyLineHeightDecisionInfo` | NullableScalar 渲染 `.compareTo`，预期随 F3ai 消除（tdart2 r1） |
| 1 | `compareTo` @ `MaxLinesDecisionInfo` | 同上 |
| 1 | `compareTo` @ `LineSpacingDecisionInfo` | 同上 |
| 1 | `compareTo` @ `LineRepairDecisionInfo` | 同上 |
| 1 | `compareTo` @ `LineLengthGridDecisionInfo` | 同上 |
| 1 | `compareTo` @ `LineCandidate` | 同上 |
| 1 | `compareTo` @ `KinsokuDecisionInfo` | 同上 |
| 1 | `compareTo` @ `InlineObjectLineHeightDecisionInfo` | 同上 |
| 1 | `compareTo` @ `FirstLineIndentDecisionInfo` | 同上 |

UNDEFINED_FUNCTION 的名称分布（求和 46，消息全部为 The function 'X'
isn't defined.；末两行为 dartstd-r2 合并后新增；floatToI32 7 条与
i32ToFloat 5 条已由 fph32-r1（`2382140b`，合并 `014d9640`）修复，两行
删除）：

| 计数 | 名称 | 判定 |
|---:|---|---|
| 2 | `compareTextStyle` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 2 | `compareFontMetricsRequest` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 2 | `compareRawFontMetrics` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareAutoSpacePolicy` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareAdjustmentStylePolicy` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `comparePunctuationWidthPolicy` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareBopomofoGlyphPlacement` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareMetricDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareClusterGeometryDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareAutoSpaceDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareRubyDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareShapingDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `comparePunctuationDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareSpacingDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareJustificationDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareLineEdgeTrimDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareDecorationDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareDecorationSegmentInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareInlineBoxDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareInlineObjectDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareInlineObjectPunctuationAttachmentDecisionInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareParagraphStyle` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareLayoutConstraints` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareInlineBoxSpan` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareInlineObjectSpan` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareSize` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareCluster` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareGlyphRun` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareLineBox` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareLineRepairCandidateInfo` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareLayoutFontMetrics` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareLineCandidate` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareRepairCandidate` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareGlue` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareShapingEvidenceKey` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareRecordedShapingResult` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareMetricsEvidenceKey` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `compareRecordedFontMetrics` | 修复位置：是否生成比较函数的判定与引用侧不一致（F3ai） |
| 1 | `mkdirSync` | 修复位置：测试 extern 静态成员调用点输出不带限定名的名字（F3ak） |
| 1 | `writeFileSync` | 修复位置：同 F3ak |
| 1 | `sumOfFloat` | 未判定（dartstd-r2 合并后新增，justifier.dart:141） |
| 2 | `forEach` | 未判定（dartstd-r2 合并后新增，justifier.dart:147 与 :155） |

EXPECTED_TOKEN 的期待符号分布（求和 56）：';' 49 条、')' 4 条、':' 2 条、
'}' 1 条（全部未判定）。

URI_DOES_NOT_EXIST 的引用路径分布（求和 6，census11 复测值；目录列 gen 指
dart-gen 内的文件、tests 指 dart-gen-tests 内的文件；已判定 F3i；生成侧
3 条与测试侧 sorted_table 1 条为 dunitsurf-r3 的 `db0f713a` 不再写出
runtime/ 目录后的 import 断链，测试侧 test_host 与跨目录 liang_hyphenator_
test 两条为 dartstd-r2 期已有残余）：

| 计数 | 引用路径 | 目录 |
|---:|---|---|
| 3 | `../../../runtime/sorted_table.dart` | gen（liang_hyphenator.dart:3、parse_tex_hyphenation_patterns.dart:3、parsed_tex_hyphenation.dart:2） |
| 1 | `test_host.dart` | tests（main.dart:6） |
| 1 | `../../../../dart-gen/lib/runtime/sorted_table.dart` | tests（liang_hyphenator_test.dart:7） |
| 1 | `../../../../dart-gen/lib/org/tiqian/linebreak/liang_hyphenator_test.dart` | tests（line_break_coverage_test.dart:8） |

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
| K2 | 各目标逐类判定完成度 | kotlin 33 类：修复位置或挂靠修复项 20、探针待因果 3、形状证实 10、假设 0、未判定 0（3.2 节小结，tattr4 r1）；rust 28 码：E0432、E0053、E0424、E0423、E0308-198、E0599 四子形与 r4 两组已判，E0277 仅 send 5 条已判，其余 372 条在判（第 6 节小结）；swift gen 侧 1 类已判、tests 侧 2 类已判 F3u（第 5 节）；ts 16/16 类已判（第 7 节小结）；dart 修复位置 8 码加 F3at 的 charCodeAt 组，其余 25 类形状证实或假设（第 8 节小结） | 五目标全部类有判定结论 | T-attr、T-swift、T-dart、tsprobe2、rustsem、rustprobe |
| K3 | 各目标错误总数 | kotlin f32 343 / f64 354；rust f32 4548 / f64 4558（合并引入回归，第 6 节）；ts 706；dart 1594（生成侧 1193、测试侧 401）；swift gen 每精度 1、tests 阻断（第 5 节）。kotlin、rust、dart 为 census12 值（`a72f994d`，日志 /tmp/census12）；ts、swift 沿用 census11 值 | 全部 0 | 各节逐类表求和（完整性数字，非派发单位） |
| K4 | kotlin 两个精度目录 warning 计数 | 0 / 0 | 保持 0 | 每次复测 |
| K5 | 各目标重生成退出码 | 八个生成入口全部 0（2026-09-07，/tmp/census6/report.txt：kotlin、swift、rust 各 f32 与 f64，ts、dart） | 全部 0 | 逐处重跑阶段（已完成，第 4 节） |
| K6 | boring 验收命令 | 19 项 gates 统一重跑全部退出码 0：`b822afee`（dartnull 合并态，/tmp/gates-dartnull5-rc.txt）与 `c38a359c`（rustcoalesce-r4 合并态，/tmp/gates-rustcoalesce4m-rc.txt）与 `4644e29b`（f4m 合并态，/tmp/gates-f4mm-rc.txt）；`a72f994d` 合并后曾 17 项通过、3 项失败，两 rust 项失败的交互机制与修复见第 6 节 census12 段，consistency 为既有失败项、已随其后合并转绿；更早合并的 gates 结果与失败定性见第 12 节对应行 | 每次合并后保持 | 不适用 |
| K7 | 五目标普查覆盖 | 八格矩阵逐类表已建（kotlin、swift、rust 各 f32 与 f64 加 ts、dart，第 3、5、6、7、8 节） | 五目标各有逐类表 | 首次普查记录（第 12 节进度表） |

## 11 修复项清单

本清单只留未完成条目。已完成并在新基线复测确认的条目自 2026-09-07 起从
本清单删除，只在第 12 节进度表留行；条目编号保留不复用（第 1 节更新规则）。
已删除的条目：第 0 组 F0a-F0d、第 0.5 组 F0e 至 F0l（含 F0i-逐处重跑与
F0k-核）、第 2 组 F2b、F3f、F3g、F3h、F3k、F3t、T-ts、第 4 组 F4a 至
F4c、第 1 组 T-swift（tswift1 r1 交付，判定并入第 5 节，F3t 与 F3u 随之
开列）、F3n、F3o、F3q（rustfix-r1 合并 `b7054019`，2026-09-07 复测三类
计数 0）、F3v（kf3v-r1 合并 `b7500d45`，2026-09-07 复测计数 0）、
F3e（tsgetcal-r6 合并 `4fa632e6`，2026-09-07 晚 `d0df20db` 复测两个错误类
计数 0）、F3p、F3r、F3s（rustfix r2 至 r4 合并 `e509f6df` 与 `d0df20db`，
2026-09-07 晚 `d0df20db` 复测解析层三类计数 0）、F3ar（tsforce-r1 合并
`908305a0`，2026-09-08 复测 TS2304 类内 TestCore 8 条降为 0）、F3ag
（knamefix-r8 合并 `3044bf91`，2026-09-08 census11 复测 ambiguous
1/1→0，随之消除的条目见第 3 组导语）、F4j（rustmisc-r2 合并
`746742dd`，2026-09-08 census12 复测 E0072 每精度 1→0）；
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
      dartguard（#64）由本会话派出，r1 至 r4 已交付合并，F3l 覆盖的条目
      已降为 0，F3m 残余 17 条由 r5 继续修复。第二轮（tdart2 r1）于 2026-09-07 交付（报告
      /tmp/dispatch-state/boring-tdart2-r1.report.md，基线 boring
      `e01b03b3` 只读，引用与样本由派发方逐处复核），五类探针定位待因果
      验证全部升为修复位置并入第 8 节，修复项 F3ah 至 F3ap 随之开列；
      UNDEFINED_METHOD 的 compareTo 形与 ts TS2724/TS2305、TS2451 的
      跨目标同源在该报告证实。剩余范围：给形状证实类定位机制位置、
      证实或否证其余跨目标假设（ARGUMENT_TYPE_NOT_ASSIGNABLE 的
      double 67 条是否数值转换、UNDEFINED_METHOD 新增 `toDouble @ bool`
      5 条的原因查明、UNDEFINED_IDENTIFIER 类内未判定名字的归属都在
      其列）。C2，P1，S-。

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
十二条。2026-09-07 rustfix-r1 合并（`b7054019`）后 F3n、F3o、F3q 完成
删除，F3p、F3r、F3s 的修复位置按复测现场更正；tdart2 r1 判定并入第
8 节后开列 F3ah 至 F3ap 十条；kf3v-r1 合并（`b7500d45`）后 F3v 完成删除
（接收方在基线分解中发现 slice 调用点是同一错误类的第二个来源，第二笔提交
一并降级，见第 12 节对应行）；dartguard r1 至 r4 合并（`f42c38c9`、
`18d5d8d1` 与 `e4952dab`）后 F3l 完成删除（类型名条目全部降为 0，判据的
其余名字计数不上升，见第 8 节复测），F3m 残余 17 条由 r5 继续修复；
2026-09-07 晚 tsgetcal-r6 合并（`4fa632e6`）后 F3e 完成删除，rustfix
r2 至 r4 合并（`e509f6df`、`d0df20db`）后 F3p、F3r、F3s 完成删除；
ts 第三次复测（第 2.2 节类型配置）后开列 F3aq 与 F3ar 两条，
tsforce-r1 合并（`908305a0`）后 F3ar 完成删除；knamefix-r8 合并
（`3044bf91`，2026-09-08）后 F3ag 完成删除（census11 复测 ambiguous
1/1→0，cannot infer 42/42→6/6 与 function invocation 行的 `range()`
连锁条目随之消除，见 3.2 节表后说明）；rustprobe r1 判定并入第 6 节后开列
F4l 至 F4o 四条。

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
      （提交号与逐轮计数见第 12 节；r4 后形状分布见 8.2），r4 报告
      （/tmp/dispatch-state/boring-dartguard-r4.report.md）记录了当时的逐条
      三件套；r4 曾把 coalescing 内层构造的参数类型具化，实验 `2fe7fac9`
      使 dart 分析错误升到 1506 条量级，已回退，该方向禁用。修复位置为
      dart 生成器对可空接收者的成员访问、调用、运算
      与条件位没有生成守卫（clreq_punctuation_advance_policy.dart:27 的
      int? 接收者直接比较实测；DartExpr.hx:1315-1341 的 binop 守卫位
      派发方在 31627b5c 复核），与 kotlin 修复位置 A（#22）同构造。判据为
      该错误码计数降为 0，其余类计数不上升。C2，P1，S2。
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
- [ ] F3u swift tests 目录 import 头的消费侧配置：覆盖第 5 节 tests 侧
      cannot find 32 与 contextual type 9，每精度 41 条（合并测量下这类错误数为 0，
      证明符号本身可解析）。修复位置为 tiqian 的
      engine-haxe/targets/swift-common.hxml 补 `-D swift-test-import=<模块名>`
      （并与 package-shell 配置一起评估，当前为 none；boring 合同见
      examples/swift.hxml:20-21 与 Compiler.hx:422-425）。判据为生成的
      tests 文件带 import 头且 SwiftPM 构建下测试目标符号解析通过。
      C1，P2，S-。
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
- [ ] F3ah dart coalescing 内层构造省略实参未补全：覆盖第 8 节
      NOT_ENOUGH_POSITIONAL_ARGUMENTS 全类 105 条（按构造器名计数
      PunctuationAtomBuilder.new 56、GreedyLineBreaker.new 39、
      LruWidthIndependentAnnotationCache.new 10，tdart2 r1）。修复位置为
      dartcompiler/DartExpr.hx 的 completeCoalescingCallArgs（e01b03b3
      位于 235-244）对内层构造调用没有把类名传给 omittedCallDefaults，
      以及 packages/compiler/DefaultArgExpander.hx 的 omittedCallDefaults
      （:1534-1578）只按模块路径解析且只认 TInst：模块无同名主类型或主类型
      为接口时返回 null，省略实参不填充（生成侧 punctuation_model.dart:243
      两参构造、调用点零参实测）。判据为该类计数降为 0，其余类计数不上升。
      C2，P1，S1。
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
- [ ] F3aj dart FPHelper 32 位转换运行时未写出：覆盖第 8 节
      UNDEFINED_FUNCTION 的 floatToI32 7 条与 i32ToFloat 5 条（tdart2 r1）。
      修复位置为 dartcompiler/DartExpr.hx 把 haxe.io.FPHelper 成员路由到
      运行时限定名（e01b03b3 位于 :1602-1604）而 DartRuntime.hx 只带
      doubleToI64 与 i64ToDouble，32 位转换两个函数没有写出。判据为两条
      名字计数降为 0，其余类计数不上升。C1，P2，S2。
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
- [ ] F3aq ts runtime 模块导出缺失：覆盖第 7 节 TS2305 类内 `@tiqian/runtime`
      模块 18 条（UString 8、floatToI32 6、i32ToFloat 4；生成代码从
      `@tiqian/runtime` 导入这三个名字，生成树 runtime.ts 只导出 12 个
      名字、不含它们，第三次复测配类型后新暴露）。其中 UString 与 dart
      F3am 的运行时 UString 是同一个问题（dart 侧 F3al/F3am 记录了同名
      落空机制）；floatToI32 与 i32ToFloat 把 Float 与 Int32 按存储位互转
      （kotlin 生成树的
      runtime/FPHelper.kt 有同名成员，ts 侧 runtime.ts 只有 f64 对
      doubleToI64/i64ToDouble）。修复位置在 ts 编译器的常驻 runtime 模块
      编译与导出（tscompiler/Compiler.hx 的 TEST_MODULES 之外，runtime
      侧居民经 TsRuntime.hx 与 RuntimeResidents 走同一管道；对齐 kotlin
      侧把这三个名字编译进 runtime.ts 的机制我还没有验证）。判据为
      TS2305 类内 `@tiqian/runtime` 条目降为 0，其余类计数不上升。
      C2，P2，S2。
- [ ] F3as ts coalescing 静态字段对 abstract 类的完整路径泄漏：覆盖第 7 节
      TS2304 类内 org 108 条（org108 r1）。默认值展开保留的完整静态路径
      `org.tiqian.core.Ic.Zero` 在 TsExpr.hx:179 进入
      coalescingStaticFieldText（TsExpr.hx:244-258），该函数只处理 TInst，
      abstract `Ic` 返回 TAbstract 时在 :258 原样返回完整路径且不调用
      imports.value（正常静态引用路径在 TsExpr.hx:1478-1479 既输出短名又
      登记 import），生成文件没有 `org` 绑定。修复位置为
      coalescingStaticFieldText 补 TAbstract 分支（a.get().module 与
      name），按 staticRef 的方式输出短名并登记 import。判据为 TS2304
（修复历程：tsorg r1 交付 ts 侧修复 `b9390044`，其新增
      样本 ProbeUnit 在 rust 与 swift 的 stage1 引入新的失败
      （@:valueType abstract 静态字段默认值需要包装构造，超出 r1 的 ts
      单目标范围）；r2 交付五笔（`f43d6a02` 至 `bacb2ae0`）并合并
      `87255540`，四生成树 ProbeUnit 的 ZERO 行实测同形，19 项 gates
      全部退出码 0（consistency 转绿，非本修复引入的回归）。条目待新
      基线复测 TS2304 类内 org 条目后删除）- [ ] F3at dart charCodeAt 可空结果在消费边界未转为非空：覆盖第 8 节
      dunitsurf-r2 合并（`fc2b53b6`）后新增的 32 条（ARGUMENT_TYPE_
      NOT_ASSIGNABLE +17、UNCHECKED_USE_OF_NULLABLE_VALUE +14、
      RETURN_OF_INVALID_TYPE +1，全部生成侧）。DartExpr.hx 的 charCodeAt
      降级（d38459ab 位于 2332-2340）生成返回 `int?` 的 IIFE（越界返回
      null 的 Haxe 语义，正确），Haxe 源把结果静默用于非空 Int 语境
      （实参位、比较接收者位、返回位），dart 分析器拒绝。修复位置为
      dart 后端在静态期望类型可判定为非空 Int 的消费边界把调用包进仓库
      既有的可空值判空生成形状（先例核对：kotlin 侧同调用生成可空 run 块而
      kotlin 消费树无等价错误增长，KotlinExpr.hx:2963-2972；dart 后端
      既有判空生成形状以 grep 为准）。判据为三个错误码的 charCodeAt 新增
      条目降为 0、其余类计数不上升、越界返回 null 语义不回退（不得在
（修复历程：dartnull-r1 与 r2 死于渠道端点挂起零交付；
      r3（terra）交付 dart 侧修复 `819f6d3a`，消费树 A/B 验证与样本未复现
      的根本原因（r1 任务书规格缺陷：样本局部不标注）见第 8 节；r4（terra）
      补 kotlin、swift、dart 测试侧三笔 `c25cc3c1`、`17b3d3a8`、
      `9f0b52df`（rust 越界断言裁定为断言形态问题、非缺陷掩蔽）；r5
      （terra）交付 rust 双 unwrap_or 去重 `7994eceb`。任务分支六笔经
      合并 `b822afee` 进入 boring main 并已推送。条目待 census13 复测
      dart 三个错误码的 charCodeAt 条目与 rust E0599 同机制条目后删除）- [ ] F3au kotlin counted-loop 识别未查循环体写计数器：覆盖 3.2 节 cannot
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
- [ ] F4i rust std.Functional 未生成模块：覆盖第 6 节 E0432 类内
      crate::std::functional 1 条（rustsem r1）。std.Functional 是 extern
      不产生模块（Compiler.hx:84），rust 目标 shim 清单
      （Compiler.hx:319-325、RustImports.hx:9-21）无对应项，sumOfFloat/
      forEach 也未进惯用展开层（RustExpr.hx:3896-3913 与 :4332；
      PipelineExpander.hx:852-855 只有 sortedBy 就地展开）。修复位置为
      shim 清单补 std.Functional 或在惯用展开层与 sortedBy 同等处理。
      判据为该条降为 0，其余类计数不上升。C2，P3，S3。（排队）
- [ ] F4k rust haxe.Exception 引用未生成模块：覆盖第 6 节 E0433 类 1 条
      （rustsem r1）。RustImports.requireType 通用分支
      （RustImports.hx:73-77）把 haxe.Exception 按
      `crate::haxe::exception::Exception` 输出并登记 import，豁免表
      （:50-52）不含它，rust 目标从未生成 haxe 包模块。修复位置为 shim
      路线：SHIM_MODULES（RustImports.hx:9-21）与 Compiler.hx 的
      emitShim 调用段（:319-325）补 haxe.Exception 的 exception.rs 生成
      与 runtimeMods 登记。判据为该条降为 0，其余类计数不上升。C2，
      P3，S3。（修复任务 rustmisc-r2 已合并 `746742dd`；同分支撤回
      ProbeFaults 样本后 exception.rs 失去消费者、不再生成属预期，shim
      保留待 haxe.Exception 实际引用出现时验证）
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
- [ ] F4m rust @:dataClass 构造器关联函数内 self 绑定：覆盖第 6 节
      E0424 全类 75 条（rustprobe r1 3.1）。@:dataClass 构造器体里的
      `this.<field>` 被输出成关联函数 `fn new` 内的 `self.<field>`，
      而关联函数没有 `self` 绑定（源 InlineObjectBoundaryAdjustment.hx:18-22
      的构造器校验、生成 inline_object_boundary_adjustment.rs:21 实测；
      受害 Haxe 源共十二个，清单见 rustprobe r1 报告 3.1）。修复位置为
      rust 侧构造器体渲染把 `this.<field>` 改为直接字段名或构造器局部
      绑定。判据为该类计数降为 0，其余类计数不上升。C2，P1，S2。
      （排队）
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
- [ ] F4p rust 可空类 coalescing 默认值物化与 Option 形参不匹配：覆盖第 6 节
      E0308 类内同一消息形状的 198 条（rustjudge r2 第 3.1 节判定，错误来源
      判定为 `74371c5a`）。
      Haxe 源 `LayoutInput.hx:21` 的 `textStyle == null ? new TextStyle() :
      textStyle` 经 `RustExpr.hx:503` coalescingNormalizationLines 与 `:201`
      coalescingDefaultText 物化默认构造；该函数已有 asOption 形参（三元
      两臂在 :210-211 传 true），嵌套构造实参列表的生成路径
      （`RustExpr.hx:246`）不传它，实参不带包装直接写出（生成
      `layout_input.rs:43`），对 `text_style.rs:17` 的全 `Option<...>` 形参
      逐个不匹配。修复位置为默认值物化与被调函数形参的实际类型对齐
      （嵌套构造实参按形参类型包 `Some(...)`，数值字面量按形参的实际类型
      带后缀）。另有三个候选错误组
      （E0308-222 的 `Some(0)` 给 `Option<f64>`、E0689 的无后缀整数方法
      调用、E0599 的双重 unwrap_or），仅在 A/B 证实同一生成分支时随本项
      覆盖。判据为该 198 条降为 0、boring 侧 stage1:rust 与 rust-f32 保持
      绿、不撤销 `b74f9da9` 与 `d4a43a33` 的行为、无 panicking unwrap、
（修复任务 rustcoalesce：r1（terra，基线
      `87255540`）按任务书先补可空类全可选构造样本复现（`2584a348`），
      交付 rust 侧修复 `bc750c22`（stage1:rust 与 rust-f32 退出码 0）；
      同一样本在 swift、dart、kotlin 编译错误、ts 运行时错位，经派发方
      逐条核对 gates 日志定性为两个跨目标缺陷模式（swift/dart/ts 物化位
      丢第一个无默认值可选参数、kotlin 必填 `Null<T>` 构造参数渲染
      非空）。r2（terra）按停机条款结束：三笔提交不可保留，归档到
      rustcoalesce-r2-archive，任务分支复位 `bc750c22`；SOP 30 重审把
      r2 失败记为任务书缺陷（指定的修复位置不在渲染链上，三树输出与
      修复前逐字节相同），派发方两轮插桩探针（已还原）把缺陷产生点定
      到共享层 DefaultArgExpander.hx 的 completeNew 与 completeCall
      VCoalescing 分支只对 rustTarget 补 null 常量保位。r3（terra）交付
      `f7e6ba6c` 与 `f75015b4`，gates 7 项失败（含既有失败项 consistency），定性
      为 r3 规格错一半：常量默认值的省略物化正确，参数读取默认值的省略
      在 swift/dart/ts 把被读参数名直接写进调用点（名字不在调用点作用域），
      另有拼接位浮点缺 formatFloat 的独立缺陷（`defaultRecord` 断言
      `16.0` 对 `16`）。r4（terra）交付 `21234a8d`（参数读取默认值只在
      rust 物化，真值表见 r4 任务书 4.1 节）与 `2b139721`（kotlin、swift、
      dart 三渲染器拼接位浮点走 formatFloat），gates 19 项全部退出码 0
      （/tmp/gates-rustcoalesce4-rc.txt），合并 `c38a359c` 进入 boring
      main 并已推送，合并树统一重跑 19 项仍全部退出码 0
      （/tmp/gates-rustcoalesce4m-rc.txt）。消费树 4301/4311 回归的复测
      与本项判据核对归 census13）- [ ] F4q rust Clone derive 供给与 `.clone()` 调用需求不一致：覆盖第 6 节
      E0599 clone 子形 459 条（rustjudge r3 第 3.1 节判定，错误来源判定为
      `74371c5a`；机制表述经 r4 前置问题回答更正）。`RustExpr.hx:3873-3882`
      对非 Copy 值读取统一追加 `.clone()`，而 `LayoutResult`、`LayoutInput`、
      `LineSolution`、`LineCandidate`、`LineBox` 等结构没有拿到
      `#[derive(Clone)]`，rustc 报 no method named `clone`（位点如
      `layout_queries.rs:147:96`）。前置问题已由 rustjudge r4 只读回答、
      派发方在 a72f994d 检出逐行复核：`RustDecl.hx:216-223` 第一分支的
      `final hasCoalescingClone = true` 只把来源记录条件
      `state.recordCloneTypes.exists(...)` 换成常真，该分支同时要求
      `!cls.meta.has(":dataClass")`；上述结构全部是 `@:dataClass`（源
      LayoutResult.hx、LayoutInput.hx、LineBox.hx），走 `:223-225` 第二
      分支，该分支在无类参数时仍要求 `isAllClone(varFields)`
      （`:2274-2300` 逐字段 isCloneType 加 dataClassFieldsAllClone 递归），
      这些结构有字段不过该检查，所以 derive 没有写进声明（生成树
      layout_result.rs:15、layout_input.rs:30、line_box.rs:9 实测上方无
      derive 行）。r3 的「放宽 derive 触发条件」表述漏掉了 data-class 路径仍有的
      `isAllClone` 检查，已更正。
      修复位置候选两处二选一：derive 判定侧（data-class 路径对这些结构
      补 derive）与 `.clone()` 追加侧（判 Clone 能力再追加），随修复任务
      定案。判据为该 459 条降为 0、stage1:rust 与 rust-f32 保持绿、
      其余类计数不上升。C2，P1，S2。（排队，待 census13 重测后派发）

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
| 2026-09-06 | F0l dart 静态成员顶层重名修复（修复任务 dartic）与 dart 首次编译普查（F4b dart 半项）；dart 并入 KPI | boring `189e01ad`（合并 `31627b5c`，已推送） | dart 生成退出码 0；dart analyze 报 2931 条、35 类，求和校验相等；无测量环境条目 | 19 项验收检查在修复检出与合并检出各全部退出码 0；合并后 bun test 672 pass / 3 fail（三个既有名目） |
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
| 2026-09-07 | tattr3 r1 探针交付（kotlin 十二类升修复位置、假设检验、连锁错误归类）＋修复项 F3v 至 F3ag 开列 | 本文档第 3.2、3.4、10、11 节 | 修复位置 9→21 类、形状证实 14→0（12 类升修复位置、1 类判为连锁伪影、1 类升探针定位）；探针定位 3 类随 #23；pow 与 NodeFileSystem 因果结论在探针轮结束消息里（报告文件未及更新，锚点未存档） | 不适用（判定轮，无代码改动；探针检出 git status 干净核实，报告引用的生成器行号逐处复核） |
| 2026-09-07 | F3t swift 字符串插值内嵌语句体闭包（修复任务 swiftstr-r1） | boring `6ad8dc66`（合并 `d2c6b559`，已推送） | swift tests 侧每精度 57→41：F3t 四类（static methods 13、unterminated 1、extraneous 1、string interpolation 1）16→0；gen 侧每精度 1 条不变；基线参考目录对照无差异 | 修复任务报告内两精度 gen+test 退出码 0；合并检出两精度 gen+test 复跑退出码 0；消费方检出四目录计数（gen 1/283、tests 41/109 每精度）由派发方按磁盘日志复核，残余 41 条全部属于 cannot find 32 与 contextual type 9 两类 |
| 2026-09-07 | tdart2 r1 探针交付（dart 五类探针定位补齐因果链升修复位置、compareTo 形判定、三处跨目标同源证实）＋修复项 F3ah 至 F3ap 开列 | 本文档第 7、8、8.2、10、11 节 | 判定进度见第 8 节小结（修复位置 8 类）；错误计数未复测，基线不变 | 不适用（判定轮，无代码改动；报告引用与样本由派发方逐处复核，接收方检出两处未声明插桩已由派发方还原） |
| 2026-09-07 | rustfix-r1 合并（F3n、F3o、F3q；`e1f6d549` 至 `6852c398` 八笔，第 12 节上一行之前） | boring `b7054019`（已推送；rustfix-r1 分支） | rust 每精度 20→4：F3n 11、F3o 4、F3q 1 三类降为 0；F3p 2、F3r 1、F3s 1 仍在原位点，机制按复测现场更正（第 6 节），修复项保持开列 | 修复任务报告的 boring 侧验收命令通过；消费方检出复测由派发方补做（修复任务未完成该项）：vendored `6852c398` 与合并提交的 rust 目录内容相同（git diff 实测），cargo 每精度 4 条、日志末行 N=4 核对相等 |
| 2026-09-07 | kf3v-r1 合并（F3v 两笔 `1f2bd64b` 与 `50d90d45`；slice 调用点是接收方在基线分解中发现的同错误类第二个来源，超出任务书描述） | boring `b7500d45`（已推送；trial/kotlin-indexof 分支） | kotlin too many arguments 14/14→0、argument 类内连锁每精度 −2（UnicodeEmoji17RgiRoleAuditTest.kt:31 两处 slice 调用点，3.3 节其余转换 33→31）；总数 f32 696→680、f64 717→701；warnings 0/0 | 修复任务报告四判据通过（boring 仓库 gen 与 test 两精度四条命令退出码全部为 0、参考目录 diff 0 字节、消费方检出 vendored `50d90d45` 上复跑 680/701、TMA 0）；派发方在合并 `b7054019` 重生成后复跑同值：680/701、TMA 0、warnings 0，逐类枚举与形状分解求和校验相等，unresolved 符号分布与基线逐项相同 |
| 2026-09-07 | dartguard r1 至 r3 合并（F3l 全部、F3m 部分、浮点字面量修复）；F3l 完成并从第 11 节删除 | boring `b5bed1e3`、`1aaa9410`、`ed595487`、`f48e2715`、`a46962d5`、`f9b446df`（合并 `f42c38c9` 与 `18d5d8d1`，未推送）；r3 报告 /tmp/dispatch-state/boring-dartguard-r3.report.md | dart 分析错误 2606→1739（生成侧 1628→1298、测试侧 978→441）；UNDEFINED_IDENTIFIER 1025→280（类型名条目全部降为 0）、UNCHECKED_USE_OF_NULLABLE_VALUE 143→31、MISSING_IDENTIFIER 35→27、ARGUMENT_TYPE_NOT_ASSIGNABLE 290→288；逐类表求和校验相等 | r3 最终一轮 gates 的 `test:dart` 退出码 0、起点退出码 0 的检查全部仍为 0（派发方核对 /tmp/gates-dartguard3-rc.txt）；kotlin、rust、consistency 的失败与起点相同（修复不在该检出）；F3m 逐条三件套、全目标字节快照、bun test 对照与样本由 r4 补齐 |
| 2026-09-07 | dartguard r4 合并（F3m 续修；参数类型具化实验 `2fe7fac9` 使分析错误升到 1506 条量级后回退）；F3m 未完成，残余 17 条保留第 11 节 | boring `45eb6519`、`fe96e023`（合并 `e4952dab`，未推送）；报告 /tmp/dispatch-state/boring-dartguard-r4.report.md | dart 分析错误 1739→1707（生成侧 1298→1266、测试侧 441 不变）；UNCHECKED_USE_OF_NULLABLE_VALUE 31→17、ARGUMENT_TYPE_NOT_ASSIGNABLE 288→271、RETURN_OF_INVALID_TYPE 2→1；逐类表求和校验相等 | 派发方在合并态 `e4952dab` 复测（日志 /tmp/dg4-final-gen.log 与 dg4-final-tests.log）；`test:dart` 退出码 0（/tmp/dg4-merged-testdart.log，2 条 warning 均为 dead_code）；UNDEFINED_IDENTIFIER 的 Ic 条目为 0，日志内 2 条 Ic 是 UNDEFINED_METHOD（units.dart:5、:11）且基线已有；全目标字节快照、bun test 对照与样本注册由 r5 补齐 |
| 2026-09-07 | dartstd-r2 合并（F3i 主体）；r2 未交报告且一度留下未提交改动，处置由派发方完成 | boring `4a67d7e0` 与 `0162fa05`（合并 `c629b1d1`，已推送） | dart 分析错误 1707→1686（生成侧 1266→1253、测试侧 441→433）；URI_DOES_NOT_EXIST 36→2（生成侧 25 条全部消除）；同笔合并新暴露 13 条：UNDEFINED_PREFIXED_NAME 42→44、UNDEFINED_FUNCTION 55→58、UNDEFINED_METHOD 69→73、ARGUMENT_TYPE_NOT_ASSIGNABLE 271→273、RETURN_OF_INVALID_TYPE 1→2、UNDEFINED_CLASS 0→1（新增错误码）；逐类表求和校验相等 | 派发方在合并态复测（日志 /tmp/dartstd3-gen.log 与 dartstd3-tests.log；第二笔 `0162fa05` 只改 ts 编译器注释，dart 产物与 `4a67d7e0` 树一致）；`test:dart` 退出码 1 与基线相同（唯一 fail 为 StringUnitTests.unitSurface，`d0df20db` 基线已有）；r2 两条中文注释已改写为英文；boring 全树 doc-style 命中 0 |
| 2026-09-08 | dartguard-r6 合并（F3m 可空读取断言的续作轮：恢复 r5 未提交回滚掉的重复读取断言）；r6 交部分交付报告，实验 `cd34ca32` 扩大断言面使消费方回归 1755、由 `6ad52331` 整笔回退，样本登记与全目标 SHA 快照未完成 | boring `02210451`、`cd34ca32`、`6ad52331`（合并 `b4b574b5`，已推送） | dart 分析错误 1686→1682（生成侧 1253→1250、测试侧 433→432）；逐码对照唯一变动 UNCHECKED_USE_OF_NULLABLE_VALUE 17→13（生成侧 14→11、测试侧 3→2），其余 33 个错误码计数不变、无新增错误码；逐类表求和校验相等 | 派发方在合并态复测（日志 /tmp/census8/dart-gen.log 与 dart-tests.log，逐码表 gen-codes.txt 与 tests-codes.txt）；F3m 残余 13 条逐文件分布见第 8 节逐类表；r6 未完成项（样本登记、全目标 SHA 快照、19 条测试命令的基线对照、F3m 残余未降到 0）转入后续轮任务书 |
| 2026-09-08 | fph32-r1 合并（F3aj）；zen 免费档模型 mimo-v2.5-free 执行 | boring `2382140b`（合并 `014d9640`，已推送） | dart 分析错误 1682→1670（生成侧 1250→1238、测试侧 432 不变）；逐码对照唯一变动 UNDEFINED_FUNCTION 58→46（floatToI32 7 加 i32ToFloat 5 降为 0），其余错误码计数不变；逐类表求和校验相等 | 派发方在合并态复测（日志 /tmp/census9/dart-gen.log 与 dart-tests.log）；A/B 三判据反转、gen:dart 退出码 0、test:dart 与基线相等（唯一 fail 为 StringUnitTests.unitSurface）、bun test tests/ 120 通过优于基线 112，报告 /tmp/dispatch-state/boring-fph32-r1.report.md |
| 2026-09-08 | tsforce-r1 合并（F3ar） | boring `7c2a1c02`（合并 `908305a0`，已推送） | ts 分析错误 722→714；逐名对照唯一变化 TS2304 类内 TestCore 8→0（生成树 runtime/test.ts 现含 export class TestCore 声明），其余名称与错误码计数不变；逐类表求和校验相等 | 派发方在合并态复测（日志 /tmp/census9/ts-typed.log）；stage1:ts 367 通过、生成树输出与修复前一致（仅 Compiler.hx 一个文件改动），报告 /tmp/dispatch-state/boring-tsforce-r1.report.md |
| 2026-09-07 | swift 产物文件计数核对：`_GeneratedFiles.txt` 末行无换行符，`wc -l` 比 `grep -c .` 少 1；rm-first 重生成后 f64 目录 393 个文件与普查记录相同，std/UString 两个文件在 tiqian `77e9bd3c` 与 `3f609c0f` 两个基线都不生成（非回归） | 本文档第 2.3 节 | swift f64 rm-first 复测 393（find -type f 计数，含清单文件本身）；f32 未做 rm-first 复测 | 不适用（测量核对，无代码改动） |
| 2026-09-07 | tsgetcal-r6 合并（F3e）；F3e 完成并从第 11 节删除 | boring `4fa632e6`（未推送） | ts 1127→1004、19→16 类：TS2551 28→0 与 TS2341 get_ 前缀 10→0；TS2420 6→0、TS2693 3→0；TS2345 87→14、TS2322 8→2；TS2304 183→186（org 2→108 为新形态即 F3as）；测量环境 308 不变 | 修复任务检出 19 项 ALL-DONE；派发方在合并态重跑（tag mainmg1，`d0df20db`）15 项通过、4 项失败，失败四项非本批引入：kotlin 两项为 main 既有 UString 生成缺陷（修复归 knamefix 续作）、test:dart 为 StringUnitTests.unitSurface 单测失败（基线已有）、consistency 的进入点经逐合并树核对更正为 `dd703b1f` 进、`c332897a` 出 |
| 2026-09-07 | rustfix r2 至 r4 合并（F3p、F3r、F3s；r4 另使 boring 侧 stage1:rust 残余诊断降为 0；三笔修复各自对应哪次合并没有逐笔核对）；F3p、F3r、F3s 完成并从第 11 节删除 | boring `e509f6df`（r2，经 `09f1b8ad`）与 `d0df20db`（r4，经 `3dc775a3`；r3 无独立合并提交，其工作经 rustfix 分支并入，未逐笔核对）；报告 /tmp/dispatch-state/boring-rustfix-r4.report.md | rust 解析层每精度 4→0；语义层首次测得每精度 189、8 类（第 6 节新表：108 条的错误类有最小复现实证，42 条的 compare 前缀错误类为与 ts/dart 同源的假设）；消费方 189/189 与 rustfix 系列修复自 r2 起的消费方基线一致 | 派发方在合并态重跑 19 项检查（tag mainmg1，`d0df20db`）15 项通过、4 项失败：test:stage1:rust 与 test:rust-f32 均 0；失败四项为 kotlin 两项检查、test:dart、consistency 的既有缺陷（见上一行），与 rust 批次无关；未推送 |
| 2026-09-07 | ts 测量命令补类型配置并同树复测（用户裁定测量环境条目应随测量配方配置）：第 2.2 节新增类型支撑目录安装命令与测量树 out 目录 tsconfig.json（bun 与 node 的类型定义、`@tiqian/runtime` 两个模块名映射到生成树内文件），编译选项其余各项不变；tsprobe2 r1 对 TestCore 的测量环境判定经机制核对改判引擎侧 | 本文档第 1、2.2、7、7.2、10 节；无代码改动 | ts 1004→722、消息骨架 16 个不变：测量环境 300 条全部解析（TS2307 类内 bun:test 108、@tiqian/runtime 79、@tiqian/runtime/test 108、node:fs 1、node:path 1；TS2580 的 process 3 整类删除）；新暴露引擎侧 19 条（TS2305 的 `@tiqian/runtime` UString 8、floatToI32 6、i32ToFloat 4，runtime.ts 只导出 12 个名字不含这三个；TS2304 的 compareFontMetricsRequest 1 条换骨架为 TS2552）；TestCore 8 条改判引擎侧（ts Compiler.hx 缺 runtime.TestCore 强制类型化，kotlin 侧 kotlincompiler/Compiler.hx:58 有该调用、kotlin 生成树有 runtime/test/TestCore.kt，ts 生成树全树无声明）；逐类表求和校验相等 | 同一棵 `d0df20db` 生成树前后两次测量（日志 /tmp/census7/ts.log 与 ts-typed.log），差值全部来自类型配置；判据=bun:test、@tiqian、node:fs、node:path、process 条目在 ts-typed.log 中不再出现（TS2307 模块分布仅剩 4 个相对路径模块名） |
| 2026-09-08 | rustsem r1 判定并入（rust 语义层八类逐类三件套：第 6 节逐类表更新为修复位置判定，原 4 条的 could-not-find 类拆为 tiqian_no_such_element_exception 3 条与 crate::std::functional 1 条两行，修复项 F4d 至 F4k 开列；探针 cmd deepseek-v4-flash 死于周限额但报告完整交付）＋org108 r1 判定的 K2 进度更正（ts 侧 org 108 从待定位改为已判定，修复项 F3as 开列）＋tsforce-r1 的 F3ar 按删除制从第 11 节移除 | 本文档第 6、10、11 节 | 判定进度：rust 语义层 8/8 类全部有修复位置（探针为静态对照判定，未运行构建；108 条类的机制另有 /tmp/cfgtest-repro 最小复现）；错误计数未复测，基线不变（rust 189/精度、ts 714） | 不适用（判定轮，无代码改动） |
| 2026-09-08 | dunitsurf-r2 合并（dart 运行时崩溃消除；test:dart 链首次走到 analyze 步，暴露 boring 自身 48 条名字解析错误，dunitsurf-r3 派发）＋rustf4d-r1 合并（F4d 完成删除）＋F3at 开列（dartnull 派发）；`d38459ab` 复测 | boring `c5cf26d8`（合并 `fc2b53b6`）与 `0629b847`（合并 `d38459ab`），均已推送；日志 /tmp/census10 | rust 每精度 189→255：E0432 pub use 类 108→0，新增四码 174 条全部未判定（进入区间拓扑缩小为 `d0df20db..d38459ab`，rustprobe r1 派发）；dart 1670→1702（生成侧 +32 全部 F3at，测试侧不变） | 派发方中央复测 census10（逐类求和校验相等）；knamefix-r7 经 gates 否决（CI run 34189663133 判为本支引入），r8 返工派发；dartargs-r1 部分交付（dart 侧正确、rust 五错与 kotlin 样本拒绝待修），r2 返工派发 |
| 2026-09-08 | rustprobe r1 判定并入（新增四错误码原因判定：A/B 实测 `908305a0` 精确复现基线 189 且其非测试 .rs 文件与 `d38459ab` 逐字节相同、`c332897a` 生成整体崩溃剩 4 条属 knamefix-r2 自身回归、控制实验在 `d38459ab` 树手工删除全部 216 条 `#[cfg(test)]` 条件编译标注后计数精确回到 189；结论是新增四码为 rustf4d 条件编译揭露的既有缺陷而非新引入翻译缺陷）＋修复项 F4l 至 F4o 开列（E0425 的 org 加 region 加 count 60 条 F4l、E0424 75 条 F4m、E0423 1 条 F4n、E0433 13 条 F4o；E0425 compare 六支按位点改判随 F4e） | 本文档第 6、11 节；无代码改动 | 错误计数未复测（`d38459ab` 基线 255/精度不变），判定结论进入第 6 节原因判定段与逐类表 | 不适用（判定轮，纯只读探针；报告 /tmp/dispatch-state/boring-rustprobe.report.md，A/B 与控制实验证据 /tmp/rustprobe-*.log） |
| 2026-09-08 | dunitsurf-r3 合并（boring 自身 test:dart 链 analyze 步 48 条名字解析错误降为 0，`db0f713a`） | boring `db0f713a`（合并 `5f48b520`，已推送） | boring 自身 test:dart 名字解析 0（修复任务报告内命令）；消费树效果由 census11 统一复测：dart URI_DOES_NOT_EXIST 生成侧 0→3、测试侧 2→3（四处 runtime/sorted_table.dart import 断链），`_codePointAt` @ `SortedTable` 4 条与 SortedMapTable 两条因 import 断链、类型检查未到达错误位而不再报（掩蔽，非修复，见节首 URI 回归段） | 修复任务报告内验收命令通过；消费方复测由 census11 中央执行，未单独出对照表 |
| 2026-09-08 | knamefix-r8 合并（kotlin unresolved 365/358→54/54、cannot infer 42/42→6/6 等；F3ag 完成删除）；同合并引入新类 `'val' cannot be reassigned` 24/24（3.2 表行内判定）；r7 经 gates 否决后 r8 返工合并 | boring（合并 `3044bf91`，已推送） | kotlin f32 680→343 / f64 701→354（census11，类集 33/33 首次相同）；ts 714→706、rust 255→247、dart 1702→1695 为同轮复测值 | 派发方中央复测 census11（五目标全量）；19 项 gates 未统一重跑（K6） |
| 2026-09-08 | census11 五目标中央复测（`3044bf91` 两笔合并后，tiqian 主树 `4bfe817f` 重生成）：文档与 K2/K3/K6 全面更新；swift tests 侧计数首次记录阻断（`import TiqianEngine` 使逐目录 swiftc 在模块加载处中止，第 5 节） | 本文档全文；无代码改动 | 八生成命令退出码 0；kotlin 343/354（33 类）；swift gen 每精度 1、tests 阻断；rust 247（8 码）；ts 706（16 类）；dart 1695；各逐类表求和校验相等 | 不适用（测量轮；日志与逐类表 /tmp/census11） |
| 2026-09-08 | rustmisc-r2 合并（F4j 两笔；同分支撤回 ProbeFaults 样本，F4k shim 的 exception.rs 因失去消费者不再生成属预期）；F4j 完成删除 | boring `d4a43a33`、`b74f9da9`（合并 `746742dd`，已推送） | rust E0072 每精度 1→0（census12）；同轮出现合并引入的消费侧回归（每精度 247→4548/4558，第 6 节） | 任务报告：E0072 修复生效、cargo check 两精度无 E0072/E0433；合并后统一 gates 17 项通过、3 项失败（处置见 K6 与第 6 节） |
| 2026-09-08 | dartargs-r2 合并（F3ah dart 侧修复生效，`74371c5a` 与 `963a3ba6` 两笔）；rust 消费侧回归与 stage1:rust、rust-f32 交互失败的处置同笔记录 | boring `74371c5a`、`963a3ba6`（合并 `a72f994d`，已推送） | dart NOT_ENOUGH_POSITIONAL_ARGUMENTS 105→4（census12），其余 32 码不变；rust 每精度 247→4548/4558（第 6 节）；交互失败由派发方修复 RustExpr.hx constructorBody 同名形参不记录规则并入本合并提交，两套件单独复测通过 | /tmp/gates-postmerge12-rc.txt 为修复前统一跑记录（17 项通过、3 项失败）；修复后的提交态未再统一重跑（后续 `b822afee`、`c38a359c` 两次全量重跑覆盖，见 K6） |
| 2026-09-08 | census12 五目标中央复测（`a72f994d` 两笔合并后）：文档头、第 3.1、6、8、11 节与 K2/K3/K6 更新；rust 逐类表新增 20 码行；dart F3ah 行更新为残余 4 条 | 本文档全文；无代码改动 | 八生成命令退出码 0；kotlin 343/354 持平；rust 4548/4558（28 码回归）；dart 1594；ts 706 与 swift 沿用 census11；rust 与 dart 逐类表求和校验相等 | 不适用（测量轮；日志 /tmp/census12） |
| 2026-09-08 | tsorg-r2 合并（F3as r2 五笔，提交号范围见合入位置栏；分支含 r1 的 ts 侧修复 `b9390044`）；cmd 免费档 meituan/longcat-2.0 执行 | boring `f43d6a02` 至 `bacb2ae0`（合并 `87255540`，已推送） | boring 侧 19 项 gates 全部退出码 0（含此前为既有失败的 consistency 转绿）；消费树计数未复测，F3as 待新基线复测 TS2304 org 条目后删除 | 派发方验收：报告以磁盘产物核对（四份生成树 ProbeUnit ZERO 行同形、/tmp/gates-tsorg2-rc.txt 19 行 rc=0、修复检出 git status 输出为空、合并提交与修复分支尖端的 tree 哈希相同）；报告 /tmp/dispatch-state/boring-tsorg-r2.report.md |
| 2026-09-08 | rustjudge r1 部分验收：测量层合格并入第 6 节（27 码复算 Σ 4558/4548、E0308 形状表 Σ=2083、逐码按文件分布、E0277 按文件分布）；判定层不合格（20 码因果路径为模板填充、错误来源判定无 git 证据、档位一律形状证实、候选未处置、分组无条数），按 SOP 30 记为任务书缺陷；E0277 全类 F4g 同一机制的说法经形状分解证伪；r2 返工派发（terra，任务书 /tmp/dispatch-state/boring-rustjudge-r2.prompt.md） | 无代码改动（判定探针）；r1 测量层并入第 6 节，判定层作废 | 错误计数未复测（census12 基线不变） | r2 正在执行 |
| 2026-09-08 | rustjudge r2 验收（诚实部分交付：E0308-198 形状组完整三件套判为修复位置、错误来源判定 `74371c5a`（VNull 双包装与递归装箱两候选证伪）、E0277 全量 18 形状表 Σ=834 并证伪 F4g 同机制说法（send 仅 5 条）；19 码三件套与 E0689 四文件对照未完成、报告内声明）；F4p 开列；三项任务派发：rustcoalesce-r1（terra，F4p 修复，先补样本复现）、rustjudge-r3（terra，2230 条九档优先级续判，E0308 与 E0277-send 排除）、dartnull-r2（zen，F3at） | 无代码改动（验收与派发轮）；r2 结论并入第 6 节，F4p 进第 11 节 | 错误计数未复测（census12 基线不变；三任务交付后统一 census13） | 派发前按 env 文件 pid 验活（terra×2 加 zen×1，渠道容量内） |
| 2026-09-08 | rustjudge r3 验收（E0599 四子形 569/701 条三件套：clone 459 修复位置开列 F4q、关联常量 51、`as_deref` 36、Option `to_string` 23 三组 110 条既有暴露；E0599 逐轮计数核对实测 census10 与 census11 均为 0、census12 为 701，显现全部为新增）＋rustcoalesce-r1 部分验收（rust 侧 `bc750c22` 两项退出码 0；六项失败经派发方逐条核对 gates 日志定性为两个跨目标缺陷模式，r1 的「gate 顺序/环境状态」定性被日志证伪）＋三项再派发：rustcoalesce-r2、rustjudge-r4（398 条续判，E0061、E0277、E0689 排除待 census13）、dartnull-r3（r1 与 r2 死于渠道端点挂起，SOP 33 重发说明）＋F4q 开列 | 无代码改动（验收与派发轮）；r3 结论并入第 6 节与 K2，F4q 进第 11 节 | 错误计数未复测（census12 基线不变） | 启动后 env 核对（terra 3/3 满、zen 0/1、cmd 0/2），pid 验活并挂盯守 |
| 2026-09-08 | rustjudge r4 验收（诚实部分交付 26/398：u32 双重 `unwrap_or` 12 条（`RustExpr.hx:613-629` 局部声明分支与 `:3343-3353` 算术分支对同一 charCodeAt 调用各追加一次，bopomofo_parser.rs:22 双后缀实测）、关联常量 `AUTO_SPACE_POLICY_DEFAULT` 14 条（与 r3 的 51 条组同一机制、扩为 65）；两组均既有暴露，区间三笔 rust 提交 hunk 不触相关分支；372 条未完成在报告内声明，进程干净结束）；F4q 前置问题回答并经派发方逐行复核（data-class 路径仍有 isAllClone 检查，r3「放宽 derive 触发条件」表述更正）；第 6 节小结求和笔误更正（十三小码 167→105） | 无代码改动（判定探针验收轮）；r4 结论并入第 6 节、F4q 与 K2 | 错误计数未复测（census12 基线不变） | 报告 /tmp/dispatch-state/boring-rustjudge-r4.report.md |
| 2026-09-08 | dartnull-r3 验收（部分接受：dart 侧修复 `819f6d3a` 与样本 `fd23a839` 保留在待合并分支；样本未复现三码判定为 r1 任务书规格缺陷：样本局部不标注、消费树触发错误的是标注 `Int` 的局部；越界断言 `Test.equals(null, argShape(""))` 裁定为断言形态问题、非缺陷掩蔽；gates 六项失败逐条定性：kotlin 比较右算员与推断局部返回位、swift 调用返回位漏补非空断言、rust 越界断言语义、dart 测试侧 equalsInt(null) 类型）＋dartnull-r4 派发（terra，续修四处） | 无代码改动（验收与派发轮）；r3 结论并入第 8 节与 F3at | 消费树 A/B 实测三码 170→153、26→12、2→1、总量 −32 与新增数一致、其他错误码零扰动（/tmp/dartnull-ab-*.ml）；boring main 基线不变 | 报告 /tmp/dispatch-state/boring-dartnull-r3.report.md |
| 2026-09-08 | rustcoalesce-r2 验收（全部不可保留：三笔归档 rustcoalesce-r2-archive，任务分支复位 `bc750c22`）；SOP 30 重审记为任务书缺陷（指定锚点不在渲染链上，三树输出与修复前逐字节相同）；派发方两轮插桩探针（已还原）定缺陷产生点为共享层 DefaultArgExpander.hx completeNew 与 completeCall 的 VCoalescing 分支只对 rustTarget 补位；kotlin 侧 r2 改动引入七条新错；rustcoalesce-r3 派发（terra） | 无代码改动（验收与派发轮）；F4p 条目更新 | 错误计数未复测（census12 基线不变；复位后基线复测仅 r1 已知单错 /tmp/rco2-kotlin-baseline.log） | 探针还原后检出 git status 为空核实 |
| 2026-09-08 | dartnull-r4 验收（三笔保留并证实：`c25cc3c1` 标注形样本与越界断言改 `Null<Int>` 绑定判等〔预裁定准许的断言形态修正〕、`17b3d3a8` kotlin 声明位补 `!!` 与算员位返回位补非空断言、`9f0b52df` swift 返回非可选时追加 `!`；rust 去重尝试产出零解包形状 E0308 后按停机条款诚实结束、未保留）＋E0599 双 `unwrap_or` 机制更正（两次追加都在局部声明渲染路径，rustjudge r4 记的算术分支不在缺陷链上，第 6 节已更正）＋dartnull-r5 派发（terra，只修 rust 双 unwrap 去重） | 无代码改动（验收与派发轮）；r4 结论并入第 6 节与 F3at；任务分支合并押后至 r5 的 gates 19 项全部退出码 0 | 错误计数未复测（census12 基线不变）；r4 报告所称 registry 等既有编译错误经 HEAD 全量重生成复测定性为陈旧树假象，真基线 3 条 E0599（/tmp/dn4-fresh-stage1rust.log） | 真基线判定方法为全量重生成后 stage1:rust 复测 |
| 2026-09-08 | dartnull-r5 验收（rust 双 `unwrap_or` 去重 `7994eceb`：生成树标注形与不标注形逐字节同形）＋dartnull 任务分支六笔（`fd23a839` 至 `7994eceb`）合并 boring main 并推送 | boring `7994eceb`（合并 `b822afee`，已推送） | gates 19 项全部退出码 0（/tmp/gates-dartnull5-rc.txt）；消费树计数未复测（census12 基线不变，待 census13） | 报告 /tmp/dispatch-state/boring-dartnull-r5.report.md |
| 2026-09-08 | rustcoalesce-r3 验收（部分接受：两笔 `f7e6ba6c` 与 `f75015b4` 保留；gates 7 项失败定性为 r3 规格错一半：常量默认值的省略物化正确，参数读取默认值的省略在 swift/dart/ts 把被读参数名直接写进调用点、另有拼接位浮点缺 formatFloat 的独立缺陷）＋rustcoalesce-r4 派发（terra，两个工作项：参数读取默认值只在 rust 物化、三目标拼接位浮点走 formatFloat，任务书 /tmp/dispatch-state/boring-rustcoalesce-r4.prompt.md）＋F4m 派发（terra，修复任务 boring-f4m-r1：rust E0424 全类 75 条，@:dataClass 构造器体 this.<field> 读取改绑构造器局部，样本先行，任务书 /tmp/dispatch-state/boring-f4m-r1.prompt.md） | 无代码改动（验收与派发轮）；F4p 条目更新 | 错误计数未复测（census12 基线不变；r4 合并后统一 census13） | 启动后 env 核对（terra 2/3 在用：rustcoalesce-r4 加 boring-f4m-r1），pid 验活 |
| 2026-09-08 | tiqian-tattr4-r1 探针验收（kotlin 11 类：cannot be reassigned 升修复位置并开列 F3au（PolicyQueries.hx 的 intervalCore 与 intervalShort 未查循环体写计数器）；假设 5 类与未判定 5 类全部升形状证实，其中 collection literals、array literals、selector 三码 12 条为同一生成表达式 `?.[i]`（可空接收者数组下标）的三个解析视角，Unexpected tokens 与 overload ambiguity 同位 sumOf 渲染，null cannot be 为 lastIndexOf 降级插入 null 参） | 无代码改动（判定探针）；判定并入 3.2 表与小结，F3au 进第 11 节 | f32 343 / f64 354、warnings 0/0、逐类求和校验相等（基线锚定 `a72f994d`，与 census12 相等） | 报告 /tmp/dispatch-state/tiqian-tattr4-r1.report.md；vendored 推进记录与检出干净核对 |
| 2026-09-08 | boring-f4m-r1 验收（两笔 `1d7f72e9` 样本与 `52c574ff` 修复证实：样本在修复前复现 E0424 两条，修复后构造器读局部绑定 `magnitude`；gates 19 项全部退出码 0）＋合并 boring main 并推送；同日五条修复任务派发占满全部渠道（f3ae luna、f3ab terra、f3ao zen、f3z 与 f3aa cmd；启动途中 glm 周限额与 longcat、deepseek 未拉起均按白名单换位处置） | boring `1d7f72e9`、`52c574ff`（合并 `4644e29b`，已推送） | 合并树统一重跑 gates 19 项全部退出码 0（/tmp/gates-f4mm-rc.txt）；E0424 全类 75 条的复测归 census13；/tmp/boring-f4m 检出已删除并 worktree prune | 报告 /tmp/dispatch-state/boring-f4m-r1.report.md；`git status --porcelain` 为空 |
| 2026-09-08 | rustcoalesce-r4 验收（两笔 `21234a8d` 与 `2b139721` 证实：swift `dependenceEarlier("alpha")` 省略、dart RubySpan 6 参调用、kotlin defaultRecord 输出 `zh-Hans:16:400:false:0`；gates 19 项全部退出码 0）＋合并 boring main 并推送 | boring `21234a8d`、`2b139721`（合并 `c38a359c`，已推送） | 合并解八处 examples/*.hxml 样本登记冲突（两组登记行都保留）；合并树统一重跑 gates 19 项全部退出码 0（/tmp/gates-rustcoalesce4m-rc.txt）；消费树 4301/4311 回归复测归 census13；/tmp/boring-rustcoalesce 检出已删除并 worktree prune | 报告 /tmp/dispatch-state/boring-rustcoalesce-r4.report.md；`git status --porcelain` 为空 |
