#![cfg(test)]

use crate::org::tiqian::clreq::cjk_punctuation_glyph_policy::CjkPunctuationGlyphPolicy;
use crate::org::tiqian::clreq::clreq_punctuation_advance_policy::ClreqPunctuationAdvancePolicy;
use crate::org::tiqian::clreq::clreq_punctuation_glyph_substitutor::ClreqPunctuationGlyphSubstitutor;
use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[test]
fn prefer_policy_uses_clreq_recommended_display_codepoints() {
    testlib::run("org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preferPolicyUsesClreqRecommendedDisplayCodepoints", "org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preferPolicyUsesClreqRecommendedDisplayCodepoints", || {
        TestTraceRecorder::new("ClreqPunctuationGlyphSubstitutorTest").section(&"preferPolicyUsesClreqRecommendedDisplayCodepoints");
        let substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"⋯⋯", (substitutor.substitute(&"……").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"⸺", (substitutor.substitute(&"——").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"·", (substitutor.substitute(&"・").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"·", (substitutor.substitute(&"‧").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"·", (substitutor.substitute(&"•").unwrap().display_text).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn preserve_policy_keeps_input_display_codepoints() {
    testlib::run("org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preservePolicyKeepsInputDisplayCodepoints", "org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preservePolicyKeepsInputDisplayCodepoints", || {
        TestTraceRecorder::new("ClreqPunctuationGlyphSubstitutorTest").section(&"preservePolicyKeepsInputDisplayCodepoints");
        let substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreserveInput));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"……", (substitutor.substitute(&"……").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", (substitutor.substitute(&"——").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"・", (substitutor.substitute(&"・").unwrap().display_text).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn prefer_policy_does_not_rewrite_ambiguous_connector_or_solidus_forms() {
    testlib::run("org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preferPolicyDoesNotRewriteAmbiguousConnectorOrSolidusForms", "org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.preferPolicyDoesNotRewriteAmbiguousConnectorOrSolidusForms", || {
        TestTraceRecorder::new("ClreqPunctuationGlyphSubstitutorTest").section(&"preferPolicyDoesNotRewriteAmbiguousConnectorOrSolidusForms");
        let substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"～", (substitutor.substitute(&"～").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"-", (substitutor.substitute(&"-").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"/", (substitutor.substitute(&"/").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"／", (substitutor.substitute(&"／").unwrap().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"．", (substitutor.substitute(&"．").unwrap().display_text).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn recommended_dash_codepoint_occupies_two_em() {
    testlib::run("org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.recommendedDashCodepointOccupiesTwoEm", "org.tiqian.clreq.ClreqPunctuationGlyphSubstitutorTest.recommendedDashCodepointOccupiesTwoEm", || {
        TestTraceRecorder::new("ClreqPunctuationGlyphSubstitutorTest").section(&"recommendedDashCodepointOccupiesTwoEm");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(&"⸺").default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(&"⸺").default_advance_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(&"⸺", &"⸺"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(&"——", &"⸺"), None).unwrap();
    });
}
