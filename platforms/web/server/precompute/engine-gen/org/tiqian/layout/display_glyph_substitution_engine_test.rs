#![cfg(test)]

use crate::org::tiqian::clreq::cjk_punctuation_glyph_policy::CjkPunctuationGlyphPolicy;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DashInkOverrideShaper;
use crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupport;
use crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::MissingGlyphReportingShaper;
use crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::PerGlyphQuoteRunShaper;
use crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::SingleClusterAmbiguousShaper;
use crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::SingleClusterNoBoundsShaper;
use crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::TwoGlyphEllipsisShaper;
use crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::UnverifiedCoverageReportingShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportDefaultEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::SupportDefaultEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::SupportDefaultEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault> for DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::SupportDefaultEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestUsesTwoEmAdvanceForRecommendedDashCodepointFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionRollsBackToSourceTextWhenFontLacksTheGlyphFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportDefaultEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::SupportDefaultEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::SupportDefaultEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault> for DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::SupportDefaultEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSubstitutionIsKeptWhenFontCoversTheGlyphFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportDefaultEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::SupportDefaultEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::SupportDefaultEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault> for DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::SupportDefaultEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestStubShaperReportsProfileFallbackWhenInkBoundsAreUnavailableFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestShapingWithoutBoundsProducesNamedProfileFallbackFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportLookaheadShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault),
    SupportFindJustifiedDashHitFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::SupportLookaheadShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::SupportFindJustifiedDashHitFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::SupportLookaheadShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::SupportFindJustifiedDashHitFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault> for DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::SupportLookaheadShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault> for DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault) -> Self {
        DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::SupportFindJustifiedDashHitFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestRolledBackDashStillKeepsItsBoundariesClosedUnderJustificationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportDefaultEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::SupportDefaultEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::SupportDefaultEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault> for DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::SupportDefaultEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesSourceTextWhenUsingClreqRecommendedDisplayGlyphsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsTextRangeArrayFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsTextRangeArrayFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TracedAssertionsAssertEqualsTextRangeArrayFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsTextRangeArrayFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TracedAssertionsAssertEqualsTextRangeArrayFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsTextRangeArrayFault> for DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsTextRangeArrayFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TracedAssertionsAssertEqualsTextRangeArrayFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestPreservesOpenTypeFeaturesAsFinalGlyphRunBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TracedAssertionsAssertEqualsIntFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TracedAssertionsAssertEqualsIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault> for DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault) -> Self {
        DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TracedAssertionsAssertEqualsIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestMultiCharacterPunctuationUsesCharacterLocalInkBoundsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportProfileEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::SupportProfileEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::SupportProfileEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault> for DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::SupportProfileEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestHonorsProfilePunctuationGlyphPolicyFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestEllipsisSubstitutionRollsBackWhenCoverageCannotBeVerifiedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvanceFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyphFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvanceFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatToleranceFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TracedAssertionsAssertEqualsFloatToleranceFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TracedAssertionsAssertEqualsFloatToleranceFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault> for DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TracedAssertionsAssertEqualsFloatToleranceFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfillsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestDashCoverageTargetUsesTheDashSpanFontSizeFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportProfileEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsStringArrayFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringArrayFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::SupportProfileEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsAssertEqualsIntFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsAssertEqualsStringArrayFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::SupportProfileEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsAssertEqualsIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringArrayFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsAssertEqualsStringArrayFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault> for DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::SupportProfileEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault> for DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault) -> Self {
        DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsAssertEqualsIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringArrayFault> for DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringArrayFault) -> Self {
        DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsAssertEqualsStringArrayFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestCoalesceSetIsDrivenByProfileFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportShaperEngineFault(crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::SupportShaperEngineFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TracedAssertionsAssertEqualsIntFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault> for crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::SupportShaperEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TracedAssertionsAssertEqualsIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault {
    fn from(value: crate::org::tiqian::layout::display_glyph_substitution_engine_test_support::DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::SupportShaperEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault> for DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault) -> Self {
        DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TracedAssertionsAssertEqualsIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestAmbiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReasonFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn ambiguous_glyph_cluster_mapping_falls_back_to_policy_with_recorded_reason() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.ambiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReason", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.ambiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReason", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[97,109,98,105,103,117,111,117,115,71,108,121,112,104,67,108,117,115,116,101,114,77,97,112,112,105,110,103,70,97,108,108,115,66,97,99,107,84,111,80,111,108,105,99,121,87,105,116,104,82,101,99,111,114,100,101,100,82,101,97,115,111,110]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(SingleClusterAmbiguousShaper::new()))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[8230,8230])).unwrap();
        let punctuation_decisions = ((result.debug).clone().punctuation_decisions).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((punctuation_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        for p in &punctuation_decisions {
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,114,111,102,105,108,101,71,108,117,101,70,97,108,108,98,97,99,107,87,105,116,104,111,117,116,70,111,110,116,71,101,111,109,101,116,114,121]), (p.geometry_source).to_ustring().as_ustr(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("source for '")); __s += (p.char).to_ustring().as_ustr(); __s += &(UString::from("'")); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[103,108,121,112,104,45,99,108,117,115,116,101,114,45,109,97,112,112,105,110,103,45,97,109,98,105,103,117,111,117,115]), ((p.ink_bounds_fallback).clone()).as_deref().unwrap_or(UStr::new(&[])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("fallback for '")); __s += (p.char).to_ustring().as_ustr(); __s += &(UString::from("'")); __s }).as_str()))).unwrap();
        }
    });
}

