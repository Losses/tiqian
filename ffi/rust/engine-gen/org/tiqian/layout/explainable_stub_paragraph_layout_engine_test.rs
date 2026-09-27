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
use std::fmt::Write;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubParagraphLayoutEngineTestSourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryTextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"returnsDebuggableSingleLineResult");
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&"提椠", 240 as f64, None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"greedy", (((r.debug).clone().line_decisions[0usize]).clone().kind).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn records_injected_line_breaker_strategy_in_debug_decisions() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsInjectedLineBreakerStrategyInDebugDecisions", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsInjectedLineBreakerStrategyInDebugDecisions", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"recordsInjectedLineBreakerStrategyInDebugDecisions");
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2),
Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&"提椠", 240 as f64, None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"lookahead", (((r.debug).clone().line_decisions[0usize]).clone().kind).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn mandatory_line_break_clusters_are_zero_width_and_not_shaped() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.mandatoryLineBreakClustersAreZeroWidthAndNotShaped", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.mandatoryLineBreakClustersAreZeroWidthAndNotShaped", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"mandatoryLineBreakClustersAreZeroWidthAndNotShaped");
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2),
Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&concat!("第一行\n",
"第二行"), 240 as f64, None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[0usize].end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::ParagraphEnd), &(r.lines[1usize].end_reason), None).unwrap();
        let mut b: Option<Cluster> = None;
        for _g_index in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if c.text.to_string() == concat!("\n",
"") {
                b = Some(c.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"", (b.as_ref().unwrap().display_text).to_string().as_str(), None).unwrap();
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
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered((b.as_ref().unwrap().range).clone().to_string().as_str(), (((r.debug).clone().mandatory_break_decisions[0usize]).clone().range).clone().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn consecutive_mandatory_line_breaks_create_one_empty_line_box() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.consecutiveMandatoryLineBreaksCreateOneEmptyLineBox", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.consecutiveMandatoryLineBreaksCreateOneEmptyLineBox", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"consecutiveMandatoryLineBreaksCreateOneEmptyLineBox");
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&concat!("第一行\n",
"\n",
"第二行"), 240 as f64, None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[0usize].end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[1usize].end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::ParagraphEnd), &(r.lines[2usize].end_reason), None).unwrap();
        let c = (r.clusters[usize::try_from((r.lines[1usize]).clone().cluster_range.start).unwrap_or(0)]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&concat!("\n",
""), (c.text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"", (c.display_text).to_string().as_str(), None).unwrap();
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
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"singleMandatoryBreakAfterWrappedLineDoesNotCreateEmptyLine");
        let text = concat!("很久以前，曾经有一个名叫小红帽的孩子，生活在大森林的边上，大森林里充满了濒临灭绝的猫头鹰和珍稀植物，如果有人愿意花时间研究它们，就会发现癌症的治疗方法。\n",
"小红帽和一位称为母亲的养育者一起生活").to_string();
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2),
Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(text.as_str(), 1200 as f64, None, None, Some(TextStyle::new(Some(vec![]), Some(48 as f64), Some("zh-Hans".to_string()),
Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)))).unwrap()).unwrap();
        let mut dbg_lines: Vec<String> = vec![];
        for _g_index in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(_g_index).unwrap_or(0)]).clone();
            dbg_lines.push(format!("{}{}{}{}{}{}{}{}",
            l.cluster_range.to_string(),
            " ",
            (l.range).clone().to_string(),
            " ",
            l.end_reason.name(),
            " \"",
            {
    let from = (l.range).clone().start;
    let to = (l.range).clone().end;
    u_string::slice(text.as_str(), i32::from_ne_bytes((from).to_ne_bytes()), i32::from_ne_bytes((to).to_ne_bytes()))
},
            "\""
        ));
        }
        let dbg = { let joined = dbg_lines; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(&(concat!("\n",
""))); } let _ = write!(out, "{}", joined[index]); index += 1; } out };
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) >= 4, None).unwrap();
        let mut no = false;
        for _g_index1 in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if {
    let from = (l.range).clone().start;
    let to = (l.range).clone().end;
    u_string::slice(text.as_str(), i32::from_ne_bytes((from).to_ne_bytes()), i32::from_ne_bytes((to).to_ne_bytes()))
} == concat!("\n",
"") {
                no = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!no, Some((dbg).to_string())).unwrap();
        let mut idx = 0u32;
        for i in 0..u_string::count(text.as_str()) {
            if u_string::at(text.as_str(), i).as_ref().map_or(false, |v| v == &(10)) {
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
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"crlfIsOneMandatoryBreakCluster");
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2),
Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&concat!("甲\r\n",
"乙"), 240 as f64, None, None, None).unwrap()).unwrap();
        let mut b: Option<Cluster> = None;
        for _g_index in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if c.text.to_string() == concat!("\r\n",
"") {
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
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"consecutiveAndTrailingMandatoryBreaksPreserveBlankLines");
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2),
Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&concat!("甲\n",
"\n",
"乙\n",
""), 240 as f64, None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        {
            let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[0usize].end_reason), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[1usize].end_reason), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(r.lines[2usize].end_reason), None).unwrap();
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::ParagraphEnd), &(r.lines[3usize].end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, r.lines[1usize].visual_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=5, end=5)", ((r.lines[3usize]).clone().range).clone().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn mandatory_break_line_is_not_justified() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.mandatoryBreakLineIsNotJustified", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.mandatoryBreakLineIsNotJustified", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"mandatoryBreakLineIsNotJustified");
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(None, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2),
Some(10), Some(20), Some(12.0))))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&concat!("短\n",
"中文中文中文中文中文"), 128 as f64, None, None, None).unwrap()).unwrap();
        let l = (r.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(l.end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(l.natural_width, l.adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((r.debug).clone().justification_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn rejects_shaper_clusters_that_do_not_cover_font_decision_range() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.rejectsShaperClustersThatDoNotCoverFontDecisionRange", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.rejectsShaperClustersThatDoNotCoverFontDecisionRange", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"rejectsShaperClustersThatDoNotCoverFontDecisionRange");
        let f: Arc<dyn Fn() -> Result<(), IllegalStateException> + Send + Sync + 'static> = {  Arc::new(move || {
        ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(Some(Box::new(EmptyTextShaper::new())), None).map_err(|e| IllegalStateException::new(&format!("{:?}",
e)))?.layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&"提椠", 240 as f64, None, None, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?).map_err(|e| IllegalStateException::new(&format!("{:?}",
e)))?;
        Ok(())
}) };
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), (f).clone()).unwrap();
    });
}

