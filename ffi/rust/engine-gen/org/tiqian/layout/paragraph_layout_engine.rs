use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::FontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::FontMetricsResolver;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::font_policy::FallbackResolver;
use crate::org::tiqian::font::font_policy::FontCandidate;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_policy::FontRequest;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleClassifier;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_adjustment_stage::LineAdjustmentStage;
use crate::org::tiqian::layout::line_break_planning_stage::LineBreakPlanningStage;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCache;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheFns;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentParagraphAnnotation;
use crate::org::tiqian::linebreak::hyphenator::Hyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineNewFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    LayoutWithRejectedTechnicalTiersFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<ParagraphLayoutEngineNewFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineNewFault) -> Self {
        match value {
            ParagraphLayoutEngineNewFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineNewFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphLayoutEngineNewFault) -> Self {
        match value {
            ParagraphLayoutEngineNewFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineNewFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ParagraphLayoutEngineNewFault) -> Self {
        match value {
            ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineNewFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineNewFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphLayoutEngineNewFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphLayoutEngineNewFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ParagraphLayoutEngineNewFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    LineAdjustmentStageFinishParagraphLayoutFaultFault(crate::org::tiqian::layout::line_adjustment_stage::LineAdjustmentStageFinishParagraphLayoutFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        match value {
            ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        match value {
            ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        match value {
            ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for crate::org::tiqian::layout::line_adjustment_stage::LineAdjustmentStageFinishParagraphLayoutFault {
    fn from(value: ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        match value {
            ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::LineAdjustmentStageFinishParagraphLayoutFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        match value {
            ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_adjustment_stage::LineAdjustmentStageFinishParagraphLayoutFault> for ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: crate::org::tiqian::layout::line_adjustment_stage::LineAdjustmentStageFinishParagraphLayoutFault) -> Self {
        ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::LineAdjustmentStageFinishParagraphLayoutFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct ParagraphLayoutEngineFns;

impl ParagraphLayoutEngineFns {
    pub const PARAGRAPH_LAYOUT_ENGINE_FNS_MANDATORY_BREAK_FONT_KEY: &str = "mandatory-break";
}

pub trait ParagraphLayoutEngine: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn ParagraphLayoutEngine>;
    fn layout(&mut self, input: LayoutInput) -> Result<LayoutResult, ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault>;
}

impl Clone for Box<dyn ParagraphLayoutEngine> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn ParagraphLayoutEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

#[derive(Clone)]
pub struct ExplainableStubParagraphLayoutEngine {
    pub font_role_classifier: Box<dyn FontRoleClassifier>,
    pub fallback_resolver: Box<dyn FallbackResolver>,
    pub clreq_profile_resolver: Box<dyn ClreqProfileResolver>,
    pub font_metrics_resolver: Box<dyn FontMetricsResolver>,
    pub font_metrics_normalizer: Box<dyn FontMetricsNormalizer>,
    pub punctuation_atom_builder: PunctuationAtomBuilder,
    pub punctuation_spacing_compressor: PunctuationSpacingCompressor,
    pub quote_pair_analyzer: QuotePairAnalyzer,
    pub line_breaker: Box<dyn LineBreaker>,
    pub justifier: Justifier,
    pub text_shaper: Box<dyn ITextShaper>,
    pub hyphenator: Box<dyn Hyphenator>,
    pub annotation_cache: Box<dyn WidthIndependentAnnotationCache>,
}

impl ExplainableStubParagraphLayoutEngine {
    pub fn new(font_role_classifier: Option<Box<dyn FontRoleClassifier>>, fallback_resolver: Option<Box<dyn FallbackResolver>>, clreq_profile_resolver: Option<Box<dyn ClreqProfileResolver>>, font_metrics_resolver: Option<Box<dyn FontMetricsResolver>>, font_metrics_normalizer:
Option<Box<dyn FontMetricsNormalizer>>, punctuation_atom_builder: Option<PunctuationAtomBuilder>, punctuation_spacing_compressor: Option<PunctuationSpacingCompressor>, quote_pair_analyzer: Option<QuotePairAnalyzer>, line_breaker: Option<Box<dyn LineBreaker>>, justifier:
Option<Justifier>, text_shaper: Option<Box<dyn ITextShaper>>, hyphenator: Option<Box<dyn Hyphenator>>, annotation_cache: Option<Box<dyn WidthIndependentAnnotationCache>>) -> Result<Self, ParagraphLayoutEngineNewFault> {
        let font_role_classifier = font_role_classifier.unwrap_or_else(|| Box::new(CjkFontRoleClassifier::new()));
        let fallback_resolver = fallback_resolver.unwrap_or_else(|| Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap()));
        let clreq_profile_resolver = clreq_profile_resolver.unwrap_or_else(|| Box::new(BuiltInClreqProfileResolver::new()));
        let font_metrics_resolver = font_metrics_resolver.unwrap_or_else(|| Box::new(StubFontMetricsResolver::new()));
        let font_metrics_normalizer = font_metrics_normalizer.unwrap_or_else(|| Box::new(ScriptAwareFontMetricsNormalizer::new()));
        let punctuation_atom_builder = punctuation_atom_builder.unwrap_or_else(|| PunctuationAtomBuilder::new(None, None).unwrap());
        let punctuation_spacing_compressor = punctuation_spacing_compressor.unwrap_or_else(|| PunctuationSpacingCompressor::new().unwrap());
        let quote_pair_analyzer = quote_pair_analyzer.unwrap_or_else(|| QuotePairAnalyzer::new());
        let line_breaker = line_breaker.unwrap_or_else(|| Box::new(GreedyLineBreaker::new(None, None, None, None)));
        let justifier = justifier.unwrap_or_else(|| Justifier::new(Some(0.5), Some(0.25)));
        let text_shaper = text_shaper.unwrap_or_else(|| Box::new(ExplainableStubTextShaper::new()));
        let hyphenator = hyphenator.unwrap_or_else(|| DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap());
        let annotation_cache = annotation_cache.unwrap_or_else(|| Box::new(LruWidthIndependentAnnotationCache::new(512)));
        Ok(Self {
            font_role_classifier: font_role_classifier,
            fallback_resolver: fallback_resolver,
            clreq_profile_resolver: clreq_profile_resolver,
            font_metrics_resolver: font_metrics_resolver,
            font_metrics_normalizer: font_metrics_normalizer,
            punctuation_atom_builder: punctuation_atom_builder,
            punctuation_spacing_compressor: punctuation_spacing_compressor,
            quote_pair_analyzer: quote_pair_analyzer,
            line_breaker: line_breaker,
            justifier: justifier,
            text_shaper: text_shaper,
            hyphenator: hyphenator,
            annotation_cache: annotation_cache,
        })
    }

    pub fn layout(&mut self, input: LayoutInput) -> Result<LayoutResult, ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> {
        return Ok(self.layout_with_rejected_technical_tiers((input).clone(), SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range)).clone().build())?);
    }

    pub fn layout_with_rejected_technical_tiers(&mut self, input: LayoutInput, rejected_technical_tiers_by_span: SortedMapTable<TextRange, SortedSetTable<u32>>) -> Result<LayoutResult, ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> {
        let _ = self.validate_layout_input((input).clone()).map_err(|e| ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::TextRangeErrorFault(e))?;
        let cache_key = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_to_width_independent_annotation_key((input).clone(), Some((rejected_technical_tiers_by_span).clone()));
        let cached = self.annotation_cache.get((cache_key).clone());
        let annotation: WidthIndependentParagraphAnnotation;
        if cached.is_some() {
            annotation = (cached).as_ref().unwrap().clone();
        } else {
            annotation = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(self, (input).clone(), (rejected_technical_tiers_by_span).clone()).map_err(|e|
ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::ParagraphShapingStageShapeParagraphFaultFault(e))?;
            self.annotation_cache.put((cache_key).clone(), (annotation).clone());
        }
        let prep = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(self, (input).clone(), (annotation).clone(), (rejected_technical_tiers_by_span).clone()).map_err(|e|
ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(e))?;
        return Ok(LineAdjustmentStage::line_adjustment_stage_finish_paragraph_layout(self, (prep).clone(), LineBreakPlanningStage::line_break_planning_stage_plan_paragraph_lines((self).clone(), (prep).clone()).map_err(|e|
ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::TextRangeErrorFault(e))?).map_err(|e| ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::LineAdjustmentStageFinishParagraphLayoutFaultFault(e))?);
    }

    fn validate_layout_input(&self, input: LayoutInput) -> Result<(), TextRangeError> {
        let text = ((input.content).clone().text).to_string();
        if !((input.paragraph_style).clone().emphasis_dot_gap_em).is_finite() || ((input.paragraph_style).clone().emphasis_dot_gap_em) < (0 as f64) {
            return Err(TextRangeError::Message { text: "ParagraphStyle.emphasisDotGapEm must be finite and non-negative".to_string() });
        }
        if !((input.paragraph_style).clone().inline_object_minimum_clearance_em).is_finite() || ((input.paragraph_style).clone().inline_object_minimum_clearance_em) < (0 as f64) {
            return Err(TextRangeError::Message { text: "ParagraphStyle.inlineObjectMinimumClearanceEm must be finite and non-negative".to_string() });
        }
        let mut surrogate_scan = 0u32;
        let __units = u_string::units(&text);
        let __count = u_string::unit_count(&text);
        while (i32::from_ne_bytes((surrogate_scan).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            let code = u_string::unit_at_from(&__units, surrogate_scan).unwrap_or(0);
            if i32::from_ne_bytes((code).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes((code).to_ne_bytes())) <= 56319 {
                if !((i32::from_ne_bytes((u32::wrapping_add(surrogate_scan, 1)).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&(text))).to_ne_bytes())) && (i32::from_ne_bytes((u_string::unit_at(&text, u32::wrapping_add(surrogate_scan,
1)).unwrap_or(0)).to_ne_bytes())) >= 56320 && (i32::from_ne_bytes((u_string::unit_at(&text, u32::wrapping_add(surrogate_scan, 1)).unwrap_or(0)).to_ne_bytes())) <= 57343) {
                    return Err(TextRangeError::Message { text: format!("{}{}",
            "SourceText has an unpaired high surrogate at char ",
            crate::runtime::int_text::IntText::int_text(surrogate_scan)
        ).to_string() });
                }
                surrogate_scan = u32::wrapping_add(surrogate_scan, 2);
            } else {
                if i32::from_ne_bytes((code).to_ne_bytes()) >= 56320 && (i32::from_ne_bytes((code).to_ne_bytes())) <= 57343 {
                    return Err(TextRangeError::Message { text: format!("{}{}",
            "SourceText has an unpaired low surrogate at char ",
            crate::runtime::int_text::IntText::int_text(surrogate_scan)
        ).to_string() });
                }
                surrogate_scan = u32::wrapping_add(surrogate_scan, 1);
            }
        }
        for i in 0..match u32::try_from(input.inline_boxes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let inline_box = (input.inline_boxes[usize::try_from(i).unwrap_or(0)]).clone();
            if !(((inline_box.range).clone().start) <= 2147483647 && (i32::from_ne_bytes(((inline_box.range).clone().start).to_ne_bytes())) < (i32::from_ne_bytes(((inline_box.range).clone().end).to_ne_bytes())) &&
(i32::from_ne_bytes(((inline_box.range).clone().end).to_ne_bytes())) <= i32::from_ne_bytes((u_string::unit_count(&(text))).to_ne_bytes())) {
                return Err(TextRangeError::Message { text: format!("{}{}{}",
            "InlineBoxSpan ",
            (inline_box.range).clone().to_string(),
            " must be a non-empty source range"
        ).to_string() });
            }
            if !((inline_box.inline_start).is_finite() && (inline_box.inline_end).is_finite()) {
                return Err(TextRangeError::Message { text: format!("{}{}{}",
            "InlineBoxSpan ",
            (inline_box.range).clone().to_string(),
            " must have finite inline edges"
        ).to_string() });
            }
        }
        for i in 0..match u32::try_from((input.content).clone().line_break_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let span = ((input.content).clone().line_break_spans[usize::try_from(i).unwrap_or(0)]).clone();
            if !(((span.range).clone().start) <= 2147483647 && (i32::from_ne_bytes(((span.range).clone().start).to_ne_bytes())) < (i32::from_ne_bytes(((span.range).clone().end).to_ne_bytes())) && (i32::from_ne_bytes(((span.range).clone().end).to_ne_bytes())) <=
i32::from_ne_bytes((u_string::unit_count(&(text))).to_ne_bytes())) {
                return Err(TextRangeError::Message { text: format!("{}{}{}",
            "LineBreakSpan ",
            (span.range).clone().to_string(),
            " must be a non-empty source range"
        ).to_string() });
            }
        }
        for i in 0..match u32::try_from((input.content).clone().auto_space_suppressed_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let range = ((input.content).clone().auto_space_suppressed_ranges[usize::try_from(i).unwrap_or(0)]).clone();
            if !((range.start) <= 2147483647 && (i32::from_ne_bytes((range.start).to_ne_bytes())) < (i32::from_ne_bytes((range.end).to_ne_bytes())) && (i32::from_ne_bytes((range.end).to_ne_bytes())) <= i32::from_ne_bytes((u_string::unit_count(&(text))).to_ne_bytes())) {
                return Err(TextRangeError::Message { text: format!("{}{}{}",
            "Auto-space suppressed range ",
            range.to_string(),
            " must be a non-empty source range"
        ).to_string() });
            }
        }
        let mut seen_ranges: Vec<TextRange> = Vec::new();
        for i in 0..match u32::try_from(input.inline_objects.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let inline_object = (input.inline_objects[usize::try_from(i).unwrap_or(0)]).clone();
            for seen in &seen_ranges {
                if seen.start == (inline_object.range).clone().start && seen.end == (inline_object.range).clone().end {
                    return Err(TextRangeError::Message { text: "InlineObjectSpan ranges must be unique".to_string() });
                }
            }
            seen_ranges.push((inline_object.range).clone());
        }
        let capacity = input.inline_objects.len();
        let mut sorted_objects = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(input.inline_objects.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            sorted_objects.push((input.inline_objects[usize::try_from(i).unwrap_or(0)]).clone());
        }
        let mut i = 1u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((sorted_objects.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let item = (sorted_objects[usize::try_from(i).unwrap_or(0)]).clone();
            let mut j = i;
            while (j) > (0) && (i32::from_ne_bytes((((sorted_objects[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes())) > (i32::from_ne_bytes(((item.range).clone().start).to_ne_bytes())) {
                sorted_objects[usize::try_from(j).unwrap_or(0)] = (sorted_objects[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]).clone();
                j = u32::wrapping_sub(j, 1);
            }
            sorted_objects[usize::try_from(j).unwrap_or(0)] = item;
            i = u32::wrapping_add(i, 1);
        }
        for idx in 0..u32::try_from((sorted_objects.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
            let prev_obj = (sorted_objects[usize::try_from(idx).unwrap_or(0)]).clone();
            let next_obj = (sorted_objects[usize::try_from(u32::wrapping_add(idx, 1)).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((prev_obj.range).clone().end).to_ne_bytes()) > (i32::from_ne_bytes(((next_obj.range).clone().start).to_ne_bytes())) {
                return Err(TextRangeError::Message { text: format!("{}{}{}{}",
            "InlineObjectSpan ranges must not overlap: ",
            (prev_obj.range).clone().to_string(),
            " and ",
            (next_obj.range).clone().to_string()
        ).to_string() });
            }
        }
        for k in 0..match u32::try_from(input.inline_objects.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let inline_object = (input.inline_objects[usize::try_from(k).unwrap_or(0)]).clone();
            if !(((inline_object.range).clone().start) <= 2147483647 && (i32::from_ne_bytes(((inline_object.range).clone().start).to_ne_bytes())) < (i32::from_ne_bytes(((inline_object.range).clone().end).to_ne_bytes())) &&
(i32::from_ne_bytes(((inline_object.range).clone().end).to_ne_bytes())) <= i32::from_ne_bytes((u_string::unit_count(&(text))).to_ne_bytes())) {
                return Err(TextRangeError::Message { text: format!("{}{}{}",
            "InlineObjectSpan ",
            (inline_object.range).clone().to_string(),
            " must cover a non-empty source range"
        ).to_string() });
            }
            if !((inline_object.advance).is_finite() && (inline_object.advance) > (0 as f64) && (inline_object.ascent).is_finite() && (inline_object.ascent) >= 0 as f64 && (inline_object.descent).is_finite() && (inline_object.descent) >= 0 as f64) {
                return Err(TextRangeError::Message { text: format!("{}{}{}",
            "InlineObjectSpan ",
            (inline_object.range).clone().to_string(),
            " must have finite positive geometry"
        ).to_string() });
            }
            if inline_object.leading_boundary.clone().shrink_capacity != 0 as f64 {
                return Err(TextRangeError::Message { text: format!("{}{}{}",
            "InlineObjectSpan ",
            (inline_object.range).clone().to_string(),
            " cannot shrink its leading boundary"
        ).to_string() });
            }
            if inline_object.leading_boundary.clone().line_end_discardable_advance != 0 as f64 {
                return Err(TextRangeError::Message { text: format!("{}{}{}",
            "InlineObjectSpan ",
            (inline_object.range).clone().to_string(),
            " cannot discard advance at its leading boundary"
        ).to_string() });
            }
            if inline_object.trailing_boundary.clone().shrink_capacity > (inline_object.advance) {
                return Err(TextRangeError::Message { text: format!("{}{}{}",
            "InlineObjectSpan ",
            (inline_object.range).clone().to_string(),
            " trailing shrink capacity must not exceed its advance"
        ).to_string() });
            }
            if inline_object.trailing_boundary.clone().line_end_discardable_advance > (inline_object.advance) {
                return Err(TextRangeError::Message { text: format!("{}{}{}",
            "InlineObjectSpan ",
            (inline_object.range).clone().to_string(),
            " trailing line-end discard must not exceed its advance"
        ).to_string() });
            }
        }
        Ok(())
    }
}

impl ParagraphLayoutEngine for ExplainableStubParagraphLayoutEngine {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphLayoutEngine.ExplainableStubParagraphLayoutEngine"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ParagraphLayoutEngine> {
        Box::new(self.clone())
    }

    fn layout(&mut self, input: LayoutInput) -> Result<LayoutResult, ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> {
        return Ok(self.layout_with_rejected_technical_tiers((input).clone(), SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range)).clone().build())?);
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphLayoutFallbackResolver {
    pub cjk_font_key: String,
    pub latin_font_key: String,
    pub symbol_font_key: String,
}

impl ParagraphLayoutFallbackResolver {
    pub fn new(cjk_font_key: Option<String>, latin_font_key: Option<String>, symbol_font_key: Option<String>) -> Result<Self, ParagraphLayoutEngineNewFault> {
        let cjk_font_key = cjk_font_key.unwrap_or_else(|| "cjk-primary".to_string());
        let latin_font_key = latin_font_key.unwrap_or_else(|| "latin-primary".to_string());
        let symbol_font_key = symbol_font_key.unwrap_or_else(|| "symbol-fallback".to_string());
        Ok(Self {
            cjk_font_key: cjk_font_key,
            latin_font_key: latin_font_key,
            symbol_font_key: symbol_font_key,
        })
    }

    pub fn resolve(&self, _text: &str, range: TextRange, request: FontRequest) -> FontDecision {
        let c: FontCandidate;
        {
            let _g = request.role;
            let _ = match _g {
    FontRole::CjkText | FontRole::CjkPunctuation => c = FontCandidate::new((self.cjk_font_key).to_string().as_str(), if u32::try_from((request.preferred_families.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { (self.cjk_font_key).to_string() } else {
(request.preferred_families[0usize]).clone() }.as_str(), request.role),
    FontRole::LatinText => c = FontCandidate::new((self.latin_font_key).to_string().as_str(), (self.latin_font_key).to_string().as_str(), request.role),
    FontRole::Symbol | FontRole::Emoji | FontRole::Unknown => c = FontCandidate::new((self.symbol_font_key).to_string().as_str(), (self.symbol_font_key).to_string().as_str(), request.role),
};
        }
        return FontDecision::new((range).clone(), (c).clone(), request.role, format!("{}{}",
            "PreferCjkForAmbiguousPunctuationResolver:",
            request.role.name()
        ).as_str());
    }
}

impl FallbackResolver for ParagraphLayoutFallbackResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphLayoutEngine.ParagraphLayoutFallbackResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn FallbackResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _text: &str, range: TextRange, request: FontRequest) -> FontDecision {
        let c: FontCandidate;
        {
            let _g = request.role;
            let _ = match _g {
    FontRole::CjkText | FontRole::CjkPunctuation => c = FontCandidate::new((self.cjk_font_key).to_string().as_str(), if u32::try_from((request.preferred_families.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { (self.cjk_font_key).to_string() } else {
(request.preferred_families[0usize]).clone() }.as_str(), request.role),
    FontRole::LatinText => c = FontCandidate::new((self.latin_font_key).to_string().as_str(), (self.latin_font_key).to_string().as_str(), request.role),
    FontRole::Symbol | FontRole::Emoji | FontRole::Unknown => c = FontCandidate::new((self.symbol_font_key).to_string().as_str(), (self.symbol_font_key).to_string().as_str(), request.role),
};
        }
        return FontDecision::new((range).clone(), (c).clone(), request.role, format!("{}{}",
            "PreferCjkForAmbiguousPunctuationResolver:",
            request.role.name()
        ).as_str());
    }
}