#[test]
fn coalesce_set_is_driven_by_profile() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.coalesceSetIsDrivenByProfile", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.coalesceSetIsDrivenByProfile", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[99,111,97,108,101,115,99,101,83,101,116,73,115,68,114,105,118,101,110,66,121,80,114,111,102,105,108,101]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_profile_engine(CjkPunctuationGlyphPolicy::PreserveInput, Some(vec![])).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[8212,8212])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((result.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212]), ((result.clusters[0usize]).clone().text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212]), ((result.clusters[1usize]).clone().text).to_ustring().as_ustr(), None).unwrap();
        let latin = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[65,8212,8212,66])).unwrap();
        let mut latin_texts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(latin.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            latin_texts.push(((latin.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![
    UString::from("A").to_ustring(),
    UString::from("—").to_ustring(),
    UString::from("—").to_ustring(),
    UString::from("B").to_ustring(),
], &latin_texts, None).unwrap();
    });
}

#[test]
fn dash_coverage_target_uses_the_dash_span_font_size() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashCoverageTargetUsesTheDashSpanFontSize", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashCoverageTargetUsesTheDashSpanFontSize", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[100,97,115,104,67,111,118,101,114,97,103,101,84,97,114,103,101,116,85,115,101,115,84,104,101,68,97,115,104,83,112,97,110,70,111,110,116,83,105,122,101]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(DashInkOverrideShaper::new(32 as f64 as f64, Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967278u32) as i32).to_ne_bytes()) as f64 as f64, 31 as f64 as f64, i32::from_ne_bytes(((4294967282u32) as i32).to_ne_bytes()) as f64 as f64), true)))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320_with_spans(&mut engine, UStr::new(&[20013,8212,8212,25991]), &vec![
    (TextSpan::new(TextRange::new(1u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(32 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), UStr::new(&[8212,8212])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn dash_ink_centers_within_the_two_em_body_when_the_font_rule_underfills() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfills", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfills", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[100,97,115,104,73,110,107,67,101,110,116,101,114,115,87,105,116,104,105,110,84,104,101,84,119,111,69,109,66,111,100,121,87,104,101,110,84,104,101,70,111,110,116,82,117,108,101,85,110,100,101,114,102,105,108,108,115]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(DashInkOverrideShaper::new(32 as f64 as f64, Rect::new(0.5f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 28 as f64 as f64, i32::from_ne_bytes(((4294967288u32) as i32).to_ne_bytes()) as f64 as f64), false)))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[20013,8212,8212,25991])).unwrap();
        let dash = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), UStr::new(&[8212,8212])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[11834]), (dash.display_text).to_ustring().as_ustr(), None).unwrap();
        let glyph = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_glyph_with_cluster_range((result).clone(), (dash.range).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(1.75f64, glyph.x, 0.01f64, None).unwrap();
    });
}

