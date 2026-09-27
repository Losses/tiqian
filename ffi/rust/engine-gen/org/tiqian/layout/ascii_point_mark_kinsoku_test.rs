#![cfg(test)]

use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::layout::ascii_point_mark_kinsoku_test_support::AsciiPointMarkKinsokuTestSupport;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::string_tools;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestStyledPointMarkRunCanExtendOneImpossibleMeasureHangFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestReportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiCommaFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestPointMarkSplitFromAnOverlongLatinTokenStillCannotStartALineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestPointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffixFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestMandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffixFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestLineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallbackFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestLeadingPointMarkRunIsSplitFromFollowingLatinTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestKinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestImpossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestFirstLineIndentUsesTheSameImpossibleMeasureFallbackFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestContextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroupFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestCompressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallbackFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestCjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatinFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestAuthoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsokuFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestAdjacentImpossibleGroupsDoNotShareHangProvenanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault) -> Self {
        match value {
            AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AsciiPointMarkKinsokuTestLatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn cjk_attached_ascii_point_marks_cannot_start_wrapped_lines_and_stay_latin() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.cjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatin", "org.tiqian.layout.AsciiPointMarkKinsokuTest.cjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatin", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[99,106,107,65,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,115,67,97,110,110,111,116,83,116,97,114,116,87,114,97,112,112,101,100,76,105,110,101,115,65,110,100,83,116,97,121,76,97,116,105,110]));
        for mi in 0..6 {
            let mark = (vec![
    UString::from(",").to_ustring(),
    UString::from(".").to_ustring(),
    UString::from(":").to_ustring(),
    UString::from(";").to_ustring(),
    UString::from("!").to_ustring(),
    UString::from("?").to_ustring(),
][usize::try_from(mi).unwrap_or(0)]).clone();
            {
                let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
                for b in &_g1 {
                    let text = { let mut __s = UString::new(); __s += &(UString::from("中文中文")); __s += mark.as_ustr(); __s += &(UString::from("中文")); __s };
                    let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(text.as_ustr(), 64 as f64, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                    let ls = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), text.as_ustr());
                    let mut starts = false;
                    for i in 0..match u32::try_from(ls.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        if ls[usize::try_from(i).unwrap_or(0)].clone().starts_with(&mark) {
                            starts = true;
                        }
                    }
                    let _ = TracedAssertions::traced_assertions_assert_true(!starts, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" placed '")); __s += mark.as_ustr(); __s += &(UString::from("' at line start: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&ls).as_ustr(); __s }).as_str()))).unwrap();
                    let p = (AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_clusters_with_text((r).clone(), mark.as_ustr())[0usize]).clone();
                    let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (p.font_key).to_ustring().as_ustr(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" '")); __s += mark.as_ustr(); __s += &(UString::from("' face")); __s }).as_str()))).unwrap();
                    let f = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_font_decision((r).clone(), (p.range).clone());
                    let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[76,97,116,105,110,84,101,120,116]), match &(f) { None => UString::from("<no font decision>"), Some(__option2) => ((__option2.role).to_ustring()).clone() }.as_ustr(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" '")); __s += mark.as_ustr(); __s += &(UString::from("' role")); __s }).as_str()))).unwrap();
                    let _ = TracedAssertions::traced_assertions_assert_true(!AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_punctuation((r).clone(), (p.range).clone()), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" '")); __s += mark.as_ustr(); __s += &(UString::from("' must not enter CJK punctuation geometry")); __s }).as_str()))).unwrap();
                    let c = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_contextual((r).clone(), Some((p.range).clone()));
                    let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[76,105,110,101,83,116,97,114,116]), match &(c) { None => UString::from("<no contextual kinsoku decision>"), Some(__option5) => ((__option5.forbidden_position).to_ustring()).clone() }.as_ustr(), None).unwrap();
                    let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117]), match &(c) { None => UString::from("<no contextual kinsoku decision>"), Some(__option8) => ((__option8.reason).to_ustring()).clone() }.as_ustr(), None).unwrap();
                }
            }
        }
    });
}

