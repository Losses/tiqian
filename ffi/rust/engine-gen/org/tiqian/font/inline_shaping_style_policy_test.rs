#![cfg(test)]

use crate::org::tiqian::font::inline_shaping_style_policy::InlineShapingStylePolicy;
use crate::org::tiqian::font::inline_shaping_style_policy_test_support::InlineShapingStylePolicyTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[test]
fn reports_first_property_when_it_diverges() {
    testlib::run("org.tiqian.font.InlineShapingStylePolicyTest.reportsFirstPropertyWhenItDiverges", "org.tiqian.font.InlineShapingStylePolicyTest.reportsFirstPropertyWhenItDiverges", || {
        TestTraceRecorder::new("InlineShapingStylePolicyTest").section(&"reportsFirstPropertyWhenItDiverges");
        let mut a = InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15);
        { while a.len() <= 0usize { a.push(String::new()); } a[0usize] = "divergent".to_string(); };
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"font-feature-settings", (InlineShapingStylePolicy::inline_shaping_style_policy_first_divergent_property(&a,
&InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15))).as_deref().unwrap_or(""), None).unwrap();
    });
}

#[test]
fn reports_middle_property_when_it_is_first_divergence() {
    testlib::run("org.tiqian.font.InlineShapingStylePolicyTest.reportsMiddlePropertyWhenItIsFirstDivergence", "org.tiqian.font.InlineShapingStylePolicyTest.reportsMiddlePropertyWhenItIsFirstDivergence", || {
        TestTraceRecorder::new("InlineShapingStylePolicyTest").section(&"reportsMiddlePropertyWhenItIsFirstDivergence");
        let mut a = InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15);
        { while a.len() <= 3usize { a.push(String::new()); } a[3usize] = "divergent".to_string(); };
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"font-kerning", (InlineShapingStylePolicy::inline_shaping_style_policy_first_divergent_property(&a,
&InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15))).as_deref().unwrap_or(""), None).unwrap();
    });
}

#[test]
fn returns_null_when_all_values_match() {
    testlib::run("org.tiqian.font.InlineShapingStylePolicyTest.returnsNullWhenAllValuesMatch", "org.tiqian.font.InlineShapingStylePolicyTest.returnsNullWhenAllValuesMatch", || {
        TestTraceRecorder::new("InlineShapingStylePolicyTest").section(&"returnsNullWhenAllValuesMatch");
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(InlineShapingStylePolicy::inline_shaping_style_policy_first_divergent_property(&InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15),
&InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(15)).is_none(), &"-", None).unwrap();
    });
}

#[test]
fn returns_null_for_empty_lists() {
    testlib::run("org.tiqian.font.InlineShapingStylePolicyTest.returnsNullForEmptyLists", "org.tiqian.font.InlineShapingStylePolicyTest.returnsNullForEmptyLists", || {
        TestTraceRecorder::new("InlineShapingStylePolicyTest").section(&"returnsNullForEmptyLists");
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(InlineShapingStylePolicy::inline_shaping_style_policy_first_divergent_property(&vec![], &vec![]).is_none(), &"-", None).unwrap();
    });
}

#[test]
fn longer_value_lists_stop_at_the_property_list_boundary() {
    testlib::run("org.tiqian.font.InlineShapingStylePolicyTest.longerValueListsStopAtThePropertyListBoundary", "org.tiqian.font.InlineShapingStylePolicyTest.longerValueListsStopAtThePropertyListBoundary", || {
        TestTraceRecorder::new("InlineShapingStylePolicyTest").section(&"longerValueListsStopAtThePropertyListBoundary");
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(InlineShapingStylePolicy::inline_shaping_style_policy_first_divergent_property(&InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(19),
&InlineShapingStylePolicyTestSupport::inline_shaping_style_policy_test_support_vals(19)).is_none(), &"-", None).unwrap();
    });
}
