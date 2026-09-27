use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;


#[derive(Clone, Copy)]
pub struct QuotePairAnalyzerSurrogateAdjacencyTestSupport;

impl QuotePairAnalyzerSurrogateAdjacencyTestSupport {
    pub fn quote_pair_analyzer_surrogate_adjacency_test_support_rec(n: &str) {
        TestTraceRecorder::new("QuotePairAnalyzerSurrogateAdjacencyTest").section(n);
    }
}
