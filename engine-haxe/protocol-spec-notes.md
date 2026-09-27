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

## Stage1-P5 宿主请求模型与校验（生成器修订 pin 4f412c6a）

## 结构与 typedef（features/03-structures-and-typedefs.md）
- :180 `Haxe anonymous structure typedefs translate to named `struct` declarations in Rust with public fields and derived traits (`Debug`, `Clone`, `Copy`, `PartialEq`), to named `interface` declarations in TypeScript with `readonly` properties, and to `data class` declarations in Kotlin with `val` properties.`

结论：ParagraphRequest/TextSpanInput/LineBreakSpanInput/InlineBoxInput 用 typedef 匿名结构声明，DTO 字段单源。

## 错误与结果（features/06-errors-and-results.md）
- :34 `The failure identity is a Haxe enum as ruled in docs/specs/features/01-enums-and-pattern-matching.md; the exception class carries the enum instance`（:37-51 给出 enum + `class VectorException extends haxe.Exception` 样板）。
- :314-318 Ruling：`Failure identity is a closed variant set defined once per domain and shared by all four trees`；Rust `all fallible operations return Result<T, DomainError>`；TS `throw sites construct one exception class carrying the error union value`。
- :397 `tests assert variant identity, never message content`。

结论：ParagraphRequestError 枚举逐变体承载领域错误名（EmptyParagraph/InvalidMaximumMeasure/…，名字照 paragraph.rs:72-131 validate 顺序），ParagraphRequestException 包装抛出；字节协议错误名不进来。

## 字符串长度（stdlib/06-std-modules.md）
- :318 `String.length and codeUnitAt are constant-time UTF-16 unit access;`

结论：范围与 boundary 检查直接用 text.length，对应 paragraph.rs:213-222 utf16_length 的 UTF-16 语义与 Kotlin String.length。

## 待取证（动手改 validate 形态前）
- 空白判定：Kotlin ParagraphWireCodec.kt:279 `text.isNotBlank()` 与 Rust paragraph.rs:73 `text.trim().is_empty()` 空白集不同；取证对象为 pin 内 StringTools/字符串降级发射端与 Rust 生成侧 UString trim 语义。
- throwing 方法在 rust 发射端的 fallibility 传播：取证对象为 pin 内 rust 发射端 TThrow 处理函数与生成文本。
- 字段集：decorations/inlineObjects/emphasisDotGapEm 仅在 Kotlin DTO，Rust ParagraphRequest 没有；单源取并集还是交集待队长定。

## TS 目标两个已取证的形态前提（调试取证四步见 AGENTS.md）
- Bytes.get 降级条件：reflaxe/ts/tscompiler/TsExpr.hx:2191-2192 `if (name == "get" && isBytes(stripCast(subj)))`；isBytes(:3622-3626) 要求静态类型恰为 `haxe.io.Bytes`。传 `Null<Bytes>` 不命中、退化为普通字段调用。fix：switch 分支绑定非空 Bytes 后再调 get。
- samples 类的源范围过滤：TS 模块发射拒绝项在 tscompiler/Compiler.hx:500（`value-referenced but emits no declarations`），根因是类不在 Intercept.run 声明的源范围；按 rust-common.hxml 对 samples/std 的先例把 samples 路径加进源范围。

## Stage1-P6 错误命名 NamedError 族（生成器修订 pin 4f412c6a）

## 错误与结果（features/06-errors-and-results.md）
- :314 Ruling：`Failure identity is a closed variant set defined once per domain and shared by all four trees. Each domain declares its variants in one commit touching every tree: the Haxe enum and its exception wrapper, the Rust error enum, the TypeScript error union and exception class, and the Kotlin sealed exception hierarchy.`
- :397 `tests assert variant identity, never message content`。

结论：NamedError 即领域验证错误名的跨四树闭包变体集；Haxe 枚举逐变体承载发布名，测试断言变体清单与名字逐项相等。

## 枚举与模式匹配（features/01-enums-and-pattern-matching.md）
- :5 Scope：`In the current codebase, algebraic sum types appear in Haxe as the `VectorError` enum carried by `VectorException` in `samples/boring/`, in Rust as `VectorError` in `reference/rust/src/lib.rs`, in TypeScript as the `VectorError` union carried by `VectorException` in `reference/ts/src/vector-error.ts`, and in Kotlin as the sealed `VectorException` hierarchy in `reference/kotlin/src/boring/VectorException.kt`.`
- :335 TS 载荷访问（P1 已引）：`TypeScript branches on the discriminant and then reads the payload property (error.remaining), where narrowing supplies the type.`
- 事实（pin 4f412c6a）：reference/ 目录只有 rust 与 ts，无 reference/kotlin；Kotlin 形态以 Kotlin 发射端实际生成物为准（引擎 kotlin 束生成物里的枚举是正例）。
- Rust 参考形态：reference/rust/src/lib.rs:272 `pub enum VectorError`，显示文本渲染见本 spec :61-72 的 Display 实现。
- TS 参考形态：reference/ts/src/vector-error.ts:26-31 `export type VectorError = | BadMagicError | ...` 判别联合 + :32 `describeError`。

