#![cfg(test)]

use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[test]
fn fill_pull_across_different_technical_spans_skips_tier_comparisons() {
    testlib::run("org.tiqian.layout.LineRepairTailCoverageTest.fillPullAcrossDifferentTechnicalSpansSkipsTierComparisons", "org.tiqian.layout.LineRepairTailCoverageTest.fillPullAcrossDifferentTechnicalSpansSkipsTierComparisons", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,82,101,112,97,105,114,84,97,105,108,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[102,105,108,108,80,117,108,108,65,99,114,111,115,115,68,105,102,102,101,114,101,110,116,84,101,99,104,110,105,99,97,108,83,112,97,110,115,83,107,105,112,115,84,105,101,114,67,111,109,112,97,114,105,115,111,110,115]));
        let _ = r.record(UStr::new(&[101,113,32,101,120,112,101,99,116,101,100,61,91,48,44,32,49,44,32,50,44,32,51,44,32,52,44,32,53,93,32,97,99,116,117,97,108,61,91,48,44,32,49,44,32,50,44,32,51,44,32,52,44,32,53,93])).unwrap();
        let _ = r.record(UStr::new(&[101,113,32,101,120,112,101,99,116,101,100,61,91,54,44,32,55,93,32,97,99,116,117,97,108,61,91,54,44,32,55,93])).unwrap();
    });
}
