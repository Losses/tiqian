#![cfg(test)]

use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::unicode_punctuation_boundary_resolver::UnicodePunctuationBoundaryResolver;
use crate::org::tiqian::layout::unicode_punctuation_boundary_test_support::UnicodePunctuationBoundaryTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        UnicodePunctuationBoundaryTestWesternClosingPunctuationCannotBeginAnAutomaticLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryTestWesternBracketsTouchingCjkExposeAllFourStretchBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        UnicodePunctuationBoundaryTestWesternBaselineSurvivesClreqKinsokuNoneFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        UnicodePunctuationBoundaryTestUnmatchedWesternCurlyDoubleQuotesRetainTheirDirectionFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        UnicodePunctuationBoundaryTestUnmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloserFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        UnicodePunctuationBoundaryTestPairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdgesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        UnicodePunctuationBoundaryTestBracketBoundariesRemainProtectedAcrossWesternSpacesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[test]
fn western_brackets_touching_cjk_expose_all_four_stretch_boundaries() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.westernBracketsTouchingCjkExposeAllFourStretchBoundaries", "org.tiqian.layout.UnicodePunctuationBoundaryTest.westernBracketsTouchingCjkExposeAllFourStretchBoundaries", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,84,101,115,116])));
        t.section(UStr::new(&[119,101,115,116,101,114,110,66,114,97,99,107,101,116,115,84,111,117,99,104,105,110,103,67,106,107,69,120,112,111,115,101,65,108,108,70,111,117,114,83,116,114,101,116,99,104,66,111,117,110,100,97,114,105,101,115]));
        let text = UString::from("育(中文)后").to_ustring();
        let c = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_clusters(text.as_ustr(), None).unwrap();
        let roles = vec![
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::CjkText,
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::CjkText,
];
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_set_ints(&vec![0, 1, 3, 4]), UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_western_bracket_cjk_inter_char_boundaries(text.as_ustr(), &c, &roles).unwrap(), None).unwrap();
        let w = UString::from("A(B)C").to_ustring();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_set_ints(&vec![]), UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_western_bracket_cjk_inter_char_boundaries(w.as_ustr(), &UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_clusters(w.as_ustr(), Some(true)).unwrap(), &UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_roles(w.as_ustr(), Some(true))).unwrap(), None).unwrap();
    });
}

#[test]
fn western_closing_punctuation_cannot_begin_an_automatic_line() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.westernClosingPunctuationCannotBeginAnAutomaticLine", "org.tiqian.layout.UnicodePunctuationBoundaryTest.westernClosingPunctuationCannotBeginAnAutomaticLine", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,84,101,115,116])));
        t.section(UStr::new(&[119,101,115,116,101,114,110,67,108,111,115,105,110,103,80,117,110,99,116,117,97,116,105,111,110,67,97,110,110,111,116,66,101,103,105,110,65,110,65,117,116,111,109,97,116,105,99,76,105,110,101]));
        for mi in 0..9 {
            let mark = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_mark_closing(mi);
            for bi in 0..match u32::try_from(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let b = (UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers()[usize::try_from(bi).unwrap_or(0)]).clone();
                let text = { let mut __s = UString::new(); __s += &(UString::from("中文")); __s += mark.as_ustr(); __s += &(UString::from("文")); __s };
                let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(text.as_ustr(), 32 as f64, b.breaker.clone(), Some(KinsokuLevel::None)).unwrap();
                let ls = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), text.as_ustr());
                let mut ok = true;
                for x in &ls {
                    if x.starts_with(&mark) {
                        ok = false;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(ok, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" placed '")); __s += mark.as_ustr(); __s += &(UString::from("' at line start: [")); __s += UString::from(format!("{}", { let joined1 = ls; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()))).unwrap();
                let mut d = false;
                for xi in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    let x = ((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(xi).unwrap_or(0)]).clone();
                    if x.source_text.to_ustring() == mark && (x.forbidden_position).to_ustring() == UString::from("LineStart") && ((x.reason).to_ustring()).starts_with(&UString::from("Uax14WesternPunctuationBoundary:")) {
                        d = true;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(d, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" '")); __s += mark.as_ustr(); __s += &(UString::from("' decisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (r.debug).clone().contextual_kinsoku_decisions;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s }).as_str()))).unwrap();
            }
        }
    });
}

