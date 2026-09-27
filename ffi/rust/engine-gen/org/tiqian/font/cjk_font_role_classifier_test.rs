#![cfg(test)]

use crate::org::tiqian::font::cjk_font_role_classifier_test_support::CjkFontRoleClassifierTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesLatinTextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesLatinTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesLatinTextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesLatinTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesLatinTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesLatinTextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesLatinTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesLatinTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesLatinTextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesLatinTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesLatinTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesLatinTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesLatinTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesLatinTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesLatinTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesLatinTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesCjkTextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesCjkTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCjkTextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCjkTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCjkTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesCjkTextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCjkTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCjkTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCjkTextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCjkTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesCjkTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCjkTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesCjkTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesCjkTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesCjkTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCjkTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesCjkPunctuationFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesCjkPunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCjkPunctuationFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCjkPunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCjkPunctuationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesCjkPunctuationFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCjkPunctuationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesCjkPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesCjkPunctuationFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesCjkPunctuationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesCjkPunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCjkPunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesCjkPunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesCjkPunctuationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesCjkPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesCjkPunctuationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault) -> Self {
        match value {
            CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn classifies_ascii_brackets_as_latin() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesAsciiBracketsAsLatin", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesAsciiBracketsAsLatin", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesAsciiBracketsAsLatin");
        let xs = vec![
    "(".to_string(),
    ")".to_string(),
    "[".to_string(),
    "]".to_string(),
    "{".to_string(),
    "}".to_string(),
    "中(文".to_string(),
];
        let mut xi = 0u32;
        while (i32::from_ne_bytes((xi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let x = (xs[usize::try_from(xi).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"LatinText", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(x.as_str(), if x == "中(文" { 1 } else { 0 }, if x == "中(文" { 2 } else { 1 }).unwrap().name().to_string().as_str(),
None).unwrap();
            xi = u32::wrapping_add(xi, 1);
        }
    });
}

#[test]
fn classifies_ascii_hyphen_slash_tilde_as_latin_regardless_of_context() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContext", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContext", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContext");
        let xs = vec![
    "well-known".to_string(),
    "https://example".to_string(),
    "https://example".to_string(),
    "中文/TERFism".to_string(),
    "中文-中文".to_string(),
    "中文~中文".to_string(),
];
        let mut xi = 0u32;
        while (i32::from_ne_bytes((xi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let x = (xs[usize::try_from(xi).unwrap_or(0)]).clone();
            let p = if xi == 0 { 4 } else { if xi == 1 { 6 } else { if xi == 2 { 7 } else { 2 } } };
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"LatinText", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(x.as_str(), p, u32::wrapping_add(p, 1)).unwrap().name().to_string().as_str(), None).unwrap();
            xi = u32::wrapping_add(xi, 1);
        }
    });
}

#[test]
fn classifies_ascii_symbols_and_punctuation_as_latin() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesAsciiSymbolsAndPunctuationAsLatin", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesAsciiSymbolsAndPunctuationAsLatin", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesAsciiSymbolsAndPunctuationAsLatin");
        let xs = vec![
    "%".to_string(),
    ".".to_string(),
    ",".to_string(),
    ":".to_string(),
    ";".to_string(),
    "!".to_string(),
    "?".to_string(),
    "#".to_string(),
    "@".to_string(),
    "&".to_string(),
    "*".to_string(),
    "+".to_string(),
    "=".to_string(),
    "<".to_string(),
    ">".to_string(),
    "|".to_string(),
    "^".to_string(),
    "_".to_string(),
    "$".to_string(),
    "'".to_string(),
    "\"".to_string(),
];
        let mut xi = 0u32;
        while (i32::from_ne_bytes((xi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let x = (xs[usize::try_from(xi).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"LatinText", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(x.as_str(), 0, 1).unwrap().name().to_string().as_str(), Some((format!("{}{}",
            "char=",
            x
        )).to_string())).unwrap();
            xi = u32::wrapping_add(xi, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"LatinText", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"中%文", 1, 2).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn classifies_cjk_punctuation() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCjkPunctuation", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCjkPunctuation", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesCjkPunctuation");
        let a = vec![
    "……".to_string(),
    "⋯⋯".to_string(),
    "——".to_string(),
    "⸺".to_string(),
    "。".to_string(),
    "・".to_string(),
    "‧".to_string(),
    "～".to_string(),
    "／".to_string(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let x = (a[usize::try_from(i).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkPunctuation", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(x.as_str(), 0, 1).unwrap().name().to_string().as_str(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn classifies_cjk_text() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCjkText", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCjkText", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesCjkText");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkText", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"提", 0, 1).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn classifies_curly_quotes_as_cjk_at_text_boundary() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkAtTextBoundary", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkAtTextBoundary", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesCurlyQuotesAsCjkAtTextBoundary");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkPunctuation", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"“你好”", 0, 1).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkPunctuation", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"“你好”", 3, 4).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn classifies_curly_quotes_as_cjk_in_mixed_context() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkInMixedContext", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkInMixedContext", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesCurlyQuotesAsCjkInMixedContext");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkPunctuation", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"他说“hello”", 2, 3).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkPunctuation", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"他说“hello”", 8, 9).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn classifies_curly_quotes_as_cjk_when_surrounded_by_cjk() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkWhenSurroundedByCjk", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkWhenSurroundedByCjk", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesCurlyQuotesAsCjkWhenSurroundedByCjk");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkPunctuation", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"他说“你好”", 2, 3).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkPunctuation", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"他说“你好”", 5, 6).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkPunctuation", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"他说‘你好’", 2, 3).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"CjkPunctuation", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"他说‘你好’", 5, 6).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn classifies_curly_quotes_as_latin_when_surrounded_by_latin() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsLatinWhenSurroundedByLatin", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsLatinWhenSurroundedByLatin", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesCurlyQuotesAsLatinWhenSurroundedByLatin");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"LatinText", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"said “hello” end", 5, 6).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"LatinText", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"said “hello” end", 11, 12).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"LatinText", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"it’s", 2, 3).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn classifies_latin_text() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesLatinText", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesLatinText", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesLatinText");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"LatinText", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"English", 0, 1).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn classifies_unicode_emoji_presentation_without_reclassifying_plain_keycap_bases() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBases", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBases", || {
        TestTraceRecorder::new("CjkFontRoleClassifierTest").section(&"classifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBases");
        let xs = vec!["⌚".to_string(), "🀄".to_string(), "🫪".to_string()];
        let mut xi = 0u32;
        while (i32::from_ne_bytes((xi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let x = (xs[usize::try_from(xi).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Emoji", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(x.as_str(), 0, u_string::unit_count(&(x))).unwrap().name().to_string().as_str(), Some((x).to_string())).unwrap();
            xi = u32::wrapping_add(xi, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"LatinText", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"1", 0, 1).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Symbol", CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(&"❤", 0, 1).unwrap().name().to_string().as_str(), None).unwrap();
    });
}
