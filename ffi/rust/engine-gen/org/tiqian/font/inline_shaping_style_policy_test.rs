#![cfg(test)]

use crate::org::tiqian::font::inline_shaping_style_policy::InlineShapingStylePolicy;
use crate::org::tiqian::font::inline_shaping_style_policy_test_support::InlineShapingStylePolicyTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn reports_first_property_when_it_diverges() {
    testlib::run("org.tiqian.font.InlineShapingStylePolicyTest.reportsFirstPropertyWhenItDiverges", "org.tiqian.font.InlineShapingStylePolicyTest.reportsFirstPropertyWhenItDiverges", || {
        TestTraceRecorder::new(&(UStr::new(&[73,110,108,105,110,101,83,104,97,112,105,110,103,83,116,121,108,101,80,111,108,105,99,121,84,101,115,116]))).section(UStr::new(&[114,101,112,111,114,116,115,70,105,114,115,116,80,114,111,112,101,114,116,121,87,104,101,110,73,116,68,105,118,101,114,103,101,115]));
        let mut a = InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15);
        { while a.len() <= 0usize { a.push(UString::new()); } a[0usize] = UString::from("divergent").to_ustring(); };
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[102,111,110,116,45,102,101,97,116,117,114,101,45,115,101,116,116,105,110,103,115]), (InlineShapingStylePolicy::inline_shaping_style_policy_first_divergent_property(&a, &InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15))).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
    });
}

#[test]
fn reports_middle_property_when_it_is_first_divergence() {
    testlib::run("org.tiqian.font.InlineShapingStylePolicyTest.reportsMiddlePropertyWhenItIsFirstDivergence", "org.tiqian.font.InlineShapingStylePolicyTest.reportsMiddlePropertyWhenItIsFirstDivergence", || {
        TestTraceRecorder::new(&(UStr::new(&[73,110,108,105,110,101,83,104,97,112,105,110,103,83,116,121,108,101,80,111,108,105,99,121,84,101,115,116]))).section(UStr::new(&[114,101,112,111,114,116,115,77,105,100,100,108,101,80,114,111,112,101,114,116,121,87,104,101,110,73,116,73,115,70,105,114,115,116,68,105,118,101,114,103,101,110,99,101]));
        let mut a = InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15);
        { while a.len() <= 3usize { a.push(UString::new()); } a[3usize] = UString::from("divergent").to_ustring(); };
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[102,111,110,116,45,107,101,114,110,105,110,103]), (InlineShapingStylePolicy::inline_shaping_style_policy_first_divergent_property(&a, &InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15))).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
    });
}

#[test]
fn returns_null_when_all_values_match() {
    testlib::run("org.tiqian.font.InlineShapingStylePolicyTest.returnsNullWhenAllValuesMatch", "org.tiqian.font.InlineShapingStylePolicyTest.returnsNullWhenAllValuesMatch", || {
        TestTraceRecorder::new(&(UStr::new(&[73,110,108,105,110,101,83,104,97,112,105,110,103,83,116,121,108,101,80,111,108,105,99,121,84,101,115,116]))).section(UStr::new(&[114,101,116,117,114,110,115,78,117,108,108,87,104,101,110,65,108,108,86,97,108,117,101,115,77,97,116,99,104]));
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(InlineShapingStylePolicy::inline_shaping_style_policy_first_divergent_property(&InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15), &InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15)).is_none(), UStr::new(&[45]), None).unwrap();
    });
}

#[test]
fn returns_null_for_empty_lists() {
    testlib::run("org.tiqian.font.InlineShapingStylePolicyTest.returnsNullForEmptyLists", "org.tiqian.font.InlineShapingStylePolicyTest.returnsNullForEmptyLists", || {
        TestTraceRecorder::new(&(UStr::new(&[73,110,108,105,110,101,83,104,97,112,105,110,103,83,116,121,108,101,80,111,108,105,99,121,84,101,115,116]))).section(UStr::new(&[114,101,116,117,114,110,115,78,117,108,108,70,111,114,69,109,112,116,121,76,105,115,116,115]));
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(InlineShapingStylePolicy::inline_shaping_style_policy_first_divergent_property(&vec![], &vec![]).is_none(), UStr::new(&[45]), None).unwrap();
    });
}

#[test]
fn longer_value_lists_stop_at_the_property_list_boundary() {
    testlib::run("org.tiqian.font.InlineShapingStylePolicyTest.longerValueListsStopAtThePropertyListBoundary", "org.tiqian.font.InlineShapingStylePolicyTest.longerValueListsStopAtThePropertyListBoundary", || {
        TestTraceRecorder::new(&(UStr::new(&[73,110,108,105,110,101,83,104,97,112,105,110,103,83,116,121,108,101,80,111,108,105,99,121,84,101,115,116]))).section(UStr::new(&[108,111,110,103,101,114,86,97,108,117,101,76,105,115,116,115,83,116,111,112,65,116,84,104,101,80,114,111,112,101,114,116,121,76,105,115,116,66,111,117,110,100,97,114,121]));
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(InlineShapingStylePolicy::inline_shaping_style_policy_first_divergent_property(&InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(19), &InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(19)).is_none(), UStr::new(&[45]), None).unwrap();
    });
}
