use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct CjkDashCapabilityPolicy;

impl CjkDashCapabilityPolicy {
    pub fn cjk_dash_capability_policy_issue_name_for(status: Option<UString>) -> UString {
        return if status.as_ref().map_or(false, |v| v == &(UString::from("conforming").to_ustring())) { UString::from("ConformingCjkDashRequiresExactFontSession") } else { UString::from("NoConformingCjkDashGlyph") };
    }

    pub fn cjk_dash_capability_policy_issue_detail_for(status: Option<UString>, detail: Option<UString>) -> UString {
        return match &(status) { None => UString::from("CjkDashFontShapingNotPrepared"), Some(__option) => if match &(detail) { None => true,
Some(__option2) => __option2.trim() == UString::from("") } { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("status=")); __s += __option.as_ustr(); __s }).as_str()) } else { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("status=")); __s += __option.as_ustr(); __s += &(UString::from("; ")); __s += match &(detail) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s }).as_str()) }.to_ustring() };
    }
}
