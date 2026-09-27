#![cfg(test)]

use crate::org::tiqian::font::font_policy::font_role_name_uses_latin_face;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault {
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault) -> Self {
        match value {
            UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault) -> Self {
        match value {
            UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault) -> Self {
        match value {
            UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UsesLatinFaceTestNameOverloadAgreesWithEnumFault {
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UsesLatinFaceTestNameOverloadAgreesWithEnumFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: UsesLatinFaceTestNameOverloadAgreesWithEnumFault) -> Self {
        match value {
            UsesLatinFaceTestNameOverloadAgreesWithEnumFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UsesLatinFaceTestNameOverloadAgreesWithEnumFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: UsesLatinFaceTestNameOverloadAgreesWithEnumFault) -> Self {
        match value {
            UsesLatinFaceTestNameOverloadAgreesWithEnumFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UsesLatinFaceTestNameOverloadAgreesWithEnumFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UsesLatinFaceTestNameOverloadAgreesWithEnumFault) -> Self {
        match value {
            UsesLatinFaceTestNameOverloadAgreesWithEnumFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for UsesLatinFaceTestNameOverloadAgreesWithEnumFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        UsesLatinFaceTestNameOverloadAgreesWithEnumFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for UsesLatinFaceTestNameOverloadAgreesWithEnumFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        UsesLatinFaceTestNameOverloadAgreesWithEnumFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UsesLatinFaceTestNameOverloadAgreesWithEnumFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UsesLatinFaceTestNameOverloadAgreesWithEnumFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn only_latin_text_uses_latin_face() {
    testlib::run("org.tiqian.font.UsesLatinFaceTest.onlyLatinTextUsesLatinFace", "org.tiqian.font.UsesLatinFaceTest.onlyLatinTextUsesLatinFace", || {
        TestTraceRecorder::new("UsesLatinFaceTest").section(&"onlyLatinTextUsesLatinFace");
        let _ = TracedAssertions::traced_assertions_assert_true(FontRole::LatinText.uses_latin_face(), None).unwrap();
        let a = vec![
    FontRole::CjkText,
    FontRole::CjkPunctuation,
    FontRole::Symbol,
    FontRole::Emoji,
    FontRole::Unknown,
];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_false(a[usize::try_from(i).unwrap_or(0)].uses_latin_face(), Some((format!("{}{}",
            a[usize::try_from(i).unwrap_or(0)].name(),
            " must fall back to the CJK face"
        )).to_string())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn name_overload_agrees_with_enum() {
    testlib::run("org.tiqian.font.UsesLatinFaceTest.nameOverloadAgreesWithEnum", "org.tiqian.font.UsesLatinFaceTest.nameOverloadAgreesWithEnum", || {
        TestTraceRecorder::new("UsesLatinFaceTest").section(&"nameOverloadAgreesWithEnum");
        let names = vec![
    "CjkText".to_string(),
    "CjkPunctuation".to_string(),
    "LatinText".to_string(),
    "Symbol".to_string(),
    "Emoji".to_string(),
    "Unknown".to_string(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((names.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let x = (names[usize::try_from(i).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(if x == "LatinText" { "true".to_string() } else { "false".to_string() }.as_str(), if font_role_name_uses_latin_face(Some((x).to_string())) { "true".to_string() } else { "false".to_string() }.as_str(),
Some((format!("{}{}",
            "name overload must match the enum for ",
            x
        )).to_string())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(None.clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(Some("NotARole".to_string())), None).unwrap();
    });
}
