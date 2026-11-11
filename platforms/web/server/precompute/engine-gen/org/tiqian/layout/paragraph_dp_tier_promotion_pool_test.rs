#![cfg(test)]

use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::paragraph_dp_line_breaker::ParagraphDpLineBreaker;
use crate::org::tiqian::layout::paragraph_dp_line_breaker_test_support::ParagraphDpLineBreakerTestSupport;
use crate::org::tiqian::layout::paragraph_dp_tier_promotion_pool_test_support::ParagraphDpTierPromotionPoolTestSupport;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault) -> Self {
        match value {
            ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault) -> Self {
        match value {
            ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault) -> Self {
        match value {
            ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpTierPromotionPoolTestForeignSpanCandidateSurvivesThePromotionPoolPurgeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault) -> Self {
        match value {
            ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault) -> Self {
        match value {
            ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault) -> Self {
        match value {
            ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpTierPromotionPoolTestCommittedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReasonFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault) -> Self {
        match value {
            ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault) -> Self {
        match value {
            ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault) -> Self {
        match value {
            ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpTierPromotionPoolTestCommittedCompressedEndWithoutOpportunityKeepsPlainPushInReasonFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn foreign_span_candidate_survives_the_promotion_pool_purge() {
    testlib::run("org.tiqian.layout.ParagraphDpTierPromotionPoolTest.foreignSpanCandidateSurvivesThePromotionPoolPurge", "org.tiqian.layout.ParagraphDpTierPromotionPoolTest.foreignSpanCandidateSurvivesThePromotionPoolPurge", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,68,112,84,105,101,114,80,114,111,109,111,116,105,111,110,80,111,111,108,84,101,115,116])));
        t.section(UStr::new(&[102,111,114,101,105,103,110,83,112,97,110,67,97,110,100,105,100,97,116,101,83,117,114,118,105,118,101,115,84,104,101,80,114,111,109,111,116,105,111,110,80,111,111,108,80,117,114,103,101]));
        let c = ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_latin_clusters().unwrap();
        let span = TextRange::new(0u32, u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).unwrap();
        let s = ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).unwrap().break_lines(&c, &c, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 2u32, 5 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, None, None, None, None, None, None, None, None, None, None, Some(true), None, None, None, Some(ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_opp(&vec![1, 2, 3], &vec![
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, TextRange::new(0u32, 1u32).unwrap(), Some(0.0))).clone(),
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, (span).clone(), Some(0.0))).clone(),
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Whitespace, (span).clone(), Some(0.0))).clone(),
]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((u_string::find_from(&(ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_repair_reason(((s.lines[0usize]).clone().repair).clone())), UString::from("ProgressiveTechnicalTierPromotion").as_ustr(), 0)) as u32).to_ne_bytes()) == 0, Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_repairs_string((s).clone())).to_ustring())).unwrap();
    });
}

#[test]
fn committed_compressed_line_with_foreign_span_opportunities_keeps_plain_push_in_reason() {
    testlib::run("org.tiqian.layout.ParagraphDpTierPromotionPoolTest.committedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReason", "org.tiqian.layout.ParagraphDpTierPromotionPoolTest.committedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReason", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,68,112,84,105,101,114,80,114,111,109,111,116,105,111,110,80,111,111,108,84,101,115,116])));
        t.section(UStr::new(&[99,111,109,109,105,116,116,101,100,67,111,109,112,114,101,115,115,101,100,76,105,110,101,87,105,116,104,70,111,114,101,105,103,110,83,112,97,110,79,112,112,111,114,116,117,110,105,116,105,101,115,75,101,101,112,115,80,108,97,105,110,80,117,115,104,73,110,82,101,97,115,111,110]));
        let c = ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_han_clusters(4).unwrap();
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve(&c, 44 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 1u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(true))).clone(),
]), None, Some(true), None, Some(ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_opp(&vec![2, 3], &vec![
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, TextRange::new(0u32, 2u32).unwrap(), Some(0.0))).clone(),
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Whitespace, TextRange::new(2u32, 4u32).unwrap(), Some(0.0))).clone(),
])), None, Some(vec![1]), Some(8 as f64 as f64), None).unwrap();
        let mut _g: Vec<Option<RepairOption>> = vec![];
        {
            let _g2 = s.lines.clone();
            for l in &_g2 {
                _g.push((l.repair).clone());
            }
        }
        let _ = (_g).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(), Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_ustring())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((u_string::find_from(&(ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_repair_reason(((s.lines[0usize]).clone().repair).clone())), UString::from("LineAdjustmentPushIn").as_ustr(), 0)) as u32).to_ne_bytes()) == 0, Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_repairs_string((s).clone())).to_ustring())).unwrap();
    });
}

#[test]
fn committed_compressed_end_without_opportunity_keeps_plain_push_in_reason() {
    testlib::run("org.tiqian.layout.ParagraphDpTierPromotionPoolTest.committedCompressedEndWithoutOpportunityKeepsPlainPushInReason", "org.tiqian.layout.ParagraphDpTierPromotionPoolTest.committedCompressedEndWithoutOpportunityKeepsPlainPushInReason", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,68,112,84,105,101,114,80,114,111,109,111,116,105,111,110,80,111,111,108,84,101,115,116])));
        t.section(UStr::new(&[99,111,109,109,105,116,116,101,100,67,111,109,112,114,101,115,115,101,100,69,110,100,87,105,116,104,111,117,116,79,112,112,111,114,116,117,110,105,116,121,75,101,101,112,115,80,108,97,105,110,80,117,115,104,73,110,82,101,97,115,111,110]));
        let c = ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_han_clusters(4).unwrap();
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve(&c, 44 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 1u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(true))).clone(),
]), None, Some(true), None, Some(ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_opp(&vec![2], &vec![
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, TextRange::new(0u32, 2u32).unwrap(), Some(0.0))).clone(),
])), None, Some(vec![1]), Some(8 as f64 as f64), None).unwrap();
        let mut _g: Vec<Option<RepairOption>> = vec![];
        {
            let _g2 = s.lines.clone();
            for l in &_g2 {
                _g.push((l.repair).clone());
            }
        }
        let _ = (_g).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(), Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_ustring())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((u_string::find_from(&(ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_repair_reason(((s.lines[0usize]).clone().repair).clone())), UString::from("LineAdjustmentPushIn").as_ustr(), 0)) as u32).to_ne_bytes()) == 0, Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_repairs_string((s).clone())).to_ustring())).unwrap();
    });
}
