use crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphFns;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;


#[derive(Clone, Copy)]
pub struct PreparedParagraphJsonNumberTestSupport;

impl PreparedParagraphJsonNumberTestSupport {
    pub fn prepared_paragraph_json_number_test_support_rec(name: &str) {
        TestTraceRecorder::new("PreparedParagraphJsonNumberTest").section(name);
    }

    pub fn prepared_paragraph_json_number_test_support_eq(expected: &str, value: f64) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_string(expected, PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(value).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str(), None)?;
        Ok(())
    }
}
