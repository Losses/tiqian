use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;


pub trait ClreqProfileResolver: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver>;
    fn resolve(&self, profile_id: LayoutProfileId) -> ClreqProfile;
}

impl Clone for Box<dyn ClreqProfileResolver> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn ClreqProfileResolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

#[derive(Clone, PartialEq)]
pub struct BuiltInClreqProfileResolver {
}

impl BuiltInClreqProfileResolver {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn resolve(&self, profile_id: LayoutProfileId) -> ClreqProfile {
        if profile_id.value.to_string() == ((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone().value).to_string() || (profile_id.value).to_string() ==
((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().id).to_string() {
            return ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone()).clone();
        }
        return ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone()).clone();
    }
}

impl ClreqProfileResolver for BuiltInClreqProfileResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.clreq.ClreqProfileResolver.BuiltInClreqProfileResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, profile_id: LayoutProfileId) -> ClreqProfile {
        if profile_id.value.to_string() == ((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone().value).to_string() || (profile_id.value).to_string() ==
((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().id).to_string() {
            return ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone()).clone();
        }
        return ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone()).clone();
    }
}
