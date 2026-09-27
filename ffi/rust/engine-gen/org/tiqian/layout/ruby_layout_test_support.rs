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
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
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
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;


#[derive(Debug, Clone, PartialEq)]
pub enum RubyLayoutTestSupportLayoutContradictoryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}

impl From<RubyLayoutTestSupportLayoutContradictoryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RubyLayoutTestSupportLayoutContradictoryFault) -> Self {
        match value {
            RubyLayoutTestSupportLayoutContradictoryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestSupportLayoutContradictoryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: RubyLayoutTestSupportLayoutContradictoryFault) -> Self {
        match value {
            RubyLayoutTestSupportLayoutContradictoryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RubyLayoutTestSupportLayoutContradictoryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RubyLayoutTestSupportLayoutContradictoryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for RubyLayoutTestSupportLayoutContradictoryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        RubyLayoutTestSupportLayoutContradictoryFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RubyLayoutTestSupportEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}

impl From<RubyLayoutTestSupportEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RubyLayoutTestSupportEngineFault) -> Self {
        match value {
            RubyLayoutTestSupportEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestSupportEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: RubyLayoutTestSupportEngineFault) -> Self {
        match value {
            RubyLayoutTestSupportEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RubyLayoutTestSupportEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RubyLayoutTestSupportEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for RubyLayoutTestSupportEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        RubyLayoutTestSupportEngineFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RubyLayoutTestSupportTotalWidthFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<RubyLayoutTestSupportTotalWidthFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RubyLayoutTestSupportTotalWidthFault) -> Self {
        match value {
            RubyLayoutTestSupportTotalWidthFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestSupportTotalWidthFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: RubyLayoutTestSupportTotalWidthFault) -> Self {
        match value {
            RubyLayoutTestSupportTotalWidthFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestSupportTotalWidthFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: RubyLayoutTestSupportTotalWidthFault) -> Self {
        match value {
            RubyLayoutTestSupportTotalWidthFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RubyLayoutTestSupportTotalWidthFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RubyLayoutTestSupportTotalWidthFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for RubyLayoutTestSupportTotalWidthFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        RubyLayoutTestSupportTotalWidthFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for RubyLayoutTestSupportTotalWidthFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        RubyLayoutTestSupportTotalWidthFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct RubyLayoutTestSupport;

impl RubyLayoutTestSupport {
    pub fn ruby_layout_test_support_engine() -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?);
    }

    pub fn ruby_layout_test_support_input(ruby: &Vec<RubySpan>) -> Result<LayoutInput, TextRangeError> {
        return Ok(LayoutInput::new(TiqianTextContent::new("中文排版", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some(Ic(0 as f64)), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(400.0f64, Some(f64::INFINITY), Some(2147483647))?,
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((ruby).clone()), Some(vec![]), Some(vec![])));
    }

    pub fn ruby_layout_test_support_layout(ruby: &Vec<RubySpan>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(RubyLayoutTestSupport::ruby_layout_test_support_engine()?.layout(RubyLayoutTestSupport::ruby_layout_test_support_input(&ruby).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?).map_err(|e|
ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn ruby_layout_test_support_layout_eight(ruby: &Vec<RubySpan>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(RubyLayoutTestSupport::ruby_layout_test_support_engine()?.layout(LayoutInput::new(TiqianTextContent::new("甲乙丙丁戊己庚辛", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some(Ic(0 as f64)), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(64.0f64,
Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((ruby).clone()), Some(vec![]),
Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn ruby_layout_test_support_layout_twelve(ruby: &Vec<RubySpan>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(RubyLayoutTestSupport::ruby_layout_test_support_engine()?.layout(LayoutInput::new(TiqianTextContent::new("甲乙丙丁戊己庚辛壬癸子丑", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), Some(18.0f64), Some(Ic(0 as f64)), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(64.0f64,
Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((ruby).clone()), Some(vec![]),
Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn ruby_layout_test_support_layout_uniform(ruby: &Vec<RubySpan>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(RubyLayoutTestSupport::ruby_layout_test_support_engine()?.layout(LayoutInput::new(TiqianTextContent::new("甲乙丙丁戊己庚辛壬癸子丑", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), Some(18.0f64), Some(Ic(0 as f64)), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::UniformParagraph), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(64.0f64,
Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((ruby).clone()), Some(vec![]),
Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn ruby_layout_test_support_layout_contradictory(reading: &str) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let mut e = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ContradictoryInkShaper::new())), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?;
        return Ok(e.layout(LayoutInput::new(TiqianTextContent::new("甲乙丙丁", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), Some(18.0f64), Some(Ic(0 as f64)), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(64.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e|
ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, reading, Some(vec![]), RubyKind::Pinyin, None)).clone(),
]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn ruby_layout_test_support_total_width(texts: &Vec<String>) -> Result<f64, RubyLayoutTestSupportTotalWidthFault> {
        let mut spans: Vec<RubySpan> = vec![];
        for i in 0..match u32::try_from(texts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            spans.push(RubySpan::new(TextRange::new(i, u32::wrapping_add(i, 1)).map_err(|e| RubyLayoutTestSupportTotalWidthFault::TextRangeErrorFault(e))?, (texts[usize::try_from(i).unwrap_or(0)]).clone().as_str(), Some(vec![]), RubyKind::Pinyin, None));
        }
        let r = RubyLayoutTestSupport::ruby_layout_test_support_engine().map_err(|e| RubyLayoutTestSupportTotalWidthFault::ParagraphLayoutEngineNewFaultFault(e))?.layout(LayoutInput::new(TiqianTextContent::new("中文排版", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])),
Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some(Ic(0 as f64)), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(4000.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| RubyLayoutTestSupportTotalWidthFault::TextRangeErrorFault(e))?,
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((spans).clone()), Some(vec![]), Some(vec![]))).map_err(|e|
RubyLayoutTestSupportTotalWidthFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(e))?;
        let mut total = 0.0f64;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            total += r.clusters[usize::try_from(i).unwrap_or(0)].advance;
        }
        return Ok(total);
    }

    pub fn ruby_layout_test_support_assert_float_list_equals(expected: &Vec<f64>, actual: &[f64]) -> Result<(), TracedAssertionsFailFault> {
        let mut e: Vec<u32> = vec![];
        for i in 0..match u32::try_from(expected.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            e.push(u32::from_ne_bytes((match f64::from(expected[usize::try_from(i).unwrap_or(0)]) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 -
v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e =>
u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }).to_ne_bytes()));
        }
        let mut a: Vec<u32> = vec![];
        for i in 0..match u32::try_from(actual.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            a.push(u32::from_ne_bytes((match f64::from(actual[usize::try_from(i).unwrap_or(0)]) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 -
v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e =>
u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }).to_ne_bytes()));
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&e, &a, None)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct ContradictoryInkShaper {
    pub(crate) delegate: ExplainableStubTextShaper,
}

impl ContradictoryInkShaper {
    pub fn new() -> Self {
        Self {
            delegate: ExplainableStubTextShaper::new(),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let r = self.delegate.shape((input).clone())?;
        let bounds = if input.display_text.to_string() == "pg" { Rect::new(0.0f64, -100.0f64, 16.0f64, 100.0f64) } else { Rect::new(0.0f64, -1.0f64, 16.0f64, 1.0f64) };
        let mut runs: Vec<GlyphRun> = vec![];
        for i in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (r.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone();
            let mut gs: Vec<Glyph> = vec![];
            for j in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(j).unwrap_or(0)]).clone();
                gs.push(Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some((bounds).clone()), g.halt_advance, g.halt_placement_x));
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_string().as_str(), gs.to_vec(), run.advance, Some(vec![])));
        }
        let mut ds: Vec<ShapingDecisionInfo> = vec![];
        for i in 0..match u32::try_from(r.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (r.decisions[usize::try_from(i).unwrap_or(0)]).clone();
            ds.push(ShapingDecisionInfo::new((d.range).clone(), (d.source_text).to_string().as_str(), (d.display_text).to_string().as_str(), (d.font_key).to_string().as_str(), d.glyph_count, d.advance, (d.source).to_string().as_str(), (d.reason).to_string().as_str(), Some(0),
Some(d.missing_glyphs), d.resolved_face.clone(), d.script.clone(), d.language.clone(), d.strategy.clone(), d.feature_evidence.clone(), d.capability_issue.clone()));
        }
        return Ok(ShapingResult::new(r.clusters.to_vec(), runs.to_vec(), Some((ds).clone())));
    }
}

impl ITextShaper for ContradictoryInkShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.RubyLayoutTestSupport.ContradictoryInkShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let r = self.delegate.shape((input).clone())?;
        let bounds = if input.display_text.to_string() == "pg" { Rect::new(0.0f64, -100.0f64, 16.0f64, 100.0f64) } else { Rect::new(0.0f64, -1.0f64, 16.0f64, 1.0f64) };
        let mut runs: Vec<GlyphRun> = vec![];
        for i in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (r.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone();
            let mut gs: Vec<Glyph> = vec![];
            for j in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(j).unwrap_or(0)]).clone();
                gs.push(Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some((bounds).clone()), g.halt_advance, g.halt_placement_x));
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_string().as_str(), gs.to_vec(), run.advance, Some(vec![])));
        }
        let mut ds: Vec<ShapingDecisionInfo> = vec![];
        for i in 0..match u32::try_from(r.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (r.decisions[usize::try_from(i).unwrap_or(0)]).clone();
            ds.push(ShapingDecisionInfo::new((d.range).clone(), (d.source_text).to_string().as_str(), (d.display_text).to_string().as_str(), (d.font_key).to_string().as_str(), d.glyph_count, d.advance, (d.source).to_string().as_str(), (d.reason).to_string().as_str(), Some(0),
Some(d.missing_glyphs), d.resolved_face.clone(), d.script.clone(), d.language.clone(), d.strategy.clone(), d.feature_evidence.clone(), d.capability_issue.clone()));
        }
        return Ok(ShapingResult::new(r.clusters.to_vec(), runs.to_vec(), Some((ds).clone())));
    }
}
