use crate::org::tiqian::clreq::cjk_punctuation_glyph_policy::CjkPunctuationGlyphPolicy;
use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::clreq_punctuation_glyph_substitutor::ClreqPunctuationGlyphSubstitutor;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::font_policy::FontCandidate;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::cluster_role_resolution::ResolvedClusterRange;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStage;
use crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageResult;
use crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::linebreak::hyphenator::Hyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaper;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestSupportParagraphFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestSupportParagraphFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestSupportParagraphFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestSupportParagraphFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestSupportParagraphFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestSupportParagraphFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestSupportParagraphFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestSupportParagraphFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: ParagraphShapingStageCoverageTestSupportParagraphFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestSupportParagraphFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestSupportParagraphFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestSupportParagraphFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for ParagraphShapingStageCoverageTestSupportParagraphFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        ParagraphShapingStageCoverageTestSupportParagraphFault::IllegalStateExceptionFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestSupportLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestSupportLayoutFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestSupportLayoutFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestSupportLayoutFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestSupportLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestSupportLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestSupportLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestSupportLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestSupportLayoutFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: ParagraphShapingStageCoverageTestSupportLayoutFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestSupportLayoutFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestSupportLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphShapingStageCoverageTestSupportLayoutFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for ParagraphShapingStageCoverageTestSupportLayoutFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        ParagraphShapingStageCoverageTestSupportLayoutFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageCoverageTestSupportShapeFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
}
impl std::fmt::Display for ParagraphShapingStageCoverageTestSupportShapeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphShapingStageCoverageTestSupportShapeFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            ParagraphShapingStageCoverageTestSupportShapeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestSupportShapeFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: ParagraphShapingStageCoverageTestSupportShapeFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestSupportShapeFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageCoverageTestSupportShapeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageCoverageTestSupportShapeFault) -> Self {
        match value {
            ParagraphShapingStageCoverageTestSupportShapeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for ParagraphShapingStageCoverageTestSupportShapeFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        ParagraphShapingStageCoverageTestSupportShapeFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageCoverageTestSupportShapeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageCoverageTestSupportShapeFault::TextRangeErrorFault(value)
    }
}

#[derive(Clone)]
pub struct ParagraphEmptyHyphenShaper {
    pub delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl ParagraphEmptyHyphenShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, i: ShapingInput) -> Result<ShapingResult, ParagraphShapingStageCoverageTestSupportShapeFault> {
        if i.text.to_ustring() == UString::from("-") || (i.display_text).to_ustring() == UString::from("-") {
            return Ok(ShapingResult::new(vec![].to_vec(), vec![].to_vec(), Some(vec![])));
        }
        return Ok(self.delegate.lock().unwrap().shape((i).clone()).map_err(|e| ParagraphShapingStageCoverageTestSupportShapeFault::TextShaperShapeFaultFault(e))?);
    }
}

impl ITextShaper for ParagraphEmptyHyphenShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphEmptyHyphenShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        if i.text.to_ustring() == UString::from("-") || (i.display_text).to_ustring() == UString::from("-") {
            return Ok(ShapingResult::new(vec![].to_vec(), vec![].to_vec(), Some(vec![])));
        }
        return Ok(self.delegate.lock().unwrap().shape((i).clone())?);
    }
}

#[derive(Clone)]
pub struct ParagraphMultiClusterShaper {
    pub delegate: Arc<Mutex<ExplainableStubTextShaper>>,
    pub toggle: bool,
}

