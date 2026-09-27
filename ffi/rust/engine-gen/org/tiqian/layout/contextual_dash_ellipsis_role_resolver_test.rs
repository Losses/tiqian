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
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum ContextualDashEllipsisRoleResolverTestLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
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
    pub fn contextual_dash_ellipsis_role_resolver_test_support_role(r: FontRole) -> String {
        return r.name().to_string();
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_fail(m: &str) -> Result<(), TextRangeError> {
        return Err(TextRangeError::Message { text: m.to_string() });
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_one(r: ContextualDashEllipsisRoleResolver, s: &str, c: Option<FontRoleContext>) -> Result<DashEllipsisRoleDecision, TextRangeError> {
        let d = r.resolve(s, (c).clone())?;
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 1 {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(format!("{}{}",
            "expected one decision for ",
            s
        ).as_str())?;
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

    pub fn contextual_dash_ellipsis_role_resolver_test_support_all_source(d: &Vec<DashEllipsisRoleDecision>, s: &str) -> bool {
        for i in 0..match u32::try_from(d.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if d[usize::try_from(i).unwrap_or(0)].clone().source.to_string() != s {
                return false;
            }
        }
        return true;
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_all_reason(d: &Vec<DashEllipsisRoleDecision>, p: &str) -> bool {
        for i in 0..match u32::try_from(d.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !(((d[usize::try_from(i).unwrap_or(0)]).clone().reason).to_string()).starts_with(&p) {
                return false;
            }
        }
        return true;
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_render(d: &Vec<DashEllipsisRoleDecision>) -> String {
        let mut a: Vec<String> = vec![];
        for i in 0..match u32::try_from(d.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            a.push(format!("{}{}{}{}{}",
            d[usize::try_from(i).unwrap_or(0)].role.name().to_string(),
            ":",
            ((d[usize::try_from(i).unwrap_or(0)]).clone().source).to_string(),
            ":",
            ((d[usize::try_from(i).unwrap_or(0)]).clone().reason).to_string()
        ));
        }
        return format!("{}{}{}",
            "[",
            { let joined = a; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined[index]); index += 1; } out },
            "]"
        );
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_layout(text: &str, locale: Option<String>, spans: Option<Vec<TextSpan>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?.layout(LayoutInput::new(TiqianTextContent::new(text, Some((match &(spans) { None => vec![], Some(__option66) => (*__option66).clone() }).clone()), Some(vec![]), Some(vec![]), Some(vec![])),
Some(TextStyle::new(Some(vec![]), Some(16.0), Some(match &(locale) { None => "zh-Hans".to_string(), Some(__option72) => __option72.to_string() }.to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(1000.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e|
ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e|
ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn contextual_dash_ellipsis_role_resolver_test_support_font_at(r: LayoutResult, index: u32) -> Result<FontDecisionInfo, TextRangeError> {
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if i32::from_ne_bytes((index).to_ne_bytes()) >= i32::from_ne_bytes(((d.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes(((d.range).clone().end).to_ne_bytes())) {
                return Ok(d);
            }
        }
        return Err(TextRangeError::Message { text: format!("{}{}",
            "no font decision at index ",
            crate::runtime::int_text::IntText::int_text(index)
        ).to_string() });
    }
}

#[test]
fn resolves_by_surrounding_script_rather_than_mark_count() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.resolvesBySurroundingScriptRatherThanMarkCount", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.resolvesBySurroundingScriptRatherThanMarkCount", || {
        let r = ContextualDashEllipsisRoleResolver::new().unwrap();
        let cases = vec![
    (vec!["English — next".to_string(), "LatinText".to_string()]).clone(),
    (vec!["— English".to_string(), "LatinText".to_string()]).clone(),
    (vec!["A——B".to_string(), "LatinText".to_string()]).clone(),
    (vec!["Wait…what".to_string(), "LatinText".to_string()]).clone(),
    (vec!["Wait……what".to_string(), "LatinText".to_string()]).clone(),
    (vec!["中文—下句".to_string(), "CjkPunctuation".to_string()]).clone(),
    (vec!["中文——下句".to_string(), "CjkPunctuation".to_string()]).clone(),
    (vec!["中文—123".to_string(), "CjkPunctuation".to_string()]).clone(),
    (vec!["123—English".to_string(), "LatinText".to_string()]).clone(),
    (vec!["中文……".to_string(), "CjkPunctuation".to_string()]).clone(),
    (vec!["等等…真的".to_string(), "CjkPunctuation".to_string()]).clone(),
    (vec!["等等……真的".to_string(), "CjkPunctuation".to_string()]).clone(),
];
        for x in &cases {
            let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), (x[0usize]).clone().as_str(), None).unwrap();
            if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_role(d.role) != (x[1usize]).clone() || (d.source).to_string() != "DashEllipsisSurroundingScriptContext" {
                let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(format!("{}{}",
            "wrong role/source for ",
            (x[0usize]).clone()
        ).as_str()).unwrap();
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
                let x_1 = "CjkPunctuation".to_string();
                let x_0 = "zh-Hans".to_string();
                let c = FontRoleContext::new(Some((x_0).to_string()), None);
                {
                    {
                        let s = "中文—English".to_string();
                        let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), s.as_str(), Some((c).clone())).unwrap();
                        if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_role(d.role) != x_1 || (d.source).to_string() != "ParagraphLanguageDashEllipsisContext" {
                            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(format!("{}{}",
            "fallback ",
            x_0
        ).as_str()).unwrap();
                        }
                    }
                    {
                        let s = "…".to_string();
                        let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), s.as_str(), Some((c).clone())).unwrap();
                        if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_role(d.role) != x_1 || (d.source).to_string() != "ParagraphLanguageDashEllipsisContext" {
                            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(format!("{}{}",
            "fallback ",
            x_0
        ).as_str()).unwrap();
                        }
                    }
                }
            }
            {
                let x_1 = "LatinText".to_string();
                let x_0 = "en-US".to_string();
                let c = FontRoleContext::new(Some((x_0).to_string()), None);
                {
                    {
                        let s = "中文—English".to_string();
                        let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), s.as_str(), Some((c).clone())).unwrap();
                        if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_role(d.role) != x_1 || (d.source).to_string() != "ParagraphLanguageDashEllipsisContext" {
                            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(format!("{}{}",
            "fallback ",
            x_0
        ).as_str()).unwrap();
                        }
                    }
                    {
                        let s = "…".to_string();
                        let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), s.as_str(), Some((c).clone())).unwrap();
                        if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_role(d.role) != x_1 || (d.source).to_string() != "ParagraphLanguageDashEllipsisContext" {
                            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(format!("{}{}",
            "fallback ",
            x_0
        ).as_str()).unwrap();
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
        if !((ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), &"A—B", None).unwrap().reason).to_string()).starts_with(&"matching-surrounding-script") ||
!((ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), &"中文……", None).unwrap().reason).to_string()).starts_with(&"only-left-strong-script") ||
!((ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one((r).clone(), &"— English", None).unwrap().reason).to_string()).starts_with(&"only-right-strong-script") {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"wrong evidence reason").unwrap();
        }
    });
}

