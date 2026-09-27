#![cfg(test)]

use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver::ContextualDashEllipsisRoleResolver;
use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver::DashEllipsisRoleDecision;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyRightOuterScriptTakesTheRightRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ContextualDashEllipsisRoleResolverCoverageTestForwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogatesFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct ContextualDashEllipsisRoleResolverCoverageSupport;

impl ContextualDashEllipsisRoleResolverCoverageSupport {
    pub fn contextual_dash_ellipsis_role_resolver_coverage_support_start(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[67,111,110,116,101,120,116,117,97,108,68,97,115,104,69,108,108,105,112,115,105,115,82,111,108,101,82,101,115,111,108,118,101,114,67,111,118,101,114,97,103,101,84,101,115,116]))).section(n);
    }

    pub fn contextual_dash_ellipsis_role_resolver_coverage_support_surrogate_text(c: &Vec<u32>) -> UString {
        let mut s = UString::new();
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            s += &(if c[usize::try_from(i).unwrap_or(0)] > 0xFFFF { u_string::from_units(&[0xD800 + (((c[usize::try_from(i).unwrap_or(0)]) - 0x10000) >> 10) as u16, 0xDC00 + (((c[usize::try_from(i).unwrap_or(0)]) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(c[usize::try_from(i).unwrap_or(0)]) as u16]) });
            i = u32::wrapping_add(i, 1);
        }
        return s;
    }

    pub fn contextual_dash_ellipsis_role_resolver_coverage_support_valid(d: &Vec<DashEllipsisRoleDecision>, source: &UStr, prefix: &UStr) -> bool {
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 {
            return false;
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if d[usize::try_from(i).unwrap_or(0)].role != FontRole::CjkPunctuation || ((d[usize::try_from(i).unwrap_or(0)]).clone().source).to_ustring() != source || !(((d[usize::try_from(i).unwrap_or(0)]).clone().reason).to_ustring()).starts_with(&prefix) {
                return false;
            }
            i = u32::wrapping_add(i, 1);
        }
        return true;
    }
}

#[test]
fn parenthetical_pair_with_only_left_outer_script_takes_the_left_role() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.parentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRole", "org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.parentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRole", || {
        ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_start(UStr::new(&[112,97,114,101,110,116,104,101,116,105,99,97,108,80,97,105,114,87,105,116,104,79,110,108,121,76,101,102,116,79,117,116,101,114,83,99,114,105,112,116,84,97,107,101,115,84,104,101,76,101,102,116,82,111,108,101]));
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(UStr::new(&[20013,25991,8212,8212,119,111,114,100,8212,8212]), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if !ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_valid(&d, UStr::new(&[80,97,114,101,110,116,104,101,116,105,99,97,108,68,97,115,104,80,97,105,114,67,111,110,116,101,120,116]), UStr::new(&[111,110,108,121,45,108,101,102,116,45,111,117,116,101,114,45,115,99,114,105,112,116])) {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = d;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str())), None).unwrap();
        }
    });
}

#[test]
fn parenthetical_pair_with_only_right_outer_script_takes_the_right_role() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.parentheticalPairWithOnlyRightOuterScriptTakesTheRightRole", "org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.parentheticalPairWithOnlyRightOuterScriptTakesTheRightRole", || {
        ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_start(UStr::new(&[112,97,114,101,110,116,104,101,116,105,99,97,108,80,97,105,114,87,105,116,104,79,110,108,121,82,105,103,104,116,79,117,116,101,114,83,99,114,105,112,116,84,97,107,101,115,84,104,101,82,105,103,104,116,82,111,108,101]));
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(UStr::new(&[8212,8212,119,111,114,100,8212,8212,20013,25991]), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if !ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_valid(&d, UStr::new(&[80,97,114,101,110,116,104,101,116,105,99,97,108,68,97,115,104,80,97,105,114,67,111,110,116,101,120,116]), UStr::new(&[111,110,108,121,45,114,105,103,104,116,45,111,117,116,101,114,45,115,99,114,105,112,116])) {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = d;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str())), None).unwrap();
        }
    });
}

#[test]
fn parenthetical_pair_without_outer_script_falls_back_to_paragraph_language() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.parentheticalPairWithoutOuterScriptFallsBackToParagraphLanguage", "org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.parentheticalPairWithoutOuterScriptFallsBackToParagraphLanguage", || {
        ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_start(UStr::new(&[112,97,114,101,110,116,104,101,116,105,99,97,108,80,97,105,114,87,105,116,104,111,117,116,79,117,116,101,114,83,99,114,105,112,116,70,97,108,108,115,66,97,99,107,84,111,80,97,114,97,103,114,97,112,104,76,97,110,103,117,97,103,101]));
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(UStr::new(&[8212,8212,119,111,114,100,8212,8212]), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if !ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_valid(&d, UStr::new(&[80,97,114,97,103,114,97,112,104,76,97,110,103,117,97,103,101,68,97,115,104,69,108,108,105,112,115,105,115,67,111,110,116,101,120,116]), UStr::new(&[112,97,114,101,110,116,104,101,116,105,99,97,108,45,112,97,105,114,45,110,111,45,111,117,116,101,114,45,99,111,110,116,101,120,116])) {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = d;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str())), None).unwrap();
        }
    });
}

#[test]
fn forward_pass_walker_arms_run_before_the_classifier_rejects_lone_surrogates() {
    testlib::record_not_applicable("org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.forwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogates", "org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.forwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogates");
}
