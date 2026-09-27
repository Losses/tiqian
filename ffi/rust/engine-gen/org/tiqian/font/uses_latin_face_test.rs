#![cfg(test)]

use crate::org::tiqian::font::font_policy::font_role_name_uses_latin_face;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault {
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            UsesLatinFaceTestOnlyLatinTextUsesLatinFaceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for UsesLatinFaceTestNameOverloadAgreesWithEnumFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UsesLatinFaceTestNameOverloadAgreesWithEnumFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => write!(formatter, "{}", value),
            UsesLatinFaceTestNameOverloadAgreesWithEnumFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            UsesLatinFaceTestNameOverloadAgreesWithEnumFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[85,115,101,115,76,97,116,105,110,70,97,99,101,84,101,115,116]))).section(UStr::new(&[111,110,108,121,76,97,116,105,110,84,101,120,116,85,115,101,115,76,97,116,105,110,70,97,99,101]));
        let _ = TracedAssertions::traced_assertions_assert_true(FontRole::LatinText.uses_latin_face(), None).unwrap();
        let a = vec![
    FontRole::CjkText,
    FontRole::CjkPunctuation,
    FontRole::Symbol,
    FontRole::Emoji,
    FontRole::Unknown,
];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_false(a[usize::try_from(i).unwrap_or(0)].uses_latin_face(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += UString::from(a[usize::try_from(i).unwrap_or(0)].name()).as_ustr(); __s += &(UString::from(" must fall back to the CJK face")); __s }).as_str()))).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn name_overload_agrees_with_enum() {
    testlib::run("org.tiqian.font.UsesLatinFaceTest.nameOverloadAgreesWithEnum", "org.tiqian.font.UsesLatinFaceTest.nameOverloadAgreesWithEnum", || {
        TestTraceRecorder::new(&(UStr::new(&[85,115,101,115,76,97,116,105,110,70,97,99,101,84,101,115,116]))).section(UStr::new(&[110,97,109,101,79,118,101,114,108,111,97,100,65,103,114,101,101,115,87,105,116,104,69,110,117,109]));
        let names = vec![
    UString::from("CjkText").to_ustring(),
    UString::from("CjkPunctuation").to_ustring(),
    UString::from("LatinText").to_ustring(),
    UString::from("Symbol").to_ustring(),
    UString::from("Emoji").to_ustring(),
    UString::from("Unknown").to_ustring(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((names.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let x = (names[usize::try_from(i).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(if x == UString::from("LatinText") { UString::from("true") } else { UString::from("false") }.as_ustr(), if font_role_name_uses_latin_face(Some((x).to_ustring())) { UString::from("true") } else { UString::from("false") }.as_ustr(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("name overload must match the enum for ")); __s += x.as_ustr(); __s }).as_str()))).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(None.clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(Some(UString::from("NotARole"))), None).unwrap();
    });
}
