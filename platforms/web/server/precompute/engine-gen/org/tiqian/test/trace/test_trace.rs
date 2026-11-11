use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use std::sync::Mutex;


pub static TEST_TRACE_RECORDER: Mutex<Option<TestTraceRecorder>> = Mutex::new(None);

#[derive(Clone, Copy)]
pub struct TestTrace;

impl TestTrace {
    pub const TEST_TRACE_UPDATE_MODE: bool = true;

    pub fn test_trace_current_recorder() -> Option<TestTraceRecorder> {
        return ({ let __guard = TEST_TRACE_RECORDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).clone();
    }
}