#[test]
fn preserves_shaper_glyph_bounds_in_layout_glyph_runs() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.preservesShaperGlyphBoundsInLayoutGlyphRuns", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.preservesShaperGlyphBoundsInLayoutGlyphRuns", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"preservesShaperGlyphBoundsInLayoutGlyphRuns");
        let r = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_engine(Some(Box::new(FixedBoundsTextShaper::new())),
None).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&"A", 240 as f64, None, None, None).unwrap()).unwrap();
        let g = ((r.glyph_runs[0usize]).clone().glyphs[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(42, g.id, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(Rect::new(1 as f64 as f64, i32::from_ne_bytes((4294967286u32).to_ne_bytes()) as f64 as f64, 12 as f64 as f64, 2 as f64 as f64).to_string().as_str(),
ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_nullable_bounds((g.bounds).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20 as f64, g.advance, None).unwrap();
    });
}

#[test]
fn records_fallback_decisions_per_cluster() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsFallbackDecisionsPerCluster", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsFallbackDecisionsPerCluster", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"recordsFallbackDecisionsPerCluster");
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(Box::new(NoHyphenator::new())),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&"提椠……English——世界。", 320 as f64, None, None, None).unwrap()).unwrap();
        let mut a = false;
        for _g_index in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "……" && (d.display_text).to_string() == "⋯⋯" && (d.role).to_string() == "CjkPunctuation" && (d.font_key).to_string() == "cjk-primary" {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        a = false;
        for _g_index1 in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "——" && (d.display_text).to_string() == "⸺" {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        a = false;
        for _g_index2 in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().shaping_decisions[usize::try_from(_g_index2).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "——" && (d.display_text).to_string() == "⸺" && d.advance == 32 as f64 && (d.source).to_string() == "Stub" {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        a = false;
        for _g_index3 in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index3).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "English" && (d.role).to_string() == "LatinText" && (d.font_key).to_string() == "latin-primary" {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        let mut c: Option<Cluster> = None;
        for _g_index4 in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (r.clusters[usize::try_from(_g_index4).unwrap_or(0)]).clone();
            if x.text.to_string() == "English" {
                c = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"English", (c.as_ref().unwrap().text).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn combining_marks_stay_in_their_base_shaping_runs() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.combiningMarksStayInTheirBaseShapingRuns", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.combiningMarksStayInTheirBaseShapingRuns", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"combiningMarksStayInTheirBaseShapingRuns");
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(Box::new(NoHyphenator::new())),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&"༎ຶ Ỏ̷", 320 as f64, None, None, None).unwrap()).unwrap();
        let mut a = false;
        for _g_index in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().shaping_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "༎ຶ" {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        a = false;
        for _g_index1 in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().shaping_decisions[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "Ỏ̷" {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        a = false;
        for _g_index2 in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().shaping_decisions[usize::try_from(_g_index2).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "ຶ" || (d.source_text).to_string() == "̷" {
                a = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!a, None).unwrap();
    });
}

#[test]
fn complex_emoji_graphemes_stay_atomic_across_geometry_only_boundaries() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundaries", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundaries", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"complexEmojiGraphemesStayAtomicAcrossGeometryOnlyBoundaries");
        let text = "👩🏽‍💻".to_string();
        let content = TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![2]), Some(vec![]), Some(vec![]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new((content).clone(), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut a: Vec<TextRange> = vec![];
        for _g_index in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.role.to_string() == "Emoji" {
                a.push((d.range).clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[TextRange(start=0, end=7)]", ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_ranges(&a).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(format!("{}{}{}",
            "['",
            text,
            "']"
        ).as_str(), ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_strings(&vec![(((r.debug).clone().shaping_decisions[0usize]).clone().source_text).to_string().clone()]).as_str(), None).unwrap();
    });
}

