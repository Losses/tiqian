#[derive(Clone, Copy)]
pub struct CjkDashCapabilityPolicy;

impl CjkDashCapabilityPolicy {
    pub fn cjk_dash_capability_policy_issue_name_for(status: Option<String>) -> String {
        return if status.as_ref().map_or(false, |v| v == &("conforming".to_string())) { "ConformingCjkDashRequiresExactFontSession".to_string() } else { "NoConformingCjkDashGlyph".to_string() };
    }

    pub fn cjk_dash_capability_policy_issue_detail_for(status: Option<String>, detail: Option<String>) -> String {
        return match &(status) { None => "CjkDashFontShapingNotPrepared".to_string(), Some(__option) => if match &(detail) { None => true, Some(__option2) => __option2.trim() == "" } { format!("{}{}",
            "status=",
            __option
        ).to_string() } else { format!("{}{}{}{}",
            "status=",
            __option,
            "; ",
            match detail { Some(ref v) => v.to_string(), None => "null".to_string() }
        ).to_string() }.to_string() };
    }
}