#[test]
fn dash_substitution_is_kept_when_ink_fills_the_two_em_advance() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvance", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvance", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[100,97,115,104,83,117,98,115,116,105,116,117,116,105,111,110,73,115,75,101,112,116,87,104,101,110,73,110,107,70,105,108,108,115,84,104,101,84,119,111,69,109,65,100,118,97,110,99,101]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(DashInkOverrideShaper::new(32 as f64 as f64, Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 31 as f64 as f64, i32::from_ne_bytes(((4294967288u32) as i32).to_ne_bytes()) as f64 as f64), false)))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[20013,8212,8212,25991])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[11834]), (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), UStr::new(&[8212,8212])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn dash_substitution_rolls_back_when_fallback_reports_a_full_one_em_glyph() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyph", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyph", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[100,97,115,104,83,117,98,115,116,105,116,117,116,105,111,110,82,111,108,108,115,66,97,99,107,87,104,101,110,70,97,108,108,98,97,99,107,82,101,112,111,114,116,115,65,70,117,108,108,79,110,101,69,109,71,108,121,112,104]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(DashInkOverrideShaper::new(16 as f64 as f64, Rect::new(0.5f64, i32::from_ne_bytes(((4294967287u32) as i32).to_ne_bytes()) as f64 as f64, 15.7f64, i32::from_ne_bytes(((4294967289u32) as i32).to_ne_bytes()) as f64 as f64), true)))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[20013,8212,8212,25991])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), UStr::new(&[8212,8212])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_font_decision_with_source_text((result).clone(), UStr::new(&[8212,8212])).unwrap().substitution_reason).to_ustring()).ends_with(&UString::from("DashSubstitutionInkCoverageRollback")), None).unwrap();
    });
}

#[test]
fn dash_substitution_rolls_back_when_ink_does_not_fill_the_two_em_advance() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvance", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvance", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[100,97,115,104,83,117,98,115,116,105,116,117,116,105,111,110,82,111,108,108,115,66,97,99,107,87,104,101,110,73,110,107,68,111,101,115,78,111,116,70,105,108,108,84,104,101,84,119,111,69,109,65,100,118,97,110,99,101]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(DashInkOverrideShaper::new(32 as f64 as f64, Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 26 as f64 as f64, i32::from_ne_bytes(((4294967288u32) as i32).to_ne_bytes()) as f64 as f64), false)))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[20013,8212,8212,25991])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), UStr::new(&[8212,8212])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_font_decision_with_source_text((result).clone(), UStr::new(&[8212,8212])).unwrap().substitution_reason).to_ustring()).ends_with(&UString::from("DashSubstitutionInkCoverageRollback")), None).unwrap();
    });
}

#[test]
fn ellipsis_substitution_rolls_back_when_coverage_cannot_be_verified() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.ellipsisSubstitutionRollsBackWhenCoverageCannotBeVerified", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.ellipsisSubstitutionRollsBackWhenCoverageCannotBeVerified", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[101,108,108,105,112,115,105,115,83,117,98,115,116,105,116,117,116,105,111,110,82,111,108,108,115,66,97,99,107,87,104,101,110,67,111,118,101,114,97,103,101,67,97,110,110,111,116,66,101,86,101,114,105,102,105,101,100]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(UnverifiedCoverageReportingShaper::new()))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[20013,8230,8230,25991])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8230,8230]), (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), UStr::new(&[8230,8230])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_font_decision_with_source_text((result).clone(), UStr::new(&[8230,8230])).unwrap().substitution_reason).to_ustring()).ends_with(&UString::from("SubstitutionRollbackOnUnverifiedGlyphCoverage")), None).unwrap();
    });
}

