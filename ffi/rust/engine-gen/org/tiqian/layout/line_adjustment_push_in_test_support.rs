use crate::org::tiqian::clreq::adjustment_style_policy::AdjustmentStylePolicy;
use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::line_adjustment_strategy::LineAdjustmentStrategy;
use crate::org::tiqian::clreq::line_end_punctuation_style::LineEndPunctuationStyle;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::PushInAllocation;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::line_repair::LineRepair;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentPushInTestSupportLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}

impl From<LineAdjustmentPushInTestSupportLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentPushInTestSupportLayoutFault) -> Self {
        match value {
            LineAdjustmentPushInTestSupportLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestSupportLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentPushInTestSupportLayoutFault) -> Self {
        match value {
            LineAdjustmentPushInTestSupportLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentPushInTestSupportLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentPushInTestSupportLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentPushInTestSupportLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentPushInTestSupportLayoutFault::UStringFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct LineAdjustmentPushInTestSupport;

impl LineAdjustmentPushInTestSupport {
    pub const LINE_ADJUSTMENT_PUSH_IN_TEST_SUPPORT_FIXTURE_TEXT: &str = "咖啡（coffee）在十七世纪经威尼斯传入欧洲。最初它被当作药物出售，价格高得吓人，真正让它流行起来的是随后遍地开花的咖啡馆——读报、辩论、下棋、写作——城市生活忽然多出一个公共客厅。意大利人做出了 espresso，维也纳人往杯里加奶油，土耳其人坚持连渣同煮……每座城市都相信自己手里那一杯才是正统。有人说：「先有咖啡馆，后有启蒙运动」。这话说得夸张，但也不算太离谱。";

    pub fn line_adjustment_push_in_test_support_cluster(i: u32, text: &str, advance: f64) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, text, "test", advance, Some(text.to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn line_adjustment_push_in_test_support_ints(a: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for &x in a {
            b.put(&(x));
        }
        return b.clone().build();
    }

    pub fn line_adjustment_push_in_test_support_line(r: IntRange, cs: &Vec<Cluster>) -> Result<LineCandidate, TextRangeError> {
        let mut w = 0.0f64;
        for i in r.start..u32::wrapping_add(r.end, 1) {
            w += cs[usize::try_from(i).unwrap_or(0)].advance;
        }
        return Ok(LineCandidate::new((r).clone(), TextRange::new(r.start, u32::wrapping_add(r.end, 1))?, w, w, Some(LineEndReason::AutoWrap), None, Some(vec![]), Some(LineCandidate::line_candidate_empty_hanging()))?);
    }

    pub fn line_adjustment_push_in_test_support_empty_ranges() -> UnbreakableRanges {
        return UnbreakableRanges::new(vec![].to_vec());
    }

    pub fn line_adjustment_push_in_test_support_empty_progressive() -> SortedMapTable<u32, ProgressiveBreakOpportunity> {
        return SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build();
    }

    pub fn line_adjustment_push_in_test_support_base_clusters() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(0, &"甲", 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(1, &"乙", 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(2, &"丙", 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(3, &"丁", 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(4, &"戊", 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(5, &"己", 20 as f64)?).clone(),
]);
    }

    pub fn line_adjustment_push_in_test_support_forbidden_head_start_clusters() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(0, &"甲", 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(1, &"乙", 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(2, &"势", 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(3, &"。", 10 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(4, &"后", 50 as f64)?).clone(),
]);
    }

    pub fn line_adjustment_push_in_test_support_forbidden_head_end_clusters() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(0, &"甲", 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(1, &"乙", 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(2, &"「", 10 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(3, &"安", 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(4, &"装", 20 as f64)?).clone(),
]);
    }

    pub fn line_adjustment_push_in_test_support_technical_clusters() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(0, &"a", 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(1, &" ", 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(2, &"R", 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(3, &"e", 15 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(4, &"l", 15 as f64)?).clone(),
]);
    }

    pub fn line_adjustment_push_in_test_support_fill(cs: &Vec<Cluster>, width: f64, shrink: Option<Vec<ShrinkOpportunity>>, starts: Option<Vec<u32>>, ends: Option<Vec<u32>>, progressive: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>, split_at: Option<u32>) ->
Result<Vec<LineCandidate>, TextRangeError> {
        let split = match &(split_at) { None => 1, Some(__option) => *__option };
        let a = vec![
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_line(IntRange::new(0u32, split), &cs)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_line(IntRange::new(u32::wrapping_add(split, 1), u32::wrapping_sub(u32::try_from((cs.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)), &cs)?).clone(),
];
        let mut gaps_arr: Vec<u32> = vec![];
        for i in 0..u32::try_from((cs.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
            gaps_arr.push(i);
        }
        return Ok(LineRepair::line_repair_apply_fill_push_in(&a, &cs, &cs, width, &match &(shrink) { None => vec![], Some(__option5) => (*__option5).clone() }, 0 as f64, 1000000 as f64, match &(starts) { None => None, Some(__option6) =>
Some(LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_ints(__option6)) }, LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_ints(&match &(ends) { None => vec![], Some(__option9) => (*__option9).clone() }),
LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_empty_ranges(), 2, Some(LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_ints(&gaps_arr)), (progressive).clone())?);
    }

    pub fn line_adjustment_push_in_test_support_repair_total_shrink(o: Option<RepairOption>) -> f64 {
        if o == None {
            return i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64;
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4, .. } => _p4,
            RepairOption::Hang { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
            RepairOption::CarryPrevious { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
            RepairOption::CarryNext { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
            RepairOption::LeaveRagged { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
        };
    }

    pub fn line_adjustment_push_in_test_support_repair_offender_index(o: Option<RepairOption>) -> u32 {
        if o == None {
            return 4294967295u32;
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, .. } => _p2,
            RepairOption::Hang { .. } => 4294967295u32,
            RepairOption::CarryPrevious { .. } => 4294967295u32,
            RepairOption::CarryNext { .. } => 4294967295u32,
            RepairOption::LeaveRagged { .. } => 4294967295u32,
        };
    }

    pub fn line_adjustment_push_in_test_support_repair_allocations(o: Option<RepairOption>) -> Vec<PushInAllocation> {
        if o == None {
            return vec![];
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, .. } => _p3,
            RepairOption::Hang { .. } => vec![],
            RepairOption::CarryPrevious { .. } => vec![],
            RepairOption::CarryNext { .. } => vec![],
            RepairOption::LeaveRagged { .. } => vec![],
        };
    }

    pub fn line_adjustment_push_in_test_support_repair_reason(o: Option<RepairOption>) -> String {
        if o == None {
            return String::new();
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, .. } => _p1,
            RepairOption::Hang { penalty: _p0, reason: _p1, .. } => _p1,
            RepairOption::CarryPrevious { penalty: _p0, reason: _p1, .. } => _p1,
            RepairOption::CarryNext { penalty: _p0, reason: _p1, .. } => _p1,
            RepairOption::LeaveRagged { penalty: _p0, reason: _p1, .. } => _p1,
        };
    }

    pub fn line_adjustment_push_in_test_support_layout(strategy: LineAdjustmentStrategy) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let resolver = PushInClreqResolver::new(strategy);
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new((resolver).clone())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()),
Some(QuotePairAnalyzer::new()), Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Justifier::new(Some(0.5), Some(0.25))),
Some(Box::new(ExplainableStubTextShaper::new())), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?;
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(LineAdjustmentPushInTestSupport::LINE_ADJUSTMENT_PUSH_IN_TEST_SUPPORT_FIXTURE_TEXT.to_string().as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320.0f64,
Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn line_adjustment_push_in_test_support_fill_push_in_count(r: LayoutResult) -> u32 {
        let mut count = 0u32;
        for i in 0..match u32::try_from((r.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().line_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            match &(d.repair_decision) {
                Some(__option10) => {
                    if __option10.reason_code.to_string() == "LineAdjustmentPushIn" {
                    count = u32::wrapping_add(count, 1);
                    }
                }
                None => {
                }
            }
        }
        return count;
    }
}

#[derive(Clone, PartialEq)]
pub struct PushInClreqResolver {
    pub(crate) strategy: LineAdjustmentStrategy,
}

impl PushInClreqResolver {
    pub fn new(strategy: LineAdjustmentStrategy) -> Self {
        Self {
            strategy,
        }
    }

    pub fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        let base = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((base.id).to_string().as_str(), base.strictness, base.region, Some(base.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((base.auto_space).clone()),
Some(base.glue_placement), AdjustmentStylePolicy::new(Some(LineEndPunctuationStyle::ForceHalfWidth), Some(true), Some(true), Some(self.strategy)), (base.kinsoku_mode).clone(), (base.punctuation_width).clone());
    }
}

impl ClreqProfileResolver for PushInClreqResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.LineAdjustmentPushInTestSupport.PushInClreqResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        let base = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((base.id).to_string().as_str(), base.strictness, base.region, Some(base.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((base.auto_space).clone()),
Some(base.glue_placement), AdjustmentStylePolicy::new(Some(LineEndPunctuationStyle::ForceHalfWidth), Some(true), Some(true), Some(self.strategy)), (base.kinsoku_mode).clone(), (base.punctuation_width).clone());
    }
}
