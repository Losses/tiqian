#![cfg(test)]

use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleClassifier;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver::ContextualDashEllipsisRoles;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAwareFontRoleClassifier;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault) -> Self {
        match value {
            ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault) -> Self {
        match value {
            ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault) -> Self {
        match value {
            ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn contextual_role_extensions_wrap_outside_the_pipeline() {
    testlib::run("org.tiqian.layout.ContextualRoleExtensionCoverageTest.contextualRoleExtensionsWrapOutsideThePipeline", "org.tiqian.layout.ContextualRoleExtensionCoverageTest.contextualRoleExtensionsWrapOutsideThePipeline", || {
        ContextualRoleExtensionCoverageSupport::contextual_role_extension_coverage_support_start(UStr::new(&[99,111,110,116,101,120,116,117,97,108,82,111,108,101,69,120,116,101,110,115,105,111,110,115,87,114,97,112,79,117,116,115,105,100,101,84,104,101,80,105,112,101,108,105,110,101]));
        let base = CjkFontRoleClassifier::new();
        let context = FontRoleContext::new(Some(UString::from("zh-Hans")), None);
        if !(ContextualDashEllipsisRoles::contextual_dash_ellipsis_roles_with_contextual_dash_ellipsis_roles((Box::new((base).clone())).clone(), UStr::new(&[20013,25991]), Some((context).clone())).unwrap().__haxe_type_name() == base.__haxe_type_name()) {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from("dash extension changed the mark-free receiver")), None).unwrap();
        }
        if !(QuotePairAwareFontRoleClassifier::quote_pair_aware_font_role_classifier_with_contextual_quote_roles((Box::new((base).clone())).clone(), UStr::new(&[20013,25991]), Some((context).clone())).unwrap().__haxe_type_name() == base.__haxe_type_name()) {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from("quote extension changed the mark-free receiver")), None).unwrap();
        }
        if !(ContextualDashEllipsisRoles::contextual_dash_ellipsis_roles_with_contextual_dash_ellipsis_roles((Box::new((base).clone())).clone(), UStr::new(&[20013,25991]), None).unwrap().__haxe_type_name() == base.__haxe_type_name()) {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from("context-free dash extension changed the mark-free receiver")), None).unwrap();
        }
        if !(QuotePairAwareFontRoleClassifier::quote_pair_aware_font_role_classifier_with_contextual_quote_roles((Box::new((base).clone())).clone(), UStr::new(&[20013,25991]), None).unwrap().__haxe_type_name() == base.__haxe_type_name()) {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from("context-free quote extension changed the mark-free receiver")), None).unwrap();
        }
        let dash_text = UString::from("中文—English").to_ustring();
        let dash_aware: Box<dyn FontRoleClassifier> = ContextualDashEllipsisRoles::contextual_dash_ellipsis_roles_with_contextual_dash_ellipsis_roles((Box::new((base).clone())).clone(), dash_text.as_ustr(), Some((context).clone())).unwrap();
        if dash_aware.classify(dash_text.as_ustr(), TextRange::new(2u32, 3u32).unwrap(), Some((context).clone())) != FontRole::CjkPunctuation {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from("dash-aware wrapper missed the dash run role")), None).unwrap();
        }
        if dash_aware.classify(dash_text.as_ustr(), TextRange::new(0u32, 1u32).unwrap(), Some((context).clone())) != base.classify(dash_text.as_ustr(), TextRange::new(0u32, 1u32).unwrap(), Some((context).clone())) {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from("dash-aware wrapper stopped delegating untouched ranges")), None).unwrap();
        }
        let quote_text = UString::from("中a“b”c文").to_ustring();
        let quote_aware: Box<dyn FontRoleClassifier> = QuotePairAwareFontRoleClassifier::quote_pair_aware_font_role_classifier_with_contextual_quote_roles((Box::new((base).clone())).clone(), quote_text.as_ustr(), Some((context).clone())).unwrap();
        if quote_aware.classify(quote_text.as_ustr(), TextRange::new(2u32, 3u32).unwrap(), Some((context).clone())) != FontRole::LatinText {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from("quote-aware wrapper missed the nested Latin quote role")), None).unwrap();
        }
    });
}

#[derive(Clone, Copy)]
pub struct ContextualRoleExtensionCoverageSupport;

impl ContextualRoleExtensionCoverageSupport {
    pub fn contextual_role_extension_coverage_support_start(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[67,111,110,116,101,120,116,117,97,108,82,111,108,101,69,120,116,101,110,115,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116]))).section(n);
    }
}
