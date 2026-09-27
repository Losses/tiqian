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
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerCoverageTestSingleQuotePairMatchFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsQuoteTypeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsQuoteTypeFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"deprecatedClassifyPairsWithFontRoleClassifierDelegates");
        let t = "他说“你好”".to_string();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().classify_pairs_with_classifier(t.as_str(),
&QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(t.as_str()).unwrap(), Box::new(CjkFontRoleClassifier::new()), None).unwrap().get(&(2)), None).unwrap();
    });
}

#[test]
fn deprecated_classify_quote_roles_with_font_role_classifier_delegates() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.deprecatedClassifyQuoteRolesWithFontRoleClassifierDelegates", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.deprecatedClassifyQuoteRolesWithFontRoleClassifierDelegates", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"deprecatedClassifyQuoteRolesWithFontRoleClassifierDelegates");
        let t = "他说“你好”".to_string();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().classify_quote_roles_with_classifier(t.as_str(),
&QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(t.as_str()).unwrap(), Box::new(CjkFontRoleClassifier::new()), None).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn code_point_before_surrogate_pair_returns_supplementary() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeSurrogatePairReturnsSupplementary", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeSurrogatePairReturnsSupplementary", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"codePointBeforeSurrogatePairReturnsSupplementary");
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832, 8217]).as_str()).unwrap().len()) &
0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn code_point_at_or_null_surrogate_pair_returns_supplementary() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullSurrogatePairReturnsSupplementary", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullSurrogatePairReturnsSupplementary", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"codePointAtOrNullSurrogatePairReturnsSupplementary");
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(TestHelpers::test_helpers_surrogate_text(&vec![8217, 55357, 56832]).as_str()).unwrap();
    });
}

#[test]
fn code_point_at_or_null_non_surrogate_returns_self() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullNonSurrogateReturnsSelf", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullNonSurrogateReturnsSelf", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"codePointAtOrNullNonSurrogateReturnsSelf");
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().classify_quote_roles(&"abc", &vec![], None).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn code_point_before_returns_null_at_start() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeReturnsNullAtStart", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeReturnsNullAtStart", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"codePointBeforeReturnsNullAtStart");
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(&"’").unwrap();
    });
}

#[test]
fn code_point_before_returns_supplementary_for_surrogate_pair() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeReturnsSupplementaryForSurrogatePair", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeReturnsSupplementaryForSurrogatePair", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"codePointBeforeReturnsSupplementaryForSurrogatePair");
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832, 8217]).as_str()).unwrap();
    });
}

#[test]
fn quote_pair_aware_font_role_classifier_uses_override() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.quotePairAwareFontRoleClassifierUsesOverride", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.quotePairAwareFontRoleClassifierUsesOverride", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"quotePairAwareFontRoleClassifierUsesOverride");
        let mut b: SortedMapTableBuilder<u32, FontRole> = SortedTable::sorted_table_map_builder::<u32, FontRole>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        b.put(&(2), &(FontRole::LatinText));
        let c = QuotePairAwareFontRoleClassifier::new(Box::new(CjkFontRoleClassifier::new()), b.clone().build());
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(c.classify(&"ab", TextRange::new(0u32, 2u32).unwrap(), Some(FontRoleContext::new(Some("zh-Hans".to_string()), None)))), None).unwrap();
    });
}

#[test]
fn quote_pair_aware_font_role_classifier_delegates_when_no_override() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.quotePairAwareFontRoleClassifierDelegatesWhenNoOverride", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.quotePairAwareFontRoleClassifierDelegatesWhenNoOverride", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"quotePairAwareFontRoleClassifierDelegatesWhenNoOverride");
        let b: SortedMapTableBuilder<u32, FontRole> = SortedTable::sorted_table_map_builder::<u32, FontRole>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let c = CjkFontRoleClassifier::new();
        let w = QuotePairAwareFontRoleClassifier::new(Box::new((c).clone()), b.clone().build());
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(c.classify(&"ab", TextRange::new(0u32, 2u32).unwrap(), Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))), Some(w.classify(&"ab", TextRange::new(0u32, 2u32).unwrap(),
Some(FontRoleContext::new(Some("zh-Hans".to_string()), None)))), None).unwrap();
    });
}

#[test]
fn double_quote_close_with_empty_stack_ignores() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.doubleQuoteCloseWithEmptyStackIgnores", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.doubleQuoteCloseWithEmptyStackIgnores", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"doubleQuoteCloseWithEmptyStackIgnores");
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(&"”").unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn single_quote_close_with_empty_stack_ignores() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.singleQuoteCloseWithEmptyStackIgnores", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.singleQuoteCloseWithEmptyStackIgnores", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"singleQuoteCloseWithEmptyStackIgnores");
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(&"’").unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn in_word_apostrophe_after_supplementary_does_not_close() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.inWordApostropheAfterSupplementaryDoesNotClose", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.inWordApostropheAfterSupplementaryDoesNotClose", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"inWordApostropheAfterSupplementaryDoesNotClose");
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832, 8217, 120]).as_str()).unwrap();
    });
}

#[test]
fn code_point_at_or_null_with_supplementary_after_quote() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullWithSupplementaryAfterQuote", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullWithSupplementaryAfterQuote", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"codePointAtOrNullWithSupplementaryAfterQuote");
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(TestHelpers::test_helpers_surrogate_text(&vec![97, 8217, 55357, 56832]).as_str()).unwrap();
    });
}

#[test]
fn code_point_before_with_high_surrogate_before_quote() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeWithHighSurrogateBeforeQuote", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeWithHighSurrogateBeforeQuote", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"codePointBeforeWithHighSurrogateBeforeQuote");
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832, 8217]).as_str()).unwrap();
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
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"codePointAtOrNullWithIndexOutOfRange");
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(&"a’").unwrap();
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
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"analyzeWithDoubleQuoteOpen");
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(&"“abc").unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn code_point_at_or_null_high_surrogate_not_in_range_returns_high() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullHighSurrogateNotInRangeReturnsHigh", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointAtOrNullHighSurrogateNotInRangeReturnsHigh", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"codePointAtOrNullHighSurrogateNotInRangeReturnsHigh");
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(&"x’a").unwrap();
    });
}

#[test]
fn code_point_before_low_in_range_index_ge2_high_not_in_range() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeLowInRangeIndexGe2HighNotInRange", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeLowInRangeIndexGe2HighNotInRange");
}

#[test]
fn single_quote_pair_match() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.singleQuotePairMatch", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.singleQuotePairMatch", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"singleQuotePairMatch");
        let p = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(&"‘’").unwrap();
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
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"analyzeWithAllQuoteTypes");
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_a().analyze(&"“‘abc’”").unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn code_point_before_non_surrogate_bmp_char() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeNonSurrogateBmpChar", "org.tiqian.layout.QuotePairAnalyzerCoverageTest.codePointBeforeNonSurrogateBmpChar", || {
        QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_rec(&"codePointBeforeNonSurrogateBmpChar");
        let _ = QuotePairAnalyzerCoverageTestSupport::quote_pair_analyzer_coverage_test_support_non_empty(&"A’").unwrap();
    });
}
