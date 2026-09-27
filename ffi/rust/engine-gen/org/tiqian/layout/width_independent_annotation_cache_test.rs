#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCache;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationKey;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentParagraphAnnotation;
use crate::org::tiqian::layout::width_independent_annotation_cache_test_support::CountingTextShaper;
use crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupport;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        WidthIndependentAnnotationCacheTestRelayoutWithDifferentWidthHitsCacheAndSkipsShaperFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    SupportAssertEqualsTextRangeFault(crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault> for crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault {
    fn from(value: WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::SupportAssertEqualsTextRangeFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault> for WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault) -> Self {
        WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::SupportAssertEqualsTextRangeFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        WidthIndependentAnnotationCacheTestReflowFuzzingRandomSequenceProducesExactOutputFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    SupportAssertEqualsNullableAnnotationFault(crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault> for crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault {
    fn from(value: WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::SupportAssertEqualsNullableAnnotationFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault> for WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault) -> Self {
        WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::SupportAssertEqualsNullableAnnotationFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        WidthIndependentAnnotationCacheTestLruCacheEvictsOldestEntriesWhenCapacityExceededFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    SupportAssertEqualsTextRangeFault(crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault> for crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault {
    fn from(value: WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::SupportAssertEqualsTextRangeFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault> for WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache_test_support::WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault) -> Self {
        WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::SupportAssertEqualsTextRangeFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        WidthIndependentAnnotationCacheTestCachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidthsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        WidthIndependentAnnotationCacheTestCacheKeyDistinguishesTypographyDecorationsAndSpansFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn relayout_with_different_width_hits_cache_and_skips_shaper() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheTest.relayoutWithDifferentWidthHitsCacheAndSkipsShaper", "org.tiqian.layout.WidthIndependentAnnotationCacheTest.relayoutWithDifferentWidthHitsCacheAndSkipsShaper", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheTest");
        t.section(&"relayoutWithDifferentWidthHitsCacheAndSkipsShaper");
        let shaper = CountingTextShaper::new(None);
        let cache = LruWidthIndependentAnnotationCache::new(64u32);
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new((shaper).clone())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new((cache).clone()))).unwrap();
        let input_width1 = LayoutInput::new(TiqianTextContent::new("提椠是一个面向中文正文的 CJK 段落布局引擎。", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let _ = TracedAssertions::traced_assertions_assert_equals(0, cache.get_size(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, shaper.shape_call_count, None).unwrap();
        let result1 = engine.layout((input_width1).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result1.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, cache.get_size(), None).unwrap();
        let initial_shape_calls = shaper.shape_call_count;
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((initial_shape_calls).to_ne_bytes())) > (0), Some("Initial layout must shape segments".to_string())).unwrap();
        let input_width2 = LayoutInput::new((input_width1.content).clone(), Some((input_width1.text_style).clone()), Some((input_width1.paragraph_style).clone()), LayoutConstraints::new(180 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((input_width1.profile_id).clone()), Some((input_width1.decorations).clone()), Some((input_width1.ruby_spans).clone()), Some((input_width1.inline_boxes).clone()), Some((input_width1.inline_objects).clone()));
        let result2 = engine.layout((input_width2).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(initial_shape_calls, shaper.shape_call_count, Some("Relayout at new width must reuse cached annotation without shaping".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, cache.get_size(), None).unwrap();
        let input_width3 = LayoutInput::new((input_width1.content).clone(), Some((input_width1.text_style).clone()), Some((input_width1.paragraph_style).clone()), LayoutConstraints::new(500 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((input_width1.profile_id).clone()), Some((input_width1.decorations).clone()), Some((input_width1.ruby_spans).clone()), Some((input_width1.inline_boxes).clone()), Some((input_width1.inline_objects).clone()));
        let result3 = engine.layout((input_width3).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(initial_shape_calls, shaper.shape_call_count, Some("Relayout at third width must also reuse cached annotation".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result2.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) >= i32::from_ne_bytes((u32::try_from((result1.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()),
Some("Narrower width should have at least as many lines".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result1.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) >= i32::from_ne_bytes((u32::try_from((result3.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()),
Some("Wider width should have fewer or equal lines".to_string())).unwrap();
    });
}

#[test]
fn cached_and_uncached_engines_produce_identical_layout_results_across_widths() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheTest.cachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidths", "org.tiqian.layout.WidthIndependentAnnotationCacheTest.cachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidths", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheTest");
        t.section(&"cachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidths");
        let fixtures = vec![
    "提椠是一个面向中文正文的段落排版引擎，遵循中文排版需求规范，支持两端对齐与标点挤压。".to_string(),
    "在《中文排版需求》（CLREQ）中，要求正文「两端对齐」；当遇到『标点符号』与西文（如 OpenType / CSS Grid）混排时，应正确执行挤压与推入推出——即使在 120Hz 高频拖拽下也是如此！".to_string(),
    "第一行缩进两个字身框。标点符号如……省略号、破折号——不应出现在行首，逗号、句号。也不得出现在行首。这就是避头尾（Kinsoku）规则的严格要求。".to_string(),
];
        let mut cached_engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()),
Some("symbol-fallback".to_string())).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()),
Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())),
Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Box::new(LruWidthIndependentAnnotationCache::new(512u32)))).unwrap();
        let mut uncached_engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()),
Some("symbol-fallback".to_string())).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()),
Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())),
Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Box::new(NoOpWidthIndependentAnnotationCache::new()))).unwrap();
        let sweep_widths = WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_generate_sweep_widths(80 as f64, 7.3f64, 650 as f64);
        for fixture in &fixtures {
            {
                let mut _g = 0u32;
                while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((sweep_widths.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                    let width = sweep_widths[usize::try_from(_g).unwrap_or(0)];
                    _g = u32::wrapping_add(_g, 1);
                    let input = LayoutInput::new(TiqianTextContent::new(fixture.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0),
Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some(Ic(2 as f64)), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true),
None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
                    let expected = uncached_engine.layout((input).clone()).unwrap();
                    let actual = cached_engine.layout((input).clone()).unwrap();
                    let _ = TracedAssertions::traced_assertions_assert_equals(u32::try_from((expected.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), u32::try_from((actual.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((format!("{}{}",
            "Line count mismatch for fixture at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
                    for i in 0..match u32::try_from(expected.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let _ = WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_assert_equals_text_range(((expected.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone(),
((actual.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone(), Some((format!("{}{}{}{}",
            "Line ",
            crate::runtime::int_text::IntText::int_text(i),
            " range mismatch at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(expected.lines[usize::try_from(i).unwrap_or(0)].visual_width, actual.lines[usize::try_from(i).unwrap_or(0)].visual_width, 0.001f64, Some((format!("{}{}{}{}",
            "Line ",
            crate::runtime::int_text::IntText::int_text(i),
            " visualWidth mismatch at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(expected.lines[usize::try_from(i).unwrap_or(0)].adjusted_width, actual.lines[usize::try_from(i).unwrap_or(0)].adjusted_width, 0.001f64, Some((format!("{}{}{}{}",
            "Line ",
            crate::runtime::int_text::IntText::int_text(i),
            " adjustedWidth mismatch at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(expected.lines[usize::try_from(i).unwrap_or(0)].natural_width, actual.lines[usize::try_from(i).unwrap_or(0)].natural_width, 0.001f64, Some((format!("{}{}{}{}",
            "Line ",
            crate::runtime::int_text::IntText::int_text(i),
            " naturalWidth mismatch at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(expected.lines[usize::try_from(i).unwrap_or(0)].indent, actual.lines[usize::try_from(i).unwrap_or(0)].indent, 0.001f64, Some((format!("{}{}{}{}",
            "Line ",
            crate::runtime::int_text::IntText::int_text(i),
            " indent mismatch at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(expected.lines[usize::try_from(i).unwrap_or(0)].hanging_punctuation_advance, actual.lines[usize::try_from(i).unwrap_or(0)].hanging_punctuation_advance, 0.001f64,
Some((format!("{}{}{}{}",
            "Line ",
            crate::runtime::int_text::IntText::int_text(i),
            " hanging mismatch at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
                        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(expected.lines[usize::try_from(i).unwrap_or(0)].end_reason), &(actual.lines[usize::try_from(i).unwrap_or(0)].end_reason), Some((format!("{}{}{}{}",
            "Line ",
            crate::runtime::int_text::IntText::int_text(i),
            " endReason mismatch at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
                    }
                }
            }
        }
    });
}

#[test]
fn reflow_fuzzing_random_sequence_produces_exact_output() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheTest.reflowFuzzingRandomSequenceProducesExactOutput", "org.tiqian.layout.WidthIndependentAnnotationCacheTest.reflowFuzzingRandomSequenceProducesExactOutput", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheTest");
        t.section(&"reflowFuzzingRandomSequenceProducesExactOutput");
        let fixture = "提椠段落排版：严格遵循简体中文 CLREQ 规范。包含“双引号”、‘单引号’、以及（括号）与【括号】；汉字与 English words 混排时自动添加 0.25em 间距，最后一行保持左对齐。".to_string();
        let mut cached_engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()),
Some("symbol-fallback".to_string())).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()),
Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())),
Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Box::new(LruWidthIndependentAnnotationCache::new(512u32)))).unwrap();
        let mut uncached_engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()),
Some("symbol-fallback".to_string())).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()),
Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())),
Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Box::new(NoOpWidthIndependentAnnotationCache::new()))).unwrap();
        let random_sequence_widths = vec![
    320.0f64,
    150.0f64,
    480.5f64,
    95.2f64,
    210.0f64,
    600.0f64,
    120.3f64,
    450.0f64,
    180.7f64,
    300.0f64,
    75.0f64,
    520.0f64,
    133.3f64,
    266.6f64,
    399.9f64,
    110.0f64,
    470.0f64,
    195.0f64,
    345.0f64,
    580.0f64,
];
        for &width in &random_sequence_widths {
            let input = LayoutInput::new(TiqianTextContent::new(fixture.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some(Ic(2 as f64)), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
            let expected = uncached_engine.layout((input).clone()).unwrap();
            let actual = cached_engine.layout((input).clone()).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals(u32::try_from((expected.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), u32::try_from((actual.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((format!("{}{}",
            "Fuzz line count mismatch at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
            for i in 0..match u32::try_from(expected.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let _ = WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_assert_equals_text_range(((expected.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone(),
((actual.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone(), Some((format!("{}{}{}{}",
            "Fuzz line ",
            crate::runtime::int_text::IntText::int_text(i),
            " range mismatch at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(expected.lines[usize::try_from(i).unwrap_or(0)].visual_width, actual.lines[usize::try_from(i).unwrap_or(0)].visual_width, 0.001f64, Some((format!("{}{}{}{}",
            "Fuzz line ",
            crate::runtime::int_text::IntText::int_text(i),
            " width mismatch at width ",
            WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_float_text(width).unwrap()
        )).to_string())).unwrap();
            }
        }
    });
}

#[test]
fn cache_key_distinguishes_typography_decorations_and_spans() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheTest.cacheKeyDistinguishesTypographyDecorationsAndSpans", "org.tiqian.layout.WidthIndependentAnnotationCacheTest.cacheKeyDistinguishesTypographyDecorationsAndSpans", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheTest");
        t.section(&"cacheKeyDistinguishesTypographyDecorationsAndSpans");
        let cache = LruWidthIndependentAnnotationCache::new(512u32);
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new((cache).clone()))).unwrap();
        let base_input = LayoutInput::new(TiqianTextContent::new("中西混合排版与测试文本。", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        engine.layout((base_input).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, cache.get_size(), None).unwrap();
        let text_changed = LayoutInput::new(TiqianTextContent::new("中西混合排版与变动文本。", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some((base_input.text_style).clone()), Some((base_input.paragraph_style).clone()), (base_input.constraints).clone(),
Some((base_input.profile_id).clone()), Some((base_input.decorations).clone()), Some((base_input.ruby_spans).clone()), Some((base_input.inline_boxes).clone()), Some((base_input.inline_objects).clone()));
        engine.layout((text_changed).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, cache.get_size(), None).unwrap();
        let font_changed = LayoutInput::new((base_input.content).clone(), Some(TextStyle::new(Some(vec![]), Some(24 as f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((base_input.paragraph_style).clone()),
(base_input.constraints).clone(), Some((base_input.profile_id).clone()), Some((base_input.decorations).clone()), Some((base_input.ruby_spans).clone()), Some((base_input.inline_boxes).clone()), Some((base_input.inline_objects).clone()));
        engine.layout((font_changed).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, cache.get_size(), None).unwrap();
        let emphasis_changed = LayoutInput::new((base_input.content).clone(), Some((base_input.text_style).clone()), Some((base_input.paragraph_style).clone()), (base_input.constraints).clone(), Some((base_input.profile_id).clone()),
Some(vec![(DecorationSpan::new(TextRange::new(0u32, 4u32).unwrap(), DecorationKind::Emphasis)).clone()]), Some((base_input.ruby_spans).clone()), Some((base_input.inline_boxes).clone()), Some((base_input.inline_objects).clone()));
        engine.layout((emphasis_changed).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(4, cache.get_size(), None).unwrap();
        let ruby_changed = LayoutInput::new((base_input.content).clone(), Some((base_input.text_style).clone()), Some((base_input.paragraph_style).clone()), (base_input.constraints).clone(), Some((base_input.profile_id).clone()), Some((base_input.decorations).clone()), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), "zhōngxī", Some(vec![]), RubyKind::Pinyin, None)).clone(),
]), Some((base_input.inline_boxes).clone()), Some((base_input.inline_objects).clone()));
        engine.layout((ruby_changed).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(5, cache.get_size(), None).unwrap();
        let inline_box_changed = LayoutInput::new((base_input.content).clone(), Some((base_input.text_style).clone()), Some((base_input.paragraph_style).clone()), (base_input.constraints).clone(), Some((base_input.profile_id).clone()), Some((base_input.decorations).clone()),
Some((base_input.ruby_spans).clone()), Some(vec![
    (InlineBoxSpan::new(TextRange::new(2u32, 4u32).unwrap(), Some(4 as f64), Some(4 as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), Some((base_input.inline_objects).clone()));
        engine.layout((inline_box_changed).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(6, cache.get_size(), None).unwrap();
    });
}

#[test]
fn lru_cache_evicts_oldest_entries_when_capacity_exceeded() {
    testlib::run("org.tiqian.layout.WidthIndependentAnnotationCacheTest.lruCacheEvictsOldestEntriesWhenCapacityExceeded", "org.tiqian.layout.WidthIndependentAnnotationCacheTest.lruCacheEvictsOldestEntriesWhenCapacityExceeded", || {
        let mut t = TestTraceRecorder::new("WidthIndependentAnnotationCacheTest");
        t.section(&"lruCacheEvictsOldestEntriesWhenCapacityExceeded");
        let mut cache = LruWidthIndependentAnnotationCache::new(2u32);
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new((cache).clone()))).unwrap();
        let input1 = LayoutInput::new(TiqianTextContent::new("段落一文本内容", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let input2 = LayoutInput::new(TiqianTextContent::new("段落二文本内容", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let input3 = LayoutInput::new(TiqianTextContent::new("段落三文本内容", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        engine.layout((input1).clone()).unwrap();
        engine.layout((input2).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, cache.get_size(), None).unwrap();
        let key1 = WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_annotation_key((input1).clone());
        let key2 = WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_annotation_key((input2).clone());
        let _ = TracedAssertions::traced_assertions_assert_true(cache.get((key1).clone()).is_some(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(cache.get((key2).clone()).is_some(), None).unwrap();
        engine.layout((input3).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, cache.get_size(), None).unwrap();
        let key3 = WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_annotation_key((input3).clone());
        let _ = TracedAssertions::traced_assertions_assert_true(cache.get((key3).clone()).is_some(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(cache.get((key2).clone()).is_some(), None).unwrap();
        let _ = WidthIndependentAnnotationCacheTestSupport::width_independent_annotation_cache_test_support_assert_equals_nullable_annotation(None, cache.get((key1).clone()), Some("Oldest entry key1 should be evicted".to_string())).unwrap();
    });
}

#[derive(Clone, PartialEq)]
pub struct NoOpWidthIndependentAnnotationCache {
}

impl NoOpWidthIndependentAnnotationCache {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn get_size(&self) -> u32 {
        return 0;
    }

    pub fn get(&self, _key: WidthIndependentAnnotationKey) -> Option<WidthIndependentParagraphAnnotation> {
        return None;
    }

    pub fn put(&self, _key: WidthIndependentAnnotationKey, _annotation: WidthIndependentParagraphAnnotation) {
    }

    pub fn clear(&self) {
    }
}

impl WidthIndependentAnnotationCache for NoOpWidthIndependentAnnotationCache {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.WidthIndependentAnnotationCacheTest.NoOpWidthIndependentAnnotationCache"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn WidthIndependentAnnotationCache> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> u32 {
        return 0;
    }

    fn get(&mut self, _key: WidthIndependentAnnotationKey) -> Option<WidthIndependentParagraphAnnotation> {
        return None;
    }

    fn put(&mut self, _key: WidthIndependentAnnotationKey, _annotation: WidthIndependentParagraphAnnotation) {
    }

    fn clear(&mut self) {
    }
}
