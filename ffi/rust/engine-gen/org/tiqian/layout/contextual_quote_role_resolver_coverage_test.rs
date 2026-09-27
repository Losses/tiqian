#![cfg(test)]

use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePair;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteRoleDecision;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault) -> Self {
        match value {
            ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct ContextualQuoteRoleResolverCoverageSupport;

impl ContextualQuoteRoleResolverCoverageSupport {
    pub fn contextual_quote_role_resolver_coverage_support_start(n: &str) {
        TestTraceRecorder::new("ContextualQuoteRoleResolverCoverageTest").section(n);
    }

    pub fn contextual_quote_role_resolver_coverage_support_decisions(text: &str, pairs: Option<Vec<QuotePair>>) -> Result<Vec<QuoteRoleDecision>, TextRangeError> {
        return Ok(QuotePairAnalyzer::new().classify_quote_roles(text, &match &(pairs) { None => vec![], Some(__option1) => (*__option1).clone() }, None)?);
    }

    pub fn contextual_quote_role_resolver_coverage_support_any_role(ds: &Vec<QuoteRoleDecision>, role: FontRole) -> bool {
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((ds.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if ds[usize::try_from(i).unwrap_or(0)].role == role {
                return true;
            }
            i = u32::wrapping_add(i, 1);
        }
        return false;
    }

    pub fn contextual_quote_role_resolver_coverage_support_any_source(ds: &Vec<QuoteRoleDecision>, source: &str) -> bool {
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((ds.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if ds[usize::try_from(i).unwrap_or(0)].clone().source.to_string() == source {
                return true;
            }
            i = u32::wrapping_add(i, 1);
        }
        return false;
    }

    pub fn contextual_quote_role_resolver_coverage_support_non_empty(ds: &Vec<QuoteRoleDecision>) -> bool {
        return (i32::from_ne_bytes((u32::try_from((ds.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0);
    }

    pub fn contextual_quote_role_resolver_coverage_support_execute(n: &str, text: &str, paired: bool) -> Result<Vec<QuoteRoleDecision>, TextRangeError> {
        ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_start(n);
        let a = QuotePairAnalyzer::new();
        let p = if paired { a.analyze(text)? } else { vec![] };
        return Ok(a.classify_quote_roles(text, &p, None)?);
    }

    pub fn contextual_quote_role_resolver_coverage_support_surrogate_text(codes: &Vec<u32>) -> String {
        let mut s = String::new();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((codes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let unit = codes[usize::try_from(i).unwrap_or(0)];
            if ({ let v: u32 = unit; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 55296 && ({ let v: u32 = unit; i32::from_ne_bytes(v.to_ne_bytes()) }) <= 56319 && (i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((codes.len()) &
0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) && ({ let v: u32 = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 56320 && ({ let v: u32 = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)];
i32::from_ne_bytes(v.to_ne_bytes()) }) <= 57343 {
                let low = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)];
                s += &(if u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320)) > 0xFFFF { String::from_utf16(&[0xD800 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)),
u32::wrapping_sub(low, 56320))) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) - 0x10000) & 0x3FF) as u16]).unwrap() } else {
String::from_utf16_lossy(&[(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) as u16]) });
                i = u32::wrapping_add(i, 2);
            } else {
                s += &(if unit > 0xFFFF { String::from_utf16(&[0xD800 + (((unit) - 0x10000) >> 10) as u16, 0xDC00 + (((unit) - 0x10000) & 0x3FF) as u16]).unwrap() } else { String::from_utf16_lossy(&[(unit) as u16]) });
                i = u32::wrapping_add(i, 1);
            }
        }
        return s;
    }

    pub fn contextual_quote_role_resolver_coverage_support_locate(ds: &Vec<QuoteRoleDecision>, index: u32) -> Option<QuoteRoleDecision> {
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((ds.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if ds[usize::try_from(i).unwrap_or(0)].index == index {
                return Some((ds[usize::try_from(i).unwrap_or(0)]).clone());
            }
            i = u32::wrapping_add(i, 1);
        }
        return None;
    }
}

#[test]
fn nested_pair_inherits_enclosing_quote_role() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nestedPairInheritsEnclosingQuoteRole", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nestedPairInheritsEnclosingQuoteRole", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nestedPairInheritsEnclosingQuoteRole", &"他说：“她说‘你好’。”", true).unwrap();
        let a = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_locate(&d, 6);
        match &(a) {
            Some(__option2) => {
                let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(__option2.role.clone()), None).unwrap();
            }
            None => {
            }
        }
        let b = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_locate(&d, 9);
        match &(b) {
            Some(__option3) => {
                let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(__option3.role.clone()), None).unwrap();
            }
            None => {
            }
        }
    });
}

