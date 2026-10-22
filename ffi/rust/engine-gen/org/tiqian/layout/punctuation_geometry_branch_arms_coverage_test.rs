#![cfg(test)]

use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::punctuation_geometry_ledger::GlueCapacity;
use crate::org::tiqian::layout::punctuation_geometry_ledger::PunctuationGeometryLedger;
use crate::org::tiqian::layout::punctuation_geometry_stage::InlineObjectAttachedMark;
use crate::org::tiqian::layout::punctuation_geometry_stage::PunctuationGeometryStage;
use crate::org::tiqian::layout::punctuation_model::Glue;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationInkInput;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingAdjustment;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressionResult;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestVirtualGapWithEmptyPreviousTextHasNoNarrowCharacterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestUnionIgnoresGlyphsWithoutBoundsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestTypedSpaceWithEmptyTextNeighboursKeepsItsWidthFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpacingPlanIgnoresTargetsOutsideTheBudgetsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpacingBoundariesAtListEdgesAreFalseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestSpaceRunRequiresNonEmptyAllSpaceTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestResolveClustersAppliesGlyphShiftWithUnchangedAdvanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestNonFiniteHaltPlacementIsIgnoredFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsAssertEqualsIntFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsAssertEqualsIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault> for PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsAssertEqualsIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestInlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeadingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsNullableFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableFloatFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TracedAssertionsAssertEqualsNullableFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TracedAssertionsAssertEqualsNullableFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableFloatFault> for PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TracedAssertionsAssertEqualsNullableFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestHaltAdvanceIsRejectedAtZeroAndAtFullWidthFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestGlueCapacitiesMarkCentredFramesAsPairedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TracedAssertionsAssertEqualsIntFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TracedAssertionsAssertEqualsIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault> for PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TracedAssertionsAssertEqualsIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestEmptyTextClustersCannotBeAttachedMarksFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestCentredAdjacencyConsumesBothSidesEquallyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedTrailingGlueWidensABudgetedEndClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedRunAtParagraphEndEmitsNoAutoSpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsIntArrayFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntArrayFault),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsAssertEqualsIntArrayFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsAssertEqualsIntFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntArrayFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsAssertEqualsIntArrayFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsAssertEqualsIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntArrayFault> for PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntArrayFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsAssertEqualsIntArrayFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault> for PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsAssertEqualsIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedMarkWalkStopsMidRunAtAGapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryWithPlainPreviousClusterKeepsTheRightBudgetFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryRecordsNullCharactersForEmptyTextClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedBoundaryReasonFallsBackToNaturalWithoutLeftAtomFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAttachedAsciiPointMarkCheckSkipsEmptyPreviousTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsAssertEqualsIntFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsAssertEqualsIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault) -> Self {
        match value {
            PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault> for PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsAssertEqualsIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryBranchArmsCoverageTestAsciiPointMarkKinsokuSkipsEmptyTextClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn halt_advance_is_rejected_at_zero_and_at_full_width() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.haltAdvanceIsRejectedAtZeroAndAtFullWidth", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.haltAdvanceIsRejectedAtZeroAndAtFullWidth", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[104,97,108,116,65,100,118,97,110,99,101,73,115,82,101,106,101,99,116,101,100,65,116,90,101,114,111,65,110,100,65,116,70,117,108,108,87,105,100,116,104]));
        let b = PunctuationAtomBuilder::new(None, None).unwrap();
        let z = b.build(UStr::new(&[65292]), TextRange::new(0u32, 1u32).unwrap(), 16.0f64, Some(PunctuationInkInput::new(16 as f64 as f64, None, Some(0 as f64), None, None).unwrap()), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_float(None, z.as_ref().unwrap().halt_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,114,111,102,105,108,101,71,108,117,101,70,97,108,108,98,97,99,107,87,105,116,104,111,117,116,70,111,110,116,71,101,111,109,101,116,114,121]), (z.as_ref().unwrap().geometry_source).to_ustring().as_ustr(), None).unwrap();
        let f = b.build(UStr::new(&[65292]), TextRange::new(0u32, 1u32).unwrap(), 16.0f64, Some(PunctuationInkInput::new(16 as f64 as f64, None, Some(16 as f64), None, None).unwrap()), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_float(None, f.as_ref().unwrap().halt_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,114,111,102,105,108,101,71,108,117,101,70,97,108,108,98,97,99,107,87,105,116,104,111,117,116,70,111,110,116,71,101,111,109,101,116,114,121]), (f.as_ref().unwrap().geometry_source).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn non_finite_halt_placement_is_ignored() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.nonFiniteHaltPlacementIsIgnored", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.nonFiniteHaltPlacementIsIgnored", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[110,111,110,70,105,110,105,116,101,72,97,108,116,80,108,97,99,101,109,101,110,116,73,115,73,103,110,111,114,101,100]));
        let b = PunctuationAtomBuilder::new(None, None).unwrap();
        let a = b.build(UStr::new(&[183]), TextRange::new(0u32, 1u32).unwrap(), 16.0f64, Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(8 as f64 as f64, 4 as f64 as f64, 16 as f64 as f64, 12 as f64 as f64)), Some(8 as f64), Some(f64::NAN), None).unwrap()), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[70,111,110,116,72,97,108,116,65,100,118,97,110,99,101,87,105,116,104,73,110,107,66,111,117,110,100,115,70,105,116,116,101,100,80,108,97,99,101,109,101,110,116]), (a.as_ref().unwrap().geometry_source).to_ustring().as_ustr(), None).unwrap();
        let x = b.build(UStr::new(&[65292]), TextRange::new(0u32, 1u32).unwrap(), 16.0f64, Some(PunctuationInkInput::new(16 as f64 as f64, None, Some(8 as f64), Some(f64::NAN), None).unwrap()), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[70,111,110,116,72,97,108,116,65,100,118,97,110,99,101,87,105,116,104,80,114,111,102,105,108,101,70,97,108,108,98,97,99,107]), (x.as_ref().unwrap().geometry_source).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, (x.as_ref().unwrap().trailing_glue).clone().natural, None).unwrap();
    });
}

