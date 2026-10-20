#![cfg(test)]

use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAwareFontRoleClassifier;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteType;
use crate::org::tiqian::layout::quote_pair_analyzer_coverage_test_support::QuotePairAnalyzerCoverageTestSupport;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsQuoteTypeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsQuoteTypeFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TracedAssertionsAssertEqualsFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TracedAssertionsAssertEqualsQuoteTypeFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsQuoteTypeFault {
    fn from(value: QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TracedAssertionsAssertEqualsQuoteTypeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsQuoteTypeFault> for QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsQuoteTypeFault) -> Self {
        QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TracedAssertionsAssertEqualsQuoteTypeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsAssertEqualsFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestSingleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFontRoleFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::TracedAssertionsAssertEqualsFontRoleFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault {
    fn from(value: QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::TracedAssertionsAssertEqualsFontRoleFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault> for QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault) -> Self {
        QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::TracedAssertionsAssertEqualsFontRoleFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierUsesOverrideFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFontRoleFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::TracedAssertionsAssertEqualsFontRoleFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault {
    fn from(value: QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::TracedAssertionsAssertEqualsFontRoleFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault> for QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault) -> Self {
        QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::TracedAssertionsAssertEqualsFontRoleFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestQuotePairAwareFontRoleClassifierDelegatesWhenNoOverrideFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsAssertEqualsFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestDoubleQuoteCloseWithEmptyStackIgnoresFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestDeprecatedClassifyQuoteRolesWithFontRoleClassifierDelegatesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsFontRoleFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::TracedAssertionsAssertEqualsFontRoleFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault {
    fn from(value: QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::TracedAssertionsAssertEqualsFontRoleFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault> for QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFontRoleFault) -> Self {
        QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::TracedAssertionsAssertEqualsFontRoleFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestDeprecatedClassifyPairsWithFontRoleClassifierDelegatesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::TracedAssertionsAssertEqualsFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestCodePointBeforeSurrogatePairReturnsSupplementaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestCodePointAtOrNullNonSurrogateReturnsSelfFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::TracedAssertionsAssertEqualsFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestAnalyzeWithDoubleQuoteOpenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::TracedAssertionsAssertEqualsFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault) -> Self {
        match value {
            QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerCoverageTestAnalyzeWithAllQuoteTypesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn deprecated_classify_pairs_with_font_role_classifier_delegates() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.deprecatedClassifyPairsWithFontRoleClassifierDelegates", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.deprecatedClassifyPairsWithFontRoleClassifierDelegates", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[100,101,112,114,101,99,97,116,101,100,67,108,97,115,115,105,102,121,80,97,105,114,115,87,105,116,104,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,68,101,108,101,103,97,116,101,115]));
        let t = UString::from("他说“你好”").to_ustring();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().classify_pairs_with_classifier(t.as_ustr(), &QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(t.as_ustr()).unwrap(), Box::new(CjkFontRoleClassifier::new()), None).unwrap().get(&(2)), None).unwrap();
    });
}

#[test]
fn deprecated_classify_quote_roles_with_font_role_classifier_delegates() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.deprecatedClassifyQuoteRolesWithFontRoleClassifierDelegates", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.deprecatedClassifyQuoteRolesWithFontRoleClassifierDelegates", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[100,101,112,114,101,99,97,116,101,100,67,108,97,115,115,105,102,121,81,117,111,116,101,82,111,108,101,115,87,105,116,104,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,68,101,108,101,103,97,116,101,115]));
        let t = UString::from("他说“你好”").to_ustring();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().classify_quote_roles_with_classifier(t.as_ustr(), &QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(t.as_ustr()).unwrap(), Box::new(CjkFontRoleClassifier::new()), None).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn code_point_before_surrogate_pair_returns_supplementary() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeSurrogatePairReturnsSupplementary", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeSurrogatePairReturnsSupplementary", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[99,111,100,101,80,111,105,110,116,66,101,102,111,114,101,83,117,114,114,111,103,97,116,101,80,97,105,114,82,101,116,117,114,110,115,83,117,112,112,108,101,109,101,110,116,97,114,121]));
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832, 8217]).as_ustr()).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn code_point_at_or_null_surrogate_pair_returns_supplementary() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullSurrogatePairReturnsSupplementary", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullSurrogatePairReturnsSupplementary", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[99,111,100,101,80,111,105,110,116,65,116,79,114,78,117,108,108,83,117,114,114,111,103,97,116,101,80,97,105,114,82,101,116,117,114,110,115,83,117,112,112,108,101,109,101,110,116,97,114,121]));
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(TestHelpers::test_helpers_surrogate_text(&vec![8217, 55357, 56832]).as_ustr()).unwrap();
    });
}

