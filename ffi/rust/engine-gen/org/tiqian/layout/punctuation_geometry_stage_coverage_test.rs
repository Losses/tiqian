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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestWideToNarrowBoundariesInsertLeadingAndTrailingGapsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestVirtualGapsRespectNarrowToWideEdgesAndTheirNeighboursFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestUnmatchedGlyphCountsRecordTheAmbiguousFallbackFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestUnionWithoutBoundsFallsBackToTheFirstGlyphFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestTypedSpaceBetweenWideAndNarrowIsReplacedByTheGapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestSpacingBoundariesCountEachWideNarrowGapOnceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestSpaceReplacementSkipsDisabledModeNullBoundariesAndExactWidthsFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestPerCharacterInkSubtractsPrecedingGlyphPensFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestNarrowInlineBoxesOwnTheirOuterAutoSpaceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestMultipleGlyphsForOneCharacterUnionIntoASingleInkBoxFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestInlineObjectKinsokuProtectsOrHangsAttachedMarksFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestInlineBoxSpansAddStructuralEdgesAndSkipDegenerateRangesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestGlyphlessClustersUseThePurePolicyPathFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestEmptyDisplayTextProducesNoAtomsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedRunsOwnOneVirtualGapAtTheirTrailingEdgeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedMarksRejectMissingObjectsAndGappedRangesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedMarksCollapseSeparatorSpaceBeforeTheMarkFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedMarksAcceptAsciiPointMarksAfterObjectsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarksNeedAContiguousNonSpaceBaseFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuRejectsDetachedRunsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
            PunctuationGeometryStageCoverageTestAttachedAsciiPointMarkKinsokuProtectsRunsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
    pub fn punctuation_geometry_stage_coverage_support_start(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116]))).section(n);
    }

    pub fn punctuation_geometry_stage_coverage_support_c(t: &UStr, i: u32, a: Option<f64>, f: Option<UString>, d: Option<UString>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, u_string::unit_count(&(t))))?, t, match &(f) { None => UString::from("cjk"), Some(__option) => __option.to_ustring() }.as_ustr(), match &(a) { None => 16 as f64, Some(__option1) => *__option1 }, d.clone(), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn punctuation_geometry_stage_coverage_support_obj(i: u32) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, &(UStr::new(&[120])), &(UStr::new(&[105,110,108,105,110,101,45,111,98,106,101,99,116])), 8.0f64, Some(UString::from("")), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn punctuation_geometry_stage_coverage_support_g(id: u32, a: f64, x: Option<f64>, b: Option<Rect>) -> Result<Glyph, TextRangeError> {
        return Ok(Glyph::new(id, TextRange::new(0u32, 1u32)?, a, Some(match &(x) { None => 0 as f64, Some(__option8) => *__option8 }), Some(0 as f64), None, (b).clone(), None, None));
    }

    pub fn punctuation_geometry_stage_coverage_support_e(l: EastAsianSpacingValue, t: EastAsianSpacingValue) -> EastAsianSpacingEdges {
        return EastAsianSpacingEdges::new(l, t, l == EastAsianSpacingValue::Wide);
    }

    pub fn punctuation_geometry_stage_coverage_support_atoms(c: Cluster, g: &Vec<Glyph>) -> Result<Vec<PunctuationAtom>, TextRangeError> {
        return Ok(PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((c).clone(), 16 as f64, PunctuationAtomBuilder::new(None, None)?, &g, PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false)))?);
    }

    pub fn punctuation_geometry_stage_coverage_support_none(n: u32) -> Vec<InlineAttachment> {
        let mut r: Vec<InlineAttachment> = vec![];
        for _ in 0..n {
            r.push(InlineAttachment::None);
        }
        return r;
    }

    pub fn punctuation_geometry_stage_coverage_support_set_ints(v: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((v.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            b.put(&(v[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn punctuation_geometry_stage_coverage_support_float_map(keys: &Vec<u32>, values: &Vec<f64>) -> SortedMapTable<u32, f64> {
        let mut b: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((keys.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            b.put(&(keys[usize::try_from(i).unwrap_or(0)]), &(values[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn punctuation_geometry_stage_coverage_support_render_ranges(v: &Vec<IntRange>) -> UString {
        let mut b_b = UString::new();
        b_b += &(UString::from("["));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((v.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (0) {
                b_b += &(UString::from(", "));
            }
            let r = (v[usize::try_from(i).unwrap_or(0)]).clone();
            b_b += &(UString::from("["));
            let mut j = r.start;
            while (i32::from_ne_bytes(((j) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((r.end) as i32).to_ne_bytes()) {
                if i32::from_ne_bytes(((j) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((r.start) as i32).to_ne_bytes())) {
                    b_b += &(UString::from(", "));
                }
                b_b += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(j)).as_str()));
                j = u32::wrapping_add(j, 1);
            }
            b_b += &(UString::from("]"));
            i = u32::wrapping_add(i, 1);
        }
        b_b += &(UString::from("]"));
        return b_b;
    }

    pub fn punctuation_geometry_stage_coverage_support_render_float_map(m: SortedMapTable<u32, f64>) -> Result<UString, UStringFault> {
        let mut b_b = UString::new();
        b_b += &(UString::from("{"));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((m.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (0) {
                b_b += &(UString::from(", "));
            }
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(m.key_at(i32::from_ne_bytes(((i) as i32).to_ne_bytes())))).as_str())); __s += &(UString::from("=")); __s += TestTraceRender::test_trace_render_render_float(m.value_at(i32::from_ne_bytes(((i) as i32).to_ne_bytes())))?.as_ustr(); __s };
                b_b += &(x.to_string());
            }
            i = u32::wrapping_add(i, 1);
        }
        b_b += &(UString::from("}"));
        return Ok(b_b);
    }
}

#[test]
fn attached_ascii_point_mark_kinsoku_protects_runs() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarkKinsokuProtectsRuns", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarkKinsokuProtectsRuns", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[97,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,80,114,111,116,101,99,116,115,82,117,110,115]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[44]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[44]), 2, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let roles = vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText];
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let c = (c).clone(); let roles = (roles).clone(); Arc::new(move || {
        PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&c, &roles, &vec![(c[1usize]).clone(), (c[2usize]).clone()], KinsokuLevel::Basic, 100 as f64, 100 as f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&c, &roles, &c, KinsokuLevel::None, 100 as f64, 100 as f64).unwrap().unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let fits = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&c, &roles, &c, KinsokuLevel::Basic, 10 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,91,48,44,32,49,44,32,50,93,93]), PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&fits.unbreakable_ranges).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1, 2]), (fits.forbidden_line_start_clusters).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((fits.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut all_reasons = true;
        let mut di = 0u32;
        while (i32::from_ne_bytes(((di) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((fits.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if fits.decisions[usize::try_from(di).unwrap_or(0)].clone().reason.to_ustring() != UString::from("AttachedAsciiPointMarkKinsoku") {
                all_reasons = false;
            }
            di = u32::wrapping_add(di, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_reasons, None).unwrap();
        let hangs = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&c, &roles, &c, KinsokuLevel::Basic, 10 as f64, 5 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,91,48,44,32,49,44,32,50,93,93]), PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&hangs.unbreakable_ranges).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1, 2]), (hangs.impossible_measure_hang_eligible_clusters).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,91,48,44,32,49,44,32,50,93,93]), PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&hangs.extendable_hang_ranges).as_ustr(), None).unwrap();
        let bounded = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[44]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 2, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let bounded_result = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&bounded, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText], &bounded, KinsokuLevel::Basic, 10 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,91,48,44,32,49,93,93]), PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&bounded_result.unbreakable_ranges).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((bounded_result.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mid = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 1, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[44]), 2, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let mid_result = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&mid, &vec![FontRole::CjkText, FontRole::CjkText, FontRole::LatinText], &mid, KinsokuLevel::Basic, 100 as f64, 5 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,91,49,44,32,50,93,93]), PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&mid_result.unbreakable_ranges).as_ustr(), None).unwrap();
    });
}

