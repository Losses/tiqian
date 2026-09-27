use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::u_string::UStr;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestSupportNonEmptyFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestSupportNonEmptyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestSupportNonEmptyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSupportNonEmptyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSupportNonEmptyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSupportNonEmptyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestSupportNonEmptyFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSupportNonEmptyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSupportNonEmptyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestSupportNonEmptyFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSupportNonEmptyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSupportNonEmptyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestSupportNonEmptyFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSupportNonEmptyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestSupportNonEmptyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestSupportNonEmptyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestSupportNonEmptyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestSupportNonEmptyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestSupportNonEmptyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestSupportNonEmptyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestSupportFailLowFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestSupportFailLowFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestSupportFailLowFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSupportFailLowFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSupportFailLowFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSupportFailLowFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: QuotePairAnalyzerCoverageTestSupportFailLowFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSupportFailLowFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSupportFailLowFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestSupportFailLowFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSupportFailLowFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSupportFailLowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: QuotePairAnalyzerCoverageTestSupportFailLowFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSupportFailLowFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for QuotePairAnalyzerCoverageTestSupportFailLowFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        QuotePairAnalyzerCoverageTestSupportFailLowFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestSupportFailLowFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestSupportFailLowFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for QuotePairAnalyzerCoverageTestSupportFailLowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        QuotePairAnalyzerCoverageTestSupportFailLowFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct QuotePairAnalyzerCoverageTestSupport;

impl QuotePairAnalyzerCoverageTestSupport {
    pub fn quote_pair_analyzer_coverage_test_support_rec(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[81,117,111,116,101,80,97,105,114,65,110,97,108,121,122,101,114,67,111,118,101,114,97,103,101,84,101,115,116]))).section(n);
    }

    pub fn quote_pair_analyzer_coverage_test_support_a() -> QuotePairAnalyzer {
        return QuotePairAnalyzer::new();
    }

    pub fn quote_pair_analyzer_coverage_test_support_non_empty(t: &UStr) -> Result<(), QuotePairAnalyzerCoverageTestSupportNonEmptyFault> {
        let d = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().classify_quote_roles(t, &vec![], None).map_err(|e| QuotePairAnalyzerCoverageTestSupportNonEmptyFault::TextRangeErrorFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).map_err(|e| QuotePairAnalyzerCoverageTestSupportNonEmptyFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn quote_pair_analyzer_coverage_test_support_fail_low(t: &UStr) -> Result<(), QuotePairAnalyzerCoverageTestSupportFailLowFault> {
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let t = (t).to_ustring(); Arc::new(move || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().classify_quote_roles(t.as_ustr(), &vec![], None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).map_err(|e| QuotePairAnalyzerCoverageTestSupportFailLowFault::TracedAssertionsAssertFailsWithFaultFault(e))?;
        Ok(())
    }
}
