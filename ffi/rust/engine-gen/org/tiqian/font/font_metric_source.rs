#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontMetricSource {
    RawTables,
    OpenTypeBase,
    GlyphSampling,
    ManualOverride,
    SynthesizedIdeographicBox,
}

pub fn compare_font_metric_source(a: &FontMetricSource, b: &FontMetricSource) -> i32 {
    if a == b { return 0; }
    fn rank(v: &FontMetricSource) -> i32 {
        match v {
            FontMetricSource::RawTables => 0,
            FontMetricSource::OpenTypeBase => 1,
            FontMetricSource::GlyphSampling => 2,
            FontMetricSource::ManualOverride => 3,
            FontMetricSource::SynthesizedIdeographicBox => 4,
        }
    }
    rank(a) - rank(b)
}

impl FontMetricSource {
    pub fn to_string(&self) -> String {
        match self {
            FontMetricSource::RawTables => "RawTables".to_string(),
            FontMetricSource::OpenTypeBase => "OpenTypeBase".to_string(),
            FontMetricSource::GlyphSampling => "GlyphSampling".to_string(),
            FontMetricSource::ManualOverride => "ManualOverride".to_string(),
            FontMetricSource::SynthesizedIdeographicBox => "SynthesizedIdeographicBox".to_string(),
        }
    }
}

impl FontMetricSource {
    pub fn name(&self) -> &'static str {
        match self {
            FontMetricSource::RawTables => "RawTables",
            FontMetricSource::OpenTypeBase => "OpenTypeBase",
            FontMetricSource::GlyphSampling => "GlyphSampling",
            FontMetricSource::ManualOverride => "ManualOverride",
            FontMetricSource::SynthesizedIdeographicBox => "SynthesizedIdeographicBox",
        }
    }
}
