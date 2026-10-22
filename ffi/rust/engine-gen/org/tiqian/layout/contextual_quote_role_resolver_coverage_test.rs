#![cfg(test)]

use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePair;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteRoleDecision;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuoteUnmatchedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestWhitespaceDelimitedWesternQuotePairedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithRightRoleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteWithLeftRoleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightSingleQuoteUsesSurroundingScriptFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedRightDoubleQuoteFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithWhitespaceBeforeAndLatinRightFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteWithSurrogatePairContentFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteNonWhitespaceBeforeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedQuoteAtStartWithRightRoleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftSingleQuoteFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestUnmatchedLeftDoubleQuoteFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestResolveUnmatchedWithBothSurroundingRolesNullFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestPairByOpenSkipInNearestStrongScriptFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestPairByCloseSkipInNearestStrongScriptFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostropheWithSurrogateBeforeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNonCjkInWordApostrophePairedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNoUnmatchedQuoteContextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNestedPairSkipsInnerInScriptEvidenceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNestedPairLatinInnerInheritsCjkEnclosingFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNestedPairInheritsEnclosingQuoteRoleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardThroughSurrogatePairFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardSkipsPairedOpenQuoteFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleForwardHitsSupplementaryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardThroughSurrogatePairFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardSkipsPairedCloseQuoteFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestNearestStrongScriptRoleBackwardHitsSupplementaryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestMixedScriptEnclosingLevelUsesParagraphLanguageFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestEnclosingPairUnresolvedFallsThroughToContentFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerPairFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestEnclosingPairResolvedBeforeInnerFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesUsesParagraphLanguageFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesLeftAndRightNonNullFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestConflictingUnmatchedQuotesBothNonNullFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSurrogatePairInContentFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestCodePointLengthAtSupplementaryInContentFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatWithSupplementaryCharFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestCodePointAtCompatSupplementaryInOuterEvidenceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualQuoteRoleResolverCoverageTestAmbiguousCurlyQuoteUnmatchedInTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
    pub fn contextual_quote_role_resolver_coverage_support_start(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[67,111,110,116,101,120,116,117,97,108,81,117,111,116,101,82,111,108,101,82,101,115,111,108,118,101,114,67,111,118,101,114,97,103,101,84,101,115,116]))).section(n);
    }

    pub fn contextual_quote_role_resolver_coverage_support_decisions(text: &UStr, pairs: Option<Vec<QuotePair>>) -> Result<Vec<QuoteRoleDecision>, TextRangeError> {
        return Ok(QuotePairAnalyzer::new().classify_quote_roles(text, &match &(pairs) { None => vec![], Some(__option1) => (*__option1).clone() }, None)?);
    }

    pub fn contextual_quote_role_resolver_coverage_support_any_role(ds: &Vec<QuoteRoleDecision>, role: FontRole) -> bool {
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((ds.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if ds[usize::try_from(i).unwrap_or(0)].role == role {
                return true;
            }
            i = u32::wrapping_add(i, 1);
        }
        return false;
    }

    pub fn contextual_quote_role_resolver_coverage_support_any_source(ds: &Vec<QuoteRoleDecision>, source: &UStr) -> bool {
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((ds.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if ds[usize::try_from(i).unwrap_or(0)].clone().source.to_ustring() == source {
                return true;
            }
            i = u32::wrapping_add(i, 1);
        }
        return false;
    }

    pub fn contextual_quote_role_resolver_coverage_support_non_empty(ds: &Vec<QuoteRoleDecision>) -> bool {
        return (i32::from_ne_bytes(((u32::try_from((ds.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0);
    }

    pub fn contextual_quote_role_resolver_coverage_support_execute(n: &UStr, text: &UStr, paired: bool) -> Result<Vec<QuoteRoleDecision>, TextRangeError> {
        ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_start(n);
        let a = QuotePairAnalyzer::new();
        let p = if paired { a.analyze(text)? } else { vec![] };
        return Ok(a.classify_quote_roles(text, &p, None)?);
    }

    pub fn contextual_quote_role_resolver_coverage_support_surrogate_text(codes: &Vec<u32>) -> UString {
        let mut s = UString::new();
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((codes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let unit = codes[usize::try_from(i).unwrap_or(0)];
            if ({ let v: u32 = unit; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 55296 && ({ let v: u32 = unit; i32::from_ne_bytes(v.to_ne_bytes()) }) <= 56319 && (i32::from_ne_bytes(((u32::wrapping_add(i, 1)) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((codes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) && ({ let v: u32 = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 56320 && ({ let v: u32 = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) <= 57343 {
                let low = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)];
                s += &(if u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320)) > 0xFFFF { u_string::from_units(&[0xD800 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) as u16]) });
                i = u32::wrapping_add(i, 2);
            } else {
                s += &(if unit > 0xFFFF { u_string::from_units(&[0xD800 + (((unit) - 0x10000) >> 10) as u16, 0xDC00 + (((unit) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(unit) as u16]) });
                i = u32::wrapping_add(i, 1);
            }
        }
        return s;
    }

    pub fn contextual_quote_role_resolver_coverage_support_locate(ds: &Vec<QuoteRoleDecision>, index: u32) -> Option<QuoteRoleDecision> {
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((ds.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
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
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,101,115,116,101,100,80,97,105,114,73,110,104,101,114,105,116,115,69,110,99,108,111,115,105,110,103,81,117,111,116,101,82,111,108,101]), UStr::new(&[20182,35828,65306,8220,22905,35828,8216,20320,22909,8217,12290,8221]), true).unwrap();
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
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,101,115,116,101,100,80,97,105,114,76,97,116,105,110,73,110,110,101,114,73,110,104,101,114,105,116,115,67,106,107,69,110,99,108,111,115,105,110,103]), UStr::new(&[20182,35828,65306,8220,104,101,108,108,111,8221]), true).unwrap();
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
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[117,110,109,97,116,99,104,101,100,82,105,103,104,116,83,105,110,103,108,101,81,117,111,116,101,85,115,101,115,83,117,114,114,111,117,110,100,105,110,103,83,99,114,105,112,116]), UStr::new(&[97,98,99,8217,100,101,102]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_any_role(&d, FontRole::LatinText), None).unwrap();
    });
}

#[test]
fn unmatched_right_double_quote() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightDoubleQuote", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightDoubleQuote", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[117,110,109,97,116,99,104,101,100,82,105,103,104,116,68,111,117,98,108,101,81,117,111,116,101]), UStr::new(&[97,98,99,8221]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_left_double_quote() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedLeftDoubleQuote", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedLeftDoubleQuote", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[117,110,109,97,116,99,104,101,100,76,101,102,116,68,111,117,98,108,101,81,117,111,116,101]), UStr::new(&[8220,97,98,99]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_left_single_quote() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedLeftSingleQuote", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedLeftSingleQuote", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[117,110,109,97,116,99,104,101,100,76,101,102,116,83,105,110,103,108,101,81,117,111,116,101]), UStr::new(&[8216,97,98,99]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn conflicting_unmatched_quotes_uses_paragraph_language() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesUsesParagraphLanguage", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesUsesParagraphLanguage", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[99,111,110,102,108,105,99,116,105,110,103,85,110,109,97,116,99,104,101,100,81,117,111,116,101,115,85,115,101,115,80,97,114,97,103,114,97,112,104,76,97,110,103,117,97,103,101]), UStr::new(&[945,8217,20013]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_quote_with_surrogate_pair_content() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteWithSurrogatePairContent", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteWithSurrogatePairContent", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[117,110,109,97,116,99,104,101,100,81,117,111,116,101,87,105,116,104,83,117,114,114,111,103,97,116,101,80,97,105,114,67,111,110,116,101,110,116]), ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357, 56832, 8217, 20013]).as_ustr(), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn code_point_at_compat_with_supplementary_char() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointAtCompatWithSupplementaryChar", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointAtCompatWithSupplementaryChar", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[99,111,100,101,80,111,105,110,116,65,116,67,111,109,112,97,116,87,105,116,104,83,117,112,112,108,101,109,101,110,116,97,114,121,67,104,97,114]), ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357, 56832, 8220, 55357, 56832, 8221]).as_ustr(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn code_point_length_at_supplementary_in_content() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointLengthAtSupplementaryInContent", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointLengthAtSupplementaryInContent", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[99,111,100,101,80,111,105,110,116,76,101,110,103,116,104,65,116,83,117,112,112,108,101,109,101,110,116,97,114,121,73,110,67,111,110,116,101,110,116]), ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![8220, 55357, 56832, 8221]).as_ustr(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn non_cjk_in_word_apostrophe_with_surrogate_before() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nonCjkInWordApostropheWithSurrogateBefore", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nonCjkInWordApostropheWithSurrogateBefore", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,111,110,67,106,107,73,110,87,111,114,100,65,112,111,115,116,114,111,112,104,101,87,105,116,104,83,117,114,114,111,103,97,116,101,66,101,102,111,114,101]), ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357, 56832, 8217, 120]).as_ustr(), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn whitespace_delimited_western_quote_unmatched() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.whitespaceDelimitedWesternQuoteUnmatched", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.whitespaceDelimitedWesternQuoteUnmatched", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[119,104,105,116,101,115,112,97,99,101,68,101,108,105,109,105,116,101,100,87,101,115,116,101,114,110,81,117,111,116,101,85,110,109,97,116,99,104,101,100]), UStr::new(&[20013,25991,32,8217,57,48,115]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_any_source(&d, UStr::new(&[68,101,108,105,109,105,116,101,100,85,110,109,97,116,99,104,101,100,87,101,115,116,101,114,110,81,117,111,116,101])), None).unwrap();
    });
}

