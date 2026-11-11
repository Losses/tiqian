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
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestRubyDecisionsPinyinSingleAndSplitLinesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestInlineObjectDecisionsWithPreferredStretchAndFixedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryFallbackBranchesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDirectResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdgesFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDecorationSegmentsMourningProperNounBookTitleAndShorteningFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDecorationSegmentsLeadingAndTrailingBlanksFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestDecorationDecisionsEmphasisOnHanPunctuationAndWesternFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoOverLatinClustersCoversCrossMetricLookupFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacementsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoDecisionsAllTonesAndSymbolCountsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AnnotationGeometryStageCoverageTestBopomofoAndDecorationLeadingBlankExhaustiveBranchesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,68,101,99,105,115,105,111,110,115,87,105,116,104,80,114,101,102,101,114,114,101,100,83,116,114,101,116,99,104,65,110,100,70,105,120,101,100]));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let text = UString::from("前置文本【嵌入对象】后置文本").to_ustring();
        let obj_with_stretch = InlineObjectSpan::new(TextRange::new(4u32, 5u32).unwrap(), 30.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(true), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(0.0), Some(0.0), Some(true)).unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(true), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 10.0f64, 20.0f64).unwrap()), Some(3.0f64), Some(2.0f64), Some(false)).unwrap())).unwrap();
        let obj_fixed = InlineObjectSpan::new(TextRange::new(6u32, 7u32).unwrap(), 20.0f64, 10.0f64, 2.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(obj_with_stretch).clone(), (obj_fixed).clone()]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", result.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn decoration_decisions_emphasis_on_han_punctuation_and_western() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationDecisionsEmphasisOnHanPunctuationAndWestern", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationDecisionsEmphasisOnHanPunctuationAndWestern", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[100,101,99,111,114,97,116,105,111,110,68,101,99,105,115,105,111,110,115,69,109,112,104,97,115,105,115,79,110,72,97,110,80,117,110,99,116,117,97,116,105,111,110,65,110,100,87,101,115,116,101,114,110]));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let _ = UString::from("汉字，。English").to_ustring();
        let input = LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[27721,23383,65292,12290,69,110,103,108,105,115,104])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(0.2f64))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![(DecorationSpan::new(TextRange::new(0u32, 11u32).unwrap(), DecorationKind::Emphasis)).clone()]), Some(vec![]), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", result.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
    });
}

#[test]
fn decoration_segments_mourning_proper_noun_book_title_and_shortening() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationSegmentsMourningProperNounBookTitleAndShortening", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationSegmentsMourningProperNounBookTitleAndShortening", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[100,101,99,111,114,97,116,105,111,110,83,101,103,109,101,110,116,115,77,111,117,114,110,105,110,103,80,114,111,112,101,114,78,111,117,110,66,111,111,107,84,105,116,108,101,65,110,100,83,104,111,114,116,101,110,105,110,103]));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let _ = UString::from("张三李四王五赵六钱七孙八周吴郑王").to_ustring();
        let input = LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[24352,19977,26446,22235,29579,20116,36213,20845,38065,19971,23385,20843,21608,21556,37073,29579])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(120.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(2u32, 4u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(4u32, 8u32).unwrap(), DecorationKind::BookTitle)).clone(),
    (DecorationSpan::new(TextRange::new(8u32, 12u32).unwrap(), DecorationKind::Mourning)).clone(),
    (DecorationSpan::new(TextRange::new(0u32, 16u32).unwrap(), DecorationKind::Mourning)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", result.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
    });
}

#[test]
fn decoration_segments_leading_and_trailing_blanks() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationSegmentsLeadingAndTrailingBlanks", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.decorationSegmentsLeadingAndTrailingBlanks", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[100,101,99,111,114,97,116,105,111,110,83,101,103,109,101,110,116,115,76,101,97,100,105,110,103,65,110,100,84,114,97,105,108,105,110,103,66,108,97,110,107,115]));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let text = UString::from("「开头」中文 English 混排【结束】").to_ustring();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(150.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, u_string::count(text.as_ustr())).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", result.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
    });
}

