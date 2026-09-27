#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::auto_space_decision_info::AutoSpaceDecisionInfo;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::cluster_geometry_decision_info::ClusterGeometryDecisionInfo;
use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::inline_object_preferred_stretch_kind::InlineObjectPreferredStretchKind;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::line_debug_info::LineDebugInfo;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::ruby_span::compare_ruby_span;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::baseline_class::BaselineClass;
use crate::org::tiqian::font::baseline_policy::BaselinePolicy;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metric_source::FontMetricSource;
use crate::org::tiqian::font::font_metrics::FontMetricsRequest;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::font_metrics_policy::FontMetricsPolicy;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::layout_font_metrics::LayoutFontMetrics;
use crate::org::tiqian::font::metric_box::MetricBox;
use crate::org::tiqian::font::raw_font_metrics::RawFontMetrics;
use crate::org::tiqian::layout::annotation_geometry_stage::RubyFontGeometry;
use crate::org::tiqian::layout::annotation_geometry_stage_coverage_test_support::InkBoundsTextShaper;
use crate::org::tiqian::layout::annotation_geometry_stage_coverage_test_support::MultiGlyphBoundsShaper;
use crate::org::tiqian::layout::annotation_geometry_stage_coverage_test_support::MultiGlyphMinMaxShaper;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_adjustment_stage::LineAdjustmentStage;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_geometry_stage::ClusterMetricDecision;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
}

