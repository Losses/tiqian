#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::font_decision_info::FontDecisionInfo;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver::ContextualDashEllipsisRoleResolver;
use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver::DashEllipsisRoleDecision;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum ContextualDashEllipsisRoleResolverTestLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for ContextualDashEllipsisRoleResolverTestLayoutFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualDashEllipsisRoleResolverTestLayoutFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisRoleResolverTestLayoutFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverTestLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualDashEllipsisRoleResolverTestLayoutFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverTestLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisRoleResolverTestLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualDashEllipsisRoleResolverTestLayoutFault) -> Self {
        match value {
            ContextualDashEllipsisRoleResolverTestLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualDashEllipsisRoleResolverTestLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualDashEllipsisRoleResolverTestLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualDashEllipsisRoleResolverTestLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualDashEllipsisRoleResolverTestLayoutFault::UStringFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct ContextualDashEllipsisRoleResolverTestSupport;

impl ContextualDashEllipsisRoleResolverTestSupport {
    pub fn contextual_dash_ellipsis_role_resolver_test_support_role(r: FontRole) -> UString {
        return UString::from(r.name());
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_fail(m: &UStr) -> Result<(), TextRangeError> {
        return Err(TextRangeError::Message { text: m.to_ustring() });
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_one(r: ContextualDashEllipsisRoleResolver, s: &UStr, c: Option<FontRoleContext>) -> Result<DashEllipsisRoleDecision, TextRangeError> {
        let d = r.resolve(s, (c).clone())?;
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 1 {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("expected one decision for ")); __s += s; __s }).as_str()).as_ustr())?;
        }
        return Ok((d[0usize]).clone());
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_all_role(d: &Vec<DashEllipsisRoleDecision>, r: FontRole) -> bool {
        for i in 0..match u32::try_from(d.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if d[usize::try_from(i).unwrap_or(0)].role != r {
                return false;
            }
        }
        return true;
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_all_source(d: &Vec<DashEllipsisRoleDecision>, s: &UStr) -> bool {
        for i in 0..match u32::try_from(d.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if d[usize::try_from(i).unwrap_or(0)].clone().source.to_ustring() != s {
                return false;
            }
        }
        return true;
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_all_reason(d: &Vec<DashEllipsisRoleDecision>, p: &UStr) -> bool {
        for i in 0..match u32::try_from(d.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !(((d[usize::try_from(i).unwrap_or(0)]).clone().reason).to_ustring()).starts_with(&p) {
                return false;
            }
        }
        return true;
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_render(d: &Vec<DashEllipsisRoleDecision>) -> UString {
        let mut a: Vec<UString> = vec![];
        for i in 0..match u32::try_from(d.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            a.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += UString::from(d[usize::try_from(i).unwrap_or(0)].role.name()).as_ustr(); __s += &(UString::from(":")); __s += ((d[usize::try_from(i).unwrap_or(0)]).clone().source).to_ustring().as_ustr(); __s += &(UString::from(":")); __s += ((d[usize::try_from(i).unwrap_or(0)]).clone().reason).to_ustring().as_ustr(); __s }).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined = a; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_layout(text: &UStr, locale: Option<UString>, spans: Option<Vec<TextSpan>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?.layout(LayoutInput::new(TiqianTextContent::new(text, Some((match &(spans) { None => vec![], Some(__option66) => (*__option66).clone() }).clone()), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some((match &(locale) { None => UString::from("zh-Hans"), Some(__option72) => __option72.to_ustring() }).to_ustring()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(1000.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_font_at(r: LayoutResult, index: u32) -> Result<FontDecisionInfo, TextRangeError> {
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((index) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((d.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((d.range).clone().end) as i32).to_ne_bytes())) {
                return Ok(d);
            }
        }
        return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("no font decision at index ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(index)).as_str())); __s }).as_str()) });
    }
}

#[test]
fn resolves_by_surrounding_script_rather_than_mark_count() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.resolvesBySurroundingScriptRatherThanMarkCount", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.resolvesBySurroundingScriptRatherThanMarkCount", || {
        let r = ContextualDashEllipsisRoleResolver::new().unwrap();
        let cases = vec![
    (vec![UString::from("English — next").to_ustring(), UString::from("LatinText").to_ustring()]).clone(),
    (vec![UString::from("— English").to_ustring(), UString::from("LatinText").to_ustring()]).clone(),
    (vec![UString::from("A——B").to_ustring(), UString::from("LatinText").to_ustring()]).clone(),
    (vec![UString::from("Wait…what").to_ustring(), UString::from("LatinText").to_ustring()]).clone(),
    (vec![UString::from("Wait……what").to_ustring(), UString::from("LatinText").to_ustring()]).clone(),
    (vec![UString::from("中文—下句").to_ustring(), UString::from("CjkPunctuation").to_ustring()]).clone(),
    (vec![UString::from("中文——下句").to_ustring(), UString::from("CjkPunctuation").to_ustring()]).clone(),
    (vec![UString::from("中文—123").to_ustring(), UString::from("CjkPunctuation").to_ustring()]).clone(),
    (vec![UString::from("123—English").to_ustring(), UString::from("LatinText").to_ustring()]).clone(),
    (vec![UString::from("中文……").to_ustring(), UString::from("CjkPunctuation").to_ustring()]).clone(),
    (vec![UString::from("等等…真的").to_ustring(), UString::from("CjkPunctuation").to_ustring()]).clone(),
    (vec![UString::from("等等……真的").to_ustring(), UString::from("CjkPunctuation").to_ustring()]).clone(),
];
        for x in &cases {
            let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), (x[0usize]).clone().as_ustr(), None).unwrap();
            if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_role(d.role) != (x[1usize]).clone() || (d.source).to_ustring() != UString::from("DashEllipsisSurroundingScriptContext") {
                let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("wrong role/source for ")); __s += (x[0usize]).clone().as_ustr(); __s }).as_str()).as_ustr()).unwrap();
            }
        }
    });
}

