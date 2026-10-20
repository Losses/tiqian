//! Snapshot bundle serialization in the js object literal field order
//! (ADR 0050). The HTML parse, projection and prepare lanes moved to the
//! TypeScript `precompute-html` entry; this module keeps only the shared
//! bundle shape the assemble calls return to js.

use crate::json::Json;
use crate::snapshot_bundle::SnapshotBundle;

/// The bundle in the js object literal field order.
pub fn bundle_json(bundle: &SnapshotBundle) -> Json {
    Json::Obj(vec![
        ("id".to_string(), Json::str(bundle.id.clone())),
        ("template".to_string(), Json::str(bundle.template.clone())),
        (
            "clientTemplate".to_string(),
            Json::str(bundle.client_template.clone()),
        ),
        (
            "inertTemplate".to_string(),
            Json::str(bundle.inert_template.clone()),
        ),
        (
            "initialStyle".to_string(),
            Json::str(bundle.initial_style.clone()),
        ),
        (
            "renderFontFamilies".to_string(),
            bundle.render_font_families.clone(),
        ),
        ("fontPreloads".to_string(), bundle.font_preloads.clone()),
        ("rootAttributes".to_string(), bundle.root_attributes.clone()),
        ("entries".to_string(), bundle.entries.clone()),
    ])
}