#[test]
fn leading_point_mark_run_is_split_from_following_latin_text() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.leadingPointMarkRunIsSplitFromFollowingLatinText", "org.tiqian.layout.AsciiPointMarkKinsokuTest.leadingPointMarkRunIsSplitFromFollowingLatinText", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[108,101,97,100,105,110,103,80,111,105,110,116,77,97,114,107,82,117,110,73,115,83,112,108,105,116,70,114,111,109,70,111,108,108,111,119,105,110,103,76,97,116,105,110,84,101,120,116]));
        let text = UString::from("中文,anyway继续").to_ustring();
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(text.as_ustr(), 64 as f64, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                let ls = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), text.as_ustr());
                let mut st = false;
                for i in 0..match u32::try_from(ls.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if ls[usize::try_from(i).unwrap_or(0)].clone().starts_with(&UString::from(",")) {
                        st = true;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(!st, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&ls).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_true(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_cluster((r).clone(), UStr::new(&[44])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" comma cluster: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_clusters(&r.clusters).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_true(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_font_source((r).clone(), UStr::new(&[97,110,121,119,97,121])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" Latin decision: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_fonts(&(r.debug).clone().font_decisions).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_false(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_cluster((r).clone(), UStr::new(&[44,97,110,121,119,97,121])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" bound the word to the comma")); __s }).as_str()))).unwrap();
            }
        }
    });
}

#[test]
fn latin_tokens_and_ambiguous_ascii_characters_keep_existing_segmentation() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.LatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentation", "org.tiqian.layout.AsciiPointMarkKinsokuTest.LatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentation", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[76,97,116,105,110,84,111,107,101,110,115,65,110,100,65,109,98,105,103,117,111,117,115,65,115,99,105,105,67,104,97,114,97,99,116,101,114,115,75,101,101,112,69,120,105,115,116,105,110,103,83,101,103,109,101,110,116,97,116,105,111,110]));
        let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[102,111,111,44,98,97,114,32,49,44,50,51,52,32,53,48,37,32,34,113,117,111,116,101,100,34]), 1000 as f64, Box::new(GreedyLineBreaker::new(None, None, None, None)), None, None, None, None, None, None).unwrap();
        {
            {
                let x = UString::from("foo,bar").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_cluster((r).clone(), x.as_ustr()), None).unwrap();
            }
            {
                let x = UString::from("1,234").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_cluster((r).clone(), x.as_ustr()), None).unwrap();
            }
            {
                let x = UString::from("50%").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_cluster((r).clone(), x.as_ustr()), None).unwrap();
            }
            {
                let x = UString::from("\"quoted\"").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_cluster((r).clone(), x.as_ustr()), None).unwrap();
            }
        }
        let q = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_forbidden_for((r).clone(), UStr::new(&[34,113,117,111,116,101,100,34]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("LineStart").to_ustring(), UString::from("LineEnd").to_ustring()], &q, None).unwrap();
    });
}

#[test]
fn point_mark_split_from_an_overlong_latin_token_still_cannot_start_a_line() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.pointMarkSplitFromAnOverlongLatinTokenStillCannotStartALine", "org.tiqian.layout.AsciiPointMarkKinsokuTest.pointMarkSplitFromAnOverlongLatinTokenStillCannotStartALine", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[112,111,105,110,116,77,97,114,107,83,112,108,105,116,70,114,111,109,65,110,79,118,101,114,108,111,110,103,76,97,116,105,110,84,111,107,101,110,83,116,105,108,108,67,97,110,110,111,116,83,116,97,114,116,65,76,105,110,101]));
        {
            {
                let w = 32.0f64;
                {
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
                    for b in &_g1 {
                        let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[97,110,121,119,97,121,44,20320]), w, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                        let mut x = false;
                        {
                            let mut _g = 0u32;
                            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[97,110,121,119,97,121,44,20320]));
                            while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                                let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if s.starts_with(&UString::from(",")) {
                                    x = true;
                                }
                            }
                        }
                        let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(w)); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[97,110,121,119,97,121,44,20320]))).as_ustr(); __s }).as_str()))).unwrap();
                    }
                }
            }
            {
                let w = 36.0f64;
                {
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
                    for b in &_g1 {
                        let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[97,110,121,119,97,121,44,20320]), w, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                        let mut x = false;
                        {
                            let mut _g = 0u32;
                            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[97,110,121,119,97,121,44,20320]));
                            while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                                let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if s.starts_with(&UString::from(",")) {
                                    x = true;
                                }
                            }
                        }
                        let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(w)); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[97,110,121,119,97,121,44,20320]))).as_ustr(); __s }).as_str()))).unwrap();
                    }
                }
            }
            {
                let w = 40.0f64;
                {
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
                    for b in &_g1 {
                        let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[97,110,121,119,97,121,44,20320]), w, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                        let mut x = false;
                        {
                            let mut _g = 0u32;
                            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[97,110,121,119,97,121,44,20320]));
                            while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                                let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if s.starts_with(&UString::from(",")) {
                                    x = true;
                                }
                            }
                        }
                        let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(w)); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[97,110,121,119,97,121,44,20320]))).as_ustr(); __s }).as_str()))).unwrap();
                    }
                }
            }
            {
                let w = 48.0f64;
                {
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
                    for b in &_g1 {
                        let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[97,110,121,119,97,121,44,20320]), w, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                        let mut x = false;
                        {
                            let mut _g = 0u32;
                            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[97,110,121,119,97,121,44,20320]));
                            while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                                let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if s.starts_with(&UString::from(",")) {
                                    x = true;
                                }
                            }
                        }
                        let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(w)); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[97,110,121,119,97,121,44,20320]))).as_ustr(); __s }).as_str()))).unwrap();
                    }
                }
            }
        }
    });
}