#[test]
fn conflicting_or_absent_script_falls_back_to_paragraph_language() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.conflictingOrAbsentScriptFallsBackToParagraphLanguage", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.conflictingOrAbsentScriptFallsBackToParagraphLanguage", || {
        let r = ContextualDashEllipsisRoleResolver::new().unwrap();
        {
            {
                let x_1 = UString::from("CjkPunctuation").to_ustring();
                let x_0 = UString::from("zh-Hans").to_ustring();
                let c = FontRoleContext::new(Some((x_0).to_ustring()), None);
                {
                    {
                        let s = UString::from("中文—English").to_ustring();
                        let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), s.as_ustr(), Some((c).clone())).unwrap();
                        if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_role(d.role) != x_1 || (d.source).to_ustring() != UString::from("ParagraphLanguageDashEllipsisContext") {
                            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("fallback ")); __s += x_0.as_ustr(); __s }).as_str()).as_ustr()).unwrap();
                        }
                    }
                    {
                        let s = UString::from("…").to_ustring();
                        let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), s.as_ustr(), Some((c).clone())).unwrap();
                        if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_role(d.role) != x_1 || (d.source).to_ustring() != UString::from("ParagraphLanguageDashEllipsisContext") {
                            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("fallback ")); __s += x_0.as_ustr(); __s }).as_str()).as_ustr()).unwrap();
                        }
                    }
                }
            }
            {
                let x_1 = UString::from("LatinText").to_ustring();
                let x_0 = UString::from("en-US").to_ustring();
                let c = FontRoleContext::new(Some((x_0).to_ustring()), None);
                {
                    {
                        let s = UString::from("中文—English").to_ustring();
                        let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), s.as_ustr(), Some((c).clone())).unwrap();
                        if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_role(d.role) != x_1 || (d.source).to_ustring() != UString::from("ParagraphLanguageDashEllipsisContext") {
                            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("fallback ")); __s += x_0.as_ustr(); __s }).as_str()).as_ustr()).unwrap();
                        }
                    }
                    {
                        let s = UString::from("…").to_ustring();
                        let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), s.as_ustr(), Some((c).clone())).unwrap();
                        if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_role(d.role) != x_1 || (d.source).to_ustring() != UString::from("ParagraphLanguageDashEllipsisContext") {
                            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("fallback ")); __s += x_0.as_ustr(); __s }).as_str()).as_ustr()).unwrap();
                        }
                    }
                }
            }
        }
    });
}