#[test]
fn mandatory_break_stops_context_search() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.mandatoryBreakStopsContextSearch", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.mandatoryBreakStopsContextSearch", || {
        let d = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one(ContextualDashEllipsisRoleResolver::new().unwrap(), &concat!("—\n",
"English"), Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if d.role != FontRole::CjkPunctuation || (d.source).to_string() != "ParagraphLanguageDashEllipsisContext" || !((d.reason).to_string()).starts_with(&"no-strong-script-context") {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"mandatory break did not stop search").unwrap();
        }
    });
}

#[test]
fn linear_context_index_preserves_supplementary_script_evidence() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.linearContextIndexPreservesSupplementaryScriptEvidence", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.linearContextIndexPreservesSupplementaryScriptEvidence", || {
        let a = format!("{}{}",
            TestHelpers::test_helpers_surrogate_text(&vec![55360, 56320]),
            "—123"
        );
        let b = format!("{}{}",
            "123—",
            TestHelpers::test_helpers_surrogate_text(&vec![55297, 56320])
        );
        if ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one(ContextualDashEllipsisRoleResolver::new().unwrap(), a.as_str(), None).unwrap().role != FontRole::CjkPunctuation ||
ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_one(ContextualDashEllipsisRoleResolver::new().unwrap(), b.as_str(), None).unwrap().role != FontRole::LatinText {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"supplementary evidence").unwrap();
        }
    });
}