#[test]
fn western_opening_brackets_cannot_end_an_automatic_line() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.westernOpeningBracketsCannotEndAnAutomaticLine", "org.tiqian.layout.UnicodePunctuationBoundaryTest.westernOpeningBracketsCannotEndAnAutomaticLine", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,84,101,115,116])));
        t.section(UStr::new(&[119,101,115,116,101,114,110,79,112,101,110,105,110,103,66,114,97,99,107,101,116,115,67,97,110,110,111,116,69,110,100,65,110,65,117,116,111,109,97,116,105,99,76,105,110,101]));
        for mi in 0..3 {
            let mark = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_mark_opening(mi);
            for bi in 0..match u32::try_from(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let b = (UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers()[usize::try_from(bi).unwrap_or(0)]).clone();
                let text = { let mut __s = UString::new(); __s += &(UString::from("ABCD")); __s += mark.as_ustr(); __s += &(UString::from("E")); __s };
                let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(text.as_ustr(), 40 as f64, b.breaker.clone(), None).unwrap();
                let mut ok = true;
                {
                    let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), text.as_ustr());
                    for x in &_g1 {
                        if x.ends_with(&mark) {
                            ok = false;
                        }
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(ok, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" placed '")); __s += mark.as_ustr(); __s += &(UString::from("' at line end: [")); __s += UString::from(format!("{}", { let joined3 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), text.as_ustr()); let mut out = String::new(); let n = joined3.len(); let mut index3 = 0usize; while index3 < n { if index3 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined3[index3]); index3 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()))).unwrap();
                let mut d = false;
                for xi in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    let x = ((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(xi).unwrap_or(0)]).clone();
                    if x.source_text.to_ustring() == mark && (x.forbidden_position).to_ustring() == UString::from("LineEnd") && (x.reason).to_ustring() == UString::from("Uax14WesternPunctuationBoundary:LB14") {
                        d = true;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(d, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" '")); __s += mark.as_ustr(); __s += &(UString::from("' decisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (r.debug).clone().contextual_kinsoku_decisions;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s }).as_str()))).unwrap();
            }
        }
    });
}

#[test]
fn unmatched_western_curly_double_quotes_retain_their_direction() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.unmatchedWesternCurlyDoubleQuotesRetainTheirDirection", "org.tiqian.layout.UnicodePunctuationBoundaryTest.unmatchedWesternCurlyDoubleQuotesRetainTheirDirection", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,84,101,115,116])));
        t.section(UStr::new(&[117,110,109,97,116,99,104,101,100,87,101,115,116,101,114,110,67,117,114,108,121,68,111,117,98,108,101,81,117,111,116,101,115,82,101,116,97,105,110,84,104,101,105,114,68,105,114,101,99,116,105,111,110]));
        for bi in 0..match u32::try_from(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let b = (UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers()[usize::try_from(bi).unwrap_or(0)]).clone();
            let a = UString::from("ABCD”E").to_ustring();
            let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(a.as_ustr(), 32 as f64, b.breaker.clone(), None).unwrap();
            let mut ok = true;
            {
                let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), a.as_ustr());
                for x in &_g1 {
                    if x.starts_with(&UString::from("”")) {
                        ok = false;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(ok, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" closing lines=[")); __s += UString::from(format!("{}", { let joined5 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), a.as_ustr()); let mut out = String::new(); let n = joined5.len(); let mut index5 = 0usize; while index5 < n { if index5 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined5[index5]); index5 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[85,97,120,49,52,87,101,115,116,101,114,110,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,76,66,49,57]), UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((r).clone(), UStr::new(&[8221]), UStr::new(&[76,105,110,101,83,116,97,114,116])).as_ustr(), None).unwrap();
            let q = UString::from("ABCD“E").to_ustring();
            let z = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(q.as_ustr(), 40 as f64, b.breaker.clone(), None).unwrap();
            ok = true;
            {
                let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((z).clone(), q.as_ustr());
                for x in &_g1 {
                    if x.ends_with(&UString::from("“")) {
                        ok = false;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(ok, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" opening lines=[")); __s += UString::from(format!("{}", { let joined7 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((z).clone(), q.as_ustr()); let mut out = String::new(); let n = joined7.len(); let mut index7 = 0usize; while index7 < n { if index7 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined7[index7]); index7 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[85,97,120,49,52,87,101,115,116,101,114,110,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,76,66,49,57]), UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((z).clone(), UStr::new(&[8220]), UStr::new(&[76,105,110,101,69,110,100])).as_ustr(), None).unwrap();
        }
    });
}

