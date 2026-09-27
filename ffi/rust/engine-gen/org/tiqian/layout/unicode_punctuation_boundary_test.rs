#![cfg(test)]

use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::unicode_punctuation_boundary_resolver::UnicodePunctuationBoundaryResolver;
use crate::org::tiqian::layout::unicode_punctuation_boundary_test_support::UnicodePunctuationBoundaryTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryTestWesternOpeningBracketsCannotEndAnAutomaticLineFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
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
        let mut t = TestTraceRecorder::new("UnicodePunctuationBoundaryTest");
        t.section(&"westernBracketsTouchingCjkExposeAllFourStretchBoundaries");
        let text = "育(中文)后".to_string();
        let c = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_clusters(text.as_str(), None).unwrap();
        let roles = vec![
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::CjkText,
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::CjkText,
];
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_set_ints(&vec![0, 1, 3, 4]),
UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_western_bracket_cjk_inter_char_boundaries(text.as_str(), &c, &roles).unwrap(), None).unwrap();
        let w = "A(B)C".to_string();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_set_ints(&vec![]),
UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_western_bracket_cjk_inter_char_boundaries(w.as_str(), &UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_clusters(w.as_str(), Some(true)).unwrap(),
&UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_roles(w.as_str(), Some(true))).unwrap(), None).unwrap();
    });
}

#[test]
fn western_closing_punctuation_cannot_begin_an_automatic_line() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.westernClosingPunctuationCannotBeginAnAutomaticLine", "org.tiqian.layout.UnicodePunctuationBoundaryTest.westernClosingPunctuationCannotBeginAnAutomaticLine", || {
        let mut t = TestTraceRecorder::new("UnicodePunctuationBoundaryTest");
        t.section(&"westernClosingPunctuationCannotBeginAnAutomaticLine");
        for mi in 0..9 {
            let mark = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_mark_closing(mi);
            for bi in 0..match u32::try_from(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let b = (UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers()[usize::try_from(bi).unwrap_or(0)]).clone();
                let text = format!("{}{}{}",
            "中文",
            mark,
            "文"
        );
                let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(text.as_str(), 32 as f64, b.breaker.clone(), Some(KinsokuLevel::None)).unwrap();
                let ls = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), text.as_str());
                let mut ok = true;
                for x in &ls {
                    if x.starts_with(&mark) {
                        ok = false;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((format!("{}{}{}{}{}{}",
            (b.label).to_string(),
            " placed '",
            mark,
            "' at line start: [",
            { let joined1 = ls; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } out },
            "]"
        )).to_string())).unwrap();
                let mut d = false;
                for xi in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    let x = ((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(xi).unwrap_or(0)]).clone();
                    if x.source_text.to_string() == mark && (x.forbidden_position).to_string() == "LineStart" && ((x.reason).to_string()).starts_with(&"Uax14WesternPunctuationBoundary:") {
                        d = true;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(d, Some((format!("{}{}{}{}{}",
            (b.label).to_string(),
            " '",
            mark,
            "' decisions=",
            {
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
    }
        )).to_string())).unwrap();
            }
        }
    });
}

#[test]
fn western_opening_brackets_cannot_end_an_automatic_line() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.westernOpeningBracketsCannotEndAnAutomaticLine", "org.tiqian.layout.UnicodePunctuationBoundaryTest.westernOpeningBracketsCannotEndAnAutomaticLine", || {
        let mut t = TestTraceRecorder::new("UnicodePunctuationBoundaryTest");
        t.section(&"westernOpeningBracketsCannotEndAnAutomaticLine");
        for mi in 0..3 {
            let mark = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_mark_opening(mi);
            for bi in 0..match u32::try_from(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let b = (UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers()[usize::try_from(bi).unwrap_or(0)]).clone();
                let text = format!("{}{}{}",
            "ABCD",
            mark,
            "E"
        );
                let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(text.as_str(), 40 as f64, b.breaker.clone(), None).unwrap();
                let mut ok = true;
                {
                    let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), text.as_str());
                    for x in &_g1 {
                        if x.ends_with(&mark) {
                            ok = false;
                        }
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((format!("{}{}{}{}{}{}",
            (b.label).to_string(),
            " placed '",
            mark,
            "' at line end: [",
            { let joined3 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), text.as_str()); let mut out = String::new(); let n = joined3.len(); let mut index3 = 0usize; while index3 < n { if index3 > 0 { out.push_str(&(", ")); }
let _ = write!(out, "{}", joined3[index3]); index3 += 1; } out },
            "]"
        )).to_string())).unwrap();
                let mut d = false;
                for xi in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    let x = ((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(xi).unwrap_or(0)]).clone();
                    if x.source_text.to_string() == mark && (x.forbidden_position).to_string() == "LineEnd" && (x.reason).to_string() == "Uax14WesternPunctuationBoundary:LB14" {
                        d = true;
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(d, Some((format!("{}{}{}{}{}",
            (b.label).to_string(),
            " '",
            mark,
            "' decisions=",
            {
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
    }
        )).to_string())).unwrap();
            }
        }
    });
}

