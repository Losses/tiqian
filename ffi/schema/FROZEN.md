# assembly-record 产物冻结注记

- 生成器 tools/schema/generate_rust.py 与 generate_ts.py 已于提交 5d366296 删除（Stage1-P2 判据 2）。
- assembly-record 的 TS/Rust DTO 产物在该提交点冻结，当前为唯一事实来源。
- 后续由 P5（request-model 单源化）的生成物接线取代；在 P5 接线落地前，不要把本目录产物当作可由脚本重生成的产物。