#[test]
fn complex_emoji_sequences_reach_the_shaper_as_complete_emoji_ranges() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiSequencesReachTheShaperAsCompleteEmojiRanges", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiSequencesReachTheShaperAsCompleteEmojiRanges", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"complexEmojiSequencesReachTheShaperAsCompleteEmojiRanges");
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&"前👩🏽‍💻后🇨🇳与1️⃣和❤️。", 320 as f64, None, None, None).unwrap()).unwrap();
        let mut a: Vec<String> = vec![];
        for _g_index in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().shaping_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.font_key.to_string() == "symbol-fallback" {
                a.push((d.source_text).to_string());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"['👩🏽‍💻', '🇨🇳', '1️⃣', '❤️']", ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_strings(&a).as_str(), None).unwrap();
        let mut b: Vec<String> = vec![];
        for _g_index1 in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if d.role.to_string() == "Emoji" {
                b.push((d.source_text).to_string());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"['👩🏽‍💻', '🇨🇳', '1️⃣', '❤️']", ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_strings(&b).as_str(), None).unwrap();
    });
}

#[test]
fn emoji_role_matrix_separates_supported_sequences_from_adjacent_and_unrelated_text() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.emojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedText",
"org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.emojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedText", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"emojiRoleMatrixSeparatesSupportedSequencesFromAdjacentAndUnrelatedText");
        let cases = vec![
    "a1️⃣".to_string(),
    "1️⃣a".to_string(),
    "a😀中".to_string(),
    "a❤️中".to_string(),
    "a©️中".to_string(),
    "a⌚︎中".to_string(),
    "a1⃣中".to_string(),
    "a👍🏽中".to_string(),
    "a👩🏽‍💻中".to_string(),
    "a🏳️‍⚧️中".to_string(),
    "a🇨🇳中".to_string(),
    "a🏴🏴👧👢👥👮👧🏿中".to_string(),
    "中️".to_string(),
    "a️".to_string(),
    "a⃣中".to_string(),
    "a1️中".to_string(),
    "中🏽".to_string(),
    "a👩‍中".to_string(),
    "中‍👩a".to_string(),
];
        let mut mismatches: Vec<String> = vec![];
        for text in &cases {
            let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(text.as_str(), 320 as f64, None, None, None).unwrap()).unwrap();
            if u32::try_from(((r.debug).clone().font_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
                mismatches.push(text.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[]", ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_strings(&mismatches).as_str(), None).unwrap();
    });
}

