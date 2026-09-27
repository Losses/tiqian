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
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"ambiguousGlyphClusterMappingFallsBackToPolicyWithRecordedReason");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(SingleClusterAmbiguousShaper::new())).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"……").unwrap();
        let punctuation_decisions = ((result.debug).clone().punctuation_decisions).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((punctuation_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        for p in &punctuation_decisions {
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ProfileGlueFallbackWithoutFontGeometry", (p.geometry_source).to_string().as_str(), Some((format!("{}{}{}",
            "source for '",
            (p.char).to_string(),
            "'"
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"glyph-cluster-mapping-ambiguous", ((p.ink_bounds_fallback).clone()).as_deref().unwrap_or(""), Some((format!("{}{}{}",
            "fallback for '",
            (p.char).to_string(),
            "'"
        )).to_string())).unwrap();
        }
    });
}

#[test]
fn coalesce_set_is_driven_by_profile() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.coalesceSetIsDrivenByProfile", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.coalesceSetIsDrivenByProfile", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"coalesceSetIsDrivenByProfile");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_profile_engine(CjkPunctuationGlyphPolicy::PreserveInput, Some(vec![])).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"——").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((result.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"—", ((result.clusters[0usize]).clone().text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"—", ((result.clusters[1usize]).clone().text).to_string().as_str(), None).unwrap();
        let latin = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"A——B").unwrap();
        let mut latin_texts: Vec<String> = vec![];
        for i in 0..match u32::try_from(latin.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            latin_texts.push(((latin.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_string());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["A".to_string(), "—".to_string(), "—".to_string(), "B".to_string()], &latin_texts, None).unwrap();
    });
}

#[test]
fn dash_coverage_target_uses_the_dash_span_font_size() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashCoverageTargetUsesTheDashSpanFontSize", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashCoverageTargetUsesTheDashSpanFontSize", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"dashCoverageTargetUsesTheDashSpanFontSize");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(DashInkOverrideShaper::new(32 as f64 as f64, Rect::new(1 as f64 as f64, i32::from_ne_bytes((4294967278u32).to_ne_bytes()) as f64 as f64, 31 as
f64 as f64, i32::from_ne_bytes((4294967282u32).to_ne_bytes()) as f64 as f64), true))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320_with_spans(&mut engine, &"中——文", &vec![
    (TextSpan::new(TextRange::new(1u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(32 as f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), &"——").unwrap().display_text).to_string().as_str(),
None).unwrap();
    });
}

#[test]
fn dash_ink_centers_within_the_two_em_body_when_the_font_rule_underfills() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfills", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfills", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"dashInkCentersWithinTheTwoEmBodyWhenTheFontRuleUnderfills");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(DashInkOverrideShaper::new(32 as f64 as f64, Rect::new(0.5f64, i32::from_ne_bytes((4294967286u32).to_ne_bytes()) as f64 as f64, 28 as f64 as
f64, i32::from_ne_bytes((4294967288u32).to_ne_bytes()) as f64 as f64), false))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"中——文").unwrap();
        let dash = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), &"——").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"⸺", (dash.display_text).to_string().as_str(), None).unwrap();
        let glyph = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_glyph_with_cluster_range((result).clone(), (dash.range).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(1.75f64, glyph.x, 0.01f64, None).unwrap();
    });
}

