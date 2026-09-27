#![cfg(test)]

use crate::org::tiqian::font::cjk_dash_capability_policy::CjkDashCapabilityPolicy;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[test]
fn null_status_names_missing_conforming_glyph_and_unprepared_detail() {
    testlib::run("org.tiqian.font.CjkDashCapabilityPolicyTest.nullStatusNamesMissingConformingGlyphAndUnpreparedDetail", "org.tiqian.font.CjkDashCapabilityPolicyTest.nullStatusNamesMissingConformingGlyphAndUnpreparedDetail", || {
        TestTraceRecorder::new("CjkDashCapabilityPolicyTest").section(&"nullStatusNamesMissingConformingGlyphAndUnpreparedDetail");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"NoConformingCjkDashGlyph", CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_name_for(None.clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CjkDashFontShapingNotPrepared", CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_detail_for(None.clone(), None.clone()).as_str(), None).unwrap();
    });
}

#[test]
fn conforming_status_with_blank_detail_names_the_missing_session() {
    testlib::run("org.tiqian.font.CjkDashCapabilityPolicyTest.conformingStatusWithBlankDetailNamesTheMissingSession", "org.tiqian.font.CjkDashCapabilityPolicyTest.conformingStatusWithBlankDetailNamesTheMissingSession", || {
        TestTraceRecorder::new("CjkDashCapabilityPolicyTest").section(&"conformingStatusWithBlankDetailNamesTheMissingSession");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ConformingCjkDashRequiresExactFontSession", CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_name_for(Some("conforming".to_string())).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"status=conforming", CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_detail_for(Some("conforming".to_string()), Some("  ".to_string())).as_str(), None).unwrap();
    });
}

#[test]
fn conforming_status_with_detail_appends_host_evidence() {
    testlib::run("org.tiqian.font.CjkDashCapabilityPolicyTest.conformingStatusWithDetailAppendsHostEvidence", "org.tiqian.font.CjkDashCapabilityPolicyTest.conformingStatusWithDetailAppendsHostEvidence", || {
        TestTraceRecorder::new("CjkDashCapabilityPolicyTest").section(&"conformingStatusWithDetailAppendsHostEvidence");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ConformingCjkDashRequiresExactFontSession", CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_name_for(Some("conforming".to_string())).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"status=conforming; FixtureDashFace", CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_detail_for(Some("conforming".to_string()), Some("FixtureDashFace".to_string())).as_str(), None).unwrap();
    });
}

#[test]
fn non_conforming_status_with_detail_names_missing_glyph_and_appends_evidence() {
    testlib::run("org.tiqian.font.CjkDashCapabilityPolicyTest.nonConformingStatusWithDetailNamesMissingGlyphAndAppendsEvidence", "org.tiqian.font.CjkDashCapabilityPolicyTest.nonConformingStatusWithDetailNamesMissingGlyphAndAppendsEvidence", || {
        TestTraceRecorder::new("CjkDashCapabilityPolicyTest").section(&"nonConformingStatusWithDetailNamesMissingGlyphAndAppendsEvidence");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"NoConformingCjkDashGlyph", CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_name_for(Some("unavailable".to_string())).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"status=unavailable; BrowserHarfBuzzDisabled", CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_detail_for(Some("unavailable".to_string()), Some("BrowserHarfBuzzDisabled".to_string())).as_str(),
None).unwrap();
    });
}

#[test]
fn non_conforming_status_with_blank_detail_keeps_only_status_prefix() {
    testlib::run("org.tiqian.font.CjkDashCapabilityPolicyTest.nonConformingStatusWithBlankDetailKeepsOnlyStatusPrefix", "org.tiqian.font.CjkDashCapabilityPolicyTest.nonConformingStatusWithBlankDetailKeepsOnlyStatusPrefix", || {
        TestTraceRecorder::new("CjkDashCapabilityPolicyTest").section(&"nonConformingStatusWithBlankDetailKeepsOnlyStatusPrefix");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"NoConformingCjkDashGlyph", CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_name_for(Some("unavailable".to_string())).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"status=unavailable", CjkDashCapabilityPolicy::cjk_dash_capability_policy_issue_detail_for(Some("unavailable".to_string()), None.clone()).as_str(), None).unwrap();
    });
}
