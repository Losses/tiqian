#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::inline_object_preferred_stretch_kind::InlineObjectPreferredStretchKind;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::justification_allocation_info::JustificationAllocationInfo;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_queries::LayoutQueries;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::positioned_cluster::PositionedCluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::inline_object_layout_test_support::InlineObjectLayoutTestSupport;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::line_geometry_stage::LineGeometryStageFns;
use crate::org::tiqian::layout::paragraph_dp_line_breaker::ParagraphDpLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakDecisions;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::layout::punctuation_model::GlueKind;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::string_tools;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault) -> Self {
        match value {
            InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault) -> Self {
        match value {
            InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault) -> Self {
        match value {
            InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault) -> Self {
        match value {
            InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault) -> Self {
        match value {
            InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        InlineObjectLayoutTestSeparatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObjectFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault) -> Self {
        match value {
            InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault) -> Self {
        match value {
            InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault) -> Self {
        match value {
            InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault) -> Self {
        match value {
            InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault) -> Self {
        match value {
            InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        InlineObjectLayoutTestRelationStretchMovesBothFormulaSidesByTheSameFinalGeometryFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault) -> Self {
        match value {
            InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault) -> Self {
        match value {
            InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault) -> Self {
        match value {
            InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault) -> Self {
        match value {
            InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault) -> Self {
        match value {
            InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        InlineObjectLayoutTestPunctuationAttachedToInlineObjectNeverStartsWrappedLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault) -> Self {
        match value {
            InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault) -> Self {
        match value {
            InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault) -> Self {
        match value {
            InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault) -> Self {
        match value {
            InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault) -> Self {
        match value {
            InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        InlineObjectLayoutTestPerAtomFormulaChainNeverBreaksMidRunFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestInlineObjectUsesExistingInterlineSpaceWithoutMovingBaselinesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestInlineObjectSkipsFontShapingAndExpandsItsOwnLineMetricsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestInlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShapingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestInlineObjectIsOneIndivisibleBreakClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault) -> Self {
        match value {
            InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        InlineObjectLayoutTestInlineObjectExpandsBaselineGapOnlyForActualCollisionFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault) -> Self {
        match value {
            InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault) -> Self {
        match value {
            InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault) -> Self {
        match value {
            InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault) -> Self {
        match value {
            InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault) -> Self {
        match value {
            InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        InlineObjectLayoutTestFormulaBreakKeepsBaselineOperatorOnPreviousLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault) -> Self {
        match value {
            InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault) -> Self {
        match value {
            InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault) -> Self {
        match value {
            InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault) -> Self {
        match value {
            InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault) -> Self {
        match value {
            InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        InlineObjectLayoutTestFormulaBoundaryCompressionPushesAttachedCommaIntoPreviousLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn line_boundary_closes_one_ulp_gap_without_changing_baseline_distance() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.lineBoundaryClosesOneUlpGapWithoutChangingBaselineDistance", "org.tiqian.layout.InlineObjectLayoutTest.lineBoundaryClosesOneUlpGapWithoutChangingBaselineDistance", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[108,105,110,101,66,111,117,110,100,97,114,121,67,108,111,115,101,115,79,110,101,85,108,112,71,97,112,87,105,116,104,111,117,116,67,104,97,110,103,105,110,103,66,97,115,101,108,105,110,101,68,105,115,116,97,110,99,101]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(84.14f64, LineGeometryStageFns::line_geometry_stage_fns_resolve_inline_object_line_boundary_extent(80 as f64, 84.14f64, 100 as f64, 15.86001f64), None).unwrap();
    });
}

#[test]
fn inline_object_uses_existing_interline_space_without_moving_baselines() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.inlineObjectUsesExistingInterlineSpaceWithoutMovingBaselines", "org.tiqian.layout.InlineObjectLayoutTest.inlineObjectUsesExistingInterlineSpaceWithoutMovingBaselines", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,85,115,101,115,69,120,105,115,116,105,110,103,73,110,116,101,114,108,105,110,101,83,112,97,99,101,87,105,116,104,111,117,116,77,111,118,105,110,103,66,97,115,101,108,105,110,101,115]));
        let p = InlineObjectLayoutTestSupport::inline_object_layout_test_support_layout(UStr::new(&[30002,20057]), 16 as f64, None, None).unwrap();
        let o = InlineObjectLayoutTestSupport::inline_object_layout_test_support_layout(UStr::new(&[30002,20057]), 16 as f64, Some(vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 16 as f64 as f64, 20 as f64 as f64, 2 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((o.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(p.lines[1usize].baseline - p.lines[0usize].baseline, o.lines[1usize].baseline - o.lines[0usize].baseline, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(24 as f64, o.lines[1usize].baseline - o.lines[0usize].baseline, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance((p.size).clone().height, (o.size).clone().height, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(o.lines[1usize].baseline - format!("{}", (20i32)).parse::<f64>().unwrap_or(0.0), o.lines[1usize].top, 0.001f64, Some(UString::from("the existing inter-line gap should be reassigned to the object's own line box"))).unwrap();
        let d = (o.debug).clone().inline_object_line_height_decision;
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(d.is_some(), InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_nullable_decision((d).clone()).unwrap().as_ustr(), None).unwrap();
        if d.is_none() {
            return;
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(1.6f64, (d).as_ref().unwrap().minimum_clearance, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((o.lines[1usize].baseline - format!("{}", (20i32)).parse::<f64>().unwrap_or(0.0)) - (o.lines[0usize].baseline + (d).as_ref().unwrap().base_face_descent)) >= (d).as_ref().unwrap().minimum_clearance - 0.001f64, None).unwrap();
        let mut extras = true;
        for i in 0..match u32::try_from(((d).as_ref().unwrap().line_extras).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if d.as_ref().unwrap().line_extras.clone()[usize::try_from(i).unwrap_or(0)] != 0 as f64 {
                extras = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(extras, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((((d).as_ref().unwrap().expanded_line_indices).clone().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((((d).as_ref().unwrap().boundary_shifts_after).clone().len()) & 0xFFFF_FFFF).unwrap_or(0) == 1 && (((d).as_ref().unwrap().boundary_shifts_after).clone()[0usize]) < (0 as f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[69,120,105,115,116,105,110,103,73,110,116,101,114,108,105,110,101,83,112,97,99,101,70,105,116,115,73,110,108,105,110,101,79,98,106,101,99,116,115]), ((d).as_ref().unwrap().reason).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn inline_object_expands_baseline_gap_only_for_actual_collision() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.inlineObjectExpandsBaselineGapOnlyForActualCollision", "org.tiqian.layout.InlineObjectLayoutTest.inlineObjectExpandsBaselineGapOnlyForActualCollision", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,69,120,112,97,110,100,115,66,97,115,101,108,105,110,101,71,97,112,79,110,108,121,70,111,114,65,99,116,117,97,108,67,111,108,108,105,115,105,111,110]));
        let a = vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), 16 as f64 as f64, 14 as f64 as f64, 10 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 16 as f64 as f64, 20 as f64 as f64, 2 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
];
        let r = InlineObjectLayoutTestSupport::inline_object_layout_test_support_layout(UStr::new(&[30002,20057]), 16 as f64, Some((a).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(31.6f64, r.lines[1usize].baseline - r.lines[0usize].baseline, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(1.6f64, (r.lines[1usize].baseline - format!("{}", (20i32)).parse::<f64>().unwrap_or(0.0)) - (r.lines[0usize].baseline + format!("{}", (10i32)).parse::<f64>().unwrap_or(0.0)), 0.001f64, Some(UString::from("the measured collision deficit must retain the configured safety clearance"))).unwrap();
        let d = (r.debug).clone().inline_object_line_height_decision;
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(d.is_some(), InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_nullable_decision((d).clone()).unwrap().as_ustr(), None).unwrap();
        if d.is_none() {
            return;
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0 as f64, ((d).as_ref().unwrap().line_extras).clone()[0usize], 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(7.6f64, ((d).as_ref().unwrap().line_extras).clone()[1usize], 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![1], &((d).as_ref().unwrap().expanded_line_indices).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,108,105,110,101,79,98,106,101,99,116,73,110,116,101,114,108,105,110,101,67,111,108,108,105,115,105,111,110]), ((d).as_ref().unwrap().reason).to_ustring().as_ustr(), None).unwrap();
        let s = ParagraphStyle::new(Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone().last_line_alignment), Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone().writing_mode), (*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone().line_height, ((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone().first_line_indent).clone(), Some(((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone().block_indent).clone()), Some(((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone().first_line_indent_policy).clone()), Some(((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone().line_length_grid).clone()), Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone().ruby_line_height_mode), Some(0 as f64), Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone().emphasis_dot_gap_em));
        let z = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[30002,20057])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((s).clone()), LayoutConstraints::new(16 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some((a).clone()))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(30 as f64, z.lines[1usize].baseline - z.lines[0usize].baseline, 0.001f64, None).unwrap();
        let zd = (z.debug).clone().inline_object_line_height_decision;
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(zd.is_some(), InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_nullable_decision((zd).clone()).unwrap().as_ustr(), None).unwrap();
        if zd.is_none() {
            return;
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, (zd).as_ref().unwrap().minimum_clearance, None).unwrap();
    });
}

#[test]
fn inline_object_skips_font_shaping_and_expands_its_own_line_metrics() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.inlineObjectSkipsFontShapingAndExpandsItsOwnLineMetrics", "org.tiqian.layout.InlineObjectLayoutTest.inlineObjectSkipsFontShapingAndExpandsItsOwnLineMetrics", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,83,107,105,112,115,70,111,110,116,83,104,97,112,105,110,103,65,110,100,69,120,112,97,110,100,115,73,116,115,79,119,110,76,105,110,101,77,101,116,114,105,99,115]));
        let r = InlineObjectLayoutTestSupport::inline_object_layout_test_support_layout(UStr::new(&[20013,65532,25991]), 120 as f64, Some(vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20 as f64 as f64, 30 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]), None).unwrap();
        let c = InlineObjectLayoutTestSupport::inline_object_layout_test_support_single_cluster((r).clone(), TextRange::new(1u32, 2u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(20 as f64, c.advance, 0.001f64, None).unwrap();
        let mut no_glyph = true;
        for i in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            for j in 0..match u32::try_from((r.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone().glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if InlineObjectLayoutTestSupport::inline_object_layout_test_support_same_range((((r.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone().glyphs[usize::try_from(j).unwrap_or(0)]).clone().cluster_range).clone(), (c.range).clone()) {
                    no_glyph = false;
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(no_glyph, None).unwrap();
        let shaping = InlineObjectLayoutTestSupport::inline_object_layout_test_support_single_shaping_decision((r).clone(), (c.range).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, shaping.glyph_count, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[77,101,97,115,117,114,97,98,108,101,79,112,97,113,117,101,73,110,108,105,110,101,79,98,106,101,99,116,58,110,111,45,102,111,110,116,45,115,104,97,112,105,110,103]), (shaping.reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.lines[0usize].baseline - r.lines[0usize].top) >= 30 as f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.lines[0usize].bottom - r.lines[0usize].baseline) >= 4 as f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, (r.debug).clone().inline_object_decisions[0usize].line_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[77,101,97,115,117,114,97,98,108,101,79,112,97,113,117,101,73,110,108,105,110,101,79,98,106,101,99,116]), (((r.debug).clone().inline_object_decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn inline_object_is_one_indivisible_break_cluster() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.inlineObjectIsOneIndivisibleBreakCluster", "org.tiqian.layout.InlineObjectLayoutTest.inlineObjectIsOneIndivisibleBreakCluster", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,73,115,79,110,101,73,110,100,105,118,105,115,105,98,108,101,66,114,101,97,107,67,108,117,115,116,101,114]));
        let r = InlineObjectLayoutTestSupport::inline_object_layout_test_support_layout(UStr::new(&[20013,65532,25991]), 35 as f64, Some(vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20 as f64 as f64, 16 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]), None).unwrap();
        let mut object_index = 4294967295u32;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if InlineObjectLayoutTestSupport::inline_object_layout_test_support_same_range(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone(), TextRange::new(1u32, 2u32).unwrap()) {
                object_index = i;
            }
        }
        let mut object_line: Option<LineBox> = None;
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i32::from_ne_bytes(((object_index) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((r.lines[usize::try_from(i).unwrap_or(0)]).clone().cluster_range.start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((object_index) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((r.lines[usize::try_from(i).unwrap_or(0)]).clone().cluster_range.end) as i32).to_ne_bytes()) {
                object_line = Some((r.lines[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(object_index, object_index), (object_line.as_ref().unwrap().cluster_range).clone(), None).unwrap();
    });
}

#[test]
fn inline_object_keeps_alternate_source_text_while_skipping_its_glyph_shaping() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.inlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShaping", "org.tiqian.layout.InlineObjectLayoutTest.inlineObjectKeepsAlternateSourceTextWhileSkippingItsGlyphShaping", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,75,101,101,112,115,65,108,116,101,114,110,97,116,101,83,111,117,114,99,101,84,101,120,116,87,104,105,108,101,83,107,105,112,112,105,110,103,73,116,115,71,108,121,112,104,83,104,97,112,105,110,103]));
        let r = InlineObjectLayoutTestSupport::inline_object_layout_test_support_layout(UStr::new(&[20013,22270,29255,25991]), 120 as f64, Some(vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 3u32).unwrap(), 20 as f64 as f64, 16 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]), None).unwrap();
        let c = (r.clusters[1usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[22270,29255]), (c.text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[]), (c.display_text).to_ustring().as_ustr(), None).unwrap();
        let mut no_glyph = true;
        for i in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            for j in 0..match u32::try_from((r.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone().glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if InlineObjectLayoutTestSupport::inline_object_layout_test_support_same_range((((r.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone().glyphs[usize::try_from(j).unwrap_or(0)]).clone().cluster_range).clone(), (c.range).clone()) {
                    no_glyph = false;
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(no_glyph, None).unwrap();
        let shaping = InlineObjectLayoutTestSupport::inline_object_layout_test_support_single_shaping_decision((r).clone(), (c.range).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[22270,29255]), (shaping.source_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[]), (shaping.display_text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn adjust_break_for_unbreakables_retreats_past_the_whole_contiguous_run() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.adjustBreakForUnbreakablesRetreatsPastTheWholeContiguousRun", "org.tiqian.layout.InlineObjectLayoutTest.adjustBreakForUnbreakablesRetreatsPastTheWholeContiguousRun", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[97,100,106,117,115,116,66,114,101,97,107,70,111,114,85,110,98,114,101,97,107,97,98,108,101,115,82,101,116,114,101,97,116,115,80,97,115,116,84,104,101,87,104,111,108,101,67,111,110,116,105,103,117,111,117,115,82,117,110]));
        let u = UnbreakableRanges::new(vec![
    (IntRange::new(1u32, 2u32)).clone(),
    (IntRange::new(2u32, 3u32)).clone(),
    (IntRange::new(3u32, 4u32)).clone(),
].to_vec());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(4, 0, (u).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(3, 0, (u).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(2, 0, (u).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(5, 0, (u).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(5, 2, UnbreakableRanges::new(vec![(IntRange::new(3u32, 4u32)).clone(), (IntRange::new(4u32, 5u32)).clone()].to_vec())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(4, 1, (u).clone()), None).unwrap();
    });
}

#[test]
fn per_atom_formula_chain_never_breaks_mid_run() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.perAtomFormulaChainNeverBreaksMidRun", "org.tiqian.layout.InlineObjectLayoutTest.perAtomFormulaChainNeverBreaksMidRun", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[112,101,114,65,116,111,109,70,111,114,109,117,108,97,67,104,97,105,110,78,101,118,101,114,66,114,101,97,107,115,77,105,100,82,117,110]));
        let mut a: Vec<InlineObjectSpan> = vec![];
        {
            a.push(InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 12 as f64 as f64, 16 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(true)).unwrap())).unwrap());
            a.push(InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 12 as f64 as f64, 16 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(true)).unwrap())).unwrap());
            a.push(InlineObjectSpan::new(TextRange::new(3u32, 4u32).unwrap(), 12 as f64 as f64, 16 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(true)).unwrap())).unwrap());
            a.push(InlineObjectSpan::new(TextRange::new(4u32, 5u32).unwrap(), 12 as f64 as f64, 16 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(false)).unwrap())).unwrap());
        }
        let r = InlineObjectLayoutTestSupport::inline_object_layout_test_support_fixed_basic_kinsoku_engine(Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[20013,19968,20108,19977,22235])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone()), LayoutConstraints::new(60 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some((a).clone()))).unwrap();
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_true((l.range).clone().end != 2 && (l.range).clone().end != 3 && (l.range).clone().end != 4, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("line ended inside the unbreakable formula chain: ")); __s += InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_line_ranges((r).clone()).as_ustr(); __s }).as_str()))).unwrap();
        }
    });
}

#[test]
fn formula_boundary_compression_pushes_attached_comma_into_previous_line() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.formulaBoundaryCompressionPushesAttachedCommaIntoPreviousLine", "org.tiqian.layout.InlineObjectLayoutTest.formulaBoundaryCompressionPushesAttachedCommaIntoPreviousLine", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[102,111,114,109,117,108,97,66,111,117,110,100,97,114,121,67,111,109,112,114,101,115,115,105,111,110,80,117,115,104,101,115,65,116,116,97,99,104,101,100,67,111,109,109,97,73,110,116,111,80,114,101,118,105,111,117,115,76,105,110,101]));
        let text = UString::from("x+，后").to_ustring();
        let r = InlineObjectLayoutTestSupport::inline_object_layout_test_support_fixed_basic_kinsoku_engine(None).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone()), LayoutConstraints::new(36 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 2u32).unwrap(), 30 as f64 as f64, 16 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(true), None, Some(4 as f64), Some(0.0), Some(false)).unwrap())).unwrap()).clone(),
]))).unwrap();
        let ls = InlineObjectLayoutTestSupport::inline_object_layout_test_support_lines((r).clone(), text.as_ustr());
        let mut bad = false;
        for i in 0..match u32::try_from(ls.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if ls[usize::try_from(i).unwrap_or(0)].clone().starts_with(&UString::from("，")) {
                bad = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!bad, None).unwrap();
        let repair = ((r.debug).clone().line_decisions[0usize]).clone().repair_decision;
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(repair.is_some(), InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_nullable_repair((repair).clone()).unwrap().as_ustr(), None).unwrap();
        if repair.is_none() {
            return;
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,117,115,104,73,110]), ((repair).as_ref().unwrap().kind).to_ustring().as_ustr(), None).unwrap();
        let mut found = false;
        for i in 0..match u32::try_from(((repair).as_ref().unwrap().push_in_allocations).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if InlineObjectLayoutTestSupport::inline_object_layout_test_support_same_range(((((repair).as_ref().unwrap().push_in_allocations).clone()[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone(), TextRange::new(0u32, 2u32).unwrap()) && (((repair).as_ref().unwrap().push_in_allocations).clone()[usize::try_from(i).unwrap_or(0)].shrink) > (0 as f64) {
                found = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(found, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("formula boundary space must contribute to compression: ")); __s += InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_repair_allocations(&((repair).as_ref().unwrap().push_in_allocations).clone()).as_ustr(); __s }).as_str()))).unwrap();
    });
}

#[test]
fn punctuation_attached_to_inline_object_never_starts_wrapped_line() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.punctuationAttachedToInlineObjectNeverStartsWrappedLine", "org.tiqian.layout.InlineObjectLayoutTest.punctuationAttachedToInlineObjectNeverStartsWrappedLine", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[112,117,110,99,116,117,97,116,105,111,110,65,116,116,97,99,104,101,100,84,111,73,110,108,105,110,101,79,98,106,101,99,116,78,101,118,101,114,83,116,97,114,116,115,87,114,97,112,112,101,100,76,105,110,101]));
        {
            let _g1 = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
    Box::new((ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).unwrap()).clone()) as Box<dyn LineBreaker>,
];
            for b in &_g1 {
                {
                    let mut _g = 0u32;
                    let _g1 = vec![UString::from("，").to_ustring(), UString::from(",").to_ustring()];
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let comma = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        {
                            let mut _g = 0u32;
                            let _g1 = vec![24, 32, 36, 48, 64];
                            while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                                let w = _g1[usize::try_from(_g).unwrap_or(0)];
                                _g = u32::wrapping_add(_g, 1);
                                let text = { let mut __s = UString::new(); __s += &(UString::from("x+")); __s += comma.as_ustr(); __s += &(UString::from(" 后")); __s };
                                let r = InlineObjectLayoutTestSupport::inline_object_layout_test_support_fixed_basic_kinsoku_engine(Some((*b).clone())).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone()), LayoutConstraints::new({ let v: u32 = w; i32::from_ne_bytes(v.to_ne_bytes()) } as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 2u32).unwrap(), 30 as f64 as f64, 16 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]))).unwrap();
                                let ls = InlineObjectLayoutTestSupport::inline_object_layout_test_support_lines((r).clone(), text.as_ustr());
                                let mut bad = false;
                                for i in 0..match u32::try_from(ls.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                    if ls[usize::try_from(i).unwrap_or(0)].clone().starts_with(&comma) {
                                        bad = true;
                                    }
                                }
                                let _ = TracedAssertions::traced_assertions_assert_true(!bad, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("breaker=")); __s += b.get_strategy_name().as_ustr(); __s += &(UString::from(" width=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(w)).as_str())); __s += &(UString::from(" comma=")); __s += comma.as_ustr(); __s += &(UString::from(" lines=")); __s += InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_strings(&ls).as_ustr(); __s }).as_str()))).unwrap();
                                let mut any = false;
                                for i in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                    if r.debug.clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_ustring() == comma && (((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)]).clone().reason).to_ustring() == UString::from("InlineObjectAttachedKinsoku") {
                                        any = true;
                                    }
                                }
                                let _ = TracedAssertions::traced_assertions_assert_true(any, None).unwrap();
                            }
                        }
                    }
                }
            }
        }
    });
}

#[test]
fn separator_space_before_punctuation_collapses_and_stays_with_inline_object() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.separatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObject", "org.tiqian.layout.InlineObjectLayoutTest.separatorSpaceBeforePunctuationCollapsesAndStaysWithInlineObject", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[115,101,112,97,114,97,116,111,114,83,112,97,99,101,66,101,102,111,114,101,80,117,110,99,116,117,97,116,105,111,110,67,111,108,108,97,112,115,101,115,65,110,100,83,116,97,121,115,87,105,116,104,73,110,108,105,110,101,79,98,106,101,99,116]));
        {
            let _g1 = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
    Box::new((ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).unwrap()).clone()) as Box<dyn LineBreaker>,
];
            for b in &_g1 {
                {
                    let mut _g = 0u32;
                    let _g1 = vec![32, 40, 48, 56, 64];
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let w = _g1[usize::try_from(_g).unwrap_or(0)];
                        _g = u32::wrapping_add(_g, 1);
                        let text = UString::from("前x ，后文").to_ustring();
                        let r = InlineObjectLayoutTestSupport::inline_object_layout_test_support_fixed_basic_kinsoku_engine(Some((*b).clone())).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone()), LayoutConstraints::new({ let v: u32 = w; i32::from_ne_bytes(v.to_ne_bytes()) } as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 24 as f64 as f64, 16 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::new(Some(true), None, Some(0.0), Some(0.0), Some(false)).unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(true), None, Some(0.0), Some(0.0), Some(false)).unwrap())).unwrap()).clone(),
]))).unwrap();
                        let space = InlineObjectLayoutTestSupport::inline_object_layout_test_support_single_cluster((r).clone(), TextRange::new(2u32, 3u32).unwrap());
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0 as f64, space.advance, 0.001f64, None).unwrap();
                        let ls = InlineObjectLayoutTestSupport::inline_object_layout_test_support_lines((r).clone(), text.as_ustr());
                        let mut bad = false;
                        for i in 0..match u32::try_from(ls.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            if string_tools::StringTools::string_tools_ltrim((ls[usize::try_from(i).unwrap_or(0)]).clone().as_ustr()).starts_with(&UString::from("，")) {
                                bad = true;
                            }
                        }
                        let _ = TracedAssertions::traced_assertions_assert_true(!bad, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("breaker=")); __s += b.get_strategy_name().as_ustr(); __s += &(UString::from(" width=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(w)).as_str())); __s += &(UString::from(" lines=")); __s += InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_strings(&ls).as_ustr(); __s }).as_str()))).unwrap();
                        let mut any = false;
                        for i in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            if r.debug.clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_ustring() == UString::from("，") && (((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)]).clone().reason).to_ustring() == UString::from("InlineObjectAttachedKinsokuAcrossCollapsedSeparatorSpace") {
                                any = true;
                            }
                        }
                        let _ = TracedAssertions::traced_assertions_assert_true(any, None).unwrap();
                        let att = ((r.debug).clone().inline_object_punctuation_attachment_decisions[0usize]).clone();
                        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(2u32, 3u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (att.separator_range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_true((att.collapsed_advance) > (0 as f64), None).unwrap();
                        let mut closed = true;
                        for i in 0..match u32::try_from((r.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            for j in 0..match u32::try_from(((r.debug).clone().justification_decisions[usize::try_from(i).unwrap_or(0)]).clone().allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                let al = (((r.debug).clone().justification_decisions[usize::try_from(i).unwrap_or(0)]).clone().allocations[usize::try_from(j).unwrap_or(0)]).clone();
                                if InlineObjectLayoutTestSupport::inline_object_layout_test_support_same_range((al.cluster_range).clone(), TextRange::new(1u32, 2u32).unwrap()) && (al.kind).to_ustring() == UString::from(GlueKind::InlineObjectBoundary.name()) {
                                    closed = false;
                                }
                            }
                        }
                        let _ = TracedAssertions::traced_assertions_assert_true(closed, Some(UString::from("the formula edge before attached punctuation must stay closed"))).unwrap();
                    }
                }
            }
        }
    });
}

