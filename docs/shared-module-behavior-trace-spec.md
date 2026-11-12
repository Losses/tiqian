# 共享模块标准行为测试集与 trace 验证规格

本文定义用「标准行为测试集 + trace 验证」替换 `tools/registry-guard/backstop.ts`
grep 断言的规格。规格只定义用例类别、记录字段与比较判据，不包含实现。

## 背景与现状

`SemanticPassRegistry` 注册的共享模块（`DataTableHelper`、`DefaultArgExpander`、
`EnumCycleDetector` 等 23 个）当前的验证手段是
`tools/registry-guard/backstop.ts`：该脚本对每个消费目标的
`packages/compiler/reflaxe/<target>` 树做文本扫描，断言目标提及注册模块且不重定义模块。
grep 断言只证明「名称出现」，不证明「行为一致」：一个目标可以正确引用模块名称，
却在该模块的数值排序、异常命名或浮点打印上偏离其他目标，grep 断言对此不报错。

目标行为的既有验证基础设施有两处，规格全部复用它们，不新建格式：

1. trace 行语法（`engine TraceFormat`，解析实现见 `tiqian/engine-haxe/tools/compare-traces.py`）：
   `class: <Name>` 与 `test: <fn>` 两类头行，其后为 `<event> key=value ...` 事件行；
   数值为十进制文本，`NaN`、`Infinity` 与 `-Infinity` 以带引号文本存储，
   超长文本写入截断标记 `~<len>#<hash>`。
2. 逐测试判定记录：每用例一行 JSONL，`{"id", "name", "verdict", "message"}`，
   解析见 `tiqian/engine-haxe/tools/compare-verdicts-l2.py`。

第三处基础设施（引擎 golden 轨迹）随 Kotlin 割接删除：golden 是对齐工具，
105/105 对齐达成后没有保留用途。

## 标准行为测试集的用例类别

每个注册共享模块至少覆盖下列类别中与其职责相关的类别；与职责无关的类别
（例如 `PackageArtifacts` 不涉及闭包）允许缺省，缺省须在模块的用例清单中注明理由。

### 数值

最小用例形态：同一表达式在 f32 与 f64 两种精度下各运行一次，trace 记录结果值。
必须覆盖的值集合：`+0` 与 `-0`、quiet NaN 与 signaling NaN（以位模式构造，见
`boring/tests/ts/float-canonical.test.ts` 的 `f64FromBits` 形态）、正负 Infinity、
最小正规数、最大有限值、舍入到不同位模式的中间值。
期望记录字段：结果值的位模式十六进制（沿 `canonicalFloatBits` 的编码），
禁止只记录十进制文本，因为不同运行时的浮点转十进制规则会产出位数不同的文本
（判据依据见 `tiqian/docs/cross-target-alignment.md`）。
比较判据：逐位。sin、cos、pow 的末位差按偏离处理，无容差豁免。

### 字符串

最小用例形态：拼接、按码点截取、包含代理对（surrogate pair）的文本、
含引号与反斜杠的文本、超过截断阈值的文本。
期望记录字段：完整结果文本以带引号形式存储；超长文本写入
`~<len>#<hash>`，hash 覆盖完整操作数文本。
比较判据：逐值（码点序列相等）。f32 与 f64 下字符串行为相同，两侧各跑一次作冗余校验。

### 集合

最小用例形态：插入乱序键后按排序域输出；同键多值；嵌套记录与嵌套数组；
复制后修改副本。排序域用例须同时覆盖整数键、字符串键与混合键域
（既有先例见 `boring/tests/ts/sorted-key-domains.test.ts` 与
`sorted-dataclass-keys.test.ts`）。
期望记录字段：排序后的键序列与值序列，逐项一条事件；记录打印使用
`toString` 输出文本（先例见 `printed-record.test.ts`）。
比较判据：逐结构。先按排序域规范化键序，再逐项比对值；
值内部的浮点按数值判据逐位比较。

### 闭包

最小用例形态：捕获局部变量并在定义作用域外调用；捕获可变变量并观察两次调用
之间修改的效果；默认参数表达式每次调用求值（先例见 `default-args.test.ts`）。
期望记录字段：每次调用一条事件，记录返回值与捕获变量在调用时刻的值。
比较判据：逐值；返回值为浮点时逐位。

### 异常

最小用例形态：抛出注册的异常类并捕获；抛出未捕获异常由运行器记为失败判定；
异常消息含动态值。
期望记录字段：`raises exception=<Name>` 事件行；异常名等价规则
（`IllegalArgumentException` 与 Tiqian 前缀名互认）保留在
`compare-traces.py` 的 `EXCEPTION_NAME_ALIASES`，基准轨迹已删除。
比较判据：逐值。异常名与消息文本必须完全一致；别名规则仅用于内置名向
Tiqian 前缀名的过渡，迁移完成后删除。

