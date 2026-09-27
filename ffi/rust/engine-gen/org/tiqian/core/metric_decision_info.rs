use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct MetricDecisionInfo {
    pub range: TextRange,
    pub source_text: UString,
    pub role: UString,
    pub font_key: UString,
    pub raw_ascent: f64,
    pub raw_descent: f64,
    pub raw_leading: f64,
    pub raw_source: UString,
    pub layout_ascent: f64,
    pub layout_descent: f64,
    pub baseline_class: UString,
    pub metric_box: UString,
    pub layout_source: UString,
    pub reason: UString,
}

impl MetricDecisionInfo {
    pub fn new(range: TextRange, source_text: &UStr, role: &UStr, font_key: &UStr, raw_ascent: f64, raw_descent: f64, raw_leading: f64, raw_source: &UStr, layout_ascent: f64, layout_descent: f64, baseline_class: &UStr, metric_box: &UStr, layout_source: &UStr, reason: &UStr) -> Self {
        Self {
            range,
            source_text: source_text.to_ustring(),
            role: role.to_ustring(),
            font_key: font_key.to_ustring(),
            raw_ascent,
            raw_descent,
            raw_leading,
            raw_source: raw_source.to_ustring(),
            layout_ascent,
            layout_descent,
            baseline_class: baseline_class.to_ustring(),
            metric_box: metric_box.to_ustring(),
            layout_source: layout_source.to_ustring(),
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("MetricDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("role=")); __s += (self.role).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontKey=")); __s += (self.font_key).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("rawAscent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.raw_ascent)); __s += &(UString::from(", ")); __s += &(UString::from("rawDescent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.raw_descent)); __s += &(UString::from(", ")); __s += &(UString::from("rawLeading=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.raw_leading)); __s += &(UString::from(", ")); __s += &(UString::from("rawSource=")); __s += (self.raw_source).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("layoutAscent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.layout_ascent)); __s += &(UString::from(", ")); __s += &(UString::from("layoutDescent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.layout_descent)); __s += &(UString::from(", ")); __s += &(UString::from("baselineClass=")); __s += (self.baseline_class).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("metricBox=")); __s += (self.metric_box).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("layoutSource=")); __s += (self.layout_source).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
