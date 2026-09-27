use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphFns;
use crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphRenderEvidenceTestSupportLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}

impl From<PreparedParagraphRenderEvidenceTestSupportLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphRenderEvidenceTestSupportLayoutFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestSupportLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestSupportLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphRenderEvidenceTestSupportLayoutFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestSupportLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphRenderEvidenceTestSupportLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphRenderEvidenceTestSupportLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphRenderEvidenceTestSupportLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphRenderEvidenceTestSupportLayoutFault::UStringFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct PreparedParagraphRenderEvidenceTestSupport;

impl PreparedParagraphRenderEvidenceTestSupport {
    pub fn prepared_paragraph_render_evidence_test_support_layout(input: LayoutInput) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()),
Some(QuotePairAnalyzer::new()), Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Justifier::new(Some(0.5), Some(0.25))),
Some(Box::new(ExplainableStubTextShaper::new())), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?.layout((input).clone()).map_err(|e|
ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn prepared_paragraph_render_evidence_test_support_evidence(r: LayoutResult) -> Result<String, PreparedParagraphToPreparedParagraphJsonFault> {
        return Ok(PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), true)?);
    }

    pub fn prepared_paragraph_render_evidence_test_support_plain(r: LayoutResult) -> Result<String, PreparedParagraphToPreparedParagraphJsonFault> {
        return Ok(PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), false)?);
    }

    pub fn prepared_paragraph_render_evidence_test_support_contains(s: &str, x: &str, msg: &str) -> Result<(), TracedAssertionsFailFault> {
        if u32::from_ne_bytes((u_string::find_from(&s, x, 0)).to_ne_bytes()) > 2147483647 {
            let _ = TracedAssertions::traced_assertions_fail(Some((msg).to_string()), None)?;
        }
        Ok(())
    }
}
