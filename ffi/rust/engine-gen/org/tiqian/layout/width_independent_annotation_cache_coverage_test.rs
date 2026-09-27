#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::inline_object_preferred_stretch_kind::InlineObjectPreferredStretchKind;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::spacing_decision_info::SpacingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheFns;
use crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::ConflictingOpenTypeFeaturesShaper;
use crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::NarrowInkShaper;
use crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestVerbatimRangesAndAutoSpaceDecisionsFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestStyleAtAndEmphasisItalicAtAndDynamicShapingBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestShrinkOpportunitiesCoverAllPunctuationClassesAndSpacesFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadSecondVisitAndZeroFirstClusterFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestRubySpreadAccumulationAndEdgesFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPrepareWidthIndependentAnnotationBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestPairedPunctuationWithZeroCapacityFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportAnnotationForTextFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault::SupportAnnotationForTextFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault> for WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault::SupportAnnotationForTextFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLruCacheUpdateExistingKeyAndClearFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestLineLengthGridBodyAlignmentBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingTriggersAndEmphasisItalicFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestDynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranchesFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestContainingItemsAndFirstContainedItemBranchesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestConflictingOpenTypeFeaturesThrowsFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestCenteredPunctBeforeAttachedReferenceKeepsLeadingGlueOnlyFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    SupportEngineFault(crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault> for crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::SupportEngineFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_coverage_test_support::WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::SupportEngineFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestAdjacentInlineObjectBoundariesMergingAndConflictsFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[test]
fn lru_cache_update_existing_key_and_clear() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.lruCacheUpdateExistingKeyAndClear", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.lruCacheUpdateExistingKeyAndClear", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"lruCacheUpdateExistingKeyAndClear");
        let mut cache = LruWidthIndependentAnnotationCache::new(2u32);
        let dummy_input = LayoutInput::new(TiqianTextContent::new("测试缓存", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let key = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_key((dummy_input).clone());
        cache.put((key).clone(), WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_annotation_for_text(&"v1").unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, cache.get_size(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"v1", (cache.get((key).clone()).as_ref().unwrap().text).to_string().as_str(), None).unwrap();
        cache.put((key).clone(), WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_annotation_for_text(&"v2").unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, cache.get_size(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"v2", (cache.get((key).clone()).as_ref().unwrap().text).to_string().as_str(), None).unwrap();
        let key2 = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_key(LayoutInput::new((dummy_input.content).clone(), Some(TextStyle::new(Some(vec![]), Some(20.0f64), Some("zh-Hans".to_string()), Some(400), Some(false),
Some(0.0), Some(InlineAttachment::None))), Some((dummy_input.paragraph_style).clone()), (dummy_input.constraints).clone(), Some((dummy_input.profile_id).clone()), Some((dummy_input.decorations).clone()), Some((dummy_input.ruby_spans).clone()),
Some((dummy_input.inline_boxes).clone()), Some((dummy_input.inline_objects).clone())));
        cache.put((key2).clone(), WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_annotation_for_text(&"v3").unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, cache.get_size(), None).unwrap();
        let key3 = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_key(LayoutInput::new((dummy_input.content).clone(), Some(TextStyle::new(Some(vec![]), Some(30.0f64), Some("zh-Hans".to_string()), Some(400), Some(false),
Some(0.0), Some(InlineAttachment::None))), Some((dummy_input.paragraph_style).clone()), (dummy_input.constraints).clone(), Some((dummy_input.profile_id).clone()), Some((dummy_input.decorations).clone()), Some((dummy_input.ruby_spans).clone()),
Some((dummy_input.inline_boxes).clone()), Some((dummy_input.inline_objects).clone())));
        cache.put((key3).clone(), WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_annotation_for_text(&"v4").unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, cache.get_size(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(cache.get((key).clone()).is_none(), &"-", None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"v3", (cache.get((key2).clone()).as_ref().unwrap().text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"v4", (cache.get((key3).clone()).as_ref().unwrap().text).to_string().as_str(), None).unwrap();
        cache.clear();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, cache.get_size(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(cache.get((key2).clone()).is_none(), &"-", None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(cache.get((key3).clone()).is_none(), &"-", None).unwrap();
    });
}

#[test]
fn containing_items_and_first_contained_item_branches() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.containingItemsAndFirstContainedItemBranches", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.containingItemsAndFirstContainedItemBranches", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"containingItemsAndFirstContainedItemBranches");
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), "aa", "k", 10.0f64, Some("aa".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 5u32).unwrap(), "bbb", "k", 15.0f64, Some("bbb".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(5u32, 7u32).unwrap(), "cc", "k", 10.0f64, Some("cc".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(7u32, 9u32).unwrap(), "dd", "k", 10.0f64, Some("dd".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let items = vec![
    (TextRange::new(0u32, 2u32).unwrap()).clone(),
    (TextRange::new(1u32, 4u32).unwrap()).clone(),
    (TextRange::new(5u32, 8u32).unwrap()).clone(),
    (TextRange::new(10u32, 12u32).unwrap()).clone(),
];
        let contained = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_containing_items(&clusters, &items);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, u32::try_from((contained.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=2)", WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_render_nullable_range((contained[0usize]).clone()).as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered((contained[1usize]).clone().is_none(), &"-", None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=5, end=8)", WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_render_nullable_range((contained[2usize]).clone()).as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered((contained[3usize]).clone().is_none(), &"-", None).unwrap();
        let first_contained = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_first_contained_item(&clusters, &items);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, u32::try_from((first_contained.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=2)", WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_render_nullable_range((first_contained[0usize]).clone()).as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered((first_contained[1usize]).clone().is_none(), &"-", None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered((first_contained[2usize]).clone().is_none(), &"-", None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered((first_contained[3usize]).clone().is_none(), &"-", None).unwrap();
    });
}

#[test]
fn prepare_width_independent_annotation_branches() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.prepareWidthIndependentAnnotationBranches", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.prepareWidthIndependentAnnotationBranches", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"prepareWidthIndependentAnnotationBranches");
        let mut engine = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).unwrap();
        let input = LayoutInput::new(TiqianTextContent::new("测试文本【中文】与English，以及注音与行内框。", Some(vec![
    (TextSpan::new(TextRange::new(0u32, 0u32).unwrap(), TextStyle::new(Some(vec![]), Some(10.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
    (TextSpan::new(TextRange::new(0u32, 1u32).unwrap(), TextStyle::new(Some(vec![]), Some(18.0f64), Some("zh-Hans".to_string()), Some(500), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
    (TextSpan::new(TextRange::new(1u32, 4u32).unwrap(), TextStyle::new(Some(vec![]), Some(18.0f64), Some("zh-Hans".to_string()), Some(500), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
    (TextSpan::new(TextRange::new(4u32, 8u32).unwrap(), TextStyle::new(Some(vec![]), Some(14.0f64), Some("zh-Hans".to_string()), Some(300), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![1, 2, 3, 4, 6]), Some(vec![
    (LineBreakSpan::new(TextRange::new(8u32, 15u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-CN".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 4u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(4u32, 8u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), "cèshì", Some(vec![]), RubyKind::Pinyin, Some("zh-Latn".to_string()))).clone(),
    (RubySpan::new(TextRange::new(2u32, 4u32).unwrap(), "", Some(vec![]), RubyKind::Pinyin, None)).clone(),
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "˙ㄅ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "ㄆ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(99u32, 100u32).unwrap(), "invalid", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
]), Some(vec![
    (InlineBoxSpan::new(TextRange::new(15u32, 17u32).unwrap(), Some(4.0f64), Some(0.0f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(17u32, 19u32).unwrap(), Some(0.0f64), Some(4.0f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(19u32, 21u32).unwrap(), Some(0.0), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(21u32, 23u32).unwrap(), Some(0.0f64), Some(0.0f64), Some(InlineBoxOuterSpacing::Source))).clone(),
]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(23u32, 24u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]));
        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "WidthIndependentParagraphAnnotation@identity".to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.0f64, (annotation.font_size_at)(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.0f64, (annotation.font_size_at)(5), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, (annotation.font_size_at)(24), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(800, (annotation.bopomofo_font_weight_at)(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(600, (annotation.bopomofo_font_weight_at)(5), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(700, (annotation.bopomofo_font_weight_at)(24), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.0f64, (annotation.style_at)(0).font_size, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.0f64, (annotation.style_at)(3).font_size, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.0f64, (annotation.style_at)(4).font_size, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.0f64, (annotation.style_at)(7).font_size, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, (annotation.style_at)(8).font_size, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, (annotation.style_at)(25).font_size, None).unwrap();
        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((prep.ruby_and_bopomofo_spread.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn line_length_grid_body_alignment_branches() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.lineLengthGridBodyAlignmentBranches", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.lineLengthGridBodyAlignmentBranches", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"lineLengthGridBodyAlignmentBranches");
        let mut engine = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).unwrap();
        let text = "一二三四五六七八九十".to_string();
        {
            {
                let align = LastLineAlignment::Start;
                let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), Some(align))), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
                let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                if align == LastLineAlignment::Start {
                    let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, prep.grid_body_offset, None).unwrap();
                } else {
                    if align == LastLineAlignment::Center {
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(2.0f64, prep.grid_body_offset, 0.001f64, None).unwrap();
                    } else {
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(4.0f64, prep.grid_body_offset, 0.001f64, None).unwrap();
                    }
                }
            }
            {
                let align = LastLineAlignment::Center;
                let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), Some(align))), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
                let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                if align == LastLineAlignment::Start {
                    let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, prep.grid_body_offset, None).unwrap();
                } else {
                    if align == LastLineAlignment::Center {
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(2.0f64, prep.grid_body_offset, 0.001f64, None).unwrap();
                    } else {
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(4.0f64, prep.grid_body_offset, 0.001f64, None).unwrap();
                    }
                }
            }
            {
                let align = LastLineAlignment::End;
                let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), Some(align))), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
                let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                if align == LastLineAlignment::Start {
                    let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, prep.grid_body_offset, None).unwrap();
                } else {
                    if align == LastLineAlignment::Center {
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(2.0f64, prep.grid_body_offset, 0.001f64, None).unwrap();
                    } else {
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(4.0f64, prep.grid_body_offset, 0.001f64, None).unwrap();
                    }
                }
            }
        }
    });
}

#[test]
fn dynamic_shaping_triggers_and_emphasis_italic() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.dynamicShapingTriggersAndEmphasisItalic", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.dynamicShapingTriggersAndEmphasisItalic", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"dynamicShapingTriggersAndEmphasisItalic");
        let mut engine = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).unwrap();
        let simple_input = LayoutInput::new(TiqianTextContent::new("中文正文排版", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(500 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let simple_annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (simple_input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let simple_prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (simple_input).clone(), (simple_annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
        let input = LayoutInput::new(TiqianTextContent::new("Hello World with English Words", Some(vec![]), Some(vec![]), Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 11u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(50 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 5u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(6u32, 11u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_tier_map(TextRange::new(0u32, 11u32).unwrap(), &vec![ProgressiveBreakTier::Structural])).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
        let over_measure_input = LayoutInput::new(TiqianTextContent::new("VeryLongEnglishWordThatExceedsMeasure", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(30 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let over_measure_annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (over_measure_input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let over_measure_prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (over_measure_input).clone(), (over_measure_annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn conflicting_open_type_features_throws() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.conflictingOpenTypeFeaturesThrows", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.conflictingOpenTypeFeaturesThrows", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"conflictingOpenTypeFeaturesThrows");
        let engine: Arc<Mutex<ExplainableStubParagraphLayoutEngine>> = Arc::new(Mutex::new(WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None,
Some(Box::new(ConflictingOpenTypeFeaturesShaper::new()))).unwrap()));
        let input = LayoutInput::new(TiqianTextContent::new("测试", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let error = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let engine = (engine).clone(); let input = (input).clone(); let annotation = (annotation).clone(); Arc::new({ let engine = Arc::clone(&engine); move || {
        WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).map_err(|e| IllegalStateException::new(&format!("{:?}", e)))?;
        Ok(())
} }) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&format!("{}", error), "Conflicting OpenType features", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn adjacent_inline_object_boundaries_merging_and_conflicts() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.adjacentInlineObjectBoundariesMergingAndConflicts", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.adjacentInlineObjectBoundariesMergingAndConflicts", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"adjacentInlineObjectBoundariesMergingAndConflicts");
        let engine: Arc<Mutex<ExplainableStubParagraphLayoutEngine>> = Arc::new(Mutex::new(WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).unwrap()));
        let text = "一二三四".to_string();
        {
            {
                let uniform1 = true;
                {
                    {
                        let uniform2 = true;
                        {
                            {
                                let prevent1 = true;
                                {
                                    {
                                        let prevent2 = true;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                    {
                                        let prevent2 = false;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                }
                            }
                            {
                                let prevent1 = false;
                                {
                                    {
                                        let prevent2 = true;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                    {
                                        let prevent2 = false;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                }
                            }
                        }
                    }
                    {
                        let uniform2 = false;
                        {
                            {
                                let prevent1 = true;
                                {
                                    {
                                        let prevent2 = true;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                    {
                                        let prevent2 = false;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                }
                            }
                            {
                                let prevent1 = false;
                                {
                                    {
                                        let prevent2 = true;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                    {
                                        let prevent2 = false;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }
            {
                let uniform1 = false;
                {
                    {
                        let uniform2 = true;
                        {
                            {
                                let prevent1 = true;
                                {
                                    {
                                        let prevent2 = true;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                    {
                                        let prevent2 = false;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                }
                            }
                            {
                                let prevent1 = false;
                                {
                                    {
                                        let prevent2 = true;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                    {
                                        let prevent2 = false;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                }
                            }
                        }
                    }
                    {
                        let uniform2 = false;
                        {
                            {
                                let prevent1 = true;
                                {
                                    {
                                        let prevent2 = true;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                    {
                                        let prevent2 = false;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                }
                            }
                            {
                                let prevent1 = false;
                                {
                                    {
                                        let prevent2 = true;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                    {
                                        let prevent2 = false;
                                        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::new(Some(uniform1), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(2.0f64), Some(1.0f64), Some(prevent1)).unwrap())).unwrap();
                                        let obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(uniform2),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(prevent2)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
                                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (obj2).clone()]));
                                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let obj1 = InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(0.0), Some(0.0f64), Some(false)).unwrap())).unwrap();
        let conflicting_obj2 = InlineObjectSpan::new(TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(false), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 10.0f64, 20.0f64).unwrap()),
Some(0.0), Some(0.0f64), Some(false)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let conflict_input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj1).clone(), (conflicting_obj2).clone()]));
        let conflict_annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine.lock().unwrap(), (conflict_input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let conflict_error = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let engine = (engine).clone(); let conflict_input = (conflict_input).clone(); let conflict_annotation = (conflict_annotation).clone(); Arc::new({ let engine = Arc::clone(&engine);
move || {
        WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine.lock().unwrap(), (conflict_input).clone(), (conflict_annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).map_err(|e| IllegalStateException::new(&format!("{:?}", e)))?;
        Ok(())
} }) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&format!("{}", conflict_error), "Conflicting inline-object stretch classes", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn verbatim_ranges_and_auto_space_decisions() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.verbatimRangesAndAutoSpaceDecisions", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.verbatimRangesAndAutoSpaceDecisions", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"verbatimRangesAndAutoSpaceDecisions");
        let mut engine = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).unwrap();
        let text = "中文 English 混排测试 12345".to_string();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(TextRange::new(0u32, 15u32).unwrap()).clone()])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false),
Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true),
None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY),
Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![
    (InlineBoxSpan::new(TextRange::new(2u32, 9u32).unwrap(), Some(0.0), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), Some(vec![]));
        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn ruby_spread_accumulation_and_edges() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.rubySpreadAccumulationAndEdges", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.rubySpreadAccumulationAndEdges", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"rubySpreadAccumulationAndEdges");
        let mut engine = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).unwrap();
        let text = "中文测试段落".to_string();
        let ruby0 = RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), "zhōngwén", Some(vec![]), RubyKind::Pinyin, None);
        let ruby1 = RubySpan::new(TextRange::new(2u32, 4u32).unwrap(), "cèshìchángdà", Some(vec![]), RubyKind::Pinyin, None);
        let ruby2 = RubySpan::new(TextRange::new(4u32, 6u32).unwrap(), "duànluòchángdà", Some(vec![]), RubyKind::Pinyin, None);
        let ruby_invalid = RubySpan::new(TextRange::new(99u32, 100u32).unwrap(), "invalid", Some(vec![]), RubyKind::Pinyin, None);
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![1, 2, 3, 4, 5]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![(ruby0).clone(), (ruby1).clone(), (ruby2).clone(), (ruby_invalid).clone()]), Some(vec![]), Some(vec![]));
        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn shrink_opportunities_cover_all_punctuation_classes_and_spaces() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.shrinkOpportunitiesCoverAllPunctuationClassesAndSpaces", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.shrinkOpportunitiesCoverAllPunctuationClassesAndSpaces", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"shrinkOpportunitiesCoverAllPunctuationClassesAndSpaces");
        let mut engine = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(Some(WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_non_gb_resolver()),
None).unwrap();
        let text = "「引用」·中点‧间隔•中点，逗号。句号！问号？．点号、顿号以及 English words 间距".to_string();
        let spans = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_text_span_list(text.as_str(), 16.0f64).unwrap();
        {
            {
                let allow_inline_stop = true;
                {
                    {
                        let allow_sino_western = true;
                        let mut sbs: Vec<u32> = vec![];
                        for i in 0..u_string::count(text.as_str()) {
                            sbs.push(i);
                        }
                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some((spans).clone()), Some((sbs).clone()), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(5.0f64), Some(0.0f64),
Some(false)).unwrap())).unwrap()).clone(),
]));
                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                        let modified_annotation = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_with_adjusted_profile((annotation).clone(), allow_inline_stop, allow_sino_western);
                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (modified_annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((prep.shrink_opportunities.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
                    }
                    {
                        let allow_sino_western = false;
                        let mut sbs: Vec<u32> = vec![];
                        for i in 0..u_string::count(text.as_str()) {
                            sbs.push(i);
                        }
                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some((spans).clone()), Some((sbs).clone()), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(5.0f64), Some(0.0f64),
Some(false)).unwrap())).unwrap()).clone(),
]));
                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                        let modified_annotation = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_with_adjusted_profile((annotation).clone(), allow_inline_stop, allow_sino_western);
                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (modified_annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((prep.shrink_opportunities.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
                    }
                }
            }
            {
                let allow_inline_stop = false;
                {
                    {
                        let allow_sino_western = true;
                        let mut sbs: Vec<u32> = vec![];
                        for i in 0..u_string::count(text.as_str()) {
                            sbs.push(i);
                        }
                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some((spans).clone()), Some((sbs).clone()), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(5.0f64), Some(0.0f64),
Some(false)).unwrap())).unwrap()).clone(),
]));
                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                        let modified_annotation = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_with_adjusted_profile((annotation).clone(), allow_inline_stop, allow_sino_western);
                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (modified_annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((prep.shrink_opportunities.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
                    }
                    {
                        let allow_sino_western = false;
                        let mut sbs: Vec<u32> = vec![];
                        for i in 0..u_string::count(text.as_str()) {
                            sbs.push(i);
                        }
                        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some((spans).clone()), Some((sbs).clone()), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), 20.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(5.0f64), Some(0.0f64),
Some(false)).unwrap())).unwrap()).clone(),
]));
                        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                        let modified_annotation = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_with_adjusted_profile((annotation).clone(), allow_inline_stop, allow_sino_western);
                        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (modified_annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
                        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((prep.shrink_opportunities.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
                    }
                }
            }
        }
    });
}

#[test]
fn style_at_and_emphasis_italic_at_and_dynamic_shaping_branches() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.styleAtAndEmphasisItalicAtAndDynamicShapingBranches", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.styleAtAndEmphasisItalicAtAndDynamicShapingBranches", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"styleAtAndEmphasisItalicAtAndDynamicShapingBranches");
        let mut engine = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).unwrap();
        let text = "English 中文 混排 Latin 测试 样式".to_string();
        let spans = vec![
    (TextSpan::new(TextRange::new(8u32, 10u32).unwrap(), TextStyle::new(Some(vec![]), Some(24.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
];
        let decorations = vec![
    (DecorationSpan::new(TextRange::new(0u32, 7u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(11u32, 13u32).unwrap(), DecorationKind::ProperNoun)).clone(),
];
        let line_break_spans = vec![
    (LineBreakSpan::new(TextRange::new(0u32, 7u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
];
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some((spans).clone()), Some(vec![]), Some((line_break_spans).clone()), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(50 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some((decorations).clone()), Some(vec![]), Some(vec![]), Some(vec![]));
        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(24.0f64, (annotation.font_size_at)(8), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(24.0f64, (annotation.font_size_at)(9), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float((input.text_style).clone().font_size, (annotation.font_size_at)(4294967295u32), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float((input.text_style).clone().font_size, (annotation.font_size_at)(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float((input.text_style).clone().font_size, (annotation.font_size_at)(7), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float((input.text_style).clone().font_size, (annotation.font_size_at)(10), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float((input.text_style).clone().font_size, (annotation.font_size_at)(20), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float((input.text_style).clone().font_size, (annotation.font_size_at)(100), None).unwrap();
        let rejected: SortedMapTable<TextRange, SortedSetTable<u32>> = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_tier_map(TextRange::new(0u32, 7u32).unwrap(), &vec![ProgressiveBreakTier::Structural]);
        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (annotation).clone(), (rejected).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
        let no_break_input = LayoutInput::new(TiqianTextContent::new("English", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(500 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let no_break_annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (no_break_input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let prep_no_dynamic = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (no_break_input).clone(), (no_break_annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
        let small_measure_input = LayoutInput::new(TiqianTextContent::new("English", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(1 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let small_annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (small_measure_input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let prep_small = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (small_measure_input).clone(), (small_annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
        let modified_annotation = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_with_first_font_decision_only((annotation).clone());
        let prep_unknown_roles = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (modified_annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn ruby_spread_second_visit_and_zero_first_cluster() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.rubySpreadSecondVisitAndZeroFirstCluster", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.rubySpreadSecondVisitAndZeroFirstCluster", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"rubySpreadSecondVisitAndZeroFirstCluster");
        let mut engine = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).unwrap();
        let text = "一二三四五六七八".to_string();
        let ruby0a = RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "chángdàchángdà", Some(vec![]), RubyKind::Pinyin, None);
        let ruby0b = RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "chángdàchángdà", Some(vec![]), RubyKind::Pinyin, None);
        let ruby1 = RubySpan::new(TextRange::new(2u32, 3u32).unwrap(), "chángdàchángdàchángdà", Some(vec![]), RubyKind::Pinyin, None);
        let ruby2 = RubySpan::new(TextRange::new(2u32, 3u32).unwrap(), "chángdàchángdàchángdà", Some(vec![]), RubyKind::Pinyin, None);
        let mut sbs: Vec<u32> = vec![];
        for i in 0..u_string::count(text.as_str()) {
            sbs.push(i);
        }
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some((sbs).clone()), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![(ruby0a).clone(), (ruby0b).clone(), (ruby1).clone(), (ruby2).clone()]), Some(vec![]), Some(vec![]));
        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn paired_punctuation_with_zero_capacity() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.pairedPunctuationWithZeroCapacity", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.pairedPunctuationWithZeroCapacity", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"pairedPunctuationWithZeroCapacity");
        let mut engine = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).unwrap();
        let text = "（括号）".to_string();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (annotation).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn dynamic_shaping_emphasis_italic_at_and_zero_paired_capacity_branches() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.dynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranches", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.dynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranches", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"dynamicShapingEmphasisItalicAtAndZeroPairedCapacityBranches");
        let mut engine = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).unwrap();
        let text = "Hello World Latin".to_string();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 17u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 5u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(6u32, 11u32).unwrap(), DecorationKind::Emphasis)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut engine, (input).clone(),
WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).unwrap();
        let uncached_annotation = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_with_empty_shaping_cache((annotation).clone());
        let rejected: SortedMapTable<TextRange, SortedSetTable<u32>> = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_tier_map(TextRange::new(0u32, 17u32).unwrap(), &vec![ProgressiveBreakTier::Structural]);
        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut engine, (input).clone(), (uncached_annotation).clone(), (rejected).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { "ParagraphLayoutPrep@identity".to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn centered_punct_before_attached_reference_keeps_leading_glue_only() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.centeredPunctBeforeAttachedReferenceKeepsLeadingGlueOnly", "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTest.centeredPunctBeforeAttachedReferenceKeepsLeadingGlueOnly", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheCoverageTest");
        t.section(&"centeredPunctBeforeAttachedReferenceKeepsLeadingGlueOnly");
        let text = "正文：“内容·[1]，后文".to_string();
        let attach_at = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_index_of(text.as_str(), &"[1]");
        let result = WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, Some(Box::new(NarrowInkShaper::new()))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![
    (TextSpan::new(TextRange::new(attach_at, u32::wrapping_add(attach_at, 3)).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb),
None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut boundary: Option<SpacingDecisionInfo> = None;
        for i in 0..match u32::try_from((result.debug).clone().spacing_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((result.debug).clone().spacing_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if u32::from_ne_bytes((u_string::find_from(&(d.reason).to_string(), "AttachedInlineVirtualPunctuationBoundary", 0)).to_ne_bytes()) == 0 {
                boundary = Some(d.clone());
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualPunctuationBoundary:adjacent-punctuation", (boundary.as_ref().unwrap().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"·", (boundary.as_ref().unwrap().left_char).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"，", (boundary.as_ref().unwrap().right_char).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((boundary.as_ref().unwrap().natural_inner_glue) > (0.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((boundary.as_ref().unwrap().reduction) > (0.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_index_of(text.as_str(), &"·"), (boundary.as_ref().unwrap().reduction_target_range).clone().start,
None).unwrap();
    });
}
