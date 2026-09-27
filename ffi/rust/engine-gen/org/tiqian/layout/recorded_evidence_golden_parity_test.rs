#![cfg(test)]

use crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormat;
use crate::org::tiqian::layout::recorded_layout_dump_goldens::RecordedLayoutDumpGoldens;
use crate::org::tiqian::layout::recorded_shaping_evidence_data::RecordedShapingEvidenceData;
use crate::org::tiqian::test::shaping_evidence::RecordedEvidenceFontMetricsResolver;
use crate::org::tiqian::test::shaping_evidence::RecordedEvidenceTextShaper;
use crate::org::tiqian::test::shaping_evidence_json::ShapingEvidenceJson;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum RecordedEvidenceGoldenParityTestRecordedEvidenceLayoutMatchesGoldenFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutDumpFormatLayoutFixtureDumpFaultFault(crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormatLayoutFixtureDumpFault),
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
        let mut test_trace = TestTraceRecorder::new("RecordedEvidenceGoldenParityTest");
        test_trace.section(&"recordedEvidenceLayoutMatchesGolden");
        if u_string::unit_count(&(RecordedShapingEvidenceData::recorded_shaping_evidence_data_evidence_json())) == 0 {
            let _ = TracedAssertions::traced_assertions_fail(Some((format!("{}{}",
            "No recorded shaping evidence embedded — record on the JVM with ",
            "TIQIAN_RECORD_SHAPING=1 ./gradlew :engine:jvmTest --tests '*ShapingEvidenceRecorder*'"
        )).to_string()), None).unwrap();
        }
        let evidence = ShapingEvidenceJson::shaping_evidence_json_parse(RecordedShapingEvidenceData::recorded_shaping_evidence_data_evidence_json().as_str()).unwrap();
        let shaper = RecordedEvidenceTextShaper::new((evidence).clone());
        let metrics = RecordedEvidenceFontMetricsResolver::new((evidence).clone());
        let mut failures: Vec<String> = vec![];
        {
            let _g1 = (*crate::org::tiqian::test::layout_fixtures::EARLY_LAYOUT_FIXTURES_ALL).clone();
            for fixture in &_g1 {
                let golden = RecordedLayoutDumpGoldens::recorded_layout_dump_goldens_by_id().get(&(fixture.id).to_string());
                if golden.is_none() {
                    failures.push(format!("{}{}{}",
            "missing recorded golden for fixture '",
            (fixture.id).to_string(),
            "' — re-record"
        ));
                    continue;
                }
                let dump = LayoutDumpFormat::layout_dump_format_layout_fixture_dump((fixture).clone(), Some(Box::new((shaper).clone())), Some(Box::new((metrics).clone()))).unwrap();
                if !(golden.as_ref().map_or(false, |v| v == &(dump.clone()))) {
                    failures.push(LayoutDumpFormat::layout_dump_format_layout_dump_diff_message((fixture.id).to_string().as_str(), (golden).as_deref().unwrap_or(""), dump.as_str()));
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((failures.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some((format!("{}{}{}",
            { let joined1 = failures; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(&(concat!("\n",
"\n",
""))); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } out },
            concat!("\n",
"\n",
"If the change is intentional, re-record with "),
            "TIQIAN_RECORD_SHAPING=1 on the JVM and review the golden diff."
        )).to_string())).unwrap();
    });
}
