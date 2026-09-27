#![cfg(test)]

use crate::org::tiqian::clreq::auto_space_mode::AutoSpaceMode;
use crate::org::tiqian::clreq::auto_space_policy::AutoSpacePolicy;
use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::punctuation_geometry_stage::InlineObjectAttachedMark;
use crate::org::tiqian::layout::punctuation_geometry_stage::PunctuationGeometryStage;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::std::u_string_exception::UStringFault;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault) -> Self {
        match value {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct PunctuationGeometryStageCoverageSupport;

impl PunctuationGeometryStageCoverageSupport {
    pub fn punctuation_geometry_stage_coverage_support_start(n: &str) {
        TestTraceRecorder::new("PunctuationGeometryStageCoverageTest").section(n);
    }

    pub fn punctuation_geometry_stage_coverage_support_c(t: &str, i: u32, a: Option<f64>, f: Option<String>, d: Option<String>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, u_string::unit_count(&(t))))?, t, match &(f) { None => "cjk".to_string(), Some(__option) => __option.to_string() }.as_str(), match &(a) { None => 16 as f64, Some(__option1) => *__option1 }, d.clone(), Some(0.0),
Some(0.0), Some(0.0)));
    }

    pub fn punctuation_geometry_stage_coverage_support_obj(i: u32) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, "x", "inline-object", 8.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn punctuation_geometry_stage_coverage_support_g(id: u32, a: f64, x: Option<f64>, b: Option<Rect>) -> Result<Glyph, TextRangeError> {
        return Ok(Glyph::new(id, TextRange::new(0u32, 1u32)?, a, Some(match &(x) { None => 0 as f64, Some(__option8) => *__option8 }), Some(0 as f64), None, (b).clone(), None, None));
    }

    pub fn punctuation_geometry_stage_coverage_support_e(l: EastAsianSpacingValue, t: EastAsianSpacingValue) -> EastAsianSpacingEdges {
        return EastAsianSpacingEdges::new(l, t, l == EastAsianSpacingValue::Wide);
    }

    pub fn punctuation_geometry_stage_coverage_support_atoms(c: Cluster, g: &Vec<Glyph>) -> Result<Vec<PunctuationAtom>, TextRangeError> {
        return Ok(PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((c).clone(), 16 as f64, PunctuationAtomBuilder::new(None, None)?, &g, PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth),
Some(false)))?);
    }

    pub fn punctuation_geometry_stage_coverage_support_none(n: u32) -> Vec<InlineAttachment> {
        let mut r: Vec<InlineAttachment> = vec![];
        for _ in 0..n {
            r.push(InlineAttachment::None);
        }
        return r;
    }

    pub fn punctuation_geometry_stage_coverage_support_set_ints(v: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((v.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            b.put(&(v[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn punctuation_geometry_stage_coverage_support_float_map(keys: &Vec<u32>, values: &Vec<f64>) -> SortedMapTable<u32, f64> {
        let mut b: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((keys.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            b.put(&(keys[usize::try_from(i).unwrap_or(0)]), &(values[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn punctuation_geometry_stage_coverage_support_render_ranges(v: &Vec<IntRange>) -> String {
        let mut b_b = String::new();
        b_b += &("[");
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((v.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                b_b += &(", ");
            }
            let r = (v[usize::try_from(i).unwrap_or(0)]).clone();
            b_b += &("[");
            let mut j = r.start;
            while (i32::from_ne_bytes((j).to_ne_bytes())) <= i32::from_ne_bytes((r.end).to_ne_bytes()) {
                if i32::from_ne_bytes((j).to_ne_bytes()) > (i32::from_ne_bytes((r.start).to_ne_bytes())) {
                    b_b += &(", ");
                }
                b_b += &(crate::runtime::int_text::IntText::int_text(j));
                j = u32::wrapping_add(j, 1);
            }
            b_b += &("]");
            i = u32::wrapping_add(i, 1);
        }
        b_b += &("]");
        return b_b;
    }

    pub fn punctuation_geometry_stage_coverage_support_render_float_map(m: SortedMapTable<u32, f64>) -> Result<String, UStringFault> {
        let mut b_b = String::new();
        b_b += &("{");
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes((m.size()).to_ne_bytes())).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                b_b += &(", ");
            }
            {
                let x = format!("{}{}{}",
            crate::runtime::int_text::IntText::int_text(m.key_at(i32::from_ne_bytes((i).to_ne_bytes()))),
            "=",
            TestTraceRender::test_trace_render_render_float(m.value_at(i32::from_ne_bytes((i).to_ne_bytes())))?
        );
                b_b += &(x.to_string());
            }
            i = u32::wrapping_add(i, 1);
        }
        b_b += &("}");
        return Ok(b_b);
    }
}

#[test]
fn attached_ascii_point_mark_kinsoku_protects_runs() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarkKinsokuProtectsRuns", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarkKinsokuProtectsRuns", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"attachedAsciiPointMarkKinsokuProtectsRuns");
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&",", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&",", 2, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let roles = vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText];
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let c = (c).clone(); let roles = (roles).clone(); Arc::new(move || {
        PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&c, &roles, &vec![(c[1usize]).clone(), (c[2usize]).clone()], KinsokuLevel::Basic, 100 as f64, 100 as f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&c, &roles, &c, KinsokuLevel::None, 100 as f64, 100 as f64).unwrap().unbreakable_ranges.len()) &
0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let fits = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&c, &roles, &c, KinsokuLevel::Basic, 10 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[[0, 1, 2]]", PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&fits.unbreakable_ranges).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1, 2]), (fits.forbidden_line_start_clusters).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((fits.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut all_reasons = true;
        let mut di = 0u32;
        while (i32::from_ne_bytes((di).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((fits.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if fits.decisions[usize::try_from(di).unwrap_or(0)].clone().reason.to_string() != "AttachedAsciiPointMarkKinsoku" {
                all_reasons = false;
            }
            di = u32::wrapping_add(di, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_reasons, None).unwrap();
        let hangs = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&c, &roles, &c, KinsokuLevel::Basic, 10 as f64, 5 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[[0, 1, 2]]", PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&hangs.unbreakable_ranges).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1, 2]), (hangs.impossible_measure_hang_eligible_clusters).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[[0, 1, 2]]", PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&hangs.extendable_hang_ranges).as_str(), None).unwrap();
        let bounded = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&",", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 2, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let bounded_result = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&bounded, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText], &bounded, KinsokuLevel::Basic, 10 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[[0, 1]]", PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&bounded_result.unbreakable_ranges).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((bounded_result.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mid = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 1, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&",", 2, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let mid_result = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&mid, &vec![FontRole::CjkText, FontRole::CjkText, FontRole::LatinText], &mid, KinsokuLevel::Basic, 100 as f64, 5 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[[1, 2]]", PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&mid_result.unbreakable_ranges).as_str(), None).unwrap();
    });
}

#[test]
fn attached_ascii_point_mark_kinsoku_rejects_detached_runs() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarkKinsokuRejectsDetachedRuns", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarkKinsokuRejectsDetachedRuns", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"attachedAsciiPointMarkKinsokuRejectsDetachedRuns");
        let _ = PunctuationGeometryStageCoverageSupport;
        let after_space = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 1, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&",", 2, None, Some("latin".to_string()), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&after_space, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText], &after_space,
KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let gapped = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&",", 2, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&gapped, &vec![FontRole::CjkText, FontRole::LatinText], &gapped, KinsokuLevel::Basic, 100 as f64, 100 as
f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let object_base = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&",", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&object_base, &vec![FontRole::Unknown, FontRole::LatinText], &object_base, KinsokuLevel::Basic, 100 as f64, 100 as
f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let plain = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&plain, &vec![FontRole::CjkText, FontRole::LatinText], &plain, KinsokuLevel::Basic, 100 as f64, 100 as
f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let cjk_mark = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，", 1, None, None, None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&cjk_mark, &vec![FontRole::CjkText, FontRole::CjkPunctuation], &cjk_mark, KinsokuLevel::Basic, 100 as f64, 100 as
f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn attached_ascii_point_marks_need_a_contiguous_non_space_base() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarksNeedAContiguousNonSpaceBase", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarksNeedAContiguousNonSpaceBase", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"attachedAsciiPointMarksNeedAContiguousNonSpaceBase");
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&",", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&c, 1), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&c, 0), None).unwrap();
        let empty_mark = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&empty_mark, 1), None).unwrap();
        let plain_letter = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&plain_letter, 1), None).unwrap();
        let after_space = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&after_space, 1), None).unwrap();
        let gapped = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&",", 2, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&gapped, 1), None).unwrap();
    });
}