#[test]
fn enclosing_pair_resolved_before_inner() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairResolvedBeforeInner", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairResolvedBeforeInner", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[101,110,99,108,111,115,105,110,103,80,97,105,114,82,101,115,111,108,118,101,100,66,101,102,111,114,101,73,110,110,101,114]), UStr::new(&[8220,8216,20013,8217,8221]), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn pair_by_close_skip_in_nearest_strong_script() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.pairByCloseSkipInNearestStrongScript", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.pairByCloseSkipInNearestStrongScript", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[112,97,105,114,66,121,67,108,111,115,101,83,107,105,112,73,110,78,101,97,114,101,115,116,83,116,114,111,110,103,83,99,114,105,112,116]), UStr::new(&[8220,8216,97,98,99,8217,8221]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn pair_by_open_skip_in_nearest_strong_script() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.pairByOpenSkipInNearestStrongScript", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.pairByOpenSkipInNearestStrongScript", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[112,97,105,114,66,121,79,112,101,110,83,107,105,112,73,110,78,101,97,114,101,115,116,83,116,114,111,110,103,83,99,114,105,112,116]), UStr::new(&[8220,8216,97,98,99,8217,8221]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn ambiguous_curly_quote_unmatched_in_text() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.ambiguousCurlyQuoteUnmatchedInText", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.ambiguousCurlyQuoteUnmatchedInText", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[97,109,98,105,103,117,111,117,115,67,117,114,108,121,81,117,111,116,101,85,110,109,97,116,99,104,101,100,73,110,84,101,120,116]), UStr::new(&[97,98,99,8217]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn resolve_unmatched_with_both_surrounding_roles_null() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.resolveUnmatchedWithBothSurroundingRolesNull", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.resolveUnmatchedWithBothSurroundingRolesNull", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[114,101,115,111,108,118,101,85,110,109,97,116,99,104,101,100,87,105,116,104,66,111,116,104,83,117,114,114,111,117,110,100,105,110,103,82,111,108,101,115,78,117,108,108]), UStr::new(&[8217]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_backward_skips_paired_close_quote() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardSkipsPairedCloseQuote", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardSkipsPairedCloseQuote", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,101,97,114,101,115,116,83,116,114,111,110,103,83,99,114,105,112,116,82,111,108,101,66,97,99,107,119,97,114,100,83,107,105,112,115,80,97,105,114,101,100,67,108,111,115,101,81,117,111,116,101]), UStr::new(&[8220,8216,97,8217,8221,8217]), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_forward_skips_paired_open_quote() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardSkipsPairedOpenQuote", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardSkipsPairedOpenQuote", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,101,97,114,101,115,116,83,116,114,111,110,103,83,99,114,105,112,116,82,111,108,101,70,111,114,119,97,114,100,83,107,105,112,115,80,97,105,114,101,100,79,112,101,110,81,117,111,116,101]), UStr::new(&[8217,8220,97,98,99,8221]), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn enclosing_pair_resolved_before_inner_pair() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairResolvedBeforeInnerPair", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairResolvedBeforeInnerPair", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[101,110,99,108,111,115,105,110,103,80,97,105,114,82,101,115,111,108,118,101,100,66,101,102,111,114,101,73,110,110,101,114,80,97,105,114]), UStr::new(&[8220,8216,97,98,99,8217,8221]), true).unwrap();
        let a = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_locate(&d, 1);
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(a.is_some(), match &(a) { None => UString::from("null"), Some(__option7) => UString::from(format!("{}", __option7.to_string()).as_str()) }.as_ustr(), None).unwrap();
        match &(a) {
            Some(__option8) => {
                let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(__option8.role.clone()), None).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,97,105,114,101,100,80,117,110,99,116,117,97,116,105,111,110,69,110,99,108,111,115,105,110,103,81,117,111,116,101,67,111,110,116,101,120,116]), (__option8.source).to_ustring().as_ustr(), None).unwrap();
            }
            None => {
            }
        }
    });
}

