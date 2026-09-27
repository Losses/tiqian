#![cfg(test)]

use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::layout::auto_space_single_gap_test_support::AutoSpaceSingleGapTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault) -> Self {
        match value {
            AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault) -> Self {
        match value {
            AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault) -> Self {
        match value {
            AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault) -> Self {
        match value {
            AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault) -> Self {
        match value {
            AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault) -> Self {
        match value {
            AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault) -> Self {
        match value {
            AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault) -> Self {
        match value {
            AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault) -> Self {
        match value {
            AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault) -> Self {
        match value {
            AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault) -> Self {
        match value {
            AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault) -> Self {
        match value {
            AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}

impl From<AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault) -> Self {
        match value {
            AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn attached_reference_between_cjk_text_does_not_invent_an_autospace_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.attachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGap", "org.tiqian.layout.AutoSpaceSingleGapTest.attachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGap", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"attachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGap");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"正文1后文", &vec![
    (TextSpan::new(TextRange::new(2u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0,
Some((AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_render_auto_space_decisions(&(r.debug).clone().auto_space_decisions)).to_string())).unwrap();
    });
}

#[test]
fn attached_reference_before_latin_text_gets_the_virtual_cjk_latin_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.attachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGap", "org.tiqian.layout.AutoSpaceSingleGapTest.attachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGap", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"attachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGap");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"正文1ABC", &vec![
    (TextSpan::new(TextRange::new(2u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
]).unwrap();
        let d = ((r.debug).clone().auto_space_decisions[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"trailing", (d.side).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InlineAttachment.Previous", (d.boundary_role).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualAutoSpace:east-asian-spacing-W-N", (d.reason).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn attached_reference_at_paragraph_end_has_no_autospace_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.attachedReferenceAtParagraphEndHasNoAutospaceGap", "org.tiqian.layout.AutoSpaceSingleGapTest.attachedReferenceAtParagraphEndHasNoAutospaceGap", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"attachedReferenceAtParagraphEndHasNoAutospaceGap");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"正文1", &vec![
    (TextSpan::new(TextRange::new(2u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0,
Some((AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_render_auto_space_decisions(&(r.debug).clone().auto_space_decisions)).to_string())).unwrap();
    });
}

#[test]
fn one_typed_space_becomes_one_autospace_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.oneTypedSpaceBecomesOneAutospaceGap", "org.tiqian.layout.AutoSpaceSingleGapTest.oneTypedSpaceBecomesOneAutospaceGap", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"oneTypedSpaceBecomesOneAutospaceGap");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"中文 CJK 段落", &vec![]).unwrap();
        let s = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), &" ");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((s.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut all = true;
        for i in 0..match u32::try_from(s.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if s[usize::try_from(i).unwrap_or(0)].advance != 2.0f64 {
                all = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(48 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), &"CJK")[0usize].advance, None).unwrap();
    });
}

#[test]
fn two_typed_spaces_at_boundary_still_collapse_to_one_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.twoTypedSpacesAtBoundaryStillCollapseToOneGap", "org.tiqian.layout.AutoSpaceSingleGapTest.twoTypedSpacesAtBoundaryStillCollapseToOneGap", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"twoTypedSpacesAtBoundaryStillCollapseToOneGap");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"中文  CJK 段落", &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), &"  ")[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), &" ")[0usize].advance, None).unwrap();
    });
}

#[test]
fn three_typed_spaces_still_one_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.threeTypedSpacesStillOneGap", "org.tiqian.layout.AutoSpaceSingleGapTest.threeTypedSpacesStillOneGap", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"threeTypedSpacesStillOneGap");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"中文   CJK段落", &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), &"   ")[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(50 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), &"CJK")[0usize].advance, None).unwrap();
    });
}

#[test]
fn zero_spaces_get_inserted_gaps() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.zeroSpacesGetInsertedGaps", "org.tiqian.layout.AutoSpaceSingleGapTest.zeroSpacesGetInsertedGaps", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"zeroSpacesGetInsertedGaps");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"中文CJK段落", &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(52 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), &"CJK")[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut modes = true;
        let mut reductions = true;
        let mut reasons = true;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if x.mode.to_string() != "Insert" || x.characters_affected != 0 {
                modes = false;
            }
            if x.total_reduction != -2.0f64 {
                reductions = false;
            }
            if u32::from_ne_bytes((u_string::find_from(&(x.reason).to_string(), "TextAutoSpaceInsert", 0)).to_ne_bytes()) != 0 {
                reasons = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(modes, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(reductions, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(reasons, None).unwrap();
    });
}

#[test]
fn unicode_east_asian_spacing_covers_narrow_scripts_without_script_whitelists() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.unicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelists", "org.tiqian.layout.AutoSpaceSingleGapTest.unicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelists", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"unicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelists");
        for si in 0..3 {
            let sample = (vec!["α".to_string(), "я".to_string(), "ա".to_string()][usize::try_from(si).unwrap_or(0)]).clone();
            let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(format!("{}{}{}",
            "中",
            sample,
            "文"
        ).as_str(), &vec![]).unwrap();
            let n = (AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), sample.as_str())[0usize]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(20 as f64, n.advance, Some((format!("{}{}",
            "sample=",
            sample
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((format!("{}{}",
            "sample=",
            sample
        )).to_string())).unwrap();
            let mut all = false;
            if i32::from_ne_bytes((u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                all = true;
                for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if !AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_same_range((((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone(), (n.range).clone()) {
                        all = false;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(all, Some((format!("{}{}",
            "sample=",
            sample
        )).to_string())).unwrap();
            let mut rr = true;
            for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if r.debug.clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_string() != "TextAutoSpaceInsert:east-asian-spacing-W-N" {
                    rr = false;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(rr, Some((format!("{}{}",
            "sample=",
            sample
        )).to_string())).unwrap();
        }
    });
}

#[test]
fn conditional_punctuation_follows_chinese_language_resolution() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.conditionalPunctuationFollowsChineseLanguageResolution", "org.tiqian.layout.AutoSpaceSingleGapTest.conditionalPunctuationFollowsChineseLanguageResolution", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"conditionalPunctuationFollowsChineseLanguageResolution");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"中%文", &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut ok = true;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)].clone().boundary_role.to_string() != "EastAsianSpacing.Wide" {
                ok = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn autospace_does_not_fire_between_latin_and_cjk_punctuation() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.autospaceDoesNotFireBetweenLatinAndCjkPunctuation", "org.tiqian.layout.AutoSpaceSingleGapTest.autospaceDoesNotFireBetweenLatinAndCjkPunctuation", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"autospaceDoesNotFireBetweenLatinAndCjkPunctuation");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from(((AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"Tiqian ）说明", &vec![]).unwrap().debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0),
None).unwrap();
    });
}

#[test]
fn autospace_does_not_fire_before_slash_led_latin_technical_run() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.autospaceDoesNotFireBeforeSlashLedLatinTechnicalRun", "org.tiqian.layout.AutoSpaceSingleGapTest.autospaceDoesNotFireBeforeSlashLedLatinTechnicalRun", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"autospaceDoesNotFireBeforeSlashLedLatinTechnicalRun");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"恐跨/TERFism。如果", &vec![]).unwrap();
        let c = (AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), &"/TERFism")[0usize]).clone();
        let mut saw = false;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_same_range((((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone(), (c.range).clone()) &&
(((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone().side).to_string() == "leading" {
                saw = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!saw, Some((format!("{}{}",
            "slash-led Latin technical run must not receive leading autospace: ",
            AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_render_auto_space_decisions(&(r.debug).clone().auto_space_decisions)
        )).to_string())).unwrap();
    });
}

#[test]
fn autospace_still_fires_between_latin_and_cjk_text_even_with_punctuation_nearby() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.autospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearby", "org.tiqian.layout.AutoSpaceSingleGapTest.autospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearby", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"autospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearby");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(&"中文 shaping 之后", &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut role = true;
        let mut side = true;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)].clone().boundary_role.to_string() != "EastAsianSpacing.Wide" {
                role = false;
            }
            if r.debug.clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)].clone().side.to_string() != "gap" {
                side = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(role, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(side, None).unwrap();
    });
}

#[test]
fn autospace_distinguishes_letter_from_digit_at_boundary() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.autospaceDistinguishesLetterFromDigitAtBoundary", "org.tiqian.layout.AutoSpaceSingleGapTest.autospaceDistinguishesLetterFromDigitAtBoundary", || {
        let mut t = TestTraceRecorder::new("AutoSpaceSingleGapTest");
        t.section(&"autospaceDistinguishesLetterFromDigitAtBoundary");
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_letter_digit().unwrap();
        let a = ((AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), &"A")[0usize]).clone().range).clone();
        let n = ((AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), &"9")[0usize]).clone().range).clone();
        let mut all = true;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_same_range((((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone(), (a).clone()) {
                all = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) && all, Some((format!("{}{}",
            "only the letter fires: ",
            AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_render_auto_space_decisions(&(r.debug).clone().auto_space_decisions)
        )).to_string())).unwrap();
        let mut saw = false;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_same_range((((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone(), (n).clone()) {
                saw = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!saw, Some("digit boundary must not fire when cjkDigit disabled".to_string())).unwrap();
    });
}
