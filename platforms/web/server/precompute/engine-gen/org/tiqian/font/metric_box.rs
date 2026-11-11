#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetricBox {
    RawFontBox,
    IdeographicEmBox,
    IdeographicCharacterFace,
    SampledInkBox,
}

pub fn compare_metric_box(a: &MetricBox, b: &MetricBox) -> i32 {
    if a == b { return 0; }
    fn rank(v: &MetricBox) -> i32 {
        match v {
            MetricBox::RawFontBox => 0,
            MetricBox::IdeographicEmBox => 1,
            MetricBox::IdeographicCharacterFace => 2,
            MetricBox::SampledInkBox => 3,
        }
    }
    rank(a) - rank(b)
}

impl MetricBox {
    pub fn to_string(&self) -> String {
        match self {
            MetricBox::RawFontBox => "RawFontBox".to_string(),
            MetricBox::IdeographicEmBox => "IdeographicEmBox".to_string(),
            MetricBox::IdeographicCharacterFace => "IdeographicCharacterFace".to_string(),
            MetricBox::SampledInkBox => "SampledInkBox".to_string(),
        }
    }
}

impl MetricBox {
    pub fn name(&self) -> &'static str {
        match self {
            MetricBox::RawFontBox => "RawFontBox",
            MetricBox::IdeographicEmBox => "IdeographicEmBox",
            MetricBox::IdeographicCharacterFace => "IdeographicCharacterFace",
            MetricBox::SampledInkBox => "SampledInkBox",
        }
    }
}
