#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::explainable_stub_paragraph_layout_engine_test_support::EmptyTextShaper;
use crate::org::tiqian::layout::explainable_stub_paragraph_layout_engine_test_support::ExplainableStubParagraphLayoutEngineTestSupport;
use crate::org::tiqian::layout::explainable_stub_paragraph_layout_engine_test_support::FixedBoundsTextShaper;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestSingleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestReturnsDebuggableSingleLineResultFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRejectsShaperClustersThatDoNotCoverFontDecisionRangeFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsUnicodeEmojiSequenceRolePromotionsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsInjectedLineBreakerStrategyInDebugDecisionsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestRecordsFallbackDecisionsPerClusterFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestPreservesShaperGlyphBoundsInLayoutGlyphRunsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestMandatoryLineBreakClustersAreZeroWidthAndNotShapedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestMandatoryBreakLineIsNotJustifiedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestEmojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedTextFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestCrlfIsOneMandatoryBreakClusterFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestConsecutiveMandatoryLineBreaksCreateOneEmptyLineBoxFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestConsecutiveAndTrailingMandatoryBreaksPreserveBlankLinesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiSequencesReachTheShaperAsCompleteEmojiRangesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestComplexEmojiGraphemesHonorTextSpanStyleBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault) -> Self {
        match value {
            ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ExplainableStubParagraphLayoutEngineTestCombiningMarksStayInTheirBaseShapingRunsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn returns_debuggable_single_line_result() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.returnsDebuggableSingleLineResult", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.returnsDebuggableSingleLineResult", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[114,101,116,117,114,110,115,68,101,98,117,103,103,97,98,108,101,83,105,110,103,108,101,76,105,110,101,82,101,115,117,108,116]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[25552,26912]), 240 as f64, None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[103,114,101,101,100,121]), (((r.debug).clone().line_decisions[0usize]).clone().kind).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn records_injected_line_breaker_strategy_in_debug_decisions() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsInjectedLineBreakerStrategyInDebugDecisions", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsInjectedLineBreakerStrategyInDebugDecisions", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[114,101,99,111,114,100,115,73,110,106,101,99,116,101,100,76,105,110,101,66,114,101,97,107,101,114,83,116,114,97,116,101,103,121,73,110,68,101,98,117,103,68,101,99,105,115,105,111,110,115]));
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[25552,26912]), 240 as f64, None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,111,111,107,97,104,101,97,100]), (((r.debug).clone().line_decisions[0usize]).clone().kind).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn mandatory_line_break_clusters_are_zero_width_and_not_shaped() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.mandatoryLineBreakClustersAreZeroWidthAndNotShaped", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.mandatoryLineBreakClustersAreZeroWidthAndNotShaped", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[109,97,110,100,97,116,111,114,121,76,105,110,101,66,114,101,97,107,67,108,117,115,116,101,114,115,65,114,101,90,101,114,111,87,105,100,116,104,65,110,100,78,111,116,83,104,97,112,101,100]));
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[31532,19968,34892,10,31532,20108,34892]), 240 as f64, None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[0usize].end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::ParagraphEnd), &(r.lines[1usize].end_reason), None).unwrap();
        let mut b: Option<Cluster> = None;
        for _g_index in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if c.text.to_ustring() == UString::from(concat!("\n",
"")) {
                b = Some(c.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[]), (b.as_ref().unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, b.as_ref().unwrap().advance, None).unwrap();
        let mut found = false;
        for ri in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (r.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                if g.cluster_range.clone() == (b.as_ref().unwrap().range).clone() {
                    found = true;
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!found, None).unwrap();
        let pair = vec![((r.glyph_runs[0usize]).clone().range).clone(), ((r.glyph_runs[1usize]).clone().range).clone()];
        let _ = TracedAssertions::traced_assertions_assert_equals_text_range_array(&vec![(TextRange::new(0u32, 3u32).unwrap()).clone(), (TextRange::new(4u32, 7u32).unwrap()).clone()], &pair, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", (b.as_ref().unwrap().range).clone().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (((r.debug).clone().mandatory_break_decisions[0usize]).clone().range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn consecutive_mandatory_line_breaks_create_one_empty_line_box() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.consecutiveMandatoryLineBreaksCreateOneEmptyLineBox", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.consecutiveMandatoryLineBreaksCreateOneEmptyLineBox", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[99,111,110,115,101,99,117,116,105,118,101,77,97,110,100,97,116,111,114,121,76,105,110,101,66,114,101,97,107,115,67,114,101,97,116,101,79,110,101,69,109,112,116,121,76,105,110,101,66,111,120]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[31532,19968,34892,10,10,31532,20108,34892]), 240 as f64, None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[0usize].end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[1usize].end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::ParagraphEnd), &(r.lines[2usize].end_reason), None).unwrap();
        let c = (r.clusters[usize::try_from((r.lines[1usize]).clone().cluster_range.start).unwrap_or(0)]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[10]), (c.text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[]), (c.display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, c.advance, None).unwrap();
        let h = (r.debug).clone().line_spacing_decision.as_ref().unwrap().resolved_height;
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(h, r.lines[1usize].bottom - r.lines[1usize].top, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(h, r.lines[1usize].baseline - r.lines[0usize].baseline, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(h, r.lines[2usize].baseline - r.lines[1usize].baseline, 0.001f64, None).unwrap();
    });
}

#[test]
fn single_mandatory_break_after_wrapped_line_does_not_create_empty_line() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.singleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLine", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.singleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLine", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[115,105,110,103,108,101,77,97,110,100,97,116,111,114,121,66,114,101,97,107,65,102,116,101,114,87,114,97,112,112,101,100,76,105,110,101,68,111,101,115,78,111,116,67,114,101,97,116,101,69,109,112,116,121,76,105,110,101]));
        let text = UString::from(concat!("很久以前，曾经有一个名叫小红帽的孩子，生活在大森林的边上，大森林里充满了濒临灭绝的猫头鹰和珍稀植物，如果有人愿意花时间研究它们，就会发现癌症的治疗方法。\n",
"小红帽和一位称为母亲的养育者一起生活")).to_ustring();
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(text.as_ustr(), 1200 as f64, None, None, Some(TextStyle::new(Some(vec![]), Some(48 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)))).unwrap()).unwrap();
        let mut dbg_lines: Vec<UString> = vec![];
        for _g_index in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(_g_index).unwrap_or(0)]).clone();
            dbg_lines.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += UString::from(format!("{}", l.cluster_range.to_string()).as_str()).as_ustr(); __s += &(UString::from(" ")); __s += UString::from(format!("{}", (l.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(" ")); __s += UString::from(l.end_reason.name()).as_ustr(); __s += &(UString::from(" \"")); __s += UString::from(format!("{}", {
    let from = (l.range).clone().start;
    let to = (l.range).clone().end;
    u_string::slice(text.as_ustr(), i32::from_ne_bytes(((from) as i32).to_ne_bytes()), i32::from_ne_bytes(((to) as i32).to_ne_bytes()))
}).as_str()).as_ustr(); __s += &(UString::from("\"")); __s }).as_str()));
        }
        let dbg = { let joined = dbg_lines; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(concat!("\n",
"")); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) };
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) >= 4, None).unwrap();
        let mut no = false;
        for _g_index1 in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if {
    let from = (l.range).clone().start;
    let to = (l.range).clone().end;
    u_string::slice(text.as_ustr(), i32::from_ne_bytes(((from) as i32).to_ne_bytes()), i32::from_ne_bytes(((to) as i32).to_ne_bytes()))
} == UString::from(concat!("\n",
"")) {
                no = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!no, Some((dbg).to_ustring())).unwrap();
        let mut idx = 0u32;
        for i in 0..u_string::count(text.as_ustr()) {
            if u_string::at(text.as_ustr(), i).as_ref().map_or(false, |v| v == &(10)) {
                idx = u32::wrapping_add(i, 1);
            }
        }
        let mut line: Option<LineBox> = None;
        for _g_index2 in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(_g_index2).unwrap_or(0)]).clone();
            if l.range.clone().end == idx {
                line = Some(l.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(line.as_ref().unwrap().end_reason), None).unwrap();
        let h = (r.debug).clone().line_spacing_decision.as_ref().unwrap().resolved_height;
        for i in 1..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(h, r.lines[usize::try_from(i).unwrap_or(0)].baseline - r.lines[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)].baseline, 0.001f64, None).unwrap();
        }
    });
}