#[test]
fn decision_reason_names_the_evidence_shape() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.decisionReasonNamesTheEvidenceShape", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.decisionReasonNamesTheEvidenceShape", || {
        let r = ContextualDashEllipsisRoleResolver::new().unwrap();
        if !((ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), UStr::new(&[65,8212,66]), None).unwrap().reason).to_ustring()).starts_with(&UString::from("matching-surrounding-script")) || !((ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), UStr::new(&[20013,25991,8230,8230]), None).unwrap().reason).to_ustring()).starts_with(&UString::from("only-left-strong-script")) || !((ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), UStr::new(&[8212,32,69,110,103,108,105,115,104]), None).unwrap().reason).to_ustring()).starts_with(&UString::from("only-right-strong-script")) {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[119,114,111,110,103,32,101,118,105,100,101,110,99,101,32,114,101,97,115,111,110])).unwrap();
        }
    });
}

#[test]
fn mandatory_break_stops_context_search() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.mandatoryBreakStopsContextSearch", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.mandatoryBreakStopsContextSearch", || {
        let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one(ContextualDashEllipsisRoleResolver::new().unwrap(), UStr::new(&[8212,10,69,110,103,108,105,115,104]), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if d.role != FontRole::CjkPunctuation || (d.source).to_ustring() != UString::from("ParagraphLanguageDashEllipsisContext") || !((d.reason).to_ustring()).starts_with(&UString::from("no-strong-script-context")) {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[109,97,110,100,97,116,111,114,121,32,98,114,101,97,107,32,100,105,100,32,110,111,116,32,115,116,111,112,32,115,101,97,114,99,104])).unwrap();
        }
    });
}

#[test]
fn linear_context_index_preserves_supplementary_script_evidence() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.linearContextIndexPreservesSupplementaryScriptEvidence", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.linearContextIndexPreservesSupplementaryScriptEvidence", || {
        let a = { let mut __s = UString::new(); __s += TestHelpers::test_helpers_surrogate_text(&vec![55360, 56320]).as_ustr(); __s += &(UString::from("—123")); __s };
        let b = { let mut __s = UString::new(); __s += &(UString::from("123—")); __s += TestHelpers::test_helpers_surrogate_text(&vec![55297, 56320]).as_ustr(); __s };
        if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one(ContextualDashEllipsisRoleResolver::new().unwrap(), a.as_ustr(), None).unwrap().role != FontRole::CjkPunctuation || ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one(ContextualDashEllipsisRoleResolver::new().unwrap(), b.as_ustr(), None).unwrap().role != FontRole::LatinText {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[115,117,112,112,108,101,109,101,110,116,97,114,121,32,101,118,105,100,101,110,99,101])).unwrap();
        }
    });
}

#[test]
fn resolves_many_neutral_separated_runs_from_one_paragraph_index() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.resolvesManyNeutralSeparatedRunsFromOneParagraphIndex", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.resolvesManyNeutralSeparatedRunsFromOneParagraphIndex", || {
        let mut s = UString::from("A").to_ustring();
        for _ in 0..2048 {
            s += &(UString::from(" — "));
        }
        s += &(UString::from("B"));
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(s.as_ustr(), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2048 || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_role(&d, FontRole::LatinText) {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[109,97,110,121,32,114,117,110,115])).unwrap();
        }
    });
}

#[test]
fn pairs_parenthetical_dashes_across_inserted_content() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.pairsParentheticalDashesAcrossInsertedContent", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.pairsParentheticalDashesAcrossInsertedContent", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(UStr::new(&[20182,24443,22812,24819,74,101,115,115,105,99,97,8212,8212,74,101,115,115,105,99,97,26159,20182,30340,21069,22899,21451,8212,8212,30561,19981,30528,35273]), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_role(&d, FontRole::CjkPunctuation) || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_source(&d, UStr::new(&[80,97,114,97,103,114,97,112,104,76,97,110,103,117,97,103,101,68,97,115,104,69,108,108,105,112,115,105,115,67,111,110,116,101,120,116])) || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_reason(&d, UStr::new(&[112,97,114,101,110,116,104,101,116,105,99,97,108,45,112,97,105,114,45,99,111,110,102,108,105,99,116,105,110,103,45,111,117,116,101,114,45,115,99,114,105,112,116])) {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("parenthetical conflict: ")); __s += ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_render(&d).as_ustr(); __s }).as_str()).as_ustr()).unwrap();
        }
    });
}

