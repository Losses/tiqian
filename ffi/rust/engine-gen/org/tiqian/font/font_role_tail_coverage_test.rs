#![cfg(test)]

use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            FontRoleTailCoverageTestSupplementarySymbolIsUnknownBecauseItHasNoBmpCategoryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            FontRoleTailCoverageTestBmpMathAndCurrencySymbolsResolveToSymbolRoleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,82,111,108,101,84,97,105,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[115,117,112,112,108,101,109,101,110,116,97,114,121,83,121,109,98,111,108,73,115,85,110,107,110,111,119,110,66,101,99,97,117,115,101,73,116,72,97,115,78,111,66,109,112,67,97,116,101,103,111,114,121]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[85,110,107,110,111,119,110]), UString::from(CjkFontRoleClassifier::new().classify(UStr::new(&[55349,56320]), TextRange::new(0u32, 2u32).unwrap(), None).name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn bmp_math_and_currency_symbols_resolve_to_symbol_role() {
    testlib::run("org.tiqian.font.FontRoleTailCoverageTest.bmpMathAndCurrencySymbolsResolveToSymbolRole", "org.tiqian.font.FontRoleTailCoverageTest.bmpMathAndCurrencySymbolsResolveToSymbolRole", || {
        TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,82,111,108,101,84,97,105,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[98,109,112,77,97,116,104,65,110,100,67,117,114,114,101,110,99,121,83,121,109,98,111,108,115,82,101,115,111,108,118,101,84,111,83,121,109,98,111,108,82,111,108,101]));
        let xs = vec![UString::from("±").to_ustring(), UString::from("€").to_ustring()];
        let mut xi = 0u32;
        while (i32::from_ne_bytes(((xi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let x = (xs[usize::try_from(xi).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[83,121,109,98,111,108]), UString::from(CjkFontRoleClassifier::new().classify(x.as_ustr(), TextRange::new(0u32, 1u32).unwrap(), None).name()).as_ustr(), None).unwrap();
            xi = u32::wrapping_add(xi, 1);
        }
    });
}
