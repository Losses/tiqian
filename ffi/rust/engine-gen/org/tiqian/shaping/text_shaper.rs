use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::runtime::u_string;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum TextShaperShapeFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<TextShaperShapeFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: TextShaperShapeFault) -> Self {
        match value {
            TextShaperShapeFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperShapeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextShaperShapeFault) -> Self {
        match value {
            TextShaperShapeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperShapeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextShaperShapeFault) -> Self {
        match value {
            TextShaperShapeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperShapeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextShaperShapeFault) -> Self {
        match value {
            TextShaperShapeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for TextShaperShapeFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        TextShaperShapeFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextShaperShapeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextShaperShapeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextShaperShapeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextShaperShapeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextShaperShapeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextShaperShapeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapingInput {
    pub text: String,
    pub range: TextRange,
    pub style: TextStyle,
    pub font_decision: FontDecision,
    pub display_text: String,
    pub open_type_features: Vec<String>,
}

impl ShapingInput {
    pub fn new(text: &str, range: TextRange, style: TextStyle, font_decision: FontDecision, display_text: Option<String>, open_type_features: Option<Vec<String>>) -> Self {
        let display_text = display_text.unwrap_or_else(|| u_string::substring(text, i32::from_ne_bytes((range.start).to_ne_bytes()), i32::from_ne_bytes((range.end).to_ne_bytes())));
        let open_type_features = open_type_features.unwrap_or_else(|| vec![]);
        Self {
            text: text.to_string(),
            range,
            style,
            font_decision,
            display_text: display_text,
            open_type_features: open_type_features,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "ShapingInput(",
            "text=",
            (self.text).to_string(),
            ", ",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "style=",
            (self.style).clone().to_string(),
            ", ",
            "fontDecision=",
            (self.font_decision).clone().to_string(),
            ", ",
            "displayText=",
            (self.display_text).to_string(),
            ", ",
            "openTypeFeatures=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.open_type_features).clone();
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
            ")"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapingResult {
    pub clusters: Vec<Cluster>,
    pub glyph_runs: Vec<GlyphRun>,
    pub decisions: Vec<ShapingDecisionInfo>,
}

impl ShapingResult {
    pub fn new(clusters: Vec<Cluster>, glyph_runs: Vec<GlyphRun>, decisions: Option<Vec<ShapingDecisionInfo>>) -> Self {
        let decisions = decisions.unwrap_or_else(|| vec![]);
        Self {
            clusters,
            glyph_runs,
            decisions: decisions,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "ShapingResult(",
            "clusters=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.clusters).clone();
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
            "glyphRuns=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.glyph_runs).clone();
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
            "decisions=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decisions).clone();
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
            ")"
        );
    }
}

#[derive(Clone, Copy)]
pub struct TextShaper;

impl TextShaper {
    pub const TEXT_SHAPER_UNVERIFIED_DISPLAY_SUBSTITUTION_COVERAGE_ISSUE: &str = "UnverifiedDisplaySubstitutionCoverage";
    pub const TEXT_SHAPER_PLATFORM_MULTI_FACE_STRING_DRAW_ISSUE: &str = "PlatformMultiFaceStringDraw";
}

pub trait ITextShaper: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn ITextShaper>;
    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault>;
}

impl Clone for Box<dyn ITextShaper> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn ITextShaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

#[derive(Clone, PartialEq)]
pub struct ExplainableStubTextShaper {
}

impl ExplainableStubTextShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let source_text = u_string::substring(&(input.text).to_string(), i32::from_ne_bytes(((input.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((input.range).clone().end).to_ne_bytes()));
        let mut glyph_count = ExplainableStubTextShaper::explainable_stub_text_shaper_code_point_count((input.display_text).to_string().as_str());
        if i32::from_ne_bytes((glyph_count).to_ne_bytes()) < (1) {
            glyph_count = 1u32;
        }
        let advance = (input.style).clone().font_size * ExplainableStubTextShaper::explainable_stub_text_shaper_nominal_advance_em(source_text.as_str(), (input.display_text).to_string().as_str());
        let cluster = Cluster::new((input.range).clone(), source_text.as_str(), (((input.font_decision).clone().candidate).clone().key).to_string().as_str(), advance, Some((input.display_text).to_string()), Some(0.0), Some(0.0), Some(0.0));
        let glyph_advance = advance / format!("{}", (i32::from_ne_bytes((glyph_count).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0);
        let mut glyphs: Vec<Glyph> = vec![];
        let mut glyph_id = 0u32;
        while (i32::from_ne_bytes((glyph_id).to_ne_bytes())) < (i32::from_ne_bytes((glyph_count).to_ne_bytes())) {
            glyphs.push(Glyph::new(glyph_id, (input.range).clone(), glyph_advance, Some(glyph_advance * format!("{}", (i32::from_ne_bytes((glyph_id).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0)), Some(0.0), None, None, None, None));
            glyph_id = u32::wrapping_add(glyph_id, 1);
        }
        let run = GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_string().as_str(), glyphs.to_vec(), advance, Some(vec![]));
        let decision = ShapingDecisionInfo::new((input.range).clone(), source_text.as_str(), (input.display_text).to_string().as_str(), (((input.font_decision).clone().candidate).clone().key).to_string().as_str(), glyph_count, advance,
ShapingSource::Stub.name().to_string().as_str(), "ExplainableStubTextShaper:nominal-em-advance", Some(glyph_count), Some(0), None, None, None, None, None, None);
        return Ok(ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![(run).clone()].to_vec(), Some(vec![(decision).clone()])));
    }

    pub(crate) fn explainable_stub_text_shaper_nominal_advance_em(source: &str, display_text: &str) -> f64 {
    let __units = u_string::units(&source);
    let __count = u_string::unit_count(&source);
        if source == "⸺" || display_text == "⸺" {
            return 2.0f64;
        }
        if i32::from_ne_bytes((__count).to_ne_bytes()) > (0) {
            let mut all_spaces = true;
            let mut i = 0u32;
            let __units1 = u_string::units(&source);
            let __count1 = u_string::unit_count(&source);
            while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
                if !(u_string::unit_at_from(&__units1, i).as_ref().map_or(false, |v| v == &(32))) {
                    all_spaces = false;
                }
                i = u32::wrapping_add(i, 1);
            }
            if all_spaces {
                return 0.5f64 * format!("{}", (i32::from_ne_bytes((__count).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0);
            }
        }
        let a = ExplainableStubTextShaper::explainable_stub_text_shaper_code_point_count(source);
        let b = ExplainableStubTextShaper::explainable_stub_text_shaper_code_point_count(display_text);
        return { let v: u32 = if i32::from_ne_bytes((a).to_ne_bytes()) > (i32::from_ne_bytes((b).to_ne_bytes())) { a } else { b }; i32::from_ne_bytes(v.to_ne_bytes()) } as f64;
    }

    pub(crate) fn explainable_stub_text_shaper_code_point_count(value: &str) -> u32 {
    let __units2 = u_string::units(&value);
    let __count2 = u_string::unit_count(&value);
        let mut count = 0u32;
        let mut index = 0u32;
        let __units3 = u_string::units(&value);
        let __count3 = u_string::unit_count(&value);
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count2).to_ne_bytes())) {
            let code = u_string::unit_at_from(&__units3, index).unwrap_or(0);
            if i32::from_ne_bytes((code).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes((code).to_ne_bytes())) <= 56319 && (i32::from_ne_bytes((u32::wrapping_add(index, 1)).to_ne_bytes())) < (i32::from_ne_bytes((__count2).to_ne_bytes())) {
                let next = u_string::unit_at_from(&__units2, u32::wrapping_add(index, 1)).unwrap_or(0);
                if i32::from_ne_bytes((next).to_ne_bytes()) >= 56320 && (i32::from_ne_bytes((next).to_ne_bytes())) <= 57343 {
                    index = u32::wrapping_add(index, 2);
                } else {
                    index = u32::wrapping_add(index, 1);
                }
            } else {
                index = u32::wrapping_add(index, 1);
            }
            count = u32::wrapping_add(count, 1);
        }
        return count;
    }
}

impl ITextShaper for ExplainableStubTextShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.shaping.TextShaper.ExplainableStubTextShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let source_text = u_string::substring(&(input.text).to_string(), i32::from_ne_bytes(((input.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((input.range).clone().end).to_ne_bytes()));
        let mut glyph_count = ExplainableStubTextShaper::explainable_stub_text_shaper_code_point_count((input.display_text).to_string().as_str());
        if i32::from_ne_bytes((glyph_count).to_ne_bytes()) < (1) {
            glyph_count = 1u32;
        }
        let advance = (input.style).clone().font_size * ExplainableStubTextShaper::explainable_stub_text_shaper_nominal_advance_em(source_text.as_str(), (input.display_text).to_string().as_str());
        let cluster = Cluster::new((input.range).clone(), source_text.as_str(), (((input.font_decision).clone().candidate).clone().key).to_string().as_str(), advance, Some((input.display_text).to_string()), Some(0.0), Some(0.0), Some(0.0));
        let glyph_advance = advance / format!("{}", (i32::from_ne_bytes((glyph_count).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0);
        let mut glyphs: Vec<Glyph> = vec![];
        let mut glyph_id = 0u32;
        while (i32::from_ne_bytes((glyph_id).to_ne_bytes())) < (i32::from_ne_bytes((glyph_count).to_ne_bytes())) {
            glyphs.push(Glyph::new(glyph_id, (input.range).clone(), glyph_advance, Some(glyph_advance * format!("{}", (i32::from_ne_bytes((glyph_id).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0)), Some(0.0), None, None, None, None));
            glyph_id = u32::wrapping_add(glyph_id, 1);
        }
        let run = GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_string().as_str(), glyphs.to_vec(), advance, Some(vec![]));
        let decision = ShapingDecisionInfo::new((input.range).clone(), source_text.as_str(), (input.display_text).to_string().as_str(), (((input.font_decision).clone().candidate).clone().key).to_string().as_str(), glyph_count, advance,
ShapingSource::Stub.name().to_string().as_str(), "ExplainableStubTextShaper:nominal-em-advance", Some(glyph_count), Some(0), None, None, None, None, None, None);
        return Ok(ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![(run).clone()].to_vec(), Some(vec![(decision).clone()])));
    }
}

#[derive(Clone, PartialEq)]
pub struct UnimplementedTextShaper {
}

impl UnimplementedTextShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, _input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        return Err(TextShaperShapeFault::IllegalStateExceptionFault(IllegalStateException::new("Text shaping is platform-specific and has not been wired for this target yet.")));
    }
}

impl ITextShaper for UnimplementedTextShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.shaping.TextShaper.UnimplementedTextShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, _input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        return Err(TextShaperShapeFault::IllegalStateExceptionFault(IllegalStateException::new("Text shaping is platform-specific and has not been wired for this target yet.")));
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapingSource {
    Stub,
    JvmAwt,
    AndroidPaint,
    Skia,
    HarfBuzz,
    CoreText,
}

pub fn compare_shaping_source(a: &ShapingSource, b: &ShapingSource) -> i32 {
    if a == b { return 0; }
    fn rank(v: &ShapingSource) -> i32 {
        match v {
            ShapingSource::Stub => 0,
            ShapingSource::JvmAwt => 1,
            ShapingSource::AndroidPaint => 2,
            ShapingSource::Skia => 3,
            ShapingSource::HarfBuzz => 4,
            ShapingSource::CoreText => 5,
        }
    }
    rank(a) - rank(b)
}

impl ShapingSource {
    pub fn to_string(&self) -> String {
        match self {
            ShapingSource::Stub => "Stub".to_string(),
            ShapingSource::JvmAwt => "JvmAwt".to_string(),
            ShapingSource::AndroidPaint => "AndroidPaint".to_string(),
            ShapingSource::Skia => "Skia".to_string(),
            ShapingSource::HarfBuzz => "HarfBuzz".to_string(),
            ShapingSource::CoreText => "CoreText".to_string(),
        }
    }
}

impl ShapingSource {
    pub const ALL: [ShapingSource; 6] = [ShapingSource::Stub, ShapingSource::JvmAwt, ShapingSource::AndroidPaint, ShapingSource::Skia, ShapingSource::HarfBuzz, ShapingSource::CoreText];
    pub fn name(&self) -> &'static str {
        match self {
            ShapingSource::Stub => "Stub",
            ShapingSource::JvmAwt => "JvmAwt",
            ShapingSource::AndroidPaint => "AndroidPaint",
            ShapingSource::Skia => "Skia",
            ShapingSource::HarfBuzz => "HarfBuzz",
            ShapingSource::CoreText => "CoreText",
        }
    }
    pub fn from_name(name: &str) -> Option<ShapingSource> {
        match name {
            "Stub" => Some(ShapingSource::Stub),
            "JvmAwt" => Some(ShapingSource::JvmAwt),
            "AndroidPaint" => Some(ShapingSource::AndroidPaint),
            "Skia" => Some(ShapingSource::Skia),
            "HarfBuzz" => Some(ShapingSource::HarfBuzz),
            "CoreText" => Some(ShapingSource::CoreText),
            _ => None,
        }
    }
}
