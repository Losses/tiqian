use crate::org::tiqian::font::font_metric_source::FontMetricSource;


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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "RawFontMetrics(",
            "ascent=",
            self.ascent,
            ", ",
            "descent=",
            self.descent,
            ", ",
            "leading=",
            self.leading,
            ", ",
            "source=",
            self.source.name(),
            ", ",
            "typoAscent=",
            match self.typo_ascent { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "typoDescent=",
            match self.typo_descent { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ")"
        );
    }
}