#[test]
fn crlf_is_one_mandatory_break_cluster() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.crlfIsOneMandatoryBreakCluster", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.crlfIsOneMandatoryBreakCluster", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[99,114,108,102,73,115,79,110,101,77,97,110,100,97,116,111,114,121,66,114,101,97,107,67,108,117,115,116,101,114]));
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[30002,13,10,20057]), 240 as f64, None, None, None).unwrap()).unwrap();
        let mut b: Option<Cluster> = None;
        for _g_index in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if c.text.to_ustring() == UString::from(concat!("\r\n",
"")) {
                b = Some(c.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((r.debug).clone().mandatory_break_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, (b.as_ref().unwrap().range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, (b.as_ref().unwrap().range).clone().end, None).unwrap();
    });
}

#[test]
fn consecutive_and_trailing_mandatory_breaks_preserve_blank_lines() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.consecutiveAndTrailingMandatoryBreaksPreserveBlankLines", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.consecutiveAndTrailingMandatoryBreaksPreserveBlankLines", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[99,111,110,115,101,99,117,116,105,118,101,65,110,100,84,114,97,105,108,105,110,103,77,97,110,100,97,116,111,114,121,66,114,101,97,107,115,80,114,101,115,101,114,118,101,66,108,97,110,107,76,105,110,101,115]));
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[30002,10,10,20057,10]), 240 as f64, None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        {
            let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[0usize].end_reason), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[1usize].end_reason), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[2usize].end_reason), None).unwrap();
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::ParagraphEnd), &(r.lines[3usize].end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, r.lines[1usize].visual_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,53,44,32,101,110,100,61,53,41]), UString::from(format!("{}", ((r.lines[3usize]).clone().range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn mandatory_break_line_is_not_justified() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.mandatoryBreakLineIsNotJustified", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.mandatoryBreakLineIsNotJustified", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[109,97,110,100,97,116,111,114,121,66,114,101,97,107,76,105,110,101,73,115,78,111,116,74,117,115,116,105,102,105,101,100]));
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[30701,10,20013,25991,20013,25991,20013,25991,20013,25991,20013,25991]), 128 as f64, None, None, None).unwrap()).unwrap();
        let l = (r.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(l.end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(l.natural_width, l.adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((r.debug).clone().justification_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn rejects_shaper_clusters_that_do_not_cover_font_decision_range() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.rejectsShaperClustersThatDoNotCoverFontDecisionRange", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.rejectsShaperClustersThatDoNotCoverFontDecisionRange", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[114,101,106,101,99,116,115,83,104,97,112,101,114,67,108,117,115,116,101,114,115,84,104,97,116,68,111,78,111,116,67,111,118,101,114,70,111,110,116,68,101,99,105,115,105,111,110,82,97,110,103,101]));
        let f: Arc<dyn Fn() -> Result<(), IllegalStateException> + Send + Sync + 'static> = {  Arc::new(move || {
        ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(Some(Arc::new(Mutex::new(EmptyTextShaper::new()))), None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?.layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[25552,26912]), 240 as f64, None, None, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) };
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), (f).clone()).unwrap();
    });
}

