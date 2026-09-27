# engine-haxe

`engine-haxe` 是排版引擎的 Haxe 源码树。`engine`（Kotlin）是当前的产品代码；
本目录与它并行存在：先把 Kotlin 逐包翻译成 Haxe，再用生成的 Kotlin 逐个
文件替换 `engine` 中的手写文件。全部替换完成后，删除 `engine`，本目录改名
为 `engine`，Haxe 成为唯一源码，Kotlin、Swift、Dart、JS、Rust 五种目标
都由 boring（Haxe 到五种语言的编译器，独立仓库）从本目录编译生成。

## 布局

- `src/`：Haxe 源码，含与引擎测试一一对应的测试类。
- `tests/`：测试入口 `Main.hx` 与编译清单 `compile.hxml`。
- `tools/compare-traces.py`：把 Haxe 测试记录的执行轨迹与引擎 golden
  逐行比对。golden 指引擎测试留下的基准轨迹文件，位于
  `engine/src/jvmTest/resources/golden/test-traces/`（本地生成，不入库）。
- `targets/`：各 HXML 提供目标编译参数。进入 flake 开发环境时，driver 按
  `boring.json` 的 `engine` source set 生成忽略入库的 `classes.hxml`，供直接
  调用 HXML 的脚本使用；Haxe 不会仅凭 `-cp` 编译目录下全部类。
  `common.hxml` 提供引擎配置共用的
  类路径与宏，`kotlin-common.hxml` 等文件补充各目标的编译器与运行时
  参数，精度入口沿用这些设置。`protocol-*.hxml` 单列协议根类；
  `rust-f32.hxml` 和 `rust-f64.hxml` 是 `boring.json` 正在使用的配置。
- `boring.json`：生成与测试配置驱动器（boring feature spec 59）的项目文件，声明本目录
  的目标、精度及命名源码范围。`engine` 范围从六个包发现 `*Test.hx`，加上两个
  runtime 根类型；八个引擎配置共用它。生成与测试以此文件为准。
- `tools/setup-haxe-env.sh`：把两组不入库的数据同步进当前检出，见下节。
- `textrange-kotlin.hxml`、`smoke-kotlin.hxml`：独立用途的 Kotlin
  生成清单。
- `data/`：生成 Unicode 数据类所需的区间数据。
- `patches/`：vendored boring（`.haxelib/`，不入库）之上的本地补丁存档。
- `out/`、`baseline-goldens/`、`smoke/`：生成物与本地基线拷贝，不入库。

## 首次准备

生成阶段要读两组不入库的数据，新建的检出或 git worktree 里可能没有，
缺任何一个都会在生成时报出与准备无关的错误：

| 输入 | 谁读它 | 缺了会怎样 |
|---|---|---|
| `engine-haxe/baseline-goldens/` | `GoldenDataMacros.init()` | `Golden data directory not found: engine-haxe/baseline-goldens/layout-dumps` |
| `tools/unicode-data/` | 各 Unicode 数据类在编译期读取 | `pinned Unicode data file is missing: tools/unicode-data/GraphemeBreakProperty-17.0.0.txt` |

在检出根目录执行一次即可补齐（已存在的一律不动）。脚本默认选择主 worktree
作为已准备输入的源检出；目录布局不同时显式传入源检出。复制前会逐项检查输入：

```shell
bash tools/setup-haxe-env.sh
bash tools/setup-haxe-env.sh /path/to/prepared/tiqian
```

flake 把 boring 的 driver 与编译器固定在同一修订，并在进入开发 shell 时配置
`.haxelib/`。已有的本地 boring 检出会先备份，再由固定修订的可写副本接替，
以兼容现有 HXML 的相对路径及编译期诊断输出。`baseline-goldens/` 来自原 Kotlin 引擎记录的
本地基准输出；`tools/unicode-data/` 包含 Unicode 17.0.0 的数据文件。两组
输入目前都不入库，因此新 worktree 仍需要从已准备的检出获取它们。

