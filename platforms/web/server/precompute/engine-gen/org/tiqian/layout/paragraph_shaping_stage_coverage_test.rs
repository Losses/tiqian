#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::cluster_role_resolution::ResolvedClusterRange;
use crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStage;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphBiblioHyphenator;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphCoverageHyphenator;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphDeficientDashShaper;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphDirectShapeHyphenator;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphEmptyClusterShaper;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphEmptyHyphenShaper;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphMultiClusterShaper;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphMultiGlyphShaper;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphRollbackShaper;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphSegmentationHyphenator;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupport;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphSufficientDashShaper;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphTierHyphenator;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphTierLoopHyphenator;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphTierPriorityHyphenator;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphWordCutsHyphenator;
use crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphWordShaper;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_tier_priority::ProgressiveBreakTierPriority;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheFns;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::ParagraphShapingStageShapeParagraphFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalTierPriorityAndFalseBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::ParagraphShapingStageShapeParagraphFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        ParagraphShapingStageCoverageTestProgressiveTechnicalSpanBreaksAndTiersFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphShapingStageCoverageTestMapToClusterRangeWithZeroAndPositiveAdvanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    SupportParagraphFault(crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportParagraphFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::SupportParagraphFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::ParagraphShapingStageShapeParagraphFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault> for crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportParagraphFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::SupportParagraphFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportParagraphFault> for ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportParagraphFault) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::SupportParagraphFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorTokensCoverUrlLeadingSlashAndDashLocatorsFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportParagraphFault(crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportParagraphFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::SupportParagraphFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::ParagraphShapingStageShapeParagraphFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault> for crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportParagraphFault {
    fn from(value: ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::SupportParagraphFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportParagraphFault> for ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportParagraphFault) -> Self {
        ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::SupportParagraphFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        ParagraphShapingStageCoverageTestDirectShapeParagraphEdgeCasesFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    SupportLayoutFault(crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault> for crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault> for ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestClusterPredicatesAndCurlyQuoteFeaturesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault {
    SupportLayoutFault(crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault> for crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault> for ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestMultiClusterShaperForWordCutsAndOpaqueHardCutsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault {
    SupportLayoutFault(crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault> for crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault> for ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestLatinWordCutsLoHiAndEmptyBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    SupportLayoutFault(crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault> for crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault> for ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorCutsExhaustiveBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    SupportLayoutFault(crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault> for crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault> for ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestLatinSeparatorCutsAndSolidusBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    SupportLayoutFault(crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault> for crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault> for ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestLatinSegmentationAndCutsBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault {
    SupportLayoutFault(crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault> for crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault> for ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestHyphenAdvanceFallbackWhenShaperReturnsEmptyClustersFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    SupportLayoutFault(crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault> for crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault> for ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ParagraphShapingStageCoverageTestDashSubstitutionRollbackAndCoverageBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[test]
fn cluster_predicates_and_curly_quote_features() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.clusterPredicatesAndCurlyQuoteFeatures", "org.tiqian.layout.ParagraphShapingStageCoverageTest.clusterPredicatesAndCurlyQuoteFeatures", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[99,108,117,115,116,101,114,80,114,101,100,105,99,97,116,101,115,65,110,100,67,117,114,108,121,81,117,111,116,101,70,101,97,116,117,114,101,115]));
        let a = Cluster::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[10])), &(UStr::new(&[109,97,110,100,97,116,111,114,121,45,98,114,101,97,107])), 0.0f64, Some(UString::from("")), Some(0.0), Some(0.0), Some(0.0));
        let _ = TracedAssertions::traced_assertions_assert_true(ParagraphShapingStage::paragraph_shaping_stage_is_mandatory_break_cluster((a).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_zero_width_soft_break_cluster((a).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_inline_object_cluster((a).clone()), None).unwrap();
        let b = Cluster::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[8203])), &(UStr::new(&[122,101,114,111,45,119,105,100,116,104,45,115,112,97,99,101])), 0.0f64, Some(UString::from("")), Some(0.0), Some(0.0), Some(0.0));
        let _ = TracedAssertions::traced_assertions_assert_true(ParagraphShapingStage::paragraph_shaping_stage_is_zero_width_soft_break_cluster((b).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_mandatory_break_cluster((b).clone()), None).unwrap();
        let c = Cluster::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[120])), &(UStr::new(&[105,110,108,105,110,101,45,111,98,106,101,99,116])), 20.0f64, Some(UString::from("")), Some(0.0), Some(0.0), Some(0.0));
        let _ = TracedAssertions::traced_assertions_assert_true(ParagraphShapingStage::paragraph_shaping_stage_is_inline_object_cluster((c).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_mandatory_break_cluster((c).clone()), None).unwrap();
        let d = Cluster::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[20013])), &(UStr::new(&[102,111,110,116])), 16.0f64, Some(UString::from("中")), Some(0.0), Some(0.0), Some(0.0));
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_mandatory_break_cluster((d).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_zero_width_soft_break_cluster((d).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_inline_object_cluster((d).clone()), None).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, None).unwrap(), UStr::new(&[8220,21452,24341,21495,8221,19982,8216,21333,24341,21495,8217]), 300.0f64).unwrap();
    });
}

#[test]
fn dash_substitution_rollback_and_coverage_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.dashSubstitutionRollbackAndCoverageBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.dashSubstitutionRollbackAndCoverageBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[100,97,115,104,83,117,98,115,116,105,116,117,116,105,111,110,82,111,108,108,98,97,99,107,65,110,100,67,111,118,101,114,97,103,101,66,114,97,110,99,104,101,115]));
        {
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphDeficientDashShaper::new()))), None).unwrap();
                let _ = { let __mutref_read = if e.text_shaper.lock().unwrap().__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { UString::from("……") } else { UString::from("——") }; ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read.as_ustr(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphSufficientDashShaper::new()))), None).unwrap();
                let _ = { let __mutref_read1 = if e.text_shaper.lock().unwrap().__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { UString::from("……") } else { UString::from("——") }; ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read1.as_ustr(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphRollbackShaper::new()))), None).unwrap();
                let _ = { let __mutref_read2 = if e.text_shaper.lock().unwrap().__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { UString::from("……") } else { UString::from("——") }; ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read2.as_ustr(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphMultiGlyphShaper::new()))), None).unwrap();
                let _ = { let __mutref_read3 = if e.text_shaper.lock().unwrap().__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { UString::from("……") } else { UString::from("——") }; ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read3.as_ustr(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphMultiGlyphShaper::new()))), None).unwrap();
                let _ = { let __mutref_read4 = if e.text_shaper.lock().unwrap().__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { UString::from("……") } else { UString::from("——") }; ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read4.as_ustr(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphMultiGlyphShaper::new()))), None).unwrap();
                let _ = { let __mutref_read5 = if e.text_shaper.lock().unwrap().__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { UString::from("……") } else { UString::from("——") }; ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read5.as_ustr(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphMultiGlyphShaper::new()))), None).unwrap();
                let _ = { let __mutref_read6 = if e.text_shaper.lock().unwrap().__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { UString::from("……") } else { UString::from("——") }; ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read6.as_ustr(), 300.0f64).unwrap() };
            }
        }
    });
}

#[test]
fn direct_shape_paragraph_edge_cases() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.directShapeParagraphEdgeCases", "org.tiqian.layout.ParagraphShapingStageCoverageTest.directShapeParagraphEdgeCases", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[100,105,114,101,99,116,83,104,97,112,101,80,97,114,97,103,114,97,112,104,69,100,103,101,67,97,115,101,115]));
        let text = UString::from("abcdef abcdeg antidisestablishmentarianism singlecluster Machine2Machine /a/b/c 12(3):. 12a(3):45 12(3a):45 12(3):-45 12(3):45- 12(3):45-6a 12(3):4a-65 12(3):abc aaaaaa111111 a1b2c3d4e5f6 http://example.com/foo https://example.com/foo?a=1&b=2#x%20~y abc.d abc.12 abc.de abc.de12 --.com foo.-bar /start end/ a/b a//b").to_ustring();
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphEmptyClusterShaper::new()))), Some(Box::new(ParagraphDirectShapeHyphenator::new()))).unwrap();
        let i = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(text.as_ustr(), 1 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 10u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let p1 = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), text.as_ustr(), 1 as f64, FontRole::LatinText, Some(true)).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p1.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
        let p2 = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), text.as_ustr(), 40 as f64, FontRole::CjkText, Some(false)).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p2.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
        let si = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(UStr::new(&[32]), 100 as f64, None).unwrap();
        let p3 = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (si).clone(), UStr::new(&[32]), 100 as f64, FontRole::LatinText, Some(false)).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p3.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
    });
}

#[test]
fn hyphen_advance_fallback_when_shaper_returns_empty_clusters() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.hyphenAdvanceFallbackWhenShaperReturnsEmptyClusters", "org.tiqian.layout.ParagraphShapingStageCoverageTest.hyphenAdvanceFallbackWhenShaperReturnsEmptyClusters", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[104,121,112,104,101,110,65,100,118,97,110,99,101,70,97,108,108,98,97,99,107,87,104,101,110,83,104,97,112,101,114,82,101,116,117,114,110,115,69,109,112,116,121,67,108,117,115,116,101,114,115]));
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphEmptyHyphenShaper::new()))), None).unwrap(), UStr::new(&[115,117,112,101,114,99,97,108,105,102,114,97,103,105,108,105,115,116,105,99,101,120,112,105,97,108,105,100,111,99,105,111,117,115]), 50.0f64).unwrap();
    });
}

#[test]
fn latin_segmentation_and_cuts_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSegmentationAndCutsBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSegmentationAndCutsBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[108,97,116,105,110,83,101,103,109,101,110,116,97,116,105,111,110,65,110,100,67,117,116,115,66,114,97,110,99,104,101,115]));
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, Some(Box::new(ParagraphSegmentationHyphenator::new()))).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, UStr::new(&[84,101,120,116,32,119,105,116,104,32,44,72,101,108,108,111,32,77,97,99,104,105,110,101,50,77,97,99,104,105,110,101,32,88,77,76,72,116,116,112,32,72,84,84,80,83,101,114,118,101,114,32,84,101,88,47,76,97,84,101,88,32,47,115,116,97,114,116,32,101,110,100,47,32,47,97,32,97,47,32,97,47,98,32,104,116,116,112,115,58,47,47,101,120,97,109,112,108,101,46,99,111,109,47,112,97,116,104,32,119,119,119,46,116,101,115,116,46,111,114,103,32,115,117,98,46,100,111,109,97,105,110,46,99,111,32,46,99,111,109,32,97,46,32,97,46,46,98,32,97,46,98,32,45,45,46,99,111,109,32,116,101,115,116,46,45,99,111,109,32,116,101,115,116,46,99,32,116,101,115,116,46,49,50,51,32,116,101,115,116,46,99,111,49,50,51,32,49,50,40,51,41,58,52,53,32,49,50,40,51,41,58,52,53,46,32,49,50,40,51,41,58,52,53,45,53,48,32,49,50,40,51,41,58,52,53,8211,53,48,32,49,50,40,51,41,58,52,53,8212,53,48,32,40,49,41,58,50,32,97,40,49,41,58,50,32,49,40,41,58,50,32,49,40,50,41,97,58,51,32,49,40,50,41,58,32,49,40,50,41,58,97,45,98,32,49,40,50,41,58,45,53,32,49,40,50,41,58,53,45,32,49,40,50,41,58,97,32,49,50,40,41,58,51,52,32,49,50,40,51,52,41,58,32,97,40,98,41,58,99,45,100,32,49,50,40,51,41,58,46,32,49,50,97,40,51,41,58,52,53,32,49,50,40,51,97,41,58,52,53,32,49,50,40,51,41,58,45,52,53,32,49,50,40,51,41,58,52,53,45,32,49,50,40,51,41,58,52,53,45,54,97,32,49,50,40,51,41,58,52,97,45,54,53,32,49,50,40,51,41,58,97,98,99,32,104,121,112,104,101,110,97,116,101,100,119,111,114,100,32,86,69,82,89,76,79,78,71,65,76,76,67,65,80,83,87,79,82,68,84,72,65,84,73,83,78,79,84,65,78,65,66,66,82,69,86,73,65,84,73,79,78,65,78,68,83,72,79,85,76,68,66,69,79,80,65,81]), 80 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, UStr::new(&[97,110,116,105,100,105,115,101,115,116,97,98,108,105,115,104,109,101,110,116,97,114,105,97,110,105,115,109,32,97,98,99,32,100,101,102,32,120,121,122]), 30 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, UStr::new(&[115,101,109,105,45,99,111,110,100,117,99,116,111,114,32,99,111,45,49,57,32,97,45,98,32,51,45,52,32,67,79,86,73,68,45,49,57,32,99,114,111,115,115,45,109,111,100,117,108,101,45,108,105,110,107]), 80 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, UStr::new(&[97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,32,48,49,50,51,52,53,54,55,56,57,97,98,99,100,101,102,32,97,49,98,50,99,51,100,52,101,53,102,54,103,55,104,56,32,97,97,97,97,97,97,49,49,49,49,49,49,32,97,97,97,97,97,97,97,97,97,97,97,97,49,32,97,49]), 100 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, UStr::new(&[97,66,99,32,65,66,99,32,97,98,67,32,109,121,73,100,101,110,116,105,102,105,101,114,32,88,77,76,32,102,111,111,66,65,82,32,97,66,67,32,88,77,76,72,84,84,80]), 100 as f64).unwrap();
    });
}

#[test]
fn latin_separator_cuts_and_solidus_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorCutsAndSolidusBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorCutsAndSolidusBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[108,97,116,105,110,83,101,112,97,114,97,116,111,114,67,117,116,115,65,110,100,83,111,108,105,100,117,115,66,114,97,110,99,104,101,115]));
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, None).unwrap();
        let t = UString::from("http://example.com/path a/b /start end/ a//b foo_bar").to_ustring();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, t.as_ustr(), 500 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, t.as_ustr(), 1 as f64).unwrap();
    });
}

#[test]
fn latin_separator_cuts_exhaustive_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorCutsExhaustiveBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorCutsExhaustiveBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[108,97,116,105,110,83,101,112,97,114,97,116,111,114,67,117,116,115,69,120,104,97,117,115,116,105,118,101,66,114,97,110,99,104,101,115]));
        let t = { let mut __s = UString::new(); __s += &(UString::from("12(3):45-67 12(3):45–67 12(3):45—67 12(3):45 12(3):. 12():45 12(3): :(3):45 12(3):- 12(3):45- 12(3):4a-65 12(3):45-6a 12(3):abc ")); __s += &(UString::from("http://example.com/a/b/c https://test.org:8080/foo?bar=1&baz=2#frag%20~val+1*2|3;4,5.6-7_8 http:/test /a a/ a//b a/b ")); __s += &(UString::from("ABC CamelCase aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa hyphenated-word clean/solidus hyphenated")); __s };
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, Some(Box::new(ParagraphBiblioHyphenator::new()))).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, t.as_ustr(), 500 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, t.as_ustr(), 10 as f64).unwrap();
    });
}

#[test]
fn latin_separator_tokens_cover_url_leading_slash_and_dash_locators() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorTokensCoverUrlLeadingSlashAndDashLocators", "org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorTokensCoverUrlLeadingSlashAndDashLocators", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[108,97,116,105,110,83,101,112,97,114,97,116,111,114,84,111,107,101,110,115,67,111,118,101,114,85,114,108,76,101,97,100,105,110,103,83,108,97,115,104,65,110,100,68,97,115,104,76,111,99,97,116,111,114,115]));
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, None).unwrap();
        {
            {
                let t = UString::from("//example.com/a").to_ustring();
                let i = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(t.as_ustr(), 500 as f64, None).unwrap();
                {
                    {
                        let m = 500.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_ustr(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
                    }
                    {
                        let m = 8.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_ustr(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
                    }
                }
            }
            {
                let t = UString::from("12(3):45–67").to_ustring();
                let i = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(t.as_ustr(), 500 as f64, None).unwrap();
                {
                    {
                        let m = 500.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_ustr(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
                    }
                    {
                        let m = 8.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_ustr(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
                    }
                }
            }
            {
                let t = UString::from("12(3):45—67").to_ustring();
                let i = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(t.as_ustr(), 500 as f64, None).unwrap();
                {
                    {
                        let m = 500.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_ustr(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
                    }
                    {
                        let m = 8.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_ustr(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
                    }
                }
            }
        }
    });
}

#[test]
fn latin_word_cuts_lo_hi_and_empty_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.latinWordCutsLoHiAndEmptyBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.latinWordCutsLoHiAndEmptyBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[108,97,116,105,110,87,111,114,100,67,117,116,115,76,111,72,105,65,110,100,69,109,112,116,121,66,114,97,110,99,104,101,115]));
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphWordShaper::new()))), Some(Box::new(ParagraphWordCutsHyphenator::new()))).unwrap(), UStr::new(&[97,98,99,100,101,102,32,103,104,105,106,107,108,32,109,110,111,112,113,114,32,101,109,112,116,121]), 1 as f64).unwrap();
    });
}

#[test]
fn map_to_cluster_range_with_zero_and_positive_advance() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.mapToClusterRangeWithZeroAndPositiveAdvance", "org.tiqian.layout.ParagraphShapingStageCoverageTest.mapToClusterRangeWithZeroAndPositiveAdvance", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[109,97,112,84,111,67,108,117,115,116,101,114,82,97,110,103,101,87,105,116,104,90,101,114,111,65,110,100,80,111,115,105,116,105,118,101,65,100,118,97,110,99,101]));
        let c = Cluster::new(TextRange::new(0u32, 4u32).unwrap(), &(UStr::new(&[116,101,115,116])), &(UStr::new(&[107])), 20.0f64, Some(UString::from("test")), Some(0.0), Some(0.0), Some(0.0));
        {
            {
                let g = vec![
    (Glyph::new(1u32, TextRange::new(0u32, 2u32).unwrap(), 0 as f64 as f64, Some(0 as f64), Some(0.0), None, None, None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(2u32, 4u32).unwrap(), 0 as f64 as f64, Some(0 as f64), Some(0.0), None, None, None, None)).clone(),
];
                let m = ParagraphShapingStage::paragraph_shaping_stage_map_to_cluster_range(&g, (c).clone());
                let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((m.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_float(if g[0usize].advance <= 0 as f64 { 10 as f64 } else { g[0usize].advance } as f64, m[0usize].advance, None).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_float(if g[1usize].advance <= 0 as f64 { 10 as f64 } else { g[1usize].advance } as f64, m[1usize].advance, None).unwrap();
                if g[0usize].advance <= 0 as f64 {
                    let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(0u32, 4u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", ((m[0usize]).clone().cluster_range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
                }
            }
            {
                let g = vec![
    (Glyph::new(1u32, TextRange::new(0u32, 2u32).unwrap(), 8 as f64 as f64, Some(0 as f64), Some(0.0), None, None, None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(2u32, 4u32).unwrap(), 12 as f64 as f64, Some(8 as f64), Some(0.0), None, None, None, None)).clone(),
];
                let m = ParagraphShapingStage::paragraph_shaping_stage_map_to_cluster_range(&g, (c).clone());
                let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((m.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_float(if g[0usize].advance <= 0 as f64 { 10 as f64 } else { g[0usize].advance } as f64, m[0usize].advance, None).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_float(if g[1usize].advance <= 0 as f64 { 10 as f64 } else { g[1usize].advance } as f64, m[1usize].advance, None).unwrap();
                if g[0usize].advance <= 0 as f64 {
                    let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(0u32, 4u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", ((m[0usize]).clone().cluster_range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
                }
            }
        }
    });
}

#[test]
fn multi_cluster_shaper_for_word_cuts_and_opaque_hard_cuts() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.multiClusterShaperForWordCutsAndOpaqueHardCuts", "org.tiqian.layout.ParagraphShapingStageCoverageTest.multiClusterShaperForWordCutsAndOpaqueHardCuts", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[109,117,108,116,105,67,108,117,115,116,101,114,83,104,97,112,101,114,70,111,114,87,111,114,100,67,117,116,115,65,110,100,79,112,97,113,117,101,72,97,114,100,67,117,116,115]));
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Arc::new(Mutex::new(ParagraphMultiClusterShaper::new()))), Some(Box::new(ParagraphCoverageHyphenator::new(Some(2))))).unwrap(), UStr::new(&[97,110,116,105,100,105,115,101,115,116,97,98,108,105,115,104,109,101,110,116,97,114,105,97,110,105,115,109,32,115,111,109,101,95,111,112,97,113,117,101,95,116,111,107,101,110,95,119,105,116,104,95,115,101,112,97,114,97,116,111,114,115,47,97,110,100,47,109,111,114,101]), 20 as f64).unwrap();
    });
}

