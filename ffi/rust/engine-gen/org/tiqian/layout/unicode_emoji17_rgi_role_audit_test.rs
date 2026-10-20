#![cfg(test)]

use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::cluster_role_resolution::ClusterRoleResolution;
use crate::org::tiqian::layout::unicode_emoji17_rgi_role_audit_test_support::UnicodeEmoji17RgiRoleAuditTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault) -> Self {
        match value {
            UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault) -> Self {
        match value {
            UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault) -> Self {
        match value {
            UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodeEmoji17RgiRoleAuditTestFullyQualifiedEmojiSequencesResolveToOneEmojiRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn fully_qualified_emoji_sequences_resolve_to_one_emoji_range() {
    testlib::run("org.tiqian.layout.UnicodeEmoji17RgiRoleAuditTest.fullyQualifiedEmojiSequencesResolveToOneEmojiRange", "org.tiqian.layout.UnicodeEmoji17RgiRoleAuditTest.fullyQualifiedEmojiSequencesResolveToOneEmojiRange", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,69,109,111,106,105,49,55,82,103,105,82,111,108,101,65,117,100,105,116,84,101,115,116])));
        test_trace.section(UStr::new(&[102,117,108,108,121,81,117,97,108,105,102,105,101,100,69,109,111,106,105,83,101,113,117,101,110,99,101,115,82,101,115,111,108,118,101,84,111,79,110,101,69,109,111,106,105,82,97,110,103,101]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3944, u32::try_from(((*crate::org::tiqian::layout::unicode_emoji17_rgi_role_audit_test_support::UNICODE_EMOJI17_RGI_ROLE_AUDIT_TEST_SUPPORT_FULLY_QUALIFIED_CODE_POINT_SEQUENCES).clone().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let classifier = CjkFontRoleClassifier::new();
        let mut failures: Vec<UString> = vec![];
        {
            let _g1 = (*crate::org::tiqian::layout::unicode_emoji17_rgi_role_audit_test_support::UNICODE_EMOJI17_RGI_ROLE_AUDIT_TEST_SUPPORT_FULLY_QUALIFIED_CODE_POINT_SEQUENCES).clone();
            for code_points in &_g1 {
                let text = UnicodeEmoji17RgiRoleAuditTestSupport::unicode_emoji17_rgi_role_audit_test_support_to_unicode_string(code_points.as_ustr());
                let ranges = ClusterRoleResolution::cluster_role_resolution_cluster_role_ranges(text.as_ustr(), (Box::new((classifier).clone())).clone(), FontRoleContext::new(Some(UString::from("zh-Hans")), None), (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone(), SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(), SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(), None).unwrap();
                if u32::try_from((ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) != 1 || ranges[0usize].role != FontRole::Emoji || ((ranges[0usize]).clone().range).clone().start != 0 || ((ranges[0usize]).clone().range).clone().end != u_string::unit_count(&(text)) {
                    failures.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += code_points.as_ustr(); __s += &(UString::from(": expected=[TextRange(start=0, end=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(u_string::unit_count(&(text)))).as_str())); __s += &(UString::from(") to Emoji] actual=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = ranges;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s }).as_str()));
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from((failures.len()) & 0xFFFF_FFFF).unwrap_or(0), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(u32::try_from((failures.len()) & 0xFFFF_FFFF).unwrap_or(0))).as_str())); __s += &(UString::from(" RGI role mismatches: ")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = { let _a = &(failures); let _n = i32::try_from(_a.len()).unwrap_or(0); let _from = 0i32; let _start = if _from < 0 { let _tail = _n + _from; if _tail < 0 { 0 } else { _tail } } else if _from > _n { _n } else { _from }; let _to = 20i32; let _end = if _to < 0 { let _tail = _n + _to; if _tail < 0 { 0 } else { _tail } } else if _to > _n { _n } else { _to }; let _stop = if _end < _start { _start } else { _end }; _a[usize::try_from(_start).unwrap_or(0)..usize::try_from(_stop).unwrap_or(0)].to_vec() };
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s }).as_str()))).unwrap();
    });
}