#[test]
fn union_ignores_glyphs_without_bounds() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.unionIgnoresGlyphsWithoutBounds", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.unionIgnoresGlyphsWithoutBounds", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[117,110,105,111,110,73,103,110,111,114,101,115,71,108,121,112,104,115,87,105,116,104,111,117,116,66,111,117,110,100,115]));
        let c = Cluster::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[65292])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("，")), Some(0.0), Some(0.0), Some(0.0));
        let gs = vec![
    (Glyph::new(1u32, TextRange::new(0u32, 1u32).unwrap(), 8 as f64 as f64, Some(0 as f64), Some(0 as f64), None, Some(Rect::new(0 as f64 as f64, 0 as f64 as f64, 8 as f64 as f64, 16 as f64 as f64)), None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(0u32, 1u32).unwrap(), 6 as f64 as f64, Some(8 as f64), Some(0 as f64), None, None, None, None)).clone(),
];
        let a = (PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((c).clone(), 16 as f64, PunctuationAtomBuilder::new(None, None).unwrap(), &gs, PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))).unwrap()[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.ink_bounds.as_ref().unwrap().get_width(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.advance, None).unwrap();
    });
}

#[test]
fn attached_mark_walk_stops_mid_run_at_a_gap() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedMarkWalkStopsMidRunAtAGap", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedMarkWalkStopsMidRunAtAGap", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[97,116,116,97,99,104,101,100,77,97,114,107,87,97,108,107,83,116,111,112,115,77,105,100,82,117,110,65,116,65,71,97,112]));
        let r = ClreqKinsokuRule::new(Some(KinsokuLevel::Basic));
        let g = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_o(0).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32]), 2, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[65292]), 4, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&g, &vec![FontRole::Unknown, FontRole::LatinText, FontRole::LatinText, FontRole::CjkPunctuation], KinsokuLevel::Basic, (Box::new((r).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let q = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_o(0).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32]), 2, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[65292]), 3, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let m = (PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&q, &vec![FontRole::Unknown, FontRole::LatinText, FontRole::LatinText, FontRole::CjkPunctuation], KinsokuLevel::Basic, (Box::new((r).clone())).clone())[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![1, 2], &m.separator_cluster_indices, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, m.mark_cluster_index, None).unwrap();
    });
}