#[test]
fn dash_substitution_is_kept_when_ink_fills_the_two_em_advance() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvance", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvance", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"dashSubstitutionIsKeptWhenInkFillsTheTwoEmAdvance");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(DashInkOverrideShaper::new(32 as f64 as f64, Rect::new(1 as f64 as f64, i32::from_ne_bytes((4294967286u32).to_ne_bytes()) as f64 as f64, 31 as
f64 as f64, i32::from_ne_bytes((4294967288u32).to_ne_bytes()) as f64 as f64), false))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"中——文").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"⸺", (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), &"——").unwrap().display_text).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn dash_substitution_rolls_back_when_fallback_reports_a_full_one_em_glyph() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyph", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyph", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"dashSubstitutionRollsBackWhenFallbackReportsAFullOneEmGlyph");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(DashInkOverrideShaper::new(16 as f64 as f64, Rect::new(0.5f64, i32::from_ne_bytes((4294967287u32).to_ne_bytes()) as f64 as f64, 15.7f64,
i32::from_ne_bytes((4294967289u32).to_ne_bytes()) as f64 as f64), true))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"中——文").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), &"——").unwrap().display_text).to_string().as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_font_decision_with_source_text((result).clone(),
&"——").unwrap().substitution_reason).to_string()).ends_with(&"DashSubstitutionInkCoverageRollback"), None).unwrap();
    });
}

#[test]
fn dash_substitution_rolls_back_when_ink_does_not_fill_the_two_em_advance() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvance", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.dashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvance", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"dashSubstitutionRollsBackWhenInkDoesNotFillTheTwoEmAdvance");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(DashInkOverrideShaper::new(32 as f64 as f64, Rect::new(1 as f64 as f64, i32::from_ne_bytes((4294967286u32).to_ne_bytes()) as f64 as f64, 26 as
f64 as f64, i32::from_ne_bytes((4294967288u32).to_ne_bytes()) as f64 as f64), false))).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"中——文").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), &"——").unwrap().display_text).to_string().as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_font_decision_with_source_text((result).clone(),
&"——").unwrap().substitution_reason).to_string()).ends_with(&"DashSubstitutionInkCoverageRollback"), None).unwrap();
    });
}

#[test]
fn ellipsis_substitution_rolls_back_when_coverage_cannot_be_verified() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.ellipsisSubstitutionRollsBackWhenCoverageCannotBeVerified", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.ellipsisSubstitutionRollsBackWhenCoverageCannotBeVerified", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"ellipsisSubstitutionRollsBackWhenCoverageCannotBeVerified");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(UnverifiedCoverageReportingShaper::new())).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"中……文").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"……", (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), &"……").unwrap().display_text).to_string().as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_font_decision_with_source_text((result).clone(),
&"……").unwrap().substitution_reason).to_string()).ends_with(&"SubstitutionRollbackOnUnverifiedGlyphCoverage"), None).unwrap();
    });
}

#[test]
fn honors_profile_punctuation_glyph_policy() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.honorsProfilePunctuationGlyphPolicy", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.honorsProfilePunctuationGlyphPolicy", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"honorsProfilePunctuationGlyphPolicy");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_profile_engine(CjkPunctuationGlyphPolicy::PreserveInput, None).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"……——").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"……", (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), &"……").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), &"——").unwrap().display_text).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn multi_character_punctuation_uses_character_local_ink_bounds() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.multiCharacterPunctuationUsesCharacterLocalInkBounds", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.multiCharacterPunctuationUsesCharacterLocalInkBounds", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"multiCharacterPunctuationUsesCharacterLocalInkBounds");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(TwoGlyphEllipsisShaper::new())).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"……").unwrap();
        let decisions = ((result.debug).clone().punctuation_decisions).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut ink_centers: Vec<Option<f64>> = vec![];
        let mut advances: Vec<Option<f64>> = vec![];
        for i in 0..match u32::try_from(decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            ink_centers.push(decisions[usize::try_from(i).unwrap_or(0)].ink_center);
            advances.push(Some(decisions[usize::try_from(i).unwrap_or(0)].advance));
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_nullable_floats(&vec![Some(8.0f64), Some(8.0f64)]).unwrap().as_str(),
DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_nullable_floats(&ink_centers).unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_nullable_floats(&vec![Some(16.0f64), Some(16.0f64)]).unwrap().as_str(),
DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_nullable_floats(&advances).unwrap().as_str(), None).unwrap();
    });
}

