#![cfg(test)]

use crate::org::tiqian::protocol::font_contract_test_support::FontContractTestSupport;
use crate::org::tiqian::protocol::wire_field::WireField;
use crate::org::tiqian::protocol::wire_value::WireValue;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum FontContractTestUnsafeSpanRangesNeverOwnADashFault {
    TracedAssertionsAssertEqualsNullableStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for FontContractTestUnsafeSpanRangesNeverOwnADashFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontContractTestUnsafeSpanRangesNeverOwnADashFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value) => write!(formatter, "{}", value),
            FontContractTestUnsafeSpanRangesNeverOwnADashFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<FontContractTestUnsafeSpanRangesNeverOwnADashFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault {
    fn from(value: FontContractTestUnsafeSpanRangesNeverOwnADashFault) -> Self {
        match value {
            FontContractTestUnsafeSpanRangesNeverOwnADashFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontContractTestUnsafeSpanRangesNeverOwnADashFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontContractTestUnsafeSpanRangesNeverOwnADashFault) -> Self {
        match value {
            FontContractTestUnsafeSpanRangesNeverOwnADashFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault> for FontContractTestUnsafeSpanRangesNeverOwnADashFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault) -> Self {
        FontContractTestUnsafeSpanRangesNeverOwnADashFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontContractTestUnsafeSpanRangesNeverOwnADashFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontContractTestUnsafeSpanRangesNeverOwnADashFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontContractTestTwoStyledDashRunsGroupByStyleFault {
    TracedAssertionsAssertEqualsNullableStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for FontContractTestTwoStyledDashRunsGroupByStyleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontContractTestTwoStyledDashRunsGroupByStyleFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value) => write!(formatter, "{}", value),
            FontContractTestTwoStyledDashRunsGroupByStyleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<FontContractTestTwoStyledDashRunsGroupByStyleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault {
    fn from(value: FontContractTestTwoStyledDashRunsGroupByStyleFault) -> Self {
        match value {
            FontContractTestTwoStyledDashRunsGroupByStyleFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontContractTestTwoStyledDashRunsGroupByStyleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontContractTestTwoStyledDashRunsGroupByStyleFault) -> Self {
        match value {
            FontContractTestTwoStyledDashRunsGroupByStyleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault> for FontContractTestTwoStyledDashRunsGroupByStyleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault) -> Self {
        FontContractTestTwoStyledDashRunsGroupByStyleFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontContractTestTwoStyledDashRunsGroupByStyleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontContractTestTwoStyledDashRunsGroupByStyleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontContractTestTextWithoutADashReturnsNoneFault {
    TracedAssertionsAssertEqualsNullableStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for FontContractTestTextWithoutADashReturnsNoneFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontContractTestTextWithoutADashReturnsNoneFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value) => write!(formatter, "{}", value),
            FontContractTestTextWithoutADashReturnsNoneFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<FontContractTestTextWithoutADashReturnsNoneFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault {
    fn from(value: FontContractTestTextWithoutADashReturnsNoneFault) -> Self {
        match value {
            FontContractTestTextWithoutADashReturnsNoneFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontContractTestTextWithoutADashReturnsNoneFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontContractTestTextWithoutADashReturnsNoneFault) -> Self {
        match value {
            FontContractTestTextWithoutADashReturnsNoneFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault> for FontContractTestTextWithoutADashReturnsNoneFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault) -> Self {
        FontContractTestTextWithoutADashReturnsNoneFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontContractTestTextWithoutADashReturnsNoneFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontContractTestTextWithoutADashReturnsNoneFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontContractTestOneDashGroupProducesOneProbeSpanFault {
    TracedAssertionsAssertEqualsNullableStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for FontContractTestOneDashGroupProducesOneProbeSpanFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontContractTestOneDashGroupProducesOneProbeSpanFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value) => write!(formatter, "{}", value),
            FontContractTestOneDashGroupProducesOneProbeSpanFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<FontContractTestOneDashGroupProducesOneProbeSpanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault {
    fn from(value: FontContractTestOneDashGroupProducesOneProbeSpanFault) -> Self {
        match value {
            FontContractTestOneDashGroupProducesOneProbeSpanFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontContractTestOneDashGroupProducesOneProbeSpanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontContractTestOneDashGroupProducesOneProbeSpanFault) -> Self {
        match value {
            FontContractTestOneDashGroupProducesOneProbeSpanFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault> for FontContractTestOneDashGroupProducesOneProbeSpanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault) -> Self {
        FontContractTestOneDashGroupProducesOneProbeSpanFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontContractTestOneDashGroupProducesOneProbeSpanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontContractTestOneDashGroupProducesOneProbeSpanFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontContractTestLaterSpansWinAndLooseValuesCoerceFault {
    TracedAssertionsAssertEqualsNullableStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for FontContractTestLaterSpansWinAndLooseValuesCoerceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontContractTestLaterSpansWinAndLooseValuesCoerceFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value) => write!(formatter, "{}", value),
            FontContractTestLaterSpansWinAndLooseValuesCoerceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<FontContractTestLaterSpansWinAndLooseValuesCoerceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault {
    fn from(value: FontContractTestLaterSpansWinAndLooseValuesCoerceFault) -> Self {
        match value {
            FontContractTestLaterSpansWinAndLooseValuesCoerceFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontContractTestLaterSpansWinAndLooseValuesCoerceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontContractTestLaterSpansWinAndLooseValuesCoerceFault) -> Self {
        match value {
            FontContractTestLaterSpansWinAndLooseValuesCoerceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault> for FontContractTestLaterSpansWinAndLooseValuesCoerceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableStringFault) -> Self {
        FontContractTestLaterSpansWinAndLooseValuesCoerceFault::TracedAssertionsAssertEqualsNullableStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontContractTestLaterSpansWinAndLooseValuesCoerceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontContractTestLaterSpansWinAndLooseValuesCoerceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn text_without_a_dash_returns_none() {
    testlib::run("org.tiqian.protocol.FontContractTest.textWithoutADashReturnsNone", "org.tiqian.protocol.FontContractTest.textWithoutADashReturnsNone", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,67,111,110,116,114,97,99,116,84,101,115,116])));
        recorder.section(UStr::new(&[116,101,120,116,87,105,116,104,111,117,116,65,68,97,115,104,82,101,116,117,114,110,115,78,111,110,101]));
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(None.clone(), FontContractTestSupport::font_contract_test_support_run(WireValue::WStr { value: UString::from("p1") }, WireValue::WStr { value: UString::from("正文段落") }, (WireValue::WNull).clone()).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(None.clone(), FontContractTestSupport::font_contract_test_support_run(WireValue::WStr { value: UString::from("p1") }, WireValue::WStr { value: UString::from("a—b") }, (WireValue::WNull).clone()).clone(), None).unwrap();
    });
}

#[test]
fn one_dash_group_produces_one_probe_span() {
    testlib::run("org.tiqian.protocol.FontContractTest.oneDashGroupProducesOneProbeSpan", "org.tiqian.protocol.FontContractTest.oneDashGroupProducesOneProbeSpan", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,67,111,110,116,114,97,99,116,84,101,115,116])));
        recorder.section(UStr::new(&[111,110,101,68,97,115,104,71,114,111,117,112,80,114,111,100,117,99,101,115,79,110,101,80,114,111,98,101,83,112,97,110]));
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("{\"key\":\"p1\",\"text\":\"——\",\"textSpans\":[{\"start\":0,\"end\":2,\"fontFamilies\":[\"Dela Gothic One\"],\"fontSizePx\":18,\"fontWeight\":400,\"italic\":false,\"baselineShiftPx\":0}]}")), FontContractTestSupport::font_contract_test_support_run(WireValue::WStr { value: UString::from("p1") }, WireValue::WStr { value: UString::from("前——后——尾") }, (WireValue::WNull).clone()).clone(), None).unwrap();
    });
}

#[test]
fn two_styled_dash_runs_group_by_style() {
    testlib::run("org.tiqian.protocol.FontContractTest.twoStyledDashRunsGroupByStyle", "org.tiqian.protocol.FontContractTest.twoStyledDashRunsGroupByStyle", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,67,111,110,116,114,97,99,116,84,101,115,116])));
        recorder.section(UStr::new(&[116,119,111,83,116,121,108,101,100,68,97,115,104,82,117,110,115,71,114,111,117,112,66,121,83,116,121,108,101]));
        let spans = WireValue::WArr { items: vec![
    (WireValue::WObj { fields: vec![
    (WireField { name: UString::from("start").to_ustring(), value: WireValue::WNum { value: 0.0f64 } }).clone(),
    (WireField { name: UString::from("end").to_ustring(), value: WireValue::WNum { value: 3.0f64 } }).clone(),
    (WireField { name: UString::from("fontFamilies").to_ustring(), value: WireValue::WArr { items: vec![(WireValue::WStr { value: UString::from("F B") }).clone()].to_vec() } }).clone(),
].to_vec() }).clone(),
    (WireValue::WObj { fields: vec![
    (WireField { name: UString::from("start").to_ustring(), value: WireValue::WNum { value: 3.0f64 } }).clone(),
    (WireField { name: UString::from("end").to_ustring(), value: WireValue::WNum { value: 9.0f64 } }).clone(),
    (WireField { name: UString::from("fontSizePx").to_ustring(), value: WireValue::WNum { value: 22.0f64 } }).clone(),
    (WireField { name: UString::from("italic").to_ustring(), value: WireValue::WBool { value: true } }).clone(),
].to_vec() }).clone(),
].to_vec() };
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("{\"key\":\"p1\",\"text\":\"——⸺——\",\"textSpans\":[{\"start\":0,\"end\":2,\"fontFamilies\":[\"F B\"],\"fontSizePx\":18,\"fontWeight\":400,\"italic\":false,\"baselineShiftPx\":0},{\"start\":2,\"end\":3,\"fontFamilies\":[\"Dela Gothic One\"],\"fontSizePx\":22,\"fontWeight\":400,\"italic\":true,\"baselineShiftPx\":0},{\"start\":3,\"end\":5,\"fontFamilies\":[\"Dela Gothic One\"],\"fontSizePx\":22,\"fontWeight\":400,\"italic\":true,\"baselineShiftPx\":0}]}")), FontContractTestSupport::font_contract_test_support_run(WireValue::WStr { value: UString::from("p1") }, WireValue::WStr { value: UString::from("前——中⸺尾——末") }, (spans).clone()).clone(), None).unwrap();
    });
}

#[test]
fn later_spans_win_and_loose_values_coerce() {
    testlib::run("org.tiqian.protocol.FontContractTest.laterSpansWinAndLooseValuesCoerce", "org.tiqian.protocol.FontContractTest.laterSpansWinAndLooseValuesCoerce", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,67,111,110,116,114,97,99,116,84,101,115,116])));
        recorder.section(UStr::new(&[108,97,116,101,114,83,112,97,110,115,87,105,110,65,110,100,76,111,111,115,101,86,97,108,117,101,115,67,111,101,114,99,101]));
        let spans = WireValue::WArr { items: vec![
    (WireValue::WObj { fields: vec![
    (WireField { name: UString::from("start").to_ustring(), value: WireValue::WNum { value: 0.0f64 } }).clone(),
    (WireField { name: UString::from("end").to_ustring(), value: WireValue::WNum { value: 2.0f64 } }).clone(),
    (WireField { name: UString::from("fontSizePx").to_ustring(), value: WireValue::WNum { value: 9.0f64 } }).clone(),
].to_vec() }).clone(),
    (WireValue::WObj { fields: vec![
    (WireField { name: UString::from("start").to_ustring(), value: WireValue::WNum { value: 0.0f64 } }).clone(),
    (WireField { name: UString::from("end").to_ustring(), value: WireValue::WNum { value: 2.0f64 } }).clone(),
    (WireField { name: UString::from("fontFamilies").to_ustring(), value: WireValue::WArr { items: vec![
    (WireValue::WStr { value: UString::from("A") }).clone(),
    (WireValue::WStr { value: UString::from("B") }).clone(),
].to_vec() } }).clone(),
    (WireField { name: UString::from("fontSizePx").to_ustring(), value: WireValue::WStr { value: UString::from("20") } }).clone(),
    (WireField { name: UString::from("fontWeight").to_ustring(), value: WireValue::WStr { value: UString::from("500") } }).clone(),
    (WireField { name: UString::from("baselineShiftPx").to_ustring(), value: WireValue::WStr { value: UString::from("1.5") } }).clone(),
].to_vec() }).clone(),
].to_vec() };
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("{\"key\":\"7\",\"text\":\"——\",\"textSpans\":[{\"start\":0,\"end\":2,\"fontFamilies\":[\"A\",\"B\"],\"fontSizePx\":20,\"fontWeight\":500,\"italic\":false,\"baselineShiftPx\":1.5}]}")), FontContractTestSupport::font_contract_test_support_run(WireValue::WNum { value: 7.0f64 }, WireValue::WStr { value: UString::from("——") }, (spans).clone()).clone(), None).unwrap();
    });
}