#[test]
fn empty_text_clusters_cannot_be_attached_marks() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.emptyTextClustersCannotBeAttachedMarks", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.emptyTextClustersCannotBeAttachedMarks", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[101,109,112,116,121,84,101,120,116,67,108,117,115,116,101,114,115,67,97,110,110,111,116,66,101,65,116,116,97,99,104,101,100,77,97,114,107,115]));
        let cs = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_o(0).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let r = ClreqKinsokuRule::new(Some(KinsokuLevel::Basic));
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&cs, &vec![FontRole::Unknown, FontRole::LatinText], KinsokuLevel::Basic, (Box::new((r).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let x = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&cs, &vec![(InlineObjectAttachedMark::new(0u32, vec![].to_vec(), 1u32)).clone()], &cs, KinsokuLevel::Basic, 10 as f64, 10 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((x.extendable_hang_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((x.impossible_measure_hang_eligible_clusters.size()) as u32).to_ne_bytes()) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((x.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn ascii_point_mark_kinsoku_skips_empty_text_clusters() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.asciiPointMarkKinsokuSkipsEmptyTextClusters", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.asciiPointMarkKinsokuSkipsEmptyTextClusters", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[97,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,83,107,105,112,115,69,109,112,116,121,84,101,120,116,67,108,117,115,116,101,114,115]));
        let _ = ClreqKinsokuRule::new(Some(KinsokuLevel::Basic));
        let a = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), Some(UString::from("x"))).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&a, &vec![FontRole::CjkText, FontRole::LatinText], &a, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let b = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), Some(UString::from("x"))).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[44]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&b, &vec![FontRole::LatinText, FontRole::LatinText], &b, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let d = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[44]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 2, Some(16 as f64 as f64), Some(UString::from("latin")), Some(UString::from("x"))).unwrap()).clone(),
];
        let dr = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&d, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText], &d, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,91,48,44,32,49,93,93]), PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_render(&dr.unbreakable_ranges).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((dr.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let e = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[44]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[44]), 3, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let er = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&e, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText], &e, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,91,48,44,32,49,93,93]), PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_render(&er.unbreakable_ranges).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((er.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn space_run_requires_non_empty_all_space_text() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.spaceRunRequiresNonEmptyAllSpaceText", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.spaceRunRequiresNonEmptyAllSpaceText", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[115,112,97,99,101,82,117,110,82,101,113,117,105,114,101,115,78,111,110,69,109,112,116,121,65,108,108,83,112,97,99,101,84,101,120,116]));
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_space_run(PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_space_run(PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32,32]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(PunctuationGeometryStage::punctuation_geometry_stage_is_space_run(PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), Some(UString::from(" "))).unwrap()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(PunctuationGeometryStage::punctuation_geometry_stage_is_space_run(PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[97,32,98]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(PunctuationGeometryStage::punctuation_geometry_stage_is_space_run(PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()), None).unwrap();
    });
}

#[test]
fn attached_run_at_paragraph_end_emits_no_auto_space() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedRunAtParagraphEndEmitsNoAutoSpace", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedRunAtParagraphEndEmitsNoAutoSpace", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[97,116,116,97,99,104,101,100,82,117,110,65,116,80,97,114,97,103,114,97,112,104,69,110,100,69,109,105,116,115,78,111,65,117,116,111,83,112,97,99,101]));
        let x = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[114]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let r = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&x, &vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
], &vec![InlineAttachment::None, InlineAttachment::Previous], (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, r.clusters[1usize].advance, None).unwrap();
    });
}