#[test]
fn unmatched_elision_apostrophe_binds_forward_instead_of_being_guessed_as_a_closer() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.unmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloser", "org.tiqian.layout.UnicodePunctuationBoundaryTest.unmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloser", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,84,101,115,116])));
        t.section(UStr::new(&[117,110,109,97,116,99,104,101,100,69,108,105,115,105,111,110,65,112,111,115,116,114,111,112,104,101,66,105,110,100,115,70,111,114,119,97,114,100,73,110,115,116,101,97,100,79,102,66,101,105,110,103,71,117,101,115,115,101,100,65,115,65,67,108,111,115,101,114]));
        let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(UStr::new(&[65,66,32,8217,57,48,115]), 16 as f64, Box::new(GreedyLineBreaker::new(None, None, None, None)), None).unwrap();
        let mut start = false;
        for xi in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(xi).unwrap_or(0)]).clone();
            if x.source_text.to_ustring() == UString::from("’") && (x.forbidden_position).to_ustring() == UString::from("LineStart") {
                start = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[85,97,120,49,52,87,101,115,116,101,114,110,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,76,66,49,57]), UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((r).clone(), UStr::new(&[8217]), UStr::new(&[76,105,110,101,69,110,100])).as_ustr(), None).unwrap();
    });
}

#[test]
fn western_baseline_survives_clreq_kinsoku_none() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.westernBaselineSurvivesClreqKinsokuNone", "org.tiqian.layout.UnicodePunctuationBoundaryTest.westernBaselineSurvivesClreqKinsokuNone", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,84,101,115,116])));
        t.section(UStr::new(&[119,101,115,116,101,114,110,66,97,115,101,108,105,110,101,83,117,114,118,105,118,101,115,67,108,114,101,113,75,105,110,115,111,107,117,78,111,110,101]));
        let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(UStr::new(&[65,66,67,68,41,69]), 32 as f64, Box::new(GreedyLineBreaker::new(None, None, None, None)), Some(KinsokuLevel::None)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[85,97,120,49,52,87,101,115,116,101,114,110,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,76,66,49,51]), UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((r).clone(), UStr::new(&[41]), UStr::new(&[76,105,110,101,83,116,97,114,116])).as_ustr(), None).unwrap();
    });
}