#[test]
fn unsafe_span_ranges_never_own_a_dash() {
    testlib::run("org.tiqian.protocol.FontContractTest.unsafeSpanRangesNeverOwnADash", "org.tiqian.protocol.FontContractTest.unsafeSpanRangesNeverOwnADash", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,67,111,110,116,114,97,99,116,84,101,115,116])));
        recorder.section(UStr::new(&[117,110,115,97,102,101,83,112,97,110,82,97,110,103,101,115,78,101,118,101,114,79,119,110,65,68,97,115,104]));
        let spans = WireValue::WArr { items: vec![
    (WireValue::WObj { fields: vec![
    (WireField { name: UString::from("start").to_ustring(), value: WireValue::WStr { value: UString::from("0") } }).clone(),
    (WireField { name: UString::from("end").to_ustring(), value: WireValue::WNum { value: 2.0f64 } }).clone(),
    (WireField { name: UString::from("fontFamilies").to_ustring(), value: WireValue::WArr { items: vec![(WireValue::WStr { value: UString::from("A") }).clone()].to_vec() } }).clone(),
].to_vec() }).clone(),
    (WireValue::WObj { fields: vec![
    (WireField { name: UString::from("start").to_ustring(), value: WireValue::WNum { value: 0.5f64 } }).clone(),
    (WireField { name: UString::from("end").to_ustring(), value: WireValue::WNum { value: 2.0f64 } }).clone(),
    (WireField { name: UString::from("fontFamilies").to_ustring(), value: WireValue::WArr { items: vec![(WireValue::WStr { value: UString::from("B") }).clone()].to_vec() } }).clone(),
].to_vec() }).clone(),
    (WireValue::WObj { fields: vec![
    (WireField { name: UString::from("end").to_ustring(), value: WireValue::WNum { value: 2.0f64 } }).clone(),
    (WireField { name: UString::from("fontFamilies").to_ustring(), value: WireValue::WArr { items: vec![(WireValue::WStr { value: UString::from("C") }).clone()].to_vec() } }).clone(),
].to_vec() }).clone(),
].to_vec() };
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("{\"key\":\"p1\",\"text\":\"——\",\"textSpans\":[{\"start\":0,\"end\":2,\"fontFamilies\":[\"Dela Gothic One\"],\"fontSizePx\":18,\"fontWeight\":400,\"italic\":false,\"baselineShiftPx\":0}]}")), FontContractTestSupport::font_contract_test_support_run(WireValue::WStr { value: UString::from("p1") }, WireValue::WStr { value: UString::from("——") }, (spans).clone()).clone(), None).unwrap();
    });
}
