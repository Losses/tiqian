use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::baseline_class::BaselineClass;
use crate::org::tiqian::font::baseline_policy::BaselinePolicy;
use crate::org::tiqian::font::font_metric_source::FontMetricSource;
use crate::org::tiqian::font::font_metrics_policy::FontMetricsPolicy;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::layout_font_metrics::LayoutFontMetrics;
use crate::org::tiqian::font::metric_box::MetricBox;
use crate::org::tiqian::font::raw_font_metrics::RawFontMetrics;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct FontMetricsRequest {
    pub font_key: String,
    pub font_size: f64,
    pub role: FontRole,
    pub locale: String,
    pub font_families: Vec<String>,
    pub font_weight: u32,
    pub italic: bool,
    pub face_selection_text: String,
}

impl FontMetricsRequest {
    pub fn new(font_key: &str, font_size: f64, role: FontRole, locale: &str, font_families: Option<Vec<String>>, font_weight: Option<u32>, italic: Option<bool>, face_selection_text: Option<String>) -> Self {
        let font_families = font_families.unwrap_or_else(|| vec![]);
        let font_weight = font_weight.unwrap_or_else(|| 400);
        let italic = italic.unwrap_or_else(|| false);
        let face_selection_text = face_selection_text.unwrap_or_else(|| "".to_string());
        Self {
            font_key: font_key.to_string(),
            font_size,
            role,
            locale: locale.to_string(),
            font_families: font_families,
            font_weight: font_weight,
            italic: italic,
            face_selection_text: face_selection_text,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "FontMetricsRequest(",
            "fontKey=",
            (self.font_key).to_string(),
            ", ",
            "fontSize=",
            self.font_size,
            ", ",
            "role=",
            self.role.name(),
            ", ",
            "locale=",
            (self.locale).to_string(),
            ", ",
            "fontFamilies=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.font_families).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "fontWeight=",
            crate::runtime::int_text::IntText::int_text(self.font_weight),
            ", ",
            "italic=",
            self.italic,
            ", ",
            "faceSelectionText=",
            (self.face_selection_text).to_string(),
            ")"
        );
    }
}

pub trait FontMetricsResolver: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn FontMetricsResolver>;
    fn resolve(&self, request: FontMetricsRequest) -> Result<RawFontMetrics, TextRangeError>;
}

impl Clone for Box<dyn FontMetricsResolver> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn FontMetricsResolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

#[derive(Clone, PartialEq)]
pub struct StubFontMetricsResolver {
}

impl StubFontMetricsResolver {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn resolve(&self, r: FontMetricsRequest) -> RawFontMetrics {
        let result: RawFontMetrics;
        {
            let _g = r.role;
            let _ = match _g {
    FontRole::CjkText | FontRole::CjkPunctuation => result = RawFontMetrics::new(r.font_size * 1.16f64, r.font_size * 0.288f64, Some(0 as f64), Some(FontMetricSource::RawTables), Some(r.font_size * 0.88f64), Some(r.font_size * 0.12f64)),
    FontRole::LatinText => result = RawFontMetrics::new(r.font_size * 0.8f64, r.font_size * 0.2f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None),
    FontRole::Symbol | FontRole::Emoji | FontRole::Unknown => result = RawFontMetrics::new(r.font_size * 0.9f64, r.font_size * 0.25f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None),
};
        }
        return result;
    }
}

impl FontMetricsResolver for StubFontMetricsResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.font.FontMetrics.StubFontMetricsResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn FontMetricsResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, r: FontMetricsRequest) -> Result<RawFontMetrics, TextRangeError> {
        let result: RawFontMetrics;
        {
            let _g = r.role;
            let _ = match _g {
    FontRole::CjkText | FontRole::CjkPunctuation => result = RawFontMetrics::new(r.font_size * 1.16f64, r.font_size * 0.288f64, Some(0 as f64), Some(FontMetricSource::RawTables), Some(r.font_size * 0.88f64), Some(r.font_size * 0.12f64)),
    FontRole::LatinText => result = RawFontMetrics::new(r.font_size * 0.8f64, r.font_size * 0.2f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None),
    FontRole::Symbol | FontRole::Emoji | FontRole::Unknown => result = RawFontMetrics::new(r.font_size * 0.9f64, r.font_size * 0.25f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None),
};
        }
        return Ok(result);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontMetricsNormalizationInput {
    pub request: FontMetricsRequest,
    pub raw_metrics: RawFontMetrics,
}

impl FontMetricsNormalizationInput {
    pub fn new(request: FontMetricsRequest, raw_metrics: RawFontMetrics) -> Self {
        Self {
            request,
            raw_metrics,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "FontMetricsNormalizationInput(",
            "request=",
            (self.request).clone().to_string(),
            ", ",
            "rawMetrics=",
            (self.raw_metrics).clone().to_string(),
            ")"
        );
    }
}

pub trait FontMetricsNormalizer: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn FontMetricsNormalizer>;
    fn normalize(&self, input: FontMetricsNormalizationInput) -> LayoutFontMetrics;
}