impl From<AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::TextShaperShapeFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault) -> Self {
        match value {
            AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn inline_object_decisions_with_preferred_stretch_and_fixed() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.inlineObjectDecisionsWithPreferredStretchAndFixed", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.inlineObjectDecisionsWithPreferredStretchAndFixed", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"inlineObjectDecisionsWithPreferredStretchAndFixed");
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let text = "前置文本【嵌入对象】后置文本".to_string();
        let obj_with_stretch = InlineObjectSpan::new(TextRange::new(4u32, 5u32).unwrap(), 30.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(true), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64,
15.0f64).unwrap()), Some(0.0), Some(0.0), Some(true)).unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(true), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 10.0f64, 20.0f64).unwrap()), Some(3.0f64), Some(2.0f64),
Some(false)).unwrap())).unwrap();
        let obj_fixed = InlineObjectSpan::new(TextRange::new(6u32, 7u32).unwrap(), 20.0f64, 10.0f64, 2.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj_with_stretch).clone(), (obj_fixed).clone()]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(result.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn decoration_decisions_emphasis_on_han_punctuation_and_western() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationDecisionsEmphasisOnHanPunctuationAndWestern", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationDecisionsEmphasisOnHanPunctuationAndWestern", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"decorationDecisionsEmphasisOnHanPunctuationAndWestern");
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let _ = "汉字，。English".to_string();
        let input = LayoutInput::new(TiqianTextContent::new("汉字，。English", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(0.2f64))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![(DecorationSpan::new(TextRange::new(0u32, 11u32).unwrap(), DecorationKind::Emphasis)).clone()]), Some(vec![]), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(result.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn decoration_segments_mourning_proper_noun_book_title_and_shortening() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationSegmentsMourningProperNounBookTitleAndShortening", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationSegmentsMourningProperNounBookTitleAndShortening", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"decorationSegmentsMourningProperNounBookTitleAndShortening");
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let _ = "张三李四王五赵六钱七孙八周吴郑王".to_string();
        let input = LayoutInput::new(TiqianTextContent::new("张三李四王五赵六钱七孙八周吴郑王", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(120.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(2u32, 4u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(4u32, 8u32).unwrap(), DecorationKind::BookTitle)).clone(),
    (DecorationSpan::new(TextRange::new(8u32, 12u32).unwrap(), DecorationKind::Mourning)).clone(),
    (DecorationSpan::new(TextRange::new(0u32, 16u32).unwrap(), DecorationKind::Mourning)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(result.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn decoration_segments_leading_and_trailing_blanks() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationSegmentsLeadingAndTrailingBlanks", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationSegmentsLeadingAndTrailingBlanks", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"decorationSegmentsLeadingAndTrailingBlanks");
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let text = "「开头」中文 English 混排【结束】".to_string();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(150.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, u_string::count(text.as_str())).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(result.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn ruby_decisions_pinyin_single_and_split_lines() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.rubyDecisionsPinyinSingleAndSplitLines", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.rubyDecisionsPinyinSingleAndSplitLines", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"rubyDecisionsPinyinSingleAndSplitLines");
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let text = "这是一个很长很长的段落用于测试拼音行间注跨行".to_string();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), "zhèshì", Some(vec![]), RubyKind::Pinyin, Some("zh-Latn".to_string()))).clone(),
    (RubySpan::new(TextRange::new(2u32, 6u32).unwrap(), "yīgehěncháng", Some(vec![]), RubyKind::Pinyin, None)).clone(),
    (RubySpan::new(TextRange::new(6u32, 12u32).unwrap(), "chángdeduànluò", Some(vec![]), RubyKind::Pinyin, None)).clone(),
]), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(result.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn bopomofo_decisions_all_tones_and_symbol_counts() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoDecisionsAllTonesAndSymbolCounts", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoDecisionsAllTonesAndSymbolCounts", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"bopomofoDecisionsAllTonesAndSymbolCounts");
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(InkBoundsTextShaper::new(Some(ExplainableStubTextShaper::new())))),
Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let text = "一二三四五六七八九十甲乙丙丁戊己庚辛".to_string();
        let ruby_spans = vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "˙ㄅ", Some(vec![]), RubyKind::Bopomofo, Some("zh-Bopo".to_string()))).clone(),
    (RubySpan::new(TextRange::new(1u32, 2u32).unwrap(), "˙ㄅㄆ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(2u32, 3u32).unwrap(), "˙ㄅㄆㄇ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(3u32, 4u32).unwrap(), "ㄅˊ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(4u32, 5u32).unwrap(), "ㄅㄆˊ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(5u32, 6u32).unwrap(), "ㄅㄆㄇˊ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(6u32, 7u32).unwrap(), "ㄅˇ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(7u32, 8u32).unwrap(), "ㄅㄆˇ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(8u32, 9u32).unwrap(), "ㄅㄆㄇˇ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(9u32, 10u32).unwrap(), "ㄅˋ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(10u32, 11u32).unwrap(), "ㄅㄆˋ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(11u32, 12u32).unwrap(), "ㄅㄆㄇˋ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(12u32, 13u32).unwrap(), "ㄅ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(13u32, 14u32).unwrap(), "ㄅㄆ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(14u32, 15u32).unwrap(), "ㄅㄆㄇ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
];
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((ruby_spans).clone()), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(result.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn direct_resolve_annotation_geometry_fallback_branches() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.directResolveAnnotationGeometryFallbackBranches", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.directResolveAnnotationGeometryFallbackBranches", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"directResolveAnnotationGeometryFallbackBranches");
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let text = "汉字，测试English".to_string();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(2u32, 3u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(5u32, 12u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(0u32, 4u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), "汉字", "k", 32.0f64, Some("汉字".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "，", "k", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 5u32).unwrap(), "测试", "k", 32.0f64, Some("测试".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(5u32, 12u32).unwrap(), "English", "k", 56.0f64, Some("English".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let line_boxes = vec![
    (LineBox::new(TextRange::new(0u32, 5u32).unwrap(), IntRange::new(0u32, 2u32), 16.0f64, 0.0f64, 20.0f64, 80.0f64, 80.0f64, 80.0f64, Some(0.0), Some(0.0f64), Some(LineEndReason::AutoWrap), Some(0.0), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])))).clone(),
    (LineBox::new(TextRange::new(5u32, 12u32).unwrap(), IntRange::new(3u32, 3u32), 36.0f64, 20.0f64, 40.0f64, 56.0f64, 56.0f64, 56.0f64, Some(0.0), Some(0.0f64), Some(LineEndReason::MandatoryBreak), Some(0.0), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])))).clone(),

];
        let line_solution = LineSolution::new(Some(vec![
    (LineCandidate::new(IntRange::new(0u32, 2u32), TextRange::new(0u32, 5u32).unwrap(), 80.0f64, 80.0f64, Some(LineEndReason::AutoWrap), None, Some(vec![]), Some(LineCandidate::line_candidate_empty_hanging())).unwrap()).clone(),
    (LineCandidate::new(IntRange::new(3u32, 3u32), TextRange::new(5u32, 12u32).unwrap(), 56.0f64, 56.0f64, Some(LineEndReason::MandatoryBreak), None, Some(vec![]), Some(LineCandidate::line_candidate_empty_hanging())).unwrap()).clone(),
]), Some(0 as f64)).unwrap();
        let clreq_profile = engine.clreq_profile_resolver.resolve((input.profile_id).clone());
        let inline_object1 = InlineObjectSpan::new(TextRange::new(0u32, 2u32).unwrap(), 32.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(false), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64,
15.0f64).unwrap()), Some(0.0), Some(0.0), Some(false)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let inline_object2 = InlineObjectSpan::new(TextRange::new(5u32, 12u32).unwrap(), 56.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0), Some(false)).unwrap())).unwrap();
        let inline_object_not_in_line = InlineObjectSpan::new(TextRange::new(99u32, 100u32).unwrap(), 10.0f64, 8.0f64, 2.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let geom_decision = ClusterGeometryDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), "汉字", "汉字", 32.0f64, 32.0f64, 4.0f64, 2.0f64, 4.0f64, 2.0f64, 0.0f64, 32.0f64, "test", "test", Some(0.0), Some(0.0), None);
        let mut inline_obj_map_builder: SortedMapTableBuilder<u32, InlineObjectSpan> = SortedTable::sorted_table_map_builder::<u32, InlineObjectSpan>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        inline_obj_map_builder.put(&(0), &(inline_object1));
        inline_obj_map_builder.put(&(3), &(inline_object2));
        inline_obj_map_builder.put(&(99), &(inline_object_not_in_line));
        let inline_obj_map: SortedMapTable<u32, InlineObjectSpan> = inline_obj_map_builder.clone().build();
        let mut justify_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        justify_builder.put(&(0), &(2.0f64));
        let justify_map: SortedMapTable<u32, f64> = justify_builder.clone().build();
        let mut spread_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        spread_builder.put(&(0), &(4.0f64));
        let spread_map: SortedMapTable<u32, f64> = spread_builder.clone().build();
        let pinyin_a = RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), "hànzì", Some(vec![]), RubyKind::Pinyin, Some("zh-Latn".to_string()));
        let pinyin_b = RubySpan::new(TextRange::new(3u32, 5u32).unwrap(), "cèshì", Some(vec![]), RubyKind::Pinyin, None);
        let mut rfg_builder: SortedMapTableBuilder<RubySpan, RubyFontGeometry> = SortedTable::sorted_table_map_builder::<RubySpan, RubyFontGeometry>(Arc::new(compare_ruby_span));
        rfg_builder.put(&(pinyin_a), &(RubyFontGeometry::new(20.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        rfg_builder.put(&(pinyin_b), &(RubyFontGeometry::new(20.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        let rfg_map: SortedMapTable<RubySpan, RubyFontGeometry> = rfg_builder.clone().build();
        let res1 = LineAdjustmentStage::line_adjustment_stage_resolve_annotation_geometry(&mut engine, (input).clone(), 16.0f64, (inline_obj_map).clone(), (line_solution).clone(), (clreq_profile).clone(), &vec![(geom_decision).clone()], &vec![
    (AutoSpaceDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), "leading", "Wide", "Normal", 1u32, 0.0f64, 0.0f64, "test")).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(2u32, 3u32).unwrap(), "leading", "Wide", "Normal", 1u32, 0.0f64, 0.0f64, "test")).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(3u32, 5u32).unwrap(), "trailing", "Wide", "Normal", 1u32, 0.0f64, 0.0f64, "test")).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(5u32, 12u32).unwrap(), "trailing", "Wide", "Normal", 1u32, 0.0f64, 0.0f64, "test")).clone(),
], &vec![(IntRange::new(0u32, 2u32)).clone(), (IntRange::new(3u32, 3u32)).clone()], &line_boxes, &clusters, &vec![FontRole::CjkText, FontRole::CjkPunctuation, FontRole::CjkText, FontRole::LatinText], (justify_map).clone(), (spread_map).clone(), &vec![], &vec![(pinyin_a).clone(),
(pinyin_b).clone()], &clusters, (rfg_map).clone(), 0.0f64, 16.0f64, 8.0f64, 400, 4.0f64, {  Arc::new(move |__| {
        return 400;
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(res1.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::try_from((res1.inline_object_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(4294967295u32, res1.inline_object_decisions[usize::try_from(u32::wrapping_sub(u32::try_from((res1.inline_object_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].line_index, None).unwrap();
        let metric_decision = ClusterMetricDecision::new(TextRange::new(0u32, 2u32).unwrap(), "汉字", FontMetricsRequest::new("k", 16.0f64, FontRole::CjkText, "zh-Hans", Some(vec![]), Some(400), Some(false), Some("".to_string())), RawFontMetrics::new(14.0f64, 4.0f64, Some(0 as f64),
Some(FontMetricSource::RawTables), None, None), LayoutFontMetrics::new(14.0f64, 4.0f64, 0.0f64, FontMetricsPolicy::Raw, BaselinePolicy::Alphabetic, Some(BaselineClass::Roman), Some(MetricBox::RawFontBox), Some(FontMetricSource::RawTables), Some("".to_string())));
        let pinyin_c = RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), "hànzì", Some(vec![]), RubyKind::Pinyin, None);
        let mut rfg_builder2: SortedMapTableBuilder<RubySpan, RubyFontGeometry> = SortedTable::sorted_table_map_builder::<RubySpan, RubyFontGeometry>(Arc::new(compare_ruby_span));
        rfg_builder2.put(&(pinyin_c), &(RubyFontGeometry::new(20.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        let rfg_map2: SortedMapTable<RubySpan, RubyFontGeometry> = rfg_builder2.clone().build();
        let res2 = LineAdjustmentStage::line_adjustment_stage_resolve_annotation_geometry(&mut engine, (input).clone(), 16.0f64, SortedTable::sorted_table_map_builder::<u32, InlineObjectSpan>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), (line_solution).clone(), (clreq_profile).clone(), &vec![], &vec![], &vec![(IntRange::new(0u32, 2u32)).clone(), (IntRange::new(3u32,
3u32)).clone()], &line_boxes, &clusters, &vec![FontRole::CjkText, FontRole::CjkPunctuation, FontRole::CjkText, FontRole::LatinText], SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(),
&vec![(metric_decision).clone()], &vec![(pinyin_c).clone()], &clusters, (rfg_map2).clone(), 0.0f64, 16.0f64, 8.0f64, 400, 4.0f64, {  Arc::new(move |__| {
        return 400;
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(res2.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn bopomofo_decisions_multi_glyph_min_max_and_empty_placements() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacements", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacements", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"bopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacements");
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(MultiGlyphMinMaxShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let text = "一二三四五六七八".to_string();
        let ruby_spans = vec![
    (RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), "ㄅㄆˊ", Some(vec![]), RubyKind::Bopomofo, Some("zh-Bopo".to_string()))).clone(),
    (RubySpan::new(TextRange::new(2u32, 3u32).unwrap(), " ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(3u32, 4u32).unwrap(), "ㄅ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(4u32, 5u32).unwrap(), "˙ㄅ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(5u32, 6u32).unwrap(), "ㄅˇ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(6u32, 7u32).unwrap(), "ㄅˋ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
];
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((ruby_spans).clone()), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(result.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn direct_resolve_annotation_geometry_empty_line_ranges_and_gap_at_line_edges() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.directResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdges", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.directResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdges", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"directResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdges");
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let text = "汉字，测试English".to_string();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(0u32, 5u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), "汉字", "k", 32.0f64, Some("汉字".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "，", "k", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 5u32).unwrap(), "测试", "k", 32.0f64, Some("测试".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(5u32, 12u32).unwrap(), "English", "k", 56.0f64, Some("English".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let line_boxes = vec![
    (LineBox::new(TextRange::new(0u32, 0u32).unwrap(), IntRange::new(1u32, 0u32), 0.0f64, 0.0f64, 20.0f64, 0.0f64, 0.0f64, 0.0f64, Some(0.0), Some(0.0f64), Some(LineEndReason::AutoWrap), Some(0.0), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])))).clone(),
    (LineBox::new(TextRange::new(0u32, 5u32).unwrap(), IntRange::new(0u32, 2u32), 16.0f64, 0.0f64, 20.0f64, 80.0f64, 80.0f64, 80.0f64, Some(0.0), Some(0.0f64), Some(LineEndReason::AutoWrap), Some(0.0), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])))).clone(),
];
        let line_solution = LineSolution::new(Some(vec![
    (LineCandidate::new(IntRange::new(1u32, 0u32), TextRange::new(0u32, 0u32).unwrap(), 0.0f64, 0.0f64, Some(LineEndReason::AutoWrap), None, Some(vec![]), Some(LineCandidate::line_candidate_empty_hanging())).unwrap()).clone(),
    (LineCandidate::new(IntRange::new(0u32, 2u32), TextRange::new(0u32, 5u32).unwrap(), 80.0f64, 80.0f64, Some(LineEndReason::AutoWrap), None, Some(vec![]), Some(LineCandidate::line_candidate_empty_hanging())).unwrap()).clone(),
]), Some(0 as f64)).unwrap();
        let clreq_profile = engine.clreq_profile_resolver.resolve((input.profile_id).clone());
        let geom_decision = ClusterGeometryDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), "汉字", "汉字", 32.0f64, 32.0f64, 4.0f64, 2.0f64, 4.0f64, 2.0f64, 0.0f64, 32.0f64, "test", "test", Some(0.0), Some(0.0), None);
        let metric_decision1 = ClusterMetricDecision::new(TextRange::new(0u32, 2u32).unwrap(), "汉字", FontMetricsRequest::new("k", 24.0f64, FontRole::CjkText, "zh-Hans", Some(vec![]), Some(400), Some(false), Some("".to_string())), RawFontMetrics::new(18.0f64, 6.0f64, Some(0 as
f64), Some(FontMetricSource::RawTables), None, None), LayoutFontMetrics::new(18.0f64, 6.0f64, 0.0f64, FontMetricsPolicy::Raw, BaselinePolicy::Alphabetic, Some(BaselineClass::Roman), Some(MetricBox::RawFontBox), Some(FontMetricSource::RawTables), Some("".to_string())));
        let metric_decision2 = ClusterMetricDecision::new(TextRange::new(2u32, 3u32).unwrap(), "，", FontMetricsRequest::new("k", 24.0f64, FontRole::CjkPunctuation, "zh-Hans", Some(vec![]), Some(400), Some(false), Some("".to_string())), RawFontMetrics::new(18.0f64, 6.0f64, Some(0
as f64), Some(FontMetricSource::RawTables), None, None), LayoutFontMetrics::new(18.0f64, 6.0f64, 0.0f64, FontMetricsPolicy::Raw, BaselinePolicy::Alphabetic, Some(BaselineClass::Roman), Some(MetricBox::RawFontBox), Some(FontMetricSource::RawTables), Some("".to_string())));
        let inline_object1 = InlineObjectSpan::new(TextRange::new(0u32, 2u32).unwrap(), 32.0f64, 16.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(false), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 5.0f64, 10.0f64).unwrap()),
Some(0.0), Some(0.0), Some(false)).unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(false)).unwrap())).unwrap();
        let inline_object2 = InlineObjectSpan::new(TextRange::new(15u32, 17u32).unwrap(), 32.0f64, 16.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(false)).unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false),
Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::BinaryOperator, 5.0f64, 10.0f64).unwrap()), Some(0.0), Some(0.0), Some(false)).unwrap())).unwrap();
        let mut inline_obj_map_builder: SortedMapTableBuilder<u32, InlineObjectSpan> = SortedTable::sorted_table_map_builder::<u32, InlineObjectSpan>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        inline_obj_map_builder.put(&(0), &(inline_object1));
        inline_obj_map_builder.put(&(99), &(inline_object2));
        let inline_obj_map: SortedMapTable<u32, InlineObjectSpan> = inline_obj_map_builder.clone().build();
        let pinyin_a = RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), "hànzì", Some(vec![]), RubyKind::Pinyin, None);
        let pinyin_b = RubySpan::new(TextRange::new(2u32, 3u32).unwrap(), "chù", Some(vec![]), RubyKind::Pinyin, None);
        let pinyin_c = RubySpan::new(TextRange::new(3u32, 5u32).unwrap(), "cèshì", Some(vec![]), RubyKind::Pinyin, None);
        let mut rfg_builder: SortedMapTableBuilder<RubySpan, RubyFontGeometry> = SortedTable::sorted_table_map_builder::<RubySpan, RubyFontGeometry>(Arc::new(compare_ruby_span));
        rfg_builder.put(&(pinyin_a), &(RubyFontGeometry::new(20.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        rfg_builder.put(&(pinyin_b), &(RubyFontGeometry::new(10.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        rfg_builder.put(&(pinyin_c), &(RubyFontGeometry::new(20.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        let rfg_map: SortedMapTable<RubySpan, RubyFontGeometry> = rfg_builder.clone().build();
        let res = LineAdjustmentStage::line_adjustment_stage_resolve_annotation_geometry(&mut engine, (input).clone(), 16.0f64, (inline_obj_map).clone(), (line_solution).clone(), (clreq_profile).clone(), &vec![(geom_decision).clone()], &vec![
    (AutoSpaceDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), "leading", "Wide", "Normal", 1u32, 0.0f64, 0.0f64, "test")).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(2u32, 3u32).unwrap(), "leading", "Wide", "Normal", 1u32, 0.0f64, 0.0f64, "test")).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(2u32, 3u32).unwrap(), "trailing", "Wide", "Normal", 1u32, 0.0f64, 0.0f64, "test")).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(3u32, 5u32).unwrap(), "trailing", "Wide", "Normal", 1u32, 0.0f64, 0.0f64, "test")).clone(),
], &vec![(IntRange::new(1u32, 0u32)).clone(), (IntRange::new(0u32, 2u32)).clone()], &line_boxes, &clusters, &vec![FontRole::CjkText, FontRole::CjkPunctuation, FontRole::CjkText, FontRole::LatinText], SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![(metric_decision1).clone(), (metric_decision2).clone()], &vec![(pinyin_a).clone(), (pinyin_b).clone(), (pinyin_c).clone()], &clusters,
(rfg_map).clone(), 0.0f64, 16.0f64, 8.0f64, 400, 4.0f64, {  Arc::new(move |__| {
        return 400;
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(res.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn bopomofo_and_decoration_leading_blank_exhaustive_branches() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoAndDecorationLeadingBlankExhaustiveBranches", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoAndDecorationLeadingBlankExhaustiveBranches", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"bopomofoAndDecorationLeadingBlankExhaustiveBranches");
        let multi_glyph_shaper = MultiGlyphBoundsShaper::new();
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new((multi_glyph_shaper).clone())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let text = "中文English".to_string();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(500.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 7u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(2u32, 7u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "ㄅ", Some(vec![]), RubyKind::Bopomofo, None)).clone(),
    (RubySpan::new(TextRange::new(1u32, 2u32).unwrap(), "ㄆ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
]), Some(vec![]), Some(vec![]));
        let res = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(res.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
        let input_narrow = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(30.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 7u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(2u32, 7u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "ㄅ", Some(vec![]), RubyKind::Bopomofo, None)).clone(),
    (RubySpan::new(TextRange::new(1u32, 2u32).unwrap(), "ㄆ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
]), Some(vec![]), Some(vec![]));
        let res_narrow = engine.layout((input_narrow).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(res_narrow.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn bopomofo_over_latin_clusters_covers_cross_metric_lookup() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoOverLatinClustersCoversCrossMetricLookup", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoOverLatinClustersCoversCrossMetricLookup", || {
        let mut r = TestTraceRecorder::new("AnnotationGeometryStageCoverageTest");
        r.section(&"bopomofoOverLatinClustersCoversCrossMetricLookup");
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let text = "中文English".to_string();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(500.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(2u32, 3u32).unwrap(), "ㄅ", Some(vec![]), RubyKind::Bopomofo, None)).clone(),
    (RubySpan::new(TextRange::new(3u32, 4u32).unwrap(), "ㄆ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
]), Some(vec![]), Some(vec![]));
        let res = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "null".to_string() } else { TestTraceRender::test_trace_render_cap(res.to_string().as_str()).unwrap().to_string() }.as_str(), None).unwrap();
    });
}