#[test]
fn whitespace_delimited_western_quote_paired() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.whitespaceDelimitedWesternQuotePaired", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.whitespaceDelimitedWesternQuotePaired", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[119,104,105,116,101,115,112,97,99,101,68,101,108,105,109,105,116,101,100,87,101,115,116,101,114,110,81,117,111,116,101,80,97,105,114,101,100]), UStr::new(&[8220,32,8216,104,101,108,108,111,8217,32,8221]), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn conflicting_unmatched_quotes_both_non_null() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesBothNonNull", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesBothNonNull", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[99,111,110,102,108,105,99,116,105,110,103,85,110,109,97,116,99,104,101,100,81,117,111,116,101,115,66,111,116,104,78,111,110,78,117,108,108]), UStr::new(&[945,8217,20013]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn no_unmatched_quote_context() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.noUnmatchedQuoteContext", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.noUnmatchedQuoteContext", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,111,85,110,109,97,116,99,104,101,100,81,117,111,116,101,67,111,110,116,101,120,116]), UStr::new(&[8217]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_backward_through_surrogate_pair() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardThroughSurrogatePair", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardThroughSurrogatePair", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,101,97,114,101,115,116,83,116,114,111,110,103,83,99,114,105,112,116,82,111,108,101,66,97,99,107,119,97,114,100,84,104,114,111,117,103,104,83,117,114,114,111,103,97,116,101,80,97,105,114]), ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357, 56832, 8220, 97, 98, 99, 8221]).as_ustr(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_forward_through_surrogate_pair() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardThroughSurrogatePair", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardThroughSurrogatePair", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,101,97,114,101,115,116,83,116,114,111,110,103,83,99,114,105,112,116,82,111,108,101,70,111,114,119,97,114,100,84,104,114,111,117,103,104,83,117,114,114,111,103,97,116,101,80,97,105,114]), ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![8220, 97, 98, 99, 55357, 56832, 8221]).as_ustr(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nested_pair_skips_inner_in_script_evidence() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nestedPairSkipsInnerInScriptEvidence", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nestedPairSkipsInnerInScriptEvidence", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,101,115,116,101,100,80,97,105,114,83,107,105,112,115,73,110,110,101,114,73,110,83,99,114,105,112,116,69,118,105,100,101,110,99,101]), UStr::new(&[8220,8216,20013,8217,8221]), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn mixed_script_enclosing_level_uses_paragraph_language() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.mixedScriptEnclosingLevelUsesParagraphLanguage", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.mixedScriptEnclosingLevelUsesParagraphLanguage", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[109,105,120,101,100,83,99,114,105,112,116,69,110,99,108,111,115,105,110,103,76,101,118,101,108,85,115,101,115,80,97,114,97,103,114,97,112,104,76,97,110,103,117,97,103,101]), UStr::new(&[97,98,99,8220,20013,8221]), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_right_single_quote_with_left_role() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightSingleQuoteWithLeftRole", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightSingleQuoteWithLeftRole", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[117,110,109,97,116,99,104,101,100,82,105,103,104,116,83,105,110,103,108,101,81,117,111,116,101,87,105,116,104,76,101,102,116,82,111,108,101]), UStr::new(&[20013,8217]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_right_single_quote_with_right_role() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightSingleQuoteWithRightRole", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedRightSingleQuoteWithRightRole", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[117,110,109,97,116,99,104,101,100,82,105,103,104,116,83,105,110,103,108,101,81,117,111,116,101,87,105,116,104,82,105,103,104,116,82,111,108,101]), UStr::new(&[8217,20013]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_quote_with_whitespace_before_and_latin_right() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteWithWhitespaceBeforeAndLatinRight", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteWithWhitespaceBeforeAndLatinRight", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[117,110,109,97,116,99,104,101,100,81,117,111,116,101,87,105,116,104,87,104,105,116,101,115,112,97,99,101,66,101,102,111,114,101,65,110,100,76,97,116,105,110,82,105,103,104,116]), UStr::new(&[32,8217,97,98,99]), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_any_source(&d, UStr::new(&[68,101,108,105,109,105,116,101,100,85,110,109,97,116,99,104,101,100,87,101,115,116,101,114,110,81,117,111,116,101])), None).unwrap();
    });
}