#[test]
fn preserves_shaper_glyph_bounds_in_layout_glyph_runs() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.preservesShaperGlyphBoundsInLayoutGlyphRuns", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.preservesShaperGlyphBoundsInLayoutGlyphRuns", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[112,114,101,115,101,114,118,101,115,83,104,97,112,101,114,71,108,121,112,104,66,111,117,110,100,115,73,110,76,97,121,111,117,116,71,108,121,112,104,82,117,110,115]));
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(Some(Arc::new(Mutex::new(FixedBoundsTextShaper::new()))), None).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[65]), 240 as f64, None, None, None).unwrap()).unwrap();
        let g = ((r.glyph_runs[0usize]).clone().glyphs[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(42, g.id, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 12 as f64 as f64, 2 as f64 as f64).to_string()).as_str()).as_ustr(), ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_nullable_bounds((g.bounds).clone()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20 as f64, g.advance, None).unwrap();
    });
}

#[test]
fn records_fallback_decisions_per_cluster() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsFallbackDecisionsPerCluster", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsFallbackDecisionsPerCluster", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[114,101,99,111,114,100,115,70,97,108,108,98,97,99,107,68,101,99,105,115,105,111,110,115,80,101,114,67,108,117,115,116,101,114]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(Box::new(NoHyphenator::new())), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[25552,26912,8230,8230,69,110,103,108,105,115,104,8212,8212,19990,30028,12290]), 320 as f64, None, None, None).unwrap()).unwrap();
        let mut a = false;
        for _g_index in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("……") && (d.display_text).to_ustring() == UString::from("⋯⋯") && (d.role).to_ustring() == UString::from("CjkPunctuation") && (d.font_key).to_ustring() == UString::from("cjk-primary") {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        a = false;
        for _g_index1 in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("——") && (d.display_text).to_ustring() == UString::from("⸺") {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        a = false;
        for _g_index2 in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().shaping_decisions[usize::try_from(_g_index2).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("——") && (d.display_text).to_ustring() == UString::from("⸺") && d.advance == 32 as f64 && (d.source).to_ustring() == UString::from("Stub") {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        a = false;
        for _g_index3 in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index3).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("English") && (d.role).to_ustring() == UString::from("LatinText") && (d.font_key).to_ustring() == UString::from("latin-primary") {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        let mut c: Option<Cluster> = None;
        for _g_index4 in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (r.clusters[usize::try_from(_g_index4).unwrap_or(0)]).clone();
            if x.text.to_ustring() == UString::from("English") {
                c = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[69,110,103,108,105,115,104]), (c.as_ref().unwrap().text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn combining_marks_stay_in_their_base_shaping_runs() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.combiningMarksStayInTheirBaseShapingRuns", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.combiningMarksStayInTheirBaseShapingRuns", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[99,111,109,98,105,110,105,110,103,77,97,114,107,115,83,116,97,121,73,110,84,104,101,105,114,66,97,115,101,83,104,97,112,105,110,103,82,117,110,115]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(Box::new(NoHyphenator::new())), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[3854,3766,32,7886,823]), 320 as f64, None, None, None).unwrap()).unwrap();
        let mut a = false;
        for _g_index in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().shaping_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("༎ຶ") {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        a = false;
        for _g_index1 in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().shaping_decisions[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("Ỏ̷") {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        a = false;
        for _g_index2 in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().shaping_decisions[usize::try_from(_g_index2).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("ຶ") || (d.source_text).to_ustring() == UString::from("̷") {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!a, None).unwrap();
    });
}

#[test]
fn complex_emoji_graphemes_stay_atomic_across_geometry_only_boundaries() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundaries", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundaries", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[99,111,109,112,108,101,120,69,109,111,106,105,71,114,97,112,104,101,109,101,115,83,116,97,121,65,116,111,109,105,99,65,99,114,111,115,115,71,101,111,109,101,116,114,121,79,110,108,121,66,111,117,110,100,97,114,105,101,115]));
        let text = UString::from("👩🏽‍💻").to_ustring();
        let content = TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![2]), Some(vec![]), Some(vec![]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(LayoutInput::new((content).clone(), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut a: Vec<TextRange> = vec![];
        for _g_index in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.role.to_ustring() == UString::from("Emoji") {
                a.push((d.range).clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,55,41,93]), ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_ranges(&a).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("['")); __s += text.as_ustr(); __s += &(UString::from("']")); __s }).as_str()).as_ustr(), ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_strings(&vec![(((r.debug).clone().shaping_decisions[0usize]).clone().source_text).to_ustring().clone()]).as_ustr(), None).unwrap();
    });
}

