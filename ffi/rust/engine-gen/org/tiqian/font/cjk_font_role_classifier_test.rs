#![cfg(test)]

use crate::org::tiqian::font::cjk_font_role_classifier_test_support::CjkFontRoleClassifierTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBasesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesLatinTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesLatinTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesLatinTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesLatinTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsLatinWhenSurroundedByLatinFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkWhenSurroundedByCjkFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkInMixedContextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCurlyQuotesAsCjkAtTextBoundaryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesCjkTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesCjkTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCjkTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCjkTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesCjkPunctuationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesCjkPunctuationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCjkPunctuationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesCjkPunctuationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesAsciiSymbolsAndPunctuationAsLatinFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CjkFontRoleClassifierTestClassifiesAsciiBracketsAsLatinFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,65,115,99,105,105,66,114,97,99,107,101,116,115,65,115,76,97,116,105,110]));
        let xs = vec![
    UString::from("(").to_ustring(),
    UString::from(")").to_ustring(),
    UString::from("[").to_ustring(),
    UString::from("]").to_ustring(),
    UString::from("{").to_ustring(),
    UString::from("}").to_ustring(),
    UString::from("中(文").to_ustring(),
];
        let mut xi = 0u32;
        while (i32::from_ne_bytes(((xi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let x = (xs[usize::try_from(xi).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[76,97,116,105,110,84,101,120,116]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(x.as_ustr(), if x == UString::from("中(文") { 1 } else { 0 }, if x == UString::from("中(文") { 2 } else { 1 }).unwrap().name()).as_ustr(), None).unwrap();
            xi = u32::wrapping_add(xi, 1);
        }
    });
}

#[test]
fn classifies_ascii_hyphen_slash_tilde_as_latin_regardless_of_context() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContext", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesAsciiHyphenSlashTildeAsLatinRegardlessOfContext", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,65,115,99,105,105,72,121,112,104,101,110,83,108,97,115,104,84,105,108,100,101,65,115,76,97,116,105,110,82,101,103,97,114,100,108,101,115,115,79,102,67,111,110,116,101,120,116]));
        let xs = vec![
    UString::from("well-known").to_ustring(),
    UString::from("https://example").to_ustring(),
    UString::from("https://example").to_ustring(),
    UString::from("中文/TERFism").to_ustring(),
    UString::from("中文-中文").to_ustring(),
    UString::from("中文~中文").to_ustring(),
];
        let mut xi = 0u32;
        while (i32::from_ne_bytes(((xi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let x = (xs[usize::try_from(xi).unwrap_or(0)]).clone();
            let p = if xi == 0 { 4 } else { if xi == 1 { 6 } else { if xi == 2 { 7 } else { 2 } } };
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[76,97,116,105,110,84,101,120,116]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(x.as_ustr(), p, u32::wrapping_add(p, 1)).unwrap().name()).as_ustr(), None).unwrap();
            xi = u32::wrapping_add(xi, 1);
        }
    });
}