#[test]
fn ruby_decisions_pinyin_single_and_split_lines() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.rubyDecisionsPinyinSingleAndSplitLines", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.rubyDecisionsPinyinSingleAndSplitLines", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[114,117,98,121,68,101,99,105,115,105,111,110,115,80,105,110,121,105,110,83,105,110,103,108,101,65,110,100,83,112,108,105,116,76,105,110,101,115]));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let text = UString::from("这是一个很长很长的段落用于测试拼音行间注跨行").to_ustring();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[122,104,232,115,104,236])), Some(vec![]), RubyKind::Pinyin, Some(UString::from("zh-Latn")))).clone(),
    (RubySpan::new(TextRange::new(2u32, 6u32).unwrap(), &(UStr::new(&[121,299,103,101,104,283,110,99,104,225,110,103])), Some(vec![]), RubyKind::Pinyin, None)).clone(),
    (RubySpan::new(TextRange::new(6u32, 12u32).unwrap(), &(UStr::new(&[99,104,225,110,103,100,101,100,117,224,110,108,117,242])), Some(vec![]), RubyKind::Pinyin, None)).clone(),
]), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", result.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
    });
}

#[test]
fn bopomofo_decisions_all_tones_and_symbol_counts() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoDecisionsAllTonesAndSymbolCounts", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoDecisionsAllTonesAndSymbolCounts", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[98,111,112,111,109,111,102,111,68,101,99,105,115,105,111,110,115,65,108,108,84,111,110,101,115,65,110,100,83,121,109,98,111,108,67,111,117,110,116,115]));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(InkBoundsTextShaper::new(Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))))))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let text = UString::from("一二三四五六七八九十甲乙丙丁戊己庚辛").to_ustring();
        let ruby_spans = vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[729,12549])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-Bopo")))).clone(),
    (RubySpan::new(TextRange::new(1u32, 2u32).unwrap(), &(UStr::new(&[729,12549,12550])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[729,12549,12550,12551])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(3u32, 4u32).unwrap(), &(UStr::new(&[12549,714])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(4u32, 5u32).unwrap(), &(UStr::new(&[12549,12550,714])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(5u32, 6u32).unwrap(), &(UStr::new(&[12549,12550,12551,714])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(6u32, 7u32).unwrap(), &(UStr::new(&[12549,711])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(7u32, 8u32).unwrap(), &(UStr::new(&[12549,12550,711])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(8u32, 9u32).unwrap(), &(UStr::new(&[12549,12550,12551,711])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(9u32, 10u32).unwrap(), &(UStr::new(&[12549,715])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(10u32, 11u32).unwrap(), &(UStr::new(&[12549,12550,715])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(11u32, 12u32).unwrap(), &(UStr::new(&[12549,12550,12551,715])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(12u32, 13u32).unwrap(), &(UStr::new(&[12549])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(13u32, 14u32).unwrap(), &(UStr::new(&[12549,12550])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(14u32, 15u32).unwrap(), &(UStr::new(&[12549,12550,12551])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
];
        let input = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((ruby_spans).clone()), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", result.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
    });
}

#[test]
fn direct_resolve_annotation_geometry_fallback_branches() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.directResolveAnnotationGeometryFallbackBranches", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.directResolveAnnotationGeometryFallbackBranches", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[100,105,114,101,99,116,82,101,115,111,108,118,101,65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,70,97,108,108,98,97,99,107,66,114,97,110,99,104,101,115]));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let text = UString::from("汉字，测试English").to_ustring();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(2u32, 3u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(5u32, 12u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(0u32, 4u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[27721,23383])), &(UStr::new(&[107])), 32.0f64, Some(UString::from("汉字")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[65292])), &(UStr::new(&[107])), 16.0f64, Some(UString::from("，")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 5u32).unwrap(), &(UStr::new(&[27979,35797])), &(UStr::new(&[107])), 32.0f64, Some(UString::from("测试")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(5u32, 12u32).unwrap(), &(UStr::new(&[69,110,103,108,105,115,104])), &(UStr::new(&[107])), 56.0f64, Some(UString::from("English")), Some(0.0), Some(0.0), Some(0.0))).clone(),
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
        let inline_object1 = InlineObjectSpan::new(TextRange::new(0u32, 2u32).unwrap(), 32.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(false), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 15.0f64).unwrap()), Some(0.0), Some(0.0), Some(false)).unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let inline_object2 = InlineObjectSpan::new(TextRange::new(5u32, 12u32).unwrap(), 56.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 10.0f64, 20.0f64).unwrap()), Some(0.0), Some(0.0), Some(false)).unwrap())).unwrap();
        let inline_object_not_in_line = InlineObjectSpan::new(TextRange::new(99u32, 100u32).unwrap(), 10.0f64, 8.0f64, 2.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let geom_decision = ClusterGeometryDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[27721,23383])), &(UStr::new(&[27721,23383])), 32.0f64, 32.0f64, 4.0f64, 2.0f64, 4.0f64, 2.0f64, 0.0f64, 32.0f64, &(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])), Some(0.0), Some(0.0), None);
        let mut inline_obj_map_builder: SortedMapTableBuilder<u32, InlineObjectSpan> = SortedTable::sorted_table_map_builder::<u32,
