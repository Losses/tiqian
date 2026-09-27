use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::kinsoku_mode::KinsokuMode;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_decision_info::InlineObjectDecisionInfo;
use crate::org::tiqian::core::inline_object_line_height_decision_info::InlineObjectLineHeightDecisionInfo;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_edge_trim_decision_info::LineEdgeTrimDecisionInfo;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::line_repair_allocation_info::LineRepairAllocationInfo;
use crate::org::tiqian::core::line_repair_decision_info::LineRepairDecisionInfo;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::positioned_cluster::PositionedCluster;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault) -> Self {
        match value {
            InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault) -> Self {
        match value {
            InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestSupportFixedBasicKinsokuEngineFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineObjectLayoutTestSupportFixedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for InlineObjectLayoutTestSupportFixedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineObjectLayoutTestSupportFixedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineObjectLayoutTestSupportFixedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InlineObjectLayoutTestSupportFixedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineObjectLayoutTestSupportFixedFault) -> Self {
        match value {
            InlineObjectLayoutTestSupportFixedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineObjectLayoutTestSupportFixedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineObjectLayoutTestSupportFixedFault) -> Self {
        match value {
            InlineObjectLayoutTestSupportFixedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineObjectLayoutTestSupportFixedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineObjectLayoutTestSupportFixedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineObjectLayoutTestSupportFixedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineObjectLayoutTestSupportFixedFault::UStringFaultFault(value)
    }
}

pub static INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE: LazyLock<ParagraphStyle> = LazyLock::new(|| ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), Some(24 as f64), Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM)));

#[derive(Clone, Copy)]
pub struct InlineObjectLayoutTestSupport;

impl InlineObjectLayoutTestSupport {

    pub fn inline_object_layout_test_support_render_nullable_decision(v: Option<InlineObjectLineHeightDecisionInfo>) -> Result<UString, UStringFault> {
        return Ok(match &(v) { None => UString::from("null"), Some(__option) => InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_decision(((*__option).clone()).clone())?.to_ustring() });
    }

    pub fn inline_object_layout_test_support_render_decision(v: InlineObjectLineHeightDecisionInfo) -> Result<UString, UStringFault> {
        return Ok(TestTraceRender::test_trace_render_cap(UString::from(format!("{}", v.to_string()).as_str()).as_ustr())?);
    }

    pub fn inline_object_layout_test_support_render_nullable_repair(v: Option<LineRepairDecisionInfo>) -> Result<UString, UStringFault> {
        return Ok(match &(v) { None => UString::from("null"), Some(__option1) => InlineObjectLayoutTestSupport::inline_object_layout_test_support_render_repair(((*__option1).clone()).clone())?.to_ustring() });
    }

    pub fn inline_object_layout_test_support_render_repair(v: LineRepairDecisionInfo) -> Result<UString, UStringFault> {
        return Ok(TestTraceRender::test_trace_render_cap(UString::from(format!("{}", v.to_string()).as_str()).as_ustr())?);
    }