#[test]
fn nested_pair_latin_inner_inherits_cjk_enclosing() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nestedPairLatinInnerInheritsCjkEnclosing", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nestedPairLatinInnerInheritsCjkEnclosing", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nestedPairLatinInnerInheritsCjkEnclosing", &"他说：“hello”", true).unwrap();
        let a = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_locate(&d, 3);
        match &(a) {
            Some(__option4) => {
                let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(__option4.role.clone()), None).unwrap();
            }
            None => {
            }
        }
    });
}

#[test]
fn unmatched_right_single_quote_uses_surrounding_script() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightSingleQuoteUsesSurroundingScript", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightSingleQuoteUsesSurroundingScript", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"unmatchedRightSingleQuoteUsesSurroundingScript", &"abc’def", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_any_role(&d, FontRole::LatinText), None).unwrap();
    });
}

#[test]
fn unmatched_right_double_quote() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightDoubleQuote", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightDoubleQuote", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"unmatchedRightDoubleQuote", &"abc”", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_left_double_quote() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedLeftDoubleQuote", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedLeftDoubleQuote", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"unmatchedLeftDoubleQuote", &"“abc", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_left_single_quote() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedLeftSingleQuote", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedLeftSingleQuote", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"unmatchedLeftSingleQuote", &"‘abc", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn conflicting_unmatched_quotes_uses_paragraph_language() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesUsesParagraphLanguage", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesUsesParagraphLanguage", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"conflictingUnmatchedQuotesUsesParagraphLanguage", &"α’中", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_quote_with_surrogate_pair_content() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteWithSurrogatePairContent", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteWithSurrogatePairContent", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"unmatchedQuoteWithSurrogatePairContent", ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357,
56832, 8217, 20013]).as_str(), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn code_point_at_compat_with_supplementary_char() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointAtCompatWithSupplementaryChar", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointAtCompatWithSupplementaryChar", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"codePointAtCompatWithSupplementaryChar", ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357,
56832, 8220, 55357, 56832, 8221]).as_str(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn code_point_length_at_supplementary_in_content() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointLengthAtSupplementaryInContent", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointLengthAtSupplementaryInContent", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"codePointLengthAtSupplementaryInContent", ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![8220,
55357, 56832, 8221]).as_str(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn non_cjk_in_word_apostrophe_with_surrogate_before() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nonCjkInWordApostropheWithSurrogateBefore", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nonCjkInWordApostropheWithSurrogateBefore", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nonCjkInWordApostropheWithSurrogateBefore", ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357,
56832, 8217, 120]).as_str(), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn whitespace_delimited_western_quote_unmatched() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.whitespaceDelimitedWesternQuoteUnmatched", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.whitespaceDelimitedWesternQuoteUnmatched", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"whitespaceDelimitedWesternQuoteUnmatched", &"中文 ’90s", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_any_source(&d, &"DelimitedUnmatchedWesternQuote"), None).unwrap();
    });
}

#[test]
fn enclosing_pair_resolved_before_inner() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairResolvedBeforeInner", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairResolvedBeforeInner", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"enclosingPairResolvedBeforeInner", &"“‘中’”", true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn pair_by_close_skip_in_nearest_strong_script() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.pairByCloseSkipInNearestStrongScript", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.pairByCloseSkipInNearestStrongScript", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"pairByCloseSkipInNearestStrongScript", &"“‘abc’”", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn pair_by_open_skip_in_nearest_strong_script() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.pairByOpenSkipInNearestStrongScript", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.pairByOpenSkipInNearestStrongScript", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"pairByOpenSkipInNearestStrongScript", &"“‘abc’”", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn ambiguous_curly_quote_unmatched_in_text() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.ambiguousCurlyQuoteUnmatchedInText", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.ambiguousCurlyQuoteUnmatchedInText", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"ambiguousCurlyQuoteUnmatchedInText", &"abc’", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn resolve_unmatched_with_both_surrounding_roles_null() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.resolveUnmatchedWithBothSurroundingRolesNull", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.resolveUnmatchedWithBothSurroundingRolesNull", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"resolveUnmatchedWithBothSurroundingRolesNull", &"’", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_backward_skips_paired_close_quote() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardSkipsPairedCloseQuote", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardSkipsPairedCloseQuote", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nearestStrongScriptRoleBackwardSkipsPairedCloseQuote", &"“‘a’”’", true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_forward_skips_paired_open_quote() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardSkipsPairedOpenQuote", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardSkipsPairedOpenQuote", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nearestStrongScriptRoleForwardSkipsPairedOpenQuote", &"’“abc”", true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn enclosing_pair_resolved_before_inner_pair() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairResolvedBeforeInnerPair", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairResolvedBeforeInnerPair", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"enclosingPairResolvedBeforeInnerPair", &"“‘abc’”", true).unwrap();
        let a = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_locate(&d, 1);
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(a.is_some(), match &(a) { None => "null".to_string(), Some(__option7) => __option7.to_string() }.as_str(), None).unwrap();
        match &(a) {
            Some(__option8) => {
                let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(__option8.role.clone()), None).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PairedPunctuationEnclosingQuoteContext", (__option8.source).to_string().as_str(), None).unwrap();
            }
            None => {
            }
        }
    });
}