#[test]
fn non_cjk_in_word_apostrophe_paired() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nonCjkInWordApostrophePaired", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nonCjkInWordApostrophePaired", || {
        let _ = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,111,110,67,106,107,73,110,87,111,114,100,65,112,111,115,116,114,111,112,104,101,80,97,105,114,101,100]), UStr::new(&[8216,105,116,8217,115]), true).unwrap();
        let a = QuotePairAnalyzer::new();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((a.analyze(UStr::new(&[8216,105,116,8217,115])).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn code_point_length_at_surrogate_pair_in_content() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointLengthAtSurrogatePairInContent", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointLengthAtSurrogatePairInContent", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[99,111,100,101,80,111,105,110,116,76,101,110,103,116,104,65,116,83,117,114,114,111,103,97,116,101,80,97,105,114,73,110,67,111,110,116,101,110,116]), ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![8220, 55357, 56832, 8221]).as_ustr(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn code_point_at_compat_supplementary_in_outer_evidence() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointAtCompatSupplementaryInOuterEvidence", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.codePointAtCompatSupplementaryInOuterEvidence", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[99,111,100,101,80,111,105,110,116,65,116,67,111,109,112,97,116,83,117,112,112,108,101,109,101,110,116,97,114,121,73,110,79,117,116,101,114,69,118,105,100,101,110,99,101]), ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357, 56832, 8220, 97, 98, 99, 8221, 55357, 56832]).as_ustr(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn conflicting_unmatched_quotes_left_and_right_non_null() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesLeftAndRightNonNull", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.conflictingUnmatchedQuotesLeftAndRightNonNull", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[99,111,110,102,108,105,99,116,105,110,103,85,110,109,97,116,99,104,101,100,81,117,111,116,101,115,76,101,102,116,65,110,100,82,105,103,104,116,78,111,110,78,117,108,108]), UStr::new(&[97,8217,98,8220,99]), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_quote_non_whitespace_before() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteNonWhitespaceBefore", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteNonWhitespaceBefore", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[117,110,109,97,116,99,104,101,100,81,117,111,116,101,78,111,110,87,104,105,116,101,115,112,97,99,101,66,101,102,111,114,101]), UStr::new(&[97,8220]), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_backward_hits_supplementary() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardHitsSupplementary", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleBackwardHitsSupplementary", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,101,97,114,101,115,116,83,116,114,111,110,103,83,99,114,105,112,116,82,111,108,101,66,97,99,107,119,97,114,100,72,105,116,115,83,117,112,112,108,101,109,101,110,116,97,114,121]), ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![55357, 56832, 8220]).as_ustr(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn nearest_strong_script_role_forward_hits_supplementary() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardHitsSupplementary", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.nearestStrongScriptRoleForwardHitsSupplementary", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[110,101,97,114,101,115,116,83,116,114,111,110,103,83,99,114,105,112,116,82,111,108,101,70,111,114,119,97,114,100,72,105,116,115,83,117,112,112,108,101,109,101,110,116,97,114,121]), ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_surrogate_text(&vec![8220, 55357, 56832]).as_ustr(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn enclosing_pair_unresolved_falls_through_to_content() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairUnresolvedFallsThroughToContent", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.enclosingPairUnresolvedFallsThroughToContent", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[101,110,99,108,111,115,105,110,103,80,97,105,114,85,110,114,101,115,111,108,118,101,100,70,97,108,108,115,84,104,114,111,117,103,104,84,111,67,111,110,116,101,110,116]), UStr::new(&[8220,8216,97,98,99,8217,8221]), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}

#[test]
fn unmatched_quote_at_start_with_right_role() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteAtStartWithRightRole", "org.tiqian.layout.ContextualQuoteRoleResolverCoverageTest.unmatchedQuoteAtStartWithRightRole", || {
        let d = ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_execute(UStr::new(&[117,110,109,97,116,99,104,101,100,81,117,111,116,101,65,116,83,116,97,114,116,87,105,116,104,82,105,103,104,116,82,111,108,101]), UStr::new(&[8220,97,98,99]), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ContextualQuoteRoleResolverCoverageSupport::contextual_quote_role_resolver_coverage_support_non_empty(&d), None).unwrap();
    });
}
