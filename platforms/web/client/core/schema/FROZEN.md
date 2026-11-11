# assembly-record 产物冻结注记

- 生成器 `tools/schema/generate_rust.py` 与 `generate_ts.py` 已于提交 `5d366296` 删除（Stage1-P2 判据 2），`tools/schema/` 目录同时消失。
- 本目录的 `assembly-record.schema.json` 定义 assembly record 的标量字段、记录表、三个 wire 面与投影规则。`surfaces.workerJson.fields` 仍是 `LAYOUT_REQUEST_FIELDS` 的唯一规格。`surfaces.rustAbi` 与 `surfaces.engineWireFace` 的生产者路径（native ABI 头、Rust 请求 DTO、layout 模块）已随割接删除，这两段只保留历史定义。
- TS 侧冻结产物是 `platforms/web/client/core/src/engine/web-worker/assembly-record-fields.ts`。该文件不可由脚本重新生成，改动必须手工同步。
- Rust 侧的请求 DTO 已由 Stage1-P5 的生成物 `platforms/web/server/precompute/protocol-gen/org/tiqian/protocol/paragraph_request.rs` 承担，原 `layout_request.rs` 已删除。本目录不再为 Rust 提供 DTO。
- Stage1-P5 与 Stage1-P5b 已完成请求模型的单源生成（Haxe 源 `engine-haxe/src/org/tiqian/protocol/`，TS/Rust/Kotlin 三束）。生成树没有提供运行时的 worker 字段名数组，`LAYOUT_REQUEST_FIELDS` 的手工同步判据见下文。
- 本目录由已解散的顶层 ffi 目录迁入 `platforms/web/client/core/schema`，相对路径以本文件所在目录为基准。

## 当前消费方

- `platforms/web/client/core/src/engine/web-worker/worker-channel.ts:21` 导入 `LAYOUT_REQUEST_FIELDS`，在 `:224` 的 `layoutRequestKey` 中按该顺序构造缓存键。
- `platforms/web/client/web-component/bench/worker-necessity.ts:403` 以 `@tiqian/core/src/engine/web-worker/assembly-record-fields.js` 导入同一模块，在 `:440` 的基准测量中使用 `LAYOUT_REQUEST_FIELDS`。

## 冻结边界

- `LAYOUT_REQUEST_FIELDS` 的顺序必须与 `surfaces.workerJson.fields` 逐项相等。
- `ASSEMBLY_RECORD_REVISION` 必须等于 `assembly-record.schema.json` 的 `revision`。
- `AssemblyRecordRequest` 的 JSDoc 类型必须与 schema 的标量类型和记录表类型一致。
- 修改 wire 形状时，先改 schema 并提升 `revision`，再在同一提交内手工更新上述三处。
- 本目录的产物不得被当作可由脚本重新生成的对象；新增生成器之前，不得在构建脚本中调用 `tools/schema/generate_ts.py` 或 `generate_rust.py`。