## 生成与测试

tiqian 的 flake 从固定的 boring 修订安装 `boring` 命令，并配置相同修订的
Haxe 编译器与 reflaxe。进入开发 shell 后直接调用命令即可。

之后在 tiqian 检出根目录执行。驱动器按 `boring.json` 派生每个目标配置的输出目录
（`engine-haxe/out/<id>/gen` 与 `engine-haxe/out/<id>/gen-tests`）与结果文件
（`engine-haxe/out/test-results/<id>.jsonl`）：

```shell
cd /path/to/tiqian
nix develop -c boring gen kotlin-f32 --project boring.json
nix develop -c boring test kotlin-f32 --project boring.json
```

进入开发 shell 会更新 `engine-haxe/targets/classes.hxml`。如果在同一个 shell 内
新增测试类，重新生成入口文件，再直接调用 HXML：

```shell
boring roots engine --project boring.json --output engine-haxe/targets/classes.hxml
```

已从 tiqian 根目录验证 `nix develop -c boring gen protocol-c`
能完成 Haxe 生成和 `afterGen`，写出 `tiqian_protocol_constants.h`。上述
`kotlin-f32` 的 `gen` 也已通过；该配置的 `test` 尚未在当前检出运行。

六个动作：`roots` 从命名源码范围生成供直接 HXML 使用的入口文件；`gen` 生成该目标配置的两棵源码树；`test` 把已生成的树编译并运行，
留下结果文件（先对同一配置运行 `gen`）；`pack` 为声明了 `package` 的配置打发布包；
`compare` 按 `boring.json` 的 `baseline` 逐用例比对参与比较的配置的结果文件；
`verify` 依次对所有配置运行 `gen`，对可测试配置运行 `test`，然后运行 `compare`。
`verify --with-pack` 只打包声明了 `package` 的配置。显式对无测试配置运行 `test`
会报错；显式对没有 `package` 的配置运行 `pack` 也会报错。
`boring gen protocol-c --project boring.json` 先编译 Haxe 得到 JavaScript，
再按该配置的 `afterGen` 运行 JavaScript，写出 C 头文件。
该配置在 `boring.json` 中声明 `"test": false`，因此不参与 `compare`。完整
`verify` 是否通过，需要对当前检出实际运行后确认。
目标与精度配置写在 `boring.json` 里，当前十二个配置是 `kotlin-f32`、`kotlin-f64`、
`ts`、`swift-f32`、`swift-f64`、`dart`、`protocol-ts`、`protocol-rust`、
`protocol-kotlin`、`protocol-c`、`engine-rust-f32`、`engine-rust-f64`。

## Haxe-JS f32 参照输出

f32 层的比对参照由 Haxe 自己运行后生成的 JS 输出提供；`tests/compile.hxml` 将
`float-precision` 设为 `f32`，该入口不经过任何目标后端：

```shell
nix develop -c bash -c 'haxe engine-haxe/tests/compile.hxml'
bun engine-haxe/out/haxe-tests.js
python3 engine-haxe/tools/compare-traces.py \
  engine-haxe/baseline-goldens/test-traces engine-haxe/out/haxe-traces \
  --mode tolerance --classes <已移植的测试类，逗号分隔>
```

比对脚本在 `raises exception=` 行上把 Kotlin 标准库异常名与 Tiqian 前缀名
视为同名（`EXCEPTION_NAME_ALIASES`），每次运行输出放过的行数；引擎 golden
全部改为 Tiqian 前缀名后删除该规则。

## 同步纪律

`engine` 中已翻译区域的任何 Kotlin 改动，必须同步修改本目录的 Haxe 副本，
并重新运行上面的比对；引擎行为有意改动、golden 随之刷新时，重新拷贝
`baseline-goldens`。进度与验证记录见 `PROGRESS.md`。
