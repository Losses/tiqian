# protocol 单源化的 boring spec 阅读笔记（Stage1-P1）

生成器修订：tiqian/.haxelib/boring/git @ be1d8e45。每条记录「文件:行 + 原文片段」。

## haxe.io.Bytes / BytesBuffer（stdlib/01-haxe-io-bytes.md）
- :9-21 API 面：`Bytes.ofString(s:String):Bytes`(:12)、`b.get(pos:Int):Int`(:15)、`b.toHex():String`(:21)。
- :23 目标表示：`On the TypeScript target, haxe.io.Bytes wraps a Uint8Array over an ArrayBuffer.`
- :169 Ruling：`haxe.io.Bytes remains the Haxe source type. ... TypeScript uses Uint8Array ... BytesBuffer remains the growable sink`
- :183 修正案：`haxe.io.BytesBuffer gains add(b:Bytes):Void`。

结论：canonical 的 Writer 用 BytesBuffer(addByte/add/getBytes)，字符串经 Bytes.ofString 进缓冲；不手写 UTF-8。测试十六进制断言优先 toHex。

## Sha256（crypto/01-sha256-sha512.md）
- :9-11 API：`Sha256.make(data:Bytes):Bytes;`（第 9 行）；「portable Haxe code / No external crypto package」的原句在本文件 Implementation 段（`The source implementations are portable Haxe code. PHP and Java branches are not part of this capability.`），第 9-11 行本身只有两条签名。
- 实现 pin 内 samples/haxe/crypto/Sha256.hx（:10 `class Sha256`、:85 `public static function make(data:Bytes):Bytes`）。

结论：Canonical.digest 单源入口 = Sha256.make。

## 枚举与模式匹配（features/01-enums-and-pattern-matching.md）
- :335 TS 载荷访问：`TypeScript branches on the discriminant and then reads the payload property (error.remaining), where narrowing supplies the type. TypeScript never renders payload extraction as property access on a widened type with a cast.`

结论：生成代码按判别式收窄读载荷；Haxe 侧不写「构造枚举值再模式绑定」的回传写法（listMember 改 guard 谓词 + arrOf）。

## 控制流（features/15-control-flow.md）
- :181 TS switch 条件：`Haxe switch translates to TypeScript as a switch statement only when every case body ends in return or throw ... When a case body must fall through to shared logic, the translation uses an if/else chain on the discriminant instead.`

结论：canonical 中带循环体的分支一律提出成独立方法；switch 只保留「每臂 return/throw」或纯值臂形态。

## 表达式位块（features/43-expression-block-scopes.md）
- :5 Accepted subset：`Every statement before the final statement must be a declaration (TVar), with or without an initializer. The final statement must be a value expression.`（:7-9 为理由段。）

结论：表达式位不放 while/if；此前两条「生成器缺陷」结论（Bytes.get 未降级、模式绑定未定义）实为违反本条的源码形态所致，已撤回。

## 束驱动器（features/59-bundle-driver.md）
- :33-40 bundle 字段表（id/target/precision/build/run/package）。
- :62-67 输出目录：`<outRoot>/<id>/gen and <outRoot>/<id>/gen-tests. A project file may not name them.`
- :78-90 讲 generation defines 与 manifest/artifact defines（`**Generation defines.** `<target>-output` and `<target>-test-output` from the two directories above`）；rust 配方两步的代码在 Driver.hx:602-604（`cargo test --no-run` 后 `cargo test`，工作目录 <outRoot>/<id>/gen）。

## 字节序与语义的权威（既有实现 + golden）
- platforms/web/server/core/src/canonical.ts:18 `Buffer.from("TQCS", "ascii")`、:71 `setFloat64(0, value, true)`、:76/:86 `setUint32(..., true)`。
- platforms/web/server/precompute/engine/src/canonical.rs:23 `b"TQCS"`、:89 `to_le_bytes()`、:93 `to_bits().to_le_bytes()`。
- canonical.rs:54 `canonical_f64`（非有限值丢弃、±0 折 +0）；canonical.ts:303 同义；可选数值字段 = 存在标志 + f64 位（canonical.rs:382、canonical.ts:113）。
- 验收 golden：canonical.test.ts:16 起 vector0-vector3 四条 hex，Rust 单测钉同一组。

## TS 目标两个已取证的形态前提（调试取证四步见 AGENTS.md）
- Bytes.get 降级条件：reflaxe/ts/tscompiler/TsExpr.hx:2191-2192 `if (name == "get" && isBytes(stripCast(subj)))`；isBytes(:3622-3626) 要求静态类型恰为 `haxe.io.Bytes`。传 `Null<Bytes>` 不命中、退化为普通字段调用。fix：switch 分支绑定非空 Bytes 后再调 get。
- samples 类的源范围过滤：TS 模块发射拒绝项在 tscompiler/Compiler.hx:500（`value-referenced but emits no declarations`），根因是类不在 Intercept.run 声明的源范围；按 rust-common.hxml 对 samples/std 的先例把 samples 路径加进源范围。

