#![cfg(test)]

use crate::org::tiqian::protocol::canonical::Canonical;
use crate::org::tiqian::protocol::canonical_test_support::CanonicalTestSupport;
use crate::org::tiqian::protocol::encode_result::EncodeResult;
use crate::org::tiqian::protocol::wire_field::WireField;
use crate::org::tiqian::protocol::wire_value::WireValue;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[test]
fn golden_vectors_match_the_shared_bytes() {
    testlib::run("org.tiqian.protocol.CanonicalTest.goldenVectorsMatchTheSharedBytes", "org.tiqian.protocol.CanonicalTest.goldenVectorsMatchTheSharedBytes", || {
        let mut recorder = TestTraceRecorder::new("CanonicalTest");
        recorder.section(&"goldenVectorsMatchTheSharedBytes");
        let _ = CanonicalTestSupport::canonical_test_support_assert_hex((recorder).clone(), &"vector0", &"54514353010006000000e4b8ade6968701000000000000624000000000000000000000000000000000", Canonical::canonical_encode(CanonicalTestSupport::canonical_test_support_wire_obj(&"p-1",
&"中文", WireValue::WNum { value: 144 }), 0).unwrap()).unwrap();
        let _ = CanonicalTestSupport::canonical_test_support_assert_hex((recorder).clone(), &"vector1", &"54514353010106000000e4b8ade6968700000000000000000000000000000000", Canonical::canonical_encode(CanonicalTestSupport::canonical_test_support_wire_obj(&"fc-1", &"中文", None),
1).unwrap()).unwrap();
        let _ = CanonicalTestSupport::canonical_test_support_assert_hex((recorder).clone(), &"vector2",
&"5451435301000f000000e4b8ade69687e5ad97e68e92e7898801000000000000624002000000070100000000000000400100000000000010400100000061000000000000f03f010200000004000000687265661300000068747470733a2f2f6578616d706c652e636f6d05000000636c617373040000006c696e6b0401000000000000000001000000000000f03f02000000656d020000001f010000000000000000010000000000000040010000000f00000044656c6120476f74686963204f6e650000000000003240000000000000794001000000000000e0bf0101000000000000004001000000000000104000000000020000000701000000000000f03f0100000000000000400000000000002040000000000000104006000000536f757263650001000000000000084001000000000000104003000000000000000000000000000000000032400000000000004240", Canonical::canonical_encode(CanonicalTestSupport::canonical_test_support_full_vector_input(), 0).unwrap()).unwrap();
    });
}

#[test]
fn loose_coercions_carry_the_json_lane() {
    testlib::run("org.tiqian.protocol.CanonicalTest.looseCoercionsCarryTheJsonLane", "org.tiqian.protocol.CanonicalTest.looseCoercionsCarryTheJsonLane", || {
        let mut recorder = TestTraceRecorder::new("CanonicalTest");
        recorder.section(&"looseCoercionsCarryTheJsonLane");
        let _ = CanonicalTestSupport::canonical_test_support_assert_hex((recorder).clone(), &"vector3",
&"5451435301000800000020636f6572636520010000000000106240010000000501000000000000f03f010000000000000040010000006902130000005b5b2261222c2231225d2c5b2262222c325d5d010000000001000000000000000001000000000000f03f010000000401000000000000000001000000000000f03f01000000370200000000000000000008400000000000000000", Canonical::canonical_encode(CanonicalTestSupport::canonical_test_support_loose_vector_input(), 0).unwrap()).unwrap();
    });
}

#[test]
fn non_array_semantics_is_the_named_issue() {
    testlib::run("org.tiqian.protocol.CanonicalTest.nonArraySemanticsIsTheNamedIssue", "org.tiqian.protocol.CanonicalTest.nonArraySemanticsIsTheNamedIssue", || {
        let mut recorder = TestTraceRecorder::new("CanonicalTest");
        recorder.section(&"nonArraySemanticsIsTheNamedIssue");
        let result = Canonical::canonical_encode(WireValue::WObj { fields: vec![
    (WireField { name: "text".to_string(), value: WireValue::WStr { value: "a".to_string() } }).clone(),
    (WireField { name: "semantics".to_string(), value: WireValue::WStr { value: "no".to_string() } }).clone(),
].to_vec() }, 0).unwrap();
        let _ = match result {
    EncodeResult::COk { .. } => {
    recorder.record(&"encode unexpectedly succeeded").unwrap();
    TracedAssertions::traced_assertions_assert_true(false, Some("non-array semantics must be the named issue".to_string())).unwrap()
},
    EncodeResult::CErr { issue: _p0 } => TracedAssertions::traced_assertions_assert_equals_string(&"InvalidSnapshotSemantics", _p0.as_str(), None).unwrap(),
};
    });
}

#[test]
fn non_finite_and_minus_zero_collapse() {
    testlib::run("org.tiqian.protocol.CanonicalTest.nonFiniteAndMinusZeroCollapse", "org.tiqian.protocol.CanonicalTest.nonFiniteAndMinusZeroCollapse", || {
        let mut recorder = TestTraceRecorder::new("CanonicalTest");
        recorder.section(&"nonFiniteAndMinusZeroCollapse");
        let without_width = Canonical::canonical_encode(WireValue::WObj { fields: vec![
    (WireField { name: "text".to_string(), value: WireValue::WStr { value: "a".to_string() } }).clone(),
].to_vec() }, 0).unwrap();
        let with_na_n = Canonical::canonical_encode(WireValue::WObj { fields: vec![
    (WireField { name: "text".to_string(), value: WireValue::WStr { value: "a".to_string() } }).clone(),
    (WireField { name: "maxWidthPx".to_string(), value: WireValue::WNum { value: f64::NAN } }).clone(),
].to_vec() }, 0).unwrap();
        let _ = CanonicalTestSupport::canonical_test_support_assert_hex((recorder).clone(), &"withNaN", CanonicalTestSupport::canonical_test_support_hex_of((without_width).clone()).as_str(), (with_na_n).clone()).unwrap();
        let minus_zero = Canonical::canonical_encode(WireValue::WObj { fields: vec![
    (WireField { name: "text".to_string(), value: WireValue::WStr { value: "a".to_string() } }).clone(),
    (WireField { name: "maxWidthPx".to_string(), value: WireValue::WNum { value: -0.0f64 } }).clone(),
].to_vec() }, 0).unwrap();
        let _ = CanonicalTestSupport::canonical_test_support_assert_hex((recorder).clone(), &"minusZero", CanonicalTestSupport::canonical_test_support_hex_of((without_width).clone()).as_str(), (minus_zero).clone()).unwrap();
    });
}

#[test]
fn digest_matches_the_platform_hash() {
    testlib::run("org.tiqian.protocol.CanonicalTest.digestMatchesThePlatformHash", "org.tiqian.protocol.CanonicalTest.digestMatchesThePlatformHash", || {
        let mut recorder = TestTraceRecorder::new("CanonicalTest");
        recorder.section(&"digestMatchesThePlatformHash");
        let encoded = Canonical::canonical_encode(CanonicalTestSupport::canonical_test_support_wire_obj(&"p-1", &"中文", WireValue::WNum { value: 144 }), 0).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"0884bb15dee00d579efe3a63fc65344ed80b1f753fa04a2cdd2ccc830bd69c60", CanonicalTestSupport::canonical_test_support_hex_of_digest((encoded).clone()).as_str(), None).unwrap();
    });
}