#[test]
fn point_mark_exposed_by_a_second_stage_latin_cut_is_split_from_its_suffix() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.pointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffix", "org.tiqian.layout.AsciiPointMarkKinsokuTest.pointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffix", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[112,111,105,110,116,77,97,114,107,69,120,112,111,115,101,100,66,121,65,83,101,99,111,110,100,83,116,97,103,101,76,97,116,105,110,67,117,116,73,115,83,112,108,105,116,70,114,111,109,73,116,115,83,117,102,102,105,120]));
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[46,44,65,20013]), 32 as f64, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                let mut x = false;
                {
                    let mut _g = 0u32;
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[46,44,65,20013]));
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if s.starts_with(&UString::from(",")) {
                            x = true;
                        }
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[46,44,65,20013]))).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[46,44]), (AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[46,44,65,20013]))[0usize]).clone().as_ustr(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" should keep the avoidable pair together")); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_true(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_cluster((r).clone(), UStr::new(&[44])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" clusters: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_clusters(&r.clusters).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_false(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_cluster((r).clone(), UStr::new(&[44,65])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" kept the post-cut suffix attached")); __s }).as_str()))).unwrap();
            }
        }
    });
}

#[test]
fn impossible_measure_hangs_the_point_mark_instead_of_leaving_it_at_line_start() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.impossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStart", "org.tiqian.layout.AsciiPointMarkKinsokuTest.impossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStart", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[105,109,112,111,115,115,105,98,108,101,77,101,97,115,117,114,101,72,97,110,103,115,84,104,101,80,111,105,110,116,77,97,114,107,73,110,115,116,101,97,100,79,102,76,101,97,118,105,110,103,73,116,65,116,76,105,110,101,83,116,97,114,116]));
        {
            let _g1 = vec![1.0f64, 8.0f64, 15.0f64, 23.0f64, 31.0f64];
            for &w in &_g1 {
                {
                    let mut _g = 0u32;
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let b = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[20013,44,25991]), w, b.breaker.clone(), None, None, None, None, None, None).unwrap();
                        let mut x = false;
                        {
                            let mut _g = 0u32;
                            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,44,25991]));
                            while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                                let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if s.starts_with(&UString::from(",")) {
                                    x = true;
                                }
                            }
                        }
                        let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(w)); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,44,25991]))).as_ustr(); __s }).as_str()))).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,73,109,112,111,115,115,105,98,108,101,77,101,97,115,117,114,101,72,97,110,103]), ((AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_contextual((r).clone(), None).as_ref().unwrap().impossible_measure_fallback).clone()).as_deref().unwrap_or(UStr::new(&[])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(w)); __s += &(UString::from(" fallback")); __s }).as_str()))).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_true(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_repair((r).clone(), UStr::new(&[72,97,110,103])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(w)); __s += &(UString::from(" repairs: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_line_decisions(&(r.debug).clone().line_decisions).as_ustr(); __s }).as_str()))).unwrap();
                    }
                }
            }
        }
    });
}

