package org.tiqian.protocol

object Revision {
    const val SNAPSHOT_SCHEMA: Int = 1
    const val SNAPSHOT_TABLES_SCHEMA: Int = 2
    const val LAYOUT_REVISION: String = "tiqian-layout-v2"
    const val RENDER_REVISION: String = "prebroken-dom-v16"
    const val FONT_SOURCE_POLICY: String = "host-compatible-stylesheet-v1"
    const val FONT_BACKEND_REVISION: String = "tiqian-shared-harfbuzz-v5"
    const val FONT_REPLAY_REVISION: String = "tiqian-server-shaping-replay-v1"
    const val FONT_REPLAY_TRANSPORT: String = "shared-strings-v1"
    const val FONT_BACKEND_PROTOCOL_REVISION: Int = 2
}