#[test]
fn source_grapheme_boundaries_do_not_join_zw_j_with_ordinary_text() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.sourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryText", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.sourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryText", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"sourceGraphemeBoundariesDoNotJoinZwJWithOrdinaryText");
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 3, 4], &ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_grapheme_boundaries(&"👩‍中").unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 2, 4], &ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_grapheme_boundaries(&"中‍👩").unwrap(), None).unwrap();
    });
}

#[test]
fn records_unicode_emoji_sequence_role_promotions() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsUnicodeEmojiSequenceRolePromotions", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.recordsUnicodeEmojiSequenceRolePromotions", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"recordsUnicodeEmojiSequenceRolePromotions");
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_input(&"❤️与1️⃣", 320 as f64, None, None, None).unwrap()).unwrap();
        let mut a: Vec<String> = vec![];
        let mut b: Vec<String> = vec![];
        for _g_index in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.source.to_string() == "UnicodeEmojiSequenceRolePromotion" {
                a.push(format!("{}{}{}",
            "'",
            (d.source_text).to_string(),
            "'"
        ));
                b.push(format!("{}{}{}",
            "'",
            (d.original_role).to_string(),
            "'"
        ));
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[('❤️', 'Symbol'), ('1️⃣', 'LatinText')]", ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_pairs(&a, &b).as_str(), None).unwrap();
        let mut ok = true;
        for _g_index1 in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if d.source.to_string() == "UnicodeEmojiSequenceRolePromotion" && ((d.overridden_role).to_string() != "Emoji" || (d.reason).to_string() != "EmojiStyleVariationSequence" && (d.reason).to_string() != "KeycapSequence") {
                ok = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn complex_emoji_graphemes_honor_text_span_style_boundaries() {
    testlib::run("org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiGraphemesHonorTextSpanStyleBoundaries", "org.tiqian.layout.ExplainableStubParagraphLayoutEngineTest.complexEmojiGraphemesHonorTextSpanStyleBoundaries", || {
        let _ = ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_start(&"complexEmojiGraphemesHonorTextSpanStyleBoundaries");
        let text = "👩🏽‍💻".to_string();
        let content = TiqianTextContent::new(text.as_str(), Some(vec![
    (TextSpan::new(TextRange::new(2u32, ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_text_length(text.as_str())).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(700), Some(false),
Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![2]), Some(vec![]), Some(vec![]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new((content).clone(), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut a: Vec<TextRange> = vec![];
        for _g_index in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if d.role.to_string() == "Emoji" {
                a.push((d.range).clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[TextRange(start=0, end=2), TextRange(start=2, end=7)]", ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_ranges(&a).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"['👩', '🏽‍💻']", ExplainableStubParagraphLayoutEngineTestSupport::explainable_stub_paragraph_layout_engine_test_support_render_strings(&vec![
    (((r.debug).clone().shaping_decisions[0usize]).clone().source_text).to_string().clone(),
    (((r.debug).clone().shaping_decisions[1usize]).clone().source_text).to_string().clone(),
]).as_str(), None).unwrap();
    });
}
