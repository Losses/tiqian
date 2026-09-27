use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteRoleDecision;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestSupportRoleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestSupportRoleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestSupportRoleFault) -> Self {
        match value {
            QuotePairAnalyzerTestSupportRoleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestSupportRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestSupportRoleFault) -> Self {
        match value {
            QuotePairAnalyzerTestSupportRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestSupportRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestSupportRoleFault) -> Self {
        match value {
            QuotePairAnalyzerTestSupportRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestSupportRoleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestSupportRoleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestSupportRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestSupportRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestSupportRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestSupportRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestSupportPairRoleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestSupportPairRoleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestSupportPairRoleFault) -> Self {
        match value {
            QuotePairAnalyzerTestSupportPairRoleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestSupportPairRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestSupportPairRoleFault) -> Self {
        match value {
            QuotePairAnalyzerTestSupportPairRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestSupportPairRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestSupportPairRoleFault) -> Self {
        match value {
            QuotePairAnalyzerTestSupportPairRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestSupportPairRoleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestSupportPairRoleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestSupportPairRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestSupportPairRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestSupportPairRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestSupportPairRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct QuotePairAnalyzerTestSupport;

impl QuotePairAnalyzerTestSupport {
    pub fn quote_pair_analyzer_test_support_rec(name: &str) {
        TestTraceRecorder::new("QuotePairAnalyzerTest").section(name);
    }

    pub fn quote_pair_analyzer_test_support_a() -> QuotePairAnalyzer {
        return QuotePairAnalyzer::new();
    }

    pub fn quote_pair_analyzer_test_support_sig(text: &str, roles: SortedMapTable<u32, FontRole>) -> String {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut out = String::new();
        let mut i = 0u32;
        let __units1 = u_string::units(&text);
        let __count1 = u_string::unit_count(&text);
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            let c = u_string::unit_at_from(&__units1, i).unwrap_or(0);
            if c == 8216 || c == 8217 || c == 8220 || c == 8221 {
                let r = roles.get(&(i));
                out += &(if r == Some(FontRole::LatinText) { "L".to_string() } else { if r == Some(FontRole::CjkPunctuation) { "C".to_string() } else { "?".to_string() }.to_string() });
            }
            i = u32::wrapping_add(i, 1);
        }
        return out;
    }

    pub fn quote_pair_analyzer_test_support_role(label: &str, text: &str, expected: &str) -> Result<(), QuotePairAnalyzerTestSupportRoleFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_string(expected, QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_sig(text, QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_pairs(text,
&QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(text).map_err(|e| QuotePairAnalyzerTestSupportRoleFault::TextRangeErrorFault(e))?, None).map_err(|e| QuotePairAnalyzerTestSupportRoleFault::TextRangeErrorFault(e))?).as_str(),
Some((label).to_string())).map_err(|e| QuotePairAnalyzerTestSupportRoleFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn quote_pair_analyzer_test_support_decisions(text: &str) -> Result<Vec<QuoteRoleDecision>, TextRangeError> {
        return Ok(QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_quote_roles(text, &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(text)?, None)?);
    }

    pub fn quote_pair_analyzer_test_support_render_decisions(values: &Vec<QuoteRoleDecision>) -> String {
        let mut out = "[".to_string();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                out += &(", ");
            }
            out += &((values[usize::try_from(i).unwrap_or(0)]).clone().to_string());
            i = u32::wrapping_add(i, 1);
        }
        return format!("{}{}",
            out,
            "]"
        );
    }

    pub fn quote_pair_analyzer_test_support_pair_role(name: &str, text: &str, indexes: &Vec<u32>, expected: FontRole) -> Result<(), QuotePairAnalyzerTestSupportPairRoleFault> {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(name);
        let r: SortedMapTable<u32, FontRole> = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_pairs(text, &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(text).map_err(|e|
QuotePairAnalyzerTestSupportPairRoleFault::TextRangeErrorFault(e))?, None).map_err(|e| QuotePairAnalyzerTestSupportPairRoleFault::TextRangeErrorFault(e))?;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((indexes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_font_role(expected, r.get(&(indexes[usize::try_from(i).unwrap_or(0)])), None).map_err(|e| QuotePairAnalyzerTestSupportPairRoleFault::TracedAssertionsFailFaultFault(e))?;
            i = u32::wrapping_add(i, 1);
        }
        Ok(())
    }
}
