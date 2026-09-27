#![cfg(test)]

use crate::org::tiqian::core::unicode_word_character::UnicodeWordCharacter;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault) -> Self {
        match value {
            UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault) -> Self {
        match value {
            UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault) -> Self {
        match value {
            UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodeWordCharacterTestLettersAndNumbersAreWordCharactersAcrossScriptsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn letters_and_numbers_are_word_characters_across_scripts() {
    testlib::run("org.tiqian.core.UnicodeWordCharacterTest.lettersAndNumbersAreWordCharactersAcrossScripts", "org.tiqian.core.UnicodeWordCharacterTest.lettersAndNumbersAreWordCharactersAcrossScripts", || {
        TestTraceRecorder::new("UnicodeWordCharacterTest").section(&"lettersAndNumbersAreWordCharactersAcrossScripts");
        let positives = vec![65, 50, 16397, 769, 960, 1046, 1634, 131072];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((positives.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let code_point = positives[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_assert_true(UnicodeWordCharacter::unicode_word_character_contains(code_point).unwrap(), Some((format!("{}{}",
            "U+",
            format!("{:0w$X}", code_point, w = usize::try_from(0).unwrap_or_default()).to_lowercase()
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let negatives = vec![32, 8217, 65311, 128512];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((negatives.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let code_point = negatives[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_assert_false(UnicodeWordCharacter::unicode_word_character_contains(code_point).unwrap(), Some((format!("{}{}",
            "U+",
            format!("{:0w$X}", code_point, w = usize::try_from(0).unwrap_or_default()).to_lowercase()
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}
