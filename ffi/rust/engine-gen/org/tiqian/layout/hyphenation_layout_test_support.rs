use crate::org::tiqian::clreq::adjustment_style_policy::AdjustmentStylePolicy;
use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::clreq::line_adjustment_strategy::LineAdjustmentStrategy;
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
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::linebreak::hyphenator::Hyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::runtime::u_string;
use std::fmt::Write;


#[derive(Clone, Copy)]
pub struct HyphenationLayoutTestSupport;

impl HyphenationLayoutTestSupport {
    pub const HYPHENATION_LAYOUT_TEST_SUPPORT_TEXT: &str = "中文internationalization中文";

    pub fn hyphenation_layout_test_support_layout_with(h: Box<dyn Hyphenator>, content: &str, width: f64) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let text = (content.to_string()).clone();
        let w = width;
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some((h).clone()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?.layout(LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false),
Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(w, Some(f64::INFINITY),
Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e|
ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn hyphenation_layout_test_support_push_out_resolver() -> Box<dyn ClreqProfileResolver> {
        return Box::new(PushOutResolver::new());
    }

    pub fn hyphenation_layout_test_support_is_latin(s: &str) -> bool {
    let __units = u_string::units(&s);
    let __count = u_string::unit_count(&s);
        for i in 0..match u32::try_from(u_string::unit_count(&(s))) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = u_string::unit_at_from(&__units, i).unwrap_or(0);
            if !((i32::from_ne_bytes((c).to_ne_bytes())) >= 97 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 122 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 65 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 90) {
                return false;
            }
        }
        return true;
    }

    pub fn hyphenation_layout_test_support_rebuild(word: &str, points: &[u32]) -> String {
    let __units1 = u_string::units(&word);
    let __count1 = u_string::unit_count(&word);
        let mut parts: Vec<String> = vec![];
        let mut prev = 0u32;
        for &p in points {
            parts.push(u_string::substring(&word, i32::from_ne_bytes((prev).to_ne_bytes()), { let v: u32 = p; i32::from_ne_bytes(v.to_ne_bytes()) }));
            prev = p;
        }
        parts.push(u_string::substring_from(&word, i32::from_ne_bytes((prev).to_ne_bytes())));
        return { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(&("-")); } let _ = write!(out, "{}", joined[index]); index += 1; } out };
    }
}

#[derive(Clone, PartialEq)]
pub struct PushOutResolver {
}

impl PushOutResolver {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        let p = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((p.id).to_string().as_str(), p.strictness, p.region, Some(p.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((p.auto_space).clone()),
Some(p.glue_placement), AdjustmentStylePolicy::new(Some((p.adjustment).clone().line_end_punctuation), Some((p.adjustment).clone().allow_inline_stop_compression), Some((p.adjustment).clone().allow_sino_western_gap_adjustment), Some(LineAdjustmentStrategy::PushOutOnly)),
(p.kinsoku_mode).clone(), (p.punctuation_width).clone());
    }
}

impl ClreqProfileResolver for PushOutResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.HyphenationLayoutTestSupport.PushOutResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        let p = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((p.id).to_string().as_str(), p.strictness, p.region, Some(p.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((p.auto_space).clone()),
Some(p.glue_placement), AdjustmentStylePolicy::new(Some((p.adjustment).clone().line_end_punctuation), Some((p.adjustment).clone().allow_inline_stop_compression), Some((p.adjustment).clone().allow_sino_western_gap_adjustment), Some(LineAdjustmentStrategy::PushOutOnly)),
(p.kinsoku_mode).clone(), (p.punctuation_width).clone());
    }
}
