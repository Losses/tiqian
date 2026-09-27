#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::inline_object_preferred_stretch_kind::InlineObjectPreferredStretchKind;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::justifier::CompressionPlan;
use crate::org::tiqian::layout::justifier::JustificationAllocation;
use crate::org::tiqian::layout::justifier::JustificationPlan;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_optimization::PushInAllocation;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::punctuation_model::GlueKind;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault) -> Self {
        match value {
            JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault) -> Self {
        match value {
            JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault) -> Self {
        match value {
            JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault) -> Self {
        match value {
            JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestZeroTechnicalStretchCapacityProducesNoOpportunityFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertNullRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault) -> Self {
        match value {
            JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault) -> Self {
        match value {
            JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault) -> Self {
        match value {
            JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault) -> Self {
        match value {
            JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault {
    fn from(value: JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault) -> Self {
        match value {
            JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::TracedAssertionsAssertNullRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault) -> Self {
        match value {
            JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault> for JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault) -> Self {
        JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::TracedAssertionsAssertNullRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestZeroDeficitReturnsAnEmptyPlanWithoutReasonFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault) -> Self {
        match value {
            JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault) -> Self {
        match value {
            JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault) -> Self {
        match value {
            JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault) -> Self {
        match value {
            JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault) -> Self {
        match value {
            JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestZeroCapacitySinoWesternTierDefersEverythingDownwardFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestWordSpaceStretchesWithinItsCapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestWordSpaceStretchesWithinItsCapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestWordSpaceStretchesWithinItsCapFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWordSpaceStretchesWithinItsCapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestWordSpaceStretchesWithinItsCapFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWordSpaceStretchesWithinItsCapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestWordSpaceStretchesWithinItsCapFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWordSpaceStretchesWithinItsCapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: JustifierCoverageTestWordSpaceStretchesWithinItsCapFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWordSpaceStretchesWithinItsCapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestWordSpaceStretchesWithinItsCapFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWordSpaceStretchesWithinItsCapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestWordSpaceStretchesWithinItsCapFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWordSpaceStretchesWithinItsCapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestWordSpaceStretchesWithinItsCapFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestWordSpaceStretchesWithinItsCapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestWordSpaceStretchesWithinItsCapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestWordSpaceStretchesWithinItsCapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for JustifierCoverageTestWordSpaceStretchesWithinItsCapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestWordSpaceStretchesWithinItsCapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestWordSpaceStretchesWithinItsCapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestWordSpaceStretchesWithinItsCapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestWordSpaceStretchesWithinItsCapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault) -> Self {
        match value {
            JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestWordSpaceAtTheCapOrCollapsedIsSkippedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestWesternDominantLineStaysRaggedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestWesternDominantLineStaysRaggedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestWesternDominantLineStaysRaggedFault) -> Self {
        match value {
            JustifierCoverageTestWesternDominantLineStaysRaggedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWesternDominantLineStaysRaggedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestWesternDominantLineStaysRaggedFault) -> Self {
        match value {
            JustifierCoverageTestWesternDominantLineStaysRaggedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWesternDominantLineStaysRaggedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestWesternDominantLineStaysRaggedFault) -> Self {
        match value {
            JustifierCoverageTestWesternDominantLineStaysRaggedFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWesternDominantLineStaysRaggedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestWesternDominantLineStaysRaggedFault) -> Self {
        match value {
            JustifierCoverageTestWesternDominantLineStaysRaggedFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestWesternDominantLineStaysRaggedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestWesternDominantLineStaysRaggedFault) -> Self {
        match value {
            JustifierCoverageTestWesternDominantLineStaysRaggedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestWesternDominantLineStaysRaggedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestWesternDominantLineStaysRaggedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestWesternDominantLineStaysRaggedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestWesternDominantLineStaysRaggedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestWesternDominantLineStaysRaggedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestWesternDominantLineStaysRaggedFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestWesternDominantLineStaysRaggedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestWesternDominantLineStaysRaggedFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestWesternDominantLineStaysRaggedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestWesternDominantLineStaysRaggedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault) -> Self {
        match value {
            JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault) -> Self {
        match value {
            JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault) -> Self {
        match value {
            JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault) -> Self {
        match value {
            JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault) -> Self {
        match value {
            JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault) -> Self {
        match value {
            JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestVirtualSinoWesternGapSkipsProtectedAndTypedEdgesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault) -> Self {
        match value {
            JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault) -> Self {
        match value {
            JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault) -> Self {
        match value {
            JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault) -> Self {
        match value {
            JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault) -> Self {
        match value {
            JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault) -> Self {
        match value {
            JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestUniformTextBoundariesExcludeProtectedClassesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertNullRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault) -> Self {
        match value {
            JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault) -> Self {
        match value {
            JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault {
    fn from(value: JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault) -> Self {
        match value {
            JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::TracedAssertionsAssertNullRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault) -> Self {
        match value {
            JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault) -> Self {
        match value {
            JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault) -> Self {
        match value {
            JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault> for JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault) -> Self {
        JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::TracedAssertionsAssertNullRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestUniformObjectBoundaryOpensTheGateAndFillsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceStretchesFromItsBaseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault) -> Self {
        match value {
            JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestTypedSinoWesternSpaceNeedsBothEdgesToPairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestTechnicalWhitespaceStretchFillsAndStopsTheTierChainFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault) -> Self {
        match value {
            JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestTechnicalWhitespaceRequiresTheWhitespaceTierAndASourceSpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault) -> Self {
        match value {
            JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault) -> Self {
        match value {
            JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault) -> Self {
        match value {
            JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault) -> Self {
        match value {
            JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault) -> Self {
        match value {
            JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestSpaceGapProtectionCoversAllFourDisjunctsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault) -> Self {
        match value {
            JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault) -> Self {
        match value {
            JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault) -> Self {
        match value {
            JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault) -> Self {
        match value {
            JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault) -> Self {
        match value {
            JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault) -> Self {
        match value {
            JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestSkipKeepsTheDeficitAndRecordsTheReasonFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault) -> Self {
        match value {
            JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault) -> Self {
        match value {
            JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault) -> Self {
        match value {
            JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault) -> Self {
        match value {
            JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault) -> Self {
        match value {
            JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestSinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTrackingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectStretchRunsBySemanticKindFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault) -> Self {
        match value {
            JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestPreferredInlineObjectKindsChainUntilFilledFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault) -> Self {
        match value {
            JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault) -> Self {
        match value {
            JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault) -> Self {
        match value {
            JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault) -> Self {
        match value {
            JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault) -> Self {
        match value {
            JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestParagraphEdgeSpaceLinesCoverTheBoundaryGuardsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsIntArrayFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntArrayFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault) -> Self {
        match value {
            JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault) -> Self {
        match value {
            JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntArrayFault {
    fn from(value: JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault) -> Self {
        match value {
            JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault::TracedAssertionsAssertEqualsIntArrayFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault) -> Self {
        match value {
            JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault) -> Self {
        match value {
            JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntArrayFault> for JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntArrayFault) -> Self {
        JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault::TracedAssertionsAssertEqualsIntArrayFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestMixedCapacitySinoWesternOppsSkipZeroCapacityInOverflowFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault) -> Self {
        match value {
            JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault) -> Self {
        match value {
            JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault) -> Self {
        match value {
            JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        JustifierCoverageTestMisalignedRoleAndSpacingListsAreRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault) -> Self {
        match value {
            JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault) -> Self {
        match value {
            JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault) -> Self {
        match value {
            JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault) -> Self {
        match value {
            JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault) -> Self {
        match value {
            JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault) -> Self {
        match value {
            JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestEmptyClusterRangeDefersEveryTierLoopFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault) -> Self {
        match value {
            JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault) -> Self {
        match value {
            JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault) -> Self {
        match value {
            JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault) -> Self {
        match value {
            JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault) -> Self {
        match value {
            JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault) -> Self {
        match value {
            JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestEmergencyTrackingFillsTheResidualForAuthorizedBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertEqualsPushInAllocationArrayFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsPushInAllocationArrayFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault) -> Self {
        match value {
            JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault) -> Self {
        match value {
            JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault) -> Self {
        match value {
            JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault) -> Self {
        match value {
            JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault) -> Self {
        match value {
            JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsPushInAllocationArrayFault {
    fn from(value: JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault) -> Self {
        match value {
            JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TracedAssertionsAssertEqualsPushInAllocationArrayFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault) -> Self {
        match value {
            JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsPushInAllocationArrayFault> for JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsPushInAllocationArrayFault) -> Self {
        JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TracedAssertionsAssertEqualsPushInAllocationArrayFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestCompressEarlyExitsAndFiltersDegenerateInputsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestCompressDistributesTierByTierFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertEqualsPushInAllocationArrayFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsPushInAllocationArrayFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestCompressDistributesTierByTierFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestCompressDistributesTierByTierFault) -> Self {
        match value {
            JustifierCoverageTestCompressDistributesTierByTierFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCompressDistributesTierByTierFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestCompressDistributesTierByTierFault) -> Self {
        match value {
            JustifierCoverageTestCompressDistributesTierByTierFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCompressDistributesTierByTierFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestCompressDistributesTierByTierFault) -> Self {
        match value {
            JustifierCoverageTestCompressDistributesTierByTierFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCompressDistributesTierByTierFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsPushInAllocationArrayFault {
    fn from(value: JustifierCoverageTestCompressDistributesTierByTierFault) -> Self {
        match value {
            JustifierCoverageTestCompressDistributesTierByTierFault::TracedAssertionsAssertEqualsPushInAllocationArrayFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCompressDistributesTierByTierFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestCompressDistributesTierByTierFault) -> Self {
        match value {
            JustifierCoverageTestCompressDistributesTierByTierFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestCompressDistributesTierByTierFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestCompressDistributesTierByTierFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestCompressDistributesTierByTierFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestCompressDistributesTierByTierFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestCompressDistributesTierByTierFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestCompressDistributesTierByTierFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsPushInAllocationArrayFault> for JustifierCoverageTestCompressDistributesTierByTierFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsPushInAllocationArrayFault) -> Self {
        JustifierCoverageTestCompressDistributesTierByTierFault::TracedAssertionsAssertEqualsPushInAllocationArrayFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestCompressDistributesTierByTierFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestCompressDistributesTierByTierFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertNullRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault) -> Self {
        match value {
            JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault) -> Self {
        match value {
            JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault) -> Self {
        match value {
            JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault) -> Self {
        match value {
            JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault {
    fn from(value: JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault) -> Self {
        match value {
            JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::TracedAssertionsAssertNullRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault) -> Self {
        match value {
            JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault> for JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault) -> Self {
        JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::TracedAssertionsAssertNullRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestCjkLineWithNoOpportunitiesReportsUnfilledWithoutFallbackFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualSinoWesternNeedsStretchEnabledFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualInterCharHonoursNoStretchProtectionFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault) -> Self {
        match value {
            JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCoverageTestAttachedInlineVirtualAutoSpaceJoinsTierTwoFault::TracedAssertionsFailFaultFault(value)
    }
}

pub static JUSTIFIER_COVERAGE_TEST_SUPPORT_EM: Mutex<f64> = Mutex::new(16.0f64);

#[derive(Clone, Copy)]
pub struct JustifierCoverageTestSupport;

impl JustifierCoverageTestSupport {

    pub fn justifier_coverage_test_support_c(t: &str, i: u32, a: Option<f64>, f: Option<String>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, u_string::unit_count(&(t))))?, t, match &(f) { None => "k".to_string(), Some(__option) => __option.to_string() }.as_str(), match &(a) { None => { let __guard =
JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, Some(__option1) => *__option1 }, Some(t.to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn justifier_coverage_test_support_e(l: Option<EastAsianSpacingValue>, tr: Option<EastAsianSpacingValue>, w: Option<bool>) -> EastAsianSpacingEdges {
        return EastAsianSpacingEdges::new(match &(l) { None => EastAsianSpacingValue::Other, Some(__option10) => *__option10 }, match &(tr) { None => EastAsianSpacingValue::Other, Some(__option11) => *__option11 }, match &(w) { None => false, Some(__option12) => *__option12 });
    }

    pub fn justifier_coverage_test_support_set(xs: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            b.put(&(xs[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn justifier_coverage_test_support_int_map(xs: &Vec<u32>, ys: &Vec<u32>) -> SortedMapTable<u32, u32> {
        let mut b: SortedMapTableBuilder<u32, u32> = SortedTable::sorted_table_map_builder::<u32, u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            b.put(&(xs[usize::try_from(i).unwrap_or(0)]), &(ys[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn justifier_coverage_test_support_tier_map(xs: &Vec<u32>, ys: &Vec<ProgressiveBreakTier>) -> SortedMapTable<u32, ProgressiveBreakTier> {
        let mut b: SortedMapTableBuilder<u32, ProgressiveBreakTier> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakTier>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            b.put(&(xs[usize::try_from(i).unwrap_or(0)]), &(ys[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn justifier_coverage_test_support_str_map(xs: &Vec<u32>, ys: &Vec<String>) -> SortedMapTable<u32, String> {
        let mut b: SortedMapTableBuilder<u32, String> = SortedTable::sorted_table_map_builder::<u32, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            b.put(&(xs[usize::try_from(i).unwrap_or(0)]), &(ys[usize::try_from(i).unwrap_or(0)]).clone());
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn justifier_coverage_test_support_justify(c: &Vec<Cluster>, r: &Vec<FontRole>, e: &Vec<EastAsianSpacingEdges>, ir: IntRange, m: f64, fs: Option<f64>, sk: Option<bool>, sr: Option<String>, al: Option<bool>, ba: Option<f64>, mx: Option<f64>, ns: Option<SortedSetTable<u32>>,
nsa: Option<SortedSetTable<u32>>, br: Option<SortedSetTable<u32>>, ph: Option<SortedSetTable<u32>>, v: Option<SortedMapTable<u32, u32>>, vs: Option<SortedSetTable<u32>>, uo: Option<SortedSetTable<u32>>, pr: Option<SortedMapTable<u32, InlineObjectPreferredStretch>>, te:
Option<SortedMapTable<u32, ProgressiveBreakTier>>, emg: Option<SortedMapTable<u32, String>>, pem: Option<SortedMapTable<u32, String>>) -> Result<JustificationPlan, TextRangeError> {
        let x = Justifier::new(Some(0.5), Some(0.25));
        return Ok(x.justify(&c, &r, &e, (ir).clone(), m, match &(fs) { None => { let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, Some(__option18) => *__option18 }, match &(sk) { None => false, Some(__option19) =>
*__option19 }, match &(sr) { Some(v) => Some(v.to_string()), None => None }.clone(), Some(match &(al) { None => true, Some(__option20) => *__option20 }), match &(ba) { None => 0.25f64, Some(__option21) => *__option21 }, match &(mx) { None => 0.5f64, Some(__option22) =>
*__option22 }, (ns).clone(), (nsa).clone(), (br).clone(), (ph).clone(), (v).clone(), (vs).clone(), (uo).clone(), (pr).clone(), (te).clone(), (emg).clone(), (pem).clone())?);
    }

    pub fn justifier_coverage_test_support_cjk_cjk() -> Result<JustifierFixture, TextRangeError> {
        return Ok(JustifierFixture { c: vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None)?).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 1, None, None)?).clone(),
], r: vec![FontRole::CjkText, FontRole::CjkText], e: vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
] });
    }

    pub fn justifier_coverage_test_support_cjk_latin() -> Result<JustifierFixture, TextRangeError> {
        return Ok(JustifierFixture { c: vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None)?).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"a", 1, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string()))?).clone(),
], r: vec![FontRole::CjkText, FontRole::LatinText], e: vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
] });
    }

    pub fn justifier_coverage_test_support_latin_space_latin(s: Option<f64>, a: Option<f64>, b: Option<f64>) -> Result<JustifierFixture, TextRangeError> {
        return Ok(JustifierFixture { c: vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"a", 0, Some(match &(a) { None => { let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, Some(__option24) => *__option24 }),
Some("lat".to_string()))?).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&" ", 1, Some(match &(s) { None => 4 as f64, Some(__option26) => *__option26 }), Some("lat".to_string()))?).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"b", 2, Some(match &(b) { None => { let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, Some(__option28) => *__option28 }),
Some("lat".to_string()))?).clone(),
], r: vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText], e: vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
] });
    }
}

#[test]
fn misaligned_role_and_spacing_lists_are_rejected() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.misalignedRoleAndSpacingListsAreRejected", "org.tiqian.layout.JustifierCoverageTest.misalignedRoleAndSpacingListsAreRejected", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"misalignedRoleAndSpacingListsAreRejected");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_cjk_cjk().unwrap();
        let roles3 = vec![f.r[0usize], f.r[1usize], FontRole::LatinText];
        let edges3 = vec![
    (f.e[0usize]).clone(),
    (f.e[1usize]).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(None, None, None)).clone(),
];
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let f = (f).clone(); let roles3 = (roles3).clone(); Arc::new(move || {
        JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &roles3, &f.e, IntRange::new(0u32, 1u32), 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let f = (f).clone(); let edges3 = (edges3).clone(); Arc::new(move || {
        JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &edges3, IntRange::new(0u32, 1u32), 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn skip_keeps_the_deficit_and_records_the_reason() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.skipKeepsTheDeficitAndRecordsTheReason", "org.tiqian.layout.JustifierCoverageTest.skipKeepsTheDeficitAndRecordsTheReason", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"skipKeepsTheDeficitAndRecordsTheReason");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_cjk_cjk().unwrap();
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 64 as f64, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some(true),
Some("RaggedRight".to_string()), None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, p.deficit_before, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, p.unfilled_deficit, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"RaggedRight", (p.fallback_reason).as_deref().unwrap_or(""), None).unwrap();
    });
}

#[test]
fn zero_deficit_returns_an_empty_plan_without_reason() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.zeroDeficitReturnsAnEmptyPlanWithoutReason", "org.tiqian.layout.JustifierCoverageTest.zeroDeficitReturnsAnEmptyPlanWithoutReason", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"zeroDeficitReturnsAnEmptyPlanWithoutReason");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_cjk_cjk().unwrap();
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 32 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.deficit_before, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(p.fallback_reason.is_none(), &"-", None).unwrap();
    });
}

#[test]
fn technical_whitespace_stretch_fills_and_stops_the_tier_chain() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.technicalWhitespaceStretchFillsAndStopsTheTierChain", "org.tiqian.layout.JustifierCoverageTest.technicalWhitespaceStretchFillsAndStopsTheTierChain", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"technicalWhitespaceStretchFillsAndStopsTheTierChain");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_latin_space_latin(Some(2 as f64 as f64), None, None).unwrap();
        let j = Justifier::new(Some(0.5), Some(0.25f64));
        let p = j.justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 2u32), 38 as f64, 16 as f64, false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, None, None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_tier_map(&vec![1],
&vec![ProgressiveBreakTier::Whitespace])), None, None).unwrap();
        let a = (p.allocations[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, a.target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"ProgressiveTechnical", a.kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ProgressiveTechnicalWhitespaceStretch", (a.reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, a.delta, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn technical_whitespace_requires_the_whitespace_tier_and_a_source_space() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.technicalWhitespaceRequiresTheWhitespaceTierAndASourceSpace", "org.tiqian.layout.JustifierCoverageTest.technicalWhitespaceRequiresTheWhitespaceTierAndASourceSpace", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"technicalWhitespaceRequiresTheWhitespaceTierAndASourceSpace");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_latin_space_latin(Some(4 as f64 as f64), None, None).unwrap();
        let wrong_tier = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 2u32), 40 as f64, None, None, None.clone(), None, None, None, None, None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_tier_map(&vec![1], &vec![ProgressiveBreakTier::Structural])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"WordSpace", wrong_tier.allocations[0usize].kind.name().to_string().as_str(), None).unwrap();
        let wrong_cluster = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 2u32), 40 as f64, None, None, None.clone(), None, None, None, None, None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_tier_map(&vec![0], &vec![ProgressiveBreakTier::Whitespace])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"WordSpace", wrong_cluster.allocations[0usize].kind.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn zero_technical_stretch_capacity_produces_no_opportunity() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.zeroTechnicalStretchCapacityProducesNoOpportunity", "org.tiqian.layout.JustifierCoverageTest.zeroTechnicalStretchCapacityProducesNoOpportunity", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"zeroTechnicalStretchCapacityProducesNoOpportunity");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_latin_space_latin(Some(4 as f64 as f64), None, None).unwrap();
        let j = Justifier::new(Some(0.5), Some(0 as f64));
        let p = j.justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 2u32), 40 as f64, 16 as f64, false, None.clone(), Some(true), 0.25f64, 0.5f64, None, None, None, None, None, None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_tier_map(&vec![1],
&vec![ProgressiveBreakTier::Whitespace])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"WordSpace", p.allocations[0usize].kind.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn word_space_stretches_within_its_cap() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.wordSpaceStretchesWithinItsCap", "org.tiqian.layout.JustifierCoverageTest.wordSpaceStretchesWithinItsCap", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"wordSpaceStretchesWithinItsCap");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_latin_space_latin(Some(4 as f64 as f64), None, None).unwrap();
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 2u32), 38 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let a = (p.allocations[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"WordSpace", a.kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, a.target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, a.delta, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"WordSpace", (a.reason).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn word_space_at_the_cap_or_collapsed_is_skipped() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.wordSpaceAtTheCapOrCollapsedIsSkipped", "org.tiqian.layout.JustifierCoverageTest.wordSpaceAtTheCapOrCollapsedIsSkipped", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"wordSpaceAtTheCapOrCollapsedIsSkipped");
        let at_cap = JustifierCoverageTestSupport::justifier_coverage_test_support_latin_space_latin(Some(8 as f64 as f64), None, None).unwrap();
        let at_cap_plan = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&at_cap.c, &at_cap.r, &at_cap.e, IntRange::new(0u32, 2u32), 48 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((at_cap_plan.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"WesternDominantLineNaturalSpacing", (at_cap_plan.fallback_reason).as_deref().unwrap_or(""), None).unwrap();
        let collapsed = JustifierCoverageTestSupport::justifier_coverage_test_support_latin_space_latin(Some(0 as f64 as f64), None, None).unwrap();
        let collapsed_plan = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&collapsed.c, &collapsed.r, &collapsed.e, IntRange::new(0u32, 2u32), 40 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((collapsed_plan.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn space_gap_protection_covers_all_four_disjuncts() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.spaceGapProtectionCoversAllFourDisjuncts", "org.tiqian.layout.JustifierCoverageTest.spaceGapProtectionCoversAllFourDisjuncts", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"spaceGapProtectionCoversAllFourDisjuncts");
        let base = JustifierCoverageTestSupport::justifier_coverage_test_support_latin_space_latin(Some(4 as f64 as f64), None, None).unwrap();
        let c = vec![
    (base.c[0usize]).clone(),
    (base.c[1usize]).clone(),
    (base.c[2usize]).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 3, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"x", 4, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
];
        let r = vec![base.r[0usize], base.r[1usize], base.r[2usize], FontRole::CjkText, FontRole::LatinText];
        let e = vec![
    (base.e[0usize]).clone(),
    (base.e[1usize]).clone(),
    (base.e[2usize]).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let v1 = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 4u32), 72 as f64, None, None, None.clone(), None, None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None,
None, None, None, None, None, None, None).unwrap();
        let mut h1 = true;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((v1.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if v1.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::WordSpace {
                h1 = false;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(h1, Some("expected no word-space allocation for [0]/[]".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, v1.unfilled_deficit, None).unwrap();
        let v2 = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 4u32), 72 as f64, None, None, None.clone(), None, None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![1])), None, None,
None, None, None, None, None, None, None).unwrap();
        let mut h2 = true;
        let mut i2 = 0u32;
        while (i32::from_ne_bytes((i2).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((v2.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if v2.allocations[usize::try_from(i2).unwrap_or(0)].kind == GlueKind::WordSpace {
                h2 = false;
            }
            i2 = u32::wrapping_add(i2, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(h2, Some("expected no word-space allocation for [1]/[]".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, v2.unfilled_deficit, None).unwrap();
        let v3 = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 4u32), 72 as f64, None, None, None.clone(), None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None,
None, None, None, None, None, None, None).unwrap();
        let mut h3 = true;
        let mut i3 = 0u32;
        while (i32::from_ne_bytes((i3).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((v3.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if v3.allocations[usize::try_from(i3).unwrap_or(0)].kind == GlueKind::WordSpace {
                h3 = false;
            }
            i3 = u32::wrapping_add(i3, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(h3, Some("expected no word-space allocation for []/[0]".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, v3.unfilled_deficit, None).unwrap();
        let v4 = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 4u32), 72 as f64, None, None, None.clone(), None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![2])), None, None, None,
None, None, None, None, None, None, None).unwrap();
        let mut h4 = true;
        let mut i4 = 0u32;
        while (i32::from_ne_bytes((i4).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((v4.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if v4.allocations[usize::try_from(i4).unwrap_or(0)].kind == GlueKind::WordSpace {
                h4 = false;
            }
            i4 = u32::wrapping_add(i4, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(h4, Some("expected no word-space allocation for []/[2]".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, v4.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn virtual_sino_western_gap_skips_protected_and_typed_edges() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.virtualSinoWesternGapSkipsProtectedAndTypedEdges", "org.tiqian.layout.JustifierCoverageTest.virtualSinoWesternGapSkipsProtectedAndTypedEdges", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"virtualSinoWesternGapSkipsProtectedAndTypedEdges");
        let tlc = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&" ", 1, Some(4 as f64 as f64), None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"a", 2, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
];
        let tlr = vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText];
        let tle = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Other), Some(EastAsianSpacingValue::Wide), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let typed_left = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&tlc, &tlr, &tle, IntRange::new(0u32, 2u32), 40 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkLatinSpace", typed_left.allocations[0usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, typed_left.allocations[0usize].target_cluster_index, None).unwrap();
        let trc = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&" a", 1, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"b", 2, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
];
        let tre = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Other), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let typed_right = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&trc, &tlr, &tre, IntRange::new(0u32, 2u32), 52 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut ok1 = true;
        let mut i1 = 0u32;
        while (i32::from_ne_bytes((i1).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((typed_right.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if typed_right.allocations[usize::try_from(i1).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                ok1 = false;
            }
            i1 = u32::wrapping_add(i1, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok1, None).unwrap();
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_cjk_latin().unwrap();
        let physical = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None, None, None, None, None).unwrap();
        let mut ok2 = true;
        let mut i2 = 0u32;
        while (i32::from_ne_bytes((i2).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((physical.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if physical.allocations[usize::try_from(i2).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                ok2 = false;
            }
            i2 = u32::wrapping_add(i2, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok2, None).unwrap();
        let closed = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])),
None, None, None, None, None, None, None, None, None).unwrap();
        let mut ok3 = true;
        let mut i3 = 0u32;
        while (i32::from_ne_bytes((i3).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((closed.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if closed.allocations[usize::try_from(i3).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                ok3 = false;
            }
            i3 = u32::wrapping_add(i3, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok3, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((closed.unfilled_deficit) > (0 as f64), None).unwrap();
    });
}

#[test]
fn attached_inline_virtual_auto_space_joins_tier_two() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.attachedInlineVirtualAutoSpaceJoinsTierTwo", "org.tiqian.layout.JustifierCoverageTest.attachedInlineVirtualAutoSpaceJoinsTierTwo", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"attachedInlineVirtualAutoSpaceJoinsTierTwo");
        let c = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"", 1, Some(0 as f64 as f64), Some("obj".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"a", 2, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"b", 3, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::Unknown, FontRole::LatinText, FontRole::LatinText];
        let e = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(None, None, None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let happy = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 52 as f64, None, None, None.clone(), None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![2], &vec![0])), Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![2])), None, None, None, None, None).unwrap();
        let a = (happy.allocations[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkLatinSpace", a.kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualAutoSpace", (a.reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, a.target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, a.delta, None).unwrap();
        let no_previous = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 52 as f64, None, None, None.clone(), None, None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![2])), None, None, None, None, None).unwrap();
        let mut ok1 = true;
        let mut i1 = 0u32;
        while (i32::from_ne_bytes((i1).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((no_previous.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if no_previous.allocations[usize::try_from(i1).unwrap_or(0)].clone().reason.to_string() == "AttachedInlineVirtualAutoSpace" {
                ok1 = false;
            }
            i1 = u32::wrapping_add(i1, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok1, None).unwrap();
        let target_out_of_range = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 36 as f64, None, None, None.clone(), None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![3], &vec![0])), Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![3])), None, None, None, None, None).unwrap();
        let mut ok2 = true;
        let mut i2 = 0u32;
        while (i32::from_ne_bytes((i2).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((target_out_of_range.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if target_out_of_range.allocations[usize::try_from(i2).unwrap_or(0)].clone().reason.to_string() == "AttachedInlineVirtualAutoSpace" {
                ok2 = false;
            }
            i2 = u32::wrapping_add(i2, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok2, None).unwrap();
        let next_out_of_range = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 36 as f64, None, None, None.clone(), None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![2], &vec![0])), Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![2])), None, None, None, None, None).unwrap();
        let mut ok3 = true;
        let mut i3 = 0u32;
        while (i32::from_ne_bytes((i3).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((next_out_of_range.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if next_out_of_range.allocations[usize::try_from(i3).unwrap_or(0)].clone().reason.to_string() == "AttachedInlineVirtualAutoSpace" {
                ok3 = false;
            }
            i3 = u32::wrapping_add(i3, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok3, None).unwrap();
        let p0 = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 52 as f64, None, None, None.clone(), None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![2], &vec![0])), Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![2])), None, None, None, None, None).unwrap();
        let mut ok4 = true;
        let mut i4 = 0u32;
        while (i32::from_ne_bytes((i4).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p0.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p0.allocations[usize::try_from(i4).unwrap_or(0)].clone().reason.to_string() == "AttachedInlineVirtualAutoSpace" {
                ok4 = false;
            }
            i4 = u32::wrapping_add(i4, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok4, Some("expected skip for protected [0]".to_string())).unwrap();
        let p3 = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 52 as f64, None, None, None.clone(), None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![3])), None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![2], &vec![0])), Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![2])), None, None, None, None, None).unwrap();
        let mut ok5 = true;
        let mut i5 = 0u32;
        while (i32::from_ne_bytes((i5).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p3.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p3.allocations[usize::try_from(i5).unwrap_or(0)].clone().reason.to_string() == "AttachedInlineVirtualAutoSpace" {
                ok5 = false;
            }
            i5 = u32::wrapping_add(i5, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok5, Some("expected skip for protected [3]".to_string())).unwrap();
    });
}

#[test]
fn attached_inline_virtual_inter_char_honours_no_stretch_protection() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.attachedInlineVirtualInterCharHonoursNoStretchProtection", "org.tiqian.layout.JustifierCoverageTest.attachedInlineVirtualInterCharHonoursNoStretchProtection", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"attachedInlineVirtualInterCharHonoursNoStretchProtection");
        let c = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"a", 0, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"", 1, Some(0 as f64 as f64), Some("obj".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"b", 2, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 3, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::LatinText, FontRole::Unknown, FontRole::LatinText, FontRole::CjkText];
        let e = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(None, None, None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Other), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
];
        let happy = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 60 as f64, None, None, None.clone(), None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![1], &vec![0])), None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualInterChar", ((happy.allocations[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, happy.unfilled_deficit, None).unwrap();
        let v1 = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 60 as f64, None, None, None.clone(), None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![1], &vec![0])), None, None, None, None, None, None).unwrap();
        let mut ok1 = true;
        let mut i1 = 0u32;
        while (i32::from_ne_bytes((i1).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((v1.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if v1.allocations[usize::try_from(i1).unwrap_or(0)].clone().reason.to_string() == "AttachedInlineVirtualInterChar" {
                ok1 = false;
            }
            i1 = u32::wrapping_add(i1, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok1, Some("expected skip for [0]/[]".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((v1.unfilled_deficit) > (0 as f64), None).unwrap();
        let v2 = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 60 as f64, None, None, None.clone(), None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![2])), None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![1], &vec![0])), None, None, None, None, None, None).unwrap();
        let mut ok2 = true;
        let mut i2 = 0u32;
        while (i32::from_ne_bytes((i2).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((v2.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if v2.allocations[usize::try_from(i2).unwrap_or(0)].clone().reason.to_string() == "AttachedInlineVirtualInterChar" {
                ok2 = false;
            }
            i2 = u32::wrapping_add(i2, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok2, Some("expected skip for [2]/[]".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((v2.unfilled_deficit) > (0 as f64), None).unwrap();
        let v3 = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 60 as f64, None, None, None.clone(), None, None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![1], &vec![0])), None, None, None, None, None, None).unwrap();
        let mut ok3 = true;
        let mut i3 = 0u32;
        while (i32::from_ne_bytes((i3).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((v3.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if v3.allocations[usize::try_from(i3).unwrap_or(0)].clone().reason.to_string() == "AttachedInlineVirtualInterChar" {
                ok3 = false;
            }
            i3 = u32::wrapping_add(i3, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok3, Some("expected skip for []/[0]".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((v3.unfilled_deficit) > (0 as f64), None).unwrap();
        let promoted = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 60 as f64, None, None, None.clone(), None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![1], &vec![0])), None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![1])), None, None, None, None).unwrap();
        let mut found: Option<JustificationAllocation> = None;
        let mut i4 = 0u32;
        while (i32::from_ne_bytes((i4).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((promoted.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if promoted.allocations[usize::try_from(i4).unwrap_or(0)].target_cluster_index == 1 {
                found = Some((promoted.allocations[usize::try_from(i4).unwrap_or(0)]).clone());
            }
            i4 = u32::wrapping_add(i4, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"InlineObjectBoundary", found.as_ref().unwrap().kind.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn attached_inline_virtual_sino_western_needs_stretch_enabled() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.attachedInlineVirtualSinoWesternNeedsStretchEnabled", "org.tiqian.layout.JustifierCoverageTest.attachedInlineVirtualSinoWesternNeedsStretchEnabled", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"attachedInlineVirtualSinoWesternNeedsStretchEnabled");
        let c = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"a", 0, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"", 1, Some(0 as f64 as f64), Some("obj".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"b", 2, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 3, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::LatinText, FontRole::Unknown, FontRole::LatinText, FontRole::CjkText];
        let e = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(None, None, None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Other), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
];
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 60 as f64, None, None, None.clone(), Some(false), None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![1], &vec![0])), Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![1])), None, None, None, None, None).unwrap();
        let mut ok = true;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].clone().reason.to_string() == "AttachedInlineVirtualInterChar" {
                ok = false;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((p.unfilled_deficit) > (0 as f64), None).unwrap();
    });
}

#[test]
fn cjk_line_with_no_opportunities_reports_unfilled_without_fallback() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.cjkLineWithNoOpportunitiesReportsUnfilledWithoutFallback", "org.tiqian.layout.JustifierCoverageTest.cjkLineWithNoOpportunitiesReportsUnfilledWithoutFallback", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"cjkLineWithNoOpportunitiesReportsUnfilledWithoutFallback");
        let c = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText];
        let e = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
];
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 0u32), 20 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, p.unfilled_deficit, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(p.fallback_reason.is_none(), &"-", None).unwrap();
    });
}

#[test]
fn compress_distributes_tier_by_tier() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.compressDistributesTierByTier", "org.tiqian.layout.JustifierCoverageTest.compressDistributesTierByTier", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"compressDistributesTierByTier");
        let j = Justifier::new(Some(0.5), Some(0.25));
        let t1 = ShrinkOpportunity::new(0u32, 1u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false));
        let t2 = ShrinkOpportunity::new(1u32, 2u32, 16 as f64 as f64, ShrinkChannel::LeadingGlue, Some(false));
        let p = j.compress(12 as f64, &vec![(t2).clone(), (t1).clone()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_surplus, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_push_in_allocation_array(&vec![
    (PushInAllocation::new(0u32, 4 as f64 as f64, 4 as f64 as f64, Some(ShrinkChannel::TrailingGlue)).unwrap()).clone(),
    (PushInAllocation::new(1u32, 8 as f64 as f64, 16 as f64 as f64, Some(ShrinkChannel::LeadingGlue)).unwrap()).clone(),
], &p.allocations, None).unwrap();
    });
}

#[test]
fn compress_early_exits_and_filters_degenerate_inputs() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.compressEarlyExitsAndFiltersDegenerateInputs", "org.tiqian.layout.JustifierCoverageTest.compressEarlyExitsAndFiltersDegenerateInputs", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"compressEarlyExitsAndFiltersDegenerateInputs");
        let j = Justifier::new(Some(0.5), Some(0.25));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(CompressionPlan::new(vec![].to_vec(), 0 as f64 as f64, 0 as f64 as f64).to_string().as_str(), j.compress(0 as f64, &vec![]).unwrap().to_string().as_str(), None).unwrap();
        let zero = ShrinkOpportunity::new(0u32, 1u32, 0 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false));
        let unfilled = j.compress(8 as f64, &vec![(zero).clone()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((unfilled.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, unfilled.unfilled_surplus, None).unwrap();
        let big = ShrinkOpportunity::new(0u32, 1u32, 16 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false));
        let other = ShrinkOpportunity::new(1u32, 2u32, 16 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false));
        let capped = j.compress(8 as f64, &vec![(big).clone(), (other).clone()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_push_in_allocation_array(&vec![
    (PushInAllocation::new(0u32, 8 as f64 as f64, 16 as f64 as f64, Some(ShrinkChannel::TrailingGlue)).unwrap()).clone(),
], &capped.allocations, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, capped.unfilled_surplus, None).unwrap();
    });
}

#[test]
fn emergency_tracking_fills_the_residual_for_authorized_boundaries() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.emergencyTrackingFillsTheResidualForAuthorizedBoundaries", "org.tiqian.layout.JustifierCoverageTest.emergencyTrackingFillsTheResidualForAuthorizedBoundaries", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"emergencyTrackingFillsTheResidualForAuthorizedBoundaries");
        let c = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"a", 0, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"b", 1, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
];
        let r = vec![FontRole::LatinText, FontRole::LatinText];
        let e = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, None, None, None, None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_str_map(&vec![0], &vec!["token".to_string()])), None).unwrap();
        let a = (p.allocations[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"EmergencyGraphemeTracking", a.kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"EmergencyGraphemeTracking:token", (a.reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, a.delta, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
        let preferred = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, None, None, None, None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_str_map(&vec![0], &vec!["token".to_string()])), Some(JustifierCoverageTestSupport::justifier_coverage_test_support_str_map(&vec![0], &vec!["code".to_string()]))).unwrap();
        let pa = (preferred.allocations[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"TerminalTechnicalEmergencyTracking:code", (pa.reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"EmergencyGraphemeTracking", pa.kind.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn empty_cluster_range_defers_every_tier_loop() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.emptyClusterRangeDefersEveryTierLoop", "org.tiqian.layout.JustifierCoverageTest.emptyClusterRangeDefersEveryTierLoop", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"emptyClusterRangeDefersEveryTierLoop");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_cjk_latin().unwrap();
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(1u32, 0u32), 16 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, p.unfilled_deficit, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"WesternDominantLineNaturalSpacing", (p.fallback_reason).as_deref().unwrap_or(""), None).unwrap();
    });
}

#[test]
fn paragraph_edge_space_lines_cover_the_boundary_guards() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.paragraphEdgeSpaceLinesCoverTheBoundaryGuards", "org.tiqian.layout.JustifierCoverageTest.paragraphEdgeSpaceLinesCoverTheBoundaryGuards", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"paragraphEdgeSpaceLinesCoverTheBoundaryGuards");
        let leading = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&" ", 0, Some(4 as f64 as f64), None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 1, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"x", 2, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
];
        let lr = vec![FontRole::LatinText, FontRole::CjkText, FontRole::LatinText];
        let le = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let lp = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&leading, &lr, &le, IntRange::new(0u32, 2u32), 40 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut wc1 = 0u32;
        let mut i1 = 0u32;
        while (i32::from_ne_bytes((i1).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((lp.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if lp.allocations[usize::try_from(i1).unwrap_or(0)].kind == GlueKind::WordSpace {
                wc1 = u32::wrapping_add(wc1, 1);
            }
            i1 = u32::wrapping_add(i1, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(0, wc1, None).unwrap();
        let mut lg: Option<JustificationAllocation> = None;
        let mut i2 = 0u32;
        while (i32::from_ne_bytes((i2).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((lp.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if lp.allocations[usize::try_from(i2).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                lg = Some((lp.allocations[usize::try_from(i2).unwrap_or(0)]).clone());
            }
            i2 = u32::wrapping_add(i2, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(1, lg.as_ref().unwrap().target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, lp.unfilled_deficit, None).unwrap();
        let trailing = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"x", 1, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&" ", 2, Some(4 as f64 as f64), None).unwrap()).clone(),
];
        let tp = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&trailing, &lr, &le, IntRange::new(0u32, 2u32), 40 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut wc2 = 0u32;
        let mut i3 = 0u32;
        while (i32::from_ne_bytes((i3).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((tp.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if tp.allocations[usize::try_from(i3).unwrap_or(0)].kind == GlueKind::WordSpace {
                wc2 = u32::wrapping_add(wc2, 1);
            }
            i3 = u32::wrapping_add(i3, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(0, wc2, None).unwrap();
        let mut tg: Option<JustificationAllocation> = None;
        let mut i4 = 0u32;
        while (i32::from_ne_bytes((i4).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((tp.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if tp.allocations[usize::try_from(i4).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                tg = Some((tp.allocations[usize::try_from(i4).unwrap_or(0)]).clone());
            }
            i4 = u32::wrapping_add(i4, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(0, tg.as_ref().unwrap().target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, tp.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn preferred_inline_object_kinds_chain_until_filled() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.preferredInlineObjectKindsChainUntilFilled", "org.tiqian.layout.JustifierCoverageTest.preferredInlineObjectKindsChainUntilFilled", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"preferredInlineObjectKindsChainUntilFilled");
        let c = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"", 1, Some(0 as f64 as f64), Some("obj".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 2, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::Unknown, FontRole::CjkText];
        let e = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(None, None, None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
];
        let mut pb: SortedMapTableBuilder<u32, InlineObjectPreferredStretch> = SortedTable::sorted_table_map_builder::<u32, InlineObjectPreferredStretch>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        pb.put(&(1), &(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 4 as f64 as f64, 6 as f64 as f64).unwrap()));
        pb.put(&(0), &(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 4 as f64 as f64, 6 as f64 as f64).unwrap()));
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 36 as f64, None, None, None.clone(), None, None, None, None, None, None, None, None, None, None, Some(pb.clone().build()), None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
        let mut all = true;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind != GlueKind::InlineObjectPunctuationTrailing && p.allocations[usize::try_from(i).unwrap_or(0)].kind != GlueKind::InlineObjectRelation {
                all = false;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all, None).unwrap();
    });
}

#[test]
fn preferred_inline_object_stretch_runs_by_semantic_kind() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.preferredInlineObjectStretchRunsBySemanticKind", "org.tiqian.layout.JustifierCoverageTest.preferredInlineObjectStretchRunsBySemanticKind", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"preferredInlineObjectStretchRunsBySemanticKind");
        let c = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"", 1, Some(0 as f64 as f64), Some("obj".to_string())).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 2, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::Unknown, FontRole::CjkText];
        let e = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(None, None, None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
];
        let kinds = vec![
    InlineObjectPreferredStretchKind::PunctuationTrailing,
    InlineObjectPreferredStretchKind::Relation,
    InlineObjectPreferredStretchKind::BinaryOperator,
];
        let reasons = vec![
    "InlineObjectPunctuationTrailing".to_string(),
    "InlineObjectRelation".to_string(),
    "InlineObjectBinaryOperator".to_string(),
];
        let glues = vec![
    "InlineObjectPunctuationTrailing".to_string(),
    "InlineObjectRelation".to_string(),
    "InlineObjectBinaryOperator".to_string(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (3) {
            let mut pb: SortedMapTableBuilder<u32, InlineObjectPreferredStretch> = SortedTable::sorted_table_map_builder::<u32, InlineObjectPreferredStretch>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
            pb.put(&(1), &(InlineObjectPreferredStretch::new(kinds[usize::try_from(i).unwrap_or(0)], 4 as f64 as f64, 8 as f64 as f64).unwrap()));
            let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 36 as f64, None, None, None.clone(), None, None, None, None, None, None, None, None, None, None, Some(pb.clone().build()), None, None, None).unwrap();
            let a = (p.allocations[0usize]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered((glues[usize::try_from(i).unwrap_or(0)]).clone().as_str(), a.kind.name().to_string().as_str(), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string((reasons[usize::try_from(i).unwrap_or(0)]).clone().as_str(), (a.reason).to_string().as_str(), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, a.delta, None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals(2, a.priority, None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let mut pb2: SortedMapTableBuilder<u32, InlineObjectPreferredStretch> = SortedTable::sorted_table_map_builder::<u32, InlineObjectPreferredStretch>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        pb2.put(&(1), &(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 4 as f64 as f64, 8 as f64 as f64).unwrap()));
        let at_end = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 1u32), 20 as f64, None, None, None.clone(), None, None, None, None, None, None, None, None, None, None, Some(pb2.clone().build()), None, None,
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((at_end.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, at_end.unfilled_deficit, None).unwrap();
        let mut pb3: SortedMapTableBuilder<u32, InlineObjectPreferredStretch> = SortedTable::sorted_table_map_builder::<u32, InlineObjectPreferredStretch>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        pb3.put(&(1), &(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 4 as f64 as f64, 8 as f64 as f64).unwrap()));
        let closed = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 36 as f64, None, None, None.clone(), None, None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![1])), None,
None, None, None, None, Some(pb3.clone().build()), None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, closed.unfilled_deficit, None).unwrap();
        let mut ok = true;
        let mut i5 = 0u32;
        while (i32::from_ne_bytes((i5).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((closed.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if closed.allocations[usize::try_from(i5).unwrap_or(0)].kind == GlueKind::InlineObjectRelation {
                ok = false;
            }
            i5 = u32::wrapping_add(i5, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn sino_western_stretch_disabled_skips_tier_two_and_its_virtual_tracking() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.sinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTracking", "org.tiqian.layout.JustifierCoverageTest.sinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTracking", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"sinoWesternStretchDisabledSkipsTierTwoAndItsVirtualTracking");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_cjk_latin().unwrap();
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), Some(false), None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn typed_sino_western_space_needs_both_edges_to_pair() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.typedSinoWesternSpaceNeedsBothEdgesToPair", "org.tiqian.layout.JustifierCoverageTest.typedSinoWesternSpaceNeedsBothEdgesToPair", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"typedSinoWesternSpaceNeedsBothEdgesToPair");
        let c = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&" ", 1, Some(4 as f64 as f64), None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 2, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::LatinText, FontRole::CjkText];
        let e = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
];
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 40 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut ok = true;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::WordSpace || p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                ok = false;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn typed_sino_western_space_stretches_from_its_base() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.typedSinoWesternSpaceStretchesFromItsBase", "org.tiqian.layout.JustifierCoverageTest.typedSinoWesternSpaceStretchesFromItsBase", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"typedSinoWesternSpaceStretchesFromItsBase");
        let c = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&" ", 1, Some(2 as f64 as f64), None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"b", 2, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText];
        let e = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 38 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let a = (p.allocations[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkLatinSpace", a.kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, a.target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, a.delta, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
        let at_cap = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&" ", 1, Some(8 as f64 as f64), None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"b", 2, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
];
        let ap = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&at_cap, &r, &e, IntRange::new(0u32, 2u32), 44 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut n1 = 0u32;
        let mut i1 = 0u32;
        while (i32::from_ne_bytes((i1).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((ap.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if ap.allocations[usize::try_from(i1).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                n1 = u32::wrapping_add(n1, 1);
            }
            i1 = u32::wrapping_add(i1, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(0, n1, None).unwrap();
        let mut n2 = 0u32;
        let mut i2 = 0u32;
        while (i32::from_ne_bytes((i2).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((ap.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if ap.allocations[usize::try_from(i2).unwrap_or(0)].kind == GlueKind::CjkInterChar {
                n2 = u32::wrapping_add(n2, 1);
            }
            i2 = u32::wrapping_add(i2, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(2, n2, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, ap.unfilled_deficit, None).unwrap();
        let collapsed = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&" ", 1, Some(0 as f64 as f64), None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"b", 2, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
];
        let cp = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&collapsed, &r, &e, IntRange::new(0u32, 2u32), 36 as f64, None, None, None.clone(), None, Some(0.25f64), Some(0.25f64), None, None, None, None, None, None, None, None, None, None,
None).unwrap();
        let mut ok = true;
        let mut i3 = 0u32;
        while (i32::from_ne_bytes((i3).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((cp.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if cp.allocations[usize::try_from(i3).unwrap_or(0)].target_cluster_index == 1 && (cp.allocations[usize::try_from(i3).unwrap_or(0)].delta) > (0 as f64) {
                ok = false;
            }
            i3 = u32::wrapping_add(i3, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn uniform_object_boundary_opens_the_gate_and_fills() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.uniformObjectBoundaryOpensTheGateAndFills", "org.tiqian.layout.JustifierCoverageTest.uniformObjectBoundaryOpensTheGateAndFills", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"uniformObjectBoundaryOpensTheGateAndFills");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_latin_space_latin(Some(8 as f64 as f64), None, None).unwrap();
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 2u32), 64 as f64, None, None, None.clone(), None, None, None, None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(p.fallback_reason.is_none(), &"-", None).unwrap();
        let mut has = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::InlineObjectBoundary {
                has = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn uniform_text_boundaries_exclude_protected_classes() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.uniformTextBoundariesExcludeProtectedClasses", "org.tiqian.layout.JustifierCoverageTest.uniformTextBoundariesExcludeProtectedClasses", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"uniformTextBoundariesExcludeProtectedClasses");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_cjk_latin().unwrap();
        let plain = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, Some(0.25f64), None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkInterChar", plain.allocations[0usize].kind.name().to_string().as_str(), None).unwrap();
        let bracket = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, Some(0.25f64), None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"WesternBracketCjkInterChar", ((bracket.allocations[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let physical = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, Some(0.25f64), None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, physical.unfilled_deficit, None).unwrap();
        let virtual_owned = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, Some(0.25f64), None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_int_map(&vec![0], &vec![4294967295u32])), None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualInterChar", ((virtual_owned.allocations[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let uniform_object = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, Some(0.25f64), None, None, None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"InlineObjectBoundary", uniform_object.allocations[0usize].kind.name().to_string().as_str(), None).unwrap();
        let bracket_physical = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, Some(0.25f64), None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, bracket_physical.unfilled_deficit, None).unwrap();
        let bracket_object = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, Some(0.25f64), None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"InlineObjectBoundary", bracket_object.allocations[0usize].kind.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn western_dominant_line_stays_ragged() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.westernDominantLineStaysRagged", "org.tiqian.layout.JustifierCoverageTest.westernDominantLineStaysRagged", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"westernDominantLineStaysRagged");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_latin_space_latin(Some(8 as f64 as f64), None, None).unwrap();
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 2u32), 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"WesternDominantLineNaturalSpacing", (p.fallback_reason).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((p.unfilled_deficit) > (0 as f64), None).unwrap();
        let closed_object = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 2u32), 64 as f64, None, None, None.clone(), None, None, None, None,
Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None, None, Some(JustifierCoverageTestSupport::justifier_coverage_test_support_set(&vec![0])), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"WesternDominantLineNaturalSpacing", (closed_object.fallback_reason).as_deref().unwrap_or(""), None).unwrap();
    });
}

#[test]
fn mixed_capacity_sino_western_opps_skip_zero_capacity_in_overflow() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.mixedCapacitySinoWesternOppsSkipZeroCapacityInOverflow", "org.tiqian.layout.JustifierCoverageTest.mixedCapacitySinoWesternOppsSkipZeroCapacityInOverflow", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"mixedCapacitySinoWesternOppsSkipZeroCapacityInOverflow");
        let c = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"中", 0, None, None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&" ", 1, Some(2 as f64 as f64), None).unwrap()).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_c(&"a", 2, Some({ let __guard = JUSTIFIER_COVERAGE_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some("lat".to_string())).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText];
        let e = vec![
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierCoverageTestSupport::justifier_coverage_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 40 as f64, None, None, None.clone(), None, None, Some(0.25f64), None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut tier2: Vec<JustificationAllocation> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                tier2.push((p.allocations[usize::try_from(i).unwrap_or(0)]).clone());
            }
            i = u32::wrapping_add(i, 1);
        }
        let mut idxs: Vec<u32> = vec![];
        let mut i2 = 0u32;
        while (i32::from_ne_bytes((i2).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((tier2.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            idxs.push(tier2[usize::try_from(i2).unwrap_or(0)].target_cluster_index);
            i2 = u32::wrapping_add(i2, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![1], &idxs, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, tier2[0usize].delta, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn zero_capacity_sino_western_tier_defers_everything_downward() {
    testlib::run("org.tiqian.layout.JustifierCoverageTest.zeroCapacitySinoWesternTierDefersEverythingDownward", "org.tiqian.layout.JustifierCoverageTest.zeroCapacitySinoWesternTierDefersEverythingDownward", || {
        TestTraceRecorder::new("JustifierCoverageTest").section(&"zeroCapacitySinoWesternTierDefersEverythingDownward");
        let f = JustifierCoverageTestSupport::justifier_coverage_test_support_cjk_latin().unwrap();
        let p = JustifierCoverageTestSupport::justifier_coverage_test_support_justify(&f.c, &f.r, &f.e, IntRange::new(0u32, 1u32), 36 as f64, None, None, None.clone(), None, None, Some(0.25f64), None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, p.unfilled_deficit, None).unwrap();
        let a = (p.allocations[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkInterChar", a.kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, a.delta, None).unwrap();
    });
}

#[derive(Debug, Clone, PartialEq)]
pub struct JustifierFixture {
    pub c: Vec<Cluster>,
    pub r: Vec<FontRole>,
    pub e: Vec<EastAsianSpacingEdges>,
}