#[test]
fn classifies_ascii_symbols_and_punctuation_as_latin() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesAsciiSymbolsAndPunctuationAsLatin", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesAsciiSymbolsAndPunctuationAsLatin", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,65,115,99,105,105,83,121,109,98,111,108,115,65,110,100,80,117,110,99,116,117,97,116,105,111,110,65,115,76,97,116,105,110]));
        let xs = vec![
    UString::from("%").to_ustring(),
    UString::from(".").to_ustring(),
    UString::from(",").to_ustring(),
    UString::from(":").to_ustring(),
    UString::from(";").to_ustring(),
    UString::from("!").to_ustring(),
    UString::from("?").to_ustring(),
    UString::from("#").to_ustring(),
    UString::from("@").to_ustring(),
    UString::from("&").to_ustring(),
    UString::from("*").to_ustring(),
    UString::from("+").to_ustring(),
    UString::from("=").to_ustring(),
    UString::from("<").to_ustring(),
    UString::from(">").to_ustring(),
    UString::from("|").to_ustring(),
    UString::from("^").to_ustring(),
    UString::from("_").to_ustring(),
    UString::from("$").to_ustring(),
    UString::from("'").to_ustring(),
    UString::from("\"").to_ustring(),
];
        let mut xi = 0u32;
        while (i32::from_ne_bytes(((xi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let x = (xs[usize::try_from(xi).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[76,97,116,105,110,84,101,120,116]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(x.as_ustr(), 0, 1).unwrap().name()).as_ustr(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("char=")); __s += x.as_ustr(); __s }).as_str()))).unwrap();
            xi = u32::wrapping_add(xi, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[76,97,116,105,110,84,101,120,116]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[20013,37,25991]), 1, 2).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn classifies_cjk_punctuation() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCjkPunctuation", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCjkPunctuation", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,67,106,107,80,117,110,99,116,117,97,116,105,111,110]));
        let a = vec![
    UString::from("……").to_ustring(),
    UString::from("⋯⋯").to_ustring(),
    UString::from("——").to_ustring(),
    UString::from("⸺").to_ustring(),
    UString::from("。").to_ustring(),
    UString::from("・").to_ustring(),
    UString::from("‧").to_ustring(),
    UString::from("～").to_ustring(),
    UString::from("／").to_ustring(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let x = (a[usize::try_from(i).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(x.as_ustr(), 0, 1).unwrap().name()).as_ustr(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn classifies_cjk_text() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCjkText", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCjkText", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,67,106,107,84,101,120,116]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[67,106,107,84,101,120,116]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[25552]), 0, 1).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn classifies_curly_quotes_as_cjk_at_text_boundary() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkAtTextBoundary", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkAtTextBoundary", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,67,117,114,108,121,81,117,111,116,101,115,65,115,67,106,107,65,116,84,101,120,116,66,111,117,110,100,97,114,121]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[8220,20320,22909,8221]), 0, 1).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[8220,20320,22909,8221]), 3, 4).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn classifies_curly_quotes_as_cjk_in_mixed_context() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkInMixedContext", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkInMixedContext", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,67,117,114,108,121,81,117,111,116,101,115,65,115,67,106,107,73,110,77,105,120,101,100,67,111,110,116,101,120,116]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[20182,35828,8220,104,101,108,108,111,8221]), 2, 3).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[20182,35828,8220,104,101,108,108,111,8221]), 8, 9).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn classifies_curly_quotes_as_cjk_when_surrounded_by_cjk() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkWhenSurroundedByCjk", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsCjkWhenSurroundedByCjk", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,67,117,114,108,121,81,117,111,116,101,115,65,115,67,106,107,87,104,101,110,83,117,114,114,111,117,110,100,101,100,66,121,67,106,107]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[20182,35828,8220,20320,22909,8221]), 2, 3).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[20182,35828,8220,20320,22909,8221]), 5, 6).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[20182,35828,8216,20320,22909,8217]), 2, 3).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[20182,35828,8216,20320,22909,8217]), 5, 6).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn classifies_curly_quotes_as_latin_when_surrounded_by_latin() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsLatinWhenSurroundedByLatin", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesCurlyQuotesAsLatinWhenSurroundedByLatin", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,67,117,114,108,121,81,117,111,116,101,115,65,115,76,97,116,105,110,87,104,101,110,83,117,114,114,111,117,110,100,101,100,66,121,76,97,116,105,110]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[76,97,116,105,110,84,101,120,116]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[115,97,105,100,32,8220,104,101,108,108,111,8221,32,101,110,100]), 5, 6).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[76,97,116,105,110,84,101,120,116]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[115,97,105,100,32,8220,104,101,108,108,111,8221,32,101,110,100]), 11, 12).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[76,97,116,105,110,84,101,120,116]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[105,116,8217,115]), 2, 3).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn classifies_latin_text() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesLatinText", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesLatinText", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,76,97,116,105,110,84,101,120,116]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[76,97,116,105,110,84,101,120,116]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[69,110,103,108,105,115,104]), 0, 1).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn classifies_unicode_emoji_presentation_without_reclassifying_plain_keycap_bases() {
    testlib::run("org.tiqian.font.CjkFontRoleClassifierTest.classifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBases", "org.tiqian.font.CjkFontRoleClassifierTest.classifiesUnicodeEmojiPresentationWithoutReclassifyingPlainKeycapBases", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,84,101,115,116]))).section(UStr::new(&[99,108,97,115,115,105,102,105,101,115,85,110,105,99,111,100,101,69,109,111,106,105,80,114,101,115,101,110,116,97,116,105,111,110,87,105,116,104,111,117,116,82,101,99,108,97,115,115,105,102,121,105,110,103,80,108,97,105,110,75,101,121,99,97,112,66,97,115,101,115]));
        let xs = vec![
    UString::from("⌚").to_ustring(),
    UString::from("🀄").to_ustring(),
    UString::from("🫪").to_ustring(),
];
        let mut xi = 0u32;
        while (i32::from_ne_bytes(((xi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let x = (xs[usize::try_from(xi).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[69,109,111,106,105]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(x.as_ustr(), 0, u_string::unit_count(&(x))).unwrap().name()).as_ustr(), Some((x).to_ustring())).unwrap();
            xi = u32::wrapping_add(xi, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[76,97,116,105,110,84,101,120,116]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[49]), 0, 1).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[83,121,109,98,111,108]), UString::from(CjkFontRoleClassifierTestSupport::cjk_font_role_classifier_test_support_c(UStr::new(&[10084]), 0, 1).unwrap().name()).as_ustr(), None).unwrap();
    });
}