#[test]
fn first_line_indent_uses_the_same_impossible_measure_fallback() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.firstLineIndentUsesTheSameImpossibleMeasureFallback", "org.tiqian.layout.AsciiPointMarkKinsokuTest.firstLineIndentUsesTheSameImpossibleMeasureFallback", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[102,105,114,115,116,76,105,110,101,73,110,100,101,110,116,85,115,101,115,84,104,101,83,97,109,101,73,109,112,111,115,115,105,98,108,101,77,101,97,115,117,114,101,70,97,108,108,98,97,99,107]));
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout_without_explicit_indent(UStr::new(&[20013,44,25991]), 32 as f64, (b.breaker).clone()).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_true(!((AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,44,25991]))[0usize]).clone()).starts_with(&UString::from(",")), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,44,25991]))).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,73,109,112,111,115,115,105,98,108,101,77,101,97,115,117,114,101,72,97,110,103]), ((AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_contextual((r).clone(), None).as_ref().unwrap().impossible_measure_fallback).clone()).as_deref().unwrap_or(UStr::new(&[])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" adaptive first-line indent")); __s }).as_str()))).unwrap();
            }
        }
    });
}

#[test]
fn line_break_geometry_includes_bopomofo_spread_when_choosing_the_fallback() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.lineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallback", "org.tiqian.layout.AsciiPointMarkKinsokuTest.lineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallback", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[108,105,110,101,66,114,101,97,107,71,101,111,109,101,116,114,121,73,110,99,108,117,100,101,115,66,111,112,111,109,111,102,111,83,112,114,101,97,100,87,104,101,110,67,104,111,111,115,105,110,103,84,104,101,70,97,108,108,98,97,99,107]));
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[20013,44,25991]), 32 as f64, (b.breaker).clone(), None, None, None, Some(vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[12549])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
]), None, None).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_true(!((AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,44,25991]))[0usize]).clone()).starts_with(&UString::from(",")), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,44,25991]))).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,73,109,112,111,115,115,105,98,108,101,77,101,97,115,117,114,101,72,97,110,103]), ((AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_contextual((r).clone(), None).as_ref().unwrap().impossible_measure_fallback).clone()).as_deref().unwrap_or(UStr::new(&[])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" must use post-spread line-break geometry")); __s }).as_str()))).unwrap();
            }
        }
    });
}

#[test]
fn styled_point_mark_run_can_extend_one_impossible_measure_hang() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.styledPointMarkRunCanExtendOneImpossibleMeasureHang", "org.tiqian.layout.AsciiPointMarkKinsokuTest.styledPointMarkRunCanExtendOneImpossibleMeasureHang", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[115,116,121,108,101,100,80,111,105,110,116,77,97,114,107,82,117,110,67,97,110,69,120,116,101,110,100,79,110,101,73,109,112,111,115,115,105,98,108,101,77,101,97,115,117,114,101,72,97,110,103]));
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[20013,33,44,25991]), 15 as f64, (b.breaker).clone(), None, None, None, None, Some(vec![
    (TextSpan::new(TextRange::new(2u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(700), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), None).unwrap();
                let mut x = false;
                {
                    let mut _g = 0u32;
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,33,44,25991]));
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if s.starts_with(&UString::from("!")) || (s).starts_with(&UString::from(",")) {
                            x = true;
                        }
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,33,44,25991]))).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_true(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_cluster((r).clone(), UStr::new(&[33])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" exclamation cluster")); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_true(AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_cluster((r).clone(), UStr::new(&[44])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" comma cluster")); __s }).as_str()))).unwrap();
                let mut n = 0u32;
                for i in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if r.debug.clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)].clone().impossible_measure_fallback.as_ref().map_or(false, |v| v == &(UString::from("AttachedAsciiPointMarkImpossibleMeasureHang").to_ustring())) {
                        n = u32::wrapping_add(n, 1);
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_equals_int(2, n, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" applied fallbacks: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_contextual(&(r.debug).clone().contextual_kinsoku_decisions).as_ustr(); __s }).as_str()))).unwrap();
                let mut hanging_advance = 0.0f64;
                for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if r.lines[usize::try_from(i).unwrap_or(0)].hanging_punctuation_advance > (0 as f64) {
                        hanging_advance = r.lines[usize::try_from(i).unwrap_or(0)].hanging_punctuation_advance;
                    }
                }
                let mut expected_advance = 0.0f64;
                for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == UString::from("!") || ((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring() == UString::from(",") {
                        expected_advance += r.clusters[usize::try_from(i).unwrap_or(0)].advance;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_equals_float(expected_advance, hanging_advance, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" run advance")); __s }).as_str()))).unwrap();
            }
        }
    });
}

