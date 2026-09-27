#![cfg(test)]

use crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormat;
use crate::org::tiqian::layout::recorded_layout_dump_goldens::RecordedLayoutDumpGoldens;
use crate::org::tiqian::layout::recorded_shaping_evidence_data::RecordedShapingEvidenceData;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::test::shaping_evidence::RecordedEvidenceFontMetricsResolver;
use crate::org::tiqian::test::shaping_evidence::RecordedEvidenceTextShaper;
use crate::org::tiqian::test::shaping_evidence_json::ShapingEvidenceJson;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutDumpFormatLayoutFixtureDumpFaultFault(crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormatLayoutFixtureDumpFault),
}
impl std::fmt::Display for RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::LayoutDumpFormatLayoutFixtureDumpFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault) -> Self {
        match value {
            RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault) -> Self {
        match value {
            RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault) -> Self {
        match value {
            RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault) -> Self {
        match value {
            RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault> for crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormatLayoutFixtureDumpFault {
    fn from(value: RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault) -> Self {
        match value {
            RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::LayoutDumpFormatLayoutFixtureDumpFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormatLayoutFixtureDumpFault> for RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault {
    fn from(value: crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormatLayoutFixtureDumpFault) -> Self {
        RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault::LayoutDumpFormatLayoutFixtureDumpFaultFault(value)
    }
}

#[test]
fn recorded_evidence_layout_matches_golden() {
    testlib::run("org.tiqian.layout.RecordedEvidenceGoldenParityTest.recordedEvidenceLayoutMatchesGolden", "org.tiqian.layout.RecordedEvidenceGoldenParityTest.recordedEvidenceLayoutMatchesGolden", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[82,101,99,111,114,100,101,100,69,118,105,100,101,110,99,101,71,111,108,100,101,110,80,97,114,105,116,121,84,101,115,116])));
        test_trace.section(UStr::new(&[114,101,99,111,114,100,101,100,69,118,105,100,101,110,99,101,76,97,121,111,117,116,77,97,116,99,104,101,115,71,111,108,100,101,110]));
        if u_string::unit_count(&(RecordedShapingEvidenceData::recorded_shaping_evidence_data_evidence_json())) == 0 {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("No recorded shaping evidence embedded — record on the JVM with ")); __s += &(UString::from("TIQIAN_RECORD_SHAPING=1 ./gradlew :engine:jvmTest --tests '*ShapingEvidenceRecorder*'")); __s }).as_str())), None).unwrap();
        }
        let evidence = ShapingEvidenceJson::shaping_evidence_json_parse(RecordedShapingEvidenceData::recorded_shaping_evidence_data_evidence_json().as_ustr()).unwrap();
        let shaper = Arc::new(Mutex::new(RecordedEvidenceTextShaper::new((evidence).clone())));
        let metrics = RecordedEvidenceFontMetricsResolver::new((evidence).clone());
        let mut failures: Vec<UString> = vec![];
        {
            let _g1 = (*crate::org::tiqian::test::layout_fixtures::EARLY_LAYOUT_FIXTURES_ALL).clone();
            for fixture in &_g1 {
                let golden = RecordedLayoutDumpGoldens::recorded_layout_dump_goldens_by_id().get(&(fixture.id).to_ustring());
                if golden.is_none() {
                    failures.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("missing recorded golden for fixture '")); __s += (fixture.id).to_ustring().as_ustr(); __s += &(UString::from("' — re-record")); __s }).as_str()));
                    continue;
                }
                let dump = LayoutDumpFormat::layout_dump_format_layout_fixture_dump((fixture).clone(), Some({ let __shared_handle: Arc<Mutex<dyn ITextShaper>> = shaper.clone(); __shared_handle }), Some(Box::new((metrics).clone()))).unwrap();
                if !(golden.as_ref().map_or(false, |v| v == &(dump.clone()))) {
                    failures.push(LayoutDumpFormat::layout_dump_format_layout_dump_diff_message((fixture.id).to_ustring().as_ustr(), (golden).as_deref().unwrap_or(UStr::new(&[])), dump.as_ustr()));
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((failures.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += UString::from(format!("{}", { let joined1 = failures; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(concat!("\n",
"\n",
"")); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from(concat!("\n",
"\n",
"If the change is intentional, re-record with "))); __s += &(UString::from("TIQIAN_RECORD_SHAPING=1 on the JVM and review the golden diff.")); __s }).as_str()))).unwrap();
    });
}