#[test]
fn preserves_open_type_features_as_final_glyph_run_boundaries() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.preservesOpenTypeFeaturesAsFinalGlyphRunBoundaries", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.preservesOpenTypeFeaturesAsFinalGlyphRunBoundaries", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"preservesOpenTypeFeaturesAsFinalGlyphRunBoundaries");
        let proportional_quote_features = vec!["pwid".to_string(), "palt".to_string()];
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(PerGlyphQuoteRunShaper::new())).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"A’B").unwrap();
        let mut ranges: Vec<TextRange> = vec![];
        let mut feature_lists: Vec<Vec<String>> = vec![];
        for i in 0..match u32::try_from(result.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            ranges.push(((result.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
            feature_lists.push((result.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone().open_type_features.clone());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_text_range_array(&vec![
    (TextRange::new(0u32, 1u32).unwrap()).clone(),
    (TextRange::new(1u32, 2u32).unwrap()).clone(),
    (TextRange::new(2u32, 3u32).unwrap()).clone(),
], &ranges, None).unwrap();
        let empty_features: Vec<String> = vec![];
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_string_list_array(&vec![(empty_features).clone(), (proportional_quote_features).clone(),
(empty_features).clone()]).unwrap().as_str(), DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_render_string_list_array(&feature_lists).unwrap().as_str(), None).unwrap();
    });
}

#[test]
fn preserves_source_text_when_using_clreq_recommended_display_glyphs() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.preservesSourceTextWhenUsingClreqRecommendedDisplayGlyphs", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.preservesSourceTextWhenUsingClreqRecommendedDisplayGlyphs", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"preservesSourceTextWhenUsingClreqRecommendedDisplayGlyphs");
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_default_engine().unwrap(), &"……——・／").unwrap();
        let ellipsis = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), &"……").unwrap();
        let dash = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), &"——").unwrap();
        let interpunct = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), &"・").unwrap();
        let solidus = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_cluster_with_text((result).clone(), &"／").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"……", (ellipsis.text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"⋯⋯", (ellipsis.display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", (dash.text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"⸺", (dash.display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"・", (interpunct.text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"·", (interpunct.display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"／", (solidus.text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"／", (solidus.display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"cjk-primary", (ellipsis.font_key).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"cjk-primary", (dash.font_key).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"cjk-primary", (interpunct.font_key).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"cjk-primary", (solidus.font_key).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn rolled_back_dash_still_keeps_its_boundaries_closed_under_justification() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.rolledBackDashStillKeepsItsBoundariesClosedUnderJustification", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.rolledBackDashStillKeepsItsBoundariesClosedUnderJustification", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"rolledBackDashStillKeepsItsBoundariesClosedUnderJustification");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_lookahead_shaper_engine(Box::new(DashInkOverrideShaper::new(32 as f64 as f64, Rect::new(1 as f64 as f64, i32::from_ne_bytes((4294967286u32).to_ne_bytes()) as f64 as
f64, 26 as f64 as f64, i32::from_ne_bytes((4294967288u32).to_ne_bytes()) as f64 as f64), false))).unwrap();
        let hit = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_find_justified_dash_hit(&mut engine, &"在所谓中文语境下——不如说中文中文中文中文").unwrap();
        let dash = (hit.dash).clone().clone();
        let decision = (hit.decision).clone().clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", (dash.display_text).to_string().as_str(), None).unwrap();
        let mut opened = false;
        for i in 0..match u32::try_from(decision.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let allocation = (decision.allocations[usize::try_from(i).unwrap_or(0)]).clone();
            if allocation.kind.to_string() == "CjkInterChar" && (allocation.cluster_range).clone().start == (dash.range).clone().start && (allocation.cluster_range).clone().end == (dash.range).clone().end {
                opened = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!opened, Some("boundary after a rolled-back dash must stay closed: ${decision.allocations}".to_string())).unwrap();
    });
}

