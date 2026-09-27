#![cfg(test)]

use crate::org::tiqian::linebreak::unicode_punctuation_line_break::UnicodePunctuationLineBreak;
use crate::org::tiqian::linebreak::unicode_punctuation_line_break::UnicodePunctuationLineBreakClass;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault) -> Self {
        match value {
            UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault) -> Self {
        match value {
            UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault) -> Self {
        match value {
            UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationLineBreakTestOrdinaryLettersAreOutsideThePunctuationSubsetFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault) -> Self {
        match value {
            UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault) -> Self {
        match value {
            UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault) -> Self {
        match value {
            UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationLineBreakTestExposesPinnedWesternAndCjkPunctuationClassesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn exposes_pinned_western_and_cjk_punctuation_classes() {
    testlib::run("org.tiqian.linebreak.UnicodePunctuationLineBreakTest.exposesPinnedWesternAndCjkPunctuationClasses", "org.tiqian.linebreak.UnicodePunctuationLineBreakTest.exposesPinnedWesternAndCjkPunctuationClasses", || {
        TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,76,105,110,101,66,114,101,97,107,84,101,115,116]))).section(UStr::new(&[101,120,112,111,115,101,115,80,105,110,110,101,100,87,101,115,116,101,114,110,65,110,100,67,106,107,80,117,110,99,116,117,97,116,105,111,110,67,108,97,115,115,101,115]));
        let cps = vec![40, 41, 123, 125, 33, 44, 47, 45, 8230, 8220, 8221, 65288, 65289];
        let k = vec![
    UnicodePunctuationLineBreakClass::OpenPunctuation,
    UnicodePunctuationLineBreakClass::CloseParenthesis,
    UnicodePunctuationLineBreakClass::OpenPunctuation,
    UnicodePunctuationLineBreakClass::ClosePunctuation,
    UnicodePunctuationLineBreakClass::Exclamation,
    UnicodePunctuationLineBreakClass::InfixNumericSeparator,
    UnicodePunctuationLineBreakClass::SymbolsAllowingBreakAfter,
    UnicodePunctuationLineBreakClass::Hyphen,
    UnicodePunctuationLineBreakClass::Inseparable,
    UnicodePunctuationLineBreakClass::Quotation,
    UnicodePunctuationLineBreakClass::Quotation,
    UnicodePunctuationLineBreakClass::OpenPunctuation,
    UnicodePunctuationLineBreakClass::ClosePunctuation,
];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((cps.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(k[usize::try_from(i).unwrap_or(0)].name()).as_ustr(), UString::from(UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(cps[usize::try_from(i).unwrap_or(0)]).unwrap().name()).as_ustr(), Some((if cps[usize::try_from(i).unwrap_or(0)] > 0xFFFF { u_string::from_units(&[0xD800 + (((cps[usize::try_from(i).unwrap_or(0)]) - 0x10000) >> 10) as u16, 0xDC00 + (((cps[usize::try_from(i).unwrap_or(0)]) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(cps[usize::try_from(i).unwrap_or(0)]) as u16]) }).to_ustring())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn ordinary_letters_are_outside_the_punctuation_subset() {
    testlib::run("org.tiqian.linebreak.UnicodePunctuationLineBreakTest.ordinaryLettersAreOutsideThePunctuationSubset", "org.tiqian.linebreak.UnicodePunctuationLineBreakTest.ordinaryLettersAreOutsideThePunctuationSubset", || {
        TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,76,105,110,101,66,114,101,97,107,84,101,115,116]))).section(UStr::new(&[111,114,100,105,110,97,114,121,76,101,116,116,101,114,115,65,114,101,79,117,116,115,105,100,101,84,104,101,80,117,110,99,116,117,97,116,105,111,110,83,117,98,115,101,116]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[79,116,104,101,114]), UString::from(UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(65).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[79,116,104,101,114]), UString::from(UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(20013).unwrap().name()).as_ustr(), None).unwrap();
    });
}
