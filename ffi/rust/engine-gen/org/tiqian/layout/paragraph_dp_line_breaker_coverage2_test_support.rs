use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::paragraph_dp_line_breaker_test_support::ParagraphDpLineBreakerTestSupport;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::sorted_table::SortedMapTable;


#[derive(Clone, Copy)]
pub struct ParagraphDpLineBreakerCoverage2TestSupport;

impl ParagraphDpLineBreakerCoverage2TestSupport {
    pub fn paragraph_dp_line_breaker_coverage2_test_support_rec(n: &str) {
        TestTraceRecorder::new("ParagraphDpLineBreakerCoverage2Test").section(n);
    }

    pub fn paragraph_dp_line_breaker_coverage2_test_support_solve(c: &Vec<Cluster>, width: f64, shrink: Option<Vec<ShrinkOpportunity>>, hard: Option<Vec<u32>>, push: Option<bool>, ranges: Option<UnbreakableRanges>, progressive: Option<SortedMapTable<u32,
ProgressiveBreakOpportunity>>, window: Option<u32>) -> Result<LineSolution, TextRangeError> {
        return Ok(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve(&c, width, (shrink).clone(), (hard).clone(), push, (ranges).clone(), (progressive).clone(), window, None, None, None)?);
    }

    pub fn paragraph_dp_line_breaker_coverage2_test_support_han(n: u32, a: Option<f64>) -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(n, a)?);
    }

    pub fn paragraph_dp_line_breaker_coverage2_test_support_latin() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_latin()?);
    }

    pub fn paragraph_dp_line_breaker_coverage2_test_support_opp(v: &Vec<u32>, spans: &Vec<TextRange>, tiers: &Vec<ProgressiveBreakTier>) -> SortedMapTable<u32, ProgressiveBreakOpportunity> {
        return ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_opportunities(&v, &spans, &tiers);
    }
}
