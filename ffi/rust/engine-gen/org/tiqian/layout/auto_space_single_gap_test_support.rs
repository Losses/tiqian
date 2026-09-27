use crate::org::tiqian::clreq::auto_space_mode::AutoSpaceMode;
use crate::org::tiqian::clreq::auto_space_policy::AutoSpacePolicy;
use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::core::auto_space_decision_info::AutoSpaceDecisionInfo;
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
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestSupportLetterDigitFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for AutoSpaceSingleGapTestSupportLetterDigitFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestSupportLetterDigitFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestSupportLetterDigitFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AutoSpaceSingleGapTestSupportLetterDigitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestSupportLetterDigitFault) -> Self {
        match value {
            AutoSpaceSingleGapTestSupportLetterDigitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestSupportLetterDigitFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestSupportLetterDigitFault) -> Self {
        match value {
            AutoSpaceSingleGapTestSupportLetterDigitFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestSupportLetterDigitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestSupportLetterDigitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestSupportLetterDigitFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestSupportLetterDigitFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSpaceSingleGapTestSupportLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for AutoSpaceSingleGapTestSupportLayoutFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoSpaceSingleGapTestSupportLayoutFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AutoSpaceSingleGapTestSupportLayoutFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AutoSpaceSingleGapTestSupportLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AutoSpaceSingleGapTestSupportLayoutFault) -> Self {
        match value {
            AutoSpaceSingleGapTestSupportLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AutoSpaceSingleGapTestSupportLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AutoSpaceSingleGapTestSupportLayoutFault) -> Self {
        match value {
            AutoSpaceSingleGapTestSupportLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AutoSpaceSingleGapTestSupportLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AutoSpaceSingleGapTestSupportLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AutoSpaceSingleGapTestSupportLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AutoSpaceSingleGapTestSupportLayoutFault::UStringFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct AutoSpaceSingleGapTestSupport;

impl AutoSpaceSingleGapTestSupport {
    pub fn auto_space_single_gap_test_support_render_auto_space_decisions(list: &[AutoSpaceDecisionInfo]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for d in list {
            parts.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("AutoSpaceDecisionInfo(clusterRange=TextRange(start=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.cluster_range).clone().start)).as_str())); __s += &(UString::from(", end=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.cluster_range).clone().end)).as_str())); __s += &(UString::from("), side=")); __s += (d.side).to_ustring().as_ustr(); __s += &(UString::from(", boundaryRole=")); __s += (d.boundary_role).to_ustring().as_ustr(); __s += &(UString::from(", mode=")); __s += (d.mode).to_ustring().as_ustr(); __s += &(UString::from(", charactersAffected=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(d.characters_affected)).as_str())); __s += &(UString::from(", reductionPerChar=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(d.reduction_per_char)); __s += &(UString::from(", totalReduction=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(d.total_reduction)); __s += &(UString::from(", reason=")); __s += (d.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn auto_space_single_gap_test_support_layout(text: &UStr, spans: &Vec<TextSpan>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?.layout(LayoutInput::new(TiqianTextContent::new(text, Some((spans).clone()), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn auto_space_single_gap_test_support_letter_digit() -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(AutoSpaceLetterDigitResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?;
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[30002,65,20057,57,19993])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn auto_space_single_gap_test_support_same_range(first: TextRange, second: TextRange) -> bool {
        return first.start == second.start && first.end == second.end;
    }

    pub fn auto_space_single_gap_test_support_clusters_with_text(result: LayoutResult, text: &UStr) -> Vec<Cluster> {
        let mut matches: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == text {
                matches.push((result.clusters[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return matches;
    }
}

#[derive(Clone, PartialEq)]
pub struct AutoSpaceLetterDigitResolver {
}

impl AutoSpaceLetterDigitResolver {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        let base = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((base.id).to_ustring().as_ustr(), base.strictness, base.region, Some(base.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some(AutoSpacePolicy::new(Some(AutoSpaceMode::Insert), Some(AutoSpaceMode::Disabled), Some(0.125), Some(1.0 / 3.0))), Some(base.glue_placement), (base.adjustment).clone(), (base.kinsoku_mode).clone(), (base.punctuation_width).clone());
    }
}

impl ClreqProfileResolver for AutoSpaceLetterDigitResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.AutoSpaceSingleGapTestSupport.AutoSpaceLetterDigitResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        let base = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((base.id).to_ustring().as_ustr(), base.strictness, base.region, Some(base.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some(AutoSpacePolicy::new(Some(AutoSpaceMode::Insert), Some(AutoSpaceMode::Disabled), Some(0.125), Some(1.0 / 3.0))), Some(base.glue_placement), (base.adjustment).clone(), (base.kinsoku_mode).clone(), (base.punctuation_width).clone());
    }
}