### 异步

最小用例形态：两个并发任务按确定调度顺序完成；一个任务失败另一个成功。
期望记录字段：完成事件按实际完成顺序记录，每条记录任务 id 与结果；
trace 中记录事件序号，比较时要求序号序列一致。
比较判据：逐值加逐序。凡调度顺序不确定的用例不得进入测试集；
若某目标的运行时无法保证顺序，该目标记为偏离而非跳过。

### 边界

最小用例形态：空集合、空字符串、单元素集合、达到截断阈值的字符串、
超过截断阈值的字符串、深度嵌套达到记录打印的递归上限、整型运算达到位宽上限。
期望记录字段：与对应常规类别相同；截断行为额外记录截断标记。
比较判据：与对应常规类别相同；截断标记只比较标记种类，
标记内 hash 不参与跨精度比较（沿 `compare-traces.py` 的既有规则）。

## trace 记录字段规格

每条标准行为测试的 trace 输出沿用 TraceFormat，固定字段如下：

| 字段 | 形态 | 说明 |
|---|---|---|
| class | `class: <Name>` | 被测共享模块名，与 `SemanticPassRegistry` 注册名一致 |
| test | `test: <fn>` | 用例函数名，命名包含类别与场景 |
| event | `<event> key=value ...` | 一次可观察行为的记录；数值按精度记录位模式 |
| verdict | JSONL `{"id","name","verdict","message"}` | 每用例一行的运行判定，`id` 形如 `<module>.<category>.<case>` |

JSONL 行的 `id` 三段命名保证 compare 工具可以按模块与类别聚合偏离，
偏离报告不需要新增解析逻辑。

## 跨目标比较判据

三级判据按对象类型选择，选型规则写入每个用例的期望记录说明：

1. 逐位（bit-exact）：浮点值。比较前把两侧文本解析回 binary32 或 binary64
   位模式再比较位模式；文本化差异不参与判定。
   入口沿 `engine-haxe/tools/compare-traces.py` 的 byte 模式。
2. 逐值：字符串、异常名、异常消息、事件序号。两侧文本完全一致，无容差。
3. 逐结构：集合与记录。先按排序域与打印格式规范化，再逐项套用逐位或逐值判据。

比较入口沿 `docs/cross-target-alignment.md` 的分层判据：
f32 侧由 `boring test kotlin-f32` 的生成测试判定，f64 侧五目标
（ts、kotlin、rust、swift、dart）逐行 JSONL 一致，命令入口为
`boring compare` 与 `boring verify`。

## 替换 backstop.ts 的方式与迁移顺序

backstop.ts 承担两项断言：消费目标提及注册模块（存在性），以及目标不重定义
注册模块（唯一性）。行为测试集只替代第一项；第二项由
`tests/haxe/semantic-registry-comment-control/run.sh` 的注册器宏时校验承担，
该校验已能拒绝仅注释、仅字符串与同前缀名称的伪消费（见
`.haxelib/boring/git/docs/compiler-policy-interfaces.md` 对 consumer 行的说明）。
因此迁移完成后，backstop.ts 整体删除，不保留精简版本。

迁移顺序，每步完成后进入下一步：

1. 建立标准行为测试集骨架：按本文类别为每个注册模块编写最小用例清单，
   用例以 boring 测试形态写入仓库，trace 输出接入 driver 的
   `BORING_TEST_RESULTS` 与 TraceFormat。
2. 双轨运行：backstop.ts 与行为测试集同时执行。运行 `boring verify`，
   记录每个模块在五个目标上的 verdict JSONL；此阶段 backstop.ts 的失败
   仍然阻塞，行为测试的失败记录为待修条目。
3. 逐模块切换判定：某模块的行为测试集在五个目标全部通过后，从 backstop.ts
   的 `consumers` 表删除该模块的行；backstop.ts 对已迁移模块不再扫描。
4. 收尾删除：最后一个模块迁移完成后，删除
   `tools/registry-guard/backstop.ts` 与其调用入口；唯一性断言由
   semantic-registry-comment-control 校验继续承担。若运行中发现
   trace 验证无法覆盖的共享机制（例如仅存在于编译期的行为），
   在删除前于本文档补充该机制的验证规格，不得静默保留 grep 断言。

## 结论

本规格定义了七类用例（数值、字符串、集合、闭包、异常、异步、边界）、
每类的最小用例形态与期望 trace 字段，以及逐位、逐值、逐结构三级比较判据，
全部复用 `engine TraceFormat` 与 `boring compare`/`boring verify` 既有入口。
迁移分四步：建骨架、双轨运行、逐模块从 `consumers` 表摘除、
删除 `tools/registry-guard/backstop.ts`；唯一性断言由
`tests/haxe/semantic-registry-comment-control/run.sh` 承担。