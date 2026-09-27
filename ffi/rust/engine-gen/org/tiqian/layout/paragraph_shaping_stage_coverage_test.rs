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
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestProgressiveTierLoopRevisitsOffsetsWithLowerPriorityTiersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
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
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"clusterPredicatesAndCurlyQuoteFeatures");
        let a = Cluster::new(TextRange::new(0u32, 1u32).unwrap(), concat!("\n",
""), "mandatory-break", 0.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0));
        let _ = TracedAssertions::traced_assertions_assert_true(ParagraphShapingStage::paragraph_shaping_stage_is_mandatory_break_cluster((a).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_zero_width_soft_break_cluster((a).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_inline_object_cluster((a).clone()), None).unwrap();
        let b = Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "​", "zero-width-space", 0.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0));
        let _ = TracedAssertions::traced_assertions_assert_true(ParagraphShapingStage::paragraph_shaping_stage_is_zero_width_soft_break_cluster((b).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_mandatory_break_cluster((b).clone()), None).unwrap();
        let c = Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "x", "inline-object", 20.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0));
        let _ = TracedAssertions::traced_assertions_assert_true(ParagraphShapingStage::paragraph_shaping_stage_is_inline_object_cluster((c).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_mandatory_break_cluster((c).clone()), None).unwrap();
        let d = Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "中", "font", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0));
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_mandatory_break_cluster((d).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_zero_width_soft_break_cluster((d).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ParagraphShapingStage::paragraph_shaping_stage_is_inline_object_cluster((d).clone()), None).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, None).unwrap(), &"“双引号”与‘单引号’", 300.0f64).unwrap();
    });
}

#[test]
fn dash_substitution_rollback_and_coverage_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.dashSubstitutionRollbackAndCoverageBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.dashSubstitutionRollbackAndCoverageBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"dashSubstitutionRollbackAndCoverageBranches");
        {
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphDeficientDashShaper::new())), None).unwrap();
                let _ = { let __mutref_read = if e.text_shaper.__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { "……".to_string() } else { "——".to_string() };
ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read.as_str(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphSufficientDashShaper::new())), None).unwrap();
                let _ = { let __mutref_read1 = if e.text_shaper.__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { "……".to_string() } else { "——".to_string() };
ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read1.as_str(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphRollbackShaper::new())), None).unwrap();
                let _ = { let __mutref_read2 = if e.text_shaper.__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { "……".to_string() } else { "——".to_string() };
ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read2.as_str(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphMultiGlyphShaper::new())), None).unwrap();
                let _ = { let __mutref_read3 = if e.text_shaper.__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { "……".to_string() } else { "——".to_string() };
ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read3.as_str(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphMultiGlyphShaper::new())), None).unwrap();
                let _ = { let __mutref_read4 = if e.text_shaper.__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { "……".to_string() } else { "——".to_string() };
ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read4.as_str(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphMultiGlyphShaper::new())), None).unwrap();
                let _ = { let __mutref_read5 = if e.text_shaper.__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { "……".to_string() } else { "——".to_string() };
ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read5.as_str(), 300.0f64).unwrap() };
            }
            {
                let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphMultiGlyphShaper::new())), None).unwrap();
                let _ = { let __mutref_read6 = if e.text_shaper.__haxe_type_name() == "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper" { "……".to_string() } else { "——".to_string() };
ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, __mutref_read6.as_str(), 300.0f64).unwrap() };
            }
        }
    });
}

#[test]
fn direct_shape_paragraph_edge_cases() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.directShapeParagraphEdgeCases", "org.tiqian.layout.ParagraphShapingStageCoverageTest.directShapeParagraphEdgeCases", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"directShapeParagraphEdgeCases");
        let text =
"abcdef abcdeg antidisestablishmentarianism singlecluster Machine2Machine /a/b/c 12(3):. 12a(3):45 12(3a):45 12(3):-45 12(3):45- 12(3):45-6a 12(3):4a-65 12(3):abc aaaaaa111111 a1b2c3d4e5f6 http://example.com/foo https://example.com/foo?a=1&b=2#x%20~y abc.d abc.12 abc.de abc.de12 --.com foo.-bar /start end/ a/b a//b".to_string();
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphEmptyClusterShaper::new())), Some(Box::new(ParagraphDirectShapeHyphenator::new()))).unwrap();
        let i = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(text.as_str(), 1 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 10u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let p1 = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), text.as_str(), 1 as f64, FontRole::LatinText, Some(true)).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p1.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
        let p2 = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), text.as_str(), 40 as f64, FontRole::CjkText, Some(false)).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p2.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
        let si = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(&" ", 100 as f64, None).unwrap();
        let p3 = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (si).clone(), &" ", 100 as f64, FontRole::LatinText, Some(false)).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p3.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
    });
}

#[test]
fn hyphen_advance_fallback_when_shaper_returns_empty_clusters() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.hyphenAdvanceFallbackWhenShaperReturnsEmptyClusters", "org.tiqian.layout.ParagraphShapingStageCoverageTest.hyphenAdvanceFallbackWhenShaperReturnsEmptyClusters", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"hyphenAdvanceFallbackWhenShaperReturnsEmptyClusters");
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphEmptyHyphenShaper::new())), None).unwrap(),
&"supercalifragilisticexpialidocious", 50.0f64).unwrap();
    });
}

#[test]
fn latin_segmentation_and_cuts_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSegmentationAndCutsBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSegmentationAndCutsBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"latinSegmentationAndCutsBranches");
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, Some(Box::new(ParagraphSegmentationHyphenator::new()))).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e,
&"Text with ,Hello Machine2Machine XMLHttp HTTPServer TeX/LaTeX /start end/ /a a/ a/b https://example.com/path www.test.org sub.domain.co .com a. a..b a.b --.com test.-com test.c test.123 test.co123 12(3):45 12(3):45. 12(3):45-50 12(3):45–50 12(3):45—50 (1):2 a(1):2 1():2 1(2)a:3 1(2): 1(2):a-b 1(2):-5 1(2):5- 1(2):a 12():34 12(34): a(b):c-d 12(3):. 12a(3):45 12(3a):45 12(3):-45 12(3):45- 12(3):45-6a 12(3):4a-65 12(3):abc hyphenatedword VERYLONGALLCAPSWORDTHATISNOTANABBREVIATIONANDSHOULDBEOPAQ", 80 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, &"antidisestablishmentarianism abc def xyz", 30 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, &"semi-conductor co-19 a-b 3-4 COVID-19 cross-module-link", 80 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, &"aaaaaaaaaaaaaaaa 0123456789abcdef a1b2c3d4e5f6g7h8 aaaaaa111111 aaaaaaaaaaaa1 a1", 100 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, &"aBc ABc abC myIdentifier XML fooBAR aBC XMLHTTP", 100 as f64).unwrap();
    });
}

#[test]
fn latin_separator_cuts_and_solidus_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorCutsAndSolidusBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorCutsAndSolidusBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"latinSeparatorCutsAndSolidusBranches");
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, None).unwrap();
        let t = "http://example.com/path a/b /start end/ a//b foo_bar".to_string();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, t.as_str(), 500 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, t.as_str(), 1 as f64).unwrap();
    });
}

#[test]
fn latin_separator_cuts_exhaustive_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorCutsExhaustiveBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorCutsExhaustiveBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"latinSeparatorCutsExhaustiveBranches");
        let t = format!("{}{}{}",
            "12(3):45-67 12(3):45–67 12(3):45—67 12(3):45 12(3):. 12():45 12(3): :(3):45 12(3):- 12(3):45- 12(3):4a-65 12(3):45-6a 12(3):abc ",
            "http://example.com/a/b/c https://test.org:8080/foo?bar=1&baz=2#frag%20~val+1*2|3;4,5.6-7_8 http:/test /a a/ a//b a/b ",
            "ABC CamelCase aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa hyphenated-word clean/solidus hyphenated"
        );
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, Some(Box::new(ParagraphBiblioHyphenator::new()))).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, t.as_str(), 500 as f64).unwrap();
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut e, t.as_str(), 10 as f64).unwrap();
    });
}

#[test]
fn latin_separator_tokens_cover_url_leading_slash_and_dash_locators() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorTokensCoverUrlLeadingSlashAndDashLocators", "org.tiqian.layout.ParagraphShapingStageCoverageTest.latinSeparatorTokensCoverUrlLeadingSlashAndDashLocators", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"latinSeparatorTokensCoverUrlLeadingSlashAndDashLocators");
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, None).unwrap();
        {
            {
                let t = "//example.com/a".to_string();
                let i = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(t.as_str(), 500 as f64, None).unwrap();
                {
                    {
                        let m = 500.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_str(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
                    }
                    {
                        let m = 8.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_str(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
                    }
                }
            }
            {
                let t = "12(3):45–67".to_string();
                let i = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(t.as_str(), 500 as f64, None).unwrap();
                {
                    {
                        let m = 500.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_str(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
                    }
                    {
                        let m = 8.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_str(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
                    }
                }
            }
            {
                let t = "12(3):45—67".to_string();
                let i = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(t.as_str(), 500 as f64, None).unwrap();
                {
                    {
                        let m = 500.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_str(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
                    }
                    {
                        let m = 8.0f64;
                        let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph(&mut e, (i).clone(), t.as_str(), m, FontRole::LatinText, Some(false)).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
                    }
                }
            }
        }
    });
}

#[test]
fn latin_word_cuts_lo_hi_and_empty_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.latinWordCutsLoHiAndEmptyBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.latinWordCutsLoHiAndEmptyBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"latinWordCutsLoHiAndEmptyBranches");
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphWordShaper::new())),
Some(Box::new(ParagraphWordCutsHyphenator::new()))).unwrap(), &"abcdef ghijkl mnopqr empty", 1 as f64).unwrap();
    });
}

#[test]
fn map_to_cluster_range_with_zero_and_positive_advance() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.mapToClusterRangeWithZeroAndPositiveAdvance", "org.tiqian.layout.ParagraphShapingStageCoverageTest.mapToClusterRangeWithZeroAndPositiveAdvance", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"mapToClusterRangeWithZeroAndPositiveAdvance");
        let c = Cluster::new(TextRange::new(0u32, 4u32).unwrap(), "test", "k", 20.0f64, Some("test".to_string()), Some(0.0), Some(0.0), Some(0.0));
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
                    let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 4u32).unwrap().to_string().as_str(), ((m[0usize]).clone().cluster_range).clone().to_string().as_str(), None).unwrap();
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
                    let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 4u32).unwrap().to_string().as_str(), ((m[0usize]).clone().cluster_range).clone().to_string().as_str(), None).unwrap();
                }
            }
        }
    });
}

#[test]
fn multi_cluster_shaper_for_word_cuts_and_opaque_hard_cuts() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.multiClusterShaperForWordCutsAndOpaqueHardCuts", "org.tiqian.layout.ParagraphShapingStageCoverageTest.multiClusterShaperForWordCutsAndOpaqueHardCuts", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"multiClusterShaperForWordCutsAndOpaqueHardCuts");
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_layout(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(Some(Box::new(ParagraphMultiClusterShaper::new())),
Some(Box::new(ParagraphCoverageHyphenator::new(Some(2))))).unwrap(), &"antidisestablishmentarianism some_opaque_token_with_separators/and/more", 20 as f64).unwrap();
    });
}

