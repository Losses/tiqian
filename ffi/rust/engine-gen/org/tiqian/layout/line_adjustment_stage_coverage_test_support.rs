use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::justification_decision_info::JustificationDecisionInfo;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::line_edge_trim_decision_info::LineEdgeTrimDecisionInfo;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError;
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
use crate::org::tiqian::linebreak::english_hyphenation::EnglishHyphenation;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestSupportEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for LineAdjustmentStageCoverageTestSupportEngineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentStageCoverageTestSupportEngineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageCoverageTestSupportEngineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestSupportEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestSupportEngineFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestSupportEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestSupportEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestSupportEngineFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestSupportEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestSupportEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestSupportEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestSupportEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestSupportEngineFault::UStringFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct LineAdjustmentStageCoverageTestSupport;

impl LineAdjustmentStageCoverageTestSupport {
    pub fn line_adjustment_stage_coverage_test_support_engine(shaper: Option<Arc<Mutex<dyn ITextShaper>>>, hyphenate: bool) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        match &(shaper) {
            Some(__option) => {
                return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some((*__option).clone()), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
            }
            None => {
            }
        }
        if hyphenate {
            return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(EnglishHyphenation::english_hyphenation_en_us().map_err(|e| ParagraphLayoutEngineNewFault::UStringFaultFault(e))?), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
        }
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn line_adjustment_stage_coverage_test_support_layout(text: &UStr, width: f64, spans: Option<Vec<TextSpan>>, inline_objects: Option<Vec<InlineObjectSpan>>, line_break_spans: Option<Vec<LineBreakSpan>>, hyphenate: Option<bool>, shaper: Option<Arc<Mutex<dyn ITextShaper>>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let resolved_spans = match &(spans) { None => vec![], Some(__option1) => (*__option1).clone() };
        let resolved_objects = match &(inline_objects) { None => vec![], Some(__option2) => (*__option2).clone() };
        let resolved_breaks = match &(line_break_spans) { None => vec![], Some(__option3) => (*__option3).clone() };
        let mut e = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_engine((shaper).clone(), hyphenate.as_ref().map_or(false, |v| v == &(true)))?;
        return Ok(e.layout(LayoutInput::new(TiqianTextContent::new(text, Some((resolved_spans).clone()), Some(vec![]), Some((resolved_breaks).clone()), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some(Ic(0.0f64)), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some((resolved_objects).clone()))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn line_adjustment_stage_coverage_test_support_render_lines(a: &[LineBox]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", (a[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn line_adjustment_stage_coverage_test_support_render_trims(a: &[LineEdgeTrimDecisionInfo]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", (a[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined1 = parts; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn line_adjustment_stage_coverage_test_support_render_justifications(a: &[JustificationDecisionInfo]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", (a[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined2 = parts; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn line_adjustment_stage_coverage_test_support_render_glyphs(a: &[Glyph]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", (a[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined3 = parts; let mut out = String::new(); let n = joined3.len(); let mut index3 = 0usize; while index3 < n { if index3 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined3[index3]); index3 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn line_adjustment_stage_coverage_test_support_render_strings(a: &[UString]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push((a[usize::try_from(i).unwrap_or(0)]).clone());
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined4 = parts; let mut out = String::new(); let n = joined4.len(); let mut index4 = 0usize; while index4 < n { if index4 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined4[index4]); index4 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn line_adjustment_stage_coverage_test_support_render_floats(a: &Vec<f64>) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(a[usize::try_from(i).unwrap_or(0)])).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined5 = parts; let mut out = String::new(); let n = joined5.len(); let mut index5 = 0usize; while index5 < n { if index5 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined5[index5]); index5 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn line_adjustment_stage_coverage_test_support_render_cluster_text_advance(a: &[Cluster]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += ((a[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring().as_ustr(); __s += &(UString::from("@")); __s += UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(a[usize::try_from(i).unwrap_or(0)].advance)).as_str()).as_ustr(); __s }).as_str()));
        }
        return UString::from(format!("{}", { let joined6 = parts; let mut out = String::new(); let n = joined6.len(); let mut index6 = 0usize; while index6 < n { if index6 > 0 { out.push_str(","); } let _ = write!(out, "{}", joined6[index6]); index6 += 1; } UString::from(out.as_str()) }).as_str());
    }

    pub fn line_adjustment_stage_coverage_test_support_render_advances(a: &[Cluster]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(a[usize::try_from(i).unwrap_or(0)].advance)).as_str()));
        }
        return UString::from(format!("{}", { let joined7 = parts; let mut out = String::new(); let n = joined7.len(); let mut index7 = 0usize; while index7 < n { if index7 > 0 { out.push_str(","); } let _ = write!(out, "{}", joined7[index7]); index7 += 1; } UString::from(out.as_str()) }).as_str());
    }

    pub fn line_adjustment_stage_coverage_test_support_render_cluster_texts(a: &[Cluster]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(((a[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring());
        }
        return UString::from(format!("{}", { let joined8 = parts; let mut out = String::new(); let n = joined8.len(); let mut index8 = 0usize; while index8 < n { if index8 > 0 { out.push_str(","); } let _ = write!(out, "{}", joined8[index8]); index8 += 1; } UString::from(out.as_str()) }).as_str());
    }

    pub fn line_adjustment_stage_coverage_test_support_line_range_widths(a: &[LineBox]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += UString::from(format!("{}", ((a[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(":")); __s += UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(a[usize::try_from(i).unwrap_or(0)].natural_width)).as_str()).as_ustr(); __s += &(UString::from("/")); __s += UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(a[usize::try_from(i).unwrap_or(0)].adjusted_width)).as_str()).as_ustr(); __s }).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined9 = parts; let mut out = String::new(); let n = joined9.len(); let mut index9 = 0usize; while index9 < n { if index9 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined9[index9]); index9 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn line_adjustment_stage_coverage_test_support_cluster_by_text(r: LayoutResult, text: &UStr) -> Result<Cluster, NoSuchElementError> {
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == text {
                return Ok((r.clusters[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return Err(NoSuchElementError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("missing cluster ")); __s += text; __s }).as_str()) });
    }

    pub fn line_adjustment_stage_coverage_test_support_trim_by_reason(r: LayoutResult, reason: &UStr) -> Result<LineEdgeTrimDecisionInfo, NoSuchElementError> {
        for i in 0..match u32::try_from((r.debug).clone().line_edge_trim_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().line_edge_trim_decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_ustring() == reason {
                return Ok(((r.debug).clone().line_edge_trim_decisions[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return Err(NoSuchElementError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("missing trim ")); __s += reason; __s }).as_str()) });
    }

    pub fn line_adjustment_stage_coverage_test_support_trims_by_reason(r: LayoutResult, reason: &UStr) -> Vec<LineEdgeTrimDecisionInfo> {
        let mut out: Vec<LineEdgeTrimDecisionInfo> = vec![];
        for i in 0..match u32::try_from((r.debug).clone().line_edge_trim_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().line_edge_trim_decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_ustring() == reason {
                out.push(((r.debug).clone().line_edge_trim_decisions[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return out;
    }

    pub fn line_adjustment_stage_coverage_test_support_allocation_deltas(r: LayoutResult, kind: &UStr) -> Vec<f64> {
        let mut out: Vec<f64> = vec![];
        for i in 0..match u32::try_from((r.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let allocs = (((r.debug).clone().justification_decisions[usize::try_from(i).unwrap_or(0)]).clone().allocations).clone();
            for j in 0..match u32::try_from(allocs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if allocs[usize::try_from(j).unwrap_or(0)].clone().kind.to_ustring() == kind {
                    out.push(allocs[usize::try_from(j).unwrap_or(0)].delta);
                }
            }
        }
        return out;
    }

    pub fn line_adjustment_stage_coverage_test_support_emergency_reasons(r: LayoutResult) -> Vec<UString> {
        let mut out: Vec<UString> = vec![];
        for i in 0..match u32::try_from((r.debug).clone().emergency_tracking_eligibility_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            out.push((((r.debug).clone().emergency_tracking_eligibility_decisions[usize::try_from(i).unwrap_or(0)]).clone().reason).to_ustring());
        }
        return out;
    }

    pub fn line_adjustment_stage_coverage_test_support_break_reasons(r: LayoutResult) -> Vec<UString> {
        let mut out: Vec<UString> = vec![];
        for i in 0..match u32::try_from((r.debug).clone().break_opportunity_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            out.push((((r.debug).clone().break_opportunity_decisions[usize::try_from(i).unwrap_or(0)]).clone().reason).to_ustring());
        }
        return out;
    }

    pub fn line_adjustment_stage_coverage_test_support_is_all_spaces(s: &UStr) -> bool {
    let __units = u_string::units(&s);
    let __count = u_string::unit_count(&s);
        if __count == 0 {
            return false;
        }
        for i in 0..match u32::try_from(u_string::unit_count(&(s))) { Ok(value) => value, Err(_) => u32::MAX } {
            if u_string::char_at_from(&__units, i) != UString::from(" ") {
                return false;
            }
        }
        return true;
    }

    pub fn line_adjustment_stage_coverage_test_support_text_length(s: &UStr) -> u32 {
        return u_string::unit_count(&(s));
    }
}

#[derive(Clone)]
pub struct ZeroSpaceShaper {
    pub(crate) delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl ZeroSpaceShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let shaped = self.delegate.lock().unwrap().shape((input).clone())?;
        let mut clusters: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from(shaped.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (shaped.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            clusters.push(if LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_is_all_spaces((c.text).to_ustring().as_ustr()) { Cluster::new((c.range).clone(), (c.text).to_ustring().as_ustr(), (c.font_key).to_ustring().as_ustr(), 0.0f64, Some((c.display_text).to_ustring()), Some(c.baseline_shift), Some(c.leading_layout_advance), Some(c.glyph_inline_shift)) } else { c });
        }
        return Ok(ShapingResult::new(clusters.to_vec(), shaped.glyph_runs.to_vec(), Some((shaped.decisions).clone())));
    }
}

impl ITextShaper for ZeroSpaceShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.LineAdjustmentStageCoverageTestSupport.ZeroSpaceShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let shaped = self.delegate.lock().unwrap().shape((input).clone())?;
        let mut clusters: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from(shaped.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (shaped.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            clusters.push(if LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_is_all_spaces((c.text).to_ustring().as_ustr()) { Cluster::new((c.range).clone(), (c.text).to_ustring().as_ustr(), (c.font_key).to_ustring().as_ustr(), 0.0f64, Some((c.display_text).to_ustring()), Some(c.baseline_shift), Some(c.leading_layout_advance), Some(c.glyph_inline_shift)) } else { c });
        }
        return Ok(ShapingResult::new(clusters.to_vec(), shaped.glyph_runs.to_vec(), Some((shaped.decisions).clone())));
    }
}
