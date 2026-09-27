#![cfg(test)]

use crate::org::tiqian::core::unicode_script_evidence::UnicodeScriptEvidence;
use crate::org::tiqian::core::unicode_script_evidence_classifier::UnicodeScriptEvidenceClassifier;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault) -> Self {
        match value {
            UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault) -> Self {
        match value {
            UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault) -> Self {
        match value {
            UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodeScriptEvidenceTestEastAsianScriptsAreDistinctFromOtherStrongScriptsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault) -> Self {
        match value {
            UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault) -> Self {
        match value {
            UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault) -> Self {
        match value {
            UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodeScriptEvidenceTestCommonAndInheritedScalarsDoNotVoteFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn common_and_inherited_scalars_do_not_vote() {
    testlib::run("org.tiqian.core.UnicodeScriptEvidenceTest.commonAndInheritedScalarsDoNotVote", "org.tiqian.core.UnicodeScriptEvidenceTest.commonAndInheritedScalarsDoNotVote", || {
        TestTraceRecorder::new("UnicodeScriptEvidenceTest").section(&"commonAndInheritedScalarsDoNotVote");
        let code_points = vec![32, 48, 8220, 65311, 769, 128512];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((code_points.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let code_point = code_points[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UnicodeScriptEvidence::Neutral.name().to_string().as_str(), UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(code_point).unwrap().name().to_string().as_str(),
Some((format!("{}{}",
            "U+",
            format!("{:0w$X}", code_point, w = usize::try_from(0).unwrap_or_default()).to_lowercase()
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}

#[test]
fn east_asian_scripts_are_distinct_from_other_strong_scripts() {
    testlib::run("org.tiqian.core.UnicodeScriptEvidenceTest.eastAsianScriptsAreDistinctFromOtherStrongScripts", "org.tiqian.core.UnicodeScriptEvidenceTest.eastAsianScriptsAreDistinctFromOtherStrongScripts", || {
        TestTraceRecorder::new("UnicodeScriptEvidenceTest").section(&"eastAsianScriptsAreDistinctFromOtherStrongScripts");
        let east_asian_code_points = vec![16397, 12549, 12354, 12450, 44032, 131072];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((east_asian_code_points.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let code_point = east_asian_code_points[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UnicodeScriptEvidence::EastAsian.name().to_string().as_str(), UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(code_point).unwrap().name().to_string().as_str(),
Some((format!("{}{}",
            "U+",
            format!("{:0w$X}", code_point, w = usize::try_from(0).unwrap_or_default()).to_lowercase()
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let other_code_points = vec![65, 960, 1046, 1575];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((other_code_points.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let code_point = other_code_points[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UnicodeScriptEvidence::Other.name().to_string().as_str(), UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(code_point).unwrap().name().to_string().as_str(),
Some((format!("{}{}",
            "U+",
            format!("{:0w$X}", code_point, w = usize::try_from(0).unwrap_or_default()).to_lowercase()
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}