#[test]
fn complex_emoji_sequences_reach_the_shaper_as_complete_emoji_ranges() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiSequencesReachTheShaperAsCompleteEmojiRanges", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiSequencesReachTheShaperAsCompleteEmojiRanges", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[99,111,109,112,108,101,120,69,109,111,106,105,83,101,113,117,101,110,99,101,115,82,101,97,99,104,84,104,101,83,104,97,112,101,114,65,115,67,111,109,112,108,101,116,101,69,109,111,106,105,82,97,110,103,101,115]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[21069,55357,56425,55356,57341,8205,55357,56507,21518,55356,56808,55356,56819,19982,49,65039,8419,21644,10084,65039,12290]), 320 as f64, None, None, None).unwrap()).unwrap();
        let mut a: Vec<UString> = vec![];
        for _g_index in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().shaping_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.font_key.to_ustring() == UString::from("symbol-fallback") {
                a.push((d.source_text).to_ustring());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,39,55357,56425,55356,57341,8205,55357,56507,39,44,32,39,55356,56808,55356,56819,39,44,32,39,49,65039,8419,39,44,32,39,10084,65039,39,93]), ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_strings(&a).as_ustr(), None).unwrap();
        let mut b: Vec<UString> = vec![];
        for _g_index1 in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if d.role.to_ustring() == UString::from("Emoji") {
                b.push((d.source_text).to_ustring());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,39,55357,56425,55356,57341,8205,55357,56507,39,44,32,39,55356,56808,55356,56819,39,44,32,39,49,65039,8419,39,44,32,39,10084,65039,39,93]), ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_strings(&b).as_ustr(), None).unwrap();
    });
}