#[test]
fn progressive_technical_span_breaks_and_tiers() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTechnicalSpanBreaksAndTiers", "org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTechnicalSpanBreaksAndTiers", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"progressiveTechnicalSpanBreaksAndTiers");
        let t = "Machine2Machine /v2.0_alpha=beta&gamma supercalifragilisticexpialidocious short".to_string();
        let mut e = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None, Some(Box::new(ParagraphTierHyphenator::new()))).unwrap();
        let i = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(t.as_str(), 80 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, u_string::unit_count(&(t))).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new(TextRange::new(5u32, 10u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let key = TextRange::new(0u32, u_string::unit_count(&(t))).unwrap();
        {
            {
                let tier = ProgressiveBreakTier::Structural;
                let ti = ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(tier);
                let mut tiers: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
                tiers.put(&(ti));
                let mut m: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
                m.put(&(key), &(tiers.clone().build()));
                let ann = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut e, (i).clone(), m.clone().build()).unwrap();
                let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut e, (i).clone(), (ann).clone(), m.clone().build()).unwrap();
                let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str()).unwrap().as_str(), None).unwrap();
            }
            {
                let tier = ProgressiveBreakTier::Syllable;
                let ti = ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(tier);
                let mut tiers: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
                tiers.put(&(ti));
                let mut m: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
                m.put(&(key), &(tiers.clone().build()));
                let ann = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut e, (i).clone(), m.clone().build()).unwrap();
                let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut e, (i).clone(), (ann).clone(), m.clone().build()).unwrap();
                let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str()).unwrap().as_str(), None).unwrap();
            }
            {
                let tier = ProgressiveBreakTier::Emergency;
                let ti = ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(tier);
                let mut tiers: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
                tiers.put(&(ti));
                let mut m: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
                m.put(&(key), &(tiers.clone().build()));
                let ann = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut e, (i).clone(), m.clone().build()).unwrap();
                let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut e, (i).clone(), (ann).clone(), m.clone().build()).unwrap();
                let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str()).unwrap().as_str(), None).unwrap();
            }
        }
        let ann_empty = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut e, (i).clone(), SortedTable::sorted_table_map_builder::<TextRange,
SortedSetTable<u32>>(Arc::new(compare_text_range)).clone().build()).unwrap();
        let mut multi: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        multi.put(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(ProgressiveBreakTier::Structural)));
        multi.put(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(ProgressiveBreakTier::Syllable)));
        let mut mm: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
        mm.put(&(key), &(multi.clone().build()));
        let prep_multi = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut e, (i).clone(), (ann_empty).clone(), mm.clone().build()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str()).unwrap().as_str(), None).unwrap();
    });
}

#[test]
fn progressive_technical_tier_priority_and_false_branches() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTechnicalTierPriorityAndFalseBranches", "org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTechnicalTierPriorityAndFalseBranches", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"progressiveTechnicalTierPriorityAndFalseBranches");
        let t = "abcdef/ghijkl".to_string();
        let prog_span = TextRange::new(0u32, 13u32).unwrap();
        {
            let input = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(t.as_str(), 10 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 2u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new((prog_span).clone(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new(TextRange::new(10u32, 13u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
            let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph_ranges_with_rejected_tiers(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None,
Some(Box::new(ParagraphTierPriorityHyphenator::new()))).unwrap(), (input).clone(), t.as_str(), 10 as f64, &vec![
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
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
        }
    });
}

#[test]
fn progressive_tier_loop_revisits_offsets_with_lower_priority_tiers() {
    testlib::run("org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTierLoopRevisitsOffsetsWithLowerPriorityTiers", "org.tiqian.layout.ParagraphShapingStageCoverageTest.progressiveTierLoopRevisitsOffsetsWithLowerPriorityTiers", || {
        let _ = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_begin(&"progressiveTierLoopRevisitsOffsetsWithLowerPriorityTiers");
        {
            let p = ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_paragraph_ranges(&mut ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_engine(None,
Some(Box::new(ParagraphTierLoopHyphenator::new()))).unwrap(), ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_input(&"abcdef/", 4 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 7u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap(), &"abcdef/", 4 as f64, &vec![
    (ResolvedClusterRange::new(TextRange::new(0u32, 7u32).unwrap(), FontRole::LatinText, Some(false), Some(false), None)).clone(),
    (ResolvedClusterRange::new(TextRange::new(2u32, 7u32).unwrap(), FontRole::LatinText, Some(false), Some(false), None)).clone(),
], &vec![(TextRange::new(0u32, 7u32).unwrap()).clone(), (TextRange::new(2u32, 7u32).unwrap()).clone()]).unwrap();
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { "null".to_string() } else { p.to_string() }.as_str()).unwrap().as_str(), None).unwrap();
        }
    });
}
