use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::emergency_tracking_eligibility_decision_info::EmergencyTrackingEligibilityDecisionInfo;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_break_planning_stage::LineBreakPlanningStage;
use crate::org::tiqian::layout::line_break_planning_stage::LineBreakPlanningStageResult;
use crate::org::tiqian::layout::line_break_planning_stage::ParagraphLayoutPrep;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheFns;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestSupportEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestSupportEngineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestSupportEngineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestSupportEngineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestSupportEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestSupportEngineFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestSupportEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestSupportEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverage2TestSupportEngineFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestSupportEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestSupportEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestSupportEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverage2TestSupportEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverage2TestSupportEngineFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestSupportPrepFault {
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestSupportPrepFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestSupportPrepFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestSupportPrepFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestSupportPrepFault::ParagraphShapingStageShapeParagraphFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestSupportPrepFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestSupportPrepFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestSupportPrepFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestSupportPrepFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestSupportPrepFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestSupportPrepFault> for crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestSupportPrepFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestSupportPrepFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestSupportPrepFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestSupportPrepFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestSupportPrepFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverage2TestSupportPrepFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        LineBreakPlanningStageCoverage2TestSupportPrepFault::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        LineBreakPlanningStageCoverage2TestSupportPrepFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        LineBreakPlanningStageCoverage2TestSupportPrepFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestSupportPrepFault::TextRangeErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestSupportPlanFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestSupportPlanFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestSupportPlanFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestSupportPlanFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestSupportPlanFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestSupportPlanFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestSupportPlanFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestSupportPlanFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverage2TestSupportPlanFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestSupportPlanFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestSupportPlanFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestSupportPlanFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverage2TestSupportPlanFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverage2TestSupportPlanFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct LineBreakPlanningStageCoverage2TestSupport;

impl LineBreakPlanningStageCoverage2TestSupport {
    pub fn line_break_planning_stage_coverage2_test_support_engine() -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn line_break_planning_stage_coverage2_test_support_layout(text: &UStr, width: f64, objects: Option<Vec<InlineObjectSpan>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let mut e = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_engine()?;
        return Ok(e.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), (objects).clone())).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn line_break_planning_stage_coverage2_test_support_input(text: &UStr) -> Result<LayoutInput, TextRangeError> {
        return Ok(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
    }

    pub fn line_break_planning_stage_coverage2_test_support_prep(text: &UStr) -> Result<ParagraphLayoutPrep, LineBreakPlanningStageCoverage2TestSupportPrepFault> {
        let mut e = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_engine().map_err(|e| LineBreakPlanningStageCoverage2TestSupportPrepFault::ParagraphLayoutEngineNewFaultFault(e))?;
        let i = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_input(text).map_err(|e| LineBreakPlanningStageCoverage2TestSupportPrepFault::TextRangeErrorFault(e))?;
        let z: SortedMapTable<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range)).clone().build();
        let a = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_prepare_width_independent_annotation(&mut e, (i).clone(), (z).clone()).map_err(|e| LineBreakPlanningStageCoverage2TestSupportPrepFault::ParagraphShapingStageShapeParagraphFaultFault(e))?;
        return Ok(WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_build_paragraph_layout_prep(&mut e, (i).clone(), (a).clone(), (z).clone()).map_err(|e| LineBreakPlanningStageCoverage2TestSupportPrepFault::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFaultFault(e))?);
    }

    pub fn line_break_planning_stage_coverage2_test_support_with_prep(p: ParagraphLayoutPrep, natural: Option<Vec<Cluster>>, clusters: Option<Vec<Cluster>>, fonts: Option<Vec<FontDecision>>, offsets: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>, elig: Option<Vec<EmergencyTrackingEligibilityDecisionInfo>>, uniform: Option<SortedSetTable<u32>>, atoms: Option<SortedMapTable<TextRange, PunctuationClass>>) -> ParagraphLayoutPrep {
        let nc = match &(natural) { None => (p.natural_clusters).clone(), Some(__option) => (*__option).clone() };
        let cr = match &(clusters) { None => (p.clusters).clone(), Some(__option1) => (*__option1).clone() };
        let mut roles = p.cluster_roles.clone();
        let mut edges = p.east_asian_spacing_edges.clone();
        let mut attachments = p.natural_inline_attachments.clone();
        if match &(natural) { Some(__option3) => u32::try_from((nc.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((p.natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None => false } {
            roles = vec![];
            edges = vec![];
            attachments = vec![];
            for _ in 0..match u32::try_from(nc.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                roles.push(FontRole::LatinText);
                edges.push(EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false));
                attachments.push(InlineAttachment::None);
            }
        }
        return ParagraphLayoutPrep::new((p.input).clone(), (p.rejected_technical_tiers_by_span).clone(), (p.text).to_ustring().as_ustr(), p.font_size, (p.style_at).clone(), (p.font_size_at).clone(), (p.bopomofo_font_weight_at).clone(), p.ruby_font_size, p.ruby_stack_gap, p.ruby_font_weight, p.pinyin_spans.to_vec(), (p.clreq_profile).clone(), (p.punctuation_glyph_substitutor).clone(), p.measure, p.measure_em, p.grid_body_offset, (p.line_length_grid_decision).clone(), p.quote_pairs.to_vec(), p.role_override_infos.to_vec(), match &(fonts) { None => (p.font_decisions).clone(), Some(__option4) => (*__option4).clone() }.to_vec(), (p.hyphen_offsets).clone(), p.hyphen_advance, p.hyphen_glyphs.to_vec(), (p.substitution_rollbacks).clone(), p.break_opportunity_decisions.to_vec(), match &(elig) { None => (p.emergency_tracking_eligibility_decisions).clone(), Some(__option5) => (*__option5).clone() }.to_vec(), match &(offsets) { None => (p.progressive_break_offsets).clone(), Some(__option6) => (*__option6).clone() }, (p.shaped_glyphs_by_cluster_range).clone(), (p.open_type_features_by_cluster_range).clone(), p.shaping_decisions.to_vec(), edges.to_vec(), p.auto_space_decisions.to_vec(), (p.inline_box_result).clone(), (nc).clone(), (p.inline_object_by_cluster_index).clone(), match &(uniform) { None => (p.uniform_inline_object_boundary_after_clusters).clone(), Some(__option7) => (*__option7).clone() }, (p.preferred_inline_object_boundary_after_clusters).clone(), p.inline_object_boundary_unbreakable_ranges.to_vec(), roles.to_vec(), (p.resolved_kinsoku).clone(), (p.kinsoku_rule).clone(), p.inline_object_attached_marks.to_vec(), (p.inline_object_separator_space_trims).clone(), (p.inline_object_attachment_no_stretch_boundaries).clone(), p.inline_object_punctuation_attachment_decisions.to_vec(), (p.mandatory_break_clusters).clone(), (p.zero_width_break_clusters).clone(), p.mandatory_break_decisions.to_vec(), p.zero_width_break_decisions.to_vec(), p.punctuation_atoms.to_vec(), (p.spacing_plan).clone(), (p.ruby_font_geometry_by_span).clone(), (p.ruby_and_bopomofo_spread).clone(), attachments.to_vec(), (p.attached_punctuation_boundary).clone(), (p.base_geometry).clone(), (p.attached_punctuation_trailing_glue_by_cluster).clone(), (cr).clone(), (p.adjustment_style).clone(), match &(atoms) { None => (p.atom_class_by_range).clone(), Some(__option8) => (*__option8).clone() }, p.shrink_opportunities.to_vec());
    }

    pub fn line_break_planning_stage_coverage2_test_support_plan(p: ParagraphLayoutPrep) -> Result<LineBreakPlanningStageResult, LineBreakPlanningStageCoverage2TestSupportPlanFault> {
        return Ok(LineBreakPlanningStage::line_break_planning_stage_plan_paragraph_lines(LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_engine().map_err(|e| LineBreakPlanningStageCoverage2TestSupportPlanFault::ParagraphLayoutEngineNewFaultFault(e))?, (p).clone()).map_err(|e| LineBreakPlanningStageCoverage2TestSupportPlanFault::TextRangeErrorFault(e))?);
    }

    pub fn line_break_planning_stage_coverage2_test_support_map_opp() -> Result<SortedMapTable<u32, ProgressiveBreakOpportunity>, TextRangeError> {
        let mut b: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        b.put(&(999), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Whitespace, TextRange::new(0u32, 3u32)?, Some(0.0))));
        return Ok(b.clone().build());
    }

    pub fn line_break_planning_stage_coverage2_test_support_map_atom() -> Result<SortedMapTable<TextRange, PunctuationClass>, TextRangeError> {
        let mut b: SortedMapTableBuilder<TextRange, PunctuationClass> = SortedTable::sorted_table_map_builder::<TextRange, PunctuationClass>(Arc::new(compare_text_range));
        b.put(&(TextRange::new(0u32, 1u32)?), &(PunctuationClass::Dash));
        b.put(&(TextRange::new(2u32, 3u32)?), &(PunctuationClass::Connector));
        return Ok(b.clone().build());
    }

    pub fn line_break_planning_stage_coverage2_test_support_set_uniform() -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        b.put(&(0));
        b.put(&(1));
        b.put(&(3));
        return b.clone().build();
    }
}
