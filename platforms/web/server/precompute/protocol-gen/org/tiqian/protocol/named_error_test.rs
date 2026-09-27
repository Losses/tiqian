#![cfg(test)]

use crate::org::tiqian::protocol::named_error_names::NamedErrorNames;
use crate::org::tiqian::protocol::named_error_test_support::NamedErrorTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;


#[test]
fn the_variant_list_matches_the_published_names_in_check_order() {
    testlib::run("org.tiqian.protocol.NamedErrorTest.theVariantListMatchesThePublishedNamesInCheckOrder", "org.tiqian.protocol.NamedErrorTest.theVariantListMatchesThePublishedNamesInCheckOrder", || {
        let mut recorder = TestTraceRecorder::new("NamedErrorTest");
        recorder.section(&"theVariantListMatchesThePublishedNamesInCheckOrder");
        let variants = NamedErrorNames::named_error_names_variants();
        let mut names: Vec<String> = vec![];
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((variants.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            names.push(NamedErrorNames::named_error_names_describe(NamedErrorTestSupport::named_error_test_support_variant_at(index_idx)));
            let _ = TracedAssertions::traced_assertions_assert_true(variants[usize::try_from(index_idx).unwrap_or(0)] == NamedErrorTestSupport::named_error_test_support_variant_at(index_idx), Some("the variant list order drifts from the declaration order".to_string())).unwrap();
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(u32::try_from((NamedErrorTestSupport::named_error_test_support_golden().len()) & 0xFFFF_FFFF).unwrap_or(0), u32::try_from((variants.len()) & 0xFFFF_FFFF).unwrap_or(0),
Some("the family has the 21 published names".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&NamedErrorTestSupport::named_error_test_support_golden(), &names, None).unwrap();
    });
}

#[test]
fn the_published_names_are_all_distinct() {
    testlib::run("org.tiqian.protocol.NamedErrorTest.thePublishedNamesAreAllDistinct", "org.tiqian.protocol.NamedErrorTest.thePublishedNamesAreAllDistinct", || {
        let mut recorder = TestTraceRecorder::new("NamedErrorTest");
        recorder.section(&"thePublishedNamesAreAllDistinct");
        let variants = NamedErrorNames::named_error_names_variants();
        let mut outer_idx = 0u32;
        while (i32::from_ne_bytes((outer_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((variants.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let mut inner_idx = u32::wrapping_add(outer_idx, 1);
            while (i32::from_ne_bytes((inner_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((variants.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                let _ = TracedAssertions::traced_assertions_assert_true(NamedErrorNames::named_error_names_describe(NamedErrorTestSupport::named_error_test_support_variant_at(outer_idx)) !=
NamedErrorNames::named_error_names_describe(NamedErrorTestSupport::named_error_test_support_variant_at(inner_idx)), Some("two variants publish the same name".to_string())).unwrap();
                inner_idx = u32::wrapping_add(inner_idx, 1);
            }
            outer_idx = u32::wrapping_add(outer_idx, 1);
        }
    });
}
