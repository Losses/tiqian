#![cfg(test)]

use crate::org::tiqian::protocol::context_fingerprint::ContextFingerprint;
use crate::org::tiqian::protocol::context_fingerprint_test_support::ContextFingerprintTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[test]
fn empty_face_set_golden() {
    testlib::run("org.tiqian.protocol.ContextFingerprintTest.emptyFaceSetGolden", "org.tiqian.protocol.ContextFingerprintTest.emptyFaceSetGolden", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[67,111,110,116,101,120,116,70,105,110,103,101,114,112,114,105,110,116,84,101,115,116])));
        recorder.section(UStr::new(&[101,109,112,116,121,70,97,99,101,83,101,116,71,111,108,100,101,110]));
        let digest = ContextFingerprint::context_fingerprint_context_fingerprint(ContextFingerprintTestSupport::context_fingerprint_test_support_typography(), &vec![], ContextFingerprintTestSupport::context_fingerprint_test_support_harfbuzz_version().as_ustr());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,56,55,54,55,100,53,57,100,102,53,51,99,56,57,50,57,99,55,99,97,97,99,56,97,102,100,55,97,98,56,52,102,54,101,55,97,97,50,100,99,48,98,102,100,51,55,56,48,57,98,54,102,98,54,55,98,50,48,101,54,99,97,54]), ContextFingerprintTestSupport::context_fingerprint_test_support_hex_bytes(&digest).as_ustr(), None).unwrap();
    });
}

#[test]
fn one_face_golden() {
    testlib::run("org.tiqian.protocol.ContextFingerprintTest.oneFaceGolden", "org.tiqian.protocol.ContextFingerprintTest.oneFaceGolden", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[67,111,110,116,101,120,116,70,105,110,103,101,114,112,114,105,110,116,84,101,115,116])));
        recorder.section(UStr::new(&[111,110,101,70,97,99,101,71,111,108,100,101,110]));
        let digest = ContextFingerprint::context_fingerprint_context_fingerprint(ContextFingerprintTestSupport::context_fingerprint_test_support_typography(), &vec![(ContextFingerprintTestSupport::context_fingerprint_test_support_face()).clone()], ContextFingerprintTestSupport::context_fingerprint_test_support_harfbuzz_version().as_ustr());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[51,49,101,102,55,57,49,57,56,55,55,55,101,102,101,50,48,99,51,98,49,48,53,100,53,55,48,98,57,56,49,98,102,54,99,56,51,54,102,48,101,53,99,52,48,48,56,52,50,101,50,53,100,100,102,55,99,52,99,99,50,50,57,52]), ContextFingerprintTestSupport::context_fingerprint_test_support_hex_bytes(&digest).as_ustr(), None).unwrap();
    });
}

#[test]
fn font_size_change_alters_the_fingerprint() {
    testlib::run("org.tiqian.protocol.ContextFingerprintTest.fontSizeChangeAltersTheFingerprint", "org.tiqian.protocol.ContextFingerprintTest.fontSizeChangeAltersTheFingerprint", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[67,111,110,116,101,120,116,70,105,110,103,101,114,112,114,105,110,116,84,101,115,116])));
        recorder.section(UStr::new(&[102,111,110,116,83,105,122,101,67,104,97,110,103,101,65,108,116,101,114,115,84,104,101,70,105,110,103,101,114,112,114,105,110,116]));
        let digest = ContextFingerprint::context_fingerprint_context_fingerprint(ContextFingerprintTestSupport::context_fingerprint_test_support_typography_at(19.0f64), &vec![], ContextFingerprintTestSupport::context_fingerprint_test_support_harfbuzz_version().as_ustr());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[100,97,97,100,48,50,50,54,53,55,52,55,98,100,54,48,53,50,52,49,56,98,56,57,56,100,48,49,57,97,53,99,51,56,56,51,52,102,51,55,98,57,52,54,48,57,49,51,97,101,49,99,102,101,57,52,101,100,97,100,101,99,49,52]), ContextFingerprintTestSupport::context_fingerprint_test_support_hex_bytes(&digest).as_ustr(), None).unwrap();
    });
}

#[test]
fn equal_configuration_is_stable() {
    testlib::run("org.tiqian.protocol.ContextFingerprintTest.equalConfigurationIsStable", "org.tiqian.protocol.ContextFingerprintTest.equalConfigurationIsStable", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[67,111,110,116,101,120,116,70,105,110,103,101,114,112,114,105,110,116,84,101,115,116])));
        recorder.section(UStr::new(&[101,113,117,97,108,67,111,110,102,105,103,117,114,97,116,105,111,110,73,115,83,116,97,98,108,101]));
        let a = ContextFingerprint::context_fingerprint_context_fingerprint(ContextFingerprintTestSupport::context_fingerprint_test_support_typography(), &vec![], ContextFingerprintTestSupport::context_fingerprint_test_support_harfbuzz_version().as_ustr());
        let b = ContextFingerprint::context_fingerprint_context_fingerprint(ContextFingerprintTestSupport::context_fingerprint_test_support_typography(), &vec![], ContextFingerprintTestSupport::context_fingerprint_test_support_harfbuzz_version().as_ustr());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(ContextFingerprintTestSupport::context_fingerprint_test_support_hex_bytes(&a).as_ustr(), ContextFingerprintTestSupport::context_fingerprint_test_support_hex_bytes(&b).as_ustr(), None).unwrap();
    });
}