#[test]
fn progressive_technical_span_breaks_and_tiers() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTechnicalSpanBreaksAndTiers", "org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTechnicalSpanBreaksAndTiers", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[112,114,111,103,114,101,115,115,105,118,101,84,101,99,104,110,105,99,97,108,83,112,97,110,66,114,101,97,107,115,65,110,100,84,105,101,114,115]));
        let t = UString::from("Machine2Machine /v2.0_alpha=beta&gamma supercalifragilisticexpialidocious short").to_ustring();
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, Some(Box::new(ParagraphTierHyphenator::new()))).unwrap();
        let i = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(t.as_ustr(), 80 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, u_string::unit_count(&(t))).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new(TextRange::new(5u32, 10u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let key = TextRange::new(0u32, u_string::unit_count(&(t))).unwrap();
        {
            {
                let tier = ProgressiveBreakTier::Structural;
                let ti = ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(tier);
                let mut tiers: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
                tiers.put(&(ti));
                let mut m: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
                m.put(&(key), &(tiers.clone().build()));
                let ann = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut e, (i).clone(), m.clone().build()).unwrap();
                let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut e, (i).clone(), (ann).clone(), m.clone().build()).unwrap();
                let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from("ParagraphLayoutPrep@identity") }.as_ustr()).unwrap().as_ustr(), None).unwrap();
            }
            {
                let tier = ProgressiveBreakTier::Syllable;
                let ti = ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(tier);
                let mut tiers: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
                tiers.put(&(ti));
                let mut m: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
                m.put(&(key), &(tiers.clone().build()));
                let ann = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut e, (i).clone(), m.clone().build()).unwrap();
                let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut e, (i).clone(), (ann).clone(), m.clone().build()).unwrap();
                let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from("ParagraphLayoutPrep@identity") }.as_ustr()).unwrap().as_ustr(), None).unwrap();
            }
            {
                let tier = ProgressiveBreakTier::Emergency;
                let ti = ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(tier);
                let mut tiers: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
                tiers.put(&(ti));
                let mut m: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
                m.put(&(key), &(tiers.clone().build()));
                let ann = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut e, (i).clone(), m.clone().build()).unwrap();
                let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut e, (i).clone(), (ann).clone(), m.clone().build()).unwrap();
                let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from("ParagraphLayoutPrep@identity") }.as_ustr()).unwrap().as_ustr(), None).unwrap();
            }
        }
        let ann_empty = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut e, (i).clone(), SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range)).clone().build()).unwrap();
        let mut multi: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        multi.put(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(ProgressiveBreakTier::Structural)));
        multi.put(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(ProgressiveBreakTier::Syllable)));
        let mut mm: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
        mm.put(&(key), &(multi.clone().build()));
        let prep_multi = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut e, (i).clone(), (ann_empty).clone(), mm.clone().build()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from("ParagraphLayoutPrep@identity") }.as_ustr()).unwrap().as_ustr(), None).unwrap();
    });
}

