use crate::org::tiqian::font::baseline_class::BaselineClass;
use crate::org::tiqian::font::baseline_policy::BaselinePolicy;
use crate::org::tiqian::font::font_metric_source::FontMetricSource;
use crate::org::tiqian::font::font_metrics_policy::FontMetricsPolicy;
use crate::org::tiqian::font::metric_box::MetricBox;
use crate::runtime::u_string::UString;


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
    pub reason: UString,
}

impl LayoutFontMetrics {
    pub fn new(ascent: f64, descent: f64, baseline_offset: f64, policy: FontMetricsPolicy, baseline_policy: BaselinePolicy, baseline_class: Option<BaselineClass>, metric_box: Option<MetricBox>, source: Option<FontMetricSource>, reason: Option<UString>) -> Self {
        let baseline_class = baseline_class.unwrap_or_else(|| BaselineClass::Roman);
        let metric_box = metric_box.unwrap_or_else(|| MetricBox::RawFontBox);
        let source = source.unwrap_or_else(|| FontMetricSource::RawTables);
        let reason = reason.unwrap_or_else(|| UString::from(""));
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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LayoutFontMetrics(")); __s += &(UString::from("ascent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.ascent)); __s += &(UString::from(", ")); __s += &(UString::from("descent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.descent)); __s += &(UString::from(", ")); __s += &(UString::from("baselineOffset=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.baseline_offset)); __s += &(UString::from(", ")); __s += &(UString::from("policy=")); __s += UString::from(self.policy.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("baselinePolicy=")); __s += UString::from(self.baseline_policy.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("baselineClass=")); __s += UString::from(self.baseline_class.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("metricBox=")); __s += UString::from(self.metric_box.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("source=")); __s += UString::from(self.source.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