InlineObjectSpan>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        inline_obj_map_builder.put(&(0), &(inline_object1));
        inline_obj_map_builder.put(&(3), &(inline_object2));
        inline_obj_map_builder.put(&(99), &(inline_object_not_in_line));
        let inline_obj_map: SortedMapTable<u32, InlineObjectSpan> = inline_obj_map_builder.clone().build();
        let mut justify_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        justify_builder.put(&(0), &(2.0f64));
        let justify_map: SortedMapTable<u32, f64> = justify_builder.clone().build();
        let mut spread_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        spread_builder.put(&(0), &(4.0f64));
        let spread_map: SortedMapTable<u32, f64> = spread_builder.clone().build();
        let pinyin_a = RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[104,224,110,122,236])), Some(vec![]), RubyKind::Pinyin, Some(UString::from("zh-Latn")));
        let pinyin_b = RubySpan::new(TextRange::new(3u32, 5u32).unwrap(), &(UStr::new(&[99,232,115,104,236])), Some(vec![]), RubyKind::Pinyin, None);
        let mut rfg_builder: SortedMapTableBuilder<RubySpan, RubyFontGeometry> = SortedTable::sorted_table_map_builder::<RubySpan, RubyFontGeometry>(Arc::new(compare_ruby_span));
        rfg_builder.put(&(pinyin_a), &(RubyFontGeometry::new(20.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        rfg_builder.put(&(pinyin_b), &(RubyFontGeometry::new(20.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        let rfg_map: SortedMapTable<RubySpan, RubyFontGeometry> = rfg_builder.clone().build();
        let res1 = LineAdjustmentStage::line_adjustment_stage_resolve_annotation_geometry(&mut engine, (input).clone(), 16.0f64, (inline_obj_map).clone(), (line_solution).clone(), (clreq_profile).clone(), &vec![(geom_decision).clone()], &vec![
    (AutoSpaceDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[108,101,97,100,105,110,103])), &(UStr::new(&[87,105,100,101])), &(UStr::new(&[78,111,114,109,97,108])), 1u32, 0.0f64, 0.0f64, &(UStr::new(&[116,101,115,116])))).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[108,101,97,100,105,110,103])), &(UStr::new(&[87,105,100,101])), &(UStr::new(&[78,111,114,109,97,108])), 1u32, 0.0f64, 0.0f64, &(UStr::new(&[116,101,115,116])))).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(3u32, 5u32).unwrap(), &(UStr::new(&[116,114,97,105,108,105,110,103])), &(UStr::new(&[87,105,100,101])), &(UStr::new(&[78,111,114,109,97,108])), 1u32, 0.0f64, 0.0f64, &(UStr::new(&[116,101,115,116])))).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(5u32, 12u32).unwrap(), &(UStr::new(&[116,114,97,105,108,105,110,103])), &(UStr::new(&[87,105,100,101])), &(UStr::new(&[78,111,114,109,97,108])), 1u32, 0.0f64, 0.0f64, &(UStr::new(&[116,101,115,116])))).clone(),
], &vec![(IntRange::new(0u32, 2u32)).clone(), (IntRange::new(3u32, 3u32)).clone()], &line_boxes, &clusters, &vec![FontRole::CjkText, FontRole::CjkPunctuation, FontRole::CjkText, FontRole::LatinText], (justify_map).clone(), (spread_map).clone(), &vec![],
&vec![(pinyin_a).clone(), (pinyin_b).clone()], &clusters, (rfg_map).clone(), 0.0f64, 16.0f64, 8.0f64, 400, 4.0f64, {  Arc::new(move |__| {
        return 400;
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", res1.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::try_from((res1.inline_object_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(4294967295u32, res1.inline_object_decisions[usize::try_from(u32::wrapping_sub(u32::try_from((res1.inline_object_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].line_index, None).unwrap();
        let metric_decision = ClusterMetricDecision::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[27721,23383])), FontMetricsRequest::new(&(UStr::new(&[107])), 16.0f64, FontRole::CjkText, &(UStr::new(&[122,104,45,72,97,110,115])), Some(vec![]), Some(400), Some(false), Some(UString::from(""))), RawFontMetrics::new(14.0f64, 4.0f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None), LayoutFontMetrics::new(14.0f64, 4.0f64, 0.0f64, FontMetricsPolicy::Raw, BaselinePolicy::Alphabetic, Some(BaselineClass::Roman), Some(MetricBox::RawFontBox), Some(FontMetricSource::RawTables), Some(UString::from(""))));
        let pinyin_c = RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[104,224,110,122,236])), Some(vec![]), RubyKind::Pinyin, None);
        let mut rfg_builder2: SortedMapTableBuilder<RubySpan, RubyFontGeometry> = SortedTable::sorted_table_map_builder::<RubySpan, RubyFontGeometry>(Arc::new(compare_ruby_span));
        rfg_builder2.put(&(pinyin_c), &(RubyFontGeometry::new(20.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        let rfg_map2: SortedMapTable<RubySpan, RubyFontGeometry> = rfg_builder2.clone().build();
        let res2 = LineAdjustmentStage::line_adjustment_stage_resolve_annotation_geometry(&mut engine, (input).clone(), 16.0f64, SortedTable::sorted_table_map_builder::<u32, InlineObjectSpan>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(), (line_solution).clone(), (clreq_profile).clone(), &vec![], &vec![], &vec![(IntRange::new(0u32, 2u32)).clone(), (IntRange::new(3u32, 3u32)).clone()], &line_boxes, &clusters, &vec![FontRole::CjkText, FontRole::CjkPunctuation, FontRole::CjkText, FontRole::LatinText], SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(), SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(), &vec![(metric_decision).clone()], &vec![(pinyin_c).clone()], &clusters, (rfg_map2).clone(), 0.0f64, 16.0f64, 8.0f64, 400, 4.0f64, {  Arc::new(move |__| {
        return 400;
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", res2.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
    });
}

#[test]
fn bopomofo_decisions_multi_glyph_min_max_and_empty_placements() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacements", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoDecisionsMultiGlyphMinMaxAndEmptyPlacements", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[98,111,112,111,109,111,102,111,68,101,99,105,115,105,111,110,115,77,117,108,116,105,71,108,121,112,104,77,105,110,77,97,120,65,110,100,69,109,112,116,121,80,108,97,99,101,109,101,110,116,115]));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(MultiGlyphMinMaxShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let text = UString::from("一二三四五六七八").to_ustring();
        let ruby_spans = vec![
    (RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[12549,12550,714])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-Bopo")))).clone(),
    (RubySpan::new(TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[32])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(3u32, 4u32).unwrap(), &(UStr::new(&[12549])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(4u32, 5u32).unwrap(), &(UStr::new(&[729,12549])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(5u32, 6u32).unwrap(), &(UStr::new(&[12549,711])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(6u32, 7u32).unwrap(), &(UStr::new(&[12549,715])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
];
        let input = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some((ruby_spans).clone()), Some(vec![]), Some(vec![]));
        let result = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", result.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
    });
}

#[test]
fn direct_resolve_annotation_geometry_empty_line_ranges_and_gap_at_line_edges() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.directResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdges", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.directResolveAnnotationGeometryEmptyLineRangesAndGapAtLineEdges", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[100,105,114,101,99,116,82,101,115,111,108,118,101,65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,69,109,112,116,121,76,105,110,101,82,97,110,103,101,115,65,110,100,71,97,112,65,116,76,105,110,101,69,100,103,101,115]));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let text = UString::from("汉字，测试English").to_ustring();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new(TextRange::new(0u32, 5u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[27721,23383])), &(UStr::new(&[107])), 32.0f64, Some(UString::from("汉字")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[65292])), &(UStr::new(&[107])), 16.0f64, Some(UString::from("，")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 5u32).unwrap(), &(UStr::new(&[27979,35797])), &(UStr::new(&[107])), 32.0f64, Some(UString::from("测试")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(5u32, 12u32).unwrap(), &(UStr::new(&[69,110,103,108,105,115,104])), &(UStr::new(&[107])), 56.0f64, Some(UString::from("English")), Some(0.0), Some(0.0), Some(0.0))).clone(),
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
        let geom_decision = ClusterGeometryDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[27721,23383])), &(UStr::new(&[27721,23383])), 32.0f64, 32.0f64, 4.0f64, 2.0f64, 4.0f64, 2.0f64, 0.0f64, 32.0f64, &(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])), Some(0.0), Some(0.0), None);
        let metric_decision1 = ClusterMetricDecision::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[27721,23383])), FontMetricsRequest::new(&(UStr::new(&[107])), 24.0f64, FontRole::CjkText, &(UStr::new(&[122,104,45,72,97,110,115])), Some(vec![]), Some(400), Some(false), Some(UString::from(""))), RawFontMetrics::new(18.0f64, 6.0f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None), LayoutFontMetrics::new(18.0f64, 6.0f64, 0.0f64, FontMetricsPolicy::Raw, BaselinePolicy::Alphabetic, Some(BaselineClass::Roman), Some(MetricBox::RawFontBox), Some(FontMetricSource::RawTables), Some(UString::from(""))));
        let metric_decision2 = ClusterMetricDecision::new(TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[65292])), FontMetricsRequest::new(&(UStr::new(&[107])), 24.0f64, FontRole::CjkPunctuation, &(UStr::new(&[122,104,45,72,97,110,115])), Some(vec![]), Some(400), Some(false), Some(UString::from(""))), RawFontMetrics::new(18.0f64, 6.0f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None), LayoutFontMetrics::new(18.0f64, 6.0f64, 0.0f64, FontMetricsPolicy::Raw, BaselinePolicy::Alphabetic, Some(BaselineClass::Roman), Some(MetricBox::RawFontBox), Some(FontMetricSource::RawTables), Some(UString::from(""))));
        let inline_object1 = InlineObjectSpan::new(TextRange::new(0u32, 2u32).unwrap(), 32.0f64, 16.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(false), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 5.0f64, 10.0f64).unwrap()), Some(0.0), Some(0.0), Some(false)).unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(false)).unwrap())).unwrap();
        let inline_object2 = InlineObjectSpan::new(TextRange::new(15u32, 17u32).unwrap(), 32.0f64, 16.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(false)).unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), Some(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::BinaryOperator, 5.0f64, 10.0f64).unwrap()), Some(0.0), Some(0.0), Some(false)).unwrap())).unwrap();
        let mut inline_obj_map_builder: SortedMapTableBuilder<u32, InlineObjectSpan> = SortedTable::sorted_table_map_builder::<u32,
