use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use std::sync::LazyLock;


pub static BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL: LazyLock<LayoutProfileId> = LazyLock::new(|| LayoutProfileId::new("clreq-horizontal"));

#[derive(Clone, Copy)]
pub struct BuiltInLayoutProfiles;

impl BuiltInLayoutProfiles {
}
