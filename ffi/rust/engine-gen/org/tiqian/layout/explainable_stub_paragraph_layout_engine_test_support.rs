use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::source_interaction_boundaries::SourceInteractionBoundaries;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Clone, Copy)]
pub struct ExplainableStubParagraphLayoutEngineTestSupport;

impl ExplainableStubParagraphLayoutEngineTestSupport {
    pub fn explainable_stub_paragraph_layout_engine_test_support_render_nullable_bounds(v: Option<Rect>) -> UString {
        return match &(v) { None => UString::from("null"), Some(__option) => ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_bounds(((*__option).clone()).clone()).to_ustring() };
    }

    pub fn explainable_stub_paragraph_layout_engine_test_support_render_bounds(v: Rect) -> UString {
        return UString::from(format!("{}", v.to_string()).as_str());
    }

    pub fn explainable_stub_paragraph_layout_engine_test_support_start(name: &UStr) -> TestTraceRecorder {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,120,112,108,97,105,110,97,98,108,101,83,116,117,98,80,97,114,97,103,114,97,112,104,76,97,121,111,117,116,69,110,103,105,110,101,84,101,115,116])));
        t.section(name);
        return t;
    }

    pub fn explainable_stub_paragraph_layout_engine_test_support_text_length(s: &UStr) -> u32 {
        return u_string::unit_count(&(s));
    }

    pub fn explainable_stub_paragraph_layout_engine_test_support_grapheme_boundaries(s: &UStr) -> Result<Vec<u32>, TextRangeError> {
        return Ok(SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(s, TextRange::new(0u32, u_string::unit_count(&(s)))?));
    }

    pub fn explainable_stub_paragraph_layout_engine_test_support_render_ranges(a: &Vec<TextRange>) -> UString {
        let mut s = UString::from("[").to_ustring();
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                s += &(UString::from(", "));
            }
            s += &((a[usize::try_from(i).unwrap_or(0)]).clone().to_string());
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += s.as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn explainable_stub_paragraph_layout_engine_test_support_render_pairs(a: &Vec<UString>, b: &Vec<UString>) -> UString {
        let mut s = UString::from("[").to_ustring();
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                s += &(UString::from(", "));
            }
            s += &({ let mut __s = UString::new(); __s += &(UString::from("(")); __s += (a[usize::try_from(i).unwrap_or(0)]).clone().as_ustr(); __s += &(UString::from(", ")); __s += (b[usize::try_from(i).unwrap_or(0)]).clone().as_ustr(); __s += &(UString::from(")")); __s });
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += s.as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn explainable_stub_paragraph_layout_engine_test_support_input(text: &UStr, width: f64, style: Option<ParagraphStyle>, _shaper: Option<Arc<Mutex<dyn ITextShaper>>>, text_style: Option<TextStyle>) -> Result<LayoutInput, TextRangeError> {
        return Ok(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), (text_style).clone(), Some((match &(style) { None => ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM)), Some(__option1) => (*__option1).clone() }).clone()), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
    }

    pub fn explainable_stub_paragraph_layout_engine_test_support_engine(shaper: Option<Arc<Mutex<dyn ITextShaper>>>, breaker: Option<Box<dyn LineBreaker>>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), breaker, Some(Justifier::new(Some(0.5), Some(0.25))), shaper, Some(Box::new(NoHyphenator::new())), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn explainable_stub_paragraph_layout_engine_test_support_render_strings(a: &Vec<UString>) -> UString {
        let mut s = UString::from("[").to_ustring();
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                s += &(UString::from(", "));
            }
            s += &({ let mut __s = UString::new(); __s += &(UString::from("'")); __s += (a[usize::try_from(i).unwrap_or(0)]).clone().as_ustr(); __s += &(UString::from("'")); __s });
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += s.as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }
}

#[derive(Clone, PartialEq)]
pub struct EmptyTextShaper {
}

impl EmptyTextShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, _input: ShapingInput) -> ShapingResult {
        return ShapingResult::new(vec![].to_vec(), vec![].to_vec(), Some(vec![]));
    }
}

impl ITextShaper for EmptyTextShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTestSupport.EmptyTextShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, _input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        return Ok(ShapingResult::new(vec![].to_vec(), vec![].to_vec(), Some(vec![])));
    }
}

#[derive(Clone, PartialEq)]
pub struct FixedBoundsTextShaper {
}

impl FixedBoundsTextShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, input: ShapingInput) -> ShapingResult {
        let text = u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes()));
        let c = Cluster::new((input.range).clone(), text.as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), { let v: u32 = if u32::wrapping_sub((input.range).clone().end, (input.range).clone().start) == 0 { 0 } else { 20 }; i32::from_ne_bytes(v.to_ne_bytes()) } as f64 as f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let g = Glyph::new(42u32, (input.range).clone(), 20 as f64 as f64, Some(0.0), Some(0.0), None, Some(Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 12 as f64 as f64, 2 as f64 as f64)), None, None);
        return ShapingResult::new(vec![(c).clone()].to_vec(), vec![
    (GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), vec![(g).clone()].to_vec(), 20 as f64 as f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![]));
    }
}

impl ITextShaper for FixedBoundsTextShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTestSupport.FixedBoundsTextShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let text = u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes()));
        let c = Cluster::new((input.range).clone(), text.as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), { let v: u32 = if u32::wrapping_sub((input.range).clone().end, (input.range).clone().start) == 0 { 0 } else { 20 }; i32::from_ne_bytes(v.to_ne_bytes()) } as f64 as f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let g = Glyph::new(42u32, (input.range).clone(), 20 as f64 as f64, Some(0.0), Some(0.0), None, Some(Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 12 as f64 as f64, 2 as f64 as f64)), None, None);
        return Ok(ShapingResult::new(vec![(c).clone()].to_vec(), vec![
    (GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), vec![(g).clone()].to_vec(), 20 as f64 as f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}