## 三 lane 名字站点（变体集与序的权威）
- Rust：paragraph.rs:74-131 的 `named("...")` 16 名按 validate() 检查序（EmptyParagraph…InvalidInlineBoxGeometry）；main 树其余站点 normalize.rs:143/147/151/154、precomputer.rs:286/441、precompute_html.rs:582；normalize.rs:32 PARAGRAPH_CAPABILITY_ISSUES 列表含 "EmptyParagraph"。
- Kotlin：ParagraphWireCodec.kt:279-297 与 :386-397 段级 8 名（含 Kotlin 独有的 InvalidEmphasisDotGapEm），:48-144 区段检查 13 名（含 Kotlin 独有的 InvalidInlineObjectRange/Advance/VerticalGeometry 与 InvalidDecorationRange）；:56/71/106/124/136 的 Invalid*Wire 是字符串打包名，随打包消失，不进枚举。
- TS：markdown-lowering.ts:778/:815 EmptyParagraph、lifecycle.ts:265 InvalidFontSize、astro/integration.ts:110 InvalidMaximumMeasure——三处属消费侧接线（需重构建 @tiqian/ffi 包），队长裁定移交 C1 的四条 JS 接线；P6 只交付生成物（TS 束 publish 到 platforms/web/server/core/src/protocol-gen/）。
- 字节协议名不进枚举：ffi/native/src/nativeMain/kotlin/org/tiqian/ffi/cabi/LayoutRequestReader.kt:40-228 的 InvalidLayoutRequest* 属打包机制，割接后随打包整体删除。

## 变体集与声明序（21 名）
- 序=paragraph.rs:74-131 检查序，Kotlin 独有名插到 Kotlin lane 对应检查位：InvalidEmphasisDotGapEm 在 InvalidFontWeight 后；InvalidInlineObjectRange/InvalidInlineObjectAdvance/InvalidInlineObjectVerticalGeometry 在 InvalidInlineBoxGeometry 后；InvalidDecorationRange 末位。
- TS 三名为该集子集；与 P5 分支（cutover/stage1-p5-request-model）的 ParagraphRequestError 同名同序（P5 未并），队长裁定 (b)：P5 落地后本分支 rebase 到 main 并把 checks/exception 改指向 NamedError，退役 P5 的枚举；本阶段不碰 paragraph.rs 与 ParagraphWireCodec.kt。

## 束接线
- protocol-ts.hxml / protocol-rust.hxml 加 root org.tiqian.protocol.NamedError 与 NamedErrorTest。
- 新 protocol-kotlin.hxml（按 protocol-ts.hxml 的 standalone 形态 + kotlin-common.hxml 的 kotlin 发射端行；输出目录不手抄 define，由驱动器按 spec 59:62-67 从 outRoot/<id> 派生）+ boring.json 加 protocol-kotlin 束（任务书要求生成 Kotlin/TS/Rust）。
- 生成物 vendored（只从驱动器产物搬，结构与 gen 根逐项一致）：TS→platforms/web/server/core/src/protocol-gen/org/tiqian/protocol/；Rust→platforms/web/server/precompute/protocol-gen/org/tiqian/protocol/（crate tiqian-protocol-gen，engine 经 path 依赖）；Kotlin→ffi/js/src/jsMain/kotlin/org/tiqian/protocol/。

## 待取证（动手改调用点前，取证对象为 pin 内发射端函数）
- 无 exception 包装的纯枚举在三目标的渲染形态：TS 发射端为 .haxelib/boring/git/packages/compiler/reflaxe/ts/tscompiler/ 内渲染 VectorError 参考物的函数（模块发射拒绝点在 tscompiler/Compiler.hx:500）；Rust 发射端为 packages/compiler/reflaxe/rust/ 内枚举渲染函数；Kotlin 发射端为 Kotlin 编译器内枚举渲染函数。
- 生成物是否已把变体名暴露为字符串（Rust Display/str、TS kind 判别式、Kotlin 枚举名）——决定是否补 describe 类。
- Kotlin 渲染形态（enum class 还是 sealed class）——决定 Kotlin 调用点引用写法。
- mac ssh 不可达（banner 前断连）：protocol-gen 小 crate 按本机例外编译（cp -a 到 /tmp/<label>-pg，XDG_CACHE_HOME=/tmp/tq-nix-cache nix develop 内 cargo build --lib --message-format=short，跑完删 target）；整棵 precompute engine 树仍只在 mac 上构建，本阶段 engine crate 改动只做机械字面量替换并以生成 crate 的本地编译+三侧名字 diff 佐证。
