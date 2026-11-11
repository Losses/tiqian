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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestZeroSpacesGetInsertedGapsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestUnicodeEastAsianSpacingCoversNarrowScriptsWithoutScriptWhitelistsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestTwoTypedSpacesAtBoundaryStillCollapseToOneGapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestThreeTypedSpacesStillOneGapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestOneTypedSpaceBecomesOneAutospaceGapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestConditionalPunctuationFollowsChineseLanguageResolutionFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearbyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceDoesNotFireBetweenLatinAndCjkPunctuationFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceDoesNotFireBeforeSlashLedLatinTechnicalRunFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAutospaceDistinguishesLetterFromDigitAtBoundaryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAttachedReferenceBetweenCjkTextDoesNotInventAnAutospaceGapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAttachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestAttachedReferenceAtParagraphEndHasNoAutospaceGapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[97,116,116,97,99,104,101,100,82,101,102,101,114,101,110,99,101,66,101,116,119,101,101,110,67,106,107,84,101,120,116,68,111,101,115,78,111,116,73,110,118,101,110,116,65,110,65,117,116,111,115,112,97,99,101,71,97,112]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[27491,25991,49,21518,25991]), &vec![
    (TextSpan::new(TextRange::new(2u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some((AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_render_auto_space_decisions(&(r.debug).clone().auto_space_decisions)).to_ustring())).unwrap();
    });
}

#[test]
fn attached_reference_before_latin_text_gets_the_virtual_cjk_latin_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.attachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGap", "org.tiqian.layout.AutoSpaceSingleGapTest.attachedReferenceBeforeLatinTextGetsTheVirtualCjkLatinGap", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[97,116,116,97,99,104,101,100,82,101,102,101,114,101,110,99,101,66,101,102,111,114,101,76,97,116,105,110,84,101,120,116,71,101,116,115,84,104,101,86,105,114,116,117,97,108,67,106,107,76,97,116,105,110,71,97,112]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[27491,25991,49,65,66,67]), &vec![
    (TextSpan::new(TextRange::new(2u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
]).unwrap();
        let d = ((r.debug).clone().auto_space_decisions[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[116,114,97,105,108,105,110,103]), (d.side).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,108,105,110,101,65,116,116,97,99,104,109,101,110,116,46,80,114,101,118,105,111,117,115]), (d.boundary_role).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,65,117,116,111,83,112,97,99,101,58,101,97,115,116,45,97,115,105,97,110,45,115,112,97,99,105,110,103,45,87,45,78]), (d.reason).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn attached_reference_at_paragraph_end_has_no_autospace_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.attachedReferenceAtParagraphEndHasNoAutospaceGap", "org.tiqian.layout.AutoSpaceSingleGapTest.attachedReferenceAtParagraphEndHasNoAutospaceGap", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[97,116,116,97,99,104,101,100,82,101,102,101,114,101,110,99,101,65,116,80,97,114,97,103,114,97,112,104,69,110,100,72,97,115,78,111,65,117,116,111,115,112,97,99,101,71,97,112]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[27491,25991,49]), &vec![
    (TextSpan::new(TextRange::new(2u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some((AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_render_auto_space_decisions(&(r.debug).clone().auto_space_decisions)).to_ustring())).unwrap();
    });
}

#[test]
fn one_typed_space_becomes_one_autospace_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.oneTypedSpaceBecomesOneAutospaceGap", "org.tiqian.layout.AutoSpaceSingleGapTest.oneTypedSpaceBecomesOneAutospaceGap", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[111,110,101,84,121,112,101,100,83,112,97,99,101,66,101,99,111,109,101,115,79,110,101,65,117,116,111,115,112,97,99,101,71,97,112]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[20013,25991,32,67,74,75,32,27573,33853]), &vec![]).unwrap();
        let s = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), UStr::new(&[32]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((s.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut all = true;
        for i in 0..match u32::try_from(s.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if s[usize::try_from(i).unwrap_or(0)].advance != 2.0f64 {
                all = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(48 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), UStr::new(&[67,74,75]))[0usize].advance, None).unwrap();
    });
}