#[test]
fn whitespace_delimited_western_quote_paired() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.whitespaceDelimitedWesternQuotePaired", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.whitespaceDelimitedWesternQuotePaired", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"whitespaceDelimitedWesternQuotePaired", &"“ ‘hello’ ”", true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn conflicting_unmatched_quotes_both_non_null() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesBothNonNull", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesBothNonNull", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"conflictingUnmatchedQuotesBothNonNull", &"α’中", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn no_unmatched_quote_context() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.noUnmatchedQuoteContext", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.noUnmatchedQuoteContext", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"noUnmatchedQuoteContext", &"’", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_backward_through_surrogate_pair() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardThroughSurrogatePair", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardThroughSurrogatePair", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nearestStrongScriptRoleBackwardThroughSurrogatePair",
ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357, 56832, 8220, 97, 98, 99, 8221]).as_str(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_forward_through_surrogate_pair() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardThroughSurrogatePair", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardThroughSurrogatePair", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nearestStrongScriptRoleForwardThroughSurrogatePair",
ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![8220, 97, 98, 99, 55357, 56832, 8221]).as_str(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nested_pair_skips_inner_in_script_evidence() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nestedPairSkipsInnerInScriptEvidence", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nestedPairSkipsInnerInScriptEvidence", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nestedPairSkipsInnerInScriptEvidence", &"“‘中’”", true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn mixed_script_enclosing_level_uses_paragraph_language() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.mixedScriptEnclosingLevelUsesParagraphLanguage", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.mixedScriptEnclosingLevelUsesParagraphLanguage", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"mixedScriptEnclosingLevelUsesParagraphLanguage", &"abc“中”", true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_right_single_quote_with_left_role() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightSingleQuoteWithLeftRole", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightSingleQuoteWithLeftRole", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"unmatchedRightSingleQuoteWithLeftRole", &"中’", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_right_single_quote_with_right_role() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightSingleQuoteWithRightRole", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightSingleQuoteWithRightRole", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"unmatchedRightSingleQuoteWithRightRole", &"’中", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_quote_with_whitespace_before_and_latin_right() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteWithWhitespaceBeforeAndLatinRight", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteWithWhitespaceBeforeAndLatinRight", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"unmatchedQuoteWithWhitespaceBeforeAndLatinRight", &" ’abc", false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_any_source(&d, &"DelimitedUnmatchedWesternQuote"), None).unwrap();
    });
}

#[test]
fn non_cjk_in_word_apostrophe_paired() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nonCjkInWordApostrophePaired", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nonCjkInWordApostrophePaired", || {
        let _ = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nonCjkInWordApostrophePaired", &"‘it’s", true).unwrap();
        let a = QuotePairAnalyzer::new();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((a.analyze(&"‘it’s").unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn code_point_length_at_surrogate_pair_in_content() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointLengthAtSurrogatePairInContent", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointLengthAtSurrogatePairInContent", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"codePointLengthAtSurrogatePairInContent", ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![8220,
55357, 56832, 8221]).as_str(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn code_point_at_compat_supplementary_in_outer_evidence() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointAtCompatSupplementaryInOuterEvidence", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointAtCompatSupplementaryInOuterEvidence", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"codePointAtCompatSupplementaryInOuterEvidence",
ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357, 56832, 8220, 97, 98, 99, 8221, 55357, 56832]).as_str(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn conflicting_unmatched_quotes_left_and_right_non_null() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesLeftAndRightNonNull", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesLeftAndRightNonNull", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"conflictingUnmatchedQuotesLeftAndRightNonNull", &"a’b“c", true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_quote_non_whitespace_before() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteNonWhitespaceBefore", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteNonWhitespaceBefore", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"unmatchedQuoteNonWhitespaceBefore", &"a“", true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_backward_hits_supplementary() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardHitsSupplementary", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardHitsSupplementary", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nearestStrongScriptRoleBackwardHitsSupplementary",
ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357, 56832, 8220]).as_str(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_forward_hits_supplementary() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardHitsSupplementary", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardHitsSupplementary", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"nearestStrongScriptRoleForwardHitsSupplementary",
ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![8220, 55357, 56832]).as_str(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn enclosing_pair_unresolved_falls_through_to_content() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairUnresolvedFallsThroughToContent", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairUnresolvedFallsThroughToContent", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"enclosingPairUnresolvedFallsThroughToContent", &"“‘abc’”", true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_quote_at_start_with_right_role() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteAtStartWithRightRole", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteAtStartWithRightRole", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(&"unmatchedQuoteAtStartWithRightRole", &"“abc", true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}