#[test]
fn unmatched_western_curly_double_quotes_retain_their_direction() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.unmatchedWesternCurlyDoubleQuotesRetainTheirDirection", "org.tiqian.layout.UnicodePunctuationBoundaryTest.unmatchedWesternCurlyDoubleQuotesRetainTheirDirection", || {
        let mut t = TestTraceRecorder::new("UnicodePunctuationBoundaryTest");
        t.section(&"unmatchedWesternCurlyDoubleQuotesRetainTheirDirection");
        for bi in 0..match u32::try_from(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let b = (UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers()[usize::try_from(bi).unwrap_or(0)]).clone();
            let a = "ABCD”E".to_string();
            let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(a.as_str(), 32 as f64, b.breaker.clone(), None).unwrap();
            let mut ok = true;
            {
                let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), a.as_str());
                for x in &_g1 {
                    if x.starts_with(&"”") {
                        ok = false;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((format!("{}{}{}{}",
            (b.label).to_string(),
            " closing lines=[",
            { let joined5 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), a.as_str()); let mut out = String::new(); let n = joined5.len(); let mut index5 = 0usize; while index5 < n { if index5 > 0 { out.push_str(&(", ")); }
let _ = write!(out, "{}", joined5[index5]); index5 += 1; } out },
            "]"
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Uax14WesternPunctuationBoundary:LB19", UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((r).clone(), &"”", &"LineStart").as_str(), None).unwrap();
            let q = "ABCD“E".to_string();
            let z = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(q.as_str(), 40 as f64, b.breaker.clone(), None).unwrap();
            ok = true;
            {
                let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((z).clone(), q.as_str());
                for x in &_g1 {
                    if x.ends_with(&"“") {
                        ok = false;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((format!("{}{}{}{}",
            (b.label).to_string(),
            " opening lines=[",
            { let joined7 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((z).clone(), q.as_str()); let mut out = String::new(); let n = joined7.len(); let mut index7 = 0usize; while index7 < n { if index7 > 0 { out.push_str(&(", ")); }
let _ = write!(out, "{}", joined7[index7]); index7 += 1; } out },
            "]"
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Uax14WesternPunctuationBoundary:LB19", UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((z).clone(), &"“", &"LineEnd").as_str(), None).unwrap();
        }
    });
}

#[test]
fn unmatched_elision_apostrophe_binds_forward_instead_of_being_guessed_as_a_closer() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.unmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloser", "org.tiqian.layout.UnicodePunctuationBoundaryTest.unmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloser", || {
        let mut t = TestTraceRecorder::new("UnicodePunctuationBoundaryTest");
        t.section(&"unmatchedElisionApostropheBindsForwardInsteadOfBeingGuessedAsACloser");
        let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(&"AB ’90s", 16 as f64, Box::new(GreedyLineBreaker::new(None, None, None, None)), None).unwrap();
        let mut start = false;
        for xi in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(xi).unwrap_or(0)]).clone();
            if x.source_text.to_string() == "’" && (x.forbidden_position).to_string() == "LineStart" {
                start = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Uax14WesternPunctuationBoundary:LB19", UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((r).clone(), &"’", &"LineEnd").as_str(), None).unwrap();
    });
}

#[test]
fn western_baseline_survives_clreq_kinsoku_none() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.westernBaselineSurvivesClreqKinsokuNone", "org.tiqian.layout.UnicodePunctuationBoundaryTest.westernBaselineSurvivesClreqKinsokuNone", || {
        let mut t = TestTraceRecorder::new("UnicodePunctuationBoundaryTest");
        t.section(&"westernBaselineSurvivesClreqKinsokuNone");
        let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(&"ABCD)E", 32 as f64, Box::new(GreedyLineBreaker::new(None, None, None, None)), Some(KinsokuLevel::None)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Uax14WesternPunctuationBoundary:LB13", UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((r).clone(), &")", &"LineStart").as_str(), None).unwrap();
    });
}

