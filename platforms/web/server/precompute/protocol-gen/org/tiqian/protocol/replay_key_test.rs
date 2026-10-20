#![cfg(test)]

use crate::org::tiqian::protocol::replay_key::ReplayKey;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn shape_replay_keys_render_byte_identically() {
    testlib::run("org.tiqian.protocol.ReplayKeyTest.shapeReplayKeysRenderByteIdentically", "org.tiqian.protocol.ReplayKeyTest.shapeReplayKeysRenderByteIdentically", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[82,101,112,108,97,121,75,101,121,84,101,115,116])));
        recorder.section(UStr::new(&[115,104,97,112,101,82,101,112,108,97,121,75,101,121,115,82,101,110,100,101,114,66,121,116,101,73,100,101,110,116,105,99,97,108,108,121]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[91,34,20320,22909,34,44,34,65,92,117,48,48,49,102,66,34,44,52,48,48,44,102,97,108,115,101,44,34,122,104,45,99,110,34,44,34,67,106,107,84,101,120,116,34,44,34,20320,22909,34,93]), ReplayKey::replay_key_render_shape_replay_key(ReplayKey::replay_key_shape_replay_key(UStr::new(&[20320,22909]), &vec![UString::from("A").to_ustring(), UString::from("B").to_ustring()], 400.0f64, false, UStr::new(&[122,104,45,99,110]), Some(UString::from("CjkText")), UStr::new(&[20320,22909]))).unwrap().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[91,34,120,34,44,34,65,34,44,55,48,48,46,53,44,116,114,117,101,44,34,101,110,34,44,34,110,117,108,108,34,44,34,121,34,93]), ReplayKey::replay_key_render_shape_replay_key(ReplayKey::replay_key_shape_replay_key(UStr::new(&[120]), &vec![UString::from("A").to_ustring()], 700.5f64, true, UStr::new(&[101,110]), None.clone(), UStr::new(&[121]))).unwrap().as_ustr(), None).unwrap();
    });
}

#[test]
fn metric_replay_keys_render_byte_identically() {
    testlib::run("org.tiqian.protocol.ReplayKeyTest.metricReplayKeysRenderByteIdentically", "org.tiqian.protocol.ReplayKeyTest.metricReplayKeysRenderByteIdentically", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[82,101,112,108,97,121,75,101,121,84,101,115,116])));
        recorder.section(UStr::new(&[109,101,116,114,105,99,82,101,112,108,97,121,75,101,121,115,82,101,110,100,101,114,66,121,116,101,73,100,101,110,116,105,99,97,108,108,121]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[91,34,65,92,117,48,48,49,102,66,34,44,52,48,48,44,102,97,108,115,101,44,34,110,117,108,108,34,44,34,66,34,93]), ReplayKey::replay_key_render_metric_replay_key(ReplayKey::replay_key_metric_replay_key(&vec![UString::from("A").to_ustring(), UString::from("B").to_ustring()], 400.0f64, false, None.clone(), Some(UString::from("B")))).unwrap().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[91,34,65,34,44,52,48,48,44,102,97,108,115,101,44,34,110,117,108,108,34,44,34,110,117,108,108,34,93]), ReplayKey::replay_key_render_metric_replay_key(ReplayKey::replay_key_metric_replay_key(&vec![UString::from("A").to_ustring()], 400.0f64, false, None.clone(), None.clone())).unwrap().as_ustr(), None).unwrap();
    });
}

#[test]
fn normalized_replay_numbers_canonicalize_to_twelve_decimals() {
    testlib::run("org.tiqian.protocol.ReplayKeyTest.normalizedReplayNumbersCanonicalizeToTwelveDecimals", "org.tiqian.protocol.ReplayKeyTest.normalizedReplayNumbersCanonicalizeToTwelveDecimals", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[82,101,112,108,97,121,75,101,121,84,101,115,116])));
        recorder.section(UStr::new(&[110,111,114,109,97,108,105,122,101,100,82,101,112,108,97,121,78,117,109,98,101,114,115,67,97,110,111,110,105,99,97,108,105,122,101,84,111,84,119,101,108,118,101,68,101,99,105,109,97,108,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_float(Some(1.0f64), ReplayKey::replay_key_normalized_replay_number(16.0f64, 16.0f64).unwrap(), Some(UString::from("16/16 -> 1.0"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_float(Some(1.0f64), ReplayKey::replay_key_normalized_replay_number(15.5f64, 15.5f64).unwrap(), Some(UString::from("15.5/15.5 -> 1.0"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_float(Some(1.0f64), ReplayKey::replay_key_normalized_replay_number(3.3000000000000003f64, 3.3f64).unwrap(), Some(UString::from("3.3000000000000003/3.3 -> 1.0"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_float(Some(0.0f64), ReplayKey::replay_key_normalized_replay_number(-0.0f64, 16.0f64).unwrap(), Some(UString::from("-0.0/16 -> 0.0"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_float(None, ReplayKey::replay_key_normalized_replay_number(f64::NAN, 16.0f64).unwrap(), Some(UString::from("NaN -> null"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_float(None, ReplayKey::replay_key_normalized_replay_number(f64::INFINITY, 16.0f64).unwrap(), Some(UString::from("INF -> null"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_float(Some(0.0f64), ReplayKey::replay_key_normalized_replay_number(1.6e-12f64, 16.0f64).unwrap(), Some(UString::from("1e-13*16/16 -> 0.0"))).unwrap();
    });
}