#[test]
fn attached_ascii_point_mark_kinsoku_rejects_detached_runs() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarkKinsokuRejectsDetachedRuns", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarkKinsokuRejectsDetachedRuns", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[97,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,82,101,106,101,99,116,115,68,101,116,97,99,104,101,100,82,117,110,115]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let after_space = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 1, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[44]), 2, None, Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&after_space, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText], &after_space, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let gapped = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[44]), 2, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&gapped, &vec![FontRole::CjkText, FontRole::LatinText], &gapped, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let object_base = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[44]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&object_base, &vec![FontRole::Unknown, FontRole::LatinText], &object_base, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let plain = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&plain, &vec![FontRole::CjkText, FontRole::LatinText], &plain, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let cjk_mark = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292]), 1, None, None, None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&cjk_mark, &vec![FontRole::CjkText, FontRole::CjkPunctuation], &cjk_mark, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn attached_ascii_point_marks_need_a_contiguous_non_space_base() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarksNeedAContiguousNonSpaceBase", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedAsciiPointMarksNeedAContiguousNonSpaceBase", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[97,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,115,78,101,101,100,65,67,111,110,116,105,103,117,111,117,115,78,111,110,83,112,97,99,101,66,97,115,101]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[44]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&c, 1), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&c, 0), None).unwrap();
        let empty_mark = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&empty_mark, 1), None).unwrap();
        let plain_letter = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&plain_letter, 1), None).unwrap();
        let after_space = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&after_space, 1), None).unwrap();
        let gapped = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[44]), 2, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(!PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&gapped, 1), None).unwrap();
    });
}