#[test]
fn attached_marks_accept_ascii_point_marks_after_objects() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksAcceptAsciiPointMarksAfterObjects", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksAcceptAsciiPointMarksAfterObjects", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"attachedMarksAcceptAsciiPointMarksAfterObjects");
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&",", 1, None, Some("latin".to_string()), None).unwrap()).clone(),
];
        let r = (PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&c, &vec![FontRole::Unknown, FontRole::LatinText], KinsokuLevel::Basic, Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic))))[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, r.mark_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.separator_cluster_indices.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn attached_marks_collapse_separator_space_before_the_mark() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksCollapseSeparatorSpaceBeforeTheMark", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksCollapseSeparatorSpaceBeforeTheMark", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"attachedMarksCollapseSeparatorSpaceBeforeTheMark");
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 1, None, Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，", 2, None, None, None).unwrap()).clone(),
];
        let roles = vec![FontRole::Unknown, FontRole::LatinText, FontRole::CjkPunctuation];
        let rule = ClreqKinsokuRule::new(Some(KinsokuLevel::Basic));
        let r = (PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&c, &roles, KinsokuLevel::Basic, (Box::new((rule).clone())).clone())[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, r.object_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![1], &r.separator_cluster_indices, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, r.mark_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&c, &roles, KinsokuLevel::None, (Box::new((rule).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0,
None).unwrap();
    });
}

#[test]
fn attached_marks_reject_missing_objects_and_gapped_ranges() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksRejectMissingObjectsAndGappedRanges", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksRejectMissingObjectsAndGappedRanges", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"attachedMarksRejectMissingObjectsAndGappedRanges");
        let _ = PunctuationGeometryStageCoverageSupport;
        let rule = ClreqKinsokuRule::new(Some(KinsokuLevel::Basic));
        let no_object = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，", 1, None, None, None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&no_object, &vec![FontRole::CjkText, FontRole::CjkPunctuation], KinsokuLevel::Basic,
(Box::new((rule).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let only_spaces = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 0, None, Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 1, None, Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，", 2, None, None, None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&only_spaces, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkPunctuation], KinsokuLevel::Basic,
(Box::new((rule).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let gapped = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，", 2, None, None, None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&gapped, &vec![FontRole::Unknown, FontRole::CjkPunctuation], KinsokuLevel::Basic,
(Box::new((rule).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let plain = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&plain, &vec![FontRole::Unknown, FontRole::LatinText], KinsokuLevel::Basic, (Box::new((rule).clone())).clone()).len())
& 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn attached_runs_own_one_virtual_gap_at_their_trailing_edge() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedRunsOwnOneVirtualGapAtTheirTrailingEdge", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedRunsOwnOneVirtualGapAtTheirTrailingEdge", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"attachedRunsOwnOneVirtualGapAtTheirTrailingEdge");
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"ref", 1, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 2, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let attachments = vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None];
        let edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let r = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&c, &edges, &attachments, (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"trailing", ((r.decisions[0usize]).clone().side).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InlineAttachment.Previous", ((r.decisions[0usize]).clone().boundary_role).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualAutoSpace:east-asian-spacing-W-N", ((r.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18 as f64, r.clusters[1usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.clusters[2usize].advance, None).unwrap();
    });
}

#[test]
fn empty_display_text_produces_no_atoms() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.emptyDisplayTextProducesNoAtoms", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.emptyDisplayTextProducesNoAtoms", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"emptyDisplayTextProducesNoAtoms");
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&concat!("\n",
""), 0, None, Some("mandatory-break".to_string()), Some("".to_string())).unwrap(), &vec![]).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn glyphless_clusters_use_the_pure_policy_path() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.glyphlessClustersUseThePurePolicyPath", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.glyphlessClustersUseThePurePolicyPath", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"glyphlessClustersUseThePurePolicyPath");
        let a = (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，", 0, None, None, None).unwrap(), &vec![]).unwrap()[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ProfileGlueFallbackWithoutFontGeometry", (a.geometry_source).to_string().as_str(), None).unwrap();
        let ink_bounds_fallback = a.ink_bounds_fallback.clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"-", match &(ink_bounds_fallback) { None => "-".to_string(), Some(__option14) => __option14.to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, (a.trailing_glue).clone().natural, None).unwrap();
    });
}