#[test]
fn virtual_gap_with_empty_previous_text_has_no_narrow_character() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.virtualGapWithEmptyPreviousTextHasNoNarrowCharacter", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.virtualGapWithEmptyPreviousTextHasNoNarrowCharacter", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[118,105,114,116,117,97,108,71,97,112,87,105,116,104,69,109,112,116,121,80,114,101,118,105,111,117,115,84,101,120,116,72,97,115,78,111,78,97,114,114,111,119,67,104,97,114,97,99,116,101,114]));
        let x = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), Some(UString::from("y"))).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[114]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 2, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let a = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let r = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&x, &a, &vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None], (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn typed_space_with_empty_text_neighbours_keeps_its_width() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.typedSpaceWithEmptyTextNeighboursKeepsItsWidth", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.typedSpaceWithEmptyTextNeighboursKeepsItsWidth", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[116,121,112,101,100,83,112,97,99,101,87,105,116,104,69,109,112,116,121,84,101,120,116,78,101,105,103,104,98,111,117,114,115,75,101,101,112,115,73,116,115,87,105,100,116,104]));
        let x = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 2, Some(16 as f64 as f64), Some(UString::from("latin")), Some(UString::from("y"))).unwrap()).clone(),
];
        let r = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&x, &vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
], &vec![InlineAttachment::None, InlineAttachment::None, InlineAttachment::None], (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, r.clusters[1usize].advance, None).unwrap();
        let y = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), Some(UString::from("y"))).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 2, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let z = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&y, &vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
], &vec![InlineAttachment::None, InlineAttachment::None, InlineAttachment::None], (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((z.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let w = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 2, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let wr = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&w, &vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
], &vec![InlineAttachment::None, InlineAttachment::None, InlineAttachment::None], (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((wr.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, wr.clusters[1usize].advance, None).unwrap();
    });
}

#[test]
fn spacing_boundaries_at_list_edges_are_false() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.spacingBoundariesAtListEdgesAreFalse", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.spacingBoundariesAtListEdgesAreFalse", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[115,112,97,99,105,110,103,66,111,117,110,100,97,114,105,101,115,65,116,76,105,115,116,69,100,103,101,115,65,114,101,70,97,108,115,101]));
        let _ = TracedAssertions::traced_assertions_assert_false(PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(1, &vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
], &vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(1, &vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[32]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 1, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
], &vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
]), None).unwrap();
    });
}

#[test]
fn attached_ascii_point_mark_check_skips_empty_previous_text() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedAsciiPointMarkCheckSkipsEmptyPreviousText", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedAsciiPointMarkCheckSkipsEmptyPreviousText", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[97,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,67,104,101,99,107,83,107,105,112,115,69,109,112,116,121,80,114,101,118,105,111,117,115,84,101,120,116]));
        let _ = TracedAssertions::traced_assertions_assert_false(PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), Some(UString::from("x"))).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[44]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
], 1), None).unwrap();
    });
}

#[test]
fn inline_box_span_with_zero_net_structural_edge_still_applies_leading() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.inlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeading", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.inlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeading", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[105,110,108,105,110,101,66,111,120,83,112,97,110,87,105,116,104,90,101,114,111,78,101,116,83,116,114,117,99,116,117,114,97,108,69,100,103,101,83,116,105,108,108,65,112,112,108,105,101,115,76,101,97,100,105,110,103]));
        let r = PunctuationGeometryStage::punctuation_geometry_stage_apply_inline_box_spans(&vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[97]), 0, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
], &vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(2 as f64), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(0 as f64), Some(i32::from_ne_bytes(((4294967294u32) as i32).to_ne_bytes()) as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, r.clusters[0usize].leading_layout_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((r.advance_by_cluster.size()) as u32).to_ne_bytes()) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_clusters_applies_glyph_shift_with_unchanged_advance() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.resolveClustersAppliesGlyphShiftWithUnchangedAdvance", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.resolveClustersAppliesGlyphShiftWithUnchangedAdvance", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[114,101,115,111,108,118,101,67,108,117,115,116,101,114,115,65,112,112,108,105,101,115,71,108,121,112,104,83,104,105,102,116,87,105,116,104,85,110,99,104,97,110,103,101,100,65,100,118,97,110,99,101]));
        let cs = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[12300]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let aa = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((cs[0usize]).clone(), 16 as f64, PunctuationAtomBuilder::new(None, None).unwrap(), &vec![
    (Glyph::new(1u32, TextRange::new(0u32, 1u32).unwrap(), 8 as f64 as f64, Some(0 as f64), Some(0 as f64), None, Some(Rect::new(0 as f64 as f64, 0 as f64 as f64, 8 as f64 as f64, 16 as f64 as f64)), None, None)).clone(),
], PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))).unwrap();
        let r = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_clusters();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, r[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r[0usize].glyph_inline_shift, None).unwrap();
    });
}