#[test]
fn emoji_role_matrix_separates_supported_sequences_from_adjacent_and_unrelated_text() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.emojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedText", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.emojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedText", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[101,109,111,106,105,82,111,108,101,77,97,116,114,105,120,83,101,112,97,114,97,116,101,115,83,117,112,112,111,114,116,101,100,83,101,113,117,101,110,99,101,115,70,114,111,109,65,100,106,97,99,101,110,116,65,110,100,85,110,114,101,108,97,116,101,100,84,101,120,116]));
        let cases = vec![
    UString::from("a1️⃣").to_ustring(),
    UString::from("1️⃣a").to_ustring(),
    UString::from("a😀中").to_ustring(),
    UString::from("a❤️中").to_ustring(),
    UString::from("a©️中").to_ustring(),
    UString::from("a⌚︎中").to_ustring(),
    UString::from("a1⃣中").to_ustring(),
    UString::from("a👍🏽中").to_ustring(),
    UString::from("a👩🏽‍💻中").to_ustring(),
    UString::from("a🏳️‍⚧️中").to_ustring(),
    UString::from("a🇨🇳中").to_ustring(),
    UString::from("a🏴🏴👧👢👥👮👧🏿中").to_ustring(),
    UString::from("中️").to_ustring(),
    UString::from("a️").to_ustring(),
    UString::from("a⃣中").to_ustring(),
    UString::from("a1️中").to_ustring(),
    UString::from("中🏽").to_ustring(),
    UString::from("a👩‍中").to_ustring(),
    UString::from("中‍👩a").to_ustring(),
];
        let mut mismatches: Vec<UString> = vec![];
        for text in &cases {
            let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(text.as_ustr(), 320 as f64, None, None, None).unwrap()).unwrap();
            if u32::try_from(((r.debug).clone().font_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
                mismatches.push(text.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,93]), ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_strings(&mismatches).as_ustr(), None).unwrap();
    });
}

