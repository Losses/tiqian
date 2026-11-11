#![cfg(test)]

use crate::org::tiqian::core::link_address_display::LinkAddressDisplay;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[test]
fn identical_display_and_target_is_an_address() {
    testlib::run("org.tiqian.core.LinkAddressDisplayTest.identicalDisplayAndTargetIsAnAddress", "org.tiqian.core.LinkAddressDisplayTest.identicalDisplayAndTargetIsAnAddress", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,110,107,65,100,100,114,101,115,115,68,105,115,112,108,97,121,84,101,115,116]))).section(UStr::new(&[105,100,101,110,116,105,99,97,108,68,105,115,112,108,97,121,65,110,100,84,97,114,103,101,116,73,115,65,110,65,100,100,114,101,115,115]));
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[104,116,116,112,115,58,47,47,101,120,97,109,112,108,101,46,99,111,109,47,97]), UStr::new(&[104,116,116,112,115,58,47,47,101,120,97,109,112,108,101,46,99,111,109,47,97])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[102,111,111,116,110,111,116,101,45,49]), UStr::new(&[102,111,111,116,110,111,116,101,45,49])), None).unwrap();
    });
}

#[test]
fn scheme_less_display_of_the_target_is_an_address() {
    testlib::run("org.tiqian.core.LinkAddressDisplayTest.schemeLessDisplayOfTheTargetIsAnAddress", "org.tiqian.core.LinkAddressDisplayTest.schemeLessDisplayOfTheTargetIsAnAddress", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,110,107,65,100,100,114,101,115,115,68,105,115,112,108,97,121,84,101,115,116]))).section(UStr::new(&[115,99,104,101,109,101,76,101,115,115,68,105,115,112,108,97,121,79,102,84,104,101,84,97,114,103,101,116,73,115,65,110,65,100,100,114,101,115,115]));
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[101,120,97,109,112,108,101,46,99,111,109,47,98]), UStr::new(&[104,116,116,112,115,58,47,47,101,120,97,109,112,108,101,46,99,111,109,47,98])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[101,120,97,109,112,108,101,46,99,111,109]), UStr::new(&[104,116,116,112,58,47,47,101,120,97,109,112,108,101,46,99,111,109])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[97,64,101,120,97,109,112,108,101,46,99,111,109]), UStr::new(&[109,97,105,108,116,111,58,97,64,101,120,97,109,112,108,101,46,99,111,109])), None).unwrap();
    });
}

#[test]
fn prose_display_text_is_not_an_address() {
    testlib::run("org.tiqian.core.LinkAddressDisplayTest.proseDisplayTextIsNotAnAddress", "org.tiqian.core.LinkAddressDisplayTest.proseDisplayTextIsNotAnAddress", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,110,107,65,100,100,114,101,115,115,68,105,115,112,108,97,121,84,101,115,116]))).section(UStr::new(&[112,114,111,115,101,68,105,115,112,108,97,121,84,101,120,116,73,115,78,111,116,65,110,65,100,100,114,101,115,115]));
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[69,120,97,109,112,108,101]), UStr::new(&[104,116,116,112,115,58,47,47,101,120,97,109,112,108,101,46,99,111,109])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[31034,20363,31449]), UStr::new(&[104,116,116,112,115,58,47,47,101,120,97,109,112,108,101,46,99,111,109])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[97,99,116,105,111,110]), UStr::new(&[103,101,110,101,114,105,99])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[]), UStr::new(&[104,116,116,112,115,58,47,47,101,120,97,109,112,108,101,46,99,111,109])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[69,120,97,109,112,108,101]), UStr::new(&[])), None).unwrap();
    });
}
