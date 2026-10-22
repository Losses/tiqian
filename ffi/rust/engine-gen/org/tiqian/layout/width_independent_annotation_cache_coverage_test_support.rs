use crate::org::tiqian::clreq::adjustment_style_policy::AdjustmentStylePolicy;
use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_tier_priority::ProgressiveBreakTierPriority;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheFns;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationKey;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentParagraphAnnotation;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WidthIndependentAnnotationCacheCoverageTestSupportEngineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            WidthIndependentAnnotationCacheCoverageTestSupportEngineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestSupportEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestSupportEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestSupportEngineFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestSupportEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestSupportEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheCoverageTestSupportEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestSupportEngineFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::ParagraphShapingStageShapeParagraphFaultFault(value) => write!(formatter, "{}", value),
            WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct WidthIndependentAnnotationCacheCoverageTestSupport;

impl WidthIndependentAnnotationCacheCoverageTestSupport {
    pub fn width_independent_annotation_cache_coverage_test_support_empty_tiers() -> SortedMapTable<TextRange, SortedSetTable<u32>> {
        return SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range)).clone().build();
    }

    pub fn width_independent_annotation_cache_coverage_test_support_engine(clreq_profile_resolver: Option<Box<dyn ClreqProfileResolver>>, text_shaper: Option<Arc<Mutex<dyn ITextShaper>>>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), clreq_profile_resolver, Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), text_shaper, Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn width_independent_annotation_cache_coverage_test_support_key(input: LayoutInput) -> WidthIndependentAnnotationKey {
        return WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_to_width_independent_annotation_key((input).clone(), None);
    }

    pub fn width_independent_annotation_cache_coverage_test_support_tier_set(tiers: &Vec<ProgressiveBreakTier>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..match u32::try_from(tiers.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            b.put(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(tiers[usize::try_from(i).unwrap_or(0)])));
        }
        return b.clone().build();
    }

    pub fn width_independent_annotation_cache_coverage_test_support_tier_map(range: TextRange, tiers: &Vec<ProgressiveBreakTier>) -> SortedMapTable<TextRange, SortedSetTable<u32>> {
        let mut b: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
        b.put(&(range), &(WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_tier_set(&tiers)));
        return b.clone().build();
    }

    pub fn width_independent_annotation_cache_coverage_test_support_render_nullable_range(v: Option<TextRange>) -> UString {
        return match &(v) { None => UString::from("null"), Some(__option) => WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_render_range(((*__option).clone()).clone()).to_ustring() };
    }

    pub fn width_independent_annotation_cache_coverage_test_support_render_range(v: TextRange) -> UString {
        return UString::from(format!("{}", v.to_string()).as_str());
    }

    pub fn width_independent_annotation_cache_coverage_test_support_containing_items(clusters: &Vec<Cluster>, items: &Vec<TextRange>) -> Vec<Option<TextRange>> {
        let mut out: Vec<Option<TextRange>> = vec![];
        let mut item_index = 0u32;
        for cluster in clusters {
            while (i32::from_ne_bytes(((item_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) && (i32::from_ne_bytes(((items[usize::try_from(item_index).unwrap_or(0)].end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) {
                item_index = u32::wrapping_add(item_index, 1);
            }
            let item = if i32::from_ne_bytes(((item_index) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { Some((items[usize::try_from(item_index).unwrap_or(0)]).clone()) } else { None };
            let candidate = if match &(item) { Some(__option2) => i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((__option2.start) as i32).to_ne_bytes()) && (i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((__option2.end) as i32).to_ne_bytes()), None => false } { item } else { None };
            out.push(candidate.clone());
        }
        return out;
    }

    pub fn width_independent_annotation_cache_coverage_test_support_first_contained_item(clusters: &Vec<Cluster>, items: &Vec<TextRange>) -> Vec<Option<TextRange>> {
        let mut out: Vec<Option<TextRange>> = vec![];
        let mut item_index = 0u32;
        for cluster in clusters {
            while (i32::from_ne_bytes(((item_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) && (i32::from_ne_bytes(((items[usize::try_from(item_index).unwrap_or(0)].end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) {
                item_index = u32::wrapping_add(item_index, 1);
            }
            let item = if i32::from_ne_bytes(((item_index) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { Some((items[usize::try_from(item_index).unwrap_or(0)]).clone()) } else { None };
            let candidate = if match &(item) { Some(__option4) => i32::from_ne_bytes(((__option4.start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((__option4.end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes()), None => false } { item } else { None };
            out.push(candidate.clone());
        }
        return out;
    }

    pub(crate) fn width_independent_annotation_cache_coverage_test_support_copy_annotation(a: WidthIndependentParagraphAnnotation, clreq_profile: Option<ClreqProfile>, font_decisions: Option<Vec<FontDecision>>, segment_shaping_cache: Option<SortedMapTable<TextRange, ShapingResult>>) -> WidthIndependentParagraphAnnotation {
        return WidthIndependentParagraphAnnotation::new((a.text).to_ustring().as_ustr(), a.font_size, (a.style_at).clone(), (a.font_size_at).clone(), (a.bopomofo_font_weight_at).clone(), a.ruby_font_size, a.ruby_stack_gap, a.ruby_font_weight, a.pinyin_spans.to_vec(), match &(clreq_profile) { None => (a.clreq_profile).clone(), Some(__option5) => (*__option5).clone() }, (a.punctuation_glyph_substitutor).clone(), a.quote_pairs.to_vec(), a.role_override_infos.to_vec(), match &(font_decisions) { None => (a.font_decisions).clone(), Some(__option6) => (*__option6).clone() }.to_vec(), a.cluster_ranges.to_vec(), (a.font_decision_by_range).clone(), (a.inline_object_by_range).clone(), match &(segment_shaping_cache) { None => (a.segment_shaping_cache).clone(), Some(__option7) => (*__option7).clone() }, (a.substitution_rollbacks).clone(), (a.ruby_font_geometry_by_span).clone(), (a.base_shaping_stage).clone());
    }

    pub fn width_independent_annotation_cache_coverage_test_support_with_adjusted_profile(annotation: WidthIndependentParagraphAnnotation, allow_inline_stop_compression: bool, allow_sino_western_gap_adjustment: bool) -> WidthIndependentParagraphAnnotation {
        let source = (annotation.clreq_profile).clone().clone();
        let custom_adjustment = AdjustmentStylePolicy::new(Some((source.adjustment).clone().line_end_punctuation), Some(allow_inline_stop_compression), Some(allow_sino_western_gap_adjustment), Some((source.adjustment).clone().line_adjustment));
        let custom_profile = ClreqProfile::new((source.id).to_ustring().as_ustr(), source.strictness, source.region, Some(source.punctuation_glyph_policy), Some((source.coalesce_repeatable_punctuation).clone()), Some((source.auto_space).clone()), Some(source.glue_placement), (custom_adjustment).clone(), (source.kinsoku_mode).clone(), (source.punctuation_width).clone());
        return WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_copy_annotation((annotation).clone(), Some((custom_profile).clone()), None, None);
    }

    pub fn width_independent_annotation_cache_coverage_test_support_with_first_font_decision_only(annotation: WidthIndependentParagraphAnnotation) -> WidthIndependentParagraphAnnotation {
        let mut first_only: Vec<FontDecision> = vec![];
        for i in 0..match u32::try_from(annotation.font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i == 0 {
                first_only.push((annotation.font_decisions[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_copy_annotation((annotation).clone(), None, Some((first_only).clone()), None);
    }

    pub fn width_independent_annotation_cache_coverage_test_support_with_empty_shaping_cache(annotation: WidthIndependentParagraphAnnotation) -> WidthIndependentParagraphAnnotation {
        return WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_copy_annotation((annotation).clone(), None, None, Some(SortedTable::sorted_table_map_builder::<TextRange, ShapingResult>(Arc::new(compare_text_range)).clone().build()));
    }

    pub fn width_independent_annotation_cache_coverage_test_support_non_gb_resolver() -> Box<dyn ClreqProfileResolver> {
        return Box::new(TiqianClreqFixedResolver::new((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_TAIWAN_HORIZONTAL).clone()));
    }

    pub fn width_independent_annotation_cache_coverage_test_support_ruby(range: TextRange, text: &UStr, kind: RubyKind, locale: Option<UString>) -> RubySpan {
        return RubySpan::new((range).clone(), text, Some(vec![]), kind, locale.clone());
    }

    pub fn width_independent_annotation_cache_coverage_test_support_boundary_shrink(shrink_capacity: f64) -> Result<InlineObjectBoundaryAdjustment, TextRangeError> {
        return Ok(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(shrink_capacity), Some(0.0f64), Some(false))?);
    }

    pub fn width_independent_annotation_cache_coverage_test_support_annotation_for_text(label: &UStr) -> Result<WidthIndependentParagraphAnnotation, WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault> {
        let input = LayoutInput::new(TiqianTextContent::new(label, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(300 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        return Ok(WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_engine(None, None).map_err(|e| WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::ParagraphLayoutEngineNewFaultFault(e))?, (input).clone(), WidthIndependentAnnotationCacheCoverageTestSupport::width_independent_annotation_cache_coverage_test_support_empty_tiers()).map_err(|e| WidthIndependentAnnotationCacheCoverageTestSupportAnnotationForTextFault::ParagraphShapingStageShapeParagraphFaultFault(e))?);
    }

    pub fn width_independent_annotation_cache_coverage_test_support_text_span_list(text: &UStr, font_size: f64) -> Result<Vec<TextSpan>, TextRangeError> {
        let mut spans: Vec<TextSpan> = vec![];
        for i in 0..match u32::try_from(u_string::unit_count(&(text))) { Ok(value) => value, Err(_) => u32::MAX } {
            spans.push(TextSpan::new(TextRange::new(i, u32::wrapping_add(i, 1))?, TextStyle::new(Some(vec![]), Some(font_size), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))));
        }
        return Ok(spans);
    }

    pub fn width_independent_annotation_cache_coverage_test_support_index_of(text: &UStr, needle: &UStr) -> u32 {
        let nl = u_string::unit_count(&(needle));
        let limit = u32::wrapping_add(u32::wrapping_sub(u_string::unit_count(&(text)), nl), 1);
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((limit) as i32).to_ne_bytes())) {
            let mut r#match = true;
            let mut j = 0u32;
            while (i32::from_ne_bytes(((j) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((nl) as i32).to_ne_bytes())) {
                if u_string::at(text, u32::wrapping_add(i, j)) != u_string::at(needle, j) {
                    r#match = false;
                    break;
                }
                j = u32::wrapping_add(j, 1);
            }
            if r#match {
                return i;
            }
            i = u32::wrapping_add(i, 1);
        }
        return 4294967295u32;
    }
}

#[derive(Clone, PartialEq)]
pub struct TiqianClreqFixedResolver {
    pub(crate) profile: ClreqProfile,
}

impl TiqianClreqFixedResolver {
    pub fn new(p: ClreqProfile) -> Self {
        Self {
            profile: p,
        }
    }

    pub fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        return ((self.profile).clone()).clone();
    }
}

impl ClreqProfileResolver for TiqianClreqFixedResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTestSupport.TiqianClreqFixedResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        return ((self.profile).clone()).clone();
    }
}

#[derive(Clone, PartialEq)]
pub struct ConflictingOpenTypeFeaturesShaper {
}

impl ConflictingOpenTypeFeaturesShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let cluster = Cluster::new((input.range).clone(), u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes())).as_ustr(), &(UStr::new(&[116,101,115,116])), 16.0f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let glyph1 = Glyph::new(1u32, (input.range).clone(), 8.0f64, Some(0.0f64), Some(0.0), None, None, None, None);
        let glyph2 = Glyph::new(2u32, (input.range).clone(), 8.0f64, Some(8.0f64), Some(0.0), None, None, None, None);
        let run1 = GlyphRun::new((input.range).clone(), &(UStr::new(&[116,101,115,116])), vec![(glyph1).clone()].to_vec(), 8.0f64, Some(vec![UString::from("feat1").to_ustring()]));
        let run2 = GlyphRun::new((input.range).clone(), &(UStr::new(&[116,101,115,116])), vec![(glyph2).clone()].to_vec(), 8.0f64, Some(vec![UString::from("feat2").to_ustring()]));
        return Ok(ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![(run1).clone(), (run2).clone()].to_vec(), Some(vec![])));
    }
}

impl ITextShaper for ConflictingOpenTypeFeaturesShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTestSupport.ConflictingOpenTypeFeaturesShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let cluster = Cluster::new((input.range).clone(), u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes())).as_ustr(), &(UStr::new(&[116,101,115,116])), 16.0f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let glyph1 = Glyph::new(1u32, (input.range).clone(), 8.0f64, Some(0.0f64), Some(0.0), None, None, None, None);
        let glyph2 = Glyph::new(2u32, (input.range).clone(), 8.0f64, Some(8.0f64), Some(0.0), None, None, None, None);
        let run1 = GlyphRun::new((input.range).clone(), &(UStr::new(&[116,101,115,116])), vec![(glyph1).clone()].to_vec(), 8.0f64, Some(vec![UString::from("feat1").to_ustring()]));
        let run2 = GlyphRun::new((input.range).clone(), &(UStr::new(&[116,101,115,116])), vec![(glyph2).clone()].to_vec(), 8.0f64, Some(vec![UString::from("feat2").to_ustring()]));
        return Ok(ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![(run1).clone(), (run2).clone()].to_vec(), Some(vec![])));
    }
}

#[derive(Clone)]
pub struct NarrowInkShaper {
    pub(crate) delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl NarrowInkShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let r = self.delegate.lock().unwrap().shape((input).clone())?;
        let mut runs: Vec<GlyphRun> = vec![];
        for ri in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (r.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            let mut glyphs: Vec<Glyph> = vec![];
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                glyphs.push(Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some(Rect::new(4.0f64, 2.0f64, 12.0f64, 10.0f64)), g.halt_advance, g.halt_placement_x));
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), glyphs.to_vec(), run.advance, Some(vec![])));
        }
        return Ok(ShapingResult::new(r.clusters.to_vec(), runs.to_vec(), Some((r.decisions).clone())));
    }
}

impl ITextShaper for NarrowInkShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.WidthIndependentAnnotationCacheCoverageTestSupport.NarrowInkShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let r = self.delegate.lock().unwrap().shape((input).clone())?;
        let mut runs: Vec<GlyphRun> = vec![];
        for ri in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (r.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            let mut glyphs: Vec<Glyph> = vec![];
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                glyphs.push(Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some(Rect::new(4.0f64, 2.0f64, 12.0f64, 10.0f64)), g.halt_advance, g.halt_placement_x));
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), glyphs.to_vec(), run.advance, Some(vec![])));
        }
        return Ok(ShapingResult::new(r.clusters.to_vec(), runs.to_vec(), Some((r.decisions).clone())));
    }
}