#[test]
fn attached_marks_accept_ascii_point_marks_after_objects() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksAcceptAsciiPointMarksAfterObjects", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksAcceptAsciiPointMarksAfterObjects", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[97,116,116,97,99,104,101,100,77,97,114,107,115,65,99,99,101,112,116,65,115,99,105,105,80,111,105,110,116,77,97,114,107,115,65,102,116,101,114,79,98,106,101,99,116,115]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[44]), 1, None, Some(UString::from("latin")), None).unwrap()).clone(),
];
        let r = (PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&c, &vec![FontRole::Unknown, FontRole::LatinText], KinsokuLevel::Basic, Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic))))[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, r.mark_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.separator_cluster_indices.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn attached_marks_collapse_separator_space_before_the_mark() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksCollapseSeparatorSpaceBeforeTheMark", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksCollapseSeparatorSpaceBeforeTheMark", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[97,116,116,97,99,104,101,100,77,97,114,107,115,67,111,108,108,97,112,115,101,83,101,112,97,114,97,116,111,114,83,112,97,99,101,66,101,102,111,114,101,84,104,101,77,97,114,107]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 1, None, Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292]), 2, None, None, None).unwrap()).clone(),
];
        let roles = vec![FontRole::Unknown, FontRole::LatinText, FontRole::CjkPunctuation];
        let rule = ClreqKinsokuRule::new(Some(KinsokuLevel::Basic));
        let r = (PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&c, &roles, KinsokuLevel::Basic, (Box::new((rule).clone())).clone())[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, r.object_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![1], &r.separator_cluster_indices, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, r.mark_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&c, &roles, KinsokuLevel::None, (Box::new((rule).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn attached_marks_reject_missing_objects_and_gapped_ranges() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksRejectMissingObjectsAndGappedRanges", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedMarksRejectMissingObjectsAndGappedRanges", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[97,116,116,97,99,104,101,100,77,97,114,107,115,82,101,106,101,99,116,77,105,115,115,105,110,103,79,98,106,101,99,116,115,65,110,100,71,97,112,112,101,100,82,97,110,103,101,115]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let rule = ClreqKinsokuRule::new(Some(KinsokuLevel::Basic));
        let no_object = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292]), 1, None, None, None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&no_object, &vec![FontRole::CjkText, FontRole::CjkPunctuation], KinsokuLevel::Basic, (Box::new((rule).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let only_spaces = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 0, None, Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 1, None, Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292]), 2, None, None, None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&only_spaces, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkPunctuation], KinsokuLevel::Basic, (Box::new((rule).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let gapped = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292]), 2, None, None, None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&gapped, &vec![FontRole::Unknown, FontRole::CjkPunctuation], KinsokuLevel::Basic, (Box::new((rule).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let plain = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&plain, &vec![FontRole::Unknown, FontRole::LatinText], KinsokuLevel::Basic, (Box::new((rule).clone())).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn attached_runs_own_one_virtual_gap_at_their_trailing_edge() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedRunsOwnOneVirtualGapAtTheirTrailingEdge", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.attachedRunsOwnOneVirtualGapAtTheirTrailingEdge", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[97,116,116,97,99,104,101,100,82,117,110,115,79,119,110,79,110,101,86,105,114,116,117,97,108,71,97,112,65,116,84,104,101,105,114,84,114,97,105,108,105,110,103,69,100,103,101]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[114,101,102]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 2, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let attachments = vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None];
        let edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let r = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&c, &edges, &attachments, (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[116,114,97,105,108,105,110,103]), ((r.decisions[0usize]).clone().side).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,108,105,110,101,65,116,116,97,99,104,109,101,110,116,46,80,114,101,118,105,111,117,115]), ((r.decisions[0usize]).clone().boundary_role).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,65,117,116,111,83,112,97,99,101,58,101,97,115,116,45,97,115,105,97,110,45,115,112,97,99,105,110,103,45,87,45,78]), ((r.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18 as f64, r.clusters[1usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.clusters[2usize].advance, None).unwrap();
    });
}

