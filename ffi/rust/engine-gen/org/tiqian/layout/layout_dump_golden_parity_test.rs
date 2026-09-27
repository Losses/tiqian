#![cfg(test)]

use crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormat;
use crate::org::tiqian::layout::layout_dump_goldens::LayoutDumpGoldens;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    LayoutDumpFormatLayoutFixtureDumpFaultFault(crate::org::tiqian::layout::layout_dump_format::LayoutDumpFormatLayoutFixtureDumpFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::LayoutDumpFormatLayoutFixtureDumpFaultFault(value) => write!(formatter, "{}", value),
            LayoutDumpGoldenParityTestLayoutDecisionDumpsMatchEmbeddedGoldenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,68,117,109,112,71,111,108,100,101,110,80,97,114,105,116,121,84,101,115,116])));
        t.section(UStr::new(&[108,97,121,111,117,116,68,101,99,105,115,105,111,110,68,117,109,112,115,77,97,116,99,104,69,109,98,101,100,100,101,100,71,111,108,100,101,110]));
        let mut failures: Vec<UString> = vec![];
        {
            let _g1 = (*crate::org::tiqian::test::layout_fixtures::EARLY_LAYOUT_FIXTURES_ALL).clone();
            for fixture in &_g1 {
                let golden = LayoutDumpGoldens::layout_dump_goldens_by_id().get(&(fixture.id).to_ustring());
                if golden.is_none() {
                    failures.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("missing embedded golden for fixture '")); __s += (fixture.id).to_ustring().as_ustr(); __s += &(UString::from("' — run with TIQIAN_UPDATE_GOLDEN=1 on the JVM, then rebuild")); __s }).as_str()));
                    continue;
                }
                let actual = LayoutDumpFormat::layout_dump_format_layout_fixture_dump((fixture).clone(), None, None).unwrap();
                if !(golden.as_ref().map_or(false, |v| v == &(actual.clone()))) {
                    failures.push(LayoutDumpFormat::layout_dump_format_layout_dump_diff_message((fixture.id).to_ustring().as_ustr(), (golden).as_deref().unwrap_or(UStr::new(&[])), actual.as_ustr()));
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((failures.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += UString::from(format!("{}", { let joined1 = failures; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(concat!("\n",
"\n",
"")); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from(concat!("\n",
"\n",
"If the change is intentional, regenerate with TIQIAN_UPDATE_GOLDEN=1 on the JVM and review the golden diff."))); __s }).as_str()))).unwrap();
    });
}
