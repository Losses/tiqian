use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
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
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<ParagraphLayoutEngineValidationCoverageSupportRejectFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageSupportRejectFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageSupportRejectFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageSupportRejectFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageSupportRejectFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageSupportRejectFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageSupportRejectFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageSupportRejectFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageSupportRejectFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageSupportRejectFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageSupportRejectFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageSupportRejectFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageSupportRejectFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageSupportRejectFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageSupportRejectFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageSupportRejectFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageSupportRejectFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageSupportRejectFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphLayoutEngineValidationCoverageSupportRejectFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ParagraphLayoutEngineValidationCoverageSupportRejectFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphLayoutEngineValidationCoverageSupportRejectFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        ParagraphLayoutEngineValidationCoverageSupportRejectFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphLayoutEngineValidationCoverageSupportRejectFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ParagraphLayoutEngineValidationCoverageSupportRejectFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct ParagraphLayoutEngineValidationCoverageSupport;

impl ParagraphLayoutEngineValidationCoverageSupport {
    pub fn paragraph_layout_engine_validation_coverage_support_code(n: u32) -> String {
        let mut b_b = String::new();
        b_b += &(if n > 0xFFFF { String::from_utf16(&[0xD800 + (((n) - 0x10000) >> 10) as u16, 0xDC00 + (((n) - 0x10000) & 0x3FF) as u16]).unwrap() } else { String::from_utf16_lossy(&[(n) as u16]) });
        return b_b;
    }

    pub fn paragraph_layout_engine_validation_coverage_support_input(style: Option<ParagraphStyle>, boxes: Option<Vec<InlineBoxSpan>>, objects: Option<Vec<InlineObjectSpan>>, content: Option<TiqianTextContent>) -> Result<LayoutInput, TextRangeError> {
        return Ok(LayoutInput::new(match &(content) { None => TiqianTextContent::new("甲乙", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(__option) => (*__option).clone() }, Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some((match &(style) { None => ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM)), Some(__option3) => (*__option3).clone()
}).clone()), LayoutConstraints::new(100 as f64 as f64, Some(f64::INFINITY), Some(2147483647))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), (boxes).clone(), (objects).clone()));
    }

    pub fn paragraph_layout_engine_validation_coverage_support_obj(range: Option<TextRange>, advance: Option<f64>, ascent: Option<f64>, descent: Option<f64>, leading: Option<InlineObjectBoundaryAdjustment>, trailing: Option<InlineObjectBoundaryAdjustment>) ->
Result<InlineObjectSpan, TextRangeError> {
        return Ok(InlineObjectSpan::new(match &(range) { None => TextRange::new(0u32, 1u32)?, Some(__option10) => (*__option10).clone() }, match &(advance) { None => 10 as f64, Some(__option11) => *__option11 }, match &(ascent) { None => 8 as f64, Some(__option12) => *__option12
}, match &(descent) { None => 2 as f64, Some(__option13) => *__option13 }, Some((match &(leading) { None => InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(false))?, Some(__option14) => (*__option14).clone() }).clone()), Some((match &(trailing) {
None => InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(false))?, Some(__option15) => (*__option15).clone() }).clone()))?);
    }

    pub fn paragraph_layout_engine_validation_coverage_support_reject(bad: LayoutInput, fragment: &str) -> Result<(), ParagraphLayoutEngineValidationCoverageSupportRejectFault> {
        let e = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let bad = (bad).clone(); Arc::new(move || {
        ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).map_err(|e|
IllegalStateException::new(&format!("{:?}", e)))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?).clone()), Some((PunctuationSpacingCompressor::new().map_err(|e| IllegalStateException::new(&format!("{}", e)))?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))),
Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some((DefaultHyphenator::default_hyphenator_default_hyphenator().map_err(|e| IllegalStateException::new(&format!("{:?}", e)))?).clone()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).map_err(|e| IllegalStateException::new(&format!("{:?}", e)))?.layout((bad).clone()).map_err(|e| IllegalStateException::new(&format!("{:?}", e)))?;
        Ok(())
}) }).map_err(|e| ParagraphLayoutEngineValidationCoverageSupportRejectFault::TracedAssertionsAssertFailsWithFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&format!("{}", e), fragment, 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}", e)).to_string())).map_err(|e|
ParagraphLayoutEngineValidationCoverageSupportRejectFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }
}
