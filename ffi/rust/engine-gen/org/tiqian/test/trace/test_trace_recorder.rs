use crate::org::tiqian::test::trace::test_trace_platform::TestTracePlatform;
use crate::org::tiqian::test::trace::test_trace_store::TestTraceStore;
use crate::std::u_string_exception::UStringFault;


#[derive(Clone, PartialEq)]
pub struct TestTraceRecorder {
    pub(crate) class_name: String,
    pub(crate) section_name: Option<String>,
}

impl TestTraceRecorder {
    pub fn new(class_name: &str) -> Self {
        let mut __self = Self {
            class_name: class_name.to_string(),
            section_name: Default::default(),
        };
        *crate::org::tiqian::test::trace::test_trace::TEST_TRACE_RECORDER.lock().unwrap_or_else(|e| e.into_inner()) = Some(__self.clone());
            __self.section_name = None;
        return __self;
    }

    pub fn section(&mut self, name: &str) {
        self.section_name = Some(name.to_string());
        TestTraceStore::test_trace_store_open((self.class_name).to_string().as_str(), name);
    }

    pub fn record(&self, line: &str) -> Result<(), UStringFault> {
        let current_section = (self.section_name).clone();
        if current_section.is_none() {
            return Ok(());
        }
        let _ = TestTraceStore::test_trace_store_append((self.class_name).to_string().as_str(), (current_section).as_deref().unwrap_or(""), line)?;
        Ok(())
    }

    pub fn flush(&self) -> Result<(), UStringFault> {
        if !TestTracePlatform::TEST_TRACE_PLATFORM_UPDATE_MODE || (self.section_name).clone().is_none() {
            return Ok(());
        }
        TestTracePlatform::test_trace_platform_write_golden((self.class_name).to_string().as_str(), TestTraceStore::test_trace_store_class_text((self.class_name).to_string().as_str())?.as_str());
        Ok(())
    }

    pub fn test_trace_recorder_flush_class(class_name: &str) -> Result<(), UStringFault> {
        if !TestTracePlatform::TEST_TRACE_PLATFORM_UPDATE_MODE {
            return Ok(());
        }
        TestTracePlatform::test_trace_platform_write_golden(class_name, TestTraceStore::test_trace_store_class_text(class_name)?.as_str());
        Ok(())
    }
}