#[test]
fn two_typed_spaces_at_boundary_still_collapse_to_one_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.twoTypedSpacesAtBoundaryStillCollapseToOneGap", "org.tiqian.layout.AutoSpaceSingleGapTest.twoTypedSpacesAtBoundaryStillCollapseToOneGap", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[116,119,111,84,121,112,101,100,83,112,97,99,101,115,65,116,66,111,117,110,100,97,114,121,83,116,105,108,108,67,111,108,108,97,112,115,101,84,111,79,110,101,71,97,112]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[20013,25991,32,32,67,74,75,32,27573,33853]), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), UStr::new(&[32,32]))[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), UStr::new(&[32]))[0usize].advance, None).unwrap();
    });
}

#[test]
fn three_typed_spaces_still_one_gap() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.threeTypedSpacesStillOneGap", "org.tiqian.layout.AutoSpaceSingleGapTest.threeTypedSpacesStillOneGap", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[116,104,114,101,101,84,121,112,101,100,83,112,97,99,101,115,83,116,105,108,108,79,110,101,71,97,112]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[20013,25991,32,32,32,67,74,75,27573,33853]), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), UStr::new(&[32,32,32]))[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(50 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), UStr::new(&[67,74,75]))[0usize].advance, None).unwrap();
    });
}

#[test]
fn zero_spaces_get_inserted_gaps() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.zeroSpacesGetInsertedGaps", "org.tiqian.layout.AutoSpaceSingleGapTest.zeroSpacesGetInsertedGaps", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[122,101,114,111,83,112,97,99,101,115,71,101,116,73,110,115,101,114,116,101,100,71,97,112,115]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[20013,25991,67,74,75,27573,33853]), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(52 as f64, AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), UStr::new(&[67,74,75]))[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut modes = true;
        let mut reductions = true;
        let mut reasons = true;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if x.mode.to_ustring() != UString::from("Insert") || x.characters_affected != 0 {
                modes = false;
            }
            if x.total_reduction != -2.0f64 {
                reductions = false;
            }
            if u32::from_ne_bytes(((u_string::find_from(&((x.reason).to_ustring()), UString::from("TextAutoSpaceInsert").as_ustr(), 0)) as u32).to_ne_bytes()) != 0 {
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
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[117,110,105,99,111,100,101,69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,67,111,118,101,114,115,78,97,114,114,111,119,83,99,114,105,112,116,115,87,105,116,104,111,117,116,83,99,114,105,112,116,87,104,105,116,101,108,105,115,116,115]));
        for si in 0..3 {
            let sample = (vec![
    UString::from("α").to_ustring(),
    UString::from("я").to_ustring(),
    UString::from("ա").to_ustring(),
][usize::try_from(si).unwrap_or(0)]).clone();
            let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("中")); __s += sample.as_ustr(); __s += &(UString::from("文")); __s }).as_str()).as_ustr(), &vec![]).unwrap();
            let n = (AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), sample.as_ustr())[0usize]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(20 as f64, n.advance, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("sample=")); __s += sample.as_ustr(); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("sample=")); __s += sample.as_ustr(); __s }).as_str()))).unwrap();
            let mut all = false;
            if i32::from_ne_bytes(((u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
                all = true;
                for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if !AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_same_range((((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone(), (n.range).clone()) {
                        all = false;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(all, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("sample=")); __s += sample.as_ustr(); __s }).as_str()))).unwrap();
            let mut rr = true;
            for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if r.debug.clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_ustring() != UString::from("TextAutoSpaceInsert:east-asian-spacing-W-N") {
                    rr = false;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(rr, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("sample=")); __s += sample.as_ustr(); __s }).as_str()))).unwrap();
        }
    });
}