#[test]
fn glue_capacities_mark_centred_frames_as_paired() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.glueCapacitiesMarkCentredFramesAsPaired", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.glueCapacitiesMarkCentredFramesAsPaired", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[103,108,117,101,67,97,112,97,99,105,116,105,101,115,77,97,114,107,67,101,110,116,114,101,100,70,114,97,109,101,115,65,115,80,97,105,114,101,100]));
        let cs = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[65292]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let aa = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((cs[0usize]).clone(), 16 as f64, PunctuationAtomBuilder::new(None, None).unwrap(), &vec![], PunctuationGluePlacement::Traditional, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))).unwrap();
        let cap = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).glue_capacities().get(&(0));
        let _ = TracedAssertions::traced_assertions_assert_true(cap.as_ref().unwrap().paired, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, cap.as_ref().unwrap().leading, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, cap.as_ref().unwrap().trailing, None).unwrap();
    });
}

#[test]
fn attached_boundary_with_plain_previous_cluster_keeps_the_right_budget() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedBoundaryWithPlainPreviousClusterKeepsTheRightBudget", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedBoundaryWithPlainPreviousClusterKeepsTheRightBudget", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[97,116,116,97,99,104,101,100,66,111,117,110,100,97,114,121,87,105,116,104,80,108,97,105,110,80,114,101,118,105,111,117,115,67,108,117,115,116,101,114,75,101,101,112,115,84,104,101,82,105,103,104,116,66,117,100,103,101,116]));
        let cs = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[114]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[12300]), 2, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let aa = PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_atoms(&cs).unwrap();
        let r = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None], &aa, 16 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((r.trailing_glue_by_cluster.size()) as u32).to_ne_bytes()) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, (r.geometry).clone().resolve_clusters()[2usize].advance, None).unwrap();
    });
}

