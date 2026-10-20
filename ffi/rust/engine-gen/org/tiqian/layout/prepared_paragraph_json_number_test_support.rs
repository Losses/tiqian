use crate::org::tiqian::protocol::plan_json_number::PlanJsonNumber;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;
use crate::runtime::u_string::UStr;


#[derive(Clone, Copy)]
pub struct PreparedParagraphJsonNumberTestSupport;

impl PreparedParagraphJsonNumberTestSupport {
    pub fn prepared_paragraph_json_number_test_support_rec(name: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,74,115,111,110,78,117,109,98,101,114,84,101,115,116]))).section(name);
    }

    pub fn prepared_paragraph_json_number_test_support_eq(expected: &UStr, value: f64) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_string(expected, PlanJsonNumber::plan_json_number_ecma_json_number(value).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr(), None)?;
        Ok(())
    }
}
