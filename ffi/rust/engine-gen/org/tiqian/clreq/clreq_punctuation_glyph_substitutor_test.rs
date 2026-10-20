#![cfg(test)]

use crate::org::tiqian::clreq::cjk_punctuation_glyph_policy::CjkPunctuationGlyphPolicy;
use crate::org::tiqian::clreq::clreq_punctuation_advance_policy::ClreqPunctuationAdvancePolicy;
use crate::org::tiqian::clreq::clreq_punctuation_glyph_substitutor::ClreqPunctuationGlyphSubstitutor;
use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[test]
fn prefer_policy_uses_clreq_recommended_display_codepoints() {
    testlib::run("org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preferPolicyUsesClreqRecommendedDisplayCodepoints", "org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preferPolicyUsesClreqRecommendedDisplayCodepoints", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,117,110,99,116,117,97,116,105,111,110,71,108,121,112,104,83,117,98,115,116,105,116,117,116,111,114,84,101,115,116]))).section(UStr::new(&[112,114,101,102,101,114,80,111,108,105,99,121,85,115,101,115,67,108,114,101,113,82,101,99,111,109,109,101,110,100,101,100,68,105,115,112,108,97,121,67,111,100,101,112,111,105,110,116,115]));
        let substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8943,8943]), (substitutor.substitute(UStr::new(&[8230,8230])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[11834]), (substitutor.substitute(UStr::new(&[8212,8212])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[183]), (substitutor.substitute(UStr::new(&[12539])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[183]), (substitutor.substitute(UStr::new(&[8231])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[183]), (substitutor.substitute(UStr::new(&[8226])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn preserve_policy_keeps_input_display_codepoints() {
    testlib::run("org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preservePolicyKeepsInputDisplayCodepoints", "org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preservePolicyKeepsInputDisplayCodepoints", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,117,110,99,116,117,97,116,105,111,110,71,108,121,112,104,83,117,98,115,116,105,116,117,116,111,114,84,101,115,116]))).section(UStr::new(&[112,114,101,115,101,114,118,101,80,111,108,105,99,121,75,101,101,112,115,73,110,112,117,116,68,105,115,112,108,97,121,67,111,100,101,112,111,105,110,116,115]));
        let substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreserveInput));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8230,8230]), (substitutor.substitute(UStr::new(&[8230,8230])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), (substitutor.substitute(UStr::new(&[8212,8212])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[12539]), (substitutor.substitute(UStr::new(&[12539])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn prefer_policy_does_not_rewrite_ambiguous_connector_or_solidus_forms() {
    testlib::run("org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preferPolicyDoesNotRewriteAmbiguousConnectorOrSolidusForms", "org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preferPolicyDoesNotRewriteAmbiguousConnectorOrSolidusForms", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,117,110,99,116,117,97,116,105,111,110,71,108,121,112,104,83,117,98,115,116,105,116,117,116,111,114,84,101,115,116]))).section(UStr::new(&[112,114,101,102,101,114,80,111,108,105,99,121,68,111,101,115,78,111,116,82,101,119,114,105,116,101,65,109,98,105,103,117,111,117,115,67,111,110,110,101,99,116,111,114,79,114,83,111,108,105,100,117,115,70,111,114,109,115]));
        let substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65374]), (substitutor.substitute(UStr::new(&[65374])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[45]), (substitutor.substitute(UStr::new(&[45])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[47]), (substitutor.substitute(UStr::new(&[47])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65295]), (substitutor.substitute(UStr::new(&[65295])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65294]), (substitutor.substitute(UStr::new(&[65294])).unwrap().display_text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn recommended_dash_codepoint_occupies_two_em() {
    testlib::run("org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.recommendedDashCodepointOccupiesTwoEm", "org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.recommendedDashCodepointOccupiesTwoEm", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,117,110,99,116,117,97,116,105,111,110,71,108,121,112,104,83,117,98,115,116,105,116,117,116,111,114,84,101,115,116]))).section(UStr::new(&[114,101,99,111,109,109,101,110,100,101,100,68,97,115,104,67,111,100,101,112,111,105,110,116,79,99,99,117,112,105,101,115,84,119,111,69,109]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(UStr::new(&[11834])).default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(UStr::new(&[11834])).default_advance_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(UStr::new(&[11834]), UStr::new(&[11834])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(UStr::new(&[8212,8212]), UStr::new(&[11834])), None).unwrap();
    });
}