#[test]
fn empty_display_text_produces_no_atoms() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.emptyDisplayTextProducesNoAtoms", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.emptyDisplayTextProducesNoAtoms", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[101,109,112,116,121,68,105,115,112,108,97,121,84,101,120,116,80,114,111,100,117,99,101,115,78,111,65,116,111,109,115]));
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[10]), 0, None, Some(UString::from("mandatory-break")), Some(UString::from(""))).unwrap(), &vec![]).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn glyphless_clusters_use_the_pure_policy_path() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.glyphlessClustersUseThePurePolicyPath", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.glyphlessClustersUseThePurePolicyPath", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[103,108,121,112,104,108,101,115,115,67,108,117,115,116,101,114,115,85,115,101,84,104,101,80,117,114,101,80,111,108,105,99,121,80,97,116,104]));
        let a = (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292]), 0, None, None, None).unwrap(), &vec![]).unwrap()[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,114,111,102,105,108,101,71,108,117,101,70,97,108,108,98,97,99,107,87,105,116,104,111,117,116,70,111,110,116,71,101,111,109,101,116,114,121]), (a.geometry_source).to_ustring().as_ustr(), None).unwrap();
        let ink_bounds_fallback = a.ink_bounds_fallback.clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[45]), match &(ink_bounds_fallback) { None => UString::from("-"), Some(__option14) => __option14.to_ustring() }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, (a.trailing_glue).clone().natural, None).unwrap();
    });
}