#[test]
fn resolves_many_neutral_separated_runs_from_one_paragraph_index() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.resolvesManyNeutralSeparatedRunsFromOneParagraphIndex", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.resolvesManyNeutralSeparatedRunsFromOneParagraphIndex", || {
        let mut s = "A".to_string();
        for _ in 0..2048 {
            s += &(" — ");
        }
        s += &("B");
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(s.as_str(), Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2048 || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_role(&d, FontRole::LatinText) {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"many runs").unwrap();
        }
    });
}

#[test]
fn pairs_parenthetical_dashes_across_inserted_content() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.pairsParentheticalDashesAcrossInsertedContent", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.pairsParentheticalDashesAcrossInsertedContent", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(&"他彻夜想Jessica——Jessica是他的前女友——睡不着觉", Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_role(&d, FontRole::CjkPunctuation) ||
!ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_source(&d, &"ParagraphLanguageDashEllipsisContext") || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_reason(&d,
&"parenthetical-pair-conflicting-outer-script") {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(format!("{}{}",
            "parenthetical conflict: ",
            ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_render(&d)
        ).as_str()).unwrap();
        }
    });
}

#[test]
fn matching_outer_script_resolves_the_parenthetical_pair_directly() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.matchingOuterScriptResolvesTheParentheticalPairDirectly", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.matchingOuterScriptResolvesTheParentheticalPairDirectly", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(&"word——and stuff——word", None).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_role(&d, FontRole::LatinText) ||
!ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_source(&d, &"ParentheticalDashPairContext") {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"matching pair").unwrap();
        }
    });
}

#[test]
fn punctuation_between_runs_keeps_them_independent() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.punctuationBetweenRunsKeepsThemIndependent", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.punctuationBetweenRunsKeepsThemIndependent", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(&"地点——北京，时间——明天", Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_role(&d, FontRole::CjkPunctuation) ||
!ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_source(&d, &"DashEllipsisSurroundingScriptContext") {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"punctuation separation").unwrap();
        }
    });
}

#[test]
fn symbol_between_runs_keeps_them_independent() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.symbolBetweenRunsKeepsThemIndependent", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.symbolBetweenRunsKeepsThemIndependent", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(&"时价——$100——很贵", Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || !ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_all_source(&d, &"DashEllipsisSurroundingScriptContext") {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"symbol separation").unwrap();
        }
    });
}

#[test]
fn unequal_run_lengths_do_not_pair() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.unequalRunLengthsDoNotPair", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.unequalRunLengthsDoNotPair", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(&"想Jessica——Jessica是前女友—睡不着", Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || d[0usize].role != FontRole::LatinText || d[1usize].role != FontRole::CjkPunctuation {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"unequal runs").unwrap();
        }
    });
}

#[test]
fn ellipsis_runs_never_pair() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.ellipsisRunsNeverPair", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.ellipsisRunsNeverPair", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(&"想Jessica……Jessica是他的前女友……睡不着", Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || d[0usize].role != FontRole::LatinText || d[1usize].role != FontRole::CjkPunctuation {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"ellipsis pair").unwrap();
        }
    });
}

#[test]
fn mandatory_break_between_runs_keeps_them_independent() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.mandatoryBreakBetweenRunsKeepsThemIndependent", "org.tiqian.layout.ContextualDashEllipsisRoleResolverTest.mandatoryBreakBetweenRunsKeepsThemIndependent", || {
        let d = ContextualDashEllipsisRoleResolver::new().unwrap().resolve(&concat!("想Jessica——Jessica\n",
"是前女友——睡不着"), Some(FontRoleContext::new(Some("zh-Hans".to_string()), None))).unwrap();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 || d[0usize].role != FontRole::LatinText || d[1usize].role != FontRole::CjkPunctuation {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"break pair").unwrap();
        }
    });
}