#[test]
fn code_point_at_or_null_non_surrogate_returns_self() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullNonSurrogateReturnsSelf", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullNonSurrogateReturnsSelf", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[99,111,100,101,80,111,105,110,116,65,116,79,114,78,117,108,108,78,111,110,83,117,114,114,111,103,97,116,101,82,101,116,117,114,110,115,83,101,108,102]));
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().classify_quote_roles(UStr::new(&[97,98,99]), &vec![], None).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn code_point_before_returns_null_at_start() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeReturnsNullAtStart", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeReturnsNullAtStart", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[99,111,100,101,80,111,105,110,116,66,101,102,111,114,101,82,101,116,117,114,110,115,78,117,108,108,65,116,83,116,97,114,116]));
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(UStr::new(&[8217])).unwrap();
    });
}

#[test]
fn code_point_before_returns_supplementary_for_surrogate_pair() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeReturnsSupplementaryForSurrogatePair", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeReturnsSupplementaryForSurrogatePair", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[99,111,100,101,80,111,105,110,116,66,101,102,111,114,101,82,101,116,117,114,110,115,83,117,112,112,108,101,109,101,110,116,97,114,121,70,111,114,83,117,114,114,111,103,97,116,101,80,97,105,114]));
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832, 8217]).as_ustr()).unwrap();
    });
}

#[test]
fn quote_pair_aware_font_role_classifier_uses_override() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.quotePairAwareFontRoleClassifierUsesOverride", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.quotePairAwareFontRoleClassifierUsesOverride", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[113,117,111,116,101,80,97,105,114,65,119,97,114,101,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,85,115,101,115,79,118,101,114,114,105,100,101]));
        let mut b: SortedMapTableBuilder<u32, FontRole> = SortedTable::sorted_table_map_builder::<u32, FontRole>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        b.put(&(2), &(FontRole::LatinText));
        let c = QuotePairAwareFontRoleClassifier::new(Box::new(CjkFontRoleClassifier::new()), b.clone().build());
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(c.classify(UStr::new(&[97,98]), TextRange::new(0u32, 2u32).unwrap(), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None)))), None).unwrap();
    });
}

#[test]
fn quote_pair_aware_font_role_classifier_delegates_when_no_override() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.quotePairAwareFontRoleClassifierDelegatesWhenNoOverride", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.quotePairAwareFontRoleClassifierDelegatesWhenNoOverride", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[113,117,111,116,101,80,97,105,114,65,119,97,114,101,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,68,101,108,101,103,97,116,101,115,87,104,101,110,78,111,79,118,101,114,114,105,100,101]));
        let b: SortedMapTableBuilder<u32, FontRole> = SortedTable::sorted_table_map_builder::<u32, FontRole>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let c = CjkFontRoleClassifier::new();
        let w = QuotePairAwareFontRoleClassifier::new(Box::new((c).clone()), b.clone().build());
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(c.classify(UStr::new(&[97,98]), TextRange::new(0u32, 2u32).unwrap(), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))), Some(w.classify(UStr::new(&[97,98]), TextRange::new(0u32, 2u32).unwrap(), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None)))), None).unwrap();
    });
}

#[test]
fn double_quote_close_with_empty_stack_ignores() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.doubleQuoteCloseWithEmptyStackIgnores", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.doubleQuoteCloseWithEmptyStackIgnores", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[100,111,117,98,108,101,81,117,111,116,101,67,108,111,115,101,87,105,116,104,69,109,112,116,121,83,116,97,99,107,73,103,110,111,114,101,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(UStr::new(&[8221])).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn single_quote_close_with_empty_stack_ignores() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.singleQuoteCloseWithEmptyStackIgnores", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.singleQuoteCloseWithEmptyStackIgnores", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[115,105,110,103,108,101,81,117,111,116,101,67,108,111,115,101,87,105,116,104,69,109,112,116,121,83,116,97,99,107,73,103,110,111,114,101,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(UStr::new(&[8217])).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn in_word_apostrophe_after_supplementary_does_not_close() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.inWordApostropheAfterSupplementaryDoesNotClose", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.inWordApostropheAfterSupplementaryDoesNotClose", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[105,110,87,111,114,100,65,112,111,115,116,114,111,112,104,101,65,102,116,101,114,83,117,112,112,108,101,109,101,110,116,97,114,121,68,111,101,115,78,111,116,67,108,111,115,101]));
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832, 8217, 120]).as_ustr()).unwrap();
    });
}

