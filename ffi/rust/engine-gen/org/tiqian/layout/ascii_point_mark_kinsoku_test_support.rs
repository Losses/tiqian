use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::kinsoku_mode::KinsokuMode;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::contextual_kinsoku_decision_info::ContextualKinsokuDecisionInfo;
use crate::org::tiqian::core::font_decision_info::FontDecisionInfo;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::line_decision_info::LineDecisionInfo;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::runtime::u_string;
use std::fmt::Write;


#[derive(Clone, Copy)]
pub struct AsciiPointMarkKinsokuTestSupport;

impl AsciiPointMarkKinsokuTestSupport {
    pub fn ascii_point_mark_kinsoku_test_support_render_strings(a: &[String]) -> String {
        let mut x: Vec<String> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            x.push((a[usize::try_from(i).unwrap_or(0)]).clone());
        }
        return format!("{}{}{}",
            "[",
            { let joined = x; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined[index]); index += 1; } out },
            "]"
        );
    }

    pub fn ascii_point_mark_kinsoku_test_support_join_lines(a: &Vec<String>) -> String {
        return { let joined1 = a; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(&(concat!("\n",
""))); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } out };
    }

    pub fn ascii_point_mark_kinsoku_test_support_render_clusters(a: &[Cluster]) -> String {
        let mut parts: Vec<String> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push((a[usize::try_from(i).unwrap_or(0)]).clone().to_string());
        }
        return format!("{}{}{}",
            "[",
            { let joined2 = parts; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } out },
            "]"
        );
    }

    pub fn ascii_point_mark_kinsoku_test_support_render_fonts(a: &[FontDecisionInfo]) -> String {
        let mut parts: Vec<String> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push((a[usize::try_from(i).unwrap_or(0)]).clone().to_string());
        }
        return format!("{}{}{}",
            "[",
            { let joined3 = parts; let mut out = String::new(); let n = joined3.len(); let mut index3 = 0usize; while index3 < n { if index3 > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined3[index3]); index3 += 1; } out },
            "]"
        );
    }

    pub fn ascii_point_mark_kinsoku_test_support_render_contextual(a: &[ContextualKinsokuDecisionInfo]) -> String {
        let mut parts: Vec<String> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push((a[usize::try_from(i).unwrap_or(0)]).clone().to_string());
        }
        return format!("{}{}{}",
            "[",
            { let joined4 = parts; let mut out = String::new(); let n = joined4.len(); let mut index4 = 0usize; while index4 < n { if index4 > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined4[index4]); index4 += 1; } out },
            "]"
        );
    }

    pub fn ascii_point_mark_kinsoku_test_support_render_lines(a: &[LineBox]) -> String {
        let mut parts: Vec<String> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push((a[usize::try_from(i).unwrap_or(0)]).clone().to_string());
        }
        return format!("{}{}{}",
            "[",
            { let joined5 = parts; let mut out = String::new(); let n = joined5.len(); let mut index5 = 0usize; while index5 < n { if index5 > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined5[index5]); index5 += 1; } out },
            "]"
        );
    }

    pub fn ascii_point_mark_kinsoku_test_support_render_line_decisions(a: &[LineDecisionInfo]) -> String {
        let mut parts: Vec<String> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push((a[usize::try_from(i).unwrap_or(0)]).clone().to_string());
        }
        return format!("{}{}{}",
            "[",
            { let joined6 = parts; let mut out = String::new(); let n = joined6.len(); let mut index6 = 0usize; while index6 < n { if index6 > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined6[index6]); index6 += 1; } out },
            "]"
        );
    }

    pub fn ascii_point_mark_kinsoku_test_support_has_cluster(r: LayoutResult, s: &str) -> bool {
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == s {
                return true;
            }
        }
        return false;
    }

    pub fn ascii_point_mark_kinsoku_test_support_clusters_with_text(r: LayoutResult, s: &str) -> Vec<Cluster> {
        let mut x: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == s {
                x.push((r.clusters[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return x;
    }

    pub fn ascii_point_mark_kinsoku_test_support_same_range(first: TextRange, second: TextRange) -> bool {
        return first.start == second.start && first.end == second.end;
    }

    pub fn ascii_point_mark_kinsoku_test_support_font_decision(r: LayoutResult, range: TextRange) -> Option<FontDecisionInfo> {
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_same_range((((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone().range).clone(), (range).clone()) {
                return Some(((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return None;
    }

    pub fn ascii_point_mark_kinsoku_test_support_has_font_source(r: LayoutResult, s: &str) -> bool {
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().font_decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_string() == s {
                return true;
            }
        }
        return false;
    }

    pub fn ascii_point_mark_kinsoku_test_support_font_decision_by_text(r: LayoutResult, s: &str) -> Option<FontDecisionInfo> {
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().font_decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_string() == s {
                return Some(((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return None;
    }

    pub fn ascii_point_mark_kinsoku_test_support_has_punctuation(r: LayoutResult, range: TextRange) -> bool {
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_same_range((((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone().range).clone(), (range).clone()) {
                return true;
            }
        }
        return false;
    }

    pub fn ascii_point_mark_kinsoku_test_support_contextual(r: LayoutResult, range: Option<TextRange>) -> Option<ContextualKinsokuDecisionInfo> {
        for i in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if match &(range) { None => true, Some(__option) => AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_same_range((((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)]).clone().range).clone(),
((*__option).clone()).clone()) } {
                return Some(((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return None;
    }

    pub fn ascii_point_mark_kinsoku_test_support_contextual_by_text(r: LayoutResult, s: &str) -> Option<ContextualKinsokuDecisionInfo> {
        for i in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_string() == s {
                return Some(((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return None;
    }

    pub fn ascii_point_mark_kinsoku_test_support_has_repair(r: LayoutResult, s: &str) -> bool {
        for i in 0..match u32::try_from((r.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().line_decisions[usize::try_from(i).unwrap_or(0)].clone().repair.as_ref().map_or(false, |v| v == &(s.to_string())) {
                return true;
            }
        }
        return false;
    }

    pub fn ascii_point_mark_kinsoku_test_support_forbidden_for(r: LayoutResult, s: &str) -> Vec<String> {
        let mut x: Vec<String> = vec![];
        for i in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_string() == s {
                x.push((((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)]).clone().forbidden_position).to_string());
            }
        }
        return x;
    }

    pub fn ascii_point_mark_kinsoku_test_support_null_fallback(r: LayoutResult) -> String {
        let d = AsciiPointMarkKinsokuTestSupport::ascii_point_mark_kinsoku_test_support_contextual((r).clone(), None);
        if d.is_none() {
            return String::new();
        }
        let v = ((d).as_ref().unwrap().impossible_measure_fallback).clone().clone();
        return match &(v) { None => "".to_string(), Some(__option1) => __option1.to_string() };
    }

    pub fn ascii_point_mark_kinsoku_test_support_layout(text: &str, max_width: f64, breaker: Box<dyn LineBreaker>, level: Option<KinsokuLevel>, hanging: Option<HangingPunctuationStyle>, first_line_indent: Option<Ic>, ruby_spans: Option<Vec<RubySpan>>, spans: Option<Vec<TextSpan>>,
line_length_grid: Option<LineLengthGrid>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let resolved_level = match &(level) { None => KinsokuLevel::Basic, Some(__option2) => *__option2 };
        let resolved_hanging = match &(hanging) { None => HangingPunctuationStyle::Disabled, Some(__option3) => *__option3 };
        let resolved_indent = match &(first_line_indent) { None => Ic::zero(), Some(__option4) => (*__option4).clone() };
        let resolved_grid = match &(line_length_grid) { None => LineLengthGrid::new(Some(true), None), Some(__option5) => (*__option5).clone() };
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new(AsciiKinsokuFixedResolver::new(resolved_level, resolved_hanging))), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()),
Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some((breaker).clone()), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(Box::new(NoHyphenator::new())),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?;
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(text, (spans).clone(), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((resolved_indent).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some((resolved_grid).clone()),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647)).map_err(|e|
ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), (ruby_spans).clone(), Some(vec![]), Some(vec![]))).map_err(|e|
ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn ascii_point_mark_kinsoku_test_support_layout_without_explicit_indent(text: &str, max_width: f64, breaker: Box<dyn LineBreaker>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new(AsciiKinsokuFixedResolver::new(KinsokuLevel::Basic, HangingPunctuationStyle::Disabled))), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()),
Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some((breaker).clone()), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(Box::new(NoHyphenator::new())),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?;
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647)).map_err(|e|
ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e|
ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn ascii_point_mark_kinsoku_test_support_line_texts(result: LayoutResult, source: &str) -> Vec<String> {
        let mut texts: Vec<String> = vec![];
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line = (result.lines[usize::try_from(i).unwrap_or(0)]).clone();
            texts.push(u_string::substring(&source, i32::from_ne_bytes(((line.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((line.range).clone().end).to_ne_bytes())));
        }
        return texts;
    }

    pub fn ascii_point_mark_kinsoku_test_support_breakers() -> Vec<BreakerChoice> {
        let choices = vec![
    (BreakerChoice { label: "greedy".to_string(), breaker: Box::new(GreedyLineBreaker::new(None, None, None, None)) }).clone(),
    (BreakerChoice { label: "lookahead".to_string(), breaker: Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))) }).clone(),
];
        return choices;
    }
}

#[derive(Clone, PartialEq)]
pub struct AsciiKinsokuFixedResolver {
    pub(crate) level: KinsokuLevel,
    pub(crate) hanging: HangingPunctuationStyle,
}

impl AsciiKinsokuFixedResolver {
    pub fn new(level: KinsokuLevel, hanging: HangingPunctuationStyle) -> Self {
        Self {
            level,
            hanging,
        }
    }

    pub fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        let base = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((base.id).to_string().as_str(), base.strictness, base.region, Some(base.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()),
Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(base.glue_placement), (base.adjustment).clone(), KinsokuMode::Fixed { level: self.level, hanging: self.hanging }, (base.punctuation_width).clone());
    }
}

impl ClreqProfileResolver for AsciiKinsokuFixedResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.AsciiPointMarkKinsokuTestSupport.AsciiKinsokuFixedResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        let base = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((base.id).to_string().as_str(), base.strictness, base.region, Some(base.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()),
Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(base.glue_placement), (base.adjustment).clone(), KinsokuMode::Fixed { level: self.level, hanging: self.hanging }, (base.punctuation_width).clone());
    }
}

#[derive(Debug, Clone)]
pub struct BreakerChoice {
    pub label: String,
    pub breaker: Box<dyn LineBreaker>,
}