#[test]
fn progressive_technical_tier_priority_and_false_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTechnicalTierPriorityAndFalseBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTechnicalTierPriorityAndFalseBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[112,114,111,103,114,101,115,115,105,118,101,84,101,99,104,110,105,99,97,108,84,105,101,114,80,114,105,111,114,105,116,121,65,110,100,70,97,108,115,101,66,114,97,110,99,104,101,115]));
        let t = UString::from("abcdef/ghijkl").to_ustring();
        let prog_span = TextRange::new(0u32, 13u32).unwrap();
        {
            let input = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(t.as_ustr(), 10 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 2u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new((prog_span).clone(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new(TextRange::new(10u32, 13u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
            let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph_ranges_with_rejected_tiers(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, Some(Box::new(ParagraphTierPriorityHyphenator::new()))).unwrap(), (input).clone(), t.as_ustr(), 10 as f64, &vec![
    (ResolvedClusterRange::new(TextRange::new(0u32, 7u32).unwrap(), FontRole::LatinText, Some(false), Some(false), None)).clone(),
    (ResolvedClusterRange::new(TextRange::new(2u32, 7u32).unwrap(), FontRole::LatinText, Some(false), Some(false), None)).clone(),
    (ResolvedClusterRange::new(TextRange::new(0u32, 0u32).unwrap(), FontRole::LatinText, Some(false), Some(false), None)).clone(),
], &vec![
    (TextRange::new(0u32, 7u32).unwrap()).clone(),
    (TextRange::new(2u32, 7u32).unwrap()).clone(),
    (TextRange::new(0u32, 0u32).unwrap()).clone(),
], &vec![(prog_span).clone()], &vec![
    (vec![
    ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(ProgressiveBreakTier::Structural),
    ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(ProgressiveBreakTier::Syllable),
]).clone(),
]).unwrap();
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
        }
    });
}

#[test]
fn progressive_tier_loop_revisits_offsets_with_lower_priority_tiers() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTierLoopRevisitsOffsetsWithLowerPriorityTiers", "org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTierLoopRevisitsOffsetsWithLowerPriorityTiers", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(UStr::new(&[112,114,111,103,114,101,115,115,105,118,101,84,105,101,114,76,111,111,112,82,101,118,105,115,105,116,115,79,102,102,115,101,116,115,87,105,116,104,76,111,119,101,114,80,114,105,111,114,105,116,121,84,105,101,114,115]));
        {
            let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph_ranges(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, Some(Box::new(ParagraphTierLoopHyphenator::new()))).unwrap(), ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(UStr::new(&[97,98,99,100,101,102,47]), 4 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 7u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap(), UStr::new(&[97,98,99,100,101,102,47]), 4 as f64, &vec![
    (ResolvedClusterRange::new(TextRange::new(0u32, 7u32).unwrap(), FontRole::LatinText, Some(false), Some(false), None)).clone(),
    (ResolvedClusterRange::new(TextRange::new(2u32, 7u32).unwrap(), FontRole::LatinText, Some(false), Some(false), None)).clone(),
], &vec![(TextRange::new(0u32, 7u32).unwrap()).clone(), (TextRange::new(2u32, 7u32).unwrap()).clone()]).unwrap();
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", p.to_string()).as_str()) }.as_ustr()).unwrap().as_ustr(), None).unwrap();
        }
    });
}
