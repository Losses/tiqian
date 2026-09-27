use crate::org::tiqian::clreq::bopomofo_parser::BopomofoParser;
use crate::org::tiqian::clreq::bopomofo_tone::BopomofoTone;
use crate::org::tiqian::core::accurate_sum::AccurateSum;
use crate::org::tiqian::core::bopomofo_decision_info::BopomofoDecisionInfo;
use crate::org::tiqian::core::bopomofo_glyph_placement::BopomofoGlyphPlacement;
use crate::org::tiqian::core::bopomofo_glyph_role::BopomofoGlyphRole;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::cluster_geometry_decision_info::ClusterGeometryDecisionInfo;
use crate::org::tiqian::core::decoration_decision_info::DecorationDecisionInfo;
use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::decoration_segment_info::DecorationSegmentInfo;
use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::inline_object_decision_info::InlineObjectDecisionInfo;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::ruby_decision_info::RubyDecisionInfo;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_policy::FontRequest;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::line_geometry_stage::ClusterMetricDecision;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub struct RubyFontGeometry {
    pub width: f64,
    pub ascent: f64,
    pub descent: f64,
    pub required_extent: f64,
    pub glyphs: Vec<Glyph>,
}

impl RubyFontGeometry {
    pub fn new(width: f64, ascent: f64, descent: f64, required_extent: f64, glyphs: Vec<Glyph>) -> Self {
        Self {
            width,
            ascent,
            descent,
            required_extent,
            glyphs,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RubyFontGeometry(")); __s += &(UString::from("width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.width)); __s += &(UString::from(", ")); __s += &(UString::from("ascent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.ascent)); __s += &(UString::from(", ")); __s += &(UString::from("descent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.descent)); __s += &(UString::from(", ")); __s += &(UString::from("requiredExtent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.required_extent)); __s += &(UString::from(", ")); __s += &(UString::from("glyphs=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.glyphs).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotationGeometryStageResult {
    pub inline_object_decisions: Vec<InlineObjectDecisionInfo>,
    pub decoration_decisions: Vec<DecorationDecisionInfo>,
    pub decoration_segments: Vec<DecorationSegmentInfo>,
    pub ruby_decisions: Vec<RubyDecisionInfo>,
    pub bopomofo_decisions: Vec<BopomofoDecisionInfo>,
}

impl AnnotationGeometryStageResult {
    pub fn new(inline_object_decisions: Vec<InlineObjectDecisionInfo>, decoration_decisions: Vec<DecorationDecisionInfo>, decoration_segments: Vec<DecorationSegmentInfo>, ruby_decisions: Vec<RubyDecisionInfo>, bopomofo_decisions: Vec<BopomofoDecisionInfo>) -> Self {
        Self {
            inline_object_decisions,
            decoration_decisions,
            decoration_segments,
            ruby_decisions,
            bopomofo_decisions,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("AnnotationGeometryStageResult(")); __s += &(UString::from("inlineObjectDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.inline_object_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("decorationDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decoration_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("decorationSegments=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decoration_segments).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("rubyDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.ruby_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("bopomofoDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.bopomofo_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, PartialEq)]
pub struct IndexedDecorationSegment {
    pub index: u32,
    pub seg: DecorationSegmentInfo,
}

impl IndexedDecorationSegment {
    pub fn new(index: u32, seg: DecorationSegmentInfo) -> Self {
        Self {
            index,
            seg,
        }
    }
}

#[derive(Clone, Copy)]
pub struct AnnotationGeometryStage;

impl AnnotationGeometryStage {
    pub fn annotation_geometry_stage_compute_decoration_decisions(decorations: &[DecorationSpan], line_ranges: &Vec<IntRange>, line_boxes: &Vec<LineBox>, final_clusters: &Vec<Cluster>, cluster_roles: &Vec<FontRole>, justify_delta_by_cluster: SortedMapTable<u32, f64>, ruby_spread_by_cluster: SortedMapTable<u32, f64>, metric_decisions: &Vec<ClusterMetricDecision>, font_size: f64, emphasis_dot_gap_em: f64) -> Vec<DecorationDecisionInfo> {
        if u32::try_from((decorations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return vec![];
        }
        let mut decisions: Vec<DecorationDecisionInfo> = Vec::new();
        for span in decorations {
            if span.kind != DecorationKind::Emphasis {
                continue;
            }
            for line_index in 0..match u32::try_from(line_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let cluster_range = (line_ranges[usize::try_from(line_index).unwrap_or(0)]).clone();
                let mut x = line_boxes[usize::try_from(line_index).unwrap_or(0)].indent;
                for idx in cluster_range.start..u32::wrapping_add(cluster_range.end, 1) {
                    let cluster = (final_clusters[usize::try_from(idx).unwrap_or(0)]).clone();
                    let covered_by_span = i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((span.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((span.range).clone().end) as i32).to_ne_bytes());
                    if covered_by_span {
                        let role = cluster_roles[usize::try_from(idx).unwrap_or(0)];
                        let applied = role == FontRole::CjkText;
                        let justify_delta = if justify_delta_by_cluster.has(&(idx)) { (justify_delta_by_cluster.get(&(idx))).unwrap() } else { 0.0f64 };
                        let ruby_spread = if ruby_spread_by_cluster.has(&(idx)) { (ruby_spread_by_cluster.get(&(idx))).unwrap() } else { 0.0f64 };
                        let glyph_advance = (cluster.advance - justify_delta) - ruby_spread;
                        let mut metric: Option<ClusterMetricDecision> = None;
                        for m in metric_decisions {
                            if i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((m.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((m.range).clone().end) as i32).to_ne_bytes()) {
                                metric = Some(m.clone());
                                break;
                            }
                        }
                        let cluster_em = match &(metric) { Some(__option1) => if true { __option1.request.font_size } else { font_size }, None => font_size };
                        let face_descent = match &(metric) { Some(__option3) => if true { __option3.layout_metrics.descent } else { cluster_em * 0.12f64 }, None => cluster_em * 0.12f64 };
                        let candidate_dot_diameter = cluster_em * 0.19f64;
                        let dot_diameter = if applied { candidate_dot_diameter } else { 0.0f64 };
                        let reason = if applied { UString::from("EmphasisDotOnHanText") } else { if role == FontRole::CjkPunctuation { UString::from("clreq-no-dot-on-punctuation") } else { UString::from("no-dot-on-non-han") }.to_ustring() };
                        decisions.push(DecorationDecisionInfo::new((cluster.range).clone(), (cluster.text).to_ustring().as_ustr(), UString::from(span.kind.name()).as_ustr(), applied, reason.as_ustr(), Some(x + glyph_advance / 2.0f64), Some(line_boxes[usize::try_from(line_index).unwrap_or(0)].baseline + cluster.baseline_shift + face_descent + cluster_em * emphasis_dot_gap_em + candidate_dot_diameter / 2.0f64), Some(dot_diameter)));
                    }
                    x += cluster.advance;
                }
            }
        }
        return decisions;
    }

    pub fn annotation_geometry_stage_shorten_adjacent_interlinear_lines(segments: &Vec<DecorationSegmentInfo>, font_size: f64) -> Vec<DecorationSegmentInfo> {
        let proper_noun_name = UString::from(DecorationKind::ProperNoun.name());
        let book_title_name = UString::from(DecorationKind::BookTitle.name());
        let mut result = segments.clone();
        let mut by_line_builder: SortedMapTableBuilder<u32, Vec<IndexedDecorationSegment>> = SortedTable::sorted_table_map_builder::<u32,
Vec<IndexedDecorationSegment>>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut line_keys: Vec<u32> = vec![];
        let mut line_groups: Vec<Vec<IndexedDecorationSegment>> = vec![];
        for i in 0..match u32::try_from(result.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let seg = (result[usize::try_from(i).unwrap_or(0)]).clone();
            if seg.kind.to_ustring() == proper_noun_name || (seg.kind).to_ustring() == book_title_name {
                let mut slot = 4294967295u32;
                let mut k = 0u32;
                while (i32::from_ne_bytes(((k) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((line_keys.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    if line_keys[usize::try_from(k).unwrap_or(0)] == seg.line_index {
                        slot = k;
                        break;
                    }
                    k = u32::wrapping_add(k, 1);
                }
                if slot > 2147483647 {
                    line_keys.push(seg.line_index);
                    line_groups.push(Vec::new());
                    slot = u32::wrapping_sub(u32::try_from((line_keys.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
                }
                line_groups[usize::try_from(slot).unwrap_or(0)].push(IndexedDecorationSegment::new(i, (seg).clone()));
            }
        }
        for k in 0..match u32::try_from(line_keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            by_line_builder.put(&(line_keys[usize::try_from(k).unwrap_or(0)]), &((line_groups[usize::try_from(k).unwrap_or(0)]).clone()));
        }
        let by_line: SortedMapTable<u32, Vec<IndexedDecorationSegment>> = by_line_builder.clone().build();
        for k in 0..u32::from_ne_bytes(((by_line.size()) as u32).to_ne_bytes()) {
            let mut entries = by_line.value_at({ let v: u32 = k; i32::from_ne_bytes(v.to_ne_bytes()) });
            for p in 1..match u32::try_from(entries.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let key = (entries[usize::try_from(p).unwrap_or(0)]).clone();
                let mut q = p;
                while (q) > (0) && (((entries[usize::try_from(u32::wrapping_sub(q, 1)).unwrap_or(0)]).clone().seg).clone().left) > ((key.seg).clone().left) {
                    entries[usize::try_from(q).unwrap_or(0)] = (entries[usize::try_from(u32::wrapping_sub(q, 1)).unwrap_or(0)]).clone();
                    q = u32::wrapping_sub(q, 1);
                }
                entries[usize::try_from(q).unwrap_or(0)] = key;
            }
            let mut i = 0;
            while (i) < (i32::wrapping_sub(i32::from_ne_bytes(((i32::from_ne_bytes(u32::try_from(((entries).len()) & 4294967295).unwrap_or(0).to_ne_bytes())) as i32).to_ne_bytes()), 1)) {
                let a = (entries[usize::try_from(i).unwrap_or(0)]).clone();
                let b = (entries[usize::try_from(i32::wrapping_add(i, 1)).unwrap_or(0)]).clone();
                if b.seg.clone().left - (a.seg).clone().right <= 0.01f64 * font_size {
                    let pullback = font_size * 0.0625f64;
                    let cur_a = (result[usize::try_from(a.index).unwrap_or(0)]).clone();
                    result[usize::try_from(a.index).unwrap_or(0)] = DecorationSegmentInfo::new((cur_a.source_range).clone(), (cur_a.kind).to_ustring().as_ustr(), cur_a.line_index, cur_a.left, cur_a.top, cur_a.right - pullback, cur_a.bottom, cur_a.open_start, cur_a.open_end, UString::from(format!("{}", { let mut __s = UString::new(); __s += (cur_a.reason).to_ustring().as_ustr(); __s += &(UString::from(";AdjacentInterlinearLineShortening")); __s }).as_str()).as_ustr());
                    let cur_b = (result[usize::try_from(b.index).unwrap_or(0)]).clone();
                    result[usize::try_from(b.index).unwrap_or(0)] = DecorationSegmentInfo::new((cur_b.source_range).clone(), (cur_b.kind).to_ustring().as_ustr(), cur_b.line_index, cur_b.left + pullback, cur_b.top, cur_b.right, cur_b.bottom, cur_b.open_start, cur_b.open_end, UString::from(format!("{}", { let mut __s = UString::new(); __s += (cur_b.reason).to_ustring().as_ustr(); __s += &(UString::from(";AdjacentInterlinearLineShortening")); __s }).as_str()).as_ustr());
                }
                i = i32::wrapping_add(i, 1);
            }
        }
        return result;
    }

    pub fn annotation_geometry_stage_compute_decoration_segments(decorations: &[DecorationSpan], line_ranges: &Vec<IntRange>, line_boxes: &Vec<LineBox>, final_clusters: &Vec<Cluster>, justify_delta_by_cluster: SortedMapTable<u32, f64>, geometry_by_range: SortedMapTable<TextRange, ClusterGeometryDecisionInfo>, leading_gap_ranges: SortedSetTable<TextRange>, trailing_gap_ranges: SortedSetTable<TextRange>, auto_space_gap_px: f64, font_size: f64) -> Result<Vec<DecorationSegmentInfo>, TextRangeError> {
        let leading_blank: Arc<dyn Fn(TextRange, bool) -> f64 + Send + Sync + 'static> = { let geometry_by_range = (geometry_by_range).clone(); let leading_gap_ranges = (leading_gap_ranges).clone(); Arc::new(move |range, at_line_start| {
        let g = if geometry_by_range.has(&(range)) { geometry_by_range.get(&(range)) } else { None };
        let glue = match &(g) { Some(__option4) => __option4.leading_glue_natural - __option4.leading_glue_consumed, None => 0.0f64 };
        let auto = if leading_gap_ranges.has(&(range)) && !at_line_start { auto_space_gap_px } else { 0.0f64 };
        return glue + auto;
}) };
        let trailing_blank: Arc<dyn Fn(TextRange, bool) -> f64 + Send + Sync + 'static> = { let geometry_by_range = (geometry_by_range).clone(); let trailing_gap_ranges = (trailing_gap_ranges).clone(); Arc::new(move |range, at_line_end| {
        let g = if geometry_by_range.has(&(range)) { geometry_by_range.get(&(range)) } else { None };
        let glue = match &(g) { Some(__option5) => __option5.trailing_glue_natural - __option5.trailing_glue_consumed, None => 0.0f64 };
        let auto = if trailing_gap_ranges.has(&(range)) && !at_line_end { auto_space_gap_px } else { 0.0f64 };
        return glue + auto;
}) };
        let mut box_spans: Vec<DecorationSpan> = Vec::new();
        for s in decorations {
            if s.kind == DecorationKind::Mourning || s.kind == DecorationKind::ProperNoun || s.kind == DecorationKind::BookTitle {
                box_spans.push(s.clone());
            }
        }
        if u32::try_from((box_spans.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(vec![]);
        }
        let mut segments: Vec<DecorationSegmentInfo> = Vec::new();
        for span in &box_spans {
            let capacity = line_ranges.len();
            let mut span_segments = Vec::with_capacity(capacity);
            for line_index in 0..match u32::try_from(line_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let cluster_range = (line_ranges[usize::try_from(line_index).unwrap_or(0)]).clone();
                let mut x = line_boxes[usize::try_from(line_index).unwrap_or(0)].indent;
                let mut left: Option<f64> = None;
                let mut right = 0.0f64;
                let mut seg_start = 4294967295u32;
                let mut seg_end = 4294967295u32;
                for idx in cluster_range.start..u32::wrapping_add(cluster_range.end, 1) {
                    let cluster = (final_clusters[usize::try_from(idx).unwrap_or(0)]).clone();
                    let covered = i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((span.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((span.range).clone().end) as i32).to_ne_bytes());
                    if covered {
                        if left.is_none() {
                            left = Some(x + leading_blank((cluster.range).clone(), idx == cluster_range.start));
                            seg_start = (cluster.range).clone().start;
                        }
                        let justify_delta = if justify_delta_by_cluster.has(&(idx)) { (justify_delta_by_cluster.get(&(idx))).unwrap() } else { 0.0f64 };
                        right = (x + cluster.advance - justify_delta) - trailing_blank((cluster.range).clone(), idx == cluster_range.end);
                        seg_end = (cluster.range).clone().end;
                    }
                    x += cluster.advance;
                }
                if left.is_none() {
                    continue;
                }
                let left_edge = *(left).as_ref().unwrap();
                let baseline = line_boxes[usize::try_from(line_index).unwrap_or(0)].baseline;
                let is_line = span.kind != DecorationKind::Mourning;
                let line_y_em = if span.kind == DecorationKind::BookTitle { 0.24f64 } else { 0.18f64 };
                let line_y = baseline + font_size * line_y_em;
                span_segments.push(DecorationSegmentInfo::new(TextRange::new(seg_start, seg_end)?, UString::from(span.kind.name()).as_ustr(), line_index, left_edge, if is_line { line_y } else { baseline - font_size * 0.88f64 }, right, if is_line { line_y } else { baseline + font_size * 0.12f64 }, (i32::from_ne_bytes(((seg_start) as i32).to_ne_bytes())) > (i32::from_ne_bytes((((span.range).clone().start) as i32).to_ne_bytes())), (i32::from_ne_bytes(((seg_end) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((span.range).clone().end) as i32).to_ne_bytes())), &(UStr::new(&[]))));
            }
            let reason = if span.kind == DecorationKind::Mourning && (i32::from_ne_bytes(((u32::try_from((span_segments.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) <= 1 { UString::from("MourningSpanKeptUnbroken") } else { if span.kind == DecorationKind::Mourning { UString::from("mourning-span-split-across-lines") } else { UString::from("InterlinearLinePerAnnotatedItem") }.to_ustring() };
            for seg in &span_segments {
                segments.push(DecorationSegmentInfo::new((seg.source_range).clone(), (seg.kind).to_ustring().as_ustr(), seg.line_index, seg.left, seg.top, seg.right, seg.bottom, seg.open_start, seg.open_end, reason.as_ustr()));
            }
        }
        return Ok(AnnotationGeometryStage::annotation_geometry_stage_shorten_adjacent_interlinear_lines(&segments, font_size));
    }

    pub fn annotation_geometry_stage_compute_ruby_decisions(ruby_spans: &Vec<RubySpan>, line_ranges: &Vec<IntRange>, line_boxes: &Vec<LineBox>, final_clusters: &Vec<Cluster>, natural_clusters: &Vec<Cluster>, metric_decisions: &Vec<ClusterMetricDecision>, ruby_font_geometry_by_span: SortedMapTable<RubySpan, RubyFontGeometry>, ruby_stack_gap: f64, fallback_base_ascent: f64, ruby_font_size: f64, ruby_font_weight: u32, base_locale: &UStr) -> Vec<RubyDecisionInfo> {
        if u32::try_from((ruby_spans.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return vec![];
        }
        let mut out: Vec<RubyDecisionInfo> = Vec::new();
        for ruby in ruby_spans {
            let ruby_geometry = ruby_font_geometry_by_span.get(&(ruby));
            for line_index in 0..match u32::try_from(line_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let cluster_range = (line_ranges[usize::try_from(line_index).unwrap_or(0)]).clone();
                let mut x = line_boxes[usize::try_from(line_index).unwrap_or(0)].indent;
                let mut has_base_left = false;
                let mut base_left = 0.0f64;
                let mut content_width = 0.0f64;
                let mut base_face_top = f64::INFINITY;
                for idx in cluster_range.start..u32::wrapping_add(cluster_range.end, 1) {
                    let cluster = (final_clusters[usize::try_from(idx).unwrap_or(0)]).clone();
                    if i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((ruby.base_range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((ruby.base_range).clone().end) as i32).to_ne_bytes()) {
                        if !has_base_left {
                            base_left = x;
                            has_base_left = true;
                        }
                        content_width += natural_clusters[usize::try_from(idx).unwrap_or(0)].advance;
                        let mut metric: Option<ClusterMetricDecision> = None;
                        for m in metric_decisions {
                            if i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((m.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((m.range).clone().end) as i32).to_ne_bytes()) {
                                metric = Some(m.clone());
                                break;
                            }
                        }
                        let ascent = match &(metric) { Some(__option7) => if true { __option7.layout_metrics.ascent } else { fallback_base_ascent }, None => fallback_base_ascent };
                        let candidate_top = line_boxes[usize::try_from(line_index).unwrap_or(0)].baseline + cluster.baseline_shift - ascent;
                        if candidate_top < (base_face_top) {
                            base_face_top = candidate_top;
                        }
                    }
                    x += cluster.advance;
                }
                if has_base_left {
                    let ruby_width = ruby_geometry.as_ref().unwrap().width;
                    let mut _g: Vec<UString> = vec![];
                    for f in 0..match u32::try_from((ruby.font_families).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        _g.push((ruby.font_families[usize::try_from(f).unwrap_or(0)]).clone());
                    }
                    let font_families = (_g).clone();
                    out.push(RubyDecisionInfo::new((ruby.base_range).clone(), (ruby.text).to_ustring().as_ustr(), line_index, base_left + content_width / 2.0f64, (base_face_top - ruby_stack_gap) - ruby_geometry.as_ref().unwrap().descent, ruby_font_size, { let __min_a1 = 0.0f64 as f64; let __min_b1 = ((ruby_width - content_width) / 2.0f64) as f64; if __min_a1.is_nan() || __min_b1.is_nan() { f64::NAN } else { if __min_a1 > __min_b1 { __min_a1 } else if __min_b1 > __min_a1 { __min_b1 } else if __min_a1 == 0.0 && __min_b1 == 0.0 { if __min_a1.is_sign_negative() { __min_b1 } else { __min_a1 } } else { __min_a1 } } }, Some(ruby_geometry.as_ref().unwrap().ascent), Some(ruby_geometry.as_ref().unwrap().descent), Some(ruby_width), Some((font_families).clone()), Some(ruby_font_weight), Some((match &(ruby.locale) { Some(__option9) => (*__option9).clone(), None => base_locale.to_ustring() }).to_ustring()), Some((ruby_geometry.as_ref().unwrap().glyphs).clone())));
                }
            }
        }
        return out;
    }

    pub fn annotation_geometry_stage_bopomofo_symbol_rows(n: u32, neutral: bool) -> Vec<IntRange> {
        if i32::from_ne_bytes(((n) as i32).to_ne_bytes()) <= 1 {
            return vec![(IntRange::new(11u32, 20u32)).clone()];
        } else {
            if n == 2 {
                return vec![(IntRange::new(6u32, 15u32)).clone(), (IntRange::new(17u32, 26u32)).clone()];
            } else {
                if neutral {
                    return vec![
    (IntRange::new(3u32, 12u32)).clone(),
    (IntRange::new(12u32, 21u32)).clone(),
    (IntRange::new(21u32, 30u32)).clone(),
];
                } else {
                    return vec![
    (IntRange::new(2u32, 11u32)).clone(),
    (IntRange::new(11u32, 20u32)).clone(),
    (IntRange::new(20u32, 29u32)).clone(),
];
                }
            }
        }
    }

    pub fn annotation_geometry_stage_bopomofo_neutral_row(n: u32) -> IntRange {
        if n == 1 {
            return IntRange::new(8u32, 10u32);
        } else {
            if n == 2 {
                return IntRange::new(3u32, 5u32);
            } else {
                return IntRange::new(0u32, 2u32);
            }
        }
    }

    pub fn annotation_geometry_stage_bopomofo_regular_tone_row(n: u32) -> IntRange {
        if n == 1 {
            return IntRange::new(9u32, 14u32);
        } else {
            if n == 2 {
                return IntRange::new(15u32, 20u32);
            } else {
                return IntRange::new(18u32, 23u32);
            }
        }
    }

    pub fn annotation_geometry_stage_bopomofo_ru_tone_row(n: u32) -> IntRange {
        if n == 1 {
            return IntRange::new(16u32, 21u32);
        } else {
            if n == 2 {
                return IntRange::new(21u32, 26u32);
            } else {
                return IntRange::new(24u32, 29u32);
            }
        }
    }

    pub fn annotation_geometry_stage_bopomofo_tone_glyph(tone: BopomofoTone) -> UString {
        return match tone {
            BopomofoTone::Yinping => UString::from("").to_ustring().to_ustring(),
            BopomofoTone::Yangping => UString::from("ˊ").to_ustring().to_ustring(),
            BopomofoTone::Shang => UString::from("ˇ").to_ustring().to_ustring(),
            BopomofoTone::Qu => UString::from("ˋ").to_ustring().to_ustring(),
            BopomofoTone::Neutral => UString::from("˙").to_ustring().to_ustring(),
            BopomofoTone::Ru => UString::from("").to_ustring().to_ustring(),
        };
    }

    pub(crate) fn annotation_geometry_stage_copy_font_families(families: &[UString]) -> Vec<UString> {
        let capacity = families.len();
        let mut result = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(families.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            result.push((families[usize::try_from(i).unwrap_or(0)]).clone());
        }
        return result;
    }

    pub(crate) fn annotation_geometry_stage_bopomofo_box(zone_left: f64, box_top: f64, h_unit: f64, v_unit: f64, left_u: f64, width_u: f64, top_u: u32, bot_u: u32, role: BopomofoGlyphRole, text: &UStr) -> BopomofoGlyphPlacement {
        let b_left = zone_left + left_u * h_unit;
        let b_top = box_top + format!("{}", (i32::from_ne_bytes(((top_u) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * v_unit;
        let b_width = width_u * h_unit;
        let b_height = format!("{}", (i32::from_ne_bytes(((u32::wrapping_sub(bot_u, top_u)) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * v_unit;
        return BopomofoGlyphPlacement::new(text, b_left, b_top, b_width, b_height, role, Some(vec![]), b_left, b_top + b_height, b_height);
    }

    pub(crate) fn annotation_geometry_stage_bopomofo_ink_bounds(shaped: ShapingResult) -> Option<Rect> {
        let mut min_left = f64::INFINITY;
        let mut min_top = f64::INFINITY;
        let mut max_right = f64::NEG_INFINITY;
        let mut max_bottom = f64::NEG_INFINITY;
        let mut has_bounds = false;
        for rri in 0..match u32::try_from(shaped.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (shaped.glyph_runs[usize::try_from(rri).unwrap_or(0)]).clone();
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let glyph = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                match &(glyph.bounds) {
                    Some(__option10) => {
                        let bound = (*__option10).clone();
                        let l = bound.left + glyph.x;
                        let t = bound.top + glyph.y;
                        let r = bound.right + glyph.x;
                        let b = bound.bottom + glyph.y;
                        if l < (min_left) {
                            min_left = l;
                        }
                        if t < (min_top) {
                            min_top = t;
                        }
                        if r > (max_right) {
                            max_right = r;
                        }
                        if b > (max_bottom) {
                            max_bottom = b;
                        }
                        has_bounds = true;
                    }
                    None => {
                    }
                }
            }
        }
        if !has_bounds {
            return None;
        }
        return Some(Rect::new(min_left, min_top, max_right, max_bottom));
    }

    pub fn annotation_geometry_stage_compute_bopomofo_decisions(engine: &mut ExplainableStubParagraphLayoutEngine, ruby_spans: &Vec<RubySpan>, line_ranges: &Vec<IntRange>, line_boxes: &Vec<LineBox>, final_clusters: &Vec<Cluster>, natural_clusters: &Vec<Cluster>, base_ascent: f64, base_descent: f64, font_size: f64, bopomofo_font_weight_at: Arc<dyn Fn(u32) -> u32 + Send + Sync>, base_text_style: TextStyle) -> Result<Vec<BopomofoDecisionInfo>, TextShaperShapeFault> {
        if u32::try_from((ruby_spans.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(vec![]);
        }
        let h_unit = font_size / 30.0f64;
        let v_unit = (base_ascent + base_descent) / 30.0f64;
        let mut out: Vec<BopomofoDecisionInfo> = Vec::new();
        for ruby in ruby_spans {
            let ruby_locale = match &(ruby.locale) { Some(__option11) => (*__option11).clone(), None => ((base_text_style.locale).to_ustring()).clone() };
            for line_index in 0..match u32::try_from(line_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let cluster_range = (line_ranges[usize::try_from(line_index).unwrap_or(0)]).clone();
                let mut x = line_boxes[usize::try_from(line_index).unwrap_or(0)].indent;
                let mut has_content_left = false;
                let mut content_left = 0.0f64;
                let mut content_width = 0.0f64;
                for idx in cluster_range.start..u32::wrapping_add(cluster_range.end, 1) {
                    let cluster = (final_clusters[usize::try_from(idx).unwrap_or(0)]).clone();
                    if i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((ruby.base_range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((ruby.base_range).clone().end) as i32).to_ne_bytes()) {
                        if !has_content_left {
                            content_left = x;
                            has_content_left = true;
                        }
                        content_width += natural_clusters[usize::try_from(idx).unwrap_or(0)].advance;
                    }
                    x += cluster.advance;
                }
                if !has_content_left {
                    continue;
                }
                let zone_left = content_left + content_width;
                let box_top = line_boxes[usize::try_from(line_index).unwrap_or(0)].baseline - base_ascent;
                let parsed = BopomofoParser::bopomofo_parser_parse((ruby.text).to_ustring().as_ustr());
                let mut n = u32::try_from((parsed.symbols.len()) & 0xFFFF_FFFF).unwrap_or(0);
                if i32::from_ne_bytes(((n) as i32).to_ne_bytes()) < (1) {
                    n = 1u32;
                } else {
                    if i32::from_ne_bytes(((n) as i32).to_ne_bytes()) > (3) {
                        n = 3u32;
                    }
                }
                let neutral = parsed.tone == BopomofoTone::Neutral;
                let mut placements: Vec<BopomofoGlyphPlacement> = Vec::new();
                if parsed.tone == BopomofoTone::Neutral {
                    let row = AnnotationGeometryStage::annotation_geometry_stage_bopomofo_neutral_row(n);
                    placements.push(AnnotationGeometryStage::annotation_geometry_stage_bopomofo_box(zone_left, box_top, h_unit, v_unit, 1.0f64, 9.0f64, row.start, row.end, BopomofoGlyphRole::Neutral, UStr::new(&[729])));
                }
                let rows = AnnotationGeometryStage::annotation_geometry_stage_bopomofo_symbol_rows(n, neutral);
                let sym_count = if i32::from_ne_bytes(((u32::try_from((parsed.symbols.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) < (3) { u32::try_from((parsed.symbols.len()) & 0xFFFF_FFFF).unwrap_or(0) } else { 3 };
                for i in 0..sym_count {
                    let sym = (parsed.symbols[usize::try_from(i).unwrap_or(0)]).clone();
                    let row = (rows[usize::try_from(i).unwrap_or(0)]).clone();
                    placements.push(AnnotationGeometryStage::annotation_geometry_stage_bopomofo_box(zone_left, box_top, h_unit, v_unit, 1.0f64, 9.0f64, row.start, row.end, BopomofoGlyphRole::Symbol, sym.as_ustr()));
                }
                {
                    let _g = parsed.tone;
                    let _ = match _g {
    BopomofoTone::Yinping => {},
    BopomofoTone::Yangping => {
    let row = AnnotationGeometryStage::annotation_geometry_stage_bopomofo_regular_tone_row(n);
    placements.push(AnnotationGeometryStage::annotation_geometry_stage_bopomofo_box(zone_left, box_top, h_unit, v_unit, 10.0f64, 5.0f64, row.start, row.end, BopomofoGlyphRole::Tone, AnnotationGeometryStage::annotation_geometry_stage_bopomofo_tone_glyph(parsed.tone).as_ustr()))
},
    BopomofoTone::Shang => {
    let row = AnnotationGeometryStage::annotation_geometry_stage_bopomofo_regular_tone_row(n);
    placements.push(AnnotationGeometryStage::annotation_geometry_stage_bopomofo_box(zone_left, box_top, h_unit, v_unit, 10.0f64, 5.0f64, row.start, row.end, BopomofoGlyphRole::Tone, AnnotationGeometryStage::annotation_geometry_stage_bopomofo_tone_glyph(parsed.tone).as_ustr()))
},
    BopomofoTone::Qu => {
    let row = AnnotationGeometryStage::annotation_geometry_stage_bopomofo_regular_tone_row(n);
    placements.push(AnnotationGeometryStage::annotation_geometry_stage_bopomofo_box(zone_left, box_top, h_unit, v_unit, 10.0f64, 5.0f64, row.start, row.end, BopomofoGlyphRole::Tone, AnnotationGeometryStage::annotation_geometry_stage_bopomofo_tone_glyph(parsed.tone).as_ustr()))
},
    BopomofoTone::Neutral => {},
    BopomofoTone::Ru => {
    let row = AnnotationGeometryStage::annotation_geometry_stage_bopomofo_ru_tone_row(n);
    placements.push(AnnotationGeometryStage::annotation_geometry_stage_bopomofo_box(zone_left, box_top, h_unit, v_unit, 10.0f64, 5.0f64, row.start, row.end, BopomofoGlyphRole::Tone, AnnotationGeometryStage::annotation_geometry_stage_bopomofo_tone_glyph(parsed.tone).as_ustr()))
},
};
                }
                if i32::from_ne_bytes(((u32::try_from((placements.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
                    let placement_weight = bopomofo_font_weight_at((ruby.base_range).clone().start);
                    let capacity = placements.len();
                    let mut replay_placements = Vec::with_capacity(capacity);
                    for pi in 0..match u32::try_from(placements.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let placement = (placements[usize::try_from(pi).unwrap_or(0)]).clone();
                        let replay_font_size: f64;
                        {
                            let _g = placement.role;
                            let _ = match _g {
    BopomofoGlyphRole::Symbol => replay_font_size = font_size * 0.3f64,
    BopomofoGlyphRole::Tone => replay_font_size = font_size * 0.3f64,
    BopomofoGlyphRole::Neutral => replay_font_size = placement.width,
};
                        }
                        let range = TextRange::new(0u32, u_string::unit_count(&((placement.text).to_ustring()))).map_err(|e| TextShaperShapeFault::TextRangeErrorFault(e))?;
                        let preferred_families = AnnotationGeometryStage::annotation_geometry_stage_copy_font_families(&(ruby.font_families).clone());
                        let decision = engine.fallback_resolver.resolve((placement.text).to_ustring().as_ustr(), (range).clone(), FontRequest::new((preferred_families).clone(), ruby_locale.as_ustr(), FontRole::CjkText));
                        let styled = TextStyle::new(Some((preferred_families).clone()), Some(replay_font_size), Some((ruby_locale).to_ustring()), Some(placement_weight), Some(false), Some(base_text_style.baseline_shift), Some(base_text_style.inline_attachment));
                        let shaped = engine.text_shaper.lock().unwrap().shape(ShapingInput::new((placement.text).to_ustring().as_ustr(), (range).clone(), (styled).clone(), (decision).clone(), Some((placement.text).to_ustring()), Some(vec![UString::from("vert=1").to_ustring()])))?;
                        let mut glyphs: Vec<Glyph> = Vec::new();
                        for rri in 0..match u32::try_from(shaped.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            let run = (shaped.glyph_runs[usize::try_from(rri).unwrap_or(0)]).clone();
                            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                glyphs.push((run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone());
                            }
                        }
                        let mut advance_terms: Vec<f64> = vec![];
                        for ci in 0..match u32::try_from(shaped.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            advance_terms.push(shaped.clusters[usize::try_from(ci).unwrap_or(0)].advance);
                        }
                        let advance = AccurateSum::accurate_sum_of(&advance_terms);
                        let ink = AnnotationGeometryStage::annotation_geometry_stage_bopomofo_ink_bounds((shaped).clone());
                        let draw_x: f64;
                        {
                            let _g = placement.role;
                            let _ = match _g {
    BopomofoGlyphRole::Symbol => draw_x = placement.left + (placement.width - advance) / 2.0f64,
    BopomofoGlyphRole::Tone => {
    let ink_left = match &(ink) { Some(__option12) => __option12.left, None => 0.0f64 };
    let ink_right = match &(ink) { Some(__option13) => __option13.right, None => advance };
    draw_x = placement.left + placement.width / 2.0f64 - (ink_left + ink_right) / 2.0f64
},
    BopomofoGlyphRole::Neutral => draw_x = placement.left + (placement.width - advance) / 2.0f64,
};
                        }
                        let baseline_y: f64;
                        {
                            let _g = placement.role;
                            let _ = match _g {
    BopomofoGlyphRole::Symbol => baseline_y = placement.top + placement.height * 0.88f64,
    BopomofoGlyphRole::Tone => {
    let ink_top = match &(ink) { Some(__option14) => __option14.top, None => 0.0f64 };
    let ink_bottom = match &(ink) { Some(__option15) => __option15.bottom, None => 0.0f64 };
    baseline_y = placement.top + placement.height / 2.0f64 - (ink_top + ink_bottom) / 2.0f64
},
    BopomofoGlyphRole::Neutral => {
    let ink_top = match &(ink) { Some(__option16) => __option16.top, None => 0.0f64 };
    let ink_bottom = match &(ink) { Some(__option17) => __option17.bottom, None => 0.0f64 };
    baseline_y = placement.top + placement.height / 2.0f64 - (ink_top + ink_bottom) / 2.0f64
},
};
                        }
                        replay_placements.push(BopomofoGlyphPlacement::new((placement.text).to_ustring().as_ustr(), placement.left, placement.top, placement.width, placement.height, placement.role, Some((glyphs).clone()), draw_x, baseline_y, replay_font_size));
                    }
                    let ruby_families = AnnotationGeometryStage::annotation_geometry_stage_copy_font_families(&(ruby.font_families).clone());
                    out.push(BopomofoDecisionInfo::new((ruby.base_range).clone(), (ruby.text).to_ustring().as_ustr(), line_index, replay_placements.to_vec(), Some((ruby_families).clone()), Some(bopomofo_font_weight_at((ruby.base_range).clone().start)), Some((ruby_locale).to_ustring())));
                }
            }
        }
        return Ok(out);
    }
}
