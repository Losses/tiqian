use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct MetricDecisionInfo {
    pub range: TextRange,
    pub source_text: String,
    pub role: String,
    pub font_key: String,
    pub raw_ascent: f64,
    pub raw_descent: f64,
    pub raw_leading: f64,
    pub raw_source: String,
    pub layout_ascent: f64,
    pub layout_descent: f64,
    pub baseline_class: String,
    pub metric_box: String,
    pub layout_source: String,
    pub reason: String,
}

impl MetricDecisionInfo {
    pub fn new(range: TextRange, source_text: &str, role: &str, font_key: &str, raw_ascent: f64, raw_descent: f64, raw_leading: f64, raw_source: &str, layout_ascent: f64, layout_descent: f64, baseline_class: &str, metric_box: &str, layout_source: &str, reason: &str) -> Self {
        Self {
            range,
            source_text: source_text.to_string(),
            role: role.to_string(),
            font_key: font_key.to_string(),
            raw_ascent,
            raw_descent,
            raw_leading,
            raw_source: raw_source.to_string(),
            layout_ascent,
            layout_descent,
            baseline_class: baseline_class.to_string(),
            metric_box: metric_box.to_string(),
            layout_source: layout_source.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "MetricDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "role=",
            (self.role).to_string(),
            ", ",
            "fontKey=",
            (self.font_key).to_string(),
            ", ",
            "rawAscent=",
            self.raw_ascent,
            ", ",
            "rawDescent=",
            self.raw_descent,
            ", ",
            "rawLeading=",
            self.raw_leading,
            ", ",
            "rawSource=",
            (self.raw_source).to_string(),
            ", ",
            "layoutAscent=",
            self.layout_ascent,
            ", ",
            "layoutDescent=",
            self.layout_descent,
            ", ",
            "baselineClass=",
            (self.baseline_class).to_string(),
            ", ",
            "metricBox=",
            (self.metric_box).to_string(),
            ", ",
            "layoutSource=",
            (self.layout_source).to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
