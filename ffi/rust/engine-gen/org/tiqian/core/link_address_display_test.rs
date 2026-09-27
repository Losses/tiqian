#![cfg(test)]

use crate::org::tiqian::core::link_address_display::LinkAddressDisplay;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[test]
fn identical_display_and_target_is_an_address() {
    testlib::run("org.tiqian.core.LinkAddressDisplayTest.identicalDisplayAndTargetIsAnAddress", "org.tiqian.core.LinkAddressDisplayTest.identicalDisplayAndTargetIsAnAddress", || {
        TestTraceRecorder::new("LinkAddressDisplayTest").section(&"identicalDisplayAndTargetIsAnAddress");
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(&"https://example.com/a", &"https://example.com/a"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(&"footnote-1", &"footnote-1"), None).unwrap();
    });
}

#[test]
fn scheme_less_display_of_the_target_is_an_address() {
    testlib::run("org.tiqian.core.LinkAddressDisplayTest.schemeLessDisplayOfTheTargetIsAnAddress", "org.tiqian.core.LinkAddressDisplayTest.schemeLessDisplayOfTheTargetIsAnAddress", || {
        TestTraceRecorder::new("LinkAddressDisplayTest").section(&"schemeLessDisplayOfTheTargetIsAnAddress");
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(&"example.com/b", &"https://example.com/b"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(&"example.com", &"http://example.com"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(&"a@example.com", &"mailto:a@example.com"), None).unwrap();
    });
}

#[test]
fn prose_display_text_is_not_an_address() {
    testlib::run("org.tiqian.core.LinkAddressDisplayTest.proseDisplayTextIsNotAnAddress", "org.tiqian.core.LinkAddressDisplayTest.proseDisplayTextIsNotAnAddress", || {
        TestTraceRecorder::new("LinkAddressDisplayTest").section(&"proseDisplayTextIsNotAnAddress");
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(&"Example", &"https://example.com"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(&"示例站", &"https://example.com"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(&"action", &"generic"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(&"", &"https://example.com"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(&"Example", &""), None).unwrap();
    });
}