    pub fn inline_object_layout_test_support_rec(n: &UStr) -> TestTraceRecorder {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[73,110,108,105,110,101,79,98,106,101,99,116,76,97,121,111,117,116,84,101,115,116])));
        t.section(n);
        return t;
    }

    pub fn inline_object_layout_test_support_resolver(p: ClreqProfile) -> Box<dyn ClreqProfileResolver> {
        return Box::new(Resolver::new((p).clone()));
    }

    pub fn inline_object_layout_test_support_fixed(b: Option<Box<dyn LineBreaker>>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(InlineObjectLayoutTestSupport::inline_object_layout_test_support_resolver((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), match &(b) { None => Some(Box::new(GreedyLineBreaker::new(None, None, None, None)) as Box<dyn LineBreaker>), Some(__option9) => Some((*__option9).clone()) }, Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn inline_object_layout_test_support_layout(text: &UStr, width: f64, objects: Option<Vec<InlineObjectSpan>>, b: Option<Box<dyn LineBreaker>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(InlineObjectLayoutTestSupport::inline_object_layout_test_support_fixed((b).clone())?.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*INLINE_OBJECT_LAYOUT_TEST_SUPPORT_STYLE).clone()), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), (objects).clone())).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn inline_object_layout_test_support_fixed_basic_kinsoku_engine(b: Option<Box<dyn LineBreaker>>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        let base = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        let p = ClreqProfile::new((base.id).to_ustring().as_ustr(), base.strictness, base.region, Some(base.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((base.auto_space).clone()), Some(base.glue_placement), (base.adjustment).clone(), KinsokuMode::Fixed { level: KinsokuLevel::Basic, hanging: HangingPunctuationStyle::Disabled }, (base.punctuation_width).clone());
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(InlineObjectLayoutTestSupport::inline_object_layout_test_support_resolver((p).clone())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), match &(b) { None => Some(Box::new(GreedyLineBreaker::new(None, None, None, None)) as Box<dyn LineBreaker>), Some(__option22) => Some((*__option22).clone()) }, Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn inline_object_layout_test_support_lines(r: LayoutResult, text: &UStr) -> Vec<UString> {
        let mut a: Vec<UString> = vec![];
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            a.push(u_string::substring(&text, i32::from_ne_bytes(((((r.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes(((((r.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone().end) as i32).to_ne_bytes())));
        }
        return a;
    }

    pub fn inline_object_layout_test_support_render_strings(a: &Vec<UString>) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined = a; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn inline_object_layout_test_support_same_range(first: TextRange, second: TextRange) -> bool {
        return first.start == second.start && first.end == second.end;
    }

    pub fn inline_object_layout_test_support_single_cluster(r: LayoutResult, range: TextRange) -> Cluster {
        let mut f: Option<Cluster> = None;
        let mut n = 0u32;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if InlineObjectLayoutTestSupport::inline_object_layout_test_support_same_range(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone(), (range).clone()) {
                f = Some((r.clusters[usize::try_from(i).unwrap_or(0)]).clone());
                n = u32::wrapping_add(n, 1);
            }
        }
        return (if n == 1 { f } else { None }).unwrap();
    }

    pub fn inline_object_layout_test_support_single_positioned(cs: &Vec<PositionedCluster>, range: TextRange) -> PositionedCluster {
        let mut f: Option<PositionedCluster> = None;
        let mut n = 0u32;
        for i in 0..match u32::try_from(cs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if InlineObjectLayoutTestSupport::inline_object_layout_test_support_same_range(((cs[usize::try_from(i).unwrap_or(0)]).clone().range).clone(), (range).clone()) {
                f = Some((cs[usize::try_from(i).unwrap_or(0)]).clone());
                n = u32::wrapping_add(n, 1);
            }
        }
        return (if n == 1 { f } else { None }).unwrap();
    }

    pub fn inline_object_layout_test_support_single_inline_object_decision(r: LayoutResult, range: TextRange) -> InlineObjectDecisionInfo {
        let mut f: Option<InlineObjectDecisionInfo> = None;
        let mut n = 0u32;
        for i in 0..match u32::try_from((r.debug).clone().inline_object_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if InlineObjectLayoutTestSupport::inline_object_layout_test_support_same_range((((r.debug).clone().inline_object_decisions[usize::try_from(i).unwrap_or(0)]).clone().range).clone(), (range).clone()) {
                f = Some(((r.debug).clone().inline_object_decisions[usize::try_from(i).unwrap_or(0)]).clone());
                n = u32::wrapping_add(n, 1);
            }
        }
        return (if n == 1 { f } else { None }).unwrap();
    }

    pub fn inline_object_layout_test_support_single_shaping_decision(r: LayoutResult, range: TextRange) -> ShapingDecisionInfo {
        let mut f: Option<ShapingDecisionInfo> = None;
        let mut n = 0u32;
        for i in 0..match u32::try_from((r.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if InlineObjectLayoutTestSupport::inline_object_layout_test_support_same_range((((r.debug).clone().shaping_decisions[usize::try_from(i).unwrap_or(0)]).clone().range).clone(), (range).clone()) {
                f = Some(((r.debug).clone().shaping_decisions[usize::try_from(i).unwrap_or(0)]).clone());
                n = u32::wrapping_add(n, 1);
            }
        }
        return (if n == 1 { f } else { None }).unwrap();
    }

    pub fn inline_object_layout_test_support_render_repair_allocations(a: &[LineRepairAllocationInfo]) -> UString {
        let mut x: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            x.push(UString::from(format!("{}", (a[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined1 = x; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn inline_object_layout_test_support_render_trim_decisions(a: &[LineEdgeTrimDecisionInfo]) -> UString {
        let mut x: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            x.push(UString::from(format!("{}", (a[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined2 = x; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn inline_object_layout_test_support_render_line_ranges(r: LayoutResult) -> UString {
        let mut x: Vec<UString> = vec![];
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            x.push(UString::from(format!("{}", ((r.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined3 = x; let mut out = String::new(); let n = joined3.len(); let mut index3 = 0usize; while index3 < n { if index3 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined3[index3]); index3 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }
}

#[derive(Clone, PartialEq)]
pub struct Resolver {
    pub(crate) p: ClreqProfile,
}

impl Resolver {
    pub fn new(p: ClreqProfile) -> Self {
        Self {
            p,
        }
    }

    pub fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        return ((self.p).clone()).clone();
    }
}

impl ClreqProfileResolver for Resolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.InlineObjectLayoutTestSupport.Resolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        return ((self.p).clone()).clone();
    }
}