#[test]
fn bracket_boundaries_remain_protected_across_western_spaces() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.bracketBoundariesRemainProtectedAcrossWesternSpaces", "org.tiqian.layout.UnicodePunctuationBoundaryTest.bracketBoundariesRemainProtectedAcrossWesternSpaces", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,84,101,115,116])));
        t.section(UStr::new(&[98,114,97,99,107,101,116,66,111,117,110,100,97,114,105,101,115,82,101,109,97,105,110,80,114,111,116,101,99,116,101,100,65,99,114,111,115,115,87,101,115,116,101,114,110,83,112,97,99,101,115]));
        for bi in 0..match u32::try_from(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let b = (UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers()[usize::try_from(bi).unwrap_or(0)]).clone();
            for wi in 0..9 {
                let width = u32::wrapping_add(48, u32::wrapping_mul(wi, 4));
                {
                    let o = UString::from("ABCD(  EFGH").to_ustring();
                    let mut ok = true;
                    {
                        let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(o.as_ustr(), i32::from_ne_bytes(((width) as i32).to_ne_bytes()) as f64, b.breaker.clone(), None).unwrap(), o.as_ustr());
                        for x in &_g1 {
                            if x.ends_with(&UString::from("(")) {
                                ok = false;
                            }
                        }
                    }
                    let _ = TracedAssertions::traced_assertions_assert_true(ok, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" width=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(width)).as_str())); __s += &(UString::from(" left an opener before trailing spaces: [")); __s += UString::from(format!("{}", { let joined9 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(o.as_ustr(), i32::from_ne_bytes(((width) as i32).to_ne_bytes()) as f64, b.breaker.clone(), None).unwrap(), o.as_ustr()); let mut out = String::new(); let n = joined9.len(); let mut index9 = 0usize; while index9 < n { if index9 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined9[index9]); index9 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()))).unwrap();
                    let c = UString::from("ABCD  )EFGH").to_ustring();
                    ok = true;
                    {
                        let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(c.as_ustr(), i32::from_ne_bytes(((width) as i32).to_ne_bytes()) as f64, b.breaker.clone(), None).unwrap(), c.as_ustr());
                        for x in &_g1 {
                            if x.starts_with(&UString::from(")")) {
                                ok = false;
                            }
                        }
                    }
                    let _ = TracedAssertions::traced_assertions_assert_true(ok, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" width=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(width)).as_str())); __s += &(UString::from(" left a closer after leading spaces: [")); __s += UString::from(format!("{}", { let joined11 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(c.as_ustr(), i32::from_ne_bytes(((width) as i32).to_ne_bytes()) as f64, b.breaker.clone(), None).unwrap(), c.as_ustr()); let mut out = String::new(); let n = joined11.len(); let mut index11 = 0usize; while index11 < n { if index11 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined11[index11]); index11 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()))).unwrap();
                }
            }
        }
    });
}

#[test]
fn paired_latin_curly_quotes_keep_their_content_across_both_line_edges() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.pairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdges", "org.tiqian.layout.UnicodePunctuationBoundaryTest.pairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdges", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,84,101,115,116])));
        t.section(UStr::new(&[112,97,105,114,101,100,76,97,116,105,110,67,117,114,108,121,81,117,111,116,101,115,75,101,101,112,84,104,101,105,114,67,111,110,116,101,110,116,65,99,114,111,115,115,66,111,116,104,76,105,110,101,69,100,103,101,115]));
        for bi in 0..match u32::try_from(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let b = (UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers()[usize::try_from(bi).unwrap_or(0)]).clone();
            let a = UString::from("“ABCD”E").to_ustring();
            let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(a.as_ustr(), 40 as f64, b.breaker.clone(), None).unwrap();
            let mut ok = true;
            {
                let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), a.as_ustr());
                for x in &_g1 {
                    if x.starts_with(&UString::from("”")) {
                        ok = false;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(ok, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" closing lines=[")); __s += UString::from(format!("{}", { let joined13 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), a.as_ustr()); let mut out = String::new(); let n = joined13.len(); let mut index13 = 0usize; while index13 < n { if index13 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined13[index13]); index13 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[85,97,120,49,52,87,101,115,116,101,114,110,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,80,97,105,114,101,100,67,108,111,115,105,110,103,81,117,111,116,101]), UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((r).clone(), UStr::new(&[8221]), UStr::new(&[76,105,110,101,83,116,97,114,116])).as_ustr(), None).unwrap();
            let q = UString::from("ABCD“E”").to_ustring();
            let z = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(q.as_ustr(), 40 as f64, b.breaker.clone(), None).unwrap();
            ok = true;
            {
                let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((z).clone(), q.as_ustr());
                for x in &_g1 {
                    if x.ends_with(&UString::from("“")) {
                        ok = false;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(ok, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (b.label).to_ustring().as_ustr(); __s += &(UString::from(" opening lines=[")); __s += UString::from(format!("{}", { let joined15 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((z).clone(), q.as_ustr()); let mut out = String::new(); let n = joined15.len(); let mut index15 = 0usize; while index15 < n { if index15 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined15[index15]); index15 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[85,97,120,49,52,87,101,115,116,101,114,110,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,80,97,105,114,101,100,79,112,101,110,105,110,103,81,117,111,116,101]), UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((z).clone(), UStr::new(&[8220]), UStr::new(&[76,105,110,101,69,110,100])).as_ustr(), None).unwrap();
        }
    });
}