#[test]
fn conditional_punctuation_follows_chinese_language_resolution() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.conditionalPunctuationFollowsChineseLanguageResolution", "org.tiqian.layout.AutoSpaceSingleGapTest.conditionalPunctuationFollowsChineseLanguageResolution", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[99,111,110,100,105,116,105,111,110,97,108,80,117,110,99,116,117,97,116,105,111,110,70,111,108,108,111,119,115,67,104,105,110,101,115,101,76,97,110,103,117,97,103,101,82,101,115,111,108,117,116,105,111,110]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[20013,37,25991]), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut ok = true;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)].clone().boundary_role.to_ustring() != UString::from("EastAsianSpacing.Wide") {
                ok = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn autospace_does_not_fire_between_latin_and_cjk_punctuation() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.autospaceDoesNotFireBetweenLatinAndCjkPunctuation", "org.tiqian.layout.AutoSpaceSingleGapTest.autospaceDoesNotFireBetweenLatinAndCjkPunctuation", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[97,117,116,111,115,112,97,99,101,68,111,101,115,78,111,116,70,105,114,101,66,101,116,119,101,101,110,76,97,116,105,110,65,110,100,67,106,107,80,117,110,99,116,117,97,116,105,111,110]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from(((AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[84,105,113,105,97,110,32,65289,35828,26126]), &vec![]).unwrap().debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn autospace_does_not_fire_before_slash_led_latin_technical_run() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.autospaceDoesNotFireBeforeSlashLedLatinTechnicalRun", "org.tiqian.layout.AutoSpaceSingleGapTest.autospaceDoesNotFireBeforeSlashLedLatinTechnicalRun", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[97,117,116,111,115,112,97,99,101,68,111,101,115,78,111,116,70,105,114,101,66,101,102,111,114,101,83,108,97,115,104,76,101,100,76,97,116,105,110,84,101,99,104,110,105,99,97,108,82,117,110]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[24656,36328,47,84,69,82,70,105,115,109,12290,22914,26524]), &vec![]).unwrap();
        let c = (AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), UStr::new(&[47,84,69,82,70,105,115,109]))[0usize]).clone();
        let mut saw = false;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_same_range((((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone(), (c.range).clone()) && (((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone().side).to_ustring() == UString::from("leading") {
                saw = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!saw, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("slash-led Latin technical run must not receive leading autospace: ")); __s += AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_render_auto_space_decisions(&(r.debug).clone().auto_space_decisions).as_ustr(); __s }).as_str()))).unwrap();
    });
}

#[test]
fn autospace_still_fires_between_latin_and_cjk_text_even_with_punctuation_nearby() {
    testlib::run("org.tiqian.layout.AutoSpaceSingleGapTest.autospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearby", "org.tiqian.layout.AutoSpaceSingleGapTest.autospaceStillFiresBetweenLatinAndCjkTextEvenWithPunctuationNearby", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[97,117,116,111,115,112,97,99,101,83,116,105,108,108,70,105,114,101,115,66,101,116,119,101,101,110,76,97,116,105,110,65,110,100,67,106,107,84,101,120,116,69,118,101,110,87,105,116,104,80,117,110,99,116,117,97,116,105,111,110,78,101,97,114,98,121]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_layout(UStr::new(&[20013,25991,32,115,104,97,112,105,110,103,32,20043,21518]), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut role = true;
        let mut side = true;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)].clone().boundary_role.to_ustring() != UString::from("EastAsianSpacing.Wide") {
                role = false;
            }
            if r.debug.clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)].clone().side.to_ustring() != UString::from("gap") {
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
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,117,116,111,83,112,97,99,101,83,105,110,103,108,101,71,97,112,84,101,115,116])));
        t.section(UStr::new(&[97,117,116,111,115,112,97,99,101,68,105,115,116,105,110,103,117,105,115,104,101,115,76,101,116,116,101,114,70,114,111,109,68,105,103,105,116,65,116,66,111,117,110,100,97,114,121]));
        let r = AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_letter_digit().unwrap();
        let a = ((AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), UStr::new(&[65]))[0usize]).clone().range).clone();
        let n = ((AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_clusters_with_text((r).clone(), UStr::new(&[57]))[0usize]).clone().range).clone();
        let mut all = true;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_same_range((((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone(), (a).clone()) {
                all = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from(((r.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0) && all, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("only the letter fires: ")); __s += AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_render_auto_space_decisions(&(r.debug).clone().auto_space_decisions).as_ustr(); __s }).as_str()))).unwrap();
        let mut saw = false;
        for i in 0..match u32::try_from((r.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if AutoSpaceSingleGapTestSupport::auto_space_single_gap_test_support_same_range((((r.debug).clone().auto_space_decisions[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone(), (n).clone()) {
                saw = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!saw, Some(UString::from("digit boundary must not fire when cjkDigit disabled"))).unwrap();
    });
}