#[test]
fn shaping_without_bounds_produces_named_profile_fallback() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.shapingWithoutBoundsProducesNamedProfileFallback", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.shapingWithoutBoundsProducesNamedProfileFallback", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"shapingWithoutBoundsProducesNamedProfileFallback");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(SingleClusterNoBoundsShaper::new())).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"。").unwrap();
        let punctuation = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_punctuation_decision((result).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ProfileGlueFallbackWithoutFontGeometry", (punctuation.geometry_source).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"shaper-no-ink-bounds", (punctuation.ink_bounds_fallback).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, punctuation.body_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, punctuation.leading_glue_natural, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, punctuation.trailing_glue_natural, None).unwrap();
    });
}

#[test]
fn stub_shaper_reports_profile_fallback_when_ink_bounds_are_unavailable() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.stubShaperReportsProfileFallbackWhenInkBoundsAreUnavailable", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.stubShaperReportsProfileFallbackWhenInkBoundsAreUnavailable", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"stubShaperReportsProfileFallbackWhenInkBoundsAreUnavailable");
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_default_engine().unwrap(), &"中文，世界。").unwrap();
        let punctuation_decisions = ((result.debug).clone().punctuation_decisions).clone();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((punctuation_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
        for p in &punctuation_decisions {
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ProfileGlueFallbackWithoutFontGeometry", (p.geometry_source).to_string().as_str(), Some((format!("{}{}{}",
            "Stub shaper provides advance but no bounds for '",
            (p.char).to_string(),
            "'"
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"shaper-no-ink-bounds", ((p.ink_bounds_fallback).clone()).as_deref().unwrap_or(""), Some((format!("{}{}{}",
            "fallback for '",
            (p.char).to_string(),
            "'"
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.leading_glue_natural, Some((format!("{}{}{}",
            "leading glue for '",
            (p.char).to_string(),
            "'"
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, p.trailing_glue_natural, Some((format!("{}{}{}",
            "trailing glue for '",
            (p.char).to_string(),
            "'"
        )).to_string())).unwrap();
        }
    });
}

#[test]
fn substitution_is_kept_when_font_covers_the_glyph() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.substitutionIsKeptWhenFontCoversTheGlyph", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.substitutionIsKeptWhenFontCoversTheGlyph", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"substitutionIsKeptWhenFontCoversTheGlyph");
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_default_engine().unwrap(), &"中——文").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"⸺", (DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), &"——").unwrap().display_text).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn substitution_rolls_back_to_source_text_when_font_lacks_the_glyph() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.substitutionRollsBackToSourceTextWhenFontLacksTheGlyph", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.substitutionRollsBackToSourceTextWhenFontLacksTheGlyph", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"substitutionRollsBackToSourceTextWhenFontLacksTheGlyph");
        let mut engine = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_shaper_engine(Box::new(MissingGlyphReportingShaper::new())).unwrap();
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut engine, &"中——文").unwrap();
        let dash_cluster = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), &"——").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", (dash_cluster.display_text).to_string().as_str(), None).unwrap();
        let dash_decision = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_font_decision_with_source_text((result).clone(), &"——").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", (dash_decision.display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((dash_decision.substitution_reason).to_string()).ends_with(&"SubstitutionRollbackOnMissingGlyph"), None).unwrap();
    });
}

#[test]
fn uses_two_em_advance_for_recommended_dash_codepoint() {
    testlib::run("org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.usesTwoEmAdvanceForRecommendedDashCodepoint", "org.tiqian.layout.DisplayGlyphSubstitutionEngineTest.usesTwoEmAdvanceForRecommendedDashCodepoint", || {
        let mut t = TestTraceRecorder::new("DisplayGlyphSubstitutionEngineTest");
        t.section(&"usesTwoEmAdvanceForRecommendedDashCodepoint");
        let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout320(&mut DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_default_engine().unwrap(), &"⸺").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster((result).clone()).unwrap().advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, (result.size).clone().width, None).unwrap();
    });
}