#[test]
fn source_grapheme_boundaries_do_not_join_zw_j_with_ordinary_text() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.sourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryText", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.sourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryText", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[115,111,117,114,99,101,71,114,97,112,104,101,109,101,66,111,117,110,100,97,114,105,101,115,68,111,78,111,116,74,111,105,110,90,119,74,87,105,116,104,79,114,100,105,110,97,114,121,84,101,120,116]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 3, 4], &ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_grapheme_boundaries(UStr::new(&[55357,56425,8205,20013])).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 2, 4], &ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_grapheme_boundaries(UStr::new(&[20013,8205,55357,56425])).unwrap(), None).unwrap();
    });
}

#[test]
fn records_unicode_emoji_sequence_role_promotions() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsUnicodeEmojiSequenceRolePromotions", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsUnicodeEmojiSequenceRolePromotions", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[114,101,99,111,114,100,115,85,110,105,99,111,100,101,69,109,111,106,105,83,101,113,117,101,110,99,101,82,111,108,101,80,114,111,109,111,116,105,111,110,115]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(UStr::new(&[10084,65039,19982,49,65039,8419]), 320 as f64, None, None, None).unwrap()).unwrap();
        let mut a: Vec<UString> = vec![];
        let mut b: Vec<UString> = vec![];
        for _g_index in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.source.to_ustring() == UString::from("UnicodeEmojiSequenceRolePromotion") {
                a.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("'")); __s += (d.source_text).to_ustring().as_ustr(); __s += &(UString::from("'")); __s }).as_str()));
                b.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("'")); __s += (d.original_role).to_ustring().as_ustr(); __s += &(UString::from("'")); __s }).as_str()));
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,40,39,10084,65039,39,44,32,39,83,121,109,98,111,108,39,41,44,32,40,39,49,65039,8419,39,44,32,39,76,97,116,105,110,84,101,120,116,39,41,93]), ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_pairs(&a, &b).as_ustr(), None).unwrap();
        let mut ok = true;
        for _g_index1 in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if d.source.to_ustring() == UString::from("UnicodeEmojiSequenceRolePromotion") && ((d.overridden_role).to_ustring() != UString::from("Emoji") || (d.reason).to_ustring() != UString::from("EmojiStyleVariationSequence") && (d.reason).to_ustring() != UString::from("KeycapSequence")) {
                ok = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn complex_emoji_graphemes_honor_text_span_style_boundaries() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiGraphemesHonorTextSpanStyleBoundaries", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiGraphemesHonorTextSpanStyleBoundaries", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(UStr::new(&[99,111,109,112,108,101,120,69,109,111,106,105,71,114,97,112,104,101,109,101,115,72,111,110,111,114,84,101,120,116,83,112,97,110,83,116,121,108,101,66,111,117,110,100,97,114,105,101,115]));
        let text = UString::from("👩🏽‍💻").to_ustring();
        let content = TiqianTextContent::new(text.as_ustr(), Some(vec![
    (TextSpan::new(TextRange::new(2u32, ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_text_length(text.as_ustr())).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(700), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![2]), Some(vec![]), Some(vec![]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(LayoutInput::new((content).clone(), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut a: Vec<TextRange> = vec![];
        for _g_index in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.role.to_ustring() == UString::from("Emoji") {
                a.push((d.range).clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,50,41,44,32,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,50,44,32,101,110,100,61,55,41,93]), ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_ranges(&a).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,39,55357,56425,39,44,32,39,55356,57341,8205,55357,56507,39,93]), ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_strings(&vec![
    (((r.debug).clone().shaping_decisions[0usize]).clone().source_text).to_ustring().clone(),
    (((r.debug).clone().shaping_decisions[1usize]).clone().source_text).to_ustring().clone(),
]).as_ustr(), None).unwrap();
    });
}