#[test]
fn inline_box_spans_add_structural_edges_and_skip_degenerate_ranges() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.inlineBoxSpansAddStructuralEdgesAndSkipDegenerateRanges", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.inlineBoxSpansAddStructuralEdgesAndSkipDegenerateRanges", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"inlineBoxSpansAddStructuralEdgesAndSkipDegenerateRanges");
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 0, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"b", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"c", 2, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let passthrough = PunctuationGeometryStage::punctuation_geometry_stage_apply_inline_box_spans(&c, &vec![]);
        let _ = TracedAssertions::traced_assertions_assert_true(passthrough.clusters == c, None).unwrap();
        let from_empty = PunctuationGeometryStage::punctuation_geometry_stage_apply_inline_box_spans(&vec![], &vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(2 as f64), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((from_empty.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let skipped = PunctuationGeometryStage::punctuation_geometry_stage_apply_inline_box_spans(&c, &vec![
    (InlineBoxSpan::new(TextRange::new(2u32, 2u32).unwrap(), Some(4 as f64), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(10u32, 11u32).unwrap(), Some(4 as f64), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((skipped.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((skipped.advance_by_cluster.size()).to_ne_bytes()) == 0, None).unwrap();
        let applied = PunctuationGeometryStage::punctuation_geometry_stage_apply_inline_box_spans(&c, &vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(2 as f64), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(1u32, 2u32).unwrap(), Some(0.0), Some(3 as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(0u32, 2u32).unwrap(), Some(0.0), Some(1.5f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((applied.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_float_map(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_float_map(&vec![0,
1], &vec![2 as f64, 4.5f64])).unwrap().as_str(), PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_float_map((applied.advance_by_cluster).clone()).unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, applied.clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, applied.clusters[0usize].leading_layout_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.5f64, applied.clusters[1usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, applied.clusters[2usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, applied.clusters[2usize].leading_layout_advance, None).unwrap();
        let clamped = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 0, Some(2 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let clamped_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_inline_box_spans(&clamped, &vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(0.0), Some(i32::from_ne_bytes((4294967290u32).to_ne_bytes()) as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, clamped_result.clusters[0usize].advance, None).unwrap();
    });
}

#[test]
fn inline_object_kinsoku_protects_or_hangs_attached_marks() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.inlineObjectKinsokuProtectsOrHangsAttachedMarks", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.inlineObjectKinsokuProtectsOrHangsAttachedMarks", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"inlineObjectKinsokuProtectsOrHangsAttachedMarks");
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，", 1, None, None, None).unwrap()).clone(),
];
        let a = vec![(InlineObjectAttachedMark::new(0u32, vec![].to_vec(), 1u32)).clone()];
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let c = (c).clone(); let a = (a).clone(); Arc::new(move || {
        PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&c, &a, &vec![(c[1usize]).clone()], KinsokuLevel::Basic, 100 as f64, 100 as f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let disabled = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&c, &a, &c, KinsokuLevel::None, 100 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((disabled.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let fits = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&c, &a, &c, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[[0, 1]]", PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&fits.unbreakable_ranges).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1]), (fits.forbidden_line_start_clusters).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InlineObjectAttachedKinsoku", ((fits.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, fits.decisions[0usize].cluster_index, None).unwrap();
        let hangs = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&c, &a, &c, KinsokuLevel::Basic, 10 as f64, 10 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((hangs.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1]), (hangs.impossible_measure_hang_eligible_clusters).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[[0, 1]]", PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&hangs.extendable_hang_ranges).as_str(), None).unwrap();
        let colon = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"：", 1, None, None, None).unwrap()).clone(),
];
        let colon_marks = vec![(InlineObjectAttachedMark::new(0u32, vec![].to_vec(), 1u32)).clone()];
        let blocked = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&colon, &colon_marks, &colon, KinsokuLevel::Basic, 10 as f64, 10 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((blocked.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((blocked.extendable_hang_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((blocked.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let pair = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，。", 1, Some(16 as f64 as f64), Some("cjk".to_string()), Some("，。".to_string())).unwrap()).clone(),
];
        let pair_marks = vec![(InlineObjectAttachedMark::new(0u32, vec![].to_vec(), 1u32)).clone()];
        let no_hang = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&pair, &pair_marks, &pair, KinsokuLevel::Basic, 10 as f64, 10 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((no_hang.extendable_hang_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let first_line_fits = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&c, &a, &c, KinsokuLevel::Basic, 5 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[[0, 1]]", PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&first_line_fits.unbreakable_ranges).as_str(), None).unwrap();
        let with_separator = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 1, None, Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，", 2, None, None, None).unwrap()).clone(),
];
        let separator_marks = vec![(InlineObjectAttachedMark::new(0u32, vec![1].to_vec(), 2u32)).clone()];
        let separated = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&with_separator, &separator_marks, &with_separator, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1, 2]), (separated.forbidden_line_start_clusters).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InlineObjectAttachedKinsokuAcrossCollapsedSeparatorSpace", ((separated.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn multiple_glyphs_for_one_character_union_into_a_single_ink_box() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.multipleGlyphsForOneCharacterUnionIntoASingleInkBox", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.multipleGlyphsForOneCharacterUnionIntoASingleInkBox", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"multipleGlyphsForOneCharacterUnionIntoASingleInkBox");
        let a = (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，", 0, None, None, None).unwrap(), &vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(1, 8 as f64, Some(0 as f64 as f64), Some(Rect::new(0 as f64 as f64, 0 as f64 as f64, 8 as f64 as f64, 16 as f64 as f64))).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(2, 6 as f64, Some(8 as f64 as f64), Some(Rect::new(0 as f64 as f64, 0 as f64 as f64, 6 as f64 as f64, 16 as f64 as f64))).unwrap()).clone(),
]).unwrap()[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, a.ink_bounds.as_ref().unwrap().get_width(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.ink_bounds.as_ref().unwrap().bottom, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.advance, None).unwrap();
        let ink_bounds_fallback = a.ink_bounds_fallback.clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"-", match &(ink_bounds_fallback) { None => "-".to_string(), Some(__option17) => __option17.to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn narrow_inline_boxes_own_their_outer_auto_space() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.narrowInlineBoxesOwnTheirOuterAutoSpace", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.narrowInlineBoxesOwnTheirOuterAutoSpace", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"narrowInlineBoxesOwnTheirOuterAutoSpace");
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let r = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&c, &edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(2),
(*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, Some(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InlineBox.Narrow", ((r.decisions[0usize]).clone().boundary_role).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InlineBoxOuterAutoSpace:leading-W-N", ((r.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let tc = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 0, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 1, None, None, None).unwrap()).clone(),
];
        let te = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let tr = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&tc, &te, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(2),
(*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, Some(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![0]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InlineBox.Narrow", ((tr.decisions[0usize]).clone().boundary_role).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InlineBoxOuterAutoSpace:trailing-N-W", ((tr.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn per_character_ink_subtracts_preceding_glyph_pens() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.perCharacterInkSubtractsPrecedingGlyphPens", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.perCharacterInkSubtractsPrecedingGlyphPens", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"perCharacterInkSubtractsPrecedingGlyphPens");
        let a = PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"。，", 0, None, None, None).unwrap(), &vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(1, 16 as f64, Some(0 as f64 as f64), Some(Rect::new(2 as f64 as f64, 0 as f64 as f64, 14 as f64 as f64, 16 as f64 as f64))).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(2, 16 as f64, Some(16 as f64 as f64), Some(Rect::new(2 as f64 as f64, 0 as f64 as f64, 14 as f64 as f64, 16 as f64 as f64))).unwrap()).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=1)", ((a[0usize]).clone().range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=1, end=2)", ((a[1usize]).clone().range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12 as f64, (a[0usize]).clone().ink_bounds.as_ref().unwrap().get_width(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12 as f64, (a[1usize]).clone().ink_bounds.as_ref().unwrap().get_width(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((a[0usize]).clone().ink_bounds_fallback.is_none() && (a[1usize]).clone().ink_bounds_fallback.is_none(), None).unwrap();
    });
}

#[test]
fn space_replacement_skips_disabled_mode_null_boundaries_and_exact_widths() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.spaceReplacementSkipsDisabledModeNullBoundariesAndExactWidths", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.spaceReplacementSkipsDisabledModeNullBoundariesAndExactWidths", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"spaceReplacementSkipsDisabledModeNullBoundariesAndExactWidths");
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 1, None, Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 2, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let disabled = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&c, &edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(3),
(*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DISABLED).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((disabled.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, disabled.clusters[1usize].advance, None).unwrap();
        let replace = AutoSpacePolicy::new(Some(AutoSpaceMode::Replace), Some(AutoSpaceMode::Replace), Some(0.125), Some(1.0 / 3.0));
        let exact_width = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 1, Some(2 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 2, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let exact = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&exact_width, &edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(3), (replace).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((exact.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let lone = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 0, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let lone_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
];
        let lone_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&lone, &lone_edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(1), (replace).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((lone_result.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let c = (c).clone(); let replace = (replace).clone(); Arc::new(move || {
        PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&c, &vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
], &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(3), (replace).clone(), 16 as f64, None, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let c = (c).clone(); let edges = (edges).clone(); let replace = (replace).clone(); Arc::new(move || {
        PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&c, &edges, &vec![InlineAttachment::None], (replace).clone(), 16 as f64, None, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let empty_c: Vec<Cluster> = vec![];
        let empty_e: Vec<EastAsianSpacingEdges> = vec![];
        let empty_a: Vec<InlineAttachment> = vec![];
        let empty = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&empty_c, &empty_e, &empty_a, (replace).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((empty.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn spacing_boundaries_count_each_wide_narrow_gap_once() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.spacingBoundariesCountEachWideNarrowGapOnce", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.spacingBoundariesCountEachWideNarrowGapOnce", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"spacingBoundariesCountEachWideNarrowGapOnce");
        let _ = PunctuationGeometryStageCoverageSupport;
        let pair_wn = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 1, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let pair_wn_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(1, &pair_wn, &pair_wn_edges), None).unwrap();
        let pair_nw = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 0, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 1, None, None, None).unwrap()).clone(),
];
        let pair_nw_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(1, &pair_nw, &pair_nw_edges), None).unwrap();
        let space_right = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 1, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 2, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let space_right_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(1, &space_right, &space_right_edges), None).unwrap();
        let space_left = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 0, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 1, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 2, None, None, None).unwrap()).clone(),
];
        let space_left_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(2, &space_left, &space_left_edges), None).unwrap();
        let cjk_pair = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 1, None, None, None).unwrap()).clone(),
];
        let cjk_pair_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(1, &cjk_pair, &cjk_pair_edges), None).unwrap();
    });
}

#[test]
fn typed_space_between_wide_and_narrow_is_replaced_by_the_gap() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.typedSpaceBetweenWideAndNarrowIsReplacedByTheGap", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.typedSpaceBetweenWideAndNarrowIsReplacedByTheGap", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"typedSpaceBetweenWideAndNarrowIsReplacedByTheGap");
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 1, None, Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 2, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let r = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&c, &edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(3), AutoSpacePolicy::new(Some(AutoSpaceMode::Replace), Some(AutoSpaceMode::Replace),
Some(0.125), Some(1.0 / 3.0)), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"gap", ((r.decisions[0usize]).clone().side).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Replace", ((r.decisions[0usize]).clone().mode).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"EastAsianSpacing.Wide", ((r.decisions[0usize]).clone().boundary_role).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"TextAutoSpaceReplace:east-asian-spacing-W-space-N", ((r.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, r.decisions[0usize].characters_affected, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, r.decisions[0usize].reduction_per_char, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, r.decisions[0usize].total_reduction, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, r.clusters[1usize].advance, None).unwrap();
    });
}

#[test]
fn union_without_bounds_falls_back_to_the_first_glyph() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.unionWithoutBoundsFallsBackToTheFirstGlyph", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.unionWithoutBoundsFallsBackToTheFirstGlyph", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"unionWithoutBoundsFallsBackToTheFirstGlyph");
        let a = (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"，", 0, None, None, None).unwrap(), &vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(1, 8 as f64, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(2, 6 as f64, Some(8 as f64 as f64), None).unwrap()).clone(),
]).unwrap()[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"shaper-no-ink-bounds", (a.ink_bounds_fallback).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.body_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, (a.trailing_glue).clone().natural, None).unwrap();
    });
}

