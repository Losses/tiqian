use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::spacing_decision_info::SpacingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::ascii_point_mark_kinsoku_test_support::BreakerChoice;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::paragraph_dp_line_breaker::ParagraphDpLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::unicode_punctuation_boundary_resolver::AttachedInlineVirtualBoundary;
use crate::org::tiqian::layout::unicode_punctuation_boundary_resolver::UnicodePunctuationBoundaryResolver;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct AttachedInlineVirtualAdjacencyTestSupport;

impl AttachedInlineVirtualAdjacencyTestSupport {
    pub fn attached_inline_virtual_adjacency_test_support_resolve(a: &Vec<InlineAttachment>) -> Vec<AttachedInlineVirtualBoundary> {
        return UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_virtual_boundaries(&a);
    }

    pub fn attached_inline_virtual_adjacency_test_support_layout_attached_reference(text: &UStr) -> Result<LayoutResult, AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault> {
        let idx = u_string::find_from(&(text), UString::from("[1]").as_ustr(), 0);
        let spans = vec![
    (TextSpan::new(TextRange::new(u32::from_ne_bytes(((idx) as u32).to_ne_bytes()), u32::from_ne_bytes(((i32::wrapping_add(idx, 3)) as u32).to_ne_bytes())).map_err(|e| AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::TextRangeErrorFault(e))?, TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
];
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).map_err(|e| AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::ParagraphLayoutEngineNewFaultFault(e))?.layout(LayoutInput::new(TiqianTextContent::new(text, Some((spans).clone()), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(e))?);
    }

    pub fn attached_inline_virtual_adjacency_test_support_layout_with_breaker(text: &UStr, breaker: Box<dyn LineBreaker>) -> Result<LayoutResult, AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault> {
        let spans = vec![
    (TextSpan::new(TextRange::new(2u32, 3u32).map_err(|e| AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::TextRangeErrorFault(e))?, TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
];
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some((breaker).clone()), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).map_err(|e| AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::ParagraphLayoutEngineNewFaultFault(e))?;
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(text, Some((spans).clone()), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(32.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(e))?);
    }

    pub fn attached_inline_virtual_adjacency_test_support_breakers() -> Result<Vec<BreakerChoice>, TextRangeError> {
        return Ok(vec![
    (BreakerChoice { label: UString::from("greedy").to_ustring(), breaker: Box::new(GreedyLineBreaker::new(None, None, None, None)) }).clone(),
    (BreakerChoice { label: UString::from("lookahead").to_ustring(), breaker: Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))) }).clone(),
    (BreakerChoice { label: UString::from("paragraph-dp").to_ustring(), breaker: Box::new(ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64))?) }).clone(),
]);
    }

    pub fn attached_inline_virtual_adjacency_test_support_virtual_boundary(r: LayoutResult) -> SpacingDecisionInfo {
        let mut hit: Option<SpacingDecisionInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().spacing_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().spacing_decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_ustring().starts_with(&UString::from("AttachedInlineVirtualPunctuationBoundary")) {
                hit = Some(((r.debug).clone().spacing_decisions[usize::try_from(i).unwrap_or(0)]).clone());
                break;
            }
        }
        return hit.as_ref().unwrap().clone();
    }

    pub fn attached_inline_virtual_adjacency_test_support_render_lines(a: &[LineBox]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", (a[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn attached_inline_virtual_adjacency_test_support_render_ranges(a: &[LineBox]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", ((a[usize::try_from(i).unwrap_or(0)]).clone().range).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined1 = parts; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }
}
