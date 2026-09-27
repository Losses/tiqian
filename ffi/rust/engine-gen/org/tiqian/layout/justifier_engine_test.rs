#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::cluster_geometry_decision_info::ClusterGeometryDecisionInfo;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::justification_allocation_info::JustificationAllocationInfo;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::layout::justifier_engine_test_support::JustifierEngineTestSupport;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault) -> Self {
        match value {
            JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault) -> Self {
        match value {
            JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault) -> Self {
        match value {
            JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault) -> Self {
        match value {
            JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault) -> Self {
        match value {
            JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestUsesPunctuationGlueFirstWhenDeficitMatchesCompressionFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault) -> Self {
        match value {
            JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault) -> Self {
        match value {
            JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault) -> Self {
        match value {
            JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault) -> Self {
        match value {
            JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault) -> Self {
        match value {
            JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestUniformTrackingIncludesBracketInnerSidesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault) -> Self {
        match value {
            JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault) -> Self {
        match value {
            JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault) -> Self {
        match value {
            JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault) -> Self {
        match value {
            JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault) -> Self {
        match value {
            JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestTypedSinoWesternSpacesStretchInTierTwoFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault) -> Self {
        match value {
            JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault) -> Self {
        match value {
            JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault) -> Self {
        match value {
            JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault) -> Self {
        match value {
            JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault) -> Self {
        match value {
            JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestPunctuationToWesternBoundaryStretchesInTierThreeFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault) -> Self {
        match value {
            JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault) -> Self {
        match value {
            JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault) -> Self {
        match value {
            JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault) -> Self {
        match value {
            JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault) -> Self {
        match value {
            JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestMandatoryBreakLinesTakeLastLineAlignmentFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault) -> Self {
        match value {
            JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault) -> Self {
        match value {
            JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault) -> Self {
        match value {
            JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault) -> Self {
        match value {
            JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault) -> Self {
        match value {
            JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestLineEdgeSinoWesternSpaceStaysCollapsedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault) -> Self {
        match value {
            JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault) -> Self {
        match value {
            JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault) -> Self {
        match value {
            JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault) -> Self {
        match value {
            JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault) -> Self {
        match value {
            JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestLatinGlyphPositionsSurviveAutospaceAndJustificationFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestLastLineIsNeverJustifiedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestLastLineIsNeverJustifiedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestLastLineIsNeverJustifiedFault) -> Self {
        match value {
            JustifierEngineTestLastLineIsNeverJustifiedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLastLineIsNeverJustifiedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestLastLineIsNeverJustifiedFault) -> Self {
        match value {
            JustifierEngineTestLastLineIsNeverJustifiedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLastLineIsNeverJustifiedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestLastLineIsNeverJustifiedFault) -> Self {
        match value {
            JustifierEngineTestLastLineIsNeverJustifiedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLastLineIsNeverJustifiedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestLastLineIsNeverJustifiedFault) -> Self {
        match value {
            JustifierEngineTestLastLineIsNeverJustifiedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLastLineIsNeverJustifiedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestLastLineIsNeverJustifiedFault) -> Self {
        match value {
            JustifierEngineTestLastLineIsNeverJustifiedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestLastLineIsNeverJustifiedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestLastLineIsNeverJustifiedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestLastLineIsNeverJustifiedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestLastLineIsNeverJustifiedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestLastLineIsNeverJustifiedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestLastLineIsNeverJustifiedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestLastLineIsNeverJustifiedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestLastLineIsNeverJustifiedFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestLastLineIsNeverJustifiedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestLastLineIsNeverJustifiedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault) -> Self {
        match value {
            JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault) -> Self {
        match value {
            JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault) -> Self {
        match value {
            JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault) -> Self {
        match value {
            JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault) -> Self {
        match value {
            JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestLastLineAlignmentPositionsTheLastLineViaIndentFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault) -> Self {
        match value {
            JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault) -> Self {
        match value {
            JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault) -> Self {
        match value {
            JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault) -> Self {
        match value {
            JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault) -> Self {
        match value {
            JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestJustifyDistributesDeficitAcrossPriorityChainFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault) -> Self {
        match value {
            JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault) -> Self {
        match value {
            JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault) -> Self {
        match value {
            JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault) -> Self {
        match value {
            JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault) -> Self {
        match value {
            JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestJustifiesNonLastLineUsingCjkInterCharGapsAsLastResortFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault) -> Self {
        match value {
            JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault) -> Self {
        match value {
            JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault) -> Self {
        match value {
            JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault) -> Self {
        match value {
            JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault) -> Self {
        match value {
            JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestInseparableNumberAndUnitBoundaryAvoidsStretchUnderJustificationFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault) -> Self {
        match value {
            JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault) -> Self {
        match value {
            JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault) -> Self {
        match value {
            JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault) -> Self {
        match value {
            JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault) -> Self {
        match value {
            JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestDashBoundariesDoNotReceiveUniformTrackingFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault) -> Self {
        match value {
            JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault) -> Self {
        match value {
            JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault) -> Self {
        match value {
            JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault) -> Self {
        match value {
            JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault) -> Self {
        match value {
            JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestConnectorBoundariesAvoidStretchUnderJustificationFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault) -> Self {
        match value {
            JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault) -> Self {
        match value {
            JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault) -> Self {
        match value {
            JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault) -> Self {
        match value {
            JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault) -> Self {
        match value {
            JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestCjkInterCharActsAsLastResortWhenPunctGlueExhaustedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault) -> Self {
        match value {
            JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault) -> Self {
        match value {
            JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault) -> Self {
        match value {
            JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault) -> Self {
        match value {
            JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault) -> Self {
        match value {
            JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        JustifierEngineTestBracketWesternInteriorStretchesInTierThreeNotTierTwoFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn connector_boundaries_avoid_stretch_under_justification() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.connectorBoundariesAvoidStretchUnderJustification", "org.tiqian.layout.JustifierEngineTest.connectorBoundariesAvoidStretchUnderJustification", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"connectorBoundariesAvoidStretchUnderJustification");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中～文中Example", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(80 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) >= 2, None).unwrap();
        let d = ((r.debug).clone().justification_decisions[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, d.deficit_after, None).unwrap();
        let mut a: Vec<u32> = vec![];
        for _g_index in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.allocations[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if x.kind.to_string() == "CjkInterChar" {
                a.push((x.cluster_range).clone().start);
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[2]", {
        let mut out = String::new();
        out.push('[');
        let arr = a;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(arr[i]));
            i += 1;
        }
        out.push(']');
        out
    }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, d.allocations[0usize].delta, None).unwrap();
    });
}

#[test]
fn inseparable_number_and_unit_boundary_avoids_stretch_under_justification() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.inseparableNumberAndUnitBoundaryAvoidsStretchUnderJustification", "org.tiqian.layout.JustifierEngineTest.inseparableNumberAndUnitBoundaryAvoidsStretchUnderJustification", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"inseparableNumberAndUnitBoundaryAvoidsStretchUnderJustification");
        let text = "中文50℃中文中文中文Example".to_string();
        let nr = TextRange::new(2u32, 4u32).unwrap();
        let ur = TextRange::new(4u32, 5u32).unwrap();
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(128 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut n: Option<Cluster> = None;
        let mut u: Option<Cluster> = None;
        for _g_index in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if i32::from_ne_bytes((nr.start).to_ne_bytes()) >= i32::from_ne_bytes(((c.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes((nr.end).to_ne_bytes())) <= i32::from_ne_bytes(((c.range).clone().end).to_ne_bytes()) {
                n = Some(c.clone());
            }
            if i32::from_ne_bytes((ur.start).to_ne_bytes()) >= i32::from_ne_bytes(((c.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes((ur.end).to_ne_bytes())) <= i32::from_ne_bytes(((c.range).clone().end).to_ne_bytes()) {
                u = Some(c.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(n != u, None).unwrap();
        let d = ((r.debug).clone().justification_decisions[0usize]).clone();
        let mut bad = false;
        for _g_index1 in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let a = (d.allocations[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if a.cluster_range.clone() == (n.as_ref().unwrap().range).clone() && ((a.kind).to_string() == "CjkLatinSpace" || (a.kind).to_string() == "CjkInterChar") {
                bad = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!bad, Some((format!("{}{}",
            "50|℃ must stay closed: ",
            JustifierEngineTestSupport::justifier_engine_test_support_allocations_text(&d.allocations)
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, d.deficit_after, None).unwrap();
    });
}

#[test]
fn last_line_alignment_positions_the_last_line_via_indent() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.lastLineAlignmentPositionsTheLastLineViaIndent", "org.tiqian.layout.JustifierEngineTest.lastLineAlignmentPositionsTheLastLineViaIndent", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"lastLineAlignmentPositionsTheLastLineViaIndent");
        let l: Arc<dyn Fn(LastLineAlignment) -> LayoutResult + Send + Sync + 'static> = {  Arc::new(move |a| {
        return JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文中文中文中文中", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(a), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100 as f64
as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
}) };
        let s = l(LastLineAlignment::Start);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(100 as f64, s.lines[0usize].visual_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, s.lines[1usize].indent, None).unwrap();
        let c = l(LastLineAlignment::Center);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(26 as f64, c.lines[1usize].indent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, c.lines[0usize].indent, None).unwrap();
        let x = l(LastLineAlignment::End);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(52 as f64, x.lines[1usize].indent, None).unwrap();
    });
}

#[test]
fn mandatory_break_lines_take_last_line_alignment() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.mandatoryBreakLinesTakeLastLineAlignment", "org.tiqian.layout.JustifierEngineTest.mandatoryBreakLinesTakeLastLineAlignment", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"mandatoryBreakLinesTakeLastLineAlignment");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new(concat!("中文中\n",
"中文中文中文中"), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Center),
Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(26 as f64, r.lines[0usize].indent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, r.lines[1usize].indent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(42 as f64, r.lines[2usize].indent, None).unwrap();
    });
}

#[test]
fn last_line_is_never_justified() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.lastLineIsNeverJustified", "org.tiqian.layout.JustifierEngineTest.lastLineIsNeverJustified", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"lastLineIsNeverJustified");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文中", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()),
Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(80 as f64 as
f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(48 as f64, r.lines[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(48 as f64, r.lines[0usize].visual_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from(((r.debug).clone().justification_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn latin_glyph_positions_survive_autospace_and_justification() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.latinGlyphPositionsSurviveAutospaceAndJustification", "org.tiqian.layout.JustifierEngineTest.latinGlyphPositionsSurviveAutospaceAndJustification", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"latinGlyphPositionsSurviveAutospaceAndJustification");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_positioned().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中AV中文", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(52 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut c: Option<Cluster> = None;
        for _g_index in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (r.clusters[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if x.text.to_string() == "AV" {
                c = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((c.as_ref().unwrap().advance) > (10 as f64), Some((format!("{}{}",
            "autospace/justification should widen the cluster as trailing layout space: ",
            match c { Some(ref v) => v.to_string(), None => "null".to_string() }
        )).to_string())).unwrap();
        let mut ok = false;
        for di in 0..match u32::try_from((r.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().justification_decisions[usize::try_from(di).unwrap_or(0)]).clone();
            for ai in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let al = (d.allocations[usize::try_from(ai).unwrap_or(0)]).clone();
                if al.cluster_range.clone() == (c.as_ref().unwrap().range).clone() && (al.kind).to_string() == "CjkLatinSpace" {
                    ok = true;
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, Some("the test must exercise a justify delta on the Latin cluster".to_string())).unwrap();
        let mut xs: Vec<f64> = vec![];
        let mut adv: Vec<f64> = vec![];
        for ri in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (r.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                if g.cluster_range.clone() == (c.as_ref().unwrap().range).clone() {
                    xs.push(g.x);
                    adv.push(g.advance);
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[0, 5]", format!("{}{}{}",
            "[",
            { let joined2 = xs; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } out },
            "]"
        ).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[5, 5]", format!("{}{}{}",
            "[",
            { let joined5 = adv; let mut out = String::new(); let n = joined5.len(); let mut index5 = 0usize; while index5 < n { if index5 > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined5[index5]); index5 += 1; } out },
            "]"
        ).as_str(), None).unwrap();
    });
}

#[test]
fn justifies_non_last_line_using_cjk_inter_char_gaps_as_last_resort() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.justifiesNonLastLineUsingCjkInterCharGapsAsLastResort", "org.tiqian.layout.JustifierEngineTest.justifiesNonLastLineUsingCjkInterCharGapsAsLastResort", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"justifiesNonLastLineUsingCjkInterCharGapsAsLastResort");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文中文中文", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()),
Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(80 as f64 as
f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(80 as f64, r.lines[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(80 as f64, r.lines[0usize].visual_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, r.lines[1usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, r.lines[1usize].visual_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from(((r.debug).clone().justification_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn uses_punctuation_glue_first_when_deficit_matches_compression() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.usesPunctuationGlueFirstWhenDeficitMatchesCompression", "org.tiqian.layout.JustifierEngineTest.usesPunctuationGlueFirstWhenDeficitMatchesCompression", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"usesPunctuationGlueFirstWhenDeficitMatchesCompression");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中，。文", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()),
Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(64 as f64 as
f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from(((r.debug).clone().justification_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn justify_distributes_deficit_across_priority_chain() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.justifyDistributesDeficitAcrossPriorityChain", "org.tiqian.layout.JustifierEngineTest.justifyDistributesDeficitAcrossPriorityChain", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"justifyDistributesDeficitAcrossPriorityChain");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中」。文中文中文中", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(80 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) >= 2, None).unwrap();
        let d = ((r.debug).clone().justification_decisions[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, d.deficit_before, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, d.deficit_after, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, u32::try_from((d.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut kinds = true;
        let mut deltas = true;
        for _g_index in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let a = (d.allocations[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if a.kind.to_string() != "CjkInterChar" {
                kinds = false;
            }
            if a.delta != 2 as f64 {
                deltas = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(kinds, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(deltas, None).unwrap();
        let mut starts: Vec<u32> = vec![];
        for _g_index1 in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let a = (d.allocations[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            starts.push((a.cluster_range).clone().start);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 1, 2, 3], &starts, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(80 as f64, r.lines[0usize].visual_width, None).unwrap();
        let mut g: Option<ClusterGeometryDecisionInfo> = None;
        for _g_index2 in 0..match u32::try_from((r.debug).clone().geometry_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().geometry_decisions[usize::try_from(_g_index2).unwrap_or(0)]).clone();
            if x.source_text.to_string() == "」" {
                g = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, g.as_ref().unwrap().trailing_glue_consumed, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, g.as_ref().unwrap().justification_delta, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, g.as_ref().unwrap().resolved_advance, None).unwrap();
    });
}

#[test]
fn cjk_inter_char_acts_as_last_resort_when_punct_glue_exhausted() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.cjkInterCharActsAsLastResortWhenPunctGlueExhausted", "org.tiqian.layout.JustifierEngineTest.cjkInterCharActsAsLastResortWhenPunctGlueExhausted", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"cjkInterCharActsAsLastResortWhenPunctGlueExhausted");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文中文中文中文中文中文", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let d = ((r.debug).clone().justification_decisions[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, d.deficit_before, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, d.deficit_after, None).unwrap();
        let mut kinds = true;
        for _g_index in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let a = (d.allocations[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if a.kind.to_string() != "CjkInterChar" {
                kinds = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(kinds, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, u32::try_from((d.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        for _g_index1 in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let a = (d.allocations[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(0.8f64, a.delta, None).unwrap();
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(100 as f64, r.lines[0usize].visual_width, None).unwrap();
    });
}

#[test]
fn uniform_tracking_includes_bracket_inner_sides() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.uniformTrackingIncludesBracketInnerSides", "org.tiqian.layout.JustifierEngineTest.uniformTrackingIncludesBracketInnerSides", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"uniformTrackingIncludesBracketInnerSides");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中（中文）文中文中文中", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let d = ((r.debug).clone().justification_decisions[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, d.deficit_after, None).unwrap();
        let mut starts: Vec<u32> = vec![];
        for _g_index in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let a = (d.allocations[usize::try_from(_g_index).unwrap_or(0)]).clone();
            starts.push((a.cluster_range).clone().start);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 1, 2, 3, 4], &starts, None).unwrap();
        let mut kinds = true;
        for _g_index1 in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let a = (d.allocations[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if a.kind.to_string() != "CjkInterChar" {
                kinds = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(kinds, None).unwrap();
        for _g_index2 in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let a = (d.allocations[usize::try_from(_g_index2).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.8f64, a.delta, 0.01f64, None).unwrap();
        }
    });
}

#[test]
fn bracket_western_interior_stretches_in_tier_three_not_tier_two() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.bracketWesternInteriorStretchesInTierThreeNotTierTwo", "org.tiqian.layout.JustifierEngineTest.bracketWesternInteriorStretchesInTierThreeNotTierTwo", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"bracketWesternInteriorStretchesInTierThreeNotTierTwo");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文（Hello）中文中文", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(170 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let d = ((r.debug).clone().justification_decisions[0usize]).clone();
        let mut latin = false;
        let mut left = false;
        let mut right = false;
        for _g_index in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let a = (d.allocations[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if a.kind.to_string() == "CjkLatinSpace" {
                latin = true;
            }
            if a.kind.to_string() == "CjkInterChar" && (a.cluster_range).clone().start == 2 {
                left = true;
            }
            if a.kind.to_string() == "CjkInterChar" && (a.cluster_range).clone().start == 3 {
                right = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!latin, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(right, None).unwrap();
    });
}

#[test]
fn dash_boundaries_do_not_receive_uniform_tracking() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.dashBoundariesDoNotReceiveUniformTracking", "org.tiqian.layout.JustifierEngineTest.dashBoundariesDoNotReceiveUniformTracking", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"dashBoundariesDoNotReceiveUniformTracking");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("在所谓中文语境下——不如说中文", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(180 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) >= 2, None).unwrap();
        let mut dash: Option<Cluster> = None;
        for _g_index in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if c.text.to_string() == "——" {
                dash = Some(c.clone());
            }
        }
        let mut before: Option<Cluster> = None;
        for i in 1..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().range.clone() == (dash.as_ref().unwrap().range).clone() {
                before = Some((r.clusters[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone());
            }
        }
        let d = ((r.debug).clone().justification_decisions[0usize]).clone();
        let mut bad_before = false;
        let mut bad_after = false;
        for _g_index1 in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let a = (d.allocations[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if a.kind.to_string() == "CjkInterChar" && (a.cluster_range).clone() == (before.as_ref().unwrap().range).clone() {
                bad_before = true;
            }
            if a.kind.to_string() == "CjkInterChar" && (a.cluster_range).clone() == (dash.as_ref().unwrap().range).clone() {
                bad_after = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!bad_before, Some((format!("{}{}",
            "boundary before dash must stay closed: ",
            JustifierEngineTestSupport::justifier_engine_test_support_allocations_text(&d.allocations)
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!bad_after, Some((format!("{}{}",
            "boundary after dash must stay closed: ",
            JustifierEngineTestSupport::justifier_engine_test_support_allocations_text(&d.allocations)
        )).to_string())).unwrap();
    });
}

#[test]
fn typed_sino_western_spaces_stretch_in_tier_two() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.typedSinoWesternSpacesStretchInTierTwo", "org.tiqian.layout.JustifierEngineTest.typedSinoWesternSpacesStretchInTierTwo", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"typedSinoWesternSpacesStretchInTierTwo");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文 Hello 中文中文中文", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(180 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) >= 2, None).unwrap();
        let d = ((r.debug).clone().justification_decisions[0usize]).clone();
        let mut a: Vec<JustificationAllocationInfo> = vec![];
        for _g_index in 0..match u32::try_from(d.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.allocations[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if x.kind.to_string() == "CjkLatinSpace" {
                a.push(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut spaced = true;
        for x in &a {
            let mut c: Option<Cluster> = None;
            for yi in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let y = (r.clusters[usize::try_from(yi).unwrap_or(0)]).clone();
                if y.range.clone().start == (x.cluster_range).clone().start {
                    c = Some(y.clone());
                }
            }
            if c.as_ref().unwrap().text.to_string() != " " {
                spaced = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(spaced, Some("every 中西 stretch lands on a typed space cluster, not a boundary".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(a[0usize].delta == a[1usize].delta, Some("同时、同等量".to_string())).unwrap();
    });
}

#[test]
fn punctuation_to_western_boundary_stretches_in_tier_three() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.punctuationToWesternBoundaryStretchesInTierThree", "org.tiqian.layout.JustifierEngineTest.punctuationToWesternBoundaryStretchesInTierThree", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"punctuationToWesternBoundaryStretchesInTierThree");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("你好「World」你好你好你", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(140 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut ok = false;
        for ai in 0..match u32::try_from(((r.debug).clone().justification_decisions[0usize]).clone().allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let al = (((r.debug).clone().justification_decisions[0usize]).clone().allocations[usize::try_from(ai).unwrap_or(0)]).clone();
            let mut c: Option<Cluster> = None;
            for xi in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let x = (r.clusters[usize::try_from(xi).unwrap_or(0)]).clone();
                if x.range.clone().start == (al.cluster_range).clone().start {
                    c = Some(x.clone());
                }
            }
            if al.kind.to_string() == "CjkInterChar" && (c.as_ref().unwrap().text).to_string() == "「" {
                ok = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, Some("标点↔西文 boundary must stretch in tier ③".to_string())).unwrap();
    });
}

#[test]
fn line_edge_sino_western_space_stays_collapsed() {
    testlib::run("org.tiqian.layout.JustifierEngineTest.lineEdgeSinoWesternSpaceStaysCollapsed", "org.tiqian.layout.JustifierEngineTest.lineEdgeSinoWesternSpaceStaysCollapsed", || {
        let _ = JustifierEngineTestSupport::justifier_engine_test_support_start(&"lineEdgeSinoWesternSpaceStaysCollapsed");
        let r = JustifierEngineTestSupport::justifier_engine_test_support_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文中文 word 中文中", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(80 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        for i in 0..u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
            let mut edge: Option<Cluster> = None;
            for _g_index in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let c = (r.clusters[usize::try_from(_g_index).unwrap_or(0)]).clone();
                if i32::from_ne_bytes(((c.range).clone().start).to_ne_bytes()) < (i32::from_ne_bytes((((r.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())) {
                    edge = Some(c.clone());
                }
            }
            if edge.as_ref().unwrap().text.to_string() == " " {
                let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, edge.as_ref().unwrap().advance, Some("line-edge sino-western space must stay collapsed".to_string())).unwrap();
            }
        }
    });
}