#[test]
fn honors_profile_punctuation_glyph_policy() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.honorsProfilePunctuationGlyphPolicy", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.honorsProfilePunctuationGlyphPolicy", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[104,111,110,111,114,115,80,114,111,102,105,108,101,80,117,110,99,116,117,97,116,105,111,110,71,108,121,112,104,80,111,108,105,99,121]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_profile_engine(CjkPunctuationGlyphPolicy::PreserveInput, None).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[8230,8230,8212,8212])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8230,8230]), (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), UStr::new(&[8230,8230])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), UStr::new(&[8212,8212])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn multi_character_punctuation_uses_character_local_ink_bounds() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.multiCharacterPunctuationUsesCharacterLocalInkBounds", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.multiCharacterPunctuationUsesCharacterLocalInkBounds", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[109,117,108,116,105,67,104,97,114,97,99,116,101,114,80,117,110,99,116,117,97,116,105,111,110,85,115,101,115,67,104,97,114,97,99,116,101,114,76,111,99,97,108,73,110,107,66,111,117,110,100,115]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(TwoGlyphEllipsisShaper::new()))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[8230,8230])).unwrap();
        let decisions = ((result.debug).clone().punctuation_decisions).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut ink_centers: Vec<Option<f64>> = vec![];
        let mut advances: Vec<Option<f64>> = vec![];
        for i in 0..match u32::try_from(decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            ink_centers.push(decisions[usize::try_from(i).unwrap_or(0)].ink_center);
            advances.push(Some(decisions[usize::try_from(i).unwrap_or(0)].advance));
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_nullable_floats(&vec![Some(8.0f64), Some(8.0f64)]).unwrap().as_ustr(), DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_nullable_floats(&ink_centers).unwrap().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_nullable_floats(&vec![Some(16.0f64), Some(16.0f64)]).unwrap().as_ustr(), DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_nullable_floats(&advances).unwrap().as_ustr(), None).unwrap();
    });
}

