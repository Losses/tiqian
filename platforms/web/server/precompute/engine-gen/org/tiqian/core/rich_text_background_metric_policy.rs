#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RichTextBackgroundMetricPolicy {
    MarkedFaces,
    UniformTextStyle,
    UniformParagraphStyle,
}

pub fn compare_rich_text_background_metric_policy(a: &RichTextBackgroundMetricPolicy, b: &RichTextBackgroundMetricPolicy) -> i32 {
    if a == b { return 0; }
    fn rank(v: &RichTextBackgroundMetricPolicy) -> i32 {
        match v {
            RichTextBackgroundMetricPolicy::MarkedFaces => 0,
            RichTextBackgroundMetricPolicy::UniformTextStyle => 1,
            RichTextBackgroundMetricPolicy::UniformParagraphStyle => 2,
        }
    }
    rank(a) - rank(b)
}

impl RichTextBackgroundMetricPolicy {
    pub fn to_string(&self) -> String {
        match self {
            RichTextBackgroundMetricPolicy::MarkedFaces => "MarkedFaces".to_string(),
            RichTextBackgroundMetricPolicy::UniformTextStyle => "UniformTextStyle".to_string(),
            RichTextBackgroundMetricPolicy::UniformParagraphStyle => "UniformParagraphStyle".to_string(),
        }
    }
}

impl RichTextBackgroundMetricPolicy {
    pub fn name(&self) -> &'static str {
        match self {
            RichTextBackgroundMetricPolicy::MarkedFaces => "MarkedFaces",
            RichTextBackgroundMetricPolicy::UniformTextStyle => "UniformTextStyle",
            RichTextBackgroundMetricPolicy::UniformParagraphStyle => "UniformParagraphStyle",
        }
    }
}
