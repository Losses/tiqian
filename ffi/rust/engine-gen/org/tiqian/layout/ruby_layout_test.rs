#![cfg(test)]

use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::ruby_layout_test_support::RubyLayoutTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    SupportTotalWidthFault(crate::org::tiqian::layout::ruby_layout_test_support::RubyLayoutTestSupportTotalWidthFault),
}

impl From<RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault) -> Self {
        match value {
            RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault) -> Self {
        match value {
            RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault) -> Self {
        match value {
            RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault> for crate::org::tiqian::layout::ruby_layout_test_support::RubyLayoutTestSupportTotalWidthFault {
    fn from(value: RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault) -> Self {
        match value {
            RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault::SupportTotalWidthFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::ruby_layout_test_support::RubyLayoutTestSupportTotalWidthFault> for RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault {
    fn from(value: crate::org::tiqian::layout::ruby_layout_test_support::RubyLayoutTestSupportTotalWidthFault) -> Self {
        RubyLayoutTestWideAdjacentReadingsSpreadButNarrowDoNotFault::SupportTotalWidthFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}

impl From<RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault) -> Self {
        match value {
            RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault) -> Self {
        match value {
            RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault) -> Self {
        match value {
            RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault) -> Self {
        match value {
            RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        RubyLayoutTestUniformModeAddsTheSameDeficitToEveryLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}

impl From<RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault) -> Self {
        match value {
            RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault) -> Self {
        match value {
            RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault) -> Self {
        match value {
            RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault) -> Self {
        match value {
            RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        RubyLayoutTestTightLineHeightRaisesOnlyTheAnnotatedLineByDefaultFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    SupportLayoutContradictoryFault(crate::org::tiqian::layout::ruby_layout_test_support::RubyLayoutTestSupportLayoutContradictoryFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}

impl From<RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault) -> Self {
        match value {
            RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault) -> Self {
        match value {
            RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault> for crate::org::tiqian::layout::ruby_layout_test_support::RubyLayoutTestSupportLayoutContradictoryFault {
    fn from(value: RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault) -> Self {
        match value {
            RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault::SupportLayoutContradictoryFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault) -> Self {
        match value {
            RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault) -> Self {
        match value {
            RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::ruby_layout_test_support::RubyLayoutTestSupportLayoutContradictoryFault> for RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault {
    fn from(value: crate::org::tiqian::layout::ruby_layout_test_support::RubyLayoutTestSupportLayoutContradictoryFault) -> Self {
        RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault::SupportLayoutContradictoryFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        RubyLayoutTestRubyVerticalGeometryUsesLatinMetricsNotReadingInkFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}

impl From<RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault) -> Self {
        match value {
            RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault) -> Self {
        match value {
            RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault) -> Self {
        match value {
            RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault) -> Self {
        match value {
            RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        RubyLayoutTestRubyOnOneLineKeepsTheWholeBaselineGridStableFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}

impl From<RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault) -> Self {
        match value {
            RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault) -> Self {
        match value {
            RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault) -> Self {
        match value {
            RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault) -> Self {
        match value {
            RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        RubyLayoutTestRubyDoesNotChangeLineBoxAndCentresOverBaseFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RubyLayoutTestNoRubyIsUnchangedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}

impl From<RubyLayoutTestNoRubyIsUnchangedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: RubyLayoutTestNoRubyIsUnchangedFault) -> Self {
        match value {
            RubyLayoutTestNoRubyIsUnchangedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestNoRubyIsUnchangedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RubyLayoutTestNoRubyIsUnchangedFault) -> Self {
        match value {
            RubyLayoutTestNoRubyIsUnchangedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestNoRubyIsUnchangedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: RubyLayoutTestNoRubyIsUnchangedFault) -> Self {
        match value {
            RubyLayoutTestNoRubyIsUnchangedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RubyLayoutTestNoRubyIsUnchangedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: RubyLayoutTestNoRubyIsUnchangedFault) -> Self {
        match value {
            RubyLayoutTestNoRubyIsUnchangedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for RubyLayoutTestNoRubyIsUnchangedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        RubyLayoutTestNoRubyIsUnchangedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RubyLayoutTestNoRubyIsUnchangedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RubyLayoutTestNoRubyIsUnchangedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for RubyLayoutTestNoRubyIsUnchangedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        RubyLayoutTestNoRubyIsUnchangedFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for RubyLayoutTestNoRubyIsUnchangedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        RubyLayoutTestNoRubyIsUnchangedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[test]
fn ruby_does_not_change_line_box_and_centres_over_base() {
    testlib::run("org.tiqian.layout.RubyLayoutTest.rubyDoesNotChangeLineBoxAndCentresOverBase", "org.tiqian.layout.RubyLayoutTest.rubyDoesNotChangeLineBoxAndCentresOverBase", || {
        let mut t = TestTraceRecorder::new("RubyLayoutTest");
        t.section(&"rubyDoesNotChangeLineBoxAndCentresOverBase");
        let plain = RubyLayoutTestSupport::ruby_layout_test_support_layout(&vec![]).unwrap();
        let ruby = RubyLayoutTestSupport::ruby_layout_test_support_layout(&vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "zhōng", Some(vec![]), RubyKind::Pinyin, None)).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(plain.lines[0usize].top, ruby.lines[0usize].top, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(plain.lines[0usize].baseline, ruby.lines[0usize].baseline, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(plain.lines[0usize].bottom, ruby.lines[0usize].bottom, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance((plain.size).clone().height, (ruby.size).clone().height, 0.001f64, None).unwrap();
        let line_height_decision = (ruby.debug).clone().ruby_line_height_decision;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PerLine", (line_height_decision.as_ref().unwrap().mode).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0 as f64, line_height_decision.as_ref().unwrap().max_extra, 0.001f64, None).unwrap();
        let mut all_zero = true;
        for i in 0..match u32::try_from((line_height_decision.as_ref().unwrap().line_extras).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if line_height_decision.as_ref().unwrap().line_extras.clone()[usize::try_from(i).unwrap_or(0)] != 0 as f64 {
                all_zero = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_zero, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((line_height_decision.as_ref().unwrap().expanded_line_indices).clone().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ExistingInterlineSpaceFitsRuby", (line_height_decision.as_ref().unwrap().reason).to_string().as_str(), None).unwrap();
        let decisions = ((ruby.debug).clone().ruby_decisions).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"zhōng", ((decisions[0usize]).clone().text).to_string().as_str(), None).unwrap();
        let first_advance = ruby.clusters[0usize].advance;
        let _ = TracedAssertions::traced_assertions_assert_true((decisions[0usize].center_x) >= 0 as f64 && (decisions[0usize].center_x) <= first_advance, Some((format!("{}{}{}",
            "centre ",
            decisions[0usize].center_x,
            " within 中's span"
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((decisions[0usize].baseline_y) < (ruby.lines[0usize].baseline), Some("ruby baseline above base baseline".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(ruby.lines[0usize].baseline - 14.08f64, decisions[0usize].baseline_y + decisions[0usize].font_size * 0.2f64, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(500, decisions[0usize].font_weight, Some("ruby defaults one weight step heavier than base".to_string())).unwrap();
    });
}

#[test]
fn ruby_on_one_line_keeps_the_whole_baseline_grid_stable() {
    testlib::run("org.tiqian.layout.RubyLayoutTest.rubyOnOneLineKeepsTheWholeBaselineGridStable", "org.tiqian.layout.RubyLayoutTest.rubyOnOneLineKeepsTheWholeBaselineGridStable", || {
        let mut t = TestTraceRecorder::new("RubyLayoutTest");
        t.section(&"rubyOnOneLineKeepsTheWholeBaselineGridStable");
        let plain = RubyLayoutTestSupport::ruby_layout_test_support_layout_eight(&vec![]).unwrap();
        let annotated = RubyLayoutTestSupport::ruby_layout_test_support_layout_eight(&vec![
    (RubySpan::new(TextRange::new(4u32, 5u32).unwrap(), "wù", Some(vec![]), RubyKind::Pinyin, None)).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(u32::try_from((plain.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), u32::try_from((annotated.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance((plain.size).clone().height, (annotated.size).clone().height, 0.001f64, None).unwrap();
        for i in 0..match u32::try_from(plain.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let plain_line = (plain.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let annotated_line = (annotated.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(plain_line.top, annotated_line.top, 0.001f64, None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(plain_line.baseline, annotated_line.baseline, 0.001f64, None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(plain_line.bottom, annotated_line.bottom, 0.001f64, None).unwrap();
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(24 as f64, annotated.lines[1usize].baseline - annotated.lines[0usize].baseline, 0.001f64, None).unwrap();
    });
}

#[test]
fn tight_line_height_raises_only_the_annotated_line_by_default() {
    testlib::run("org.tiqian.layout.RubyLayoutTest.tightLineHeightRaisesOnlyTheAnnotatedLineByDefault", "org.tiqian.layout.RubyLayoutTest.tightLineHeightRaisesOnlyTheAnnotatedLineByDefault", || {
        let mut t = TestTraceRecorder::new("RubyLayoutTest");
        t.section(&"tightLineHeightRaisesOnlyTheAnnotatedLineByDefault");
        let plain = RubyLayoutTestSupport::ruby_layout_test_support_layout_twelve(&vec![]).unwrap();
        let annotated = RubyLayoutTestSupport::ruby_layout_test_support_layout_twelve(&vec![
    (RubySpan::new(TextRange::new(4u32, 5u32).unwrap(), "wù", Some(vec![]), RubyKind::Pinyin, None)).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::try_from((annotated.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance((plain.size).clone().height + format!("{}", (6i32)).parse::<f64>().unwrap_or(0.0), (annotated.size).clone().height, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(18 as f64, annotated.lines[0usize].bottom - annotated.lines[0usize].top, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(24 as f64, annotated.lines[1usize].bottom - annotated.lines[1usize].top, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(18 as f64, annotated.lines[2usize].bottom - annotated.lines[2usize].top, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(24 as f64, annotated.lines[1usize].baseline - annotated.lines[0usize].baseline, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(18 as f64, annotated.lines[2usize].baseline - annotated.lines[1usize].baseline, 0.001f64, None).unwrap();
        let decision = (annotated.debug).clone().ruby_line_height_decision;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PerLine", (decision.as_ref().unwrap().mode).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(6 as f64, decision.as_ref().unwrap().max_extra, 0.001f64, None).unwrap();
        let _ = RubyLayoutTestSupport::ruby_layout_test_support_assert_float_list_equals(&vec![0 as f64, 6 as f64, 0 as f64], &(decision.as_ref().unwrap().line_extras).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![1], &(decision.as_ref().unwrap().expanded_line_indices).clone(), None).unwrap();
    });
}

#[test]
fn uniform_mode_adds_the_same_deficit_to_every_line() {
    testlib::run("org.tiqian.layout.RubyLayoutTest.uniformModeAddsTheSameDeficitToEveryLine", "org.tiqian.layout.RubyLayoutTest.uniformModeAddsTheSameDeficitToEveryLine", || {
        let mut t = TestTraceRecorder::new("RubyLayoutTest");
        t.section(&"uniformModeAddsTheSameDeficitToEveryLine");
        let result = RubyLayoutTestSupport::ruby_layout_test_support_layout_uniform(&vec![
    (RubySpan::new(TextRange::new(4u32, 5u32).unwrap(), "wù", Some(vec![]), RubyKind::Pinyin, None)).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line = (result.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(24 as f64, line.bottom - line.top, 0.001f64, None).unwrap();
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(24 as f64, result.lines[1usize].baseline - result.lines[0usize].baseline, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(24 as f64, result.lines[2usize].baseline - result.lines[1usize].baseline, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(72 as f64, (result.size).clone().height, 0.001f64, None).unwrap();
        let decision = (result.debug).clone().ruby_line_height_decision;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"UniformParagraph", (decision.as_ref().unwrap().mode).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(6 as f64, decision.as_ref().unwrap().max_extra, 0.001f64, None).unwrap();
        let _ = RubyLayoutTestSupport::ruby_layout_test_support_assert_float_list_equals(&vec![6 as f64, 6 as f64, 6 as f64], &(decision.as_ref().unwrap().line_extras).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 1, 2], &(decision.as_ref().unwrap().expanded_line_indices).clone(), None).unwrap();
    });
}

#[test]
fn ruby_vertical_geometry_uses_latin_metrics_not_reading_ink() {
    testlib::run("org.tiqian.layout.RubyLayoutTest.rubyVerticalGeometryUsesLatinMetricsNotReadingInk", "org.tiqian.layout.RubyLayoutTest.rubyVerticalGeometryUsesLatinMetricsNotReadingInk", || {
        let mut t = TestTraceRecorder::new("RubyLayoutTest");
        t.section(&"rubyVerticalGeometryUsesLatinMetricsNotReadingInk");
        let shallow_ink = RubyLayoutTestSupport::ruby_layout_test_support_layout_contradictory(&"he").unwrap();
        let extreme_ink = RubyLayoutTestSupport::ruby_layout_test_support_layout_contradictory(&"pg").unwrap();
        let shallow_decision = ((shallow_ink.debug).clone().ruby_decisions[0usize]).clone();
        let extreme_decision = ((extreme_ink.debug).clone().ruby_decisions[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance((shallow_ink.size).clone().height, (extreme_ink.size).clone().height, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(shallow_ink.lines[0usize].top, extreme_ink.lines[0usize].top, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(shallow_ink.lines[0usize].baseline, extreme_ink.lines[0usize].baseline, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(shallow_ink.lines[0usize].bottom, extreme_ink.lines[0usize].bottom, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(shallow_decision.baseline_y, extreme_decision.baseline_y, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(shallow_decision.ascent, extreme_decision.ascent, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(shallow_decision.descent, extreme_decision.descent, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance((shallow_ink.debug).clone().ruby_line_height_decision.as_ref().unwrap().ruby_extent, (extreme_ink.debug).clone().ruby_line_height_decision.as_ref().unwrap().ruby_extent, 0.001f64, None).unwrap();
    });
}

#[test]
fn no_ruby_is_unchanged() {
    testlib::run("org.tiqian.layout.RubyLayoutTest.noRubyIsUnchanged", "org.tiqian.layout.RubyLayoutTest.noRubyIsUnchanged", || {
        let mut t = TestTraceRecorder::new("RubyLayoutTest");
        t.section(&"noRubyIsUnchanged");
        let plain = RubyLayoutTestSupport::ruby_layout_test_support_layout(&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((plain.debug).clone().ruby_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn wide_adjacent_readings_spread_but_narrow_do_not() {
    testlib::run("org.tiqian.layout.RubyLayoutTest.wideAdjacentReadingsSpreadButNarrowDoNot", "org.tiqian.layout.RubyLayoutTest.wideAdjacentReadingsSpreadButNarrowDoNot", || {
        let mut t = TestTraceRecorder::new("RubyLayoutTest");
        t.section(&"wideAdjacentReadingsSpreadButNarrowDoNot");
        let plain = RubyLayoutTestSupport::ruby_layout_test_support_total_width(&vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]).unwrap();
        let narrow = RubyLayoutTestSupport::ruby_layout_test_support_total_width(&vec!["yī".to_string(), "rén".to_string(), "yī".to_string(), "rén".to_string()]).unwrap();
        let wide = RubyLayoutTestSupport::ruby_layout_test_support_total_width(&vec!["zhuāng".to_string(), "chuáng".to_string(), "shuāng".to_string(), "guāng".to_string()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((narrow) >= plain, Some((format!("{}{}{}{}{}",
            "spread never shrinks the line (",
            narrow,
            " vs ",
            plain,
            ")"
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((wide) > (narrow), Some((format!("{}{}{}{}{}",
            "wider readings spread more (",
            wide,
            " vs ",
            narrow,
            ")"
        )).to_string())).unwrap();
    });
}