#[test]
fn bracket_boundaries_remain_protected_across_western_spaces() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.bracketBoundariesRemainProtectedAcrossWesternSpaces", "org.tiqian.layout.UnicodePunctuationBoundaryTest.bracketBoundariesRemainProtectedAcrossWesternSpaces", || {
        let mut t = TestTraceRecorder::new("UnicodePunctuationBoundaryTest");
        t.section(&"bracketBoundariesRemainProtectedAcrossWesternSpaces");
        for bi in 0..match u32::try_from(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let b = (UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers()[usize::try_from(bi).unwrap_or(0)]).clone();
            for wi in 0..9 {
                let width = u32::wrapping_add(48, u32::wrapping_mul(wi, 4));
                {
                    let o = "ABCD(  EFGH".to_string();
                    let mut ok = true;
                    {
                        let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(o.as_str(), i32::from_ne_bytes((width).to_ne_bytes()) as f64,
b.breaker.clone(), None).unwrap(), o.as_str());
                        for x in &_g1 {
                            if x.ends_with(&"(") {
                                ok = false;
                            }
                        }
                    }
                    let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((format!("{}{}{}{}{}{}",
            (b.label).to_string(),
            " width=",
            crate::runtime::int_text::IntText::int_text(width),
            " left an opener before trailing spaces: [",
            { let joined9 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(o.as_str(), i32::from_ne_bytes((width).to_ne_bytes()) as f64,
b.breaker.clone(), None).unwrap(), o.as_str()); let mut out = String::new(); let n = joined9.len(); let mut index9 = 0usize; while index9 < n { if index9 > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined9[index9]); index9 += 1; } out },
            "]"
        )).to_string())).unwrap();
                    let c = "ABCD  )EFGH".to_string();
                    ok = true;
                    {
                        let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(c.as_str(), i32::from_ne_bytes((width).to_ne_bytes()) as f64,
b.breaker.clone(), None).unwrap(), c.as_str());
                        for x in &_g1 {
                            if x.starts_with(&")") {
                                ok = false;
                            }
                        }
                    }
                    let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((format!("{}{}{}{}{}{}",
            (b.label).to_string(),
            " width=",
            crate::runtime::int_text::IntText::int_text(width),
            " left a closer after leading spaces: [",
            { let joined11 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(c.as_str(), i32::from_ne_bytes((width).to_ne_bytes()) as f64,
b.breaker.clone(), None).unwrap(), c.as_str()); let mut out = String::new(); let n = joined11.len(); let mut index11 = 0usize; while index11 < n { if index11 > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined11[index11]); index11 += 1; } out },
            "]"
        )).to_string())).unwrap();
                }
            }
        }
    });
}

#[test]
fn paired_latin_curly_quotes_keep_their_content_across_both_line_edges() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryTest.pairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdges", "org.tiqian.layout.UnicodePunctuationBoundaryTest.pairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdges", || {
        let mut t = TestTraceRecorder::new("UnicodePunctuationBoundaryTest");
        t.section(&"pairedLatinCurlyQuotesKeepTheirContentAcrossBothLineEdges");
        for bi in 0..match u32::try_from(UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let b = (UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_breakers()[usize::try_from(bi).unwrap_or(0)]).clone();
            let a = "“ABCD”E".to_string();
            let r = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(a.as_str(), 40 as f64, b.breaker.clone(), None).unwrap();
            let mut ok = true;
            {
                let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), a.as_str());
                for x in &_g1 {
                    if x.starts_with(&"”") {
                        ok = false;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((format!("{}{}{}{}",
            (b.label).to_string(),
            " closing lines=[",
            { let joined13 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((r).clone(), a.as_str()); let mut out = String::new(); let n = joined13.len(); let mut index13 = 0usize; while index13 < n { if index13 > 0 { out.push_str(&(", "));
} let _ = write!(out, "{}", joined13[index13]); index13 += 1; } out },
            "]"
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Uax14WesternPunctuationBoundary:PairedClosingQuote", UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((r).clone(), &"”", &"LineStart").as_str(),
None).unwrap();
            let q = "ABCD“E”".to_string();
            let z = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_layout(q.as_str(), 40 as f64, b.breaker.clone(), None).unwrap();
            ok = true;
            {
                let _g1 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((z).clone(), q.as_str());
                for x in &_g1 {
                    if x.ends_with(&"“") {
                        ok = false;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((format!("{}{}{}{}",
            (b.label).to_string(),
            " opening lines=[",
            { let joined15 = UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_lines((z).clone(), q.as_str()); let mut out = String::new(); let n = joined15.len(); let mut index15 = 0usize; while index15 < n { if index15 > 0 { out.push_str(&(", "));
} let _ = write!(out, "{}", joined15[index15]); index15 += 1; } out },
            "]"
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Uax14WesternPunctuationBoundary:PairedOpeningQuote", UnicodePunctuationBoundaryTestSupport::unicode_punctuation_boundary_test_support_find_reason((z).clone(), &"“", &"LineEnd").as_str(),
None).unwrap();
        }
    });
}