#[test]
fn relation_stretch_moves_both_formula_sides_by_the_same_final_geometry() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.relationStretchMovesBothFormulaSidesByTheSameFinalGeometry", "org.tiqian.layout.InlineObjectLayoutTest.relationStretchMovesBothFormulaSidesByTheSameFinalGeometry", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[114,101,108,97,116,105,111,110,83,116,114,101,116,99,104,77,111,118,101,115,66,111,116,104,70,111,114,109,117,108,97,83,105,100,101,115,66,121,84,104,101,83,97,109,101,70,105,110,97,108,71,101,111,109,101,116,114,121]));
        let natural_relation_gap = 4.44444444444444464f64;
        let target_gap = 8.0f64;
        let formula_body_width = 10u32;
        let text = UString::from("a=b中").to_ustring();
        let r = InlineObjectLayoutTestSupport::inline_object_layout_test_support_fixed_basic_kinsoku_engine(None).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone()), LayoutConstraints::new(47 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), format!("{}", (i32::from_ne_bytes(((formula_body_width) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) + natural_relation_gap, 12 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(true), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, natural_relation_gap, target_gap).unwrap()), Some(0.0), Some(0.0), Some(false)).unwrap())).unwrap()).clone(),
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), format!("{}", (i32::from_ne_bytes(((formula_body_width) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) + natural_relation_gap, 12 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(true), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, natural_relation_gap, target_gap).unwrap()), Some(0.0), Some(0.0), Some(true)).unwrap())).unwrap()).clone(),
    (InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), i32::from_ne_bytes(((formula_body_width) as i32).to_ne_bytes()) as f64 as f64, 12 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (1), None).unwrap();
        let pc = LayoutQueries::layout_queries_positioned_clusters((r).clone());
        let mut formula: Vec<PositionedCluster> = vec![];
        for i in 0..match u32::try_from(pc.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i32::from_ne_bytes(((((pc[usize::try_from(i).unwrap_or(0)]).clone().range).clone().end) as i32).to_ne_bytes()) <= 3 {
                formula.push((pc[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((formula.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, formula[0usize].line_index, None).unwrap();
        let before_equals = formula[1usize].draw_x - (formula[0usize].draw_x + format!("{}", (i32::from_ne_bytes(((formula_body_width) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0));
        let after_equals = formula[2usize].draw_x - (formula[1usize].draw_x + format!("{}", (i32::from_ne_bytes(((formula_body_width) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0));
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(before_equals, after_equals, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((before_equals) >= target_gap, None).unwrap();
        let mut rel: Vec<JustificationAllocationInfo> = vec![];
        for i in 0..match u32::try_from(((r.debug).clone().justification_decisions[0usize]).clone().allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().justification_decisions[0usize].clone().allocations[usize::try_from(i).unwrap_or(0)].clone().kind.to_ustring() == UString::from(GlueKind::InlineObjectRelation.name()) {
                rel.push((((r.debug).clone().justification_decisions[0usize]).clone().allocations[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((rel.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(rel[0usize].delta, rel[1usize].delta, 0.001f64, None).unwrap();
    });
}

#[test]
fn formula_break_keeps_baseline_operator_on_previous_line() {
    testlib::run("org.tiqian.layout.InlineObjectLayoutTest.formulaBreakKeepsBaselineOperatorOnPreviousLine", "org.tiqian.layout.InlineObjectLayoutTest.formulaBreakKeepsBaselineOperatorOnPreviousLine", || {
        InlineObjectLayoutTestSupport::inline_object_layout_test_support_rec(UStr::new(&[102,111,114,109,117,108,97,66,114,101,97,107,75,101,101,112,115,66,97,115,101,108,105,110,101,79,112,101,114,97,116,111,114,79,110,80,114,101,118,105,111,117,115,76,105,110,101]));
        let text = UString::from("a+b").to_ustring();
        let objs = vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), 12 as f64 as f64, 12 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 12 as f64 as f64, 12 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(true)).unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(4 as f64), Some(4 as f64), Some(false)).unwrap())).unwrap()).clone(),
    (InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 12 as f64 as f64, 12 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
];
        {
            let _g1 = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
    Box::new((ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).unwrap()).clone()) as Box<dyn LineBreaker>,
];
            for b in &_g1 {
                let r = InlineObjectLayoutTestSupport::inline_object_layout_test_support_fixed_basic_kinsoku_engine(Some((*b).clone())).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone()), LayoutConstraints::new(24 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some((objs).clone()))).unwrap();
                let ls = InlineObjectLayoutTestSupport::inline_object_layout_test_support_lines((r).clone(), text.as_ustr());
                let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((ls.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (1), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("breaker=")); __s += b.get_strategy_name().as_ustr(); __s += &(UString::from(" lines=")); __s += InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_strings(&ls).as_ustr(); __s }).as_str()))).unwrap();
                let mut starts = false;
                for i in 1..match u32::try_from(ls.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if ls[usize::try_from(i).unwrap_or(0)].clone().starts_with(&UString::from("+")) {
                        starts = true;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(!starts, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("the adjustment-only boundary before the operator must stay closed: breaker=")); __s += b.get_strategy_name().as_ustr(); __s += &(UString::from(" lines=")); __s += InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_strings(&ls).as_ustr(); __s }).as_str()))).unwrap();
                let mut ends = false;
                for i in 0..u32::try_from((ls.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
                    if ls[usize::try_from(i).unwrap_or(0)].clone().ends_with(&UString::from("+")) {
                        ends = true;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(ends, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("breaker=")); __s += b.get_strategy_name().as_ustr(); __s += &(UString::from(" lines=")); __s += InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_strings(&ls).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(8 as f64, InlineObjectLayoutTestSupport::inline_object_layout_test_support_single_cluster((r).clone(), TextRange::new(1u32, 2u32).unwrap()).advance, 0.001f64, Some(UString::from("the operator glyph stays while its post-operator line-end glue disappears"))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(20 as f64, r.lines[0usize].visual_width, 0.001f64, Some(UString::from("the discarded glue must not remain in the previous line's width"))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0 as f64, InlineObjectLayoutTestSupport::inline_object_layout_test_support_single_positioned(&LayoutQueries::layout_queries_positioned_clusters((r).clone()), TextRange::new(2u32, 3u32).unwrap()).draw_x, 0.001f64, Some(UString::from("the following operand must start without inherited formula glue"))).unwrap();
                let mut trim = false;
                for i in 0..match u32::try_from((r.debug).clone().line_edge_trim_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    let x = ((r.debug).clone().line_edge_trim_decisions[usize::try_from(i).unwrap_or(0)]).clone();
                    if InlineObjectLayoutTestSupport::inline_object_layout_test_support_same_range((x.cluster_range).clone(), TextRange::new(1u32, 2u32).unwrap()) && (x.reason).to_ustring() == UString::from("InlineObjectLineEndDiscardableGlue") && x.natural_glue == 4 as f64 {
                        trim = true;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(trim, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("breaker=")); __s += b.get_strategy_name().as_ustr(); __s += &(UString::from(" trims=")); __s += InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_trim_decisions(&(r.debug).clone().line_edge_trim_decisions).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_true(InlineObjectLayoutTestSupport::inline_object_layout_test_support_single_inline_object_decision((r).clone(), TextRange::new(1u32, 2u32).unwrap()).leading_prevents_line_break, None).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, InlineObjectLayoutTestSupport::inline_object_layout_test_support_single_inline_object_decision((r).clone(), TextRange::new(1u32, 2u32).unwrap()).trailing_line_end_discardable_advance, None).unwrap();
                let u = InlineObjectLayoutTestSupport::inline_object_layout_test_support_fixed_basic_kinsoku_engine(Some((*b).clone())).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*crate::org::tiqian::layout::inline_object_layout_test_support::INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone()), LayoutConstraints::new(60 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some((objs).clone()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((u.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(12 as f64, InlineObjectLayoutTestSupport::inline_object_layout_test_support_single_cluster((u).clone(), TextRange::new(1u32, 2u32).unwrap()).advance, 0.001f64, None).unwrap();
                let mut no = false;
                for i in 0..match u32::try_from((u.debug).clone().line_edge_trim_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if u.debug.clone().line_edge_trim_decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_ustring() == UString::from("InlineObjectLineEndDiscardableGlue") {
                        no = true;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(!no, None).unwrap();
            }
        }
    });
}