#[test]
fn inline_box_spans_add_structural_edges_and_skip_degenerate_ranges() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.inlineBoxSpansAddStructuralEdgesAndSkipDegenerateRanges", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.inlineBoxSpansAddStructuralEdgesAndSkipDegenerateRanges", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[105,110,108,105,110,101,66,111,120,83,112,97,110,115,65,100,100,83,116,114,117,99,116,117,114,97,108,69,100,103,101,115,65,110,100,83,107,105,112,68,101,103,101,110,101,114,97,116,101,82,97,110,103,101,115]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 0, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[98]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[99]), 2, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
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
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((skipped.advance_by_cluster.size()) as u32).to_ne_bytes()) == 0, None).unwrap();
        let applied = PunctuationGeometryStage::punctuation_geometry_stage_apply_inline_box_spans(&c, &vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(2 as f64), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(1u32, 2u32).unwrap(), Some(0.0), Some(3 as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(0u32, 2u32).unwrap(), Some(0.0), Some(1.5f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((applied.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_float_map(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_float_map(&vec![0, 1], &vec![2 as f64, 4.5f64])).unwrap().as_ustr(), PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_float_map((applied.advance_by_cluster).clone()).unwrap().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, applied.clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, applied.clusters[0usize].leading_layout_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.5f64, applied.clusters[1usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, applied.clusters[2usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, applied.clusters[2usize].leading_layout_advance, None).unwrap();
        let clamped = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 0, Some(2 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let clamped_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_inline_box_spans(&clamped, &vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(0.0), Some(i32::from_ne_bytes(((4294967290u32) as i32).to_ne_bytes()) as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, clamped_result.clusters[0usize].advance, None).unwrap();
    });
}

#[test]
fn inline_object_kinsoku_protects_or_hangs_attached_marks() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.inlineObjectKinsokuProtectsOrHangsAttachedMarks", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.inlineObjectKinsokuProtectsOrHangsAttachedMarks", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,75,105,110,115,111,107,117,80,114,111,116,101,99,116,115,79,114,72,97,110,103,115,65,116,116,97,99,104,101,100,77,97,114,107,115]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292]), 1, None, None, None).unwrap()).clone(),
];
        let a = vec![(InlineObjectAttachedMark::new(0u32, vec![].to_vec(), 1u32)).clone()];
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let c = (c).clone(); let a = (a).clone(); Arc::new(move || {
        PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&c, &a, &vec![(c[1usize]).clone()], KinsokuLevel::Basic, 100 as f64, 100 as f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let disabled = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&c, &a, &c, KinsokuLevel::None, 100 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((disabled.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let fits = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&c, &a, &c, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,91,48,44,32,49,93,93]), PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&fits.unbreakable_ranges).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1]), (fits.forbidden_line_start_clusters).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,108,105,110,101,79,98,106,101,99,116,65,116,116,97,99,104,101,100,75,105,110,115,111,107,117]), ((fits.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, fits.decisions[0usize].cluster_index, None).unwrap();
        let hangs = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&c, &a, &c, KinsokuLevel::Basic, 10 as f64, 10 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((hangs.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1]), (hangs.impossible_measure_hang_eligible_clusters).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,91,48,44,32,49,93,93]), PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&hangs.extendable_hang_ranges).as_ustr(), None).unwrap();
        let colon = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65306]), 1, None, None, None).unwrap()).clone(),
];
        let colon_marks = vec![(InlineObjectAttachedMark::new(0u32, vec![].to_vec(), 1u32)).clone()];
        let blocked = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&colon, &colon_marks, &colon, KinsokuLevel::Basic, 10 as f64, 10 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((blocked.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((blocked.extendable_hang_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((blocked.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let pair = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292,12290]), 1, Some(16 as f64 as f64), Some(UString::from("cjk")), Some(UString::from("，。"))).unwrap()).clone(),
];
        let pair_marks = vec![(InlineObjectAttachedMark::new(0u32, vec![].to_vec(), 1u32)).clone()];
        let no_hang = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&pair, &pair_marks, &pair, KinsokuLevel::Basic, 10 as f64, 10 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((no_hang.extendable_hang_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let first_line_fits = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&c, &a, &c, KinsokuLevel::Basic, 5 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,91,48,44,32,49,93,93]), PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_render_ranges(&first_line_fits.unbreakable_ranges).as_ustr(), None).unwrap();
        let with_separator = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_obj(0).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 1, None, Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292]), 2, None, None, None).unwrap()).clone(),
];
        let separator_marks = vec![(InlineObjectAttachedMark::new(0u32, vec![1].to_vec(), 2u32)).clone()];
        let separated = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&with_separator, &separator_marks, &with_separator, KinsokuLevel::Basic, 100 as f64, 100 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1, 2]), (separated.forbidden_line_start_clusters).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,108,105,110,101,79,98,106,101,99,116,65,116,116,97,99,104,101,100,75,105,110,115,111,107,117,65,99,114,111,115,115,67,111,108,108,97,112,115,101,100,83,101,112,97,114,97,116,111,114,83,112,97,99,101]), ((separated.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn multiple_glyphs_for_one_character_union_into_a_single_ink_box() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.multipleGlyphsForOneCharacterUnionIntoASingleInkBox", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.multipleGlyphsForOneCharacterUnionIntoASingleInkBox", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[109,117,108,116,105,112,108,101,71,108,121,112,104,115,70,111,114,79,110,101,67,104,97,114,97,99,116,101,114,85,110,105,111,110,73,110,116,111,65,83,105,110,103,108,101,73,110,107,66,111,120]));
        let a = (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292]), 0, None, None, None).unwrap(), &vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(1, 8 as f64, Some(0 as f64 as f64), Some(Rect::new(0 as f64 as f64, 0 as f64 as f64, 8 as f64 as f64, 16 as f64 as f64))).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(2, 6 as f64, Some(8 as f64 as f64), Some(Rect::new(0 as f64 as f64, 0 as f64 as f64, 6 as f64 as f64, 16 as f64 as f64))).unwrap()).clone(),
]).unwrap()[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, a.ink_bounds.as_ref().unwrap().get_width(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.ink_bounds.as_ref().unwrap().bottom, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.advance, None).unwrap();
        let ink_bounds_fallback = a.ink_bounds_fallback.clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[45]), match &(ink_bounds_fallback) { None => UString::from("-"), Some(__option17) => __option17.to_ustring() }.as_ustr(), None).unwrap();
    });
}