InlineObjectSpan>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        inline_obj_map_builder.put(&(0), &(inline_object1));
        inline_obj_map_builder.put(&(99), &(inline_object2));
        let inline_obj_map: SortedMapTable<u32, InlineObjectSpan> = inline_obj_map_builder.clone().build();
        let pinyin_a = RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[104,224,110,122,236])), Some(vec![]), RubyKind::Pinyin, None);
        let pinyin_b = RubySpan::new(TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[99,104,249])), Some(vec![]), RubyKind::Pinyin, None);
        let pinyin_c = RubySpan::new(TextRange::new(3u32, 5u32).unwrap(), &(UStr::new(&[99,232,115,104,236])), Some(vec![]), RubyKind::Pinyin, None);
        let mut rfg_builder: SortedMapTableBuilder<RubySpan, RubyFontGeometry> = SortedTable::sorted_table_map_builder::<RubySpan, RubyFontGeometry>(Arc::new(compare_ruby_span));
        rfg_builder.put(&(pinyin_a), &(RubyFontGeometry::new(20.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        rfg_builder.put(&(pinyin_b), &(RubyFontGeometry::new(10.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        rfg_builder.put(&(pinyin_c), &(RubyFontGeometry::new(20.0f64, 8.0f64, 2.0f64, 10.0f64, vec![].to_vec())));
        let rfg_map: SortedMapTable<RubySpan, RubyFontGeometry> = rfg_builder.clone().build();
        let res = LineAdjustmentStage::line_adjustment_stage_resolve_annotation_geometry(&mut engine, (input).clone(), 16.0f64, (inline_obj_map).clone(), (line_solution).clone(), (clreq_profile).clone(), &vec![(geom_decision).clone()], &vec![
    (AutoSpaceDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[108,101,97,100,105,110,103])), &(UStr::new(&[87,105,100,101])), &(UStr::new(&[78,111,114,109,97,108])), 1u32, 0.0f64, 0.0f64, &(UStr::new(&[116,101,115,116])))).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[108,101,97,100,105,110,103])), &(UStr::new(&[87,105,100,101])), &(UStr::new(&[78,111,114,109,97,108])), 1u32, 0.0f64, 0.0f64, &(UStr::new(&[116,101,115,116])))).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[116,114,97,105,108,105,110,103])), &(UStr::new(&[87,105,100,101])), &(UStr::new(&[78,111,114,109,97,108])), 1u32, 0.0f64, 0.0f64, &(UStr::new(&[116,101,115,116])))).clone(),
    (AutoSpaceDecisionInfo::new(TextRange::new(3u32, 5u32).unwrap(), &(UStr::new(&[116,114,97,105,108,105,110,103])), &(UStr::new(&[87,105,100,101])), &(UStr::new(&[78,111,114,109,97,108])), 1u32, 0.0f64, 0.0f64, &(UStr::new(&[116,101,115,116])))).clone(),
], &vec![(IntRange::new(1u32, 0u32)).clone(), (IntRange::new(0u32, 2u32)).clone()], &line_boxes, &clusters, &vec![FontRole::CjkText, FontRole::CjkPunctuation, FontRole::CjkText, FontRole::LatinText], SortedTable::sorted_table_map_builder::<u32,
f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(), SortedTable::sorted_table_map_builder::<u32,
f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(), &vec![(metric_decision1).clone(), (metric_decision2).clone()],
&vec![(pinyin_a).clone(), (pinyin_b).clone(), (pinyin_c).clone()], &clusters, (rfg_map).clone(), 0.0f64, 16.0f64, 8.0f64, 400, 4.0f64, {  Arc::new(move |__| {
        return 400;
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", res.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
    });
}

