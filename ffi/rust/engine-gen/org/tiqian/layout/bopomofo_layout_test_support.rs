use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
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


#[derive(Debug, Clone, PartialEq)]
pub enum BopomofoLayoutTestSupportLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}

impl From<BopomofoLayoutTestSupportLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BopomofoLayoutTestSupportLayoutFault) -> Self {
        match value {
            BopomofoLayoutTestSupportLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestSupportLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BopomofoLayoutTestSupportLayoutFault) -> Self {
        match value {
            BopomofoLayoutTestSupportLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BopomofoLayoutTestSupportLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BopomofoLayoutTestSupportLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BopomofoLayoutTestSupportLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BopomofoLayoutTestSupportLayoutFault::UStringFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct BopomofoLayoutTestSupport;

impl BopomofoLayoutTestSupport {
    pub fn bopomofo_layout_test_support_layout(bopomofo: &Vec<RubySpan>, spans: Option<Vec<TextSpan>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?.layout(LayoutInput::new(TiqianTextContent::new("中文", (spans).clone(), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false),
Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(4000.0f64,
Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((bopomofo).clone()), Some(vec![]),
Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn bopomofo_layout_test_support_plain() -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(BopomofoLayoutTestSupport::bopomofo_layout_test_support_layout(&vec![], None)?);
    }
}
