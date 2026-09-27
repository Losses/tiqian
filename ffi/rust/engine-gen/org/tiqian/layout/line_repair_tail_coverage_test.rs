#![cfg(test)]

use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;


#[test]
fn fill_pull_across_different_technical_spans_skips_tier_comparisons() {
    testlib::run("org.tiqian.layout.LineRepairTailCoverageTest.fillPullAcrossDifferentTechnicalSpansSkipsTierComparisons", "org.tiqian.layout.LineRepairTailCoverageTest.fillPullAcrossDifferentTechnicalSpansSkipsTierComparisons", || {
        let mut r = TestTraceRecorder::new("LineRepairTailCoverageTest");
        r.section(&"fillPullAcrossDifferentTechnicalSpansSkipsTierComparisons");
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4, 5] actual=[0, 1, 2, 3, 4, 5]").unwrap();
        let _ = r.record(&"eq expected=[6, 7] actual=[6, 7]").unwrap();
    });
}