#[test]
fn attached_boundary_records_null_characters_for_empty_text_clusters() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedBoundaryRecordsNullCharactersForEmptyTextClusters", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedBoundaryRecordsNullCharactersForEmptyTextClusters", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[97,116,116,97,99,104,101,100,66,111,117,110,100,97,114,121,82,101,99,111,114,100,115,78,117,108,108,67,104,97,114,97,99,116,101,114,115,70,111,114,69,109,112,116,121,84,101,120,116,67,108,117,115,116,101,114,115]));
        let cs = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[12301]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[114]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 4, Some(16 as f64 as f64), Some(UString::from("latin")), Some(UString::from("a"))).unwrap()).clone(),
];
        let aa = PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_atoms(&cs).unwrap();
        let r = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None], &aa, 16 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[0]), ((r.decisions[0usize]).clone().right_char).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,110,97,116,117,114,97,108]), ((r.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.trailing_glue_by_cluster.get(&(1)).unwrap(), None).unwrap();
        let ps = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), Some(UString::from("」"))).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[114]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[12300]), 4, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let pr = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&ps, &PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_atoms(&ps).unwrap(), PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None], &PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_atoms(&ps).unwrap(), 16 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[0]), ((pr.decisions[0usize]).clone().left_char).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,97,100,106,97,99,101,110,116,45,112,117,110,99,116,117,97,116,105,111,110]), ((pr.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn attached_trailing_glue_widens_a_budgeted_end_cluster() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedTrailingGlueWidensABudgetedEndCluster", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedTrailingGlueWidensABudgetedEndCluster", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[97,116,116,97,99,104,101,100,84,114,97,105,108,105,110,103,71,108,117,101,87,105,100,101,110,115,65,66,117,100,103,101,116,101,100,69,110,100,67,108,117,115,116,101,114]));
        let cs = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[12301]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[12301]), 1, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[12300]), 2, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let aa = PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_atoms(&cs).unwrap();
        let mut widened = aa.clone();
        widened[0usize] = PunctuationAtom::new(((widened[0usize]).clone().range).clone(), ((widened[0usize]).clone().char).to_ustring().as_ustr(), widened[0usize].punctuation_class, widened[0usize].advance, ((widened[0usize]).clone().ink_bounds).clone(), widened[0usize].body_width, widened[0usize].halt_advance, (widened[0usize]).clone().halt_validation.clone(), ((widened[0usize]).clone().leading_glue).clone(), Glue::new(((widened[0usize]).clone().trailing_glue).clone().kind, ((widened[0usize]).clone().trailing_glue).clone().min, 12 as f64 as f64, 12 as f64 as f64, ((widened[0usize]).clone().trailing_glue).clone().priority, ((widened[0usize]).clone().trailing_glue).clone().penalty).unwrap(), widened[0usize].anchor, ((widened[0usize]).clone().geometry_source).to_ustring().as_ustr(), widened[0usize].policy_body_floor, widened[0usize].ink_width, widened[0usize].ink_center, widened[0usize].ink_containment_body_floor, widened[0usize].ink_containment_applied, (widened[0usize]).clone().ink_bounds_fallback.clone(), widened[0usize].advance_expansion, widened[0usize].glyph_inline_shift, (widened[0usize]).clone().glyph_placement_reason.clone(), Some(widened[0usize].leading_glue_initially_consumed), Some(widened[0usize].trailing_glue_initially_consumed)).unwrap();
        let r = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &widened, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None], &widened, 16 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, r.trailing_glue_by_cluster.get(&(1)).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20 as f64, (r.geometry).clone().resolve_clusters()[1usize].advance, None).unwrap();
    });
}

#[test]
fn spacing_plan_ignores_targets_outside_the_budgets() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.spacingPlanIgnoresTargetsOutsideTheBudgets", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.spacingPlanIgnoresTargetsOutsideTheBudgets", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[115,112,97,99,105,110,103,80,108,97,110,73,103,110,111,114,101,115,84,97,114,103,101,116,115,79,117,116,115,105,100,101,84,104,101,66,117,100,103,101,116,115]));
        let cs = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[20013]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[12290]), 1, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let aa = PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_atoms(&cs).unwrap();
        let s = vec![
    (PunctuationSpacingAdjustment::new(TextRange::new(0u32, 2u32).unwrap(), TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[20013])), &(UStr::new(&[12290])), 8 as f64 as f64, 0 as f64 as f64, 8 as f64 as f64, &(UStr::new(&[116,101,115,116,45,115,116,114,97,121]))).unwrap()).clone(),
];
        let m: SortedMapTable<u32, GlueCapacity> = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressionResult::new(s.to_vec()).unwrap()).glue_capacities();
        let _ = TracedAssertions::traced_assertions_assert_false(m.has(&(0)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, m.get(&(1)).as_ref().unwrap().trailing, None).unwrap();
    });
}

#[test]
fn centred_adjacency_consumes_both_sides_equally() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.centredAdjacencyConsumesBothSidesEqually", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.centredAdjacencyConsumesBothSidesEqually", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[99,101,110,116,114,101,100,65,100,106,97,99,101,110,99,121,67,111,110,115,117,109,101,115,66,111,116,104,83,105,100,101,115,69,113,117,97,108,108,121]));
        let cs = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[65292]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[65292]), 1, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let mut aa: Vec<PunctuationAtom> = vec![];
        for x in &cs {
            {
                let mut _g = 0u32;
                let _g1 = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((x).clone(), 16 as f64, PunctuationAtomBuilder::new(None, None).unwrap(), &vec![], PunctuationGluePlacement::Traditional, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))).unwrap();
                while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let a = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                    _g = u32::wrapping_add(_g, 1);
                    aa.push(a.clone());
                }
            }
        }
        let m: SortedMapTable<u32, GlueCapacity> = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressor::new().unwrap().compress(&aa, 16 as f64).unwrap()).glue_capacities();
        let _ = TracedAssertions::traced_assertions_assert_true(!m.has(&(0)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, m.get(&(1)).as_ref().unwrap().leading, None).unwrap();
        let r = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressor::new().unwrap().compress(&aa, 16 as f64).unwrap()).resolve_clusters();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, r[1usize].advance, None).unwrap();
    });
}