#[test]
fn bopomofo_and_decoration_leading_blank_exhaustive_branches() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoAndDecorationLeadingBlankExhaustiveBranches", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoAndDecorationLeadingBlankExhaustiveBranches", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[98,111,112,111,109,111,102,111,65,110,100,68,101,99,111,114,97,116,105,111,110,76,101,97,100,105,110,103,66,108,97,110,107,69,120,104,97,117,115,116,105,118,101,66,114,97,110,99,104,101,115]));
        let multi_glyph_shaper = Arc::new(Mutex::new(MultiGlyphBoundsShaper::new()));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some({ let __shared_handle: Arc<Mutex<dyn ITextShaper>> = multi_glyph_shaper.clone(); __shared_handle }), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let text = UString::from("中文English").to_ustring();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(500.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 7u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(2u32, 7u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[12549])), Some(vec![]), RubyKind::Bopomofo, None)).clone(),
    (RubySpan::new(TextRange::new(1u32, 2u32).unwrap(), &(UStr::new(&[12550])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
]), Some(vec![]), Some(vec![]));
        let res = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", res.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
        let input_narrow = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(30.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 7u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(2u32, 7u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[12549])), Some(vec![]), RubyKind::Bopomofo, None)).clone(),
    (RubySpan::new(TextRange::new(1u32, 2u32).unwrap(), &(UStr::new(&[12550])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
]), Some(vec![]), Some(vec![]));
        let res_narrow = engine.layout((input_narrow).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", res_narrow.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
    });
}

#[test]
fn bopomofo_over_latin_clusters_covers_cross_metric_lookup() {
    testlib::run("org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoOverLatinClustersCoversCrossMetricLookup", "org.tiqian.layout.AnnotationGeometryStageCoverageTest.bopomofoOverLatinClustersCoversCrossMetricLookup", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[65,110,110,111,116,97,116,105,111,110,71,101,111,109,101,116,114,121,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[98,111,112,111,109,111,102,111,79,118,101,114,76,97,116,105,110,67,108,117,115,116,101,114,115,67,111,118,101,114,115,67,114,111,115,115,77,101,116,114,105,99,76,111,111,107,117,112]));
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let text = UString::from("中文English").to_ustring();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(500.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[12549])), Some(vec![]), RubyKind::Bopomofo, None)).clone(),
    (RubySpan::new(TextRange::new(3u32, 4u32).unwrap(), &(UStr::new(&[12550])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
]), Some(vec![]), Some(vec![]));
        let res = engine.layout((input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("null") } else { TestTraceRender::test_trace_render_cap(UString::from(format!("{}", res.to_string()).as_str()).as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
    });
}
