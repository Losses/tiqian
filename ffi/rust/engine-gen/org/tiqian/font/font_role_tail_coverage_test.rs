#![cfg(test)]

use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault) -> Self {
        match value {
            FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault) -> Self {
        match value {
            FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault) -> Self {
        match value {
            FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault) -> Self {
        match value {
            FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault) -> Self {
        match value {
            FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault) -> Self {
        match value {
            FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn supplementary_symbol_is_unknown_because_it_has_no_bmp_category() {
    testlib::run("org.tiqian.font.FontRoleTailCoverageTest.supplementarySymbolIsUnknownBecauseItHasNoBmpCategory", "org.tiqian.font.FontRoleTailCoverageTest.supplementarySymbolIsUnknownBecauseItHasNoBmpCategory", || {
        TestTraceRecorder::new("FontRoleTailCoverageTest").section(&"supplementarySymbolIsUnknownBecauseItHasNoBmpCategory");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Unknown", CjkFontRoleClassifier::new().classify(&"𝐀", TextRange::new(0u32, 2u32).unwrap(), None).name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn bmp_math_and_currency_symbols_resolve_to_symbol_role() {
    testlib::run("org.tiqian.font.FontRoleTailCoverageTest.bmpMathAndCurrencySymbolsResolveToSymbolRole", "org.tiqian.font.FontRoleTailCoverageTest.bmpMathAndCurrencySymbolsResolveToSymbolRole", || {
        TestTraceRecorder::new("FontRoleTailCoverageTest").section(&"bmpMathAndCurrencySymbolsResolveToSymbolRole");
        let xs = vec!["±".to_string(), "€".to_string()];
        let mut xi = 0u32;
        while (i32::from_ne_bytes((xi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let x = (xs[usize::try_from(xi).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Symbol", CjkFontRoleClassifier::new().classify(x.as_str(), TextRange::new(0u32, 1u32).unwrap(), None).name().to_string().as_str(), None).unwrap();
            xi = u32::wrapping_add(xi, 1);
        }
    });
}