impl ParagraphMultiClusterShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
            toggle: false,
        }
    }

    pub fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, ParagraphShapingStageCoverageTestSupportShapeFault> {
        let r = self.delegate.lock().unwrap().shape((i).clone()).map_err(|e| ParagraphShapingStageCoverageTestSupportShapeFault::TextShaperShapeFaultFault(e))?;
        if i32::from_ne_bytes((((i.range).clone().get_length()) as i32).to_ne_bytes()) <= 1 {
            return Ok(r);
        }
        self.toggle = !self.toggle;
        if !self.toggle {
            return Ok(r);
        }
        let m = u32::wrapping_add((i.range).clone().start, (i.range).clone().end) / (2);
        return Ok(ShapingResult::new(vec![
    (Cluster::new(TextRange::new((i.range).clone().start, m).map_err(|e| ParagraphShapingStageCoverageTestSupportShapeFault::TextRangeErrorFault(e))?, &(UStr::new(&[97])), &(UStr::new(&[107])), 100.0f64, Some(UString::from("a")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(m, (i.range).clone().end).map_err(|e| ParagraphShapingStageCoverageTestSupportShapeFault::TextRangeErrorFault(e))?, &(UStr::new(&[98])), &(UStr::new(&[107])), 100.0f64, Some(UString::from("b")), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), r.glyph_runs.to_vec(), Some((r.decisions).clone())));
    }
}

impl ITextShaper for ParagraphMultiClusterShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphMultiClusterShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let r = self.delegate.lock().unwrap().shape((i).clone())?;
        if i32::from_ne_bytes((((i.range).clone().get_length()) as i32).to_ne_bytes()) <= 1 {
            return Ok(r);
        }
        self.toggle = !self.toggle;
        if !self.toggle {
            return Ok(r);
        }
        let m = u32::wrapping_add((i.range).clone().start, (i.range).clone().end) / (2);
        return Ok(ShapingResult::new(vec![
    (Cluster::new(TextRange::new((i.range).clone().start, m).map_err(|e| TextShaperShapeFault::TextRangeErrorFault(e))?, &(UStr::new(&[97])), &(UStr::new(&[107])), 100.0f64, Some(UString::from("a")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(m, (i.range).clone().end).map_err(|e| TextShaperShapeFault::TextRangeErrorFault(e))?, &(UStr::new(&[98])), &(UStr::new(&[107])), 100.0f64, Some(UString::from("b")), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), r.glyph_runs.to_vec(), Some((r.decisions).clone())));
    }
}

#[derive(Clone)]
pub struct ParagraphWordShaper {
    pub delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl ParagraphWordShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, i: ShapingInput) -> Result<ShapingResult, ParagraphShapingStageCoverageTestSupportShapeFault> {
        let r = self.delegate.lock().unwrap().shape((i).clone()).map_err(|e| ParagraphShapingStageCoverageTestSupportShapeFault::TextShaperShapeFaultFault(e))?;
        if i.range.clone().get_length() == 2 && u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())) == UString::from("em") {
            return Ok(ShapingResult::new(vec![
    (Cluster::new(TextRange::new((i.range).clone().start, u32::wrapping_add((i.range).clone().start, 1)).map_err(|e| ParagraphShapingStageCoverageTestSupportShapeFault::TextRangeErrorFault(e))?, &(UStr::new(&[101])), &(UStr::new(&[107])), 10.0f64, Some(UString::from("e")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(u32::wrapping_add((i.range).clone().start, 1), (i.range).clone().end).map_err(|e| ParagraphShapingStageCoverageTestSupportShapeFault::TextRangeErrorFault(e))?, &(UStr::new(&[109])), &(UStr::new(&[107])), 10.0f64, Some(UString::from("m")), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), r.glyph_runs.to_vec(), Some((r.decisions).clone())));
        }
        return Ok(r);
    }
}

impl ITextShaper for ParagraphWordShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphWordShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let r = self.delegate.lock().unwrap().shape((i).clone())?;
        if i.range.clone().get_length() == 2 && u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())) == UString::from("em") {
            return Ok(ShapingResult::new(vec![
    (Cluster::new(TextRange::new((i.range).clone().start, u32::wrapping_add((i.range).clone().start, 1)).map_err(|e| TextShaperShapeFault::TextRangeErrorFault(e))?, &(UStr::new(&[101])), &(UStr::new(&[107])), 10.0f64, Some(UString::from("e")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(u32::wrapping_add((i.range).clone().start, 1), (i.range).clone().end).map_err(|e| TextShaperShapeFault::TextRangeErrorFault(e))?, &(UStr::new(&[109])), &(UStr::new(&[107])), 10.0f64, Some(UString::from("m")), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), r.glyph_runs.to_vec(), Some((r.decisions).clone())));
        }
        return Ok(r);
    }
}

#[derive(Clone)]
pub struct ParagraphEmptyClusterShaper {
    pub delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl ParagraphEmptyClusterShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, i: ShapingInput) -> Result<ShapingResult, ParagraphShapingStageCoverageTestSupportShapeFault> {
        let r = self.delegate.lock().unwrap().shape((i).clone()).map_err(|e| ParagraphShapingStageCoverageTestSupportShapeFault::TextShaperShapeFaultFault(e))?;
        if i.text.to_ustring() == UString::from("singlecluster") || (i.display_text).to_ustring() == UString::from("singlecluster") {
            return Ok(ShapingResult::new(vec![].to_vec(), r.glyph_runs.to_vec(), Some((r.decisions).clone())));
        }
        return Ok(r);
    }
}

impl ITextShaper for ParagraphEmptyClusterShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphEmptyClusterShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let r = self.delegate.lock().unwrap().shape((i).clone())?;
        if i.text.to_ustring() == UString::from("singlecluster") || (i.display_text).to_ustring() == UString::from("singlecluster") {
            return Ok(ShapingResult::new(vec![].to_vec(), r.glyph_runs.to_vec(), Some((r.decisions).clone())));
        }
        return Ok(r);
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphDeficientDashShaper {
}

impl ParagraphDeficientDashShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, i: ShapingInput) -> Result<ShapingResult, ParagraphShapingStageCoverageTestSupportShapeFault> {
        let c = Cluster::new((i.range).clone(), u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())).as_ustr(), &(UStr::new(&[116,101,115,116])), 32.0f64, Some((i.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let g = Glyph::new(1u32, (i.range).clone(), 32.0f64, Some(0.0f64), Some(0.0), None, Some(Rect::new(0.0f64, 0.0f64, 20.0f64, 10.0f64)), None, None);
        return Ok(ShapingResult::new(vec![(c).clone()].to_vec(), vec![
    (GlyphRun::new((i.range).clone(), &(UStr::new(&[116,101,115,116])), vec![(g).clone()].to_vec(), 32.0f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

impl ITextShaper for ParagraphDeficientDashShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphDeficientDashShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let c = Cluster::new((i.range).clone(), u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())).as_ustr(), &(UStr::new(&[116,101,115,116])), 32.0f64, Some((i.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let g = Glyph::new(1u32, (i.range).clone(), 32.0f64, Some(0.0f64), Some(0.0), None, Some(Rect::new(0.0f64, 0.0f64, 20.0f64, 10.0f64)), None, None);
        return Ok(ShapingResult::new(vec![(c).clone()].to_vec(), vec![
    (GlyphRun::new((i.range).clone(), &(UStr::new(&[116,101,115,116])), vec![(g).clone()].to_vec(), 32.0f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphSufficientDashShaper {
}

impl ParagraphSufficientDashShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, i: ShapingInput) -> Result<ShapingResult, ParagraphShapingStageCoverageTestSupportShapeFault> {
        let c = Cluster::new((i.range).clone(), u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())).as_ustr(), &(UStr::new(&[116,101,115,116])), 32.0f64, Some((i.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let g = Glyph::new(1u32, (i.range).clone(), 32.0f64, Some(0.0f64), Some(0.0), None, Some(Rect::new(0.0f64, 0.0f64, 30.0f64, 10.0f64)), None, None);
        return Ok(ShapingResult::new(vec![(c).clone()].to_vec(), vec![
    (GlyphRun::new((i.range).clone(), &(UStr::new(&[116,101,115,116])), vec![(g).clone()].to_vec(), 32.0f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

impl ITextShaper for ParagraphSufficientDashShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphSufficientDashShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let c = Cluster::new((i.range).clone(), u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())).as_ustr(), &(UStr::new(&[116,101,115,116])), 32.0f64, Some((i.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let g = Glyph::new(1u32, (i.range).clone(), 32.0f64, Some(0.0f64), Some(0.0), None, Some(Rect::new(0.0f64, 0.0f64, 30.0f64, 10.0f64)), None, None);
        return Ok(ShapingResult::new(vec![(c).clone()].to_vec(), vec![
    (GlyphRun::new((i.range).clone(), &(UStr::new(&[116,101,115,116])), vec![(g).clone()].to_vec(), 32.0f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphRollbackShaper {
    pub call: u32,
}

impl ParagraphRollbackShaper {
    pub fn new() -> Self {
        Self {
            call: 0,
        }
    }

    pub fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, ParagraphShapingStageCoverageTestSupportShapeFault> {
        self.call += 1;
        let c = Cluster::new((i.range).clone(), u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())).as_ustr(), &(UStr::new(&[116,101,115,116])), 16.0f64, Some((i.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let issue = if self.call == 1 { Some(TextShaper::TEXT_SHAPER_UNVERIFIED_DISPLAY_SUBSTITUTION_COVERAGE_ISSUE.to_ustring()) } else { None };
        let d = ShapingDecisionInfo::new((i.range).clone(), u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())).as_ustr(), (i.display_text).to_ustring().as_ustr(), &(UStr::new(&[116,101,115,116])), 1u32, 16.0f64, &(UStr::new(&[84,101,115,116])), &(UStr::new(&[116,101,115,116])), Some(0), if self.call == 2 { Some(1) } else { Some(0) }, None, None, None, None, None, issue.clone());
        return Ok(ShapingResult::new(vec![(c).clone()].to_vec(), vec![
    (GlyphRun::new((i.range).clone(), &(UStr::new(&[116,101,115,116])), vec![].to_vec(), 16.0f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![(d).clone()])));
    }
}

impl ITextShaper for ParagraphRollbackShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphRollbackShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        self.call += 1;
        let c = Cluster::new((i.range).clone(), u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())).as_ustr(), &(UStr::new(&[116,101,115,116])), 16.0f64, Some((i.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let issue = if self.call == 1 { Some(TextShaper::TEXT_SHAPER_UNVERIFIED_DISPLAY_SUBSTITUTION_COVERAGE_ISSUE.to_ustring()) } else { None };
        let d = ShapingDecisionInfo::new((i.range).clone(), u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())).as_ustr(), (i.display_text).to_ustring().as_ustr(), &(UStr::new(&[116,101,115,116])), 1u32, 16.0f64, &(UStr::new(&[84,101,115,116])), &(UStr::new(&[116,101,115,116])), Some(0), if self.call == 2 { Some(1) } else { Some(0) }, None, None, None, None, None, issue.clone());
        return Ok(ShapingResult::new(vec![(c).clone()].to_vec(), vec![
    (GlyphRun::new((i.range).clone(), &(UStr::new(&[116,101,115,116])), vec![].to_vec(), 16.0f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![(d).clone()])));
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphMultiGlyphShaper {
    pub count: u32,
}

impl ParagraphMultiGlyphShaper {
    pub fn new() -> Self {
        Self {
            count: 0,
        }
    }

    pub fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, ParagraphShapingStageCoverageTestSupportShapeFault> {
        self.count += 1;
        let c = Cluster::new((i.range).clone(), u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())).as_ustr(), &(UStr::new(&[116,101,115,116])), 32.0f64, Some((i.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let gs = if u32::from_ne_bytes(((i32::from_ne_bytes(((self.count) as i32).to_ne_bytes()) % 3i32) as u32).to_ne_bytes()) == 0 { vec![] } else { if u32::from_ne_bytes(((i32::from_ne_bytes(((self.count) as i32).to_ne_bytes()) % 3i32) as u32).to_ne_bytes()) == 1 { vec![
    (Glyph::new(1u32, (i.range).clone(), 16.0f64, Some(0.0f64), Some(0.0), None, None, None, None)).clone(),
    (Glyph::new(2u32, (i.range).clone(), 16.0f64, Some(16.0f64), Some(0.0), None, None, None, None)).clone(),
] } else { vec![
    (Glyph::new(1u32, (i.range).clone(), 32.0f64, Some(0.0f64), Some(0.0), None, None, None, None)).clone(),
] } };
        return Ok(ShapingResult::new(vec![(c).clone()].to_vec(), vec![
    (GlyphRun::new((i.range).clone(), &(UStr::new(&[116,101,115,116])), gs.to_vec(), 32.0f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

impl ITextShaper for ParagraphMultiGlyphShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphMultiGlyphShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, i: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        self.count += 1;
        let c = Cluster::new((i.range).clone(), u_string::substring(&(i.text).to_ustring(), i32::from_ne_bytes((((i.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((i.range).clone().end) as i32).to_ne_bytes())).as_ustr(), &(UStr::new(&[116,101,115,116])), 32.0f64, Some((i.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let gs = if u32::from_ne_bytes(((i32::from_ne_bytes(((self.count) as i32).to_ne_bytes()) % 3i32) as u32).to_ne_bytes()) == 0 { vec![] } else { if u32::from_ne_bytes(((i32::from_ne_bytes(((self.count) as i32).to_ne_bytes()) % 3i32) as u32).to_ne_bytes()) == 1 { vec![
    (Glyph::new(1u32, (i.range).clone(), 16.0f64, Some(0.0f64), Some(0.0), None, None, None, None)).clone(),
    (Glyph::new(2u32, (i.range).clone(), 16.0f64, Some(16.0f64), Some(0.0), None, None, None, None)).clone(),
] } else { vec![
    (Glyph::new(1u32, (i.range).clone(), 32.0f64, Some(0.0f64), Some(0.0), None, None, None, None)).clone(),
] } };
        return Ok(ShapingResult::new(vec![(c).clone()].to_vec(), vec![
    (GlyphRun::new((i.range).clone(), &(UStr::new(&[116,101,115,116])), gs.to_vec(), 32.0f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphCoverageHyphenator {
    pub mode: u32,
}

impl ParagraphCoverageHyphenator {
    pub fn new(mode: Option<u32>) -> Self {
        Self {
            mode: mode.unwrap_or_default(),
        }
    }

    pub fn hyphenate(&self, w: &UStr) -> Vec<u32> {
        return if self.mode == 1 && (u32::from_ne_bytes(((u_string::find_from(&(w), UString::from("hyphen").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647 { vec![2, 4] } else { if self.mode == 2 { vec![1, 2, 3] } else { vec![2] } };
    }
}

impl Hyphenator for ParagraphCoverageHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphCoverageHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, w: &UStr) -> Vec<u32> {
        return if self.mode == 1 && (u32::from_ne_bytes(((u_string::find_from(&(w), UString::from("hyphen").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647 { vec![2, 4] } else { if self.mode == 2 { vec![1, 2, 3] } else { vec![2] } };
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphSegmentationHyphenator {
}

impl ParagraphSegmentationHyphenator {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn hyphenate(&self, w: &UStr) -> Vec<u32> {
        return if u32::from_ne_bytes(((u_string::find_from(&(w), UString::from("hyphen").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 { vec![2, 4] } else { vec![] };
    }
}

impl Hyphenator for ParagraphSegmentationHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphSegmentationHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, w: &UStr) -> Vec<u32> {
        return if u32::from_ne_bytes(((u_string::find_from(&(w), UString::from("hyphen").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 { vec![2, 4] } else { vec![] };
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphBiblioHyphenator {
}

impl ParagraphBiblioHyphenator {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn hyphenate(&self, w: &UStr) -> Vec<u32> {
        return if w == UString::from("hyphenated") { vec![3, 6] } else { vec![] };
    }
}

impl Hyphenator for ParagraphBiblioHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphBiblioHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, w: &UStr) -> Vec<u32> {
        return if w == UString::from("hyphenated") { vec![3, 6] } else { vec![] };
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphWordCutsHyphenator {
}

impl ParagraphWordCutsHyphenator {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn hyphenate(&self, w: &UStr) -> Vec<u32> {
        if w == UString::from("abcdef") {
            return vec![1];
        }
        if w == UString::from("ghijkl") {
            return vec![2];
        }
        if w == UString::from("mnopqr") {
            return vec![3];
        }
        if w == UString::from("empty") {
            return vec![2];
        }
        return vec![];
    }
}

impl Hyphenator for ParagraphWordCutsHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphWordCutsHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, w: &UStr) -> Vec<u32> {
        if w == UString::from("abcdef") {
            return vec![1];
        }
        if w == UString::from("ghijkl") {
            return vec![2];
        }
        if w == UString::from("mnopqr") {
            return vec![3];
        }
        if w == UString::from("empty") {
            return vec![2];
        }
        return vec![];
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphTierHyphenator {
}

impl ParagraphTierHyphenator {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn hyphenate(&self, w: &UStr) -> Vec<u32> {
    let __units = u_string::units(&w);
    let __count = u_string::unit_count(&w);
        return if u32::from_ne_bytes(((u_string::find_from(&(w), UString::from("Machine").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 { vec![4294967295u32, 0, 3, __count, u32::wrapping_add(__count, 1)] } else { vec![2] };
    }
}

impl Hyphenator for ParagraphTierHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphTierHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, w: &UStr) -> Vec<u32> {
    let __units1 = u_string::units(&w);
    let __count1 = u_string::unit_count(&w);
        return if u32::from_ne_bytes(((u_string::find_from(&(w), UString::from("Machine").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 { vec![4294967295u32, 0, 3, __count1, u32::wrapping_add(__count1, 1)] } else { vec![2] };
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphDirectShapeHyphenator {
}

impl ParagraphDirectShapeHyphenator {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn hyphenate(&self, w: &UStr) -> Vec<u32> {
    let __units2 = u_string::units(&w);
    let __count2 = u_string::unit_count(&w);
        if w == UString::from("abcdef") {
            return vec![1];
        }
        if w == UString::from("abcdeg") {
            return vec![2];
        }
        if w == UString::from("antidisestablishmentarianism") {
            return vec![];
        }
        if w == UString::from("Machine") {
            return vec![4294967295u32, 0, 2, __count2, u32::wrapping_add(__count2, 2)];
        }
        return vec![2];
    }
}

impl Hyphenator for ParagraphDirectShapeHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphDirectShapeHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, w: &UStr) -> Vec<u32> {
    let __units3 = u_string::units(&w);
    let __count3 = u_string::unit_count(&w);
        if w == UString::from("abcdef") {
            return vec![1];
        }
        if w == UString::from("abcdeg") {
            return vec![2];
        }
        if w == UString::from("antidisestablishmentarianism") {
            return vec![];
        }
        if w == UString::from("Machine") {
            return vec![4294967295u32, 0, 2, __count3, u32::wrapping_add(__count3, 2)];
        }
        return vec![2];
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphTierPriorityHyphenator {
}

impl ParagraphTierPriorityHyphenator {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn hyphenate(&self, w: &UStr) -> Vec<u32> {
    let __units4 = u_string::units(&w);
    let __count4 = u_string::unit_count(&w);
        return if w == UString::from("abcdef") { vec![2, 4] } else { vec![4294967295u32, 0, 1, 2, __count4, u32::wrapping_add(__count4, 2)] };
    }
}

impl Hyphenator for ParagraphTierPriorityHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphTierPriorityHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, w: &UStr) -> Vec<u32> {
    let __units5 = u_string::units(&w);
    let __count5 = u_string::unit_count(&w);
        return if w == UString::from("abcdef") { vec![2, 4] } else { vec![4294967295u32, 0, 1, 2, __count5, u32::wrapping_add(__count5, 2)] };
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphTierLoopHyphenator {
}

impl ParagraphTierLoopHyphenator {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn hyphenate(&self, w: &UStr) -> Vec<u32> {
        if w == UString::from("abcdef") {
            return vec![2, 4];
        }
        if w == UString::from("cdef") {
            return vec![1];
        }
        return vec![];
    }
}

impl Hyphenator for ParagraphTierLoopHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphShapingStageCoverageTestSupport.ParagraphTierLoopHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, w: &UStr) -> Vec<u32> {
        if w == UString::from("abcdef") {
            return vec![2, 4];
        }
        if w == UString::from("cdef") {
            return vec![1];
        }
        return vec![];
    }
}

#[derive(Clone, Copy)]
pub struct ParagraphShapingStageCoverageTestSupport;

impl ParagraphShapingStageCoverageTestSupport {
    pub fn paragraph_shaping_stage_coverage_test_support_input(text: &UStr, width: f64, spans: Option<Vec<LineBreakSpan>>) -> Result<LayoutInput, TextRangeError> {
        return Ok(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), (spans).clone(), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
    }

    pub fn paragraph_shaping_stage_coverage_test_support_begin(n: &UStr) -> TestTraceRecorder {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,83,104,97,112,105,110,103,83,116,97,103,101,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(n);
        return r;
    }

    pub fn paragraph_shaping_stage_coverage_test_support_layout(e: &mut ExplainableStubParagraphLayoutEngine, text: &UStr, width: f64) -> Result<(), ParagraphShapingStageCoverageTestSupportLayoutFault> {
        let res = e.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphShapingStageCoverageTestSupportLayoutFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphShapingStageCoverageTestSupportLayoutFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(TestTraceRender::test_trace_render_cap(if false { UString::from("null") } else { UString::from(format!("{}", res.to_string()).as_str()) }.as_ustr()).map_err(|e| ParagraphShapingStageCoverageTestSupportLayoutFault::UStringFaultFault(e))?.as_ustr(), None).map_err(|e| ParagraphShapingStageCoverageTestSupportLayoutFault::UStringFaultFault(e))?;
        Ok(())
    }

    pub fn paragraph_shaping_stage_coverage_test_support_engine(shaper: Option<Arc<Mutex<dyn ITextShaper>>>, hyphenator: Option<Box<dyn Hyphenator>>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), shaper, hyphenator, Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn paragraph_shaping_stage_coverage_test_support_candidate(role: FontRole) -> FontCandidate {
        return FontCandidate::new(&(UStr::new(&[107])), &(UStr::new(&[102])), role);
    }

    pub fn paragraph_shaping_stage_coverage_test_support_decision(range: TextRange, role: FontRole) -> FontDecision {
        return FontDecision::new((range).clone(), ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_candidate(role), role, &(UStr::new(&[114])));
    }

    pub fn paragraph_shaping_stage_coverage_test_support_paragraph(engine: &mut ExplainableStubParagraphLayoutEngine, input: LayoutInput, text: &UStr, width: f64, role: FontRole, italic: Option<bool>) -> Result<ParagraphShapingStageResult,
ParagraphShapingStageShapeParagraphFault> {
        let range = TextRange::new(0u32, u_string::unit_count(&(text))).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
        let mut b: SortedMapTableBuilder<TextRange, FontDecision> = SortedTable::sorted_table_map_builder::<TextRange, FontDecision>(Arc::new(compare_text_range));
        b.put(&(range), &(ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_decision((range).clone(), role)));
        let rb: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
        let cb = vec![(ResolvedClusterRange::new((range).clone(), role, Some(false), Some(false), None)).clone()];
        return Ok(ParagraphShapingStage::paragraph_shaping_stage_shape_paragraph(engine, (input).clone(), text, 16.0f64, width, &cb, b.clone().build(), SortedTable::sorted_table_map_builder::<TextRange, InlineObjectSpan>(Arc::new(compare_text_range)).clone().build(), ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints)), {  Arc::new(move |__| {
        return TextStyle::new(Some(vec![]), Some(16.0f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None));
}) }, {  Arc::new(move |__| {
        return (italic).unwrap();
}) }, rb.clone().build(), None, None)?);
    }

    pub fn paragraph_shaping_stage_coverage_test_support_paragraph_ranges(engine: &mut ExplainableStubParagraphLayoutEngine, input: LayoutInput, text: &UStr, width: f64, ranges: &Vec<ResolvedClusterRange>, decisions: &Vec<TextRange>) -> Result<ParagraphShapingStageResult,
ParagraphShapingStageShapeParagraphFault> {
        let mut b: SortedMapTableBuilder<TextRange, FontDecision> = SortedTable::sorted_table_map_builder::<TextRange, FontDecision>(Arc::new(compare_text_range));
        for r in decisions {
            b.put(&(r), &(ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_decision((r).clone(), FontRole::LatinText)));
        }
        let rb: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
        return Ok(ParagraphShapingStage::paragraph_shaping_stage_shape_paragraph(engine, (input).clone(), text, 16.0f64, width, &ranges, b.clone().build(), SortedTable::sorted_table_map_builder::<TextRange, InlineObjectSpan>(Arc::new(compare_text_range)).clone().build(), ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints)), {  Arc::new(move |__| {
        return TextStyle::new(Some(vec![]), Some(16.0f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None));
}) }, {  Arc::new(move |__| {
        return false;
}) }, rb.clone().build(), None, None)?);
    }

    pub fn paragraph_shaping_stage_coverage_test_support_paragraph_ranges_with_rejected_tiers(engine: &mut ExplainableStubParagraphLayoutEngine, input: LayoutInput, text: &UStr, width: f64, ranges: &Vec<ResolvedClusterRange>, decisions: &Vec<TextRange>, rejected_spans: &Vec<TextRange>, rejected_tiers: &Vec<Vec<u32>>) -> Result<ParagraphShapingStageResult, ParagraphShapingStageShapeParagraphFault> {
        let mut b: SortedMapTableBuilder<TextRange, FontDecision> = SortedTable::sorted_table_map_builder::<TextRange, FontDecision>(Arc::new(compare_text_range));
        for r in decisions {
            b.put(&(r), &(ParagraphShapingStageCoverageTestSupport::paragraph_shaping_stage_coverage_test_support_decision((r).clone(), FontRole::LatinText)));
        }
        let mut rb: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
        for i in 0..match u32::try_from(rejected_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let mut tiers: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
            for j in 0..match u32::try_from((rejected_tiers[usize::try_from(i).unwrap_or(0)]).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                tiers.put(&((rejected_tiers[usize::try_from(i).unwrap_or(0)]).clone()[usize::try_from(j).unwrap_or(0)]));
            }
            rb.put(&((rejected_spans[usize::try_from(i).unwrap_or(0)]).clone()), &(tiers.clone().build()));
        }
        return Ok(ParagraphShapingStage::paragraph_shaping_stage_shape_paragraph(engine, (input).clone(), text, 16.0f64, width, &ranges, b.clone().build(), SortedTable::sorted_table_map_builder::<TextRange, InlineObjectSpan>(Arc::new(compare_text_range)).clone().build(), ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints)), {  Arc::new(move |__| {
        return TextStyle::new(Some(vec![]), Some(16.0f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None));
}) }, {  Arc::new(move |__| {
        return false;
}) }, rb.clone().build(), None, None)?);
    }
}
