use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::kinsoku_mode::KinsokuMode;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::ascii_point_mark_kinsoku_test_support::BreakerChoice;
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
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct UnicodePunctuationBoundaryTestSupport;

impl UnicodePunctuationBoundaryTestSupport {
    pub fn unicode_punctuation_boundary_test_support_breakers() -> Vec<BreakerChoice> {
        return vec![
    (BreakerChoice { label: "greedy".to_string(), breaker: Box::new(GreedyLineBreaker::new(None, None, None, None)) }).clone(),
    (BreakerChoice { label: "lookahead".to_string(), breaker: Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))) }).clone(),
];
    }

    pub fn unicode_punctuation_boundary_test_support_set_ints(values: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(values.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            b.put(&(values[usize::try_from(i).unwrap_or(0)]));
        }
        return b.clone().build();
    }

    pub fn unicode_punctuation_boundary_test_support_mark_closing(i: u32) -> String {
        return (vec![
    ")".to_string(),
    "]".to_string(),
    "}".to_string(),
    ",".to_string(),
    ".".to_string(),
    ":".to_string(),
    ";".to_string(),
    "!".to_string(),
    "?".to_string(),
][usize::try_from(i).unwrap_or(0)]).clone();
    }

    pub fn unicode_punctuation_boundary_test_support_mark_opening(i: u32) -> String {
        return (vec!["(".to_string(), "[".to_string(), "{".to_string()][usize::try_from(i).unwrap_or(0)]).clone();
    }

    pub fn unicode_punctuation_boundary_test_support_layout(text: &str, width: f64, b: Box<dyn LineBreaker>, level: Option<KinsokuLevel>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let l = match &(level) { None => KinsokuLevel::Basic, Some(__option) => *__option };
        let resolver = UnicodePunctuationBoundaryTestResolver::new(l);
        let mut boundaries: Vec<u32> = vec![];
        for i in 0..u32::wrapping_add(u_string::unit_count(&(text)), 1) {
            boundaries.push(i);
        }
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new((resolver).clone())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()),
Some(QuotePairAnalyzer::new()), Some((b).clone()), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(Box::new(NoHyphenator::new())),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some((boundaries).clone()), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width,
Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn unicode_punctuation_boundary_test_support_lines(r: LayoutResult, text: &str) -> Vec<String> {
        let mut a: Vec<String> = vec![];
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            a.push(u_string::substring(&text, i32::from_ne_bytes(((x.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((x.range).clone().end).to_ne_bytes())));
        }
        return a;
    }

    pub fn unicode_punctuation_boundary_test_support_none_starts(lines: &Vec<String>, mark: &str) -> bool {
        for i in 0..match u32::try_from(lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if lines[usize::try_from(i).unwrap_or(0)].clone().starts_with(&mark) {
                return false;
            }
        }
        return true;
    }

    pub fn unicode_punctuation_boundary_test_support_none_ends(lines: &Vec<String>, mark: &str) -> bool {
        for i in 0..match u32::try_from(lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if lines[usize::try_from(i).unwrap_or(0)].clone().ends_with(&mark) {
                return false;
            }
        }
        return true;
    }

    pub fn unicode_punctuation_boundary_test_support_find_reason(r: LayoutResult, text: &str, pos: &str) -> String {
        for i in 0..match u32::try_from((r.debug).clone().contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if x.source_text.to_string() == text && (x.forbidden_position).to_string() == pos {
                return ((x.reason).to_string()).clone();
            }
        }
        return String::new();
    }

    pub fn unicode_punctuation_boundary_test_support_clusters(text: &str, latin: Option<bool>) -> Result<Vec<Cluster>, TextRangeError> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut a: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from(u_string::unit_count(&(text))) { Ok(value) => value, Err(_) => u32::MAX } {
            a.push(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, u_string::substring(&text, { let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }, i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes())).as_str(), if latin.as_ref().map_or(false, |v| v ==
&(true)) { "latin".to_string() } else { "cjk".to_string() }.as_str(), 16 as f64 as f64, Some(u_string::substring(&text, { let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }, i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes()))), Some(0.0), Some(0.0), Some(0.0)));
        }
        return Ok(a);
    }

    pub fn unicode_punctuation_boundary_test_support_roles(text: &str, latin: Option<bool>) -> Vec<FontRole> {
        let mut a: Vec<FontRole> = vec![];
        for _ in 0..match u32::try_from(u_string::unit_count(&(text))) { Ok(value) => value, Err(_) => u32::MAX } {
            a.push(if latin.as_ref().map_or(false, |v| v == &(true)) { FontRole::LatinText } else { FontRole::CjkText });
        }
        return a;
    }
}

#[derive(Clone, PartialEq)]
pub struct UnicodePunctuationBoundaryTestResolver {
    pub(crate) level: KinsokuLevel,
}

impl UnicodePunctuationBoundaryTestResolver {
    pub fn new(level: KinsokuLevel) -> Self {
        Self {
            level,
        }
    }

    pub fn resolve(&self, __: LayoutProfileId) -> ClreqProfile {
        let b = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((b.id).to_string().as_str(), b.strictness, b.region, Some(b.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()),
Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(b.glue_placement), (b.adjustment).clone(), KinsokuMode::Fixed { level: self.level, hanging: HangingPunctuationStyle::Disabled }, (b.punctuation_width).clone());
    }
}

impl ClreqProfileResolver for UnicodePunctuationBoundaryTestResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.UnicodePunctuationBoundaryTestSupport.UnicodePunctuationBoundaryTestResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, __: LayoutProfileId) -> ClreqProfile {
        let b = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((b.id).to_string().as_str(), b.strictness, b.region, Some(b.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()),
Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(b.glue_placement), (b.adjustment).clone(), KinsokuMode::Fixed { level: self.level, hanging: HangingPunctuationStyle::Disabled }, (b.punctuation_width).clone());
    }
}
