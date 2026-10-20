#![cfg(test)]

use crate::org::tiqian::font::cjk_dash_capability_policy::CjkDashCapabilityPolicy;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn null_status_names_missing_conforming_glyph_and_unprepared_detail() {
    testlib::run("org.tiqian.font.CjkDashCapabilityPolicyTest.nullStatusNamesMissingConformingGlyphAndUnpreparedDetail", "org.tiqian.font.CjkDashCapabilityPolicyTest.nullStatusNamesMissingConformingGlyphAndUnpreparedDetail", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,68,97,115,104,67,97,112,97,98,105,108,105,116,121,80,111,108,105,99,121,84,101,115,116]))).section(UStr::new(&[110,117,108,108,83,116,97,116,117,115,78,97,109,101,115,77,105,115,115,105,110,103,67,111,110,102,111,114,109,105,110,103,71,108,121,112,104,65,110,100,85,110,112,114,101,112,97,114,101,100,68,101,116,97,105,108]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[78,111,67,111,110,102,111,114,109,105,110,103,67,106,107,68,97,115,104,71,108,121,112,104]), CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_name_for(None.clone()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,106,107,68,97,115,104,70,111,110,116,83,104,97,112,105,110,103,78,111,116,80,114,101,112,97,114,101,100]), CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_detail_for(None.clone(), None.clone()).as_ustr(), None).unwrap();
    });
}

#[test]
fn conforming_status_with_blank_detail_names_the_missing_session() {
    testlib::run("org.tiqian.font.CjkDashCapabilityPolicyTest.conformingStatusWithBlankDetailNamesTheMissingSession", "org.tiqian.font.CjkDashCapabilityPolicyTest.conformingStatusWithBlankDetailNamesTheMissingSession", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,68,97,115,104,67,97,112,97,98,105,108,105,116,121,80,111,108,105,99,121,84,101,115,116]))).section(UStr::new(&[99,111,110,102,111,114,109,105,110,103,83,116,97,116,117,115,87,105,116,104,66,108,97,110,107,68,101,116,97,105,108,78,97,109,101,115,84,104,101,77,105,115,115,105,110,103,83,101,115,115,105,111,110]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,111,110,102,111,114,109,105,110,103,67,106,107,68,97,115,104,82,101,113,117,105,114,101,115,69,120,97,99,116,70,111,110,116,83,101,115,115,105,111,110]), CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_name_for(Some(UString::from("conforming"))).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[115,116,97,116,117,115,61,99,111,110,102,111,114,109,105,110,103]), CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_detail_for(Some(UString::from("conforming")), Some(UString::from("  "))).as_ustr(), None).unwrap();
    });
}

#[test]
fn conforming_status_with_detail_appends_host_evidence() {
    testlib::run("org.tiqian.font.CjkDashCapabilityPolicyTest.conformingStatusWithDetailAppendsHostEvidence", "org.tiqian.font.CjkDashCapabilityPolicyTest.conformingStatusWithDetailAppendsHostEvidence", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,68,97,115,104,67,97,112,97,98,105,108,105,116,121,80,111,108,105,99,121,84,101,115,116]))).section(UStr::new(&[99,111,110,102,111,114,109,105,110,103,83,116,97,116,117,115,87,105,116,104,68,101,116,97,105,108,65,112,112,101,110,100,115,72,111,115,116,69,118,105,100,101,110,99,101]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,111,110,102,111,114,109,105,110,103,67,106,107,68,97,115,104,82,101,113,117,105,114,101,115,69,120,97,99,116,70,111,110,116,83,101,115,115,105,111,110]), CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_name_for(Some(UString::from("conforming"))).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[115,116,97,116,117,115,61,99,111,110,102,111,114,109,105,110,103,59,32,70,105,120,116,117,114,101,68,97,115,104,70,97,99,101]), CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_detail_for(Some(UString::from("conforming")), Some(UString::from("FixtureDashFace"))).as_ustr(), None).unwrap();
    });
}

#[test]
fn non_conforming_status_with_detail_names_missing_glyph_and_appends_evidence() {
    testlib::run("org.tiqian.font.CjkDashCapabilityPolicyTest.nonConformingStatusWithDetailNamesMissingGlyphAndAppendsEvidence", "org.tiqian.font.CjkDashCapabilityPolicyTest.nonConformingStatusWithDetailNamesMissingGlyphAndAppendsEvidence", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,68,97,115,104,67,97,112,97,98,105,108,105,116,121,80,111,108,105,99,121,84,101,115,116]))).section(UStr::new(&[110,111,110,67,111,110,102,111,114,109,105,110,103,83,116,97,116,117,115,87,105,116,104,68,101,116,97,105,108,78,97,109,101,115,77,105,115,115,105,110,103,71,108,121,112,104,65,110,100,65,112,112,101,110,100,115,69,118,105,100,101,110,99,101]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[78,111,67,111,110,102,111,114,109,105,110,103,67,106,107,68,97,115,104,71,108,121,112,104]), CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_name_for(Some(UString::from("unavailable"))).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[115,116,97,116,117,115,61,117,110,97,118,97,105,108,97,98,108,101,59,32,66,114,111,119,115,101,114,72,97,114,102,66,117,122,122,68,105,115,97,98,108,101,100]), CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_detail_for(Some(UString::from("unavailable")), Some(UString::from("BrowserHarfBuzzDisabled"))).as_ustr(), None).unwrap();
    });
}

#[test]
fn non_conforming_status_with_blank_detail_keeps_only_status_prefix() {
    testlib::run("org.tiqian.font.CjkDashCapabilityPolicyTest.nonConformingStatusWithBlankDetailKeepsOnlyStatusPrefix", "org.tiqian.font.CjkDashCapabilityPolicyTest.nonConformingStatusWithBlankDetailKeepsOnlyStatusPrefix", || {
        TestTraceRecorder::new(&(UStr::new(&[67,106,107,68,97,115,104,67,97,112,97,98,105,108,105,116,121,80,111,108,105,99,121,84,101,115,116]))).section(UStr::new(&[110,111,110,67,111,110,102,111,114,109,105,110,103,83,116,97,116,117,115,87,105,116,104,66,108,97,110,107,68,101,116,97,105,108,75,101,101,112,115,79,110,108,121,83,116,97,116,117,115,80,114,101,102,105,120]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[78,111,67,111,110,102,111,114,109,105,110,103,67,106,107,68,97,115,104,71,108,121,112,104]), CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_name_for(Some(UString::from("unavailable"))).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[115,116,97,116,117,115,61,117,110,97,118,97,105,108,97,98,108,101]), CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_detail_for(Some(UString::from("unavailable")), None.clone()).as_ustr(), None).unwrap();
    });
}