#[test]
fn contextual_run_can_extend_a_profile_hang_only_within_the_same_protected_group() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.contextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroup", "org.tiqian.layout.AsciiPointMarkKinsokuTest.contextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroup", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[99,111,110,116,101,120,116,117,97,108,82,117,110,67,97,110,69,120,116,101,110,100,65,80,114,111,102,105,108,101,72,97,110,103,79,110,108,121,87,105,116,104,105,110,84,104,101,83,97,109,101,80,114,111,116,101,99,116,101,100,71,114,111,117,112]));
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[20013,65292,44,25991]), 15 as f64, (b.breaker).clone(), None, Some(HangingPunctuationStyle::PauseStops), None, None, None, None).unwrap();
                let mut x = false;
                {
                    let mut _g = 0u32;
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,65292,44,25991]));
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if s.starts_with(&UString::from(",")) || (s).starts_with(&UString::from("，")) {
                            x = true;
                        }
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,65292,44,25991]))).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,73,109,112,111,115,115,105,98,108,101,77,101,97,115,117,114,101,72,97,110,103]), ((AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_contextual((r).clone(), None).as_ref().unwrap().impossible_measure_fallback).clone()).as_deref().unwrap_or(UStr::new(&[])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" contextual extension")); __s }).as_str()))).unwrap();
            }
        }
    });
}

#[test]
fn adjacent_impossible_groups_do_not_share_hang_provenance() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.adjacentImpossibleGroupsDoNotShareHangProvenance", "org.tiqian.layout.AsciiPointMarkKinsokuTest.adjacentImpossibleGroupsDoNotShareHangProvenance", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[97,100,106,97,99,101,110,116,73,109,112,111,115,115,105,98,108,101,71,114,111,117,112,115,68,111,78,111,116,83,104,97,114,101,72,97,110,103,80,114,111,118,101,110,97,110,99,101]));
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[20013,33,65292,63]), 15 as f64, (b.breaker).clone(), None, Some(HangingPunctuationStyle::PauseStops), None, None, None, None).unwrap();
                let mut n = 0u32;
                for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if r.lines[usize::try_from(i).unwrap_or(0)].hanging_punctuation_advance > (0 as f64) {
                        n = u32::wrapping_add(n, 1);
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_equals_int(2, n, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" must keep the adjacent protected groups separate: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_lines(&r.lines).as_ustr(); __s }).as_str()))).unwrap();
                let mut x = false;
                {
                    let mut _g = 0u32;
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,33,65292,63]));
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if s.starts_with(&UString::from("!")) || (s).starts_with(&UString::from("?")) {
                            x = true;
                        }
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,33,65292,63]))).as_ustr(); __s }).as_str()))).unwrap();
            }
        }
    });
}

#[test]
fn compressed_closing_and_point_mark_pair_does_not_report_an_unused_hang_fallback() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.compressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallback", "org.tiqian.layout.AsciiPointMarkKinsokuTest.compressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallback", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[99,111,109,112,114,101,115,115,101,100,67,108,111,115,105,110,103,65,110,100,80,111,105,110,116,77,97,114,107,80,97,105,114,68,111,101,115,78,111,116,82,101,112,111,114,116,65,110,85,110,117,115,101,100,72,97,110,103,70,97,108,108,98,97,99,107]));
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[65289,44,25991]), 24 as f64, (b.breaker).clone(), None, None, None, None, None, Some(LineLengthGrid::new(Some(false), None))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65289,44]), (AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[65289,44,25991]))[0usize]).clone().as_ustr(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[65289,44,25991]))).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_true(!AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_has_repair((r).clone(), UStr::new(&[72,97,110,103])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" repairs: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_line_decisions(&(r.debug).clone().line_decisions).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(None.clone(), (AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_contextual((r).clone(), None).as_ref().unwrap().impossible_measure_fallback).clone().clone(), None).unwrap();
            }
        }
    });
}

