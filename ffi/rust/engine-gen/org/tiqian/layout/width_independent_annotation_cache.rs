use crate::org::tiqian::clreq::auto_space_mode::AutoSpaceMode;
use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_punctuation_glyph_substitutor::ClreqPunctuationGlyphSubstitutor;
use crate::org::tiqian::clreq::kinsoku_modes::KinsokuModes;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::core::accurate_sum::AccurateSum;
use crate::org::tiqian::core::auto_space_decision_info::AutoSpaceDecisionInfo;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::inline_object_punctuation_attachment_decision_info::InlineObjectPunctuationAttachmentDecisionInfo;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::line_length_grid_decision_info::LineLengthGridDecisionInfo;
use crate::org::tiqian::core::mandatory_break_decision_info::MandatoryBreakDecisionInfo;
use crate::org::tiqian::core::role_override_info::RoleOverrideInfo;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::ruby_span::compare_ruby_span;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_east_asian_spacing::UnicodeEastAsianSpacing;
use crate::org::tiqian::core::zero_width_break_decision_info::ZeroWidthBreakDecisionInfo;
use crate::org::tiqian::font::font_metrics::FontMetricsRequest;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_policy::FontRequest;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleClassifier;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::annotation_geometry_stage::RubyFontGeometry;
use crate::org::tiqian::layout::cluster_role_resolution::ClusterRoleResolution;
use crate::org::tiqian::layout::cluster_role_resolution::ResolvedClusterRange;
use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver::ContextualDashEllipsisAwareFontRoleClassifier;
use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver::ContextualDashEllipsisRoleResolver;
use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver::ContextualDashEllipsisRoles;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_break_planning_stage::ParagraphLayoutPrep;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStage;
use crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageResult;
use crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::punctuation_geometry_ledger::GlueCapacity;
use crate::org::tiqian::layout::punctuation_geometry_ledger::PunctuationGeometryLedger;
use crate::org::tiqian::layout::punctuation_geometry_stage::PunctuationGeometryStage;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressionResult;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePair;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAwareFontRoleClassifier;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteRoleDecision;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
}

impl From<WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        match value {
            WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault) -> Self {
        match value {
            WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault::TextRangeErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    ParagraphShapingStageShapeParagraphFaultFault(crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault),
}

impl From<WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> for crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault {
    fn from(value: WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::ParagraphShapingStageShapeParagraphFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault> for WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault {
    fn from(value: crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault) -> Self {
        WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::ParagraphShapingStageShapeParagraphFaultFault(value)
    }
}

#[derive(Clone, PartialEq)]
pub struct WidthIndependentAnnotationKey {
    pub text: String,
    pub spans: Vec<TextSpan>,
    pub line_break_spans: Vec<LineBreakSpan>,
    pub source_boundaries: Vec<u32>,
    pub text_style: TextStyle,
    pub decorations: Vec<DecorationSpan>,
    pub ruby_spans: Vec<RubySpan>,
    pub inline_boxes: Vec<InlineBoxSpan>,
    pub inline_objects: Vec<InlineObjectSpan>,
    pub profile_id: LayoutProfileId,
    pub emphasis_dot_gap_em: f64,
    pub rejected_technical_tiers_by_span: SortedMapTable<TextRange, SortedSetTable<u32>>,
}

impl WidthIndependentAnnotationKey {
    pub fn new(text: &str, spans: Vec<TextSpan>, line_break_spans: Vec<LineBreakSpan>, source_boundaries: Vec<u32>, text_style: TextStyle, decorations: Vec<DecorationSpan>, ruby_spans: Vec<RubySpan>, inline_boxes: Vec<InlineBoxSpan>, inline_objects: Vec<InlineObjectSpan>,
profile_id: LayoutProfileId, emphasis_dot_gap_em: f64, rejected_technical_tiers_by_span: SortedMapTable<TextRange, SortedSetTable<u32>>) -> Self {
        Self {
            text: text.to_string(),
            spans,
            line_break_spans,
            source_boundaries,
            text_style,
            decorations,
            ruby_spans,
            inline_boxes,
            inline_objects,
            profile_id,
            emphasis_dot_gap_em,
            rejected_technical_tiers_by_span,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "WidthIndependentAnnotationKey(",
            "text=",
            (self.text).to_string(),
            ", ",
            "spans=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.spans).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "lineBreakSpans=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_break_spans).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "sourceBoundaries=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.source_boundaries).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(arr[i]));
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "textStyle=",
            (self.text_style).clone().to_string(),
            ", ",
            "decorations=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decorations).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "rubySpans=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.ruby_spans).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "inlineBoxes=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.inline_boxes).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "inlineObjects=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.inline_objects).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "profileId=",
            (self.profile_id).clone().to_string(),
            ", ",
            "emphasisDotGapEm=",
            self.emphasis_dot_gap_em,
            ", ",
            "rejectedTechnicalTiersBySpan=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.rejected_technical_tiers_by_span).clone();
        let n = map.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", map.key_at(i).to_string(), {
        let mut out = String::new();
        out.push('[');
        let set = map.value_at(i);
        let n = set.size();
        let mut i1 = 0;
        while i1 < n {
            if i1 > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(set.at(i1)));
            i1 += 1;
        }
        out.push(']');
        out
    });
            i += 1;
        }
        out.push('}');
        out
    },
            ")"
        );
    }
}

#[derive(Clone)]
pub struct WidthIndependentParagraphAnnotation {
    pub text: String,
    pub font_size: f64,
    pub style_at: Arc<dyn Fn(u32) -> TextStyle + Send + Sync>,
    pub font_size_at: Arc<dyn Fn(u32) -> f64 + Send + Sync>,
    pub bopomofo_font_weight_at: Arc<dyn Fn(u32) -> u32 + Send + Sync>,
    pub ruby_font_size: f64,
    pub ruby_stack_gap: f64,
    pub ruby_font_weight: u32,
    pub pinyin_spans: Vec<RubySpan>,
    pub clreq_profile: ClreqProfile,
    pub punctuation_glyph_substitutor: ClreqPunctuationGlyphSubstitutor,
    pub quote_pairs: Vec<QuotePair>,
    pub role_override_infos: Vec<RoleOverrideInfo>,
    pub font_decisions: Vec<FontDecision>,
    pub cluster_ranges: Vec<ResolvedClusterRange>,
    pub font_decision_by_range: SortedMapTable<TextRange, FontDecision>,
    pub inline_object_by_range: SortedMapTable<TextRange, InlineObjectSpan>,
    pub segment_shaping_cache: SortedMapTable<TextRange, ShapingResult>,
    pub substitution_rollbacks: SortedMapTable<TextRange, String>,
    pub ruby_font_geometry_by_span: SortedMapTable<RubySpan, RubyFontGeometry>,
    pub base_shaping_stage: ParagraphShapingStageResult,
}

impl WidthIndependentParagraphAnnotation {
    pub fn new(text: &str, font_size: f64, style_at: Arc<dyn Fn(u32) -> TextStyle + Send + Sync>, font_size_at: Arc<dyn Fn(u32) -> f64 + Send + Sync>, bopomofo_font_weight_at: Arc<dyn Fn(u32) -> u32 + Send + Sync>, ruby_font_size: f64, ruby_stack_gap: f64, ruby_font_weight: u32,
pinyin_spans: Vec<RubySpan>, clreq_profile: ClreqProfile, punctuation_glyph_substitutor: ClreqPunctuationGlyphSubstitutor, quote_pairs: Vec<QuotePair>, role_override_infos: Vec<RoleOverrideInfo>, font_decisions: Vec<FontDecision>, cluster_ranges: Vec<ResolvedClusterRange>,
font_decision_by_range: SortedMapTable<TextRange, FontDecision>, inline_object_by_range: SortedMapTable<TextRange, InlineObjectSpan>, segment_shaping_cache: SortedMapTable<TextRange, ShapingResult>, substitution_rollbacks: SortedMapTable<TextRange, String>,
ruby_font_geometry_by_span: SortedMapTable<RubySpan, RubyFontGeometry>, base_shaping_stage: ParagraphShapingStageResult) -> Self {
        Self {
            text: text.to_string(),
            font_size,
            style_at,
            font_size_at,
            bopomofo_font_weight_at,
            ruby_font_size,
            ruby_stack_gap,
            ruby_font_weight,
            pinyin_spans,
            clreq_profile,
            punctuation_glyph_substitutor,
            quote_pairs,
            role_override_infos,
            font_decisions,
            cluster_ranges,
            font_decision_by_range,
            inline_object_by_range,
            segment_shaping_cache,
            substitution_rollbacks,
            ruby_font_geometry_by_span,
            base_shaping_stage,
        }
    }
}

pub trait WidthIndependentAnnotationCache: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn WidthIndependentAnnotationCache>;
    fn get_size(&self) -> u32;
    fn get(&mut self, key: WidthIndependentAnnotationKey) -> Option<WidthIndependentParagraphAnnotation>;
    fn put(&mut self, key: WidthIndependentAnnotationKey, annotation: WidthIndependentParagraphAnnotation);
    fn clear(&mut self);
}

impl Clone for Box<dyn WidthIndependentAnnotationCache> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn WidthIndependentAnnotationCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

#[derive(Clone)]
pub struct LruWidthIndependentAnnotationCache {
    pub max_entries: u32,
    pub(crate) keys: Vec<WidthIndependentAnnotationKey>,
    pub(crate) values: Vec<WidthIndependentParagraphAnnotation>,
}

impl LruWidthIndependentAnnotationCache {
    pub fn new(max_entries: u32) -> Self {
        Self {
            max_entries,
            keys: vec![],
            values: vec![],
        }
    }

