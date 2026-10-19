#![cfg(test)]

use crate::org::tiqian::protocol::face_session::FaceSession;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[test]
fn ordered_face_specs_sorts_by_order_then_index() {
    testlib::run("org.tiqian.protocol.FaceSessionTest.orderedFaceSpecsSortsByOrderThenIndex", "org.tiqian.protocol.FaceSessionTest.orderedFaceSpecsSortsByOrderThenIndex", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[70,97,99,101,83,101,115,115,105,111,110,84,101,115,116])));
        recorder.section(UStr::new(&[111,114,100,101,114,101,100,70,97,99,101,83,112,101,99,115,83,111,114,116,115,66,121,79,114,100,101,114,84,104,101,110,73,110,100,101,120]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2, 0, 1], &FaceSession::face_session_ordered_face_specs(&vec![Some(3.0f64), Some(5.0f64), Some(1.0f64)]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![1, 2, 0], &FaceSession::face_session_ordered_face_specs(&vec![Some(5.0f64), None, Some(2.0f64)]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2, 1, 0], &FaceSession::face_session_ordered_face_specs(&vec![Some(9.0f64), Some(8.0f64), Some(7.0f64)]), None).unwrap();
    });
}

#[test]
fn ordered_face_specs_defaults_null_to_input_position() {
    testlib::run("org.tiqian.protocol.FaceSessionTest.orderedFaceSpecsDefaultsNullToInputPosition", "org.tiqian.protocol.FaceSessionTest.orderedFaceSpecsDefaultsNullToInputPosition", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[70,97,99,101,83,101,115,115,105,111,110,84,101,115,116])));
        recorder.section(UStr::new(&[111,114,100,101,114,101,100,70,97,99,101,83,112,101,99,115,68,101,102,97,117,108,116,115,78,117,108,108,84,111,73,110,112,117,116,80,111,115,105,116,105,111,110]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 1, 2], &FaceSession::face_session_ordered_face_specs(&vec![None, None, None]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 1, 2], &FaceSession::face_session_ordered_face_specs(&vec![None, Some(0.0f64), Some(1.0f64)]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2, 1, 0], &FaceSession::face_session_ordered_face_specs(&vec![Some(100.0f64), Some(3.0f64), None]), None).unwrap();
    });
}

#[test]
fn ordered_face_specs_tie_break_is_stable_on_index() {
    testlib::run("org.tiqian.protocol.FaceSessionTest.orderedFaceSpecsTieBreakIsStableOnIndex", "org.tiqian.protocol.FaceSessionTest.orderedFaceSpecsTieBreakIsStableOnIndex", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[70,97,99,101,83,101,115,115,105,111,110,84,101,115,116])));
        recorder.section(UStr::new(&[111,114,100,101,114,101,100,70,97,99,101,83,112,101,99,115,84,105,101,66,114,101,97,107,73,115,83,116,97,98,108,101,79,110,73,110,100,101,120]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 1], &FaceSession::face_session_ordered_face_specs(&vec![Some(2.0f64), Some(2.0f64)]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 1, 2], &FaceSession::face_session_ordered_face_specs(&vec![None, Some(0.0f64), Some(0.0f64)]), None).unwrap();
    });
}