#[test]
fn kinsoku_none_disables_clreq_but_keeps_the_uax14_ascii_point_mark_boundary() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.kinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundary", "org.tiqian.layout.AsciiPointMarkKinsokuTest.kinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundary", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[107,105,110,115,111,107,117,78,111,110,101,68,105,115,97,98,108,101,115,67,108,114,101,113,66,117,116,75,101,101,112,115,84,104,101,85,97,120,49,52,65,115,99,105,105,80,111,105,110,116,77,97,114,107,66,111,117,110,100,97,114,121]));
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[20013,25991,20013,25991,44,20013,25991]), 64 as f64, (b.breaker).clone(), Some(KinsokuLevel::None), None, None, None, None, None).unwrap();
                let mut x = false;
                {
                    let mut _g = 0u32;
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,25991,20013,25991,44,20013,25991]));
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if s.starts_with(&UString::from(",")) {
                            x = true;
                        }
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,25991,20013,25991,44,20013,25991]))).as_ustr(); __s }).as_str()))).unwrap();
                let d = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_contextual_by_text((r).clone(), UStr::new(&[44]));
                let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[85,97,120,49,52,87,101,115,116,101,114,110,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,76,66,49,53,100]), match &(d) { None => UString::from("<no contextual kinsoku decision>"), Some(__option11) => ((__option11.reason).to_ustring()).clone() }.as_ustr(), Some((b.label).to_ustring())).unwrap();
            }
        }
    });
}

#[test]
fn authored_whitespace_and_mandatory_break_do_not_create_contextual_kinsoku() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.authoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsoku", "org.tiqian.layout.AsciiPointMarkKinsokuTest.authoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsoku", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[97,117,116,104,111,114,101,100,87,104,105,116,101,115,112,97,99,101,65,110,100,77,97,110,100,97,116,111,114,121,66,114,101,97,107,68,111,78,111,116,67,114,101,97,116,101,67,111,110,116,101,120,116,117,97,108,75,105,110,115,111,107,117]));
        {
            {
                let text = UString::from(concat!("中\n",
",文")).to_ustring();
                {
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
                    for b in &_g1 {
                        let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(text.as_ustr(), 1000 as f64, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((r.debug).clone().contextual_kinsoku_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" text=")); __s += string_tools::StringTools::string_tools_replace(text.as_ustr(), UStr::new(&[10]), UStr::new(&[92,110])).as_ustr(); __s += &(UString::from(" decisions=")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_contextual(&(r.debug).clone().contextual_kinsoku_decisions).as_ustr(); __s }).as_str()))).unwrap();
                    }
                }
            }
            {
                let text = UString::from(",中文").to_ustring();
                {
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
                    for b in &_g1 {
                        let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(text.as_ustr(), 1000 as f64, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((r.debug).clone().contextual_kinsoku_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" text=")); __s += string_tools::StringTools::string_tools_replace(text.as_ustr(), UStr::new(&[10]), UStr::new(&[92,110])).as_ustr(); __s += &(UString::from(" decisions=")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_contextual(&(r.debug).clone().contextual_kinsoku_decisions).as_ustr(); __s }).as_str()))).unwrap();
                    }
                }
            }
        }
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[20013,32,44,25991]), 1000 as f64, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                let d = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_contextual_by_text((r).clone(), UStr::new(&[44]));
                let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[85,97,120,49,52,87,101,115,116,101,114,110,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,76,66,49,53,100]), match &(d) { None => UString::from("<no contextual kinsoku decision>"), Some(__option14) => ((__option14.reason).to_ustring()).clone() }.as_ustr(), Some((b.label).to_ustring())).unwrap();
            }
        }
    });
}

#[test]
fn mandatory_break_control_after_a_hung_point_mark_stays_in_the_trailing_suffix() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.mandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffix", "org.tiqian.layout.AsciiPointMarkKinsokuTest.mandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffix", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[109,97,110,100,97,116,111,114,121,66,114,101,97,107,67,111,110,116,114,111,108,65,102,116,101,114,65,72,117,110,103,80,111,105,110,116,77,97,114,107,83,116,97,121,115,73,110,84,104,101,84,114,97,105,108,105,110,103,83,117,102,102,105,120]));
        {
            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
            for b in &_g1 {
                let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UStr::new(&[20013,44,10,25991]), 15 as f64, (b.breaker).clone(), None, None, None, None, None, None).unwrap();
                let mut x = false;
                {
                    let mut _g = 0u32;
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,44,10,25991]));
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if s.starts_with(&UString::from(",")) {
                            x = true;
                        }
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" lines: ")); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_render_strings(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UStr::new(&[20013,44,10,25991]))).as_ustr(); __s }).as_str()))).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,73,109,112,111,115,115,105,98,108,101,77,101,97,115,117,114,101,72,97,110,103]), ((AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_contextual((r).clone(), None).as_ref().unwrap().impossible_measure_fallback).clone()).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
            }
        }
    });
}