#[test]
fn attached_boundary_reason_falls_back_to_natural_without_left_atom() {
    testlib::run("org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedBoundaryReasonFallsBackToNaturalWithoutLeftAtom", "org.tiqian.layout.PunctuationGeometryBranchArmsCoverageTest.attachedBoundaryReasonFallsBackToNaturalWithoutLeftAtom", || {
        PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_t(UStr::new(&[97,116,116,97,99,104,101,100,66,111,117,110,100,97,114,121,82,101,97,115,111,110,70,97,108,108,115,66,97,99,107,84,111,78,97,116,117,114,97,108,87,105,116,104,111,117,116,76,101,102,116,65,116,111,109]));
        let cs = vec![
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[12301]), 0, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[114]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[12300]), 2, Some(16 as f64 as f64), Some(UString::from("cjk")), None).unwrap()).clone(),
];
        let aa = PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_atoms(&cs).unwrap();
        let r = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None], &vec![], 16 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,110,97,116,117,114,97,108]), ((r.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[12301]), ((r.decisions[0usize]).clone().left_char).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.trailing_glue_by_cluster.get(&(1)).unwrap(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct PunctuationGeometryBranchArmsCoverageTestSupport;

impl PunctuationGeometryBranchArmsCoverageTestSupport {
    pub fn punctuation_geometry_branch_arms_coverage_test_support_t(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,66,114,97,110,99,104,65,114,109,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(n);
    }

    pub fn punctuation_geometry_branch_arms_coverage_test_support_c(s: &UStr, i: u32, advance: Option<f64>, font: Option<UString>, display: Option<UString>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, u_string::unit_count(&(s))))?, s, (font).as_deref().unwrap_or(UStr::new(&[])), (advance).unwrap(), Some((match &(display) { None => s.to_ustring(), Some(__option) => __option.to_ustring() }).to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn punctuation_geometry_branch_arms_coverage_test_support_o(i: u32) -> Result<Cluster, TextRangeError> {
        return Ok(PunctuationGeometryBranchArmsCoverageTestSupport::punctuation_geometry_branch_arms_coverage_test_support_c(UStr::new(&[120]), i, Some(8 as f64 as f64), Some(UString::from("inline-object")), Some(UString::from("")))?);
    }

    pub fn punctuation_geometry_branch_arms_coverage_test_support_e(a: EastAsianSpacingValue, b: EastAsianSpacingValue) -> EastAsianSpacingEdges {
        return EastAsianSpacingEdges::new(a, b, a == EastAsianSpacingValue::Wide);
    }

    pub fn punctuation_geometry_branch_arms_coverage_test_support_atoms(cs: &Vec<Cluster>) -> Result<Vec<PunctuationAtom>, TextRangeError> {
        let mut r: Vec<PunctuationAtom> = vec![];
        for x in cs {
            {
                let mut _g = 0u32;
                let _g1 = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((x).clone(), 16 as f64, PunctuationAtomBuilder::new(None, None)?, &vec![], PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false)))?;
                while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let a = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                    _g = u32::wrapping_add(_g, 1);
                    r.push(a.clone());
                }
            }
        }
        return Ok(r);
    }

    pub fn punctuation_geometry_branch_arms_coverage_test_support_render(rs: &Vec<IntRange>) -> UString {
        let mut s = UString::from("[").to_ustring();
        for i in 0..match u32::try_from(rs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                s += &(UString::from(", "));
            }
            s += &({ let mut __s = UString::new(); __s += &(UString::from("[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(rs[usize::try_from(i).unwrap_or(0)].start)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(rs[usize::try_from(i).unwrap_or(0)].end)).as_str())); __s += &(UString::from("]")); __s });
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += s.as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }
}
