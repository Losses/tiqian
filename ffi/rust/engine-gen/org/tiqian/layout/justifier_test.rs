#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::inline_object_preferred_stretch_kind::InlineObjectPreferredStretchKind;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_east_asian_spacing::UnicodeEastAsianSpacing;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::justifier::JustificationPlan;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::punctuation_model::GlueKind;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::std::u_string_exception::UStringFault;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault) -> Self {
        match value {
            JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault) -> Self {
        match value {
            JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault) -> Self {
        match value {
            JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestWesternDominantLineDoesNotStretchAroundCjkPunctuationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault) -> Self {
        match value {
            JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault) -> Self {
        match value {
            JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault) -> Self {
        match value {
            JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestWesternBracketsTouchingCjkShareTierThreeStretchFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault) -> Self {
        match value {
            JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault) -> Self {
        match value {
            JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault) -> Self {
        match value {
            JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestVirtualSinoWesternStretchRequiresAlphaNumericBoundaryCharFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault) -> Self {
        match value {
            JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault) -> Self {
        match value {
            JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault) -> Self {
        match value {
            JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestTypedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault) -> Self {
        match value {
            JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault) -> Self {
        match value {
            JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault) -> Self {
        match value {
            JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestTypedSinoWesternSpaceStretchesInTierTwoFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault) -> Self {
        match value {
            JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault) -> Self {
        match value {
            JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault) -> Self {
        match value {
            JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestTypedSinoWesternSpaceIsCappedAtHalfEmFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault) -> Self {
        match value {
            JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault) -> Self {
        match value {
            JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault) -> Self {
        match value {
            JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestSinoWesternStretchRespectsThirdEmCapWhenStyleSetsItFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault) -> Self {
        match value {
            JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault) -> Self {
        match value {
            JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault) -> Self {
        match value {
            JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestMixedCjkLineStillStretchesPunctuationWesternBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault) -> Self {
        match value {
            JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault) -> Self {
        match value {
            JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault) -> Self {
        match value {
            JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestInseparableNumberSymbolBoundaryNeverStretchesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault) -> Self {
        match value {
            JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault) -> Self {
        match value {
            JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault) -> Self {
        match value {
            JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestFormulaBoundariesStretchPunctuationThenRelationsThenBinaryOperatorsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault) -> Self {
        match value {
            JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault) -> Self {
        match value {
            JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault) -> Self {
        match value {
            JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestFixedSinoWesternGapDoesNotJoinFinalUniformSpacingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault) -> Self {
        match value {
            JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault) -> Self {
        match value {
            JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault) -> Self {
        match value {
            JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestFinalUniformSpacingIncludesWordAndSinoWesternGapsOnceEachFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault) -> Self {
        match value {
            JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault) -> Self {
        match value {
            JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault) -> Self {
        match value {
            JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestExplicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault) -> Self {
        match value {
            JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault) -> Self {
        match value {
            JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault) -> Self {
        match value {
            JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierTestAttachedReferenceUsesTheVirtualProseBoundaryForStretchingFault::TracedAssertionsFailFaultFault(value)
    }
}

pub static JUSTIFIER_TEST_SUPPORT_EM: Mutex<f64> = Mutex::new(16.0f64);

#[derive(Clone, Copy)]
pub struct JustifierTestSupport;

impl JustifierTestSupport {

    pub fn justifier_test_support_cjk(at: u32) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(at, u32::wrapping_add(at, 1))?, "中", "cjk", { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn justifier_test_support_space(at: u32) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(at, u32::wrapping_add(at, 1))?, " ", "latin", 0.25f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn justifier_test_support_latin(at: u32, w: f64) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(at, u32::wrapping_add(at, 2))?, "Hi", "latin", w, Some("Hi".to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn justifier_test_support_slash_latin(at: u32, w: f64) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(at, u32::wrapping_add(at, 3))?, "/Hi", "latin", w, Some("/Hi".to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn justifier_test_support_punctuation(at: u32, text: Option<String>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(at, u32::wrapping_add(at, 1))?, match &(text) { None => "（".to_string(), Some(__option) => __option.to_string() }.as_str(), "cjk", { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() },
Some(match &(text) { None => "（".to_string(), Some(__option1) => __option1.to_string() }), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn justifier_test_support_western_bracket(at: u32, text: &str) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(at, u32::wrapping_add(at, 1))?, text, "latin", 0.5f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, Some(text.to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn justifier_test_support_inline_object(at: u32, text: &str) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(at, u32::wrapping_add(at, u_string::unit_count(&(text))))?, text, "inline-object", 2.0f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, Some("".to_string()), Some(0.0),
Some(0.0), Some(0.0)));
    }

    pub fn justifier_test_support_spacing_edges(clusters: &Vec<Cluster>) -> Result<Vec<EastAsianSpacingEdges>, TextRangeError> {
        let mut a: Vec<EastAsianSpacingEdges> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            a.push(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(((clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_string().as_str(), &"zh-Hans")?);
            i = u32::wrapping_add(i, 1);
        }
        return Ok(a);
    }

    pub fn justifier_test_support_natural(c: &Vec<Cluster>) -> f64 {
        let mut n = 0.0f64;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            n += c[usize::try_from(i).unwrap_or(0)].advance;
            i = u32::wrapping_add(i, 1);
        }
        return n;
    }

    pub fn justifier_test_support_render_target_indexes(plan: JustificationPlan, kind: GlueKind) -> String {
        let mut s = "[".to_string();
        let mut first = true;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((plan.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let a = (plan.allocations[usize::try_from(i).unwrap_or(0)]).clone();
            if a.kind == kind {
                if !first {
                    s += &(", ");
                }
                s += &(a.target_cluster_index).to_string();
                first = false;
            }
            i = u32::wrapping_add(i, 1);
        }
        return format!("{}{}",
            s,
            "]"
        );
    }

    pub fn justifier_test_support_render_kinds(plan: JustificationPlan) -> String {
        let mut s = "[".to_string();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((plan.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                s += &(", ");
            }
            s += &(plan.allocations[usize::try_from(i).unwrap_or(0)].kind.name().to_string());
            i = u32::wrapping_add(i, 1);
        }
        return format!("{}{}",
            s,
            "]"
        );
    }

    pub fn justifier_test_support_render_deltas(plan: JustificationPlan) -> Result<String, UStringFault> {
        let mut s = "[".to_string();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((plan.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                s += &(", ");
            }
            s += &(TestTraceRender::test_trace_render_render_float(plan.allocations[usize::try_from(i).unwrap_or(0)].delta)?);
            i = u32::wrapping_add(i, 1);
        }
        return Ok(format!("{}{}",
            s,
            "]"
        ));
    }

    pub fn justifier_test_support_set_ints(values: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            b.put(&(values[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn justifier_test_support_roles(n: u32, role: FontRole) -> Vec<FontRole> {
        let mut a: Vec<FontRole> = vec![];
        for _ in 0..n {
            a.push(role);
        }
        return a;
    }
}

#[test]
fn western_dominant_line_does_not_stretch_around_cjk_punctuation() {
    testlib::run("org.tiqian.layout.JustifierTest.westernDominantLineDoesNotStretchAroundCjkPunctuation", "org.tiqian.layout.JustifierTest.westernDominantLineDoesNotStretchAroundCjkPunctuation", || {
        TestTraceRecorder::new("JustifierTest").section(&"westernDominantLineDoesNotStretchAroundCjkPunctuation");
        let clusters = vec![
    (JustifierTestSupport::justifier_test_support_latin(0, 3.0f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_punctuation(2, None).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_latin(3, 3.0f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_punctuation(5, Some("）".to_string())).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_punctuation(6, Some("、".to_string())).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_latin(7, 3.0f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
];
        let roles = vec![
    FontRole::LatinText,
    FontRole::CjkPunctuation,
    FontRole::LatinText,
    FontRole::CjkPunctuation,
    FontRole::CjkPunctuation,
    FontRole::LatinText,
];
        let mut natural = 0.0f64;
        let mut ni = 0u32;
        while (i32::from_ne_bytes((ni).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            natural += clusters[usize::try_from(ni).unwrap_or(0)].advance;
            ni = u32::wrapping_add(ni, 1);
        }
        let plan = Justifier::new(Some(0.5), Some(0.25)).justify(&clusters, &roles, &JustifierTestSupport::justifier_test_support_spacing_edges(&clusters).unwrap(), IntRange::new(0u32, u32::wrapping_sub(u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)), natural +
2.0f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false, None.clone(), Some(true), 0.25f64 * { let __guard =
JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, 0.5f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut none_cjk_inter_char = true;
        let mut ai = 0u32;
        while (i32::from_ne_bytes((ai).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((plan.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if plan.allocations[usize::try_from(ai).unwrap_or(0)].kind == GlueKind::CjkInterChar {
                none_cjk_inter_char = false;
                break;
            }
            ai = u32::wrapping_add(ai, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_cjk_inter_char, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(2.0f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, plan.unfilled_deficit, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"WesternDominantLineNaturalSpacing", (plan.fallback_reason).as_deref().unwrap_or(""), None).unwrap();
    });
}

#[test]
fn explicit_inline_object_boundaries_share_uniform_stretch_on_formula_only_line() {
    testlib::run("org.tiqian.layout.JustifierTest.explicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLine", "org.tiqian.layout.JustifierTest.explicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLine", || {
        TestTraceRecorder::new("JustifierTest").section(&"explicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLine");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_inline_object(0, &"a+").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_inline_object(2, &"b=").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_inline_object(4, &"c").unwrap()).clone(),
];
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        b.put(&(0));
        b.put(&(1));
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![FontRole::Unknown, FontRole::Unknown, FontRole::Unknown], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 2u32),
JustifierTestSupport::justifier_test_support_natural(&c) + { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false,
None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, None, None, Some(b.clone().build()), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0 as f64, p.unfilled_deficit, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[0, 1]", JustifierTestSupport::justifier_test_support_render_target_indexes((p).clone(), GlueKind::InlineObjectBoundary).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 2, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(8 as f64, p.allocations[0usize].delta, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(8 as f64, p.allocations[1usize].delta, 0.001f64, None).unwrap();
    });
}

#[test]
fn formula_boundaries_stretch_punctuation_then_relations_then_binary_operators() {
    testlib::run("org.tiqian.layout.JustifierTest.formulaBoundariesStretchPunctuationThenRelationsThenBinaryOperators", "org.tiqian.layout.JustifierTest.formulaBoundariesStretchPunctuationThenRelationsThenBinaryOperators", || {
        TestTraceRecorder::new("JustifierTest").section(&"formulaBoundariesStretchPunctuationThenRelationsThenBinaryOperators");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_inline_object(0, &"a").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_inline_object(1, &",").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_inline_object(2, &"b").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_inline_object(3, &"=").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_inline_object(4, &"c").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_inline_object(5, &"+").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_inline_object(6, &"d").unwrap()).clone(),
];
        let mut b: SortedMapTableBuilder<u32, InlineObjectPreferredStretch> = SortedTable::sorted_table_map_builder::<u32, InlineObjectPreferredStretch>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        b.put(&(1), &(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 1 as f64 as f64, 8 as f64 as f64).unwrap()));
        b.put(&(2), &(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 2 as f64 as f64, 8 as f64 as f64).unwrap()));
        b.put(&(3), &(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 2 as f64 as f64, 8 as f64 as f64).unwrap()));
        b.put(&(4), &(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::BinaryOperator, 3 as f64 as f64, 8 as f64 as f64).unwrap()));
        b.put(&(5), &(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::BinaryOperator, 3 as f64 as f64, 8 as f64 as f64).unwrap()));
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &JustifierTestSupport::justifier_test_support_roles(7, FontRole::Unknown), &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 6u32),
JustifierTestSupport::justifier_test_support_natural(&c) + format!("{}", (24i32)).parse::<f64>().unwrap_or(0.0), { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None,
None, None, None, None, None, Some(b.clone().build()), None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[InlineObjectPunctuationTrailing, InlineObjectRelation, InlineObjectRelation, InlineObjectBinaryOperator, InlineObjectBinaryOperator]",
JustifierTestSupport::justifier_test_support_render_kinds((p).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[7, 6, 6, 2.500000, 2.500000]", JustifierTestSupport::justifier_test_support_render_deltas((p).clone()).unwrap().as_str(), None).unwrap();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (3) {
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(8 as f64, p.allocations[usize::try_from(i).unwrap_or(0)].delta + format!("{}", { let v: u32 = if i == 0 { 1 } else { 2 }; i32::from_ne_bytes(v.to_ne_bytes()) }).parse::<f64>().unwrap_or(0.0),
0.001f64, None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(p.allocations[1usize].delta, p.allocations[2usize].delta, 0.001f64, Some("both relation sides must stretch by exactly the same amount".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
        let q = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &JustifierTestSupport::justifier_test_support_roles(7, FontRole::Unknown), &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 6u32),
JustifierTestSupport::justifier_test_support_natural(&c) + format!("{}", (34i32)).parse::<f64>().unwrap_or(0.0), { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None,
None, None, None, None, Some(JustifierTestSupport::justifier_test_support_set_ints(&vec![1, 2, 3, 4, 5])), Some(b.clone().build()), None, None, None).unwrap();
        let mut sum = 0.0f64;
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (5) {
            sum += q.allocations[usize::try_from(i).unwrap_or(0)].delta;
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(29 as f64, sum, None).unwrap();
        let mut u: Vec<u32> = vec![];
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((q.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if q.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::InlineObjectBoundary {
                u.push(q.allocations[usize::try_from(i).unwrap_or(0)].target_cluster_index);
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set_unordered(&vec![1, 2, 3, 4, 5], &u, None).unwrap();
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((u.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(1 as f64, q.allocations[usize::try_from(u32::wrapping_add(5, i)).unwrap_or(0)].delta, 0.001f64, None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let widths = vec![1, 2, 2, 3, 3];
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (5) {
            let mut total = 0.0f64;
            let mut j = 0u32;
            while (i32::from_ne_bytes((j).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((q.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                if q.allocations[usize::try_from(j).unwrap_or(0)].target_cluster_index == p.allocations[usize::try_from(i).unwrap_or(0)].target_cluster_index {
                    total += q.allocations[usize::try_from(j).unwrap_or(0)].delta;
                }
                j = u32::wrapping_add(j, 1);
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(9 as f64, format!("{}", { let v: u32 = widths[usize::try_from(i).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }).parse::<f64>().unwrap_or(0.0) + total, 0.001f64, None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, q.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn mixed_cjk_line_still_stretches_punctuation_western_boundary() {
    testlib::run("org.tiqian.layout.JustifierTest.mixedCjkLineStillStretchesPunctuationWesternBoundary", "org.tiqian.layout.JustifierTest.mixedCjkLineStillStretchesPunctuationWesternBoundary", || {
        TestTraceRecorder::new("JustifierTest").section(&"mixedCjkLineStillStretchesPunctuationWesternBoundary");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_latin(0, format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_punctuation(2, None).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(3).unwrap()).clone(),
];
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![FontRole::LatinText, FontRole::CjkPunctuation, FontRole::CjkText], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 2u32),
JustifierTestSupport::justifier_test_support_natural(&c) + 0.5f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() },
false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut has = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkInterChar && p.allocations[usize::try_from(i).unwrap_or(0)].target_cluster_index == 0 {
                has = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has, Some("mixed CJK lines retain punctuation-western tier-3 tracking".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
        let fallback_reason = p.fallback_reason.clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"-", match &(fallback_reason) { None => "-".to_string(), Some(__option7) => __option7.to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn typed_sino_western_space_stretches_in_tier_two() {
    testlib::run("org.tiqian.layout.JustifierTest.typedSinoWesternSpaceStretchesInTierTwo", "org.tiqian.layout.JustifierTest.typedSinoWesternSpaceStretchesInTierTwo", || {
        TestTraceRecorder::new("JustifierTest").section(&"typedSinoWesternSpaceStretchesInTierTwo");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_cjk(0).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_space(1).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_latin(2, format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
];
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 2u32),
JustifierTestSupport::justifier_test_support_natural(&c) + 0.2f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() },
false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
        let a = (p.allocations[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, a.target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkLatinSpace", a.kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.2f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, a.delta, 0.001f64, None).unwrap();
    });
}

#[test]
fn typed_sino_western_space_is_capped_at_half_em() {
    testlib::run("org.tiqian.layout.JustifierTest.typedSinoWesternSpaceIsCappedAtHalfEm", "org.tiqian.layout.JustifierTest.typedSinoWesternSpaceIsCappedAtHalfEm", || {
        TestTraceRecorder::new("JustifierTest").section(&"typedSinoWesternSpaceIsCappedAtHalfEm");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_cjk(0).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_space(1).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_latin(2, format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(3).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(4).unwrap()).clone(),
];
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::CjkText,
    FontRole::CjkText,
], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 4u32), JustifierTestSupport::justifier_test_support_natural(&c) + format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard =
JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, None, None,
None, None, None, None, None).unwrap();
        let mut sino_idx: Vec<u32> = vec![];
        let mut si = 0u32;
        while (i32::from_ne_bytes((si).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(si).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                sino_idx.push(p.allocations[usize::try_from(si).unwrap_or(0)].target_cluster_index);
            }
            si = u32::wrapping_add(si, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set_unordered(&vec![1, 2], &sino_idx, None).unwrap();
        si = 0u32;
        while (i32::from_ne_bytes((si).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(si).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.25f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, p.allocations[usize::try_from(si).unwrap_or(0)].delta, 0.001f64,
None).unwrap();
            }
            si = u32::wrapping_add(si, 1);
        }
        let mut uniform_count = 0u32;
        let mut ui = 0u32;
        while (i32::from_ne_bytes((ui).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(ui).unwrap_or(0)].kind == GlueKind::CjkInterChar {
                uniform_count = u32::wrapping_add(uniform_count, 1);
            }
            ui = u32::wrapping_add(ui, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(3, uniform_count, None).unwrap();
        let mut uniform_idx: Vec<u32> = vec![];
        ui = 0u32;
        while (i32::from_ne_bytes((ui).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(ui).unwrap_or(0)].kind == GlueKind::CjkInterChar {
                uniform_idx.push(p.allocations[usize::try_from(ui).unwrap_or(0)].target_cluster_index);
            }
            ui = u32::wrapping_add(ui, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set_unordered(&vec![1, 2, 3], &uniform_idx, None).unwrap();
        ui = 0u32;
        while (i32::from_ne_bytes((ui).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(ui).unwrap_or(0)].kind == GlueKind::CjkInterChar {
                let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.5f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, p.allocations[usize::try_from(ui).unwrap_or(0)].delta, 0.001f64,
None).unwrap();
            }
            ui = u32::wrapping_add(ui, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn final_uniform_spacing_includes_word_and_sino_western_gaps_once_each() {
    testlib::run("org.tiqian.layout.JustifierTest.finalUniformSpacingIncludesWordAndSinoWesternGapsOnceEach", "org.tiqian.layout.JustifierTest.finalUniformSpacingIncludesWordAndSinoWesternGapsOnceEach", || {
        TestTraceRecorder::new("JustifierTest").section(&"finalUniformSpacingIncludesWordAndSinoWesternGapsOnceEach");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_cjk(0).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_latin(1, format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_space(3).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_latin(4, format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(6).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(7).unwrap()).clone(),
];
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::CjkText,
    FontRole::CjkText,
], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 5u32), JustifierTestSupport::justifier_test_support_natural(&c) + 2.25f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, {
let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut a: Vec<u32> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::WordSpace {
                a.push(p.allocations[usize::try_from(i).unwrap_or(0)].target_cluster_index);
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[2]", JustifierTestSupport::justifier_test_support_render_target_indexes((p).clone(), GlueKind::WordSpace).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.25f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, p.allocations[0usize].delta, 0.001f64, None).unwrap();
        let mut s: Vec<u32> = vec![];
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                s.push(p.allocations[usize::try_from(i).unwrap_or(0)].target_cluster_index);
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set_unordered(&vec![0, 3], &s, None).unwrap();
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((s.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.25f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, p.allocations[usize::try_from(u32::wrapping_add(1, i)).unwrap_or(0)].delta,
0.001f64, None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let mut u: Vec<u32> = vec![];
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkInterChar {
                u.push(p.allocations[usize::try_from(i).unwrap_or(0)].target_cluster_index);
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[0, 3, 4, 2]", JustifierTestSupport::justifier_test_support_render_target_indexes((p).clone(), GlueKind::CjkInterChar).as_str(), None).unwrap();
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((u.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.375f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, p.allocations[usize::try_from(u32::wrapping_add(3, i)).unwrap_or(0)].delta,
0.001f64, None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn western_brackets_touching_cjk_share_tier_three_stretch() {
    testlib::run("org.tiqian.layout.JustifierTest.westernBracketsTouchingCjkShareTierThreeStretch", "org.tiqian.layout.JustifierTest.westernBracketsTouchingCjkShareTierThreeStretch", || {
        TestTraceRecorder::new("JustifierTest").section(&"westernBracketsTouchingCjkShareTierThreeStretch");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_cjk(0).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_western_bracket(1, &"(").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(2).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_western_bracket(3, &")").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(4).unwrap()).clone(),
];
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (4) {
            b.put(&(i));
            i = u32::wrapping_add(i, 1);
        }
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::CjkText,
], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 4u32), JustifierTestSupport::justifier_test_support_natural(&c) + { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard
= JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, Some(b.clone().build()), None, None, None, None, None, None, None, None).unwrap();
        let mut a: Vec<u32> = vec![];
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkInterChar {
                a.push(p.allocations[usize::try_from(i).unwrap_or(0)].target_cluster_index);
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set_unordered(&vec![0, 1, 2, 3], &a, None).unwrap();
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"WesternBracketCjkInterChar", ((p.allocations[usize::try_from(i).unwrap_or(0)]).clone().reason).to_string().as_str(), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.25f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, p.allocations[usize::try_from(i).unwrap_or(0)].delta, 0.001f64, None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn attached_reference_uses_the_virtual_prose_boundary_for_stretching() {
    testlib::run("org.tiqian.layout.JustifierTest.attachedReferenceUsesTheVirtualProseBoundaryForStretching", "org.tiqian.layout.JustifierTest.attachedReferenceUsesTheVirtualProseBoundaryForStretching", || {
        TestTraceRecorder::new("JustifierTest").section(&"attachedReferenceUsesTheVirtualProseBoundaryForStretching");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_cjk(0).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_western_bracket(1, &"[").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_latin(2, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_western_bracket(4, &"]").unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(5).unwrap()).clone(),
];
        let mut b: SortedMapTableBuilder<u32, u32> = SortedTable::sorted_table_map_builder::<u32, u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        b.put(&(3), &(0));
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::CjkText,
], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 4u32), JustifierTestSupport::justifier_test_support_natural(&c) + { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard
= JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, Some(b.clone().build()), None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, p.allocations[0usize].target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualInterChar", ((p.allocations[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance({ let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, p.allocations[0usize].delta, 0.001f64, None).unwrap();
        let q = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::CjkText,
], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 3u32), (JustifierTestSupport::justifier_test_support_natural(&c) - c[4usize].advance) + { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() }, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, Some(b.clone().build()), None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((q.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn inseparable_number_symbol_boundary_never_stretches() {
    testlib::run("org.tiqian.layout.JustifierTest.inseparableNumberSymbolBoundaryNeverStretches", "org.tiqian.layout.JustifierTest.inseparableNumberSymbolBoundaryNeverStretches", || {
        TestTraceRecorder::new("JustifierTest").section(&"inseparableNumberSymbolBoundaryNeverStretches");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_cjk(0).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_latin(1, format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_punctuation(3, Some("%".to_string())).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(4).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(5).unwrap()).clone(),
];
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        b.put(&(1));
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![
    FontRole::CjkText,
    FontRole::LatinText,
    FontRole::CjkPunctuation,
    FontRole::CjkText,
    FontRole::CjkText,
], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 4u32), JustifierTestSupport::justifier_test_support_natural(&c) + { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard
= JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false, None.clone(), Some(true), 0.25f64, 0.5f64, None, Some(b.clone().build()), None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
        let mut ok = true;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].target_cluster_index == 1 && (p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkLatinSpace || p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkInterChar) {
                ok = false;
            }
            i = u32::wrapping_add(i, 1);
        }
        let mut allocation_parts: Vec<String> = vec![];
        {
            let _g1 = p.allocations.clone();
            for allocation in &_g1 {
                allocation_parts.push(allocation.to_string());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((format!("{}{}",
            "50|% must stay closed: ",
            TestTraceRender::test_trace_render_legacy_list_text(&allocation_parts)
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn fixed_sino_western_gap_does_not_join_final_uniform_spacing() {
    testlib::run("org.tiqian.layout.JustifierTest.fixedSinoWesternGapDoesNotJoinFinalUniformSpacing", "org.tiqian.layout.JustifierTest.fixedSinoWesternGapDoesNotJoinFinalUniformSpacing", || {
        TestTraceRecorder::new("JustifierTest").section(&"fixedSinoWesternGapDoesNotJoinFinalUniformSpacing");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_cjk(0).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_latin(1, format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(3).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(4).unwrap()).clone(),
];
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::CjkText, FontRole::CjkText], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 3u32),
JustifierTestSupport::justifier_test_support_natural(&c) + { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false,
None.clone(), Some(false), 0.25f64, 0.5f64, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkInterChar", p.allocations[0usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, p.allocations[0usize].target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance({ let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, p.allocations[0usize].delta, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn virtual_sino_western_stretch_requires_alpha_numeric_boundary_char() {
    testlib::run("org.tiqian.layout.JustifierTest.virtualSinoWesternStretchRequiresAlphaNumericBoundaryChar", "org.tiqian.layout.JustifierTest.virtualSinoWesternStretchRequiresAlphaNumericBoundaryChar", || {
        TestTraceRecorder::new("JustifierTest").section(&"virtualSinoWesternStretchRequiresAlphaNumericBoundaryChar");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_cjk(0).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_slash_latin(1, format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_cjk(4).unwrap()).clone(),
];
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::CjkText], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 2u32),
JustifierTestSupport::justifier_test_support_natural(&c) + 0.2f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() },
false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[1]", JustifierTestSupport::justifier_test_support_render_target_indexes((p).clone(), GlueKind::CjkLatinSpace).as_str(), None).unwrap();
    });
}

#[test]
fn typed_space_before_slash_led_latin_run_is_not_sino_western_gap() {
    testlib::run("org.tiqian.layout.JustifierTest.typedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGap", "org.tiqian.layout.JustifierTest.typedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGap", || {
        TestTraceRecorder::new("JustifierTest").section(&"typedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGap");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_cjk(0).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_space(1).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_slash_latin(2, format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
];
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 2u32),
JustifierTestSupport::justifier_test_support_natural(&c) + 0.2f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() },
false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut ok = true;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                ok = false;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn sino_western_stretch_respects_third_em_cap_when_style_sets_it() {
    testlib::run("org.tiqian.layout.JustifierTest.sinoWesternStretchRespectsThirdEmCapWhenStyleSetsIt", "org.tiqian.layout.JustifierTest.sinoWesternStretchRespectsThirdEmCapWhenStyleSetsIt", || {
        TestTraceRecorder::new("JustifierTest").section(&"sinoWesternStretchRespectsThirdEmCapWhenStyleSetsIt");
        let _ = JustifierTestSupport;
        let c = vec![
    (JustifierTestSupport::justifier_test_support_cjk(0).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_space(1).unwrap()).clone(),
    (JustifierTestSupport::justifier_test_support_latin(2, format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap()).clone(),
];
        let p = Justifier::new(Some(0.5), Some(0.25)).justify(&c, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText], &JustifierTestSupport::justifier_test_support_spacing_edges(&c).unwrap(), IntRange::new(0u32, 2u32),
JustifierTestSupport::justifier_test_support_natural(&c) + { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, false,
None.clone(), Some(true), 0.25f64, 0.333333333333333315f64, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.0833333333333333148f64 * { let __guard = JUSTIFIER_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, p.allocations[0usize].delta, 0.001f64, None).unwrap();
    });
}
