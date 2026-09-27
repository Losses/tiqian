#[derive(Clone, Copy)]
pub struct Revision;

impl Revision {
    pub const REVISION_SNAPSHOT_SCHEMA: u32 = 1;
    pub const REVISION_SNAPSHOT_TABLES_SCHEMA: u32 = 2;
    pub const REVISION_LAYOUT_REVISION: &str = "tiqian-layout-v2";
    pub const REVISION_RENDER_REVISION: &str = "prebroken-dom-v16";
    pub const REVISION_FONT_SOURCE_POLICY: &str = "host-compatible-stylesheet-v1";
    pub const REVISION_FONT_BACKEND_REVISION: &str = "tiqian-shared-harfbuzz-v5";
    pub const REVISION_FONT_REPLAY_REVISION: &str = "tiqian-server-shaping-replay-v1";
    pub const REVISION_FONT_REPLAY_TRANSPORT: &str = "shared-strings-v1";
    pub const REVISION_FONT_BACKEND_PROTOCOL_REVISION: u32 = 2;
}
