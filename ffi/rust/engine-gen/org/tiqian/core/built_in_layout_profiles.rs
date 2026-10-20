use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::runtime::u_string::UStr;
use std::sync::LazyLock;


pub static BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL: LazyLock<LayoutProfileId> = LazyLock::new(|| LayoutProfileId::new(&(UStr::new(&[99,108,114,101,113,45,104,111,114,105,122,111,110,116,97,108]))));

#[derive(Clone, Copy)]
pub struct BuiltInLayoutProfiles;

impl BuiltInLayoutProfiles {
}