#[test]
fn preserves_open_type_features_as_final_glyph_run_boundaries() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.preservesOpenTypeFeaturesAsFinalGlyphRunBoundaries", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.preservesOpenTypeFeaturesAsFinalGlyphRunBoundaries", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,114,101,115,101,114,118,101,115,79,112,101,110,84,121,112,101,70,101,97,116,117,114,101,115,65,115,70,105,110,97,108,71,108,121,112,104,82,117,110,66,111,117,110,100,97,114,105,101,115]));
        let proportional_quote_features = vec![UString::from("pwid").to_ustring(), UString::from("palt").to_ustring()];
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(PerGlyphQuoteRunShaper::new()))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[65,8217,66])).unwrap();
        let mut ranges: Vec<TextRange> = vec![];
        let mut feature_lists: Vec<Vec<UString>> = vec![];
        for i in 0..match u32::try_from(result.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            ranges.push(((result.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
            feature_lists.push((result.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone().open_type_features.clone());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_text_range_array(&vec![
    (TextRange::new(0u32, 1u32).unwrap()).clone(),
    (TextRange::new(1u32, 2u32).unwrap()).clone(),
    (TextRange::new(2u32, 3u32).unwrap()).clone(),
], &ranges, None).unwrap();
        let empty_features: Vec<UString> = vec![];
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_string_list_array(&vec![(empty_features).clone(), (proportional_quote_features).clone(), (empty_features).clone()]).unwrap().as_ustr(), DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_string_list_array(&feature_lists).unwrap().as_ustr(), None).unwrap();
    });
}

#[test]
fn preserves_source_text_when_using_clreq_recommended_display_glyphs() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.preservesSourceTextWhenUsingClreqRecommendedDisplayGlyphs", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.preservesSourceTextWhenUsingClreqRecommendedDisplayGlyphs", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,114,101,115,101,114,118,101,115,83,111,117,114,99,101,84,101,120,116,87,104,101,110,85,115,105,110,103,67,108,114,101,113,82,101,99,111,109,109,101,110,100,101,100,68,105,115,112,108,97,121,71,108,121,112,104,115]));
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_default_engine().unwrap(), UStr::new(&[8230,8230,8212,8212,12539,65295])).unwrap();
        let ellipsis = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), UStr::new(&[8230,8230])).unwrap();
        let dash = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), UStr::new(&[8212,8212])).unwrap();
        let interpunct = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), UStr::new(&[12539])).unwrap();
        let solidus = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), UStr::new(&[65295])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8230,8230]), (ellipsis.text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8943,8943]), (ellipsis.display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), (dash.text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[11834]), (dash.display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[12539]), (interpunct.text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[183]), (interpunct.display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65295]), (solidus.text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65295]), (solidus.display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,106,107,45,112,114,105,109,97,114,121]), (ellipsis.font_key).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,106,107,45,112,114,105,109,97,114,121]), (dash.font_key).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,106,107,45,112,114,105,109,97,114,121]), (interpunct.font_key).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,106,107,45,112,114,105,109,97,114,121]), (solidus.font_key).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn rolled_back_dash_still_keeps_its_boundaries_closed_under_justification() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.rolledBackDashStillKeepsItsBoundariesClosedUnderJustification", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.rolledBackDashStillKeepsItsBoundariesClosedUnderJustification", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[114,111,108,108,101,100,66,97,99,107,68,97,115,104,83,116,105,108,108,75,101,101,112,115,73,116,115,66,111,117,110,100,97,114,105,101,115,67,108,111,115,101,100,85,110,100,101,114,74,117,115,116,105,102,105,99,97,116,105,111,110]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_lookahead_shaper_engine(Arc::new(Mutex::new(DashInkOverrideShaper::new(32 as f64 as f64, Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 26 as f64 as f64, i32::from_ne_bytes(((4294967288u32) as i32).to_ne_bytes()) as f64 as f64), false)))).unwrap();
        let hit = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_find_justified_dash_hit(&mut engine, UStr::new(&[22312,25152,35859,20013,25991,35821,22659,19979,8212,8212,19981,22914,35828,20013,25991,20013,25991,20013,25991,20013,25991])).unwrap();
        let dash = (hit.dash).clone().clone();
        let decision = (hit.decision).clone().clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), (dash.display_text).to_ustring().as_ustr(), None).unwrap();
        let mut opened = false;
        for i in 0..match u32::try_from(decision.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let allocation = (decision.allocations[usize::try_from(i).unwrap_or(0)]).clone();
            if allocation.kind.to_ustring() == UString::from("CjkInterChar") && (allocation.cluster_range).clone().start == (dash.range).clone().start && (allocation.cluster_range).clone().end == (dash.range).clone().end {
                opened = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!opened, Some(UString::from("boundary after a rolled-back dash must stay closed: ${decision.allocations}"))).unwrap();
    });
}

#[test]
fn shaping_without_bounds_produces_named_profile_fallback() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.shapingWithoutBoundsProducesNamedProfileFallback", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.shapingWithoutBoundsProducesNamedProfileFallback", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[115,104,97,112,105,110,103,87,105,116,104,111,117,116,66,111,117,110,100,115,80,114,111,100,117,99,101,115,78,97,109,101,100,80,114,111,102,105,108,101,70,97,108,108,98,97,99,107]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(SingleClusterNoBoundsShaper::new()))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[12290])).unwrap();
        let punctuation = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_punctuation_decision((result).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,114,111,102,105,108,101,71,108,117,101,70,97,108,108,98,97,99,107,87,105,116,104,111,117,116,70,111,110,116,71,101,111,109,101,116,114,121]), (punctuation.geometry_source).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[115,104,97,112,101,114,45,110,111,45,105,110,107,45,98,111,117,110,100,115]), (punctuation.ink_bounds_fallback).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, punctuation.body_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, punctuation.leading_glue_natural, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, punctuation.trailing_glue_natural, None).unwrap();
    });
}