#[test]
fn code_point_at_or_null_with_supplementary_after_quote() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullWithSupplementaryAfterQuote", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullWithSupplementaryAfterQuote", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[99,111,100,101,80,111,105,110,116,65,116,79,114,78,117,108,108,87,105,116,104,83,117,112,112,108,101,109,101,110,116,97,114,121,65,102,116,101,114,81,117,111,116,101]));
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(TestHelpers::test_helpers_surrogate_text(&vec![97, 8217, 55357, 56832]).as_ustr()).unwrap();
    });
}

#[test]
fn code_point_before_with_high_surrogate_before_quote() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeWithHighSurrogateBeforeQuote", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeWithHighSurrogateBeforeQuote", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[99,111,100,101,80,111,105,110,116,66,101,102,111,114,101,87,105,116,104,72,105,103,104,83,117,114,114,111,103,97,116,101,66,101,102,111,114,101,81,117,111,116,101]));
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832, 8217]).as_ustr()).unwrap();
    });
}

#[test]
fn code_point_before_with_low_surrogate_at_start() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeWithLowSurrogateAtStart", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeWithLowSurrogateAtStart");
}

#[test]
fn code_point_before_with_low_surrogate_after_non_high_surrogate() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeWithLowSurrogateAfterNonHighSurrogate", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeWithLowSurrogateAfterNonHighSurrogate");
}

#[test]
fn code_point_at_or_null_with_index_out_of_range() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullWithIndexOutOfRange", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullWithIndexOutOfRange", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[99,111,100,101,80,111,105,110,116,65,116,79,114,78,117,108,108,87,105,116,104,73,110,100,101,120,79,117,116,79,102,82,97,110,103,101]));
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(UStr::new(&[97,8217])).unwrap();
    });
}

#[test]
fn code_point_at_or_null_with_high_surrogate_at_end() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullWithHighSurrogateAtEnd", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullWithHighSurrogateAtEnd");
}

#[test]
fn code_point_at_or_null_with_high_surrogate_followed_by_non_low_surrogate() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullWithHighSurrogateFollowedByNonLowSurrogate", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullWithHighSurrogateFollowedByNonLowSurrogate");
}

#[test]
fn analyze_with_double_quote_open() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.analyzeWithDoubleQuoteOpen", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.analyzeWithDoubleQuoteOpen", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[97,110,97,108,121,122,101,87,105,116,104,68,111,117,98,108,101,81,117,111,116,101,79,112,101,110]));
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(UStr::new(&[8220,97,98,99])).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn code_point_at_or_null_high_surrogate_not_in_range_returns_high() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullHighSurrogateNotInRangeReturnsHigh", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullHighSurrogateNotInRangeReturnsHigh", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[99,111,100,101,80,111,105,110,116,65,116,79,114,78,117,108,108,72,105,103,104,83,117,114,114,111,103,97,116,101,78,111,116,73,110,82,97,110,103,101,82,101,116,117,114,110,115,72,105,103,104]));
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(UStr::new(&[120,8217,97])).unwrap();
    });
}

#[test]
fn code_point_before_low_in_range_index_ge2_high_not_in_range() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeLowInRangeIndexGe2HighNotInRange", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeLowInRangeIndexGe2HighNotInRange");
}

#[test]
fn single_quote_pair_match() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.singleQuotePairMatch", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.singleQuotePairMatch", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[115,105,110,103,108,101,81,117,111,116,101,80,97,105,114,77,97,116,99,104]));
        let p = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(UStr::new(&[8216,8217])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((p.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_quote_type(QuoteType::Single, p[0usize].quote_type, None).unwrap();
    });
}

#[test]
fn code_point_at_or_null_lone_high_surrogate_after_quote() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullLoneHighSurrogateAfterQuote", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullLoneHighSurrogateAfterQuote");
}

#[test]
fn code_point_at_or_null_high_surrogate_at_string_end() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullHighSurrogateAtStringEnd", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullHighSurrogateAtStringEnd");
}

#[test]
fn analyze_with_all_quote_types() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.analyzeWithAllQuoteTypes", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.analyzeWithAllQuoteTypes", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[97,110,97,108,121,122,101,87,105,116,104,65,108,108,81,117,111,116,101,84,121,112,101,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(UStr::new(&[8220,8216,97,98,99,8217,8221])).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn code_point_before_non_surrogate_bmp_char() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeNonSurrogateBmpChar", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeNonSurrogateBmpChar", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(UStr::new(&[99,111,100,101,80,111,105,110,116,66,101,102,111,114,101,78,111,110,83,117,114,114,111,103,97,116,101,66,109,112,67,104,97,114]));
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(UStr::new(&[65,8217])).unwrap();
    });
}
