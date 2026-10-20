use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum R3GeometryTailCoverageTestSupportLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for R3GeometryTailCoverageTestSupportLayoutFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            R3GeometryTailCoverageTestSupportLayoutFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            R3GeometryTailCoverageTestSupportLayoutFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<R3GeometryTailCoverageTestSupportLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: R3GeometryTailCoverageTestSupportLayoutFault) -> Self {
        match value {
            R3GeometryTailCoverageTestSupportLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestSupportLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: R3GeometryTailCoverageTestSupportLayoutFault) -> Self {
        match value {
            R3GeometryTailCoverageTestSupportLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for R3GeometryTailCoverageTestSupportLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        R3GeometryTailCoverageTestSupportLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for R3GeometryTailCoverageTestSupportLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        R3GeometryTailCoverageTestSupportLayoutFault::UStringFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct R3GeometryTailCoverageTestSupport;

impl R3GeometryTailCoverageTestSupport {
    pub fn r3_geometry_tail_coverage_test_support_layout(text: &UStr, max_width: Option<f64>, max_lines: Option<u32>, spans: Option<Vec<TextSpan>>, ruby_spans: Option<Vec<RubySpan>>, shaper: Option<Arc<Mutex<dyn ITextShaper>>>) -> Result<LayoutResult,
ParagraphLayoutEngineNewFault> {
        let width = match &(max_width) { None => 320.0f64, Some(__option) => *__option };
        let lines = match &(max_lines) { None => 2147483647, Some(__option1) => *__option1 };
        let ss = match &(spans) { None => vec![], Some(__option2) => (*__option2).clone() };
        let rr = match &(ruby_spans) { None => vec![], Some(__option3) => (*__option3).clone() };
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), shaper, Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?;
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(text, Some((ss).clone()), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(lines)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((rr).clone()), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn r3_geometry_tail_coverage_test_support_centered_ink_shaper() -> Arc<Mutex<dyn ITextShaper>> {
        return Arc::new(Mutex::new(CenteredInkShaper::new()));
    }
}

#[derive(Clone, PartialEq)]
pub struct CenteredInkShaper {
}

impl CenteredInkShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = Arc::new(Mutex::new(ExplainableStubTextShaper::new())).lock().unwrap().shape((input).clone())?;
        let mut runs: Vec<GlyphRun> = vec![];
        for i in 0..match u32::try_from(res.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (res.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone();
            let mut glyphs: Vec<Glyph> = vec![];
            for j in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(j).unwrap_or(0)]).clone();
                glyphs.push(Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some(Rect::new(4.0f64, 2.0f64, 12.0f64, 10.0f64)), g.halt_advance, g.halt_placement_x));
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), glyphs.to_vec(), run.advance, Some(vec![])));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), runs.to_vec(), Some((res.decisions).clone())));
    }
}

impl ITextShaper for CenteredInkShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.R3GeometryTailCoverageTestSupport.CenteredInkShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = Arc::new(Mutex::new(ExplainableStubTextShaper::new())).lock().unwrap().shape((input).clone())?;
        let mut runs: Vec<GlyphRun> = vec![];
        for i in 0..match u32::try_from(res.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (res.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone();
            let mut glyphs: Vec<Glyph> = vec![];
            for j in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(j).unwrap_or(0)]).clone();
                glyphs.push(Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some(Rect::new(4.0f64, 2.0f64, 12.0f64, 10.0f64)), g.halt_advance, g.halt_placement_x));
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), glyphs.to_vec(), run.advance, Some(vec![])));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), runs.to_vec(), Some((res.decisions).clone())));
    }
}
