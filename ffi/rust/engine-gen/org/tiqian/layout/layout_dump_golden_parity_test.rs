#![cfg(test)]

use crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormat;
use crate::org::tiqian::layout::layout_dump_goldens::LayoutDumpGoldens;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    LayoutDumpFormatLayoutFixtureDumpFaultFault(crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormatLayoutFixtureDumpFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault) -> Self {
        match value {
            LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault) -> Self {
        match value {
            LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault> for crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormatLayoutFixtureDumpFault {
    fn from(value: LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault) -> Self {
        match value {
            LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::LayoutDumpFormatLayoutFixtureDumpFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault) -> Self {
        match value {
            LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormatLayoutFixtureDumpFault> for LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault {
    fn from(value: crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormatLayoutFixtureDumpFault) -> Self {
        LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::LayoutDumpFormatLayoutFixtureDumpFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn layout_decision_dumps_match_embedded_golden() {
    testlib::run("org.tiqian.layout.LayoutDumpGoldenParityTest.layoutDecisionDumpsMatchEmbeddedGolden", "org.tiqian.layout.LayoutDumpGoldenParityTest.layoutDecisionDumpsMatchEmbeddedGolden", || {
        let mut t = TestTraceRecorder::new("LayoutDumpGoldenParityTest");
        t.section(&"layoutDecisionDumpsMatchEmbeddedGolden");
        let mut failures: Vec<String> = vec![];
        {
            let _g1 = (*crate::org::tiqian::test::layout_fixtures::EARLY_LAYOUT_FIXTURES_ALL).clone();
            for fixture in &_g1 {
                let golden = LayoutDumpGoldens::layout_dump_goldens_by_id().get(&(fixture.id).to_string());
                if golden.is_none() {
                    failures.push(format!("{}{}{}",
            "missing embedded golden for fixture '",
            (fixture.id).to_string(),
            "' — run with TIQIAN_UPDATE_GOLDEN=1 on the JVM, then rebuild"
        ));
                    continue;
                }
                let actual = LayoutDumpFormat::layout_dump_format_layout_fixture_dump((fixture).clone(), None, None).unwrap();
                if !(golden.as_ref().map_or(false, |v| v == &(actual.clone()))) {
                    failures.push(LayoutDumpFormat::layout_dump_format_layout_dump_diff_message((fixture.id).to_string().as_str(), (golden).as_deref().unwrap_or(""), actual.as_str()));
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((failures.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some((format!("{}{}",
            { let joined1 = failures; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(&(concat!("\n",
"\n",
""))); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } out },
            concat!("\n",
"\n",
"If the change is intentional, regenerate with TIQIAN_UPDATE_GOLDEN=1 on the JVM and review the golden diff.")
        )).to_string())).unwrap();
    });
}