#[test]
fn matching_outer_script_resolves_the_parenthetical_pair_directly() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.matchingOuterScriptResolvesTheParentheticalPairDirectly", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.matchingOuterScriptResolvesTheParentheticalPairDirectly", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(UStr::new(&[119,111,114,100,8212,8212,97,110,100,32,115,116,117,102,102,8212,8212,119,111,114,100]), None).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_role(&d, FontRole::LatinText) || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_source(&d, UStr::new(&[80,97,114,101,110,116,104,101,116,105,99,97,108,68,97,115,104,80,97,105,114,67,111,110,116,101,120,116])) {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[109,97,116,99,104,105,110,103,32,112,97,105,114])).unwrap();
        }
    });
}

#[test]
fn punctuation_between_runs_keeps_them_independent() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.punctuationBetweenRunsKeepsThemIndependent", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.punctuationBetweenRunsKeepsThemIndependent", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(UStr::new(&[22320,28857,8212,8212,21271,20140,65292,26102,38388,8212,8212,26126,22825]), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_role(&d, FontRole::CjkPunctuation) || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_source(&d, UStr::new(&[68,97,115,104,69,108,108,105,112,115,105,115,83,117,114,114,111,117,110,100,105,110,103,83,99,114,105,112,116,67,111,110,116,101,120,116])) {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[112,117,110,99,116,117,97,116,105,111,110,32,115,101,112,97,114,97,116,105,111,110])).unwrap();
        }
    });
}

#[test]
fn symbol_between_runs_keeps_them_independent() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.symbolBetweenRunsKeepsThemIndependent", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.symbolBetweenRunsKeepsThemIndependent", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(UStr::new(&[26102,20215,8212,8212,36,49,48,48,8212,8212,24456,36149]), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_source(&d, UStr::new(&[68,97,115,104,69,108,108,105,112,115,105,115,83,117,114,114,111,117,110,100,105,110,103,83,99,114,105,112,116,67,111,110,116,101,120,116])) {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[115,121,109,98,111,108,32,115,101,112,97,114,97,116,105,111,110])).unwrap();
        }
    });
}

#[test]
fn unequal_run_lengths_do_not_pair() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.unequalRunLengthsDoNotPair", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.unequalRunLengthsDoNotPair", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(UStr::new(&[24819,74,101,115,115,105,99,97,8212,8212,74,101,115,115,105,99,97,26159,21069,22899,21451,8212,30561,19981,30528]), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || d[0usize].role != FontRole::LatinText || d[1usize].role != FontRole::CjkPunctuation {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[117,110,101,113,117,97,108,32,114,117,110,115])).unwrap();
        }
    });
}

#[test]
fn ellipsis_runs_never_pair() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.ellipsisRunsNeverPair", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.ellipsisRunsNeverPair", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(UStr::new(&[24819,74,101,115,115,105,99,97,8230,8230,74,101,115,115,105,99,97,26159,20182,30340,21069,22899,21451,8230,8230,30561,19981,30528]), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || d[0usize].role != FontRole::LatinText || d[1usize].role != FontRole::CjkPunctuation {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[101,108,108,105,112,115,105,115,32,112,97,105,114])).unwrap();
        }
    });
}

#[test]
fn mandatory_break_between_runs_keeps_them_independent() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.mandatoryBreakBetweenRunsKeepsThemIndependent", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.mandatoryBreakBetweenRunsKeepsThemIndependent", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(UStr::new(&[24819,74,101,115,115,105,99,97,8212,8212,74,101,115,115,105,99,97,10,26159,21069,22899,21451,8212,8212,30561,19981,30528]), Some(FontRoleContext::new(Some(UString::from("zh-Hans")), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || d[0usize].role != FontRole::LatinText || d[1usize].role != FontRole::CjkPunctuation {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[98,114,101,97,107,32,112,97,105,114])).unwrap();
        }
    });
}
