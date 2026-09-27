use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::u_string::UStr;


#[derive(Clone, Copy)]
pub struct QuotePairAnalyzerSurrogateAdjacencyTestSupport;

impl QuotePairAnalyzerSurrogateAdjacencyTestSupport {
    pub fn quote_pair_analyzer_surrogate_adjacency_test_support_rec(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[81,117,111,116,101,80,97,105,114,65,110,97,108,121,122,101,114,83,117,114,114,111,103,97,116,101,65,100,106,97,99,101,110,99,121,84,101,115,116]))).section(n);
    }
}
