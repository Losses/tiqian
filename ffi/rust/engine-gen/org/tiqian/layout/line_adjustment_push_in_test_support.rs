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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentPushInTestSupportLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for LineAdjustmentPushInTestSupportLayoutFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentPushInTestSupportLayoutFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestSupportLayoutFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
    pub const LINE_ADJUSTMENT_PUSH_IN_TEST_SUPPORT_FIXTURE_TEXT: &UStr = unsafe { &*(&[0x5496u16, 0x5561u16, 0xFF08u16, 0x0063u16, 0x006Fu16, 0x0066u16, 0x0066u16, 0x0065u16, 0x0065u16, 0xFF09u16, 0x5728u16, 0x5341u16, 0x4E03u16, 0x4E16u16, 0x7EAAu16, 0x7ECFu16, 0x5A01u16, 0x5C3Cu16, 0x65AFu16, 0x4F20u16, 0x5165u16, 0x6B27u16, 0x6D32u16, 0x3002u16, 0x6700u16, 0x521Du16, 0x5B83u16, 0x88ABu16, 0x5F53u16, 0x4F5Cu16, 0x836Fu16, 0x7269u16, 0x51FAu16, 0x552Eu16, 0xFF0Cu16, 0x4EF7u16, 0x683Cu16, 0x9AD8u16, 0x5F97u16, 0x5413u16, 0x4EBAu16, 0xFF0Cu16, 0x771Fu16, 0x6B63u16, 0x8BA9u16, 0x5B83u16, 0x6D41u16, 0x884Cu16, 0x8D77u16, 0x6765u16, 0x7684u16, 0x662Fu16, 0x968Fu16, 0x540Eu16, 0x904Du16, 0x5730u16, 0x5F00u16, 0x82B1u16, 0x7684u16, 0x5496u16, 0x5561u16, 0x9986u16, 0x2014u16, 0x2014u16, 0x8BFBu16, 0x62A5u16, 0x3001u16, 0x8FA9u16, 0x8BBAu16, 0x3001u16, 0x4E0Bu16, 0x68CBu16, 0x3001u16, 0x5199u16, 0x4F5Cu16, 0x2014u16, 0x2014u16, 0x57CEu16, 0x5E02u16, 0x751Fu16, 0x6D3Bu16, 0x5FFDu16, 0x7136u16, 0x591Au16, 0x51FAu16, 0x4E00u16, 0x4E2Au16, 0x516Cu16, 0x5171u16, 0x5BA2u16, 0x5385u16, 0x3002u16, 0x610Fu16, 0x5927u16, 0x5229u16, 0x4EBAu16, 0x505Au16, 0x51FAu16, 0x4E86u16, 0x0020u16, 0x0065u16, 0x0073u16, 0x0070u16, 0x0072u16, 0x0065u16, 0x0073u16, 0x0073u16, 0x006Fu16, 0xFF0Cu16, 0x7EF4u16, 0x4E5Fu16, 0x7EB3u16, 0x4EBAu16, 0x5F80u16, 0x676Fu16, 0x91CCu16, 0x52A0u16, 0x5976u16, 0x6CB9u16, 0xFF0Cu16, 0x571Fu16, 0x8033u16, 0x5176u16, 0x4EBAu16, 0x575Au16, 0x6301u16, 0x8FDEu16, 0x6E23u16, 0x540Cu16, 0x716Eu16, 0x2026u16, 0x2026u16, 0x6BCFu16, 0x5EA7u16, 0x57CEu16, 0x5E02u16, 0x90FDu16, 0x76F8u16, 0x4FE1u16, 0x81EAu16, 0x5DF1u16, 0x624Bu16, 0x91CCu16, 0x90A3u16, 0x4E00u16, 0x676Fu16, 0x624Du16, 0x662Fu16, 0x6B63u16, 0x7EDFu16, 0x3002u16, 0x6709u16, 0x4EBAu16, 0x8BF4u16, 0xFF1Au16, 0x300Cu16, 0x5148u16, 0x6709u16, 0x5496u16, 0x5561u16, 0x9986u16, 0xFF0Cu16, 0x540Eu16, 0x6709u16, 0x542Fu16, 0x8499u16, 0x8FD0u16, 0x52A8u16, 0x300Du16, 0x3002u16, 0x8FD9u16, 0x8BDDu16, 0x8BF4u16, 0x5F97u16, 0x5938u16, 0x5F20u16, 0xFF0Cu16, 0x4F46u16, 0x4E5Fu16, 0x4E0Du16, 0x7B97u16, 0x592Au16, 0x79BBu16, 0x8C31u16, 0x3002u16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    pub fn line_adjustment_push_in_test_support_cluster(i: u32, text: &UStr, advance: f64) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, text, &(UStr::new(&[116,101,115,116])), advance, Some(text.to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn line_adjustment_push_in_test_support_ints(a: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
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
        return SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build();
    }

    pub fn line_adjustment_push_in_test_support_base_clusters() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(0, UStr::new(&[30002]), 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(1, UStr::new(&[20057]), 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(2, UStr::new(&[19993]), 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(3, UStr::new(&[19969]), 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(4, UStr::new(&[25098]), 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(5, UStr::new(&[24049]), 20 as f64)?).clone(),
]);
    }

    pub fn line_adjustment_push_in_test_support_forbidden_head_start_clusters() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(0, UStr::new(&[30002]), 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(1, UStr::new(&[20057]), 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(2, UStr::new(&[21183]), 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(3, UStr::new(&[12290]), 10 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(4, UStr::new(&[21518]), 50 as f64)?).clone(),
]);
    }

    pub fn line_adjustment_push_in_test_support_forbidden_head_end_clusters() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(0, UStr::new(&[30002]), 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(1, UStr::new(&[20057]), 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(2, UStr::new(&[12300]), 10 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(3, UStr::new(&[23433]), 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(4, UStr::new(&[35013]), 20 as f64)?).clone(),
]);
    }

    pub fn line_adjustment_push_in_test_support_technical_clusters() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(0, UStr::new(&[97]), 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(1, UStr::new(&[32]), 20 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(2, UStr::new(&[82]), 30 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(3, UStr::new(&[101]), 15 as f64)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_cluster(4, UStr::new(&[108]), 15 as f64)?).clone(),
]);
    }

    pub fn line_adjustment_push_in_test_support_fill(cs: &Vec<Cluster>, width: f64, shrink: Option<Vec<ShrinkOpportunity>>, starts: Option<Vec<u32>>, ends: Option<Vec<u32>>, progressive: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>, split_at: Option<u32>) -> Result<Vec<LineCandidate>, TextRangeError> {
        let split = match &(split_at) { None => 1, Some(__option) => *__option };
        let a = vec![
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_line(IntRange::new(0u32, split), &cs)?).clone(),
    (LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_line(IntRange::new(u32::wrapping_add(split, 1), u32::wrapping_sub(u32::try_from((cs.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)), &cs)?).clone(),
];
        let mut gaps_arr: Vec<u32> = vec![];
        for i in 0..u32::try_from((cs.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
            gaps_arr.push(i);
        }
        return Ok(LineRepair::line_repair_apply_fill_push_in(&a, &cs, &cs, width, &match &(shrink) { None => vec![], Some(__option5) => (*__option5).clone() }, 0 as f64, 1000000 as f64, match &(starts) { None => None, Some(__option6) => Some(LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_ints(__option6)) }, LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_ints(&match &(ends) { None => vec![], Some(__option9) => (*__option9).clone() }), LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_empty_ranges(), 2, Some(LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_ints(&gaps_arr)), (progressive).clone())?);
    }

    pub fn line_adjustment_push_in_test_support_repair_total_shrink(o: Option<RepairOption>) -> f64 {
        if o == None {
            return i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64;
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4, .. } => _p4,
            RepairOption::Hang { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
            RepairOption::CarryPrevious { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
            RepairOption::CarryNext { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
            RepairOption::LeaveRagged { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
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

    pub fn line_adjustment_push_in_test_support_repair_reason(o: Option<RepairOption>) -> UString {
        if o == None {
            return UString::new();
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
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new((resolver).clone())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?;
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(LineAdjustmentPushInTestSupport::LINE_ADJUSTMENT_PUSH_IN_TEST_SUPPORT_FIXTURE_TEXT.to_ustring().as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn line_adjustment_push_in_test_support_fill_push_in_count(r: LayoutResult) -> u32 {
        let mut count = 0u32;
        for i in 0..match u32::try_from((r.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().line_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            match &(d.repair_decision) {
                Some(__option10) => {
                    if __option10.reason_code.to_ustring() == UString::from("LineAdjustmentPushIn") {
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
        return ClreqProfile::new((base.id).to_ustring().as_ustr(), base.strictness, base.region, Some(base.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((base.auto_space).clone()), Some(base.glue_placement), AdjustmentStylePolicy::new(Some(LineEndPunctuationStyle::ForceHalfWidth), Some(true), Some(true), Some(self.strategy)), (base.kinsoku_mode).clone(), (base.punctuation_width).clone());
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
        return ClreqProfile::new((base.id).to_ustring().as_ustr(), base.strictness, base.region, Some(base.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((base.auto_space).clone()), Some(base.glue_placement), AdjustmentStylePolicy::new(Some(LineEndPunctuationStyle::ForceHalfWidth), Some(true), Some(true), Some(self.strategy)), (base.kinsoku_mode).clone(), (base.punctuation_width).clone());
    }
}
