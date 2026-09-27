#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontMetricsPolicy {
    Raw,
    IdeographicBox,
    GlyphBoundsSampled,
    ManualOverride,
}

pub fn compare_font_metrics_policy(a: &FontMetricsPolicy, b: &FontMetricsPolicy) -> i32 {
    if a == b { return 0; }
    fn rank(v: &FontMetricsPolicy) -> i32 {
        match v {
            FontMetricsPolicy::Raw => 0,
            FontMetricsPolicy::IdeographicBox => 1,
            FontMetricsPolicy::GlyphBoundsSampled => 2,
            FontMetricsPolicy::ManualOverride => 3,
        }
    }
    rank(a) - rank(b)
}

impl FontMetricsPolicy {
    pub fn to_string(&self) -> String {
        match self {
            FontMetricsPolicy::Raw => "Raw".to_string(),
            FontMetricsPolicy::IdeographicBox => "IdeographicBox".to_string(),
            FontMetricsPolicy::GlyphBoundsSampled => "GlyphBoundsSampled".to_string(),
            FontMetricsPolicy::ManualOverride => "ManualOverride".to_string(),
        }
    }
}

impl FontMetricsPolicy {
    pub const ALL: [FontMetricsPolicy; 4] = [FontMetricsPolicy::Raw, FontMetricsPolicy::IdeographicBox, FontMetricsPolicy::GlyphBoundsSampled, FontMetricsPolicy::ManualOverride];
    pub fn name(&self) -> &'static str {
        match self {
            FontMetricsPolicy::Raw => "Raw",
            FontMetricsPolicy::IdeographicBox => "IdeographicBox",
            FontMetricsPolicy::GlyphBoundsSampled => "GlyphBoundsSampled",
            FontMetricsPolicy::ManualOverride => "ManualOverride",
        }
    }
}
