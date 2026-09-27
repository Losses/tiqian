#![cfg(test)]

use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::layout::r3_geometry_tail_coverage_test_support::R3GeometryTailCoverageTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault) -> Self {
        match value {
            R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault) -> Self {
        match value {
            R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault) -> Self {
        match value {
            R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault) -> Self {
        match value {
            R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        R3GeometryTailCoverageTestSpaceRunsResolveBothWideNarrowOrdersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault) -> Self {
        match value {
            R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault) -> Self {
        match value {
            R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault) -> Self {
        match value {
            R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault) -> Self {
        match value {
            R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        R3GeometryTailCoverageTestRubyBaseRangeCrossingClusterBoundariesIsSkippedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault) -> Self {
        match value {
            R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault) -> Self {
        match value {
            R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault) -> Self {
        match value {
            R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault) -> Self {
        match value {
            R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        R3GeometryTailCoverageTestPureLatinParagraphStillProducesLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault) -> Self {
        match value {
            R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault) -> Self {
        match value {
            R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault) -> Self {
        match value {
            R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault) -> Self {
        match value {
            R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        R3GeometryTailCoverageTestMaxLinesCapsVisibleLinesToOneFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault) -> Self {
        match value {
            R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault) -> Self {
        match value {
            R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault) -> Self {
        match value {
            R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault) -> Self {
        match value {
            R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        R3GeometryTailCoverageTestEmptyTextProducesNoVisibleLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault) -> Self {
        match value {
            R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault) -> Self {
        match value {
            R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault) -> Self {
        match value {
            R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault) -> Self {
        match value {
            R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        R3GeometryTailCoverageTestCenteredInkPunctuationKeepsPairedGlueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault) -> Self {
        match value {
            R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault) -> Self {
        match value {
            R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault) -> Self {
        match value {
            R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault) -> Self {
        match value {
            R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        R3GeometryTailCoverageTestAttachedReferenceAtSourceEndLaysOutWithoutVirtualBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn attached_reference_at_source_end_lays_out_without_virtual_boundary() {
    testlib::run("org.tiqian.layout.R3GeometryTailCoverageTest.attachedReferenceAtSourceEndLaysOutWithoutVirtualBoundary", "org.tiqian.layout.R3GeometryTailCoverageTest.attachedReferenceAtSourceEndLaysOutWithoutVirtualBoundary", || {
        let mut t = TestTraceRecorder::new("R3GeometryTailCoverageTest");
        t.section(&"attachedReferenceAtSourceEndLaysOutWithoutVirtualBoundary");
        let text = "正文：“内容·[1]".to_string();
        let a = 7u32;
        let r = R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_layout(text.as_str(), Some(320 as f64 as f64), None, Some(vec![
    (TextSpan::new(TextRange::new(a, u32::wrapping_add(a, 3)).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
]), None, None).unwrap();
        let mut no = true;
        for i in 0..match u32::try_from((r.debug).clone().spacing_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().spacing_decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_string().starts_with(&"AttachedInlineVirtual") {
                no = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(no, None).unwrap();
        let mut collapse = false;
        for i in 0..match u32::try_from((r.debug).clone().spacing_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().spacing_decisions[usize::try_from(i).unwrap_or(0)].clone().left_char.to_string() == "：" && (((r.debug).clone().spacing_decisions[usize::try_from(i).unwrap_or(0)]).clone().right_char).to_string() == "“" &&
((r.debug).clone().spacing_decisions[usize::try_from(i).unwrap_or(0)].reduction) > (0 as f64) {
                collapse = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(collapse, None).unwrap();
    });
}

#[test]
fn centered_ink_punctuation_keeps_paired_glue() {
    testlib::run("org.tiqian.layout.R3GeometryTailCoverageTest.centeredInkPunctuationKeepsPairedGlue", "org.tiqian.layout.R3GeometryTailCoverageTest.centeredInkPunctuationKeepsPairedGlue", || {
        let mut t = TestTraceRecorder::new("R3GeometryTailCoverageTest");
        t.section(&"centeredInkPunctuationKeepsPairedGlue");
        let wide = R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_layout(&"中·文", Some(320 as f64 as f64), None, None, None, Some(R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_centered_ink_shaper())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((wide.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let tight = R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_layout(&"文·本，内容。", Some(60 as f64 as f64), None, None, None, Some(R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_centered_ink_shaper())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((tight.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (1), None).unwrap();
    });
}

#[test]
fn empty_text_produces_no_visible_lines() {
    testlib::run("org.tiqian.layout.R3GeometryTailCoverageTest.emptyTextProducesNoVisibleLines", "org.tiqian.layout.R3GeometryTailCoverageTest.emptyTextProducesNoVisibleLines", || {
        let mut t = TestTraceRecorder::new("R3GeometryTailCoverageTest");
        t.section(&"emptyTextProducesNoVisibleLines");
        let r = R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_layout(&"", Some(100 as f64 as f64), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn max_lines_caps_visible_lines_to_one() {
    testlib::run("org.tiqian.layout.R3GeometryTailCoverageTest.maxLinesCapsVisibleLinesToOne", "org.tiqian.layout.R3GeometryTailCoverageTest.maxLinesCapsVisibleLinesToOne", || {
        let mut t = TestTraceRecorder::new("R3GeometryTailCoverageTest");
        t.section(&"maxLinesCapsVisibleLinesToOne");
        let text = "中文排版引擎测试文本，用于验证多行截断行为是否正确工作并继续延伸。".to_string();
        let u = R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_layout(text.as_str(), Some(64 as f64 as f64), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((u.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (1), Some("fixture must wrap without the cap".to_string())).unwrap();
        let c = R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_layout(text.as_str(), Some(64 as f64 as f64), Some(1), None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((c.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn pure_latin_paragraph_still_produces_lines() {
    testlib::run("org.tiqian.layout.R3GeometryTailCoverageTest.pureLatinParagraphStillProducesLines", "org.tiqian.layout.R3GeometryTailCoverageTest.pureLatinParagraphStillProducesLines", || {
        let mut t = TestTraceRecorder::new("R3GeometryTailCoverageTest");
        t.section(&"pureLatinParagraphStillProducesLines");
        let r = R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_layout(&"hello justified world", Some(96 as f64 as f64), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.lines[0usize].natural_width) > (0 as f64), None).unwrap();
    });
}

#[test]
fn ruby_base_range_crossing_cluster_boundaries_is_skipped() {
    testlib::run("org.tiqian.layout.R3GeometryTailCoverageTest.rubyBaseRangeCrossingClusterBoundariesIsSkipped", "org.tiqian.layout.R3GeometryTailCoverageTest.rubyBaseRangeCrossingClusterBoundariesIsSkipped", || {
        let mut t = TestTraceRecorder::new("R3GeometryTailCoverageTest");
        t.section(&"rubyBaseRangeCrossingClusterBoundariesIsSkipped");
        let r = R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_layout(&"中文测试", Some(320 as f64 as f64), None, None, Some(vec![
    (RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), "zhōng", Some(vec![]), RubyKind::Pinyin, None)).clone(),
    (RubySpan::new(TextRange::new(1u32, 3u32).unwrap(), "wén", Some(vec![]), RubyKind::Pinyin, None)).clone(),
]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from(((r.debug).clone().ruby_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn space_runs_resolve_both_wide_narrow_orders() {
    testlib::run("org.tiqian.layout.R3GeometryTailCoverageTest.spaceRunsResolveBothWideNarrowOrders", "org.tiqian.layout.R3GeometryTailCoverageTest.spaceRunsResolveBothWideNarrowOrders", || {
        let mut t = TestTraceRecorder::new("R3GeometryTailCoverageTest");
        t.section(&"spaceRunsResolveBothWideNarrowOrders");
        let a = R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_layout(&"中文 abc", Some(320 as f64 as f64), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((a.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((a.lines[0usize].natural_width) > (0 as f64), None).unwrap();
        let b = R3GeometryTailCoverageTestSupport::r3_geometry_tail_coverage_test_support_layout(&"abc 中文", Some(320 as f64 as f64), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((b.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((b.lines[0usize].natural_width) > (0 as f64), None).unwrap();
    });
}