#[test]
fn reported_real_world_paragraph_never_wraps_directly_before_an_ascii_comma() {
    testlib::run("org.tiqian.layout.AsciiPointMarkKinsokuTest.reportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiComma", "org.tiqian.layout.AsciiPointMarkKinsokuTest.reportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiComma", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,84,101,115,116])));
        t.section(UStr::new(&[114,101,112,111,114,116,101,100,82,101,97,108,87,111,114,108,100,80,97,114,97,103,114,97,112,104,78,101,118,101,114,87,114,97,112,115,68,105,114,101,99,116,108,121,66,101,102,111,114,101,65,110,65,115,99,105,105,67,111,109,109,97]));
        {
            let _g1 = vec![36.0f64, 40.0f64, 160.0f64, 240.0f64, 320.0f64];
            for &w in &_g1 {
                {
                    let mut _g = 0u32;
                    let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_breakers();
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let b = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        let r = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_layout(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("对于你冒犯的断言不敢苟同,你以一种理所当然的语气声明\"明显的已经越过了人际尊重的基本门槛\",")); __s += &(UString::from("如此注重逻辑推导的作者居然会对论断的前提条件如此宽松以至于不留回旋余地?当然不是,在回复的一开头,")); __s += &(UString::from("聪明的作者就已经强调了自己作为被冒犯者有权力定义自己的感受,当然有权力!,但是这种感受是否可以无限扩展到")); __s += &(UString::from("\"人际尊重的基本门槛\",还是值得商榷的,逻辑严谨如你岂能放过如此基础的逻辑漏洞?也许我们可以采用更加自洽的解释,")); __s += &(UString::from(" 这种愤怒来源于作者遭到否定是的第一反应,一篇让你耿耿于怀三年的留言需要你通过反复打磨的语言和极致构思的反讽,")); __s += &(UString::from("只为了冷嘲热讽一个逻辑甚至不大通顺的留言.\" 我一定要用最严密的逻辑反驳回去,这是关乎我尊严的网络论战\",也许你心里确实这么想,")); __s += &(UString::from(" 可是承认这件事情在你的内心是一件丢脸的事情,倘若承认了自己的三年的耿耿于怀, 就等同于认可自己与对方与自己处于同一水平对话,")); __s += &(UString::from(" “居然要和一个沙文主义在相提并论, 这怎么可以接受”.但事实上,如此自视清高反而令人啼笑皆非,如果你可以大方承认自己的傲慢,")); __s += &(UString::from("我大可因为你的心胸宽广\"对你致上最高的敬意\".别急着找我的逻辑漏洞,因为我也会大大方方的承认我就是在玩,")); __s += &(UString::from("我乐意这种伪装成思辨的娱乐,这比大部分辩论赛有意思多了 .anyway,你完全有机会在一开始就讲清楚自己愤怒的来源,")); __s += &(UString::from("而不是强行带上无所谓的面具却又如此的用力过猛,希望下一次你可以清晰表述,就像你自己提到的那样,")); __s += &(UString::from("不要\"把解读的权利拱手让给对方\"")); __s }).as_str()).as_ustr(), w, b.breaker.clone(), None, None, None, None, None, None).unwrap();
                        let mut x = false;
                        {
                            let mut _g = 0u32;
                            let _g1 = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("对于你冒犯的断言不敢苟同,你以一种理所当然的语气声明\"明显的已经越过了人际尊重的基本门槛\",")); __s += &(UString::from("如此注重逻辑推导的作者居然会对论断的前提条件如此宽松以至于不留回旋余地?当然不是,在回复的一开头,")); __s += &(UString::from("聪明的作者就已经强调了自己作为被冒犯者有权力定义自己的感受,当然有权力!,但是这种感受是否可以无限扩展到")); __s += &(UString::from("\"人际尊重的基本门槛\",还是值得商榷的,逻辑严谨如你岂能放过如此基础的逻辑漏洞?也许我们可以采用更加自洽的解释,")); __s += &(UString::from(" 这种愤怒来源于作者遭到否定是的第一反应,一篇让你耿耿于怀三年的留言需要你通过反复打磨的语言和极致构思的反讽,")); __s += &(UString::from("只为了冷嘲热讽一个逻辑甚至不大通顺的留言.\" 我一定要用最严密的逻辑反驳回去,这是关乎我尊严的网络论战\",也许你心里确实这么想,")); __s += &(UString::from(" 可是承认这件事情在你的内心是一件丢脸的事情,倘若承认了自己的三年的耿耿于怀, 就等同于认可自己与对方与自己处于同一水平对话,")); __s += &(UString::from(" “居然要和一个沙文主义在相提并论, 这怎么可以接受”.但事实上,如此自视清高反而令人啼笑皆非,如果你可以大方承认自己的傲慢,")); __s += &(UString::from("我大可因为你的心胸宽广\"对你致上最高的敬意\".别急着找我的逻辑漏洞,因为我也会大大方方的承认我就是在玩,")); __s += &(UString::from("我乐意这种伪装成思辨的娱乐,这比大部分辩论赛有意思多了 .anyway,你完全有机会在一开始就讲清楚自己愤怒的来源,")); __s += &(UString::from("而不是强行带上无所谓的面具却又如此的用力过猛,希望下一次你可以清晰表述,就像你自己提到的那样,")); __s += &(UString::from("不要\"把解读的权利拱手让给对方\"")); __s }).as_str()).as_ustr());
                            while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                                let s = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if s.starts_with(&UString::from(",")) {
                                    x = true;
                                }
                            }
                        }
                        let _ = TracedAssertions::traced_assertions_assert_true(!x, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(w)); __s += &(UString::from(concat!(" still starts a line with comma:\n",
""))); __s += AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_join_lines(&AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_line_texts((r).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("对于你冒犯的断言不敢苟同,你以一种理所当然的语气声明\"明显的已经越过了人际尊重的基本门槛\",")); __s += &(UString::from("如此注重逻辑推导的作者居然会对论断的前提条件如此宽松以至于不留回旋余地?当然不是,在回复的一开头,")); __s += &(UString::from("聪明的作者就已经强调了自己作为被冒犯者有权力定义自己的感受,当然有权力!,但是这种感受是否可以无限扩展到")); __s += &(UString::from("\"人际尊重的基本门槛\",还是值得商榷的,逻辑严谨如你岂能放过如此基础的逻辑漏洞?也许我们可以采用更加自洽的解释,")); __s += &(UString::from(" 这种愤怒来源于作者遭到否定是的第一反应,一篇让你耿耿于怀三年的留言需要你通过反复打磨的语言和极致构思的反讽,")); __s += &(UString::from("只为了冷嘲热讽一个逻辑甚至不大通顺的留言.\" 我一定要用最严密的逻辑反驳回去,这是关乎我尊严的网络论战\",也许你心里确实这么想,")); __s += &(UString::from(" 可是承认这件事情在你的内心是一件丢脸的事情,倘若承认了自己的三年的耿耿于怀, 就等同于认可自己与对方与自己处于同一水平对话,")); __s += &(UString::from(" “居然要和一个沙文主义在相提并论, 这怎么可以接受”.但事实上,如此自视清高反而令人啼笑皆非,如果你可以大方承认自己的傲慢,")); __s += &(UString::from("我大可因为你的心胸宽广\"对你致上最高的敬意\".别急着找我的逻辑漏洞,因为我也会大大方方的承认我就是在玩,")); __s += &(UString::from("我乐意这种伪装成思辨的娱乐,这比大部分辩论赛有意思多了 .anyway,你完全有机会在一开始就讲清楚自己愤怒的来源,")); __s += &(UString::from("而不是强行带上无所谓的面具却又如此的用力过猛,希望下一次你可以清晰表述,就像你自己提到的那样,")); __s += &(UString::from("不要\"把解读的权利拱手让给对方\"")); __s }).as_str()).as_ustr())).as_ustr(); __s }).as_str()))).unwrap();
                    }
                }
            }
        }
    });
}