#[test]
fn narrow_inline_boxes_own_their_outer_auto_space() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.narrowInlineBoxesOwnTheirOuterAutoSpace", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.narrowInlineBoxesOwnTheirOuterAutoSpace", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[110,97,114,114,111,119,73,110,108,105,110,101,66,111,120,101,115,79,119,110,84,104,101,105,114,79,117,116,101,114,65,117,116,111,83,112,97,99,101]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let r = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&c, &edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(2), (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, Some(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![1])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,108,105,110,101,66,111,120,46,78,97,114,114,111,119]), ((r.decisions[0usize]).clone().boundary_role).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,108,105,110,101,66,111,120,79,117,116,101,114,65,117,116,111,83,112,97,99,101,58,108,101,97,100,105,110,103,45,87,45,78]), ((r.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let tc = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 0, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 1, None, None, None).unwrap()).clone(),
];
        let te = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let tr = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&tc, &te, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(2), (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, Some(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_set_ints(&vec![0]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,108,105,110,101,66,111,120,46,78,97,114,114,111,119]), ((tr.decisions[0usize]).clone().boundary_role).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,108,105,110,101,66,111,120,79,117,116,101,114,65,117,116,111,83,112,97,99,101,58,116,114,97,105,108,105,110,103,45,78,45,87]), ((tr.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn per_character_ink_subtracts_preceding_glyph_pens() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.perCharacterInkSubtractsPrecedingGlyphPens", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.perCharacterInkSubtractsPrecedingGlyphPens", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[112,101,114,67,104,97,114,97,99,116,101,114,73,110,107,83,117,98,116,114,97,99,116,115,80,114,101,99,101,100,105,110,103,71,108,121,112,104,80,101,110,115]));
        let a = PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[12290,65292]), 0, None, None, None).unwrap(), &vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(1, 16 as f64, Some(0 as f64 as f64), Some(Rect::new(2 as f64 as f64, 0 as f64 as f64, 14 as f64 as f64, 16 as f64 as f64))).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(2, 16 as f64, Some(16 as f64 as f64), Some(Rect::new(2 as f64 as f64, 0 as f64 as f64, 14 as f64 as f64, 16 as f64 as f64))).unwrap()).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,49,41]), UString::from(format!("{}", ((a[0usize]).clone().range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,49,44,32,101,110,100,61,50,41]), UString::from(format!("{}", ((a[1usize]).clone().range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12 as f64, (a[0usize]).clone().ink_bounds.as_ref().unwrap().get_width(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12 as f64, (a[1usize]).clone().ink_bounds.as_ref().unwrap().get_width(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((a[0usize]).clone().ink_bounds_fallback.is_none() && (a[1usize]).clone().ink_bounds_fallback.is_none(), None).unwrap();
    });
}

#[test]
fn space_replacement_skips_disabled_mode_null_boundaries_and_exact_widths() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.spaceReplacementSkipsDisabledModeNullBoundariesAndExactWidths", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.spaceReplacementSkipsDisabledModeNullBoundariesAndExactWidths", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[115,112,97,99,101,82,101,112,108,97,99,101,109,101,110,116,83,107,105,112,115,68,105,115,97,98,108,101,100,77,111,100,101,78,117,108,108,66,111,117,110,100,97,114,105,101,115,65,110,100,69,120,97,99,116,87,105,100,116,104,115]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 1, None, Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 2, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let disabled = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&c, &edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(3), (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DISABLED).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((disabled.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, disabled.clusters[1usize].advance, None).unwrap();
        let replace = AutoSpacePolicy::new(Some(AutoSpaceMode::Replace), Some(AutoSpaceMode::Replace), Some(0.125), Some(1.0 / 3.0));
        let exact_width = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 1, Some(2 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 2, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let exact = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&exact_width, &edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(3), (replace).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((exact.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let lone = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
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
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[115,112,97,99,105,110,103,66,111,117,110,100,97,114,105,101,115,67,111,117,110,116,69,97,99,104,87,105,100,101,78,97,114,114,111,119,71,97,112,79,110,99,101]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let pair_wn = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let pair_wn_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(1, &pair_wn, &pair_wn_edges), None).unwrap();
        let pair_nw = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 1, None, None, None).unwrap()).clone(),
];
        let pair_nw_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(1, &pair_nw, &pair_nw_edges), None).unwrap();
        let space_right = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 2, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let space_right_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(1, &space_right, &space_right_edges), None).unwrap();
        let space_left = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 0, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 2, None, None, None).unwrap()).clone(),
];
        let space_left_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(2, &space_left, &space_left_edges), None).unwrap();
        let cjk_pair = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 1, None, None, None).unwrap()).clone(),
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
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[116,121,112,101,100,83,112,97,99,101,66,101,116,119,101,101,110,87,105,100,101,65,110,100,78,97,114,114,111,119,73,115,82,101,112,108,97,99,101,100,66,121,84,104,101,71,97,112]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let c = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 1, None, Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 2, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let r = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&c, &edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(3), AutoSpacePolicy::new(Some(AutoSpaceMode::Replace), Some(AutoSpaceMode::Replace), Some(0.125), Some(1.0 / 3.0)), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[103,97,112]), ((r.decisions[0usize]).clone().side).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[82,101,112,108,97,99,101]), ((r.decisions[0usize]).clone().mode).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,46,87,105,100,101]), ((r.decisions[0usize]).clone().boundary_role).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[84,101,120,116,65,117,116,111,83,112,97,99,101,82,101,112,108,97,99,101,58,101,97,115,116,45,97,115,105,97,110,45,115,112,97,99,105,110,103,45,87,45,115,112,97,99,101,45,78]), ((r.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, r.decisions[0usize].characters_affected, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, r.decisions[0usize].reduction_per_char, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, r.decisions[0usize].total_reduction, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, r.clusters[1usize].advance, None).unwrap();
    });
}