    pub fn get(&mut self, key: WidthIndependentAnnotationKey) -> Option<WidthIndependentParagraphAnnotation> {
        let mut found_index = 4294967295u32;
        for i in 0..match u32::try_from(self.keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_key_equals((self.keys[usize::try_from(i).unwrap_or(0)]).clone(), (key).clone()) {
                found_index = i;
                break;
            }
        }
        if found_index > 2147483647 {
            return None;
        }
        let k = (self.keys[usize::try_from(found_index).unwrap_or(0)]).clone();
        let v = (self.values[usize::try_from(found_index).unwrap_or(0)]).clone();
        { let _a = &mut (self.keys); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes((found_index).to_ne_bytes()); let splice_index = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos }; let
_len = 1i32; let splice_count = if _len < 0 { 0 } else { let _rest = _n - splice_index; if _len < _rest { _len } else { _rest } }; let splice_removed: Vec<_> = _a.drain(usize::try_from(splice_index).unwrap_or(0)..usize::try_from(splice_index +
splice_count).unwrap_or(0)).collect(); splice_removed };
        { let _a = &mut (self.values); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes((found_index).to_ne_bytes()); let splice_index1 = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos };
let _len = 1i32; let splice_count1 = if _len < 0 { 0 } else { let _rest = _n - splice_index1; if _len < _rest { _len } else { _rest } }; let splice_removed1: Vec<_> = _a.drain(usize::try_from(splice_index1).unwrap_or(0)..usize::try_from(splice_index1 +
splice_count1).unwrap_or(0)).collect(); splice_removed1 };
        self.keys.push(k.clone());
        self.values.push(v.clone());
        return Some(v);
    }

    pub fn put(&mut self, key: WidthIndependentAnnotationKey, annotation: WidthIndependentParagraphAnnotation) {
        let mut found_index = 4294967295u32;
        for i in 0..match u32::try_from(self.keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_key_equals((self.keys[usize::try_from(i).unwrap_or(0)]).clone(), (key).clone()) {
                found_index = i;
                break;
            }
        }
        if found_index <= 2147483647 {
            { let _a = &mut (self.keys); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes((found_index).to_ne_bytes()); let splice_index2 = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos };
let _len = 1i32; let splice_count2 = if _len < 0 { 0 } else { let _rest = _n - splice_index2; if _len < _rest { _len } else { _rest } }; let splice_removed2: Vec<_> = _a.drain(usize::try_from(splice_index2).unwrap_or(0)..usize::try_from(splice_index2 +
splice_count2).unwrap_or(0)).collect(); splice_removed2 };
            { let _a = &mut (self.values); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes((found_index).to_ne_bytes()); let splice_index3 = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos
}; let _len = 1i32; let splice_count3 = if _len < 0 { 0 } else { let _rest = _n - splice_index3; if _len < _rest { _len } else { _rest } }; let splice_removed3: Vec<_> = _a.drain(usize::try_from(splice_index3).unwrap_or(0)..usize::try_from(splice_index3 +
splice_count3).unwrap_or(0)).collect(); splice_removed3 };
        } else {
            if i32::from_ne_bytes((u32::try_from((self.keys.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) >= i32::from_ne_bytes((self.max_entries).to_ne_bytes()) {
                { if self.keys.is_empty() { None } else { Some(self.keys.remove(0)) } };
                { if self.values.is_empty() { None } else { Some(self.values.remove(0)) } };
            }
        }
        self.keys.push(key.clone());
        self.values.push(annotation.clone());
    }

    pub fn clear(&mut self) {
        while (i32::from_ne_bytes((u32::try_from((self.keys.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) {
            self.keys.pop();
            self.values.pop();
        }
    }

    pub fn get_size(&self) -> u32 {
        return u32::try_from(((self.keys).clone().len()) & 0xFFFF_FFFF).unwrap_or(0);
    }

    pub(crate) fn lru_width_independent_annotation_cache_text_range_equals(a: TextRange, b: TextRange) -> bool {
        return a == b || true && true && (a.start == b.start && a.end == b.end);
    }

    pub(crate) fn lru_width_independent_annotation_cache_text_style_equals(a: TextStyle, b: TextStyle) -> bool {
        if a == b {
            return true;
        }
        if false || false {
            return false;
        }
        return a.font_size == b.font_size && (a.locale).to_string() == (b.locale).to_string() && a.font_weight == b.font_weight && a.italic == b.italic && a.baseline_shift == b.baseline_shift && a.inline_attachment == b.inline_attachment &&
LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_strings_equal(&a.font_families, &b.font_families);
    }

    pub(crate) fn lru_width_independent_annotation_cache_text_span_equals(a: TextSpan, b: TextSpan) -> bool {
        return a == b || true && true && LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_text_range_equals((a.range).clone(), (b.range).clone()) &&
LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_text_style_equals((a.style).clone(), (b.style).clone());
    }

    pub(crate) fn lru_width_independent_annotation_cache_line_break_span_equals(a: LineBreakSpan, b: LineBreakSpan) -> bool {
        return a == b || true && true && LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_text_range_equals((a.range).clone(), (b.range).clone()) && a.policy == b.policy;
    }

    pub(crate) fn lru_width_independent_annotation_cache_decoration_equals(a: DecorationSpan, b: DecorationSpan) -> bool {
        return a == b || true && true && LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_text_range_equals((a.range).clone(), (b.range).clone()) && a.kind == b.kind;
    }

    pub(crate) fn lru_width_independent_annotation_cache_ruby_equals(a: RubySpan, b: RubySpan) -> bool {
        if a == b {
            return true;
        }
        if false || false || !LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_text_range_equals((a.base_range).clone(), (b.base_range).clone()) || (a.text).to_string() != (b.text).to_string() || a.kind != b.kind || a.locale != b.locale {
            return false;
        }
        return LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_strings_equal(&a.font_families, &b.font_families);
    }

    pub(crate) fn lru_width_independent_annotation_cache_inline_box_equals(a: InlineBoxSpan, b: InlineBoxSpan) -> bool {
        return a == b || true && true && LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_text_range_equals((a.range).clone(), (b.range).clone()) && a.inline_start == b.inline_start && a.inline_end == b.inline_end && a.outer_spacing == b.outer_spacing;
    }

    pub(crate) fn lru_width_independent_annotation_cache_preferred_stretch_equals(a: InlineObjectPreferredStretch, b: InlineObjectPreferredStretch) -> bool {
        if a == b {
            return true;
        }
        if false || false {
            return false;
        }
        return a.kind == b.kind && a.natural_width == b.natural_width && a.target_width == b.target_width;
    }

    pub(crate) fn lru_width_independent_annotation_cache_boundary_adjustment_equals(a: InlineObjectBoundaryAdjustment, b: InlineObjectBoundaryAdjustment) -> bool {
        if a == b {
            return true;
        }
        if false || false {
            return false;
        }
        return a.participates_in_uniform_stretch == b.participates_in_uniform_stretch && LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_preferred_stretch_equals((a.preferred_stretch).as_ref().unwrap().clone(),
(b.preferred_stretch).as_ref().unwrap().clone()) && a.shrink_capacity == b.shrink_capacity && a.line_end_discardable_advance == b.line_end_discardable_advance && a.prevents_line_break == b.prevents_line_break;
    }

    pub(crate) fn lru_width_independent_annotation_cache_inline_object_equals(a: InlineObjectSpan, b: InlineObjectSpan) -> bool {
        return a == b || true && true && LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_text_range_equals((a.range).clone(), (b.range).clone()) && a.advance == b.advance && a.ascent == b.ascent && a.descent == b.descent &&
LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_boundary_adjustment_equals((a.leading_boundary).clone(), (b.leading_boundary).clone()) &&
LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_boundary_adjustment_equals((a.trailing_boundary).clone(), (b.trailing_boundary).clone());
    }

    pub(crate) fn lru_width_independent_annotation_cache_strings_equal(a: &[String], b: &[String]) -> bool {
        if a == b {
            return true;
        }
        if false || false || u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return false;
        }
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if a[usize::try_from(i).unwrap_or(0)].clone() != (b[usize::try_from(i).unwrap_or(0)]).clone() {
                return false;
            }
        }
        return true;
    }

    pub(crate) fn lru_width_independent_annotation_cache_ints_equal(a: &[u32], b: &[u32]) -> bool {
        if a == b {
            return true;
        }
        if false || false || u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return false;
        }
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if a[usize::try_from(i).unwrap_or(0)] != b[usize::try_from(i).unwrap_or(0)] {
                return false;
            }
        }
        return true;
    }

    pub(crate) fn lru_width_independent_annotation_cache_map_equals(a: SortedMapTable<TextRange, SortedSetTable<u32>>, b: SortedMapTable<TextRange, SortedSetTable<u32>>) -> bool {
        if a == b {
            return true;
        }
        if false || false || u32::from_ne_bytes((a.size()).to_ne_bytes()) != u32::from_ne_bytes((b.size()).to_ne_bytes()) {
            return false;
        }
        for i in 0..u32::from_ne_bytes((a.size()).to_ne_bytes()) {
            let ak = a.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let mut matched = false;
            for j in 0..u32::from_ne_bytes((b.size()).to_ne_bytes()) {
                let bk = b.key_at({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) });
                if LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_text_range_equals((ak).clone(), (bk).clone()) {
                    let av: SortedSetTable<u32> = a.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
                    let bv: SortedSetTable<u32> = b.value_at({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) });
                    if av == bv {
                        matched = true;
                    } else {
                        if false || false || u32::from_ne_bytes((av.size()).to_ne_bytes()) != u32::from_ne_bytes((bv.size()).to_ne_bytes()) {
                            return false;
                        } else {
                            matched = true;
                            for k in 0..u32::from_ne_bytes((av.size()).to_ne_bytes()) {
                                if av.at({ let v: u32 = k; i32::from_ne_bytes(v.to_ne_bytes()) }) != bv.at({ let v: u32 = k; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                                    matched = false;
                                    break;
                                }
                            }
                        }
                    }
                    break;
                }
            }
            if !matched {
                return false;
            }
        }
        return true;
    }

    pub(crate) fn lru_width_independent_annotation_cache_key_equals(a: WidthIndependentAnnotationKey, b: WidthIndependentAnnotationKey) -> bool {
        if a == b {
            return true;
        }
        if false || false || (a.text).to_string() != (b.text).to_string() || ((a.profile_id).clone().value).to_string() != ((b.profile_id).clone().value).to_string() || a.emphasis_dot_gap_em != b.emphasis_dot_gap_em ||
!LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_text_style_equals((a.text_style).clone(), (b.text_style).clone()) {
            return false;
        }
        if u32::try_from((a.spans.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.spans.len()) & 0xFFFF_FFFF).unwrap_or(0) || u32::try_from((a.line_break_spans.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.line_break_spans.len()) & 0xFFFF_FFFF).unwrap_or(0) ||
u32::try_from((a.decorations.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.decorations.len()) & 0xFFFF_FFFF).unwrap_or(0) || u32::try_from((a.ruby_spans.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.ruby_spans.len()) & 0xFFFF_FFFF).unwrap_or(0) ||
u32::try_from((a.inline_boxes.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.inline_boxes.len()) & 0xFFFF_FFFF).unwrap_or(0) || u32::try_from((a.inline_objects.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.inline_objects.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return false;
        }
        for i in 0..match u32::try_from(a.spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_text_span_equals((a.spans[usize::try_from(i).unwrap_or(0)]).clone(), (b.spans[usize::try_from(i).unwrap_or(0)]).clone()) {
                return false;
            }
        }
        for i in 0..match u32::try_from(a.line_break_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_line_break_span_equals((a.line_break_spans[usize::try_from(i).unwrap_or(0)]).clone(), (b.line_break_spans[usize::try_from(i).unwrap_or(0)]).clone()) {
                return false;
            }
        }
        for i in 0..match u32::try_from(a.decorations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_decoration_equals((a.decorations[usize::try_from(i).unwrap_or(0)]).clone(), (b.decorations[usize::try_from(i).unwrap_or(0)]).clone()) {
                return false;
            }
        }
        for i in 0..match u32::try_from(a.ruby_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_ruby_equals((a.ruby_spans[usize::try_from(i).unwrap_or(0)]).clone(), (b.ruby_spans[usize::try_from(i).unwrap_or(0)]).clone()) {
                return false;
            }
        }
        for i in 0..match u32::try_from(a.inline_boxes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_inline_box_equals((a.inline_boxes[usize::try_from(i).unwrap_or(0)]).clone(), (b.inline_boxes[usize::try_from(i).unwrap_or(0)]).clone()) {
                return false;
            }
        }
        for i in 0..match u32::try_from(a.inline_objects.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_inline_object_equals((a.inline_objects[usize::try_from(i).unwrap_or(0)]).clone(), (b.inline_objects[usize::try_from(i).unwrap_or(0)]).clone()) {
                return false;
            }
        }
        return LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_ints_equal(&a.source_boundaries, &b.source_boundaries) && LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_map_equals((a.rejected_technical_tiers_by_span).clone(),
(b.rejected_technical_tiers_by_span).clone());
    }
}

impl WidthIndependentAnnotationCache for LruWidthIndependentAnnotationCache {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.WidthIndependentAnnotationCache.LruWidthIndependentAnnotationCache"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn WidthIndependentAnnotationCache> {
        Box::new(self.clone())
    }

    fn get(&mut self, key: WidthIndependentAnnotationKey) -> Option<WidthIndependentParagraphAnnotation> {
        let mut found_index = 4294967295u32;
        for i in 0..match u32::try_from(self.keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_key_equals((self.keys[usize::try_from(i).unwrap_or(0)]).clone(), (key).clone()) {
                found_index = i;
                break;
            }
        }
        if found_index > 2147483647 {
            return None;
        }
        let k = (self.keys[usize::try_from(found_index).unwrap_or(0)]).clone();
        let v = (self.values[usize::try_from(found_index).unwrap_or(0)]).clone();
        { let _a = &mut (self.keys); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes((found_index).to_ne_bytes()); let splice_index4 = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos }; let
_len = 1i32; let splice_count4 = if _len < 0 { 0 } else { let _rest = _n - splice_index4; if _len < _rest { _len } else { _rest } }; let splice_removed4: Vec<_> = _a.drain(usize::try_from(splice_index4).unwrap_or(0)..usize::try_from(splice_index4 +
splice_count4).unwrap_or(0)).collect(); splice_removed4 };
        { let _a = &mut (self.values); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes((found_index).to_ne_bytes()); let splice_index5 = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos };
let _len = 1i32; let splice_count5 = if _len < 0 { 0 } else { let _rest = _n - splice_index5; if _len < _rest { _len } else { _rest } }; let splice_removed5: Vec<_> = _a.drain(usize::try_from(splice_index5).unwrap_or(0)..usize::try_from(splice_index5 +
splice_count5).unwrap_or(0)).collect(); splice_removed5 };
        self.keys.push(k.clone());
        self.values.push(v.clone());
        return Some(v);
    }

    fn put(&mut self, key: WidthIndependentAnnotationKey, annotation: WidthIndependentParagraphAnnotation) {
        let mut found_index = 4294967295u32;
        for i in 0..match u32::try_from(self.keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if LruWidthIndependentAnnotationCache::lru_width_independent_annotation_cache_key_equals((self.keys[usize::try_from(i).unwrap_or(0)]).clone(), (key).clone()) {
                found_index = i;
                break;
            }
        }
        if found_index <= 2147483647 {
            { let _a = &mut (self.keys); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes((found_index).to_ne_bytes()); let splice_index6 = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos };
let _len = 1i32; let splice_count6 = if _len < 0 { 0 } else { let _rest = _n - splice_index6; if _len < _rest { _len } else { _rest } }; let splice_removed6: Vec<_> = _a.drain(usize::try_from(splice_index6).unwrap_or(0)..usize::try_from(splice_index6 +
splice_count6).unwrap_or(0)).collect(); splice_removed6 };
            { let _a = &mut (self.values); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes((found_index).to_ne_bytes()); let splice_index7 = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos
}; let _len = 1i32; let splice_count7 = if _len < 0 { 0 } else { let _rest = _n - splice_index7; if _len < _rest { _len } else { _rest } }; let splice_removed7: Vec<_> = _a.drain(usize::try_from(splice_index7).unwrap_or(0)..usize::try_from(splice_index7 +
splice_count7).unwrap_or(0)).collect(); splice_removed7 };
        } else {
            if i32::from_ne_bytes((u32::try_from((self.keys.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) >= i32::from_ne_bytes((self.max_entries).to_ne_bytes()) {
                { if self.keys.is_empty() { None } else { Some(self.keys.remove(0)) } };
                { if self.values.is_empty() { None } else { Some(self.values.remove(0)) } };
            }
        }
        self.keys.push(key.clone());
        self.values.push(annotation.clone());
    }

    fn clear(&mut self) {
        while (i32::from_ne_bytes((u32::try_from((self.keys.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) {
            self.keys.pop();
            self.values.pop();
        }
    }

    fn get_size(&self) -> u32 {
        return u32::try_from(((self.keys).clone().len()) & 0xFFFF_FFFF).unwrap_or(0);
    }
}

#[derive(Clone, Copy)]
pub struct WidthIndependentAnnotationCacheFns;

impl WidthIndependentAnnotationCacheFns {
    pub(crate) fn width_independent_annotation_cache_fns_render_nullable_last_line_alignment(value: Option<LastLineAlignment>) -> String {
        return match &(value) { None => "null".to_string(), Some(__option) => WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_render_last_line_alignment(*__option).to_string() };
    }

    pub(crate) fn width_independent_annotation_cache_fns_render_last_line_alignment(value: LastLineAlignment) -> String {
        return value.name().to_string();
    }

    pub(crate) fn width_independent_annotation_cache_fns_copy_font_families(families: &[String]) -> Vec<String> {
        let capacity = families.len();
        let mut result = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(families.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            result.push((families[usize::try_from(i).unwrap_or(0)]).clone());
        }
        return result;
    }

    pub(crate) fn width_independent_annotation_cache_fns_is_fixed_boundary(b: InlineObjectBoundaryAdjustment) -> bool {
        return !b.participates_in_uniform_stretch && b.preferred_stretch.is_none() && b.shrink_capacity == 0.0f64 && b.line_end_discardable_advance == 0.0f64 && !b.prevents_line_break;
    }

    pub fn width_independent_annotation_cache_fns_is_contained_in(r: TextRange, other: TextRange) -> bool {
        return (i32::from_ne_bytes((r.start).to_ne_bytes())) >= i32::from_ne_bytes((other.start).to_ne_bytes()) && (i32::from_ne_bytes((r.end).to_ne_bytes())) <= i32::from_ne_bytes((other.end).to_ne_bytes());
    }

    pub fn width_independent_annotation_cache_fns_containing_font_decisions(clusters: &Vec<Cluster>, items: &Vec<FontDecision>) -> Vec<Option<FontDecision>> {
        let mut item_index = 0u32;
        let mut result: Vec<Option<FontDecision>> = Vec::new();
        for cluster in clusters {
            while (i32::from_ne_bytes((item_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) &&
(i32::from_ne_bytes((((items[usize::try_from(item_index).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())) <= i32::from_ne_bytes(((cluster.range).clone().start).to_ne_bytes()) {
                item_index = u32::wrapping_add(item_index, 1);
            }
            if i32::from_ne_bytes((item_index).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                let item = (items[usize::try_from(item_index).unwrap_or(0)]).clone();
                if i32::from_ne_bytes(((cluster.range).clone().start).to_ne_bytes()) >= i32::from_ne_bytes(((item.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes(((cluster.range).clone().end).to_ne_bytes())) <=
i32::from_ne_bytes(((item.range).clone().end).to_ne_bytes()) {
                    result.push(Some(item));
                } else {
                    result.push(None);
                }
            } else {
                result.push(None);
            }
        }
        return result;
    }

    pub fn width_independent_annotation_cache_fns_first_contained_atom(clusters: &Vec<Cluster>, items: &Vec<PunctuationAtom>) -> Vec<Option<PunctuationAtom>> {
        let mut item_index = 0u32;
        let mut result: Vec<Option<PunctuationAtom>> = Vec::new();
        for cluster in clusters {
            while (i32::from_ne_bytes((item_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) &&
(i32::from_ne_bytes((((items[usize::try_from(item_index).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())) <= i32::from_ne_bytes(((cluster.range).clone().start).to_ne_bytes()) {
                item_index = u32::wrapping_add(item_index, 1);
            }
            if i32::from_ne_bytes((item_index).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                let item = (items[usize::try_from(item_index).unwrap_or(0)]).clone();
                if i32::from_ne_bytes(((item.range).clone().start).to_ne_bytes()) >= i32::from_ne_bytes(((cluster.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes(((item.range).clone().end).to_ne_bytes())) <=
i32::from_ne_bytes(((cluster.range).clone().end).to_ne_bytes()) {
                    result.push(Some(item));
                } else {
                    result.push(None);
                }
            } else {
                result.push(None);
            }
        }
        return result;
    }

    pub(crate) fn width_independent_annotation_cache_fns_quote_to_role_override_infos(decisions: &Vec<QuoteRoleDecision>, text: &str, base_classifier: Box<dyn FontRoleClassifier>, context: FontRoleContext) -> Result<Vec<RoleOverrideInfo>, TextRangeError> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let capacity = decisions.len();
        let mut sorted = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            sorted.push((decisions[usize::try_from(i).unwrap_or(0)]).clone());
        }
        let mut r_idx = 1u32;
        while (i32::from_ne_bytes((r_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((sorted.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let curr = (sorted[usize::try_from(r_idx).unwrap_or(0)]).clone();
            let mut j = r_idx;
            while (j) > (0) && (i32::from_ne_bytes((sorted[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)].index).to_ne_bytes())) > (i32::from_ne_bytes((curr.index).to_ne_bytes())) {
                sorted[usize::try_from(j).unwrap_or(0)] = (sorted[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]).clone();
                j = u32::wrapping_sub(j, 1);
            }
            sorted[usize::try_from(j).unwrap_or(0)] = curr;
            r_idx = u32::wrapping_add(r_idx, 1);
        }
        let capacity = sorted.len();
        let mut result = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(sorted.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let decision = (sorted[usize::try_from(i).unwrap_or(0)]).clone();
            let index = decision.index;
            let end_idx = if i32::from_ne_bytes((u32::wrapping_add(index, 1)).to_ne_bytes()) < (i32::from_ne_bytes((__count).to_ne_bytes())) { u32::wrapping_add(index, 1) } else { __count };
            let source_text = u_string::substring(&text, i32::from_ne_bytes((index).to_ne_bytes()), { let v: u32 = end_idx; i32::from_ne_bytes(v.to_ne_bytes()) });
            let original_role = base_classifier.classify(text, TextRange::new(index, u32::wrapping_add(index, 1))?, Some((context).clone()));
            result.push(RoleOverrideInfo::new(TextRange::new(index, u32::wrapping_add(index, 1))?, source_text.as_str(), original_role.name().to_string().as_str(), decision.role.name().to_string().as_str(), (decision.source).to_string().as_str(),
(decision.reason).to_string().as_str()));
        }
        return Ok(result);
    }

    pub fn width_independent_annotation_cache_fns_to_width_independent_annotation_key(input: LayoutInput, rejected_technical_tiers_by_span: Option<SortedMapTable<TextRange, SortedSetTable<u32>>>) -> WidthIndependentAnnotationKey {
        let tiers = match &(rejected_technical_tiers_by_span) { Some(__option1) => (*__option1).clone(), None => SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range)).clone().build() };
        return WidthIndependentAnnotationKey::new(((input.content).clone().text).to_string().as_str(), ((input.content).clone().spans).clone(), ((input.content).clone().line_break_spans).clone(), ((input.content).clone().source_boundaries).clone(), (input.text_style).clone(),
(input.decorations).clone(), (input.ruby_spans).clone(), (input.inline_boxes).clone(), (input.inline_objects).clone(), (input.profile_id).clone(), (input.paragraph_style).clone().emphasis_dot_gap_em, (tiers).clone());
    }

    pub(crate) fn width_independent_annotation_cache_fns_clamp_int(val: u32, min: u32, max: u32) -> u32 {
        return if i32::from_ne_bytes((val).to_ne_bytes()) < (i32::from_ne_bytes((min).to_ne_bytes())) { min } else { if i32::from_ne_bytes((val).to_ne_bytes()) > (i32::from_ne_bytes((max).to_ne_bytes())) { max } else { val } };
    }

    pub(crate) fn width_independent_annotation_cache_fns_is_inline_stop(code: u32) -> bool {
        return code == 12290 || code == 65281 || code == 65311 || code == 65294;
    }

    pub fn width_independent_annotation_cache_fns_prepare_width_independent_annotation(engine: &mut ExplainableStubParagraphLayoutEngine, input: LayoutInput, rejected_technical_tiers_by_span: SortedMapTable<TextRange, SortedSetTable<u32>>) ->
Result<WidthIndependentParagraphAnnotation, ParagraphShapingStageShapeParagraphFault> {
        let text = ((input.content).clone().text).to_string();
        let font_size = (input.text_style).clone().font_size;
        let mut inline_object_by_range_builder: SortedMapTableBuilder<TextRange, InlineObjectSpan> = SortedTable::sorted_table_map_builder::<TextRange, InlineObjectSpan>(Arc::new(compare_text_range));
        for i in 0..match u32::try_from(input.inline_objects.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let obj = (input.inline_objects[usize::try_from(i).unwrap_or(0)]).clone();
            inline_object_by_range_builder.put(&((obj.range).clone()), &(obj));
        }
        let inline_object_by_range: SortedMapTable<TextRange, InlineObjectSpan> = inline_object_by_range_builder.clone().build();
        let sized_spans: Arc<Mutex<Vec<TextSpan>>> = Arc::new(Mutex::new(Vec::new()));
        let mut __loop_guard = sized_spans.lock().unwrap();
        for i in 0..match u32::try_from((input.content).clone().spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let sp = ((input.content).clone().spans[usize::try_from(i).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((sp.range).clone().start).to_ne_bytes()) < (i32::from_ne_bytes(((sp.range).clone().end).to_ne_bytes())) {
                __loop_guard.push(sp.clone());
            }
        }
        drop(__loop_guard);
        let style_at: Arc<dyn Fn(u32) -> TextStyle + Send + Sync + 'static> = { let sized_spans = Arc::clone(&sized_spans); let input = (input).clone(); Arc::new({ let sized_spans = Arc::clone(&sized_spans); move |offset| {
        let mut last_span: Option<TextSpan> = None;
        let mut __loop_guard1 = sized_spans.lock().unwrap();
        let mut __loop_i = 0u32;
        loop {
            let __loop_len = match u32::try_from(__loop_guard1.len()) { Ok(value) => value, Err(_) => u32::MAX };
            if __loop_i >= __loop_len { break; }
            let sp = (__loop_guard1[usize::try_from(__loop_i).unwrap_or(0)]).clone();
            if i32::from_ne_bytes((offset).to_ne_bytes()) >= i32::from_ne_bytes(((sp.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes((offset).to_ne_bytes())) < (i32::from_ne_bytes(((sp.range).clone().end).to_ne_bytes())) {
                last_span = Some(sp.clone());
            }
            __loop_i += 1;
        }
        drop(__loop_guard1);
        return match &(last_span) { Some(__option2) => (__option2.style).clone(), None => (input.text_style).clone() };
} }) };
        let font_size_at: Arc<dyn Fn(u32) -> f64 + Send + Sync + 'static> = { let style_at = (style_at).clone(); Arc::new(move |offset| {
        return style_at(offset).font_size;
}) };
        let emphasis_ranges: Arc<Mutex<Vec<TextRange>>> = Arc::new(Mutex::new(Vec::new()));
        let mut __loop_guard2 = emphasis_ranges.lock().unwrap();
        for i in 0..match u32::try_from(input.decorations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (input.decorations[usize::try_from(i).unwrap_or(0)]).clone();
            if d.kind == DecorationKind::Emphasis {
                __loop_guard2.push((d.range).clone());
            }
        }
        drop(__loop_guard2);
        let emphasis_italic_at: Arc<dyn Fn(u32) -> bool + Send + Sync + 'static> = { let emphasis_ranges = Arc::clone(&emphasis_ranges); Arc::new({ let emphasis_ranges = Arc::clone(&emphasis_ranges); move |offset| {
        let mut __loop_guard3 = emphasis_ranges.lock().unwrap();
        let mut __loop_i1 = 0u32;
        loop {
            let __loop_len1 = match u32::try_from(__loop_guard3.len()) { Ok(value) => value, Err(_) => u32::MAX };
            if __loop_i1 >= __loop_len1 { break; }
            let r = (__loop_guard3[usize::try_from(__loop_i1).unwrap_or(0)]).clone();
            if i32::from_ne_bytes((offset).to_ne_bytes()) >= i32::from_ne_bytes((r.start).to_ne_bytes()) && (i32::from_ne_bytes((offset).to_ne_bytes())) < (i32::from_ne_bytes((r.end).to_ne_bytes())) {
                return true;
            }
            __loop_i1 += 1;
        }
        drop(__loop_guard3);
        return false;
} }) };
        let ruby_font_size = font_size * 0.5f64;
        let ruby_stack_gap = font_size * 0.0f64;
        let ruby_font_weight = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_clamp_int(u32::wrapping_add((input.text_style).clone().font_weight, 100), 1, 900);
        let bopomofo_font_weight_at: Arc<dyn Fn(u32) -> u32 + Send + Sync + 'static> = { let style_at = (style_at).clone(); Arc::new(move |offset| {
        return WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_clamp_int(u32::wrapping_add(style_at(offset).font_weight, 300), 1, 900);
}) };
        let mut pinyin_spans: Vec<RubySpan> = Vec::new();
        for i in 0..match u32::try_from(input.ruby_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let rs = (input.ruby_spans[usize::try_from(i).unwrap_or(0)]).clone();
            if rs.kind == RubyKind::Pinyin {
                pinyin_spans.push(rs.clone());
            }
        }
        let span_boundaries_builder: Arc<Mutex<SortedSetTableBuilder<u32>>> = Arc::new(Mutex::new(SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))))));
        let add_span_boundary: Arc<dyn Fn(u32) -> () + Send + Sync + 'static> = { let text = (text).clone(); let span_boundaries_builder = (span_boundaries_builder).clone(); Arc::new({ let span_boundaries_builder = Arc::clone(&span_boundaries_builder); move |offset| {
        if i32::from_ne_bytes((offset).to_ne_bytes()) > (0) && (i32::from_ne_bytes((offset).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&(text))).to_ne_bytes())) {
            span_boundaries_builder.lock().unwrap().put(&(offset));
        }
} }) };
        let add_span_range: Arc<dyn Fn(TextRange) -> () + Send + Sync + 'static> = { let add_span_boundary = (add_span_boundary).clone(); Arc::new(move |range| {
        add_span_boundary(range.start);
        add_span_boundary(range.end);
}) };
        let mut __loop_guard4 = sized_spans.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard4.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            add_span_range(((__loop_guard4[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        drop(__loop_guard4);
        for i in 0..match u32::try_from(input.decorations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            add_span_range(((input.decorations[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        for i in 0..match u32::try_from(input.ruby_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            add_span_range(((input.ruby_spans[usize::try_from(i).unwrap_or(0)]).clone().base_range).clone());
        }
        for i in 0..match u32::try_from(input.inline_boxes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            add_span_range(((input.inline_boxes[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        for i in 0..match u32::try_from(input.inline_objects.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            add_span_range(((input.inline_objects[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        for i in 0..match u32::try_from((input.content).clone().line_break_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            add_span_range((((input.content).clone().line_break_spans[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        for i in 0..match u32::try_from((input.content).clone().source_boundaries.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            add_span_boundary((input.content).clone().source_boundaries[usize::try_from(i).unwrap_or(0)]);
        }
        let span_boundaries: SortedSetTable<u32> = span_boundaries_builder.lock().unwrap().clone().build();
        let emoji_shaping_boundaries_builder: Arc<Mutex<SortedSetTableBuilder<u32>>> = Arc::new(Mutex::new(SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))))));
        let add_emoji_boundary: Arc<dyn Fn(u32) -> () + Send + Sync + 'static> = { let text = (text).clone(); let emoji_shaping_boundaries_builder = (emoji_shaping_boundaries_builder).clone(); Arc::new({ let emoji_shaping_boundaries_builder =
Arc::clone(&emoji_shaping_boundaries_builder); move |offset| {
        if i32::from_ne_bytes((offset).to_ne_bytes()) > (0) && (i32::from_ne_bytes((offset).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&(text))).to_ne_bytes())) {
            emoji_shaping_boundaries_builder.lock().unwrap().put(&(offset));
        }
} }) };
        let add_emoji_range: Arc<dyn Fn(TextRange) -> () + Send + Sync + 'static> = { let add_emoji_boundary = (add_emoji_boundary).clone(); Arc::new(move |range| {
        add_emoji_boundary(range.start);
        add_emoji_boundary(range.end);
}) };
        let mut __loop_guard5 = sized_spans.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard5.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            add_emoji_range(((__loop_guard5[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        drop(__loop_guard5);
        for i in 0..match u32::try_from(input.inline_boxes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let r#box = (input.inline_boxes[usize::try_from(i).unwrap_or(0)]).clone();
            if r#box.inline_start != 0.0f64 || r#box.inline_end != 0.0f64 || r#box.outer_spacing == InlineBoxOuterSpacing::Narrow {
                add_emoji_range((r#box.range).clone());
            }
        }
        for i in 0..match u32::try_from(input.inline_objects.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            add_emoji_range(((input.inline_objects[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        let emoji_shaping_boundaries: SortedSetTable<u32> = emoji_shaping_boundaries_builder.lock().unwrap().clone().build();
        let clreq_profile = engine.clreq_profile_resolver.resolve((input.profile_id).clone());
        let context = FontRoleContext::new(Some(((input.text_style).clone().locale).to_string()), Some(clreq_profile.region.name().to_string()));
        let punctuation_glyph_substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(clreq_profile.punctuation_glyph_policy));
        let quote_pairs = engine.quote_pair_analyzer.analyze(text.as_str()).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
        let quote_role_decisions = engine.quote_pair_analyzer.classify_quote_roles(text.as_str(), &quote_pairs, Some((context).clone())).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
        let mut quote_role_overrides_builder: SortedMapTableBuilder<u32, FontRole> = SortedTable::sorted_table_map_builder::<u32, FontRole>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(quote_role_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            quote_role_overrides_builder.put(&(quote_role_decisions[usize::try_from(i).unwrap_or(0)].index), &(quote_role_decisions[usize::try_from(i).unwrap_or(0)].role));
        }
        let quote_role_overrides: SortedMapTable<u32, FontRole> = quote_role_overrides_builder.clone().build();
        let quote_role_override_infos = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_quote_to_role_override_infos(&quote_role_decisions, text.as_str(), engine.font_role_classifier.clone(), (context).clone()).map_err(|e|
ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
        let quote_aware_classifier: Box<dyn FontRoleClassifier> = if i32::from_ne_bytes((u32::from_ne_bytes((quote_role_overrides.size()).to_ne_bytes())).to_ne_bytes()) > (0) { Box::new(QuotePairAwareFontRoleClassifier::new(engine.font_role_classifier.clone(),
(quote_role_overrides).clone())) } else { engine.font_role_classifier.clone() };
        let dash_ellipsis_role_decisions = ContextualDashEllipsisRoleResolver::new().map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?.resolve(text.as_str(), Some((context).clone())).map_err(|e|
ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
        let dash_ellipsis_role_override_infos = ContextualDashEllipsisRoles::contextual_dash_ellipsis_roles_to_role_override_infos(&dash_ellipsis_role_decisions, text.as_str(), quote_aware_classifier.clone(), (context).clone()).map_err(|e|
ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
        let effective_classifier: Box<dyn FontRoleClassifier> = if i32::from_ne_bytes((u32::try_from((dash_ellipsis_role_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) { Box::new(ContextualDashEllipsisAwareFontRoleClassifier::new(quote_aware_classifier.clone(),
dash_ellipsis_role_decisions.to_vec()).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?) } else { quote_aware_classifier.clone() };
        let mut inline_object_by_start_builder: SortedMapTableBuilder<u32, InlineObjectSpan> = SortedTable::sorted_table_map_builder::<u32, InlineObjectSpan>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(input.inline_objects.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let obj = (input.inline_objects[usize::try_from(i).unwrap_or(0)]).clone();
            inline_object_by_start_builder.put(&((obj.range).clone().start), &(obj));
        }
        let cluster_ranges = ClusterRoleResolution::cluster_role_resolution_cluster_role_ranges(text.as_str(), effective_classifier.clone(), (context).clone(), (clreq_profile).clone(), (span_boundaries).clone(), (emoji_shaping_boundaries).clone(),
Some(inline_object_by_start_builder.clone().build())).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
        let capacity = quote_role_override_infos.len();
        let mut all_role_overrides = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(quote_role_override_infos.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all_role_overrides.push((quote_role_override_infos[usize::try_from(i).unwrap_or(0)]).clone());
        }
        for i in 0..match u32::try_from(dash_ellipsis_role_override_infos.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all_role_overrides.push((dash_ellipsis_role_override_infos[usize::try_from(i).unwrap_or(0)]).clone());
        }
        for i in 0..match u32::try_from(cluster_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let ro = (cluster_ranges[usize::try_from(i).unwrap_or(0)]).clone().role_override;
            match &(ro) {
                Some(__option3) => {
                    all_role_overrides.push((__option3).clone());
                }
                None => {
                }
            }
        }
        let mut r_idx = 1u32;
        while (i32::from_ne_bytes((r_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((all_role_overrides.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let curr = (all_role_overrides[usize::try_from(r_idx).unwrap_or(0)]).clone();
            let mut j = r_idx;
            while (j) > (0) && (i32::from_ne_bytes((((all_role_overrides[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes())) > (i32::from_ne_bytes(((curr.range).clone().start).to_ne_bytes())) {
                all_role_overrides[usize::try_from(j).unwrap_or(0)] = (all_role_overrides[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]).clone();
                j = u32::wrapping_sub(j, 1);
            }
            all_role_overrides[usize::try_from(j).unwrap_or(0)] = curr;
            r_idx = u32::wrapping_add(r_idx, 1);
        }
        let role_override_infos = (all_role_overrides).clone();
        let mut shapeable_ranges: Vec<ResolvedClusterRange> = Vec::new();
        for cr in &cluster_ranges {
            if !cr.mandatory_break && !cr.zero_width_soft_break && !inline_object_by_range.has(&((cr.range).clone())) {
                shapeable_ranges.push(cr.clone());
            }
        }
        let mut font_decisions: Vec<FontDecision> = Vec::new();
        let mut font_decision_by_range_builder: SortedMapTableBuilder<TextRange, FontDecision> = SortedTable::sorted_table_map_builder::<TextRange, FontDecision>(Arc::new(compare_text_range));
        for resolved_range in &shapeable_ranges {
            let decision = engine.fallback_resolver.resolve(text.as_str(), (resolved_range.range).clone(), FontRequest::new(((input.text_style).clone().font_families).clone(), ((input.text_style).clone().locale).to_string().as_str(), resolved_range.role));
            font_decisions.push(decision.clone());
            font_decision_by_range_builder.put(&((resolved_range.range).clone()), &(decision));
        }
        let font_decision_by_range: SortedMapTable<TextRange, FontDecision> = font_decision_by_range_builder.clone().build();
        let base_shaping_stage = ParagraphShapingStage::paragraph_shaping_stage_shape_paragraph(engine, (input).clone(), text.as_str(), font_size, 1e9f64, &cluster_ranges, (font_decision_by_range).clone(), (inline_object_by_range).clone(), (punctuation_glyph_substitutor).clone(),
(style_at).clone(), (emphasis_italic_at).clone(), (rejected_technical_tiers_by_span).clone(), None, None)?;
        let mut ruby_font_geometry_builder: SortedMapTableBuilder<RubySpan, RubyFontGeometry> = SortedTable::sorted_table_map_builder::<RubySpan, RubyFontGeometry>(Arc::new(compare_ruby_span));
        for ruby in &pinyin_spans {
            let metric_text = if u_string::unit_count(&((ruby.text).to_string())) == 0 { "x".to_string() } else { (ruby.text).to_string() };
            let range = TextRange::new(0u32, u_string::unit_count(&(metric_text))).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
            let preferred_families = (ruby.font_families).clone().clone();
            let ruby_locale = match &(ruby.locale) { Some(__option4) => (*__option4).clone(), None => (((input.text_style).clone().locale).to_string()).clone() };
            let decision = engine.fallback_resolver.resolve(metric_text.as_str(), (range).clone(), FontRequest::new((preferred_families).clone(), ruby_locale.as_str(), FontRole::LatinText));
            let raw = engine.font_metrics_resolver.resolve(FontMetricsRequest::new(((decision.candidate).clone().key).to_string().as_str(), ruby_font_size, FontRole::LatinText, ruby_locale.as_str(),
Some(WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_copy_font_families(&preferred_families)), Some(ruby_font_weight), Some((input.text_style).clone().italic), Some((metric_text).to_string())))?;
            let declared_ascent = match &(raw.typo_ascent) { Some(__option5) => *__option5, None => raw.ascent };
            let declared_descent = match &(raw.typo_descent) { Some(__option6) => *__option6, None => raw.descent };
            let mut shaped: Option<ShapingResult> = None;
            if i32::from_ne_bytes((u_string::unit_count(&((ruby.text).to_string()))).to_ne_bytes()) > (0) {
                shaped = Some(engine.text_shaper.shape(ShapingInput::new((ruby.text).to_string().as_str(), TextRange::new(0u32, u_string::unit_count(&((ruby.text).to_string()))).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?,
TextStyle::new(Some(WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_copy_font_families(&(ruby.font_families).clone())), Some(ruby_font_size), Some(ruby_locale.to_string()), Some(ruby_font_weight), Some((input.text_style).clone().italic),
Some((input.text_style).clone().baseline_shift), Some((input.text_style).clone().inline_attachment)), (decision).clone(), Some((ruby.text).to_string()), Some(vec![]))).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextShaperShapeFaultFault(e))?.clone());
            }
            let mut ruby_width_terms: Vec<f64> = vec![];
            let mut glyph_list: Vec<Glyph> = Vec::new();
            match &(shaped) {
                Some(__option7) => {
                    for c in 0..match u32::try_from(__option7.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        ruby_width_terms.push(__option7.clusters[usize::try_from(c).unwrap_or(0)].advance);
                    }
                    for r in 0..match u32::try_from(__option7.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let run = (__option7.glyph_runs[usize::try_from(r).unwrap_or(0)]).clone();
                        for g in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            glyph_list.push((run.glyphs[usize::try_from(g).unwrap_or(0)]).clone());
                        }
                    }
                }
                None => {
                }
            }
            let ruby_width = AccurateSum::accurate_sum_of(&ruby_width_terms);
            let required_extent = if u_string::unit_count(&((ruby.text).to_string())) == 0 { 0.0f64 } else { declared_ascent + declared_descent + ruby_stack_gap };
            let ascent = if u_string::unit_count(&((ruby.text).to_string())) == 0 { Some(0.0f64) } else { Some(declared_ascent) };
            let descent = if u_string::unit_count(&((ruby.text).to_string())) == 0 { Some(0.0f64) } else { Some(declared_descent) };
            let geom = RubyFontGeometry::new(ruby_width, ascent.unwrap(), descent.unwrap(), required_extent, glyph_list.to_vec());
            ruby_font_geometry_builder.put(&(ruby), &(geom));
        }
        let ruby_font_geometry_by_span: SortedMapTable<RubySpan, RubyFontGeometry> = ruby_font_geometry_builder.clone().build();
        return Ok(WidthIndependentParagraphAnnotation::new(text.as_str(), font_size, (style_at).clone(), (font_size_at).clone(), (bopomofo_font_weight_at).clone(), ruby_font_size, ruby_stack_gap, ruby_font_weight, pinyin_spans.to_vec(), (clreq_profile).clone(),
(punctuation_glyph_substitutor).clone(), quote_pairs.to_vec(), role_override_infos.to_vec(), font_decisions.to_vec(), cluster_ranges.to_vec(), (font_decision_by_range).clone(), (inline_object_by_range).clone(), (base_shaping_stage.segment_shaping_cache).clone(),
(base_shaping_stage.substitution_rollbacks).clone(), (ruby_font_geometry_by_span).clone(), (base_shaping_stage).clone()));
    }

    pub(crate) fn width_independent_annotation_cache_fns_compute_ruby_spread(natural: &Vec<Cluster>, ruby_size: f64, pinyin_spans: &Vec<RubySpan>, ruby_font_geometry_by_span: SortedMapTable<RubySpan, RubyFontGeometry>) -> SortedMapTable<u32, f64> {
        if u32::try_from((pinyin_spans.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build();
        }
        let word_space = ruby_size * 0.25f64;
        let mut left_x: Vec<f64> = Vec::new();
        let mut acc = 0.0f64;
        for i in 0..match u32::try_from(natural.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            left_x.push(acc);
            acc += natural[usize::try_from(i).unwrap_or(0)].advance;
        }
        let mut measures_first_cluster: Vec<u32> = Vec::new();
        let mut measures_center: Vec<f64> = Vec::new();
        let mut measures_rw: Vec<f64> = Vec::new();
        for ruby in pinyin_spans {
            let idx_range = PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&natural, (ruby.base_range).clone());
            if idx_range.is_none() {
                continue;
            }
            let center = (left_x[usize::try_from((idx_range).as_ref().unwrap().start).unwrap_or(0)] + left_x[usize::try_from((idx_range).as_ref().unwrap().end).unwrap_or(0)] + natural[usize::try_from((idx_range).as_ref().unwrap().end).unwrap_or(0)].advance) / 2.0f64;
            let geom = ruby_font_geometry_by_span.get(&(ruby));
            measures_first_cluster.push((idx_range).as_ref().unwrap().start);
            measures_center.push(center);
            measures_rw.push(geom.as_ref().unwrap().width);
        }
        let mut m_idx = 1u32;
        while (i32::from_ne_bytes((m_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((measures_first_cluster.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let curr_fc = measures_first_cluster[usize::try_from(m_idx).unwrap_or(0)];
            let curr_center = measures_center[usize::try_from(m_idx).unwrap_or(0)];
            let curr_rw = measures_rw[usize::try_from(m_idx).unwrap_or(0)];
            let mut j = m_idx;
            while (j) > (0) && ({ let v: u32 = measures_first_cluster[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = curr_fc; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                { while measures_first_cluster.len() <= usize::try_from(j).unwrap_or(0) { measures_first_cluster.push(0); } measures_first_cluster[usize::try_from(j).unwrap_or(0)] = measures_first_cluster[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]; };
                { while measures_center.len() <= usize::try_from(j).unwrap_or(0) { measures_center.push(0.0); } measures_center[usize::try_from(j).unwrap_or(0)] = measures_center[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]; };
                { while measures_rw.len() <= usize::try_from(j).unwrap_or(0) { measures_rw.push(0.0); } measures_rw[usize::try_from(j).unwrap_or(0)] = measures_rw[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]; };
                j = u32::wrapping_sub(j, 1);
            }
            { while measures_first_cluster.len() <= usize::try_from(j).unwrap_or(0) { measures_first_cluster.push(0); } measures_first_cluster[usize::try_from(j).unwrap_or(0)] = curr_fc; };
            { while measures_center.len() <= usize::try_from(j).unwrap_or(0) { measures_center.push(0.0); } measures_center[usize::try_from(j).unwrap_or(0)] = curr_center; };
            { while measures_rw.len() <= usize::try_from(j).unwrap_or(0) { measures_rw.push(0.0); } measures_rw[usize::try_from(j).unwrap_or(0)] = curr_rw; };
            m_idx = u32::wrapping_add(m_idx, 1);
        }
        let mut spread_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let spread_keys: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
        let spread_values: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
        let get_spread: Arc<dyn Fn(u32) -> f64 + Send + Sync + 'static> = { let spread_keys = Arc::clone(&spread_keys); let spread_values = Arc::clone(&spread_values); Arc::new({ let spread_keys = Arc::clone(&spread_keys); let spread_values = Arc::clone(&spread_values); move
|key| {
        let mut __loop_guard6 = spread_keys.lock().unwrap();
        let mut __loop_guard7 = spread_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard6.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if __loop_guard6[usize::try_from(i).unwrap_or(0)] == key {
                return __loop_guard7[usize::try_from(i).unwrap_or(0)];
            }
        }
        drop(__loop_guard6);
        drop(__loop_guard7);
        return 0.0f64;
} }) };
        let put_spread: Arc<dyn Fn(u32, f64) -> () + Send + Sync + 'static> = { let spread_keys = Arc::clone(&spread_keys); let spread_values = Arc::clone(&spread_values); Arc::new({ let spread_keys = Arc::clone(&spread_keys); let spread_values = Arc::clone(&spread_values); move
|key, val| {
        let mut __loop_guard8 = spread_keys.lock().unwrap();
        let mut __loop_guard9 = spread_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard8.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if __loop_guard8[usize::try_from(i).unwrap_or(0)] == key {
                { while __loop_guard9.len() <= usize::try_from(i).unwrap_or(0) { __loop_guard9.push(0.0); } __loop_guard9[usize::try_from(i).unwrap_or(0)] = val; };
                return;
            }
        }
        drop(__loop_guard8);
        drop(__loop_guard9);
        spread_keys.lock().unwrap().push(key);
        spread_values.lock().unwrap().push(val);
} }) };
        let mut shift = 0.0f64;
        let mut prev_right = -1e9f64;
        for m in 0..match u32::try_from(measures_first_cluster.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let first_cluster = measures_first_cluster[usize::try_from(m).unwrap_or(0)];
            let rw = measures_rw[usize::try_from(m).unwrap_or(0)];
            let mut center = measures_center[usize::try_from(m).unwrap_or(0)] + shift;
            let needed = prev_right + word_space - (center - rw / 2.0f64);
            if needed > (0.0f64) && ({ let v: u32 = first_cluster; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                let key = u32::wrapping_sub(first_cluster, 1);
                put_spread(key, get_spread(key) + needed);
                shift += needed;
                center += needed;
            }
            prev_right = center + rw / 2.0f64;
        }
        let mut __loop_guard10 = spread_keys.lock().unwrap();
        let mut __loop_guard11 = spread_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard10.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            spread_builder.put(&(__loop_guard10[usize::try_from(i).unwrap_or(0)]), &(__loop_guard11[usize::try_from(i).unwrap_or(0)]));
        }
        drop(__loop_guard10);
        drop(__loop_guard11);
        return spread_builder.clone().build();
    }

    pub(crate) fn width_independent_annotation_cache_fns_add_geometry_aware_opportunity(shrink_opportunities: &mut Vec<ShrinkOpportunity>, caps: GlueCapacity, idx: u32, tier: u32, line_end_only: bool) {
        if caps.paired {
            let min_glue = if caps.leading < (caps.trailing) { caps.leading } else { caps.trailing };
            let paired_capacity = 2.0f64 * min_glue;
            if paired_capacity > (0.0f64) {
                shrink_opportunities.push(ShrinkOpportunity::new(idx, tier, paired_capacity, ShrinkChannel::LeadingAndTrailingGlue, Some(line_end_only)));
            }
        } else {
            if caps.leading > (0.0f64) {
                shrink_opportunities.push(ShrinkOpportunity::new(idx, tier, caps.leading, ShrinkChannel::LeadingGlue, Some(line_end_only)));
            }
            if caps.trailing > (0.0f64) {
                shrink_opportunities.push(ShrinkOpportunity::new(idx, tier, caps.trailing, ShrinkChannel::TrailingGlue, Some(line_end_only)));
            }
        }
    }

    pub fn width_independent_annotation_cache_fns_build_paragraph_layout_prep(engine: &mut ExplainableStubParagraphLayoutEngine, input: LayoutInput, annotation: WidthIndependentParagraphAnnotation, rejected_technical_tiers_by_span: SortedMapTable<TextRange, SortedSetTable<u32>>)
-> Result<ParagraphLayoutPrep, WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> {
        let text = (annotation.text).to_string().clone();
        let font_size = (input.text_style).clone().font_size;
        let grid = ((input.paragraph_style).clone().line_length_grid).clone();
        let container_width = (input.constraints).clone().max_width;
        let mut grid_cells = u32::from_ne_bytes((match f64::from(f64::floor(container_width / font_size)) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match
((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e
=> u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }).to_ne_bytes());
        if i32::from_ne_bytes((grid_cells).to_ne_bytes()) < (1) {
            grid_cells = 1u32;
        }
        let grid_cells_int = grid_cells;
        let mut measure = container_width;
        if grid.enabled {
            measure = format!("{}", (i32::from_ne_bytes((grid_cells_int).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * font_size;
            if measure > (container_width) {
                measure = container_width;
            }
        }
        let grid_slack = container_width - measure;
        let grid_body_alignment = match &(grid.body_alignment) { Some(__option8) => *__option8, None => (input.paragraph_style).clone().last_line_alignment };
        let mut grid_body_offset = 0.0f64;
        if grid.enabled {
            if grid_body_alignment == LastLineAlignment::Center {
                grid_body_offset = grid_slack / 2.0f64;
            } else {
                if grid_body_alignment == LastLineAlignment::End {
                    grid_body_offset = grid_slack;
                }
            }
        }
        let line_length_grid_decision = LineLengthGridDecisionInfo::new(grid.enabled, container_width, font_size, if grid.enabled { grid_cells_int } else { u32::from_ne_bytes((match f64::from(measure / font_size) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32,
v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0)
}).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }).to_ne_bytes()) }, measure, grid_slack,
WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_render_nullable_last_line_alignment(Some(grid_body_alignment)).as_str(), grid_body_offset, if grid.enabled { "LineLengthGridQuantization".to_string() } else { "GridBypassed".to_string() }.as_str());
        let measure_em = measure / font_size;
        let resolved_kinsoku = KinsokuModes::kinsoku_modes_resolve(((annotation.clreq_profile).clone().kinsoku_mode).clone(), measure_em);
        let kinsoku_rule = ClreqKinsokuRule::new(Some(resolved_kinsoku.level));
        let mut has_prog_span = false;
        for i in 0..match u32::try_from((input.content).clone().line_break_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if input.content.clone().line_break_spans[usize::try_from(i).unwrap_or(0)].policy == LineBreakPolicy::ProgressiveTechnical {
                has_prog_span = true;
                break;
            }
        }
        let mut has_over_measure_token = false;
        for i in 0..match u32::try_from((annotation.base_shaping_stage).clone().shaping_results.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let res = ((annotation.base_shaping_stage).clone().shaping_results[usize::try_from(i).unwrap_or(0)]).clone();
            let mut res_terms: Vec<f64> = vec![];
            for c in 0..match u32::try_from(res.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                res_terms.push(res.clusters[usize::try_from(c).unwrap_or(0)].advance);
            }
            if AccurateSum::accurate_sum_of(&res_terms) > (measure) {
                has_over_measure_token = true;
                break;
            }
        }
        let needs_dynamic_shaping = true && (i32::from_ne_bytes((u32::from_ne_bytes((rejected_technical_tiers_by_span.size()).to_ne_bytes())).to_ne_bytes())) > (0) || has_prog_span || has_over_measure_token;
        let mut shaping_stage = (annotation.base_shaping_stage).clone().clone();
        if needs_dynamic_shaping {
            shaping_stage = ParagraphShapingStage::paragraph_shaping_stage_shape_paragraph(engine, (input).clone(), text.as_str(), font_size, measure, &annotation.cluster_ranges, (annotation.font_decision_by_range).clone(), (annotation.inline_object_by_range).clone(),
(annotation.punctuation_glyph_substitutor).clone(), (annotation.style_at).clone(), { let input = (input).clone(); Arc::new(move |offset| {
        for i in 0..match u32::try_from(input.decorations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (input.decorations[usize::try_from(i).unwrap_or(0)]).clone();
            if d.kind == DecorationKind::Emphasis && (i32::from_ne_bytes((offset).to_ne_bytes())) >= i32::from_ne_bytes(((d.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes((offset).to_ne_bytes())) < (i32::from_ne_bytes(((d.range).clone().end).to_ne_bytes())) {
                return true;
            }
        }
        return false;
}) }, (rejected_technical_tiers_by_span).clone(), Some((annotation.segment_shaping_cache).clone()), Some((annotation.substitution_rollbacks).clone())).map_err(|e| WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::ParagraphShapingStageShapeParagraphFaultFault(e))?;
        }
        let shaping_results = shaping_stage.shaping_results.clone();
        let mut raw_natural_clusters: Vec<Cluster> = Vec::new();
        for res in &shaping_results {
            for c in 0..match u32::try_from((res.clusters).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                raw_natural_clusters.push((res.clusters[usize::try_from(c).unwrap_or(0)]).clone());
            }
        }
        let mut shaped_glyphs_by_cluster_range_builder: SortedMapTableBuilder<TextRange, Vec<Glyph>> = SortedTable::sorted_table_map_builder::<TextRange, Vec<Glyph>>(Arc::new(compare_text_range));
        let mut open_type_features_by_cluster_range_builder: SortedMapTableBuilder<TextRange, Vec<String>> = SortedTable::sorted_table_map_builder::<TextRange, Vec<String>>(Arc::new(compare_text_range));
        let glyphs_by_cluster_range_keys: Arc<Mutex<Vec<TextRange>>> = Arc::new(Mutex::new(Vec::new()));
        let glyphs_by_cluster_range_values: Arc<Mutex<Vec<Vec<Glyph>>>> = Arc::new(Mutex::new(Vec::new()));
        let mut open_type_feature_keys: Vec<TextRange> = Vec::new();
        let mut open_type_feature_values: Vec<Vec<String>> = Vec::new();
        let add_glyph_to_cluster_range: Arc<dyn Fn(TextRange, Glyph) -> () + Send + Sync + 'static> = { let glyphs_by_cluster_range_keys = Arc::clone(&glyphs_by_cluster_range_keys); let glyphs_by_cluster_range_values = Arc::clone(&glyphs_by_cluster_range_values); Arc::new({ let
glyphs_by_cluster_range_keys = Arc::clone(&glyphs_by_cluster_range_keys); let glyphs_by_cluster_range_values = Arc::clone(&glyphs_by_cluster_range_values); move |range, g| {
        let mut __loop_guard12 = glyphs_by_cluster_range_keys.lock().unwrap();
        let mut __loop_guard13 = glyphs_by_cluster_range_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard12.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if __loop_guard12[usize::try_from(i).unwrap_or(0)].start == range.start && __loop_guard12[usize::try_from(i).unwrap_or(0)].end == range.end {
                __loop_guard13[usize::try_from(i).unwrap_or(0)].push(g.clone());
                return;
            }
        }
        drop(__loop_guard12);
        drop(__loop_guard13);
        glyphs_by_cluster_range_keys.lock().unwrap().push(range.clone());
        glyphs_by_cluster_range_values.lock().unwrap().push(vec![(g).clone()]);
} }) };
        for res in &shaping_results {
            for r in 0..match u32::try_from((res.glyph_runs).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let run = (res.glyph_runs[usize::try_from(r).unwrap_or(0)]).clone();
                let mut seen_ranges: Vec<TextRange> = Vec::new();
                for g in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    let glyph = (run.glyphs[usize::try_from(g).unwrap_or(0)]).clone();
                    add_glyph_to_cluster_range((glyph.cluster_range).clone(), (glyph).clone());
                    let mut range_seen = false;
                    for sr in 0..match u32::try_from(seen_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        if seen_ranges[usize::try_from(sr).unwrap_or(0)].start == (glyph.cluster_range).clone().start && seen_ranges[usize::try_from(sr).unwrap_or(0)].end == (glyph.cluster_range).clone().end {
                            range_seen = true;
                            break;
                        }
                    }
                    if !range_seen {
                        seen_ranges.push((glyph.cluster_range).clone());
                    }
                }
                for range in &seen_ranges {
                    let capacity = run.open_type_features.len();
                    let mut feat_copy = Vec::with_capacity(capacity);
                    for fi in 0..match u32::try_from(run.open_type_features.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        feat_copy.push((run.open_type_features[usize::try_from(fi).unwrap_or(0)]).clone());
                    }
                    let mut previous_features: Option<Vec<String>> = None;
                    for pi in 0..match u32::try_from(open_type_feature_keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        if open_type_feature_keys[usize::try_from(pi).unwrap_or(0)].start == range.start && open_type_feature_keys[usize::try_from(pi).unwrap_or(0)].end == range.end {
                            previous_features = Some((open_type_feature_values[usize::try_from(pi).unwrap_or(0)]).clone());
                            break;
                        }
                    }
                    match &(previous_features) {
                        Some(__option9) => {
                            let mut same_features = u32::try_from((__option9.len()) & 0xFFFF_FFFF).unwrap_or(0) == u32::try_from((feat_copy.len()) & 0xFFFF_FFFF).unwrap_or(0);
                            if same_features {
                                for fi in 0..match u32::try_from(__option9.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                    if __option9[usize::try_from(fi).unwrap_or(0)].clone() != (feat_copy[usize::try_from(fi).unwrap_or(0)]).clone() {
                                        same_features = false;
                                        break;
                                    }
                                }
                            }
                            if !same_features {
                                return Err(WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(TextRangeError::Message { text: format!("{}{}",
            "Conflicting OpenType features for shaped cluster ",
            range.to_string()
        ).to_string() }));
                            }
                        }
                        None => {
                        }
                    }
                    open_type_feature_keys.push(range.clone());
                    open_type_feature_values.push(feat_copy.clone());
                    open_type_features_by_cluster_range_builder.put(&(range), &(feat_copy));
                }
            }
        }
        let mut __loop_guard14 = glyphs_by_cluster_range_keys.lock().unwrap();
        let mut __loop_guard15 = glyphs_by_cluster_range_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard14.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            shaped_glyphs_by_cluster_range_builder.put(&((__loop_guard14[usize::try_from(i).unwrap_or(0)]).clone()), &((__loop_guard15[usize::try_from(i).unwrap_or(0)]).clone()));
        }
        drop(__loop_guard14);
        drop(__loop_guard15);
        let shaped_glyphs_by_cluster_range: SortedMapTable<TextRange, Vec<Glyph>> = shaped_glyphs_by_cluster_range_builder.clone().build();
        let open_type_features_by_cluster_range: SortedMapTable<TextRange, Vec<String>> = open_type_features_by_cluster_range_builder.clone().build();
        let _ = ClusterRoleResolution::cluster_role_resolution_require_covered_by(&raw_natural_clusters, &annotation.font_decisions).map_err(|e| WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?;
        let capacity = input.inline_objects.len();
        let mut inline_object_ranges = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(input.inline_objects.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            inline_object_ranges.push(((input.inline_objects[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        let mut narrow_inline_box_ranges: Vec<TextRange> = Vec::new();
        for i in 0..match u32::try_from(input.inline_boxes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let r#box = (input.inline_boxes[usize::try_from(i).unwrap_or(0)]).clone();
            if r#box.outer_spacing == InlineBoxOuterSpacing::Narrow {
                narrow_inline_box_ranges.push((r#box.range).clone());
            }
        }
        let mut narrow_inline_box_leading_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut narrow_inline_box_trailing_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for idx in 0..match u32::try_from(raw_natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let cl = (raw_natural_clusters[usize::try_from(idx).unwrap_or(0)]).clone();
            for j in 0..match u32::try_from(narrow_inline_box_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if narrow_inline_box_ranges[usize::try_from(j).unwrap_or(0)].start == (cl.range).clone().start {
                    narrow_inline_box_leading_clusters_builder.put(&(idx));
                }
                if narrow_inline_box_ranges[usize::try_from(j).unwrap_or(0)].end == (cl.range).clone().end {
                    narrow_inline_box_trailing_clusters_builder.put(&(idx));
                }
            }
        }
        let narrow_inline_box_leading_clusters: SortedSetTable<u32> = narrow_inline_box_leading_clusters_builder.clone().build();
        let narrow_inline_box_trailing_clusters: SortedSetTable<u32> = narrow_inline_box_trailing_clusters_builder.clone().build();
        let mut resolved_spacing_edges: Vec<EastAsianSpacingEdges> = Vec::new();
        for index in 0..match u32::try_from(raw_natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let cluster = (raw_natural_clusters[usize::try_from(index).unwrap_or(0)]).clone();
            let mut is_contained = false;
            for j in 0..match u32::try_from(inline_object_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_is_contained_in((cluster.range).clone(), (inline_object_ranges[usize::try_from(j).unwrap_or(0)]).clone()) {
                    is_contained = true;
                    break;
                }
            }
            if is_contained || PunctuationGeometryStage::punctuation_geometry_stage_is_attached_ascii_point_mark_at(&raw_natural_clusters, index) && !narrow_inline_box_leading_clusters.has(&(index)) {
                resolved_spacing_edges.push(EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false));
            } else {
                let resolved = UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges((cluster.text).to_string().as_str(), ((input.text_style).clone().locale).to_string().as_str()).map_err(|e|
WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?;
                let leading_val = if narrow_inline_box_leading_clusters.has(&(index)) { EastAsianSpacingValue::Narrow } else { resolved.leading };
                let trailing_val = if narrow_inline_box_trailing_clusters.has(&(index)) { EastAsianSpacingValue::Narrow } else { resolved.trailing };
                resolved_spacing_edges.push(EastAsianSpacingEdges::new(leading_val, trailing_val, resolved.contains_wide));
            }
        }
        let verbatim_ranges = ((input.content).clone().auto_space_suppressed_ranges).clone();
        let verbatim_suppressed_boundary: Arc<dyn Fn(u32) -> bool + Send + Sync + 'static> = { let verbatim_ranges = (verbatim_ranges).clone(); Arc::new(move |offset| {
        for vr in &verbatim_ranges {
            if i32::from_ne_bytes((vr.start).to_ne_bytes()) < (i32::from_ne_bytes((offset).to_ne_bytes())) && (i32::from_ne_bytes((offset).to_ne_bytes())) < (i32::from_ne_bytes((vr.end).to_ne_bytes())) {
                return true;
            }
        }
        return false;
}) };
        let mut east_asian_spacing_edges = (resolved_spacing_edges).clone();
        if i32::from_ne_bytes((u32::try_from((verbatim_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            let mut list: Vec<EastAsianSpacingEdges> = Vec::new();
            for index in 0..match u32::try_from(resolved_spacing_edges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let edges = (resolved_spacing_edges[usize::try_from(index).unwrap_or(0)]).clone();
                let cluster = (raw_natural_clusters[usize::try_from(index).unwrap_or(0)]).clone();
                let leading_suppressed = !narrow_inline_box_leading_clusters.has(&(index)) && verbatim_suppressed_boundary((cluster.range).clone().start);
                let trailing_suppressed = !narrow_inline_box_trailing_clusters.has(&(index)) && verbatim_suppressed_boundary((cluster.range).clone().end);
                if !leading_suppressed && !trailing_suppressed {
                    list.push(edges.clone());
                } else {
                    list.push(EastAsianSpacingEdges::new(if leading_suppressed { EastAsianSpacingValue::Other } else { edges.leading }, if trailing_suppressed { EastAsianSpacingValue::Other } else { edges.trailing }, edges.contains_wide));
                }
            }
            east_asian_spacing_edges = list;
        }
        let mut verbatim_suppression_decisions: Vec<AutoSpaceDecisionInfo> = Vec::new();
        if i32::from_ne_bytes((u32::try_from((verbatim_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            for index in 1..match u32::try_from(raw_natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let offset = ((raw_natural_clusters[usize::try_from(index).unwrap_or(0)]).clone().range).clone().start;
                if !verbatim_suppressed_boundary(offset) {
                    continue;
                }
                let left = resolved_spacing_edges[usize::try_from(u32::wrapping_sub(index, 1)).unwrap_or(0)].trailing;
                let right = resolved_spacing_edges[usize::try_from(index).unwrap_or(0)].leading;
                let wide_narrow_pair = left == EastAsianSpacingValue::Wide && right == EastAsianSpacingValue::Narrow || left == EastAsianSpacingValue::Narrow && right == EastAsianSpacingValue::Wide;
                if !wide_narrow_pair {
                    continue;
                }
                verbatim_suppression_decisions.push(AutoSpaceDecisionInfo::new(((raw_natural_clusters[usize::try_from(index).unwrap_or(0)]).clone().range).clone(), "leading", "EastAsianSpacing.Wide", AutoSpaceMode::Disabled.name().to_string().as_str(), 0u32, 0.0f64, 0.0f64,
"VerbatimRangeAutoSpace:east-asian-spacing-W-N-suppressed"));
            }
        }
        let capacity = raw_natural_clusters.len();
        let mut natural_inline_attachments = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(raw_natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            natural_inline_attachments.push((annotation.style_at)(((raw_natural_clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start).inline_attachment);
        }
        let auto_space_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_auto_space_policy(&raw_natural_clusters, &east_asian_spacing_edges, &natural_inline_attachments, ((annotation.clreq_profile).clone().auto_space).clone(), font_size,
Some((narrow_inline_box_leading_clusters).clone()), Some((narrow_inline_box_trailing_clusters).clone())).map_err(|e| WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?;
        let capacity = input.inline_boxes.len();
        let mut inline_boxes_copy = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(input.inline_boxes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            inline_boxes_copy.push((input.inline_boxes[usize::try_from(i).unwrap_or(0)]).clone());
        }
        let inline_box_result = PunctuationGeometryStage::punctuation_geometry_stage_apply_inline_box_spans(&auto_space_result.clusters, &inline_boxes_copy);
        let natural_clusters = inline_box_result.clusters.clone();
        let mut inline_object_by_cluster_index_builder: SortedMapTableBuilder<u32, InlineObjectSpan> = SortedTable::sorted_table_map_builder::<u32, InlineObjectSpan>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        let boundary_indices: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
        let boundary_adjustments: Arc<Mutex<Vec<InlineObjectBoundaryAdjustment>>> = Arc::new(Mutex::new(Vec::new()));
        let get_registered_boundary: Arc<dyn Fn(u32) -> Option<InlineObjectBoundaryAdjustment> + Send + Sync + 'static> = { let boundary_indices = Arc::clone(&boundary_indices); let boundary_adjustments = Arc::clone(&boundary_adjustments); Arc::new({ let boundary_indices =
Arc::clone(&boundary_indices); let boundary_adjustments = Arc::clone(&boundary_adjustments); move |left_cluster_index| {
        let mut __loop_guard16 = boundary_indices.lock().unwrap();
        let mut __loop_guard17 = boundary_adjustments.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard16.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if __loop_guard16[usize::try_from(i).unwrap_or(0)] == left_cluster_index {
                return Some((__loop_guard17[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        drop(__loop_guard16);
        drop(__loop_guard17);
        return None;
} }) };
        let set_registered_boundary: Arc<dyn Fn(u32, InlineObjectBoundaryAdjustment) -> () + Send + Sync + 'static> = { let boundary_indices = Arc::clone(&boundary_indices); let boundary_adjustments = Arc::clone(&boundary_adjustments); Arc::new({ let boundary_indices =
Arc::clone(&boundary_indices); let boundary_adjustments = Arc::clone(&boundary_adjustments); move |left_cluster_index, boundary| {
        let mut __loop_guard18 = boundary_indices.lock().unwrap();
        let mut __loop_guard19 = boundary_adjustments.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard18.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if __loop_guard18[usize::try_from(i).unwrap_or(0)] == left_cluster_index {
                __loop_guard19[usize::try_from(i).unwrap_or(0)] = boundary;
                return;
            }
        }
        drop(__loop_guard18);
        drop(__loop_guard19);
        boundary_indices.lock().unwrap().push(left_cluster_index);
        boundary_adjustments.lock().unwrap().push(boundary.clone());
} }) };
        let register_inline_object_boundary: Arc<dyn Fn(u32, InlineObjectBoundaryAdjustment) -> Result<(), WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault> + Send + Sync + 'static> = { let get_registered_boundary = (get_registered_boundary).clone(); let
set_registered_boundary = (set_registered_boundary).clone(); Arc::new(move |left_cluster_index, boundary| {
        let previous = get_registered_boundary(left_cluster_index);
        if previous.is_none() {
            set_registered_boundary(left_cluster_index, (boundary).clone());
            return Ok(());
        }
        let prev_kind = match &((previous).as_ref().unwrap().preferred_stretch) { Some(__option10) => Some(__option10.kind), None => None };
        let bound_kind = match &(boundary.preferred_stretch) { Some(__option11) => Some(__option11.kind), None => None };
        if match &(prev_kind) { Some(__option12) => bound_kind != None && Some(*__option12) != bound_kind, None => false } {
            return Err(WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(TextRangeError::Message { text: format!("{}{}",
            "Conflicting inline-object stretch classes at cluster boundary ",
            left_cluster_index
        ).to_string() }));
        }
        let preferred: Option<InlineObjectPreferredStretch>;
        match &(((previous).as_ref().unwrap().preferred_stretch).clone()) {
            Some(__option13) => {
                if boundary.preferred_stretch.is_some() {
                preferred = if __option13.get_capacity() >= boundary.preferred_stretch.as_ref().unwrap().get_capacity() { ((previous).as_ref().unwrap().preferred_stretch).clone() } else { boundary.preferred_stretch };
                }
                else {
                    if previous.as_ref().unwrap().preferred_stretch.clone().is_some() {
                        preferred = ((previous).as_ref().unwrap().preferred_stretch).clone();
                    } else {
                        preferred = boundary.preferred_stretch;
                    }
                }
            }
            None => {
                if previous.as_ref().unwrap().preferred_stretch.clone().is_some() {
                    preferred = ((previous).as_ref().unwrap().preferred_stretch).clone();
                } else {
                    preferred = boundary.preferred_stretch;
                }
            }
        }
        set_registered_boundary(left_cluster_index, InlineObjectBoundaryAdjustment::new(Some((previous).as_ref().unwrap().participates_in_uniform_stretch || boundary.participates_in_uniform_stretch), (preferred).clone(), Some({ let __min_a2 =
(previous).as_ref().unwrap().shrink_capacity as f64; let __min_b2 = boundary.shrink_capacity as f64; if __min_a2.is_nan() || __min_b2.is_nan() { f64::NAN } else { if __min_a2 > __min_b2 { __min_a2 } else if __min_b2 > __min_a2 { __min_b2 } else if __min_a2 == 0.0 && __min_b2 ==
0.0 { if __min_a2.is_sign_negative() { __min_b2 } else { __min_a2 } } else { __min_a2 } } }), Some({ let __min_a3 = (previous).as_ref().unwrap().line_end_discardable_advance as f64; let __min_b3 = boundary.line_end_discardable_advance as f64; if __min_a3.is_nan() ||
__min_b3.is_nan() { f64::NAN } else { if __min_a3 > __min_b3 { __min_a3 } else if __min_b3 > __min_a3 { __min_b3 } else if __min_a3 == 0.0 && __min_b3 == 0.0 { if __min_a3.is_sign_negative() { __min_b3 } else { __min_a3 } } else { __min_a3 } } }),
Some((previous).as_ref().unwrap().prevents_line_break || boundary.prevents_line_break)).map_err(|e| WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?);
        Ok(())
}) };
        for cluster_index in 0..match u32::try_from(natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let cluster = (natural_clusters[usize::try_from(cluster_index).unwrap_or(0)]).clone();
            let inline_object = annotation.inline_object_by_range.get(&((cluster.range).clone()));
            match &(inline_object) {
                Some(__option16) => {
                    inline_object_by_cluster_index_builder.put(&(cluster_index), __option16);
                    if ({ let v: u32 = cluster_index; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) && !WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_is_fixed_boundary((__option16.leading_boundary).clone()) {
                        register_inline_object_boundary(u32::wrapping_sub(cluster_index, 1), (__option16.leading_boundary).clone())?;
                    }
                    if ({ let v: u32 = cluster_index; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes((u32::wrapping_sub(u32::try_from((natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).to_ne_bytes())) &&
!WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_is_fixed_boundary((__option16.trailing_boundary).clone()) {
                        register_inline_object_boundary(u32::from_ne_bytes((cluster_index).to_ne_bytes()), (__option16.trailing_boundary).clone())?;
                    }
                }
                None => {
                }
            }
        }
        let inline_object_by_cluster_index: SortedMapTable<u32, InlineObjectSpan> = inline_object_by_cluster_index_builder.clone().build();
        let mut uniform_inline_object_boundary_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut preferred_inline_object_boundary_builder: SortedMapTableBuilder<u32, InlineObjectPreferredStretch> = SortedTable::sorted_table_map_builder::<u32, InlineObjectPreferredStretch>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut inline_object_boundary_unbreakable_ranges: Vec<IntRange> = Vec::new();
        let mut __loop_guard20 = boundary_indices.lock().unwrap();
        let mut __loop_guard21 = boundary_adjustments.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard20.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let b_idx = __loop_guard20[usize::try_from(i).unwrap_or(0)];
            let b_adj = (__loop_guard21[usize::try_from(i).unwrap_or(0)]).clone();
            if b_adj.participates_in_uniform_stretch {
                uniform_inline_object_boundary_builder.put(&(b_idx));
            }
            match &(b_adj.preferred_stretch) {
                Some(__option17) => {
                    preferred_inline_object_boundary_builder.put(&(b_idx), &((*__option17).clone()));
                }
                None => {
                }
            }
            if b_adj.prevents_line_break {
                inline_object_boundary_unbreakable_ranges.push(IntRange::new(b_idx, u32::wrapping_add(b_idx, 1)));
            }
        }
        drop(__loop_guard20);
        drop(__loop_guard21);
        let uniform_inline_object_boundary_after_clusters: SortedSetTable<u32> = uniform_inline_object_boundary_builder.clone().build();
        let preferred_inline_object_boundary_after_clusters: SortedMapTable<u32, InlineObjectPreferredStretch> = preferred_inline_object_boundary_builder.clone().build();
        let capacity = auto_space_result.decisions.len();
        let mut auto_space_decisions = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(auto_space_result.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            auto_space_decisions.push((auto_space_result.decisions[usize::try_from(i).unwrap_or(0)]).clone());
        }
        for i in 0..match u32::try_from(verbatim_suppression_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            auto_space_decisions.push((verbatim_suppression_decisions[usize::try_from(i).unwrap_or(0)]).clone());
        }
        let cluster_roles_decisions = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_containing_font_decisions(&natural_clusters, &annotation.font_decisions);
        let capacity = cluster_roles_decisions.len();
        let mut cluster_roles = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(cluster_roles_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (cluster_roles_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            cluster_roles.push(match &(d) { Some(__option18) => __option18.role, None => FontRole::Unknown });
        }
        let inline_object_attached_marks = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_marks(&natural_clusters, &cluster_roles, resolved_kinsoku.level, (Box::new((kinsoku_rule).clone())).clone());
        let mut inline_object_separator_space_trims_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut inline_object_attachment_no_stretch_boundaries_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut inline_object_punctuation_attachment_decisions: Vec<InlineObjectPunctuationAttachmentDecisionInfo> = Vec::new();
        for attachment in &inline_object_attached_marks {
            for s in 0..match u32::try_from((attachment.separator_cluster_indices).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let cluster_index = attachment.separator_cluster_indices[usize::try_from(s).unwrap_or(0)];
                inline_object_separator_space_trims_builder.put(&(cluster_index), &(natural_clusters[usize::try_from(cluster_index).unwrap_or(0)].advance));
            }
            for idx in attachment.object_cluster_index..attachment.mark_cluster_index {
                inline_object_attachment_no_stretch_boundaries_builder.put(&(idx));
            }
            if i32::from_ne_bytes((u32::try_from(((attachment.separator_cluster_indices).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                let separator_first = (natural_clusters[usize::try_from(attachment.separator_cluster_indices[0usize]).unwrap_or(0)]).clone();
                let separator_last = (natural_clusters[usize::try_from(attachment.separator_cluster_indices[usize::try_from(u32::wrapping_sub(u32::try_from(((attachment.separator_cluster_indices).clone().len()) & 0xFFFF_FFFF).unwrap_or(0),
1)).unwrap_or(0)]).unwrap_or(0)]).clone();
                let mark = (natural_clusters[usize::try_from(attachment.mark_cluster_index).unwrap_or(0)]).clone();
                let mut collapsed_terms: Vec<f64> = vec![];
                for s in 0..match u32::try_from((attachment.separator_cluster_indices).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    collapsed_terms.push(natural_clusters[usize::try_from(attachment.separator_cluster_indices[usize::try_from(s).unwrap_or(0)]).unwrap_or(0)].advance);
                }
                let collapsed_adv = AccurateSum::accurate_sum_of(&collapsed_terms);
                inline_object_punctuation_attachment_decisions.push(InlineObjectPunctuationAttachmentDecisionInfo::new(((natural_clusters[usize::try_from(attachment.object_cluster_index).unwrap_or(0)]).clone().range).clone(), TextRange::new((separator_first.range).clone().start,
(separator_last.range).clone().end).map_err(|e| WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?, (mark.range).clone(), (mark.text).to_string().as_str(),
TextRange::new(((natural_clusters[usize::try_from(attachment.object_cluster_index).unwrap_or(0)]).clone().range).clone().start, (mark.range).clone().end).map_err(|e| WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?, collapsed_adv,
Some("InlineObjectPunctuationSeparatorSpaceCollapse".to_string())));
            }
        }
        let inline_object_separator_space_trims: SortedMapTable<u32, f64> = inline_object_separator_space_trims_builder.clone().build();
        let inline_object_attachment_no_stretch_boundaries: SortedSetTable<u32> = inline_object_attachment_no_stretch_boundaries_builder.clone().build();
        let mut mandatory_break_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut zero_width_break_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut mandatory_break_decisions: Vec<MandatoryBreakDecisionInfo> = Vec::new();
        let mut zero_width_break_decisions: Vec<ZeroWidthBreakDecisionInfo> = Vec::new();
        for idx in 0..match u32::try_from(natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let cluster = (natural_clusters[usize::try_from(idx).unwrap_or(0)]).clone();
            if ParagraphShapingStage::paragraph_shaping_stage_is_mandatory_break_cluster((cluster).clone()) {
                mandatory_break_clusters_builder.put(&(idx));
                mandatory_break_decisions.push(MandatoryBreakDecisionInfo::new((cluster.range).clone(), (cluster.text).to_string().as_str(), idx, "MandatoryBreakNoShape"));
            }
            if ParagraphShapingStage::paragraph_shaping_stage_is_zero_width_soft_break_cluster((cluster).clone()) {
                zero_width_break_clusters_builder.put(&(idx));
                zero_width_break_decisions.push(ZeroWidthBreakDecisionInfo::new((cluster.range).clone(), (cluster.text).to_string().as_str(), idx, Some("ZeroWidthSpaceSoftBreakNoShape".to_string())));
            }
        }
        let mandatory_break_clusters: SortedSetTable<u32> = mandatory_break_clusters_builder.clone().build();
        let zero_width_break_clusters: SortedSetTable<u32> = zero_width_break_clusters_builder.clone().build();
        let mut punctuation_atoms: Vec<PunctuationAtom> = Vec::new();
        for idx in 0..match u32::try_from(natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if cluster_roles[usize::try_from(idx).unwrap_or(0)] == FontRole::LatinText {
                continue;
            }
            let cluster = (natural_clusters[usize::try_from(idx).unwrap_or(0)]).clone();
            let shaped_glyphs = if shaped_glyphs_by_cluster_range.has(&((cluster.range).clone())) { (shaped_glyphs_by_cluster_range.get(&((cluster.range).clone()))).unwrap() } else { vec![] };
            let atoms = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((cluster).clone(), font_size, (engine.punctuation_atom_builder).clone(), &shaped_glyphs, (annotation.clreq_profile).clone().glue_placement,
((annotation.clreq_profile).clone().punctuation_width).clone()).map_err(|e| WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?;
            for a in 0..match u32::try_from(atoms.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                punctuation_atoms.push((atoms[usize::try_from(a).unwrap_or(0)]).clone());
            }
        }
        let adjacent_punctuation_spacing_plan = engine.punctuation_spacing_compressor.compress(&punctuation_atoms, font_size).map_err(|e| WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?;
        let cjk_closing_before_ascii_point_mark_plan = engine.punctuation_spacing_compressor.compress_cjk_closing_before_ascii_point_mark(&punctuation_atoms, text.as_str(), font_size).map_err(|e|
WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?;
        let capacity = adjacent_punctuation_spacing_plan.adjustments.len();
        let mut all_adjustments = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(adjacent_punctuation_spacing_plan.adjustments.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all_adjustments.push((adjacent_punctuation_spacing_plan.adjustments[usize::try_from(i).unwrap_or(0)]).clone());
        }
        for i in 0..match u32::try_from(cjk_closing_before_ascii_point_mark_plan.adjustments.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all_adjustments.push((cjk_closing_before_ascii_point_mark_plan.adjustments[usize::try_from(i).unwrap_or(0)]).clone());
        }
        let spacing_plan = PunctuationSpacingCompressionResult::new(all_adjustments.to_vec()).map_err(|e| WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?;
        let ruby_spread: SortedMapTable<u32, f64> = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_compute_ruby_spread(&natural_clusters, annotation.ruby_font_size, &annotation.pinyin_spans, (annotation.ruby_font_geometry_by_span).clone());
        let mut bopomofo_spans: Vec<RubySpan> = Vec::new();
        for i in 0..match u32::try_from(input.ruby_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let rs = (input.ruby_spans[usize::try_from(i).unwrap_or(0)]).clone();
            if rs.kind == RubyKind::Bopomofo {
                bopomofo_spans.push(rs.clone());
            }
        }
        let mut ruby_and_bopomofo_spread: SortedMapTable<u32, f64> = (ruby_spread).clone();
        if i32::from_ne_bytes((u32::try_from((bopomofo_spans.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            let mut merged_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
            let mut merged_keys: Vec<u32> = Vec::new();
            let mut merged_vals: Vec<f64> = Vec::new();
            for i in 0..u32::from_ne_bytes((ruby_spread.size()).to_ne_bytes()) {
                merged_keys.push(ruby_spread.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
                merged_vals.push(ruby_spread.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
            }
            for z in &bopomofo_spans {
                let r = PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&natural_clusters, (z.base_range).clone());
                if r.is_none() {
                    continue;
                }
                let last_idx = (r).as_ref().unwrap().end;
                let mut found = false;
                for j in 0..match u32::try_from(merged_keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if merged_keys[usize::try_from(j).unwrap_or(0)] == last_idx {
                        merged_vals[usize::try_from(j).unwrap_or(0)] += 0.5f64 * font_size;
                        found = true;
                        break;
                    }
                }
                if !found {
                    merged_keys.push(last_idx);
                    merged_vals.push(0.5f64 * font_size);
                }
            }
            for i in 0..match u32::try_from(merged_keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                merged_builder.put(&(merged_keys[usize::try_from(i).unwrap_or(0)]), &(merged_vals[usize::try_from(i).unwrap_or(0)]));
            }
            ruby_and_bopomofo_spread = merged_builder.clone().build();
        }
        let adjustment_style = ((annotation.clreq_profile).clone().adjustment).clone();
        let punctuation_base_geometry = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&natural_clusters, &punctuation_atoms,
(spacing_plan).clone()).with_inline_box_advances((inline_box_result.advance_by_cluster).clone()).with_ruby_spread((ruby_and_bopomofo_spread).clone()).with_raw_edge_trims((inline_object_separator_space_trims).clone());
        let capacity = natural_clusters.len();
        let mut final_natural_inline_attachments = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            final_natural_inline_attachments.push((annotation.style_at)(((natural_clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start).inline_attachment);
        }
        let attached_punctuation_boundary = punctuation_base_geometry.resolve_attached_inline_punctuation_boundaries(&final_natural_inline_attachments, &punctuation_atoms, font_size).map_err(|e|
WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault::TextRangeErrorFault(e))?;
        let base_geometry = (attached_punctuation_boundary.geometry).clone().clone();
        let attached_punctuation_trailing_glue_by_cluster: SortedMapTable<u32, f64> = attached_punctuation_boundary.trailing_glue_by_cluster.clone();
        let clusters = base_geometry.resolve_clusters();
        let glue_caps: SortedMapTable<u32, GlueCapacity> = base_geometry.glue_capacities();
        let mut gap_cluster_ranges: Vec<TextRange> = Vec::new();
        for d in &auto_space_decisions {
            if d.side.to_string() == "gap" {
                gap_cluster_ranges.push((d.cluster_range).clone());
            }
        }
        let contained_atoms = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_first_contained_atom(&natural_clusters, &punctuation_atoms);
        let mut atom_class_by_range_builder: SortedMapTableBuilder<TextRange, PunctuationClass> = SortedTable::sorted_table_map_builder::<TextRange, PunctuationClass>(Arc::new(compare_text_range));
        for i in 0..match u32::try_from(natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let atom = (contained_atoms[usize::try_from(i).unwrap_or(0)]).clone();
            match &(atom) {
                Some(__option19) => {
                    atom_class_by_range_builder.put(&(((natural_clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone()), &(__option19.punctuation_class));
                }
                None => {
                }
            }
        }
        let atom_class_by_range: SortedMapTable<TextRange, PunctuationClass> = atom_class_by_range_builder.clone().build();
        let mut shrink_opportunities: Vec<ShrinkOpportunity> = Vec::new();
        for idx in 0..match u32::try_from(natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let cluster = (natural_clusters[usize::try_from(idx).unwrap_or(0)]).clone();
            if glue_caps.has(&(idx)) {
                let caps = (glue_caps.get(&(idx))).unwrap();
                let cls = if atom_class_by_range.has(&((cluster.range).clone())) { atom_class_by_range.get(&((cluster.range).clone())) } else { None };
                if cls == Some(PunctuationClass::Interpunct) || cls == Some(PunctuationClass::MiddleDot) {
                    WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_add_geometry_aware_opportunity(&mut shrink_opportunities, (caps).clone(), idx, 3, false);
                } else {
                    if cls == Some(PunctuationClass::Opening) || cls == Some(PunctuationClass::Closing) {
                        WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_add_geometry_aware_opportunity(&mut shrink_opportunities, (caps).clone(), idx, 4, false);
                    } else {
                        if cls == Some(PunctuationClass::PauseOrStop) {
                            let first_char = if i32::from_ne_bytes((u_string::unit_count(&((cluster.display_text).to_string()))).to_ne_bytes()) > (0) { u_string::unit_at(&(cluster.display_text).to_string(), 0u32) } else { Some(0) };
                            let is_stop = WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_is_inline_stop(*(first_char).as_ref().unwrap());
                            let tier = if is_stop { 7 } else { 5 };
                            let line_end_only = is_stop && !adjustment_style.allow_inline_stop_compression;
                            WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_add_geometry_aware_opportunity(&mut shrink_opportunities, (caps).clone(), idx, tier, line_end_only);
                        } else {
                            WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_add_geometry_aware_opportunity(&mut shrink_opportunities, (caps).clone(), idx, 5, false);
                        }
                    }
                }
            } else {
                if PunctuationGeometryStage::punctuation_geometry_stage_is_space_run((cluster).clone()) && !inline_object_separator_space_trims.has(&(idx)) {
                    let mut in_gap = false;
                    for gr in &gap_cluster_ranges {
                        if gr.start == (cluster.range).clone().start && gr.end == (cluster.range).clone().end {
                            in_gap = true;
                            break;
                        }
                    }
                    if in_gap {
                        let cap = cluster.advance - 0.125f64 * font_size;
                        if adjustment_style.allow_sino_western_gap_adjustment && (cap) > (0.0f64) {
                            shrink_opportunities.push(ShrinkOpportunity::new(idx, 6u32, cap, ShrinkChannel::RawAdvance, Some(false)));
                        }
                    } else {
                        let cap = cluster.advance - 0.25f64 * font_size;
                        if cap > (0.0f64) {
                            shrink_opportunities.push(ShrinkOpportunity::new(idx, 2u32, cap, ShrinkChannel::RawAdvance, Some(false)));
                        }
                    }
                }
            }
        }
        for i in 0..u32::from_ne_bytes((inline_object_by_cluster_index.size()).to_ne_bytes()) {
            let idx = inline_object_by_cluster_index.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let inline_object = inline_object_by_cluster_index.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let trailing = (inline_object.trailing_boundary).clone().shrink_capacity;
            if trailing > (0.0f64) {
                shrink_opportunities.push(ShrinkOpportunity::new(idx, 8u32, trailing, ShrinkChannel::RawAdvance, Some(false)));
            }
        }
        let mut shaping_decisions_list: Vec<ShapingDecisionInfo> = Vec::new();
        for sr_idx in 0..match u32::try_from(shaping_stage.shaping_results.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let sr = (shaping_stage.shaping_results[usize::try_from(sr_idx).unwrap_or(0)]).clone();
            for d_idx in 0..match u32::try_from(sr.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                shaping_decisions_list.push((sr.decisions[usize::try_from(d_idx).unwrap_or(0)]).clone());
            }
        }
        return Ok(ParagraphLayoutPrep::new((input).clone(), (rejected_technical_tiers_by_span).clone(), text.as_str(), font_size, (annotation.style_at).clone(), (annotation.font_size_at).clone(), (annotation.bopomofo_font_weight_at).clone(), annotation.ruby_font_size,
annotation.ruby_stack_gap, annotation.ruby_font_weight, annotation.pinyin_spans.to_vec(), (annotation.clreq_profile).clone(), (annotation.punctuation_glyph_substitutor).clone(), measure, measure_em, grid_body_offset, (line_length_grid_decision).clone(),
annotation.quote_pairs.to_vec(), annotation.role_override_infos.to_vec(), annotation.font_decisions.to_vec(), (shaping_stage.hyphen_offsets).clone(), shaping_stage.hyphen_advance, shaping_stage.hyphen_glyphs.to_vec(), (shaping_stage.substitution_rollbacks).clone(),
shaping_stage.break_opportunity_decisions.to_vec(), shaping_stage.emergency_tracking_eligibility_decisions.to_vec(), (shaping_stage.progressive_break_offsets).clone(), (shaped_glyphs_by_cluster_range).clone(), (open_type_features_by_cluster_range).clone(),
shaping_decisions_list.to_vec(), east_asian_spacing_edges.to_vec(), auto_space_decisions.to_vec(), (inline_box_result).clone(), natural_clusters.to_vec(), (inline_object_by_cluster_index).clone(), (uniform_inline_object_boundary_after_clusters).clone(),
(preferred_inline_object_boundary_after_clusters).clone(), inline_object_boundary_unbreakable_ranges.to_vec(), cluster_roles.to_vec(), (resolved_kinsoku).clone(), (kinsoku_rule).clone(), inline_object_attached_marks.to_vec(), (inline_object_separator_space_trims).clone(),
(inline_object_attachment_no_stretch_boundaries).clone(), inline_object_punctuation_attachment_decisions.to_vec(), (mandatory_break_clusters).clone(), (zero_width_break_clusters).clone(), mandatory_break_decisions.to_vec(), zero_width_break_decisions.to_vec(),
punctuation_atoms.to_vec(), (spacing_plan).clone(), (annotation.ruby_font_geometry_by_span).clone(), (ruby_and_bopomofo_spread).clone(), final_natural_inline_attachments.to_vec(), (attached_punctuation_boundary).clone(), (base_geometry).clone(),
(attached_punctuation_trailing_glue_by_cluster).clone(), clusters.to_vec(), (adjustment_style).clone(), (atom_class_by_range).clone(), shrink_opportunities.to_vec()));
    }
}
