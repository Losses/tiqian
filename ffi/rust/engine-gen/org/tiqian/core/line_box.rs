use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_debug_info::LineDebugInfo;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use std::fmt::Write;


#[derive(Clone, PartialEq)]
pub struct LineBox {
    pub range: TextRange,
    pub cluster_range: IntRange,
    pub baseline: f64,
    pub top: f64,
    pub bottom: f64,
    pub natural_width: f64,
    pub adjusted_width: f64,
    pub visual_width: f64,
    pub hanging_punctuation_advance: f64,
    pub indent: f64,
    pub end_reason: LineEndReason,
    pub hyphen_advance: f64,
    pub hyphen_glyphs: Vec<Glyph>,
    pub debug: LineDebugInfo,
}

impl LineBox {
    pub fn new(range: TextRange, cluster_range: IntRange, baseline: f64, top: f64, bottom: f64, natural_width: f64, adjusted_width: f64, visual_width: f64, hanging_punctuation_advance: Option<f64>, indent: Option<f64>, end_reason: Option<LineEndReason>, hyphen_advance:
Option<f64>, hyphen_glyphs: Option<Vec<Glyph>>, debug: LineDebugInfo) -> Self {
        let hanging_punctuation_advance = hanging_punctuation_advance.unwrap_or_else(|| 0.0);
        let indent = indent.unwrap_or_else(|| 0.0);
        let end_reason = end_reason.unwrap_or_else(|| LineEndReason::ParagraphEnd);
        let hyphen_advance = hyphen_advance.unwrap_or_else(|| 0.0);
        let hyphen_glyphs = hyphen_glyphs.unwrap_or_else(|| vec![]);
        Self {
            range,
            cluster_range,
            baseline,
            top,
            bottom,
            natural_width,
            adjusted_width,
            visual_width,
            hanging_punctuation_advance: hanging_punctuation_advance,
            indent: indent,
            end_reason: end_reason,
            hyphen_advance: hyphen_advance,
            hyphen_glyphs: hyphen_glyphs,
            debug,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LineBox(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "clusterRange=",
            (self.cluster_range).clone().to_string(),
            ", ",
            "baseline=",
            self.baseline,
            ", ",
            "top=",
            self.top,
            ", ",
            "bottom=",
            self.bottom,
            ", ",
            "naturalWidth=",
            self.natural_width,
            ", ",
            "adjustedWidth=",
            self.adjusted_width,
            ", ",
            "visualWidth=",
            self.visual_width,
            ", ",
            "hangingPunctuationAdvance=",
            self.hanging_punctuation_advance,
            ", ",
            "indent=",
            self.indent,
            ", ",
            "endReason=",
            self.end_reason.name(),
            ", ",
            "hyphenAdvance=",
            self.hyphen_advance,
            ", ",
            "hyphenGlyphs=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.hyphen_glyphs).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "debug=",
            (self.debug).clone().to_string(),
            ")"
        );
    }
}