#[test]
fn union_without_bounds_falls_back_to_the_first_glyph() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.unionWithoutBoundsFallsBackToTheFirstGlyph", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.unionWithoutBoundsFallsBackToTheFirstGlyph", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[117,110,105,111,110,87,105,116,104,111,117,116,66,111,117,110,100,115,70,97,108,108,115,66,97,99,107,84,111,84,104,101,70,105,114,115,116,71,108,121,112,104]));
        let a = (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[65292]), 0, None, None, None).unwrap(), &vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(1, 8 as f64, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(2, 6 as f64, Some(8 as f64 as f64), None).unwrap()).clone(),
]).unwrap()[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[115,104,97,112,101,114,45,110,111,45,105,110,107,45,98,111,117,110,100,115]), (a.ink_bounds_fallback).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.body_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, (a.trailing_glue).clone().natural, None).unwrap();
    });
}

#[test]
fn unmatched_glyph_counts_record_the_ambiguous_fallback() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.unmatchedGlyphCountsRecordTheAmbiguousFallback", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.unmatchedGlyphCountsRecordTheAmbiguousFallback", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[117,110,109,97,116,99,104,101,100,71,108,121,112,104,67,111,117,110,116,115,82,101,99,111,114,100,84,104,101,65,109,98,105,103,117,111,117,115,70,97,108,108,98,97,99,107]));
        let a = PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_atoms(PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[12290,65292]), 0, None, None, None).unwrap(), &vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(1, 8 as f64, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(2, 8 as f64, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_g(3, 8 as f64, None, None).unwrap()).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((a[0usize]).clone().ink_bounds_fallback.as_ref().map_or(false, |v| v == &(UString::from("glyph-cluster-mapping-ambiguous").to_ustring())) && (a[1usize]).clone().ink_bounds_fallback.as_ref().map_or(false, |v| v == &(UString::from("glyph-cluster-mapping-ambiguous").to_ustring())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((a[0usize]).clone().ink_bounds.is_none() && (a[1usize]).clone().ink_bounds.is_none(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a[0usize].advance, None).unwrap();
    });
}

#[test]
fn virtual_gaps_respect_narrow_to_wide_edges_and_their_neighbours() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.virtualGapsRespectNarrowToWideEdgesAndTheirNeighbours", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.virtualGapsRespectNarrowToWideEdgesAndTheirNeighbours", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[118,105,114,116,117,97,108,71,97,112,115,82,101,115,112,101,99,116,78,97,114,114,111,119,84,111,87,105,100,101,69,100,103,101,115,65,110,100,84,104,101,105,114,78,101,105,103,104,98,111,117,114,115]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let attachments = vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None];
        let reversed = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 0, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[114,101,102]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 2, None, None, None).unwrap()).clone(),
];
        let reversed_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let reversed_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&reversed, &reversed_edges, &attachments, (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((reversed_result.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,65,117,116,111,83,112,97,99,101,58,101,97,115,116,45,97,115,105,97,110,45,115,112,97,99,105,110,103,45,87,45,78]), ((reversed_result.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let space_after = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[114,101,102]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[32]), 2, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let space_after_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&space_after, &space_after_edges, &attachments, (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let break_after = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[114,101,102]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[10]), 2, Some(16 as f64 as f64), Some(UString::from("mandatory-break")), Some(UString::from(""))).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&break_after, &space_after_edges, &attachments, (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let cjk_after = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[114,101,102]), 1, Some(16 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 2, None, None, None).unwrap()).clone(),
];
        let cjk_after_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&cjk_after, &cjk_after_edges, &attachments, (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap().decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn wide_to_narrow_boundaries_insert_leading_and_trailing_gaps() {
    testlib::run("org.tiqian.layout.PunctuationGeometryStageCoverageTest.wideToNarrowBoundariesInsertLeadingAndTrailingGaps", "org.tiqian.layout.PunctuationGeometryStageCoverageTest.wideToNarrowBoundariesInsertLeadingAndTrailingGaps", || {
        PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_start(UStr::new(&[119,105,100,101,84,111,78,97,114,114,111,119,66,111,117,110,100,97,114,105,101,115,73,110,115,101,114,116,76,101,97,100,105,110,103,65,110,100,84,114,97,105,108,105,110,103,71,97,112,115]));
        let _ = PunctuationGeometryStageCoverageSupport;
        let leading = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 0, None, None, None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 1, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
];
        let leading_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
];
        let leading_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&leading, &leading_edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(2), (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,101,97,100,105,110,103]), ((leading_result.decisions[0usize]).clone().side).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,46,87,105,100,101]), ((leading_result.decisions[0usize]).clone().boundary_role).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[84,101,120,116,65,117,116,111,83,112,97,99,101,73,110,115,101,114,116,58,101,97,115,116,45,97,115,105,97,110,45,115,112,97,99,105,110,103,45,87,45,78]), ((leading_result.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(i32::from_ne_bytes(((4294967294u32) as i32).to_ne_bytes()) as f64, leading_result.decisions[0usize].total_reduction, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, leading_result.clusters[1usize].advance, None).unwrap();
        let trailing = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[97]), 0, Some(8 as f64 as f64), Some(UString::from("latin")), None).unwrap()).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_c(UStr::new(&[20013]), 1, None, None, None).unwrap()).clone(),
];
        let trailing_edges = vec![
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Narrow)).clone(),
    (PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_e(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide)).clone(),
];
        let trailing_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&trailing, &trailing_edges, &PunctuationGeometryStageCoverageSupport::punctuation_geometry_stage_coverage_support_none(2), (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone(), 16 as f64, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[116,114,97,105,108,105,110,103]), ((trailing_result.decisions[0usize]).clone().side).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, trailing_result.clusters[0usize].advance, None).unwrap();
    });
}
