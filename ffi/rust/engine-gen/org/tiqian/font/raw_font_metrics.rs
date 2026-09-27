use crate::org::tiqian::font::font_metric_source::FontMetricSource;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct RawFontMetrics {
    pub ascent: f64,
    pub descent: f64,
    pub leading: f64,
    pub source: FontMetricSource,
    pub typo_ascent: Option<f64>,
    pub typo_descent: Option<f64>,
}

impl RawFontMetrics {
    pub fn new(ascent: f64, descent: f64, leading: Option<f64>, source: Option<FontMetricSource>, typo_ascent: Option<f64>, typo_descent: Option<f64>) -> Self {
        let leading = leading.unwrap_or_else(|| 0 as f64);
        let source = source.unwrap_or_else(|| FontMetricSource::RawTables);
        Self {
            ascent,
            descent,
            leading: leading,
            source: source,
            typo_ascent,
            typo_descent,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RawFontMetrics(")); __s += &(UString::from("ascent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.ascent)); __s += &(UString::from(", ")); __s += &(UString::from("descent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.descent)); __s += &(UString::from(", ")); __s += &(UString::from("leading=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading)); __s += &(UString::from(", ")); __s += &(UString::from("source=")); __s += UString::from(self.source.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("typoAscent=")); __s += &(match self.typo_ascent { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("typoDescent=")); __s += &(match self.typo_descent { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(")")); __s }).as_str());
    }
}
