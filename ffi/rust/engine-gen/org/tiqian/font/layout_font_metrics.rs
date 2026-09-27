use crate::org::tiqian::font::baseline_class::BaselineClass;
use crate::org::tiqian::font::baseline_policy::BaselinePolicy;
use crate::org::tiqian::font::font_metric_source::FontMetricSource;
use crate::org::tiqian::font::font_metrics_policy::FontMetricsPolicy;
use crate::org::tiqian::font::metric_box::MetricBox;


#[derive(Debug, Clone, PartialEq)]
pub struct LayoutFontMetrics {
    pub ascent: f64,
    pub descent: f64,
    pub baseline_offset: f64,
    pub policy: FontMetricsPolicy,
    pub baseline_policy: BaselinePolicy,
    pub baseline_class: BaselineClass,
    pub metric_box: MetricBox,
    pub source: FontMetricSource,
    pub reason: String,
}

impl LayoutFontMetrics {
    pub fn new(ascent: f64, descent: f64, baseline_offset: f64, policy: FontMetricsPolicy, baseline_policy: BaselinePolicy, baseline_class: Option<BaselineClass>, metric_box: Option<MetricBox>, source: Option<FontMetricSource>, reason: Option<String>) -> Self {
        let baseline_class = baseline_class.unwrap_or_else(|| BaselineClass::Roman);
        let metric_box = metric_box.unwrap_or_else(|| MetricBox::RawFontBox);
        let source = source.unwrap_or_else(|| FontMetricSource::RawTables);
        let reason = reason.unwrap_or_else(|| "".to_string());
        Self {
            ascent,
            descent,
            baseline_offset,
            policy,
            baseline_policy,
            baseline_class: baseline_class,
            metric_box: metric_box,
            source: source,
            reason: reason,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LayoutFontMetrics(",
            "ascent=",
            self.ascent,
            ", ",
            "descent=",
            self.descent,
            ", ",
            "baselineOffset=",
            self.baseline_offset,
            ", ",
            "policy=",
            self.policy.name(),
            ", ",
            "baselinePolicy=",
            self.baseline_policy.name(),
            ", ",
            "baselineClass=",
            self.baseline_class.name(),
            ", ",
            "metricBox=",
            self.metric_box.name(),
            ", ",
            "source=",
            self.source.name(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