impl Clone for Box<dyn FontMetricsNormalizer> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn FontMetricsNormalizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

#[derive(Clone, PartialEq)]
pub struct ScriptAwareFontMetricsNormalizer {
}

impl ScriptAwareFontMetricsNormalizer {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn normalize(&self, i: FontMetricsNormalizationInput) -> LayoutFontMetrics {
        let result: LayoutFontMetrics;
        {
            let _g = (i.request).clone().role;
            let _ = match _g {
    FontRole::CjkText | FontRole::CjkPunctuation => result = self.normalize_cjk((i).clone()),
    FontRole::LatinText => result = self.normalize_raw((i).clone(), &"roman-raw"),
    FontRole::Symbol | FontRole::Emoji | FontRole::Unknown => result = self.normalize_raw((i).clone(), &"fallback-raw"),
};
        }
        return result;
    }

    fn normalize_cjk(&self, i: FontMetricsNormalizationInput) -> LayoutFontMetrics {
        let m = (i.raw_metrics).clone().clone();
        let t = match &(m.typo_ascent) { Some(__option) => m.typo_descent.is_some(), None => false };
        return LayoutFontMetrics::new(match &(m.typo_ascent) { Some(__option1) => *__option1, None => m.ascent }, match &(m.typo_descent) { Some(__option2) => *__option2, None => m.descent }, 0 as f64 as f64, if t { FontMetricsPolicy::IdeographicBox } else {
FontMetricsPolicy::Raw }, BaselinePolicy::Ideographic, Some(BaselineClass::IdeographicLow), Some(MetricBox::IdeographicEmBox), Some(m.source), Some((format!("{}{}{}{}",
            "ScriptAwareFontMetricsNormalizer:",
            (i.request).clone().role.name(),
            ":",
            (if t { "font-typo-box".to_string() } else { "hhea-fallback-no-os2".to_string() })
        )).to_string()));
    }

    fn normalize_raw(&self, i: FontMetricsNormalizationInput, why: &str) -> LayoutFontMetrics {
        let m = (i.raw_metrics).clone().clone();
        return LayoutFontMetrics::new(m.ascent, m.descent, 0 as f64 as f64, FontMetricsPolicy::Raw, BaselinePolicy::Alphabetic, Some(BaselineClass::Roman), Some(MetricBox::RawFontBox), Some(m.source), Some((format!("{}{}{}{}",
            "ScriptAwareFontMetricsNormalizer:",
            (i.request).clone().role.name(),
            ":",
            why
        )).to_string()));
    }
}

impl FontMetricsNormalizer for ScriptAwareFontMetricsNormalizer {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.font.FontMetrics.ScriptAwareFontMetricsNormalizer"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn FontMetricsNormalizer> {
        Box::new(self.clone())
    }

    fn normalize(&self, i: FontMetricsNormalizationInput) -> LayoutFontMetrics {
        let result: LayoutFontMetrics;
        {
            let _g = (i.request).clone().role;
            let _ = match _g {
    FontRole::CjkText | FontRole::CjkPunctuation => result = self.normalize_cjk((i).clone()),
    FontRole::LatinText => result = self.normalize_raw((i).clone(), &"roman-raw"),
    FontRole::Symbol | FontRole::Emoji | FontRole::Unknown => result = self.normalize_raw((i).clone(), &"fallback-raw"),
};
        }
        return result;
    }
}
