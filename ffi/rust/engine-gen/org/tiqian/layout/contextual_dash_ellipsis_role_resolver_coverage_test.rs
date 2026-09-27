#![cfg(test)]

use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver::ContextualDashEllipsisRoleResolver;
use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver::DashEllipsisRoleDecision;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum ContextualDashEllipsisRoleResolverCoverageTestParentheticalPairWithoutOuterScriptFallsBackToParagraphLanguageFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
    pub fn contextual_dash_ellipsis_role_resolver_coverage_support_start(n: &str) {
        TestTraceRecorder::new("ContextualDashEllipsisRoleResolverCoverageTest").section(n);
    }

    pub fn contextual_dash_ellipsis_role_resolver_coverage_support_surrogate_text(c: &Vec<u32>) -> String {
        let mut s = String::new();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            s += &(if c[usize::try_from(i).unwrap_or(0)] > 0xFFFF { String::from_utf16(&[0xD800 + (((c[usize::try_from(i).unwrap_or(0)]) - 0x10000) >> 10) as u16, 0xDC00 + (((c[usize::try_from(i).unwrap_or(0)]) - 0x10000) & 0x3FF) as u16]).unwrap() } else {
String::from_utf16_lossy(&[(c[usize::try_from(i).unwrap_or(0)]) as u16]) });
            i = u32::wrapping_add(i, 1);
        }
        return s;
    }

    pub fn contextual_dash_ellipsis_role_resolver_coverage_support_valid(d: &Vec<DashEllipsisRoleDecision>, source: &str, prefix: &str) -> bool {
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 {
            return false;
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if d[usize::try_from(i).unwrap_or(0)].role != FontRole::CjkPunctuation || ((d[usize::try_from(i).unwrap_or(0)]).clone().source).to_string() != source || !(((d[usize::try_from(i).unwrap_or(0)]).clone().reason).to_string()).starts_with(&prefix) {
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
        ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_start(&"parentheticalPairWithOnlyLeftOuterScriptTakesTheLeftRole");
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(&"中文——word——", Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if !ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_valid(&d, &"ParentheticalDashPairContext", &"only-left-outer-script") {
            let _ = TracedAssertions::traced_assertions_fail(Some({
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
    }.to_string()), None).unwrap();
        }
    });
}

#[test]
fn parenthetical_pair_with_only_right_outer_script_takes_the_right_role() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.parentheticalPairWithOnlyRightOuterScriptTakesTheRightRole", "org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.parentheticalPairWithOnlyRightOuterScriptTakesTheRightRole", || {
        ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_start(&"parentheticalPairWithOnlyRightOuterScriptTakesTheRightRole");
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(&"——word——中文", Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if !ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_valid(&d, &"ParentheticalDashPairContext", &"only-right-outer-script") {
            let _ = TracedAssertions::traced_assertions_fail(Some({
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
    }.to_string()), None).unwrap();
        }
    });
}

#[test]
fn parenthetical_pair_without_outer_script_falls_back_to_paragraph_language() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.parentheticalPairWithoutOuterScriptFallsBackToParagraphLanguage", "org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.parentheticalPairWithoutOuterScriptFallsBackToParagraphLanguage",
|| {
        ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_start(&"parentheticalPairWithoutOuterScriptFallsBackToParagraphLanguage");
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(&"——word——", Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if !ContextualDashEllipsisRoleResolverCoverageSupport::contextual_dash_ellipsis_role_resolver_coverage_support_valid(&d, &"ParagraphLanguageDashEllipsisContext", &"parenthetical-pair-no-outer-context") {
            let _ = TracedAssertions::traced_assertions_fail(Some({
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
    }.to_string()), None).unwrap();
        }
    });
}

#[test]
fn forward_pass_walker_arms_run_before_the_classifier_rejects_lone_surrogates() {
    testlib::record_not_applicable("org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.forwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogates",
"org.tiqian.layout.ContextualDashEllipsisRoleResolverCoverageTest.forwardPassWalkerArmsRunBeforeTheClassifierRejectsLoneSurrogates");
}