#[test]
fn stub_shaper_reports_profile_fallback_when_ink_bounds_are_unavailable() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.stubShaperReportsProfileFallbackWhenInkBoundsAreUnavailable", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.stubShaperReportsProfileFallbackWhenInkBoundsAreUnavailable", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[115,116,117,98,83,104,97,112,101,114,82,101,112,111,114,116,115,80,114,111,102,105,108,101,70,97,108,108,98,97,99,107,87,104,101,110,73,110,107,66,111,117,110,100,115,65,114,101,85,110,97,118,97,105,108,97,98,108,101]));
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_default_engine().unwrap(), UStr::new(&[20013,25991,65292,19990,30028,12290])).unwrap();
        let punctuation_decisions = ((result.debug).clone().punctuation_decisions).clone();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((punctuation_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
        for p in &punctuation_decisions {
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,114,111,102,105,108,101,71,108,117,101,70,97,108,108,98,97,99,107,87,105,116,104,111,117,116,70,111,110,116,71,101,111,109,101,116,114,121]), (p.geometry_source).to_ustring().as_ustr(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Stub shaper provides advance but no bounds for '")); __s += (p.char).to_ustring().as_ustr(); __s += &(UString::from("'")); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[115,104,97,112,101,114,45,110,111,45,105,110,107,45,98,111,117,110,100,115]), ((p.ink_bounds_fallback).clone()).as_deref().unwrap_or(UStr::new(&[])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("fallback for '")); __s += (p.char).to_ustring().as_ustr(); __s += &(UString::from("'")); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.leading_glue_natural, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("leading glue for '")); __s += (p.char).to_ustring().as_ustr(); __s += &(UString::from("'")); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, p.trailing_glue_natural, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("trailing glue for '")); __s += (p.char).to_ustring().as_ustr(); __s += &(UString::from("'")); __s }).as_str()))).unwrap();
        }
    });
}

#[test]
fn substitution_is_kept_when_font_covers_the_glyph() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.substitutionIsKeptWhenFontCoversTheGlyph", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.substitutionIsKeptWhenFontCoversTheGlyph", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[115,117,98,115,116,105,116,117,116,105,111,110,73,115,75,101,112,116,87,104,101,110,70,111,110,116,67,111,118,101,114,115,84,104,101,71,108,121,112,104]));
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_default_engine().unwrap(), UStr::new(&[20013,8212,8212,25991])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[11834]), (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), UStr::new(&[8212,8212])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn substitution_rolls_back_to_source_text_when_font_lacks_the_glyph() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.substitutionRollsBackToSourceTextWhenFontLacksTheGlyph", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.substitutionRollsBackToSourceTextWhenFontLacksTheGlyph", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[115,117,98,115,116,105,116,117,116,105,111,110,82,111,108,108,115,66,97,99,107,84,111,83,111,117,114,99,101,84,101,120,116,87,104,101,110,70,111,110,116,76,97,99,107,115,84,104,101,71,108,121,112,104]));
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Arc::new(Mutex::new(MissingGlyphReportingShaper::new()))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, UStr::new(&[20013,8212,8212,25991])).unwrap();
        let dash_cluster = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), UStr::new(&[8212,8212])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), (dash_cluster.display_text).to_ustring().as_ustr(), None).unwrap();
        let dash_decision = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_font_decision_with_source_text((result).clone(), UStr::new(&[8212,8212])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), (dash_decision.display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((dash_decision.substitution_reason).to_ustring()).ends_with(&UString::from("SubstitutionRollbackOnMissingGlyph")), None).unwrap();
    });
}

#[test]
fn uses_two_em_advance_for_recommended_dash_codepoint() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.usesTwoEmAdvanceForRecommendedDashCodepoint", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.usesTwoEmAdvanceForRecommendedDashCodepoint", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[68,105,115,112,108,97,121,71,108,121,112,104,83,117,98,115,116,105,116,117,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[117,115,101,115,84,119,111,69,109,65,100,118,97,110,99,101,70,111,114,82,101,99,111,109,109,101,110,100,101,100,68,97,115,104,67,111,100,101,112,111,105,110,116]));
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_default_engine().unwrap(), UStr::new(&[11834])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster((result).clone()).unwrap().advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, (result.size).clone().width, None).unwrap();
    });
}