#[test]
fn unmatched_glyph_counts_record_the_ambiguous_fallback() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.unmatchedGlyphCountsRecordTheAmbiguousFallback", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.unmatchedGlyphCountsRecordTheAmbiguousFallback", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"unmatchedGlyphCountsRecordTheAmbiguousFallback");
        let a = PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"。，", 0, None, None, None).unwrap(), &vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(1, 8 as f64, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(2, 8 as f64, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(3, 8 as f64, None, None).unwrap()).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((a[0usize]).clone().ink_bounds_fallback.as_ref().map_or(false, |v| v == &("glyph-cluster-mapping-ambiguous".to_string())) && (a[1usize]).clone().ink_bounds_fallback.as_ref().map_or(false, |v| v ==
&("glyph-cluster-mapping-ambiguous".to_string())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((a[0usize]).clone().ink_bounds.is_none() && (a[1usize]).clone().ink_bounds.is_none(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a[0usize].advance, None).unwrap();
    });
}

#[test]
fn virtual_gaps_respect_narrow_to_wide_edges_and_their_neighbours() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.virtualGapsRespectNarrowToWideEdgesAndTheirNeighbours", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.virtualGapsRespectNarrowToWideEdgesAndTheirNeighbours", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"virtualGapsRespectNarrowToWideEdgesAndTheirNeighbours");
        let _ = PunctuationGeometryStageCoverageSupport;
        let attachments = vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None];
        let reversed = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 0, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"ref", 1, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 2, None, None, None).unwrap()).clone(),
];
        let reversed_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let reversed_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&reversed, &reversed_edges, &attachments, (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((reversed_result.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualAutoSpace:east-asian-spacing-W-N", ((reversed_result.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let space_after = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"ref", 1, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&" ", 2, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let space_after_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&space_after, &space_after_edges, &attachments,
(*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let break_after = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"ref", 1, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&concat!("\n",
""), 2, Some(16 as f64 as f64), Some("mandatory-break".to_string()), Some("".to_string())).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&break_after, &space_after_edges, &attachments,
(*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let cjk_after = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"ref", 1, Some(16 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 2, None, None, None).unwrap()).clone(),
];
        let cjk_after_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&cjk_after, &cjk_after_edges, &attachments,
(*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn wide_to_narrow_boundaries_insert_leading_and_trailing_gaps() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.wideToNarrowBoundariesInsertLeadingAndTrailingGaps", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.wideToNarrowBoundariesInsertLeadingAndTrailingGaps", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(&"wideToNarrowBoundariesInsertLeadingAndTrailingGaps");
        let _ = PunctuationGeometryStageCoverageSupport;
        let leading = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 1, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
];
        let leading_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let leading_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&leading, &leading_edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(2),
(*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"leading", ((leading_result.decisions[0usize]).clone().side).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"EastAsianSpacing.Wide", ((leading_result.decisions[0usize]).clone().boundary_role).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"TextAutoSpaceInsert:east-asian-spacing-W-N", ((leading_result.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(i32::from_ne_bytes((4294967294u32).to_ne_bytes()) as f64, leading_result.decisions[0usize].total_reduction, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, leading_result.clusters[1usize].advance, None).unwrap();
        let trailing = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"a", 0, Some(8 as f64 as f64), Some("latin".to_string()), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(&"中", 1, None, None, None).unwrap()).clone(),
];
        let trailing_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let trailing_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&trailing, &trailing_edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(2),
(*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"trailing", ((trailing_result.decisions[0usize]).clone().side).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, trailing_result.clusters[0usize].advance, None).unwrap();
    });
}
