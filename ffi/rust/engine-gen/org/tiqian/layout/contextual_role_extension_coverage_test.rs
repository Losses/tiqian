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


#[derive(Debug, Clone, PartialEq)]
pub enum ContextualRoleExtensionCoverageTestContextualRoleExtensionsWrapOutsideThePipelineFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
        ContextualRoleExtensionCoverageSupport::contextual_role_extension_coverage_support_start(&"contextualRoleExtensionsWrapOutsideThePipeline");
        let base = CjkFontRoleClassifier::new();
        let context = FontRoleContext::new(Some("zh-Hans".to_string()), None);
        if !(ContextualDashEllipsisRoles::contextual_dash_ellipsis_roles_with_contextual_dash_ellipsis_roles((Box::new((base).clone())).clone(), &"中文", Some((context).clone())).unwrap().__haxe_type_name() == base.__haxe_type_name()) {
            let _ = TracedAssertions::traced_assertions_fail(Some("dash extension changed the mark-free receiver".to_string()), None).unwrap();
        }
        if !(QuotePairAwareFontRoleClassifier::quote_pair_aware_font_role_classifier_with_contextual_quote_roles((Box::new((base).clone())).clone(), &"中文", Some((context).clone())).unwrap().__haxe_type_name() == base.__haxe_type_name()) {
            let _ = TracedAssertions::traced_assertions_fail(Some("quote extension changed the mark-free receiver".to_string()), None).unwrap();
        }
        if !(ContextualDashEllipsisRoles::contextual_dash_ellipsis_roles_with_contextual_dash_ellipsis_roles((Box::new((base).clone())).clone(), &"中文", None).unwrap().__haxe_type_name() == base.__haxe_type_name()) {
            let _ = TracedAssertions::traced_assertions_fail(Some("context-free dash extension changed the mark-free receiver".to_string()), None).unwrap();
        }
        if !(QuotePairAwareFontRoleClassifier::quote_pair_aware_font_role_classifier_with_contextual_quote_roles((Box::new((base).clone())).clone(), &"中文", None).unwrap().__haxe_type_name() == base.__haxe_type_name()) {
            let _ = TracedAssertions::traced_assertions_fail(Some("context-free quote extension changed the mark-free receiver".to_string()), None).unwrap();
        }
        let dash_text = "中文—English".to_string();
        let dash_aware: Box<dyn FontRoleClassifier> = ContextualDashEllipsisRoles::contextual_dash_ellipsis_roles_with_contextual_dash_ellipsis_roles((Box::new((base).clone())).clone(), dash_text.as_str(), Some((context).clone())).unwrap();
        if dash_aware.classify(dash_text.as_str(), TextRange::new(2u32, 3u32).unwrap(), Some((context).clone())) != FontRole::CjkPunctuation {
            let _ = TracedAssertions::traced_assertions_fail(Some("dash-aware wrapper missed the dash run role".to_string()), None).unwrap();
        }
        if dash_aware.classify(dash_text.as_str(), TextRange::new(0u32, 1u32).unwrap(), Some((context).clone())) != base.classify(dash_text.as_str(), TextRange::new(0u32, 1u32).unwrap(), Some((context).clone())) {
            let _ = TracedAssertions::traced_assertions_fail(Some("dash-aware wrapper stopped delegating untouched ranges".to_string()), None).unwrap();
        }
        let quote_text = "中a“b”c文".to_string();
        let quote_aware: Box<dyn FontRoleClassifier> = QuotePairAwareFontRoleClassifier::quote_pair_aware_font_role_classifier_with_contextual_quote_roles((Box::new((base).clone())).clone(), quote_text.as_str(), Some((context).clone())).unwrap();
        if quote_aware.classify(quote_text.as_str(), TextRange::new(2u32, 3u32).unwrap(), Some((context).clone())) != FontRole::LatinText {
            let _ = TracedAssertions::traced_assertions_fail(Some("quote-aware wrapper missed the nested Latin quote role".to_string()), None).unwrap();
        }
    });
}

#[derive(Clone, Copy)]
pub struct ContextualRoleExtensionCoverageSupport;

impl ContextualRoleExtensionCoverageSupport {
    pub fn contextual_role_extension_coverage_support_start(n: &str) {
        TestTraceRecorder::new("ContextualRoleExtensionCoverageTest").section(n);
    }
}