## 表二进制读写（Stage1-P2 补记，生成器修订 4f412c6a）

### BytesInput/BytesOutput 与端序（stdlib/02-haxe-io-buffers-and-inputs.md）
- :11 `haxe.io.BytesBuffer ... carries no built-in multi-byte endianness dispatch`。
- :12-13 API 面：BytesInput 提供 readByte/readInt32/readDouble/readString 与 bigEndian 属性；BytesOutput 提供 writeInt32/writeDouble。
- :15 裁定原文：`To maintain strict IEEE 754 bit-identical float representations across all targets, this repository bypasses BytesInput/BytesOutput endian methods in favor of explicit bitwise operations and haxe.io.FPHelper conversions`。
- :215 Ruling：统一原语集 `readU16/writeU16, readU32/writeU32, readF64/writeF64`，加固定长 ASCII 读写。
- :217 边界检查位置：`Bounds checking lives in reader slice extraction ... and writer capacity growth (ensure in TypeScript, growable BytesBuffer in Haxe, Vec::extend_from_slice in Rust)`。
- :24-38 样例形态：writer 手拆字节 addByte，f64 经 `FPHelper.doubleToI64` 取 high/low 两个 u32；reader 用 Bytes.get 逐字节组装。该样例是大端；TIQTBL03 全小端，出参顺序反转即可。

结论：P2 的 Reader/Writer 不用 BytesInput/BytesOutput，用 BytesBuffer.addByte + Bytes.get + FPHelper（stdlib/05:89-92 `FPHelper continues to carry binary64 bit patterns through high and low words`），与 Canonical.hx 的 Writer 同形态。

### haxe.Json 不在子集内（stdlib/ 全目录检索）
- docs/specs/ 下 grep `haxe.Json` 与 `Json.parse` 零命中；stdlib/06-std-modules.md、stdlib/12-std-string.md 均未列 JSON。
- 结论：face/typography/valueStyle/revision 四个 JSON 文本区，Haxe 侧只做字节区搬运（写入方收已序列化的 canonical JSON 文本，读取方交回原始文本），JSON.parse/stable_stringify 留平台壳；这符合 6.2 节「出口语言零手写读写代码」——JSON 解析不属于字节读写。

### TIQTBL03 布局与语义权威（既有实现）
- snapshot_table_binary.rs:9-35 布局注释：magic "TIQTBL03" 8 字节 + 12 个 u32 计数（56 字节头），string/delta 区 u32 增量自隐式零累加。
- snapshot-table-binary.ts:8-13 常量：HEADER_U32_COUNT=12、METRIC_POOL_ROW_BYTES=40、PROBE_STYLE_ROW_BYTES=25、ABSENT_METRIC_BITS=0x7ff8000000000000n。
- table-binary-writer.ts:200-253 region 顺序逐项：strings(delta+bytes)、metric 六列、valuePool(5xf64)、probe 三列、advancePool(f64)、stylePool(25B)、features(delta+bytes, 行=u16 count+count x u32)、face/typography/valueStyle/fontPreload 四个 delta+bytes、revisionText 尾区。
- metric 行排序键 (familiesRef, weight, italic, roleRef, faceSelectionRef)：snapshot_table_binary.rs:109-118 total_cmp；table-binary-writer.ts:132-137 数值比较。f64 排序比较在 Haxe 侧的形态待取证（total_cmp 与 JS < 的 NaN 行为不同；表内容 weight 为正常值时等价）。

## FPHelper.i64ToDouble 在 TS 发射端的参数序缺陷（P2 取证，pin 4f412c6a）
- ① 被编译源：engine-haxe/src/org/tiqian/protocol/SnapshotTableBinary.hx TableReader.f64：`final low = reader.u32(); final high = reader.u32(); return FPHelper.i64ToDouble(high, low);`（Haxe std 签名 FPHelper.hx i64ToDouble(high:Int, low:Int)，高字在前）。
- ② 发射端：packages/compiler/reflaxe/ts 对 FPHelper.i64ToDouble 的降级按源参数序原样改名转发（低字仍落在第二个实参），未按运行时签名交换。
- ③ 生成文本：engine-haxe/out/protocol-ts/gen/org/tiqian/protocol/SnapshotTableBinary.ts f64() 尾行 `return i64ToDouble(high, low);`，而 gen/runtime.ts:14 声明 `export function i64ToDouble(low: number, high: number)`（低位在前）。
- ④ 运行时失败：bun 实测 400.0 的 f64 解码得 5.34416817e-315（位形 0x0000000040790000，即高低字互换后的值）。
- 结论：TS 侧 f64 读取路径在 pin 4f412c6a 上不可用，修复属 boring TS 发射端（在 FPHelper 降级处交换两参），P2 席不改源码形态绕行；写路径 doubleToI64 不受影响（P1 golden 全过）。

