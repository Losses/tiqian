use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::inline_object_line_height_decision_info::InlineObjectLineHeightDecisionInfo;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::max_lines_decision_info::MaxLinesDecisionInfo;
use crate::org::tiqian::core::ruby_line_height_decision_info::RubyLineHeightDecisionInfo;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::font_metrics::FontMetricsRequest;
use crate::org::tiqian::font::layout_font_metrics::LayoutFontMetrics;
use crate::org::tiqian::font::metric_box::MetricBox;
use crate::org::tiqian::font::raw_font_metrics::RawFontMetrics;
use crate::org::tiqian::layout::annotation_geometry_stage::RubyFontGeometry;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::punctuation_geometry_ledger::PunctuationGeometryLedger;
use crate::runtime::fp_helper::FPHelper;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Clone, PartialEq)]
pub struct LineBoxStageResult {
    pub laid_out_lines: Vec<LineBox>,
    pub visible_lines: Vec<LineBox>,
    pub max_lines_decision: Option<MaxLinesDecisionInfo>,
    pub visible_line_ranges: Vec<IntRange>,
}

impl LineBoxStageResult {
    pub fn new(laid_out_lines: Vec<LineBox>, visible_lines: Vec<LineBox>, max_lines_decision: Option<MaxLinesDecisionInfo>, visible_line_ranges: Vec<IntRange>) -> Self {
        Self {
            laid_out_lines,
            visible_lines,
            max_lines_decision,
            visible_line_ranges,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LineBoxStageResult(")); __s += &(UString::from("laidOutLines=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.laid_out_lines).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("visibleLines=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.visible_lines).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("maxLinesDecision=")); __s += (match &(self.max_lines_decision) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", __option.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("visibleLineRanges=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.visible_line_ranges).clone();
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
pub struct LineVerticalGeometryStageResult {
    pub ruby_line_height_decision: Option<RubyLineHeightDecisionInfo>,
    pub inline_object_line_height_decision: Option<InlineObjectLineHeightDecisionInfo>,
    pub line_baseline: Vec<f64>,
    pub line_top: Vec<f64>,
    pub line_bottom: Vec<f64>,
}

impl LineVerticalGeometryStageResult {
    pub fn new(ruby_line_height_decision: Option<RubyLineHeightDecisionInfo>, inline_object_line_height_decision: Option<InlineObjectLineHeightDecisionInfo>, line_baseline: Vec<f64>, line_top: Vec<f64>, line_bottom: Vec<f64>) -> Self {
        Self {
            ruby_line_height_decision,
            inline_object_line_height_decision,
            line_baseline,
            line_top,
            line_bottom,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LineVerticalGeometryStageResult(")); __s += &(UString::from("rubyLineHeightDecision=")); __s += (match &(self.ruby_line_height_decision) { None => UString::from("null"), Some(__option1) => UString::from(format!("{}", __option1.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineObjectLineHeightDecision=")); __s += (match &(self.inline_object_line_height_decision) { None => UString::from("null"), Some(__option2) => UString::from(format!("{}", __option2.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineBaseline=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_baseline).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineTop=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_top).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineBottom=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_bottom).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClusterMetricDecision {
    pub range: TextRange,
    pub source_text: UString,
    pub request: FontMetricsRequest,
    pub raw_metrics: RawFontMetrics,
    pub layout_metrics: LayoutFontMetrics,
}

impl ClusterMetricDecision {
    pub fn new(range: TextRange, source_text: &UStr, request: FontMetricsRequest, raw_metrics: RawFontMetrics, layout_metrics: LayoutFontMetrics) -> Self {
        Self {
            range,
            source_text: source_text.to_ustring(),
            request,
            raw_metrics,
            layout_metrics,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ClusterMetricDecision(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("request=")); __s += UString::from(format!("{}", (self.request).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("rawMetrics=")); __s += UString::from(format!("{}", (self.raw_metrics).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("layoutMetrics=")); __s += UString::from(format!("{}", (self.layout_metrics).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedLineMetrics {
    pub baseline: f64,
    pub height: f64,
    pub extra_leading: f64,
}

impl ResolvedLineMetrics {
    pub fn new(baseline: f64, height: f64, extra_leading: Option<f64>) -> Self {
        let extra_leading = extra_leading.unwrap_or_else(|| 0 as f64);
        Self {
            baseline,
            height,
            extra_leading: extra_leading,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ResolvedLineMetrics(")); __s += &(UString::from("baseline=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.baseline)); __s += &(UString::from(", ")); __s += &(UString::from("height=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.height)); __s += &(UString::from(", ")); __s += &(UString::from("extraLeading=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.extra_leading)); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, PartialEq)]
pub struct RubyClusterRange {
    pub ruby: RubySpan,
    pub range: IntRange,
}

impl RubyClusterRange {
    pub fn new(ruby: RubySpan, range: IntRange) -> Self {
        Self {
            ruby,
            range,
        }
    }
}

#[derive(Clone, Copy)]
pub struct LineGeometryStageFns;

impl LineGeometryStageFns {
    pub fn line_geometry_stage_fns_resolve_line_vertical_geometry(input: LayoutInput, font_size: f64, pinyin_spans: &Vec<RubySpan>, natural_clusters: &Vec<Cluster>, line_solution: LineSolution, ruby_font_geometry_by_span: SortedMapTable<RubySpan, RubyFontGeometry>, existing_interline_space: f64, base_line_metrics: ResolvedLineMetrics, base_face_height: f64, ruby_extent: f64, inline_object_by_cluster_index: HashMap<u32, InlineObjectSpan>, base_ascent: f64, base_descent: f64) -> LineVerticalGeometryStageResult {
        return LineGeometryStageFns::line_geometry_stage_fns_resolve_line_vertical_geometry_sorted((input).clone(), font_size, &pinyin_spans, &natural_clusters, (line_solution).clone(), (ruby_font_geometry_by_span).clone(), existing_interline_space, (base_line_metrics).clone(), base_face_height, ruby_extent, Some(LineGeometryStageFns::line_geometry_stage_fns_sorted_inline_object_by_cluster_index((inline_object_by_cluster_index).clone(), &natural_clusters)), base_ascent, base_descent);
    }

    pub(crate) fn line_geometry_stage_fns_sorted_inline_object_by_cluster_index(inline_object_by_cluster_index: HashMap<u32, InlineObjectSpan>, natural_clusters: &Vec<Cluster>) -> SortedMapTable<u32, InlineObjectSpan> {
        let mut builder: SortedMapTableBuilder<u32, InlineObjectSpan> = SortedTable::sorted_table_map_builder::<u32,
InlineObjectSpan>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        if true {
            for index in 0..match u32::try_from(natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let inline_object = inline_object_by_cluster_index.get(&index).cloned();
                match &(inline_object) {
                    Some(__option3) => {
                        builder.put(&(index), __option3);
                    }
                    None => {
                    }
                }
            }
        }
        return builder.clone().build();
    }

    pub fn line_geometry_stage_fns_resolve_line_vertical_geometry_sorted(input: LayoutInput, font_size: f64, pinyin_spans: &Vec<RubySpan>, natural_clusters: &Vec<Cluster>, line_solution: LineSolution, ruby_font_geometry_by_span: SortedMapTable<RubySpan, RubyFontGeometry>, existing_interline_space: f64, base_line_metrics: ResolvedLineMetrics, base_face_height: f64, ruby_extent: f64, inline_object_by_cluster_index: Option<SortedMapTable<u32, InlineObjectSpan>>, base_ascent: f64, base_descent: f64) -> LineVerticalGeometryStageResult {
        let mut pinyin_cluster_ranges: Vec<RubyClusterRange> = vec![];
        for ruby in pinyin_spans {
            let range = LineGeometryStageFns::line_geometry_stage_fns_cluster_index_range_for(&natural_clusters, (ruby.base_range).clone());
            match &(range) {
                Some(__option4) => {
                    pinyin_cluster_ranges.push(RubyClusterRange::new((*ruby).clone(), (__option4).clone()));
                }
                None => {
                }
            }
        }
        let mut per_line_ruby_extent: Vec<f64> = vec![];
        {
            let _g1 = line_solution.lines.clone();
            for line in &_g1 {
                let mut required = 0.0f64;
                {
                    let mut _g = 0u32;
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((pinyin_cluster_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let pair = (pinyin_cluster_ranges[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if i32::from_ne_bytes(((pair.range.start) as i32).to_ne_bytes()) <= i32::from_ne_bytes((((line.cluster_range).clone().end) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((pair.range.end) as i32).to_ne_bytes())) >= i32::from_ne_bytes((((line.cluster_range).clone().start) as i32).to_ne_bytes()) && ruby_font_geometry_by_span.has(&((pair.ruby).clone())) {
                            required = { let __min_a = required as f64; let __min_b = ruby_font_geometry_by_span.get(&((pair.ruby).clone())).as_ref().unwrap().required_extent as f64; if __min_a.is_nan() || __min_b.is_nan() { f64::NAN } else { if __min_a > __min_b { __min_a } else if __min_b > __min_a { __min_b } else if __min_a == 0.0 && __min_b == 0.0 { if __min_a.is_sign_negative() { __min_b } else { __min_a } } else { __min_a } } };
                        }
                    }
                }
                per_line_ruby_extent.push(required);
            }
        }
        let capacity = per_line_ruby_extent.len();
        let mut pipeline_result = Vec::with_capacity(capacity);
        for pipeline_index in 0..match u32::try_from(per_line_ruby_extent.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = per_line_ruby_extent[usize::try_from(pipeline_index).unwrap_or(0)];
            pipeline_result.push({ let __min_a1 = 0.0f64 as f64; let __min_b1 = (v - existing_interline_space) as f64; if __min_a1.is_nan() || __min_b1.is_nan() { f64::NAN } else { if __min_a1 > __min_b1 { __min_a1 } else if __min_b1 > __min_a1 { __min_b1 } else if __min_a1 == 0.0 && __min_b1 == 0.0 { if __min_a1.is_sign_negative() { __min_b1 } else { __min_a1 } } else { __min_a1 } } });
        }
        let per_line_ruby_deficit = (pipeline_result).clone();
        let mut paragraph_ruby_deficit = 0.0f64;
        for &x in &per_line_ruby_deficit {
            paragraph_ruby_deficit = { let __min_a2 = paragraph_ruby_deficit as f64; let __min_b2 = x as f64; if __min_a2.is_nan() || __min_b2.is_nan() { f64::NAN } else { if __min_a2 > __min_b2 { __min_a2 } else if __min_b2 > __min_a2 { __min_b2 } else if __min_a2 == 0.0 && __min_b2 == 0.0 { if __min_a2.is_sign_negative() { __min_b2 } else { __min_a2 } } else { __min_a2 } } };
        }
        let uniform = (input.paragraph_style).clone().ruby_line_height_mode == RubyLineHeightMode::UniformParagraph;
        let mut paragraph_ruby_extent = 0.0f64;
        for &x in &per_line_ruby_extent {
            paragraph_ruby_extent = { let __min_a3 = paragraph_ruby_extent as f64; let __min_b3 = x as f64; if __min_a3.is_nan() || __min_b3.is_nan() { f64::NAN } else { if __min_a3 > __min_b3 { __min_a3 } else if __min_b3 > __min_a3 { __min_b3 } else if __min_a3 == 0.0 && __min_b3 == 0.0 { if __min_a3.is_sign_negative() { __min_b3 } else { __min_a3 } } else { __min_a3 } } };
        }
        let mut line_ruby_top_extra: Vec<f64> = vec![];
        let mut line_ruby_interline_demand: Vec<f64> = vec![];
        for i in 0..match u32::try_from(line_solution.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            line_ruby_top_extra.push(if uniform { paragraph_ruby_deficit } else { per_line_ruby_deficit[usize::try_from(i).unwrap_or(0)] });
            line_ruby_interline_demand.push(if uniform { paragraph_ruby_extent } else { per_line_ruby_extent[usize::try_from(i).unwrap_or(0)] });
        }
        let mut max_extra = 0.0f64;
        let mut expanded_ruby: Vec<u32> = vec![];
        for i in 0..match u32::try_from(line_ruby_top_extra.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            max_extra = { let __min_a4 = max_extra as f64; let __min_b4 = line_ruby_top_extra[usize::try_from(i).unwrap_or(0)] as f64; if __min_a4.is_nan() || __min_b4.is_nan() { f64::NAN } else { if __min_a4 > __min_b4 { __min_a4 } else if __min_b4 > __min_a4 { __min_b4 } else if __min_a4 == 0.0 && __min_b4 == 0.0 { if __min_a4.is_sign_negative() { __min_b4 } else { __min_a4 } } else { __min_a4 } } };
            if line_ruby_top_extra[usize::try_from(i).unwrap_or(0)] > (0 as f64) {
                expanded_ruby.push(i);
            }
        }
        let has_ruby_extra = i32::from_ne_bytes(((u32::try_from((expanded_ruby.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0);
        let ruby_reason = if has_ruby_extra { UString::from("ConditionalRubyLineHeight") } else { UString::from("ExistingInterlineSpaceFitsRuby") };
        let rd = if u32::try_from((pinyin_spans.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { None } else { Some(RubyLineHeightDecisionInfo::new(UString::from((input.paragraph_style).clone().ruby_line_height_mode.name()).as_ustr(), base_line_metrics.height, base_face_height, ruby_extent, existing_interline_space, max_extra, line_ruby_top_extra.to_vec(), expanded_ruby.to_vec(), ruby_reason.as_ustr())) };
        let base_baseline = LineGeometryStageFns::line_geometry_stage_fns_f32(base_line_metrics.baseline);
        let base_height = LineGeometryStageFns::line_geometry_stage_fns_f32(base_line_metrics.height);
        let base_top_extent = base_baseline;
        let base_bottom_extent = LineGeometryStageFns::line_geometry_stage_fns_f32(base_height - base_baseline);
        let f_base_ascent = LineGeometryStageFns::line_geometry_stage_fns_f32(base_ascent);
        let f_base_descent = LineGeometryStageFns::line_geometry_stage_fns_f32(base_descent);
        let mut line_object_ascent: Vec<f64> = vec![];
        let mut line_object_descent: Vec<f64> = vec![];
        {
            let _g1 = line_solution.lines.clone();
            for line in &_g1 {
                let mut ascent = 0.0f64;
                let mut descent = 0.0f64;
                let mut idx = (line.cluster_range).clone().start;
                while (i32::from_ne_bytes(((idx) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((line.cluster_range).clone().end) as i32).to_ne_bytes()) {
                    match &(inline_object_by_cluster_index) {
                        Some(__option5) => {
                            if __option5.has(&(idx)) {
                            let obj = __option5.get(&(idx));
                            ascent = { let __min_a5 = ascent as f64; let __min_b5 = LineGeometryStageFns::line_geometry_stage_fns_f32(obj.as_ref().unwrap().ascent) as f64; if __min_a5.is_nan() || __min_b5.is_nan() { f64::NAN } else { if __min_a5 > __min_b5 { __min_a5 } else if __min_b5 > __min_a5 { __min_b5 } else if __min_a5 == 0.0 && __min_b5 == 0.0 { if __min_a5.is_sign_negative() { __min_b5 } else { __min_a5 } } else { __min_a5 } } };
                            descent = { let __min_a6 = descent as f64; let __min_b6 = LineGeometryStageFns::line_geometry_stage_fns_f32(obj.as_ref().unwrap().descent) as f64; if __min_a6.is_nan() || __min_b6.is_nan() { f64::NAN } else { if __min_a6 > __min_b6 { __min_a6 } else if __min_b6 > __min_a6 { __min_b6 } else if __min_a6 == 0.0 && __min_b6 == 0.0 { if __min_a6.is_sign_negative() { __min_b6 } else { __min_a6 } } else { __min_a6 } } };
                            }
                        }
                        None => {
                        }
                    }
                    idx = u32::wrapping_add(idx, 1);
                }
                line_object_ascent.push(ascent);
                line_object_descent.push(descent);
            }
        }
        let capacity = line_object_ascent.len();
        let mut pipeline_result1 = Vec::with_capacity(capacity);
        for pipeline_index1 in 0..match u32::try_from(line_object_ascent.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = line_object_ascent[usize::try_from(pipeline_index1).unwrap_or(0)];
            pipeline_result1.push({ let __min_a7 = 0.0f64 as f64; let __min_b7 = LineGeometryStageFns::line_geometry_stage_fns_f32(v - f_base_ascent) as f64; if __min_a7.is_nan() || __min_b7.is_nan() { f64::NAN } else { if __min_a7 > __min_b7 { __min_a7 } else if __min_b7 > __min_a7 { __min_b7 } else if __min_a7 == 0.0 && __min_b7 == 0.0 { if __min_a7.is_sign_negative() { __min_b7 } else { __min_a7 } } else { __min_a7 } } });
        }
        let line_object_top_intrusion = (pipeline_result1).clone();
        let capacity = line_object_descent.len();
        let mut pipeline_result2 = Vec::with_capacity(capacity);
        for pipeline_index2 in 0..match u32::try_from(line_object_descent.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = line_object_descent[usize::try_from(pipeline_index2).unwrap_or(0)];
            pipeline_result2.push({ let __min_a8 = 0.0f64 as f64; let __min_b8 = LineGeometryStageFns::line_geometry_stage_fns_f32(v - f_base_descent) as f64; if __min_a8.is_nan() || __min_b8.is_nan() { f64::NAN } else { if __min_a8 > __min_b8 { __min_a8 } else if __min_b8 > __min_a8 { __min_b8 } else if __min_a8 == 0.0 && __min_b8 == 0.0 { if __min_a8.is_sign_negative() { __min_b8 } else { __min_a8 } } else { __min_a8 } } });
        }
        let line_object_bottom_intrusion = (pipeline_result2).clone();
        let minimum_clearance = LineGeometryStageFns::line_geometry_stage_fns_f32(LineGeometryStageFns::line_geometry_stage_fns_f32((input.paragraph_style).clone().inline_object_minimum_clearance_em) * LineGeometryStageFns::line_geometry_stage_fns_f32(font_size));
        let mut combined_line_extra: Vec<f64> = vec![];
        for i in 0..match u32::try_from(line_solution.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i == 0 {
                combined_line_extra.push({ let __min_a13 = LineGeometryStageFns::line_geometry_stage_fns_f32(line_ruby_top_extra[usize::try_from(i).unwrap_or(0)]) as f64; let __min_b13 = { let __min_a14 = 0.0f64 as f64; let __min_b14 = LineGeometryStageFns::line_geometry_stage_fns_f32(line_object_ascent[usize::try_from(i).unwrap_or(0)] - base_top_extent) as f64; if __min_a14.is_nan() || __min_b14.is_nan() { f64::NAN } else { if __min_a14 > __min_b14 { __min_a14 } else if __min_b14 > __min_a14 { __min_b14 } else if __min_a14 == 0.0 && __min_b14 == 0.0 { if __min_a14.is_sign_negative() { __min_b14 } else { __min_a14 } } else { __min_a14 } } } as f64; if __min_a13.is_nan() || __min_b13.is_nan() { f64::NAN } else { if __min_a13 > __min_b13 { __min_a13 } else if __min_b13 > __min_a13 { __min_b13 } else if __min_a13 == 0.0 && __min_b13 == 0.0 { if __min_a13.is_sign_negative() { __min_b13 } else { __min_a13 } } else { __min_a13 } } });
            } else {
                let top_demand = { let __min_a15 = LineGeometryStageFns::line_geometry_stage_fns_f32(line_ruby_interline_demand[usize::try_from(i).unwrap_or(0)]) as f64; let __min_b15 = line_object_top_intrusion[usize::try_from(i).unwrap_or(0)] as f64; if __min_a15.is_nan() || __min_b15.is_nan() { f64::NAN } else { if __min_a15 > __min_b15 { __min_a15 } else if __min_b15 > __min_a15 { __min_b15 } else if __min_a15 == 0.0 && __min_b15 == 0.0 { if __min_a15.is_sign_negative() { __min_b15 } else { __min_a15 } } else { __min_a15 } } };
                let intrudes = line_object_bottom_intrusion[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)] > (0 as f64) || (line_object_top_intrusion[usize::try_from(i).unwrap_or(0)]) > (0 as f64) && (line_object_top_intrusion[usize::try_from(i).unwrap_or(0)]) >= line_ruby_interline_demand[usize::try_from(i).unwrap_or(0)];
                let clearance = if intrudes { minimum_clearance } else { 0.0f64 };
                combined_line_extra.push({ let __min_a17 = 0.0f64 as f64; let __min_b17 = LineGeometryStageFns::line_geometry_stage_fns_f32(LineGeometryStageFns::line_geometry_stage_fns_f32(LineGeometryStageFns::line_geometry_stage_fns_f32(line_object_bottom_intrusion[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)] + top_demand) + clearance) - LineGeometryStageFns::line_geometry_stage_fns_f32(existing_interline_space)) as f64; if __min_a17.is_nan() || __min_b17.is_nan() { f64::NAN } else { if __min_a17 > __min_b17 { __min_a17 } else if __min_b17 > __min_a17 { __min_b17 } else if __min_a17 == 0.0 && __min_b17 == 0.0 { if __min_a17.is_sign_negative() { __min_b17 } else { __min_a17 } } else { __min_a17 } } });
            }
        }
        let mut object_line_extra: Vec<f64> = vec![];
        for i in 0..match u32::try_from(combined_line_extra.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            object_line_extra.push({ let __min_a19 = 0.0f64 as f64; let __min_b19 = LineGeometryStageFns::line_geometry_stage_fns_f32(combined_line_extra[usize::try_from(i).unwrap_or(0)] - LineGeometryStageFns::line_geometry_stage_fns_f32(line_ruby_top_extra[usize::try_from(i).unwrap_or(0)])) as f64; if __min_a19.is_nan() || __min_b19.is_nan() { f64::NAN } else { if __min_a19 > __min_b19 { __min_a19 } else if __min_b19 > __min_a19 { __min_b19 } else if __min_a19 == 0.0 && __min_b19 == 0.0 { if __min_a19.is_sign_negative() { __min_b19 } else { __min_a19 } } else { __min_a19 } } });
        }
        let mut baselines: Vec<f64> = vec![];
        if i32::from_ne_bytes(((u32::try_from((line_solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            baselines.push(LineGeometryStageFns::line_geometry_stage_fns_f32(base_baseline + combined_line_extra[0usize]));
            for i in 1..match u32::try_from(line_solution.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                baselines.push(LineGeometryStageFns::line_geometry_stage_fns_f32(LineGeometryStageFns::line_geometry_stage_fns_f32(baselines[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)] + base_height) + combined_line_extra[usize::try_from(i).unwrap_or(0)]));
            }
        }
        let mut _g: Vec<f64> = vec![];
        for _ in 0..match u32::try_from(line_solution.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            _g.push(0.0f64);
        }
        let mut tops = (_g).clone();
        let mut _g: Vec<f64> = vec![];
        for _ in 0..match u32::try_from(line_solution.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            _g.push(0.0f64);
        }
        let mut bottoms = (_g).clone();
        let boundary_count = if i32::from_ne_bytes(((u32::try_from((line_solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (1) { u32::wrapping_sub(u32::try_from((line_solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1) } else { 0 };
        let mut _g: Vec<f64> = vec![];
        for _ in 0..boundary_count {
            _g.push(0.0f64);
        }
        let mut boundary_shifts = (_g).clone();
        for i in 0..boundary_count {
            let current = { let __min_a20 = f_base_descent as f64; let __min_b20 = line_object_descent[usize::try_from(i).unwrap_or(0)] as f64; if __min_a20.is_nan() || __min_b20.is_nan() { f64::NAN } else { if __min_a20 > __min_b20 { __min_a20 } else if __min_b20 > __min_a20 { __min_b20 } else if __min_a20 == 0.0 && __min_b20 == 0.0 { if __min_a20.is_sign_negative() { __min_b20 } else { __min_a20 } } else { __min_a20 } } };
            let boundary_extent = LineGeometryStageFns::line_geometry_stage_fns_resolve_inline_object_line_boundary_extent(base_bottom_extent, current, LineGeometryStageFns::line_geometry_stage_fns_f32(baselines[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)] - baselines[usize::try_from(i).unwrap_or(0)]), { let __min_a22 = f_base_ascent as f64; let __min_b22 = line_object_ascent[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)] as f64; if __min_a22.is_nan() || __min_b22.is_nan() { f64::NAN } else { if __min_a22 > __min_b22 { __min_a22 } else if __min_b22 > __min_a22 { __min_b22 } else if __min_a22 == 0.0 && __min_b22 == 0.0 { if __min_a22.is_sign_negative() { __min_b22 } else { __min_a22 } } else { __min_a22 } } });
            let nominal = LineGeometryStageFns::line_geometry_stage_fns_f32(baselines[usize::try_from(i).unwrap_or(0)] + base_bottom_extent);
            let boundary = LineGeometryStageFns::line_geometry_stage_fns_f32(baselines[usize::try_from(i).unwrap_or(0)] + boundary_extent);
            { while bottoms.len() <= usize::try_from(i).unwrap_or(0) { bottoms.push(0.0); } bottoms[usize::try_from(i).unwrap_or(0)] = boundary; };
            { while tops.len() <= usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0) { tops.push(0.0); } tops[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)] = boundary; };
            { while boundary_shifts.len() <= usize::try_from(i).unwrap_or(0) { boundary_shifts.push(0.0); } boundary_shifts[usize::try_from(i).unwrap_or(0)] = LineGeometryStageFns::line_geometry_stage_fns_f32(boundary - nominal); };
        }
        let trailing = if u32::try_from((line_solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { 0.0f64 } else { { let __min_a23 = 0.0f64 as f64; let __min_b23 = LineGeometryStageFns::line_geometry_stage_fns_f32(line_object_descent[usize::try_from(u32::wrapping_sub(u32::try_from((line_object_descent.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)] - base_bottom_extent) as f64; if __min_a23.is_nan() || __min_b23.is_nan() { f64::NAN } else { if __min_a23 > __min_b23 { __min_a23 } else if __min_b23 > __min_a23 { __min_b23 } else if __min_a23 == 0.0 && __min_b23 == 0.0 { if __min_a23.is_sign_negative() { __min_b23 } else { __min_a23 } } else { __min_a23 } } } };
        if i32::from_ne_bytes(((u32::try_from((line_solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            { let __grow_idx3 = usize::try_from(u32::wrapping_sub(u32::try_from((bottoms.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0); while bottoms.len() <= __grow_idx3 { bottoms.push(0.0); } bottoms[__grow_idx3] = LineGeometryStageFns::line_geometry_stage_fns_f32(LineGeometryStageFns::line_geometry_stage_fns_f32(baselines[usize::try_from(u32::wrapping_sub(u32::try_from((baselines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)] + base_bottom_extent) + trailing); };
        }
        let mut object_indices: Vec<u32> = vec![];
        for i in 0..match u32::try_from(object_line_extra.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if object_line_extra[usize::try_from(i).unwrap_or(0)] > (0 as f64) {
                object_indices.push(i);
            }
        }
        let mut has_inline_objects = false;
        match &(inline_object_by_cluster_index) {
            Some(__option6) => {
                has_inline_objects = i32::from_ne_bytes(((u32::from_ne_bytes(((__option6.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes()) > (0);
            }
            None => {
            }
        }
        let iod = if has_inline_objects { Some(InlineObjectLineHeightDecisionInfo::new(base_line_metrics.height, base_ascent, base_descent, existing_interline_space, minimum_clearance, line_object_ascent.to_vec(), line_object_descent.to_vec(), object_line_extra.to_vec(), boundary_shifts.to_vec(), trailing, object_indices.to_vec(), if u32::try_from((object_indices.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 && trailing == 0 as f64 { UString::from("ExistingInterlineSpaceFitsInlineObjects") } else { UString::from("InlineObjectInterlineCollision") }.as_ustr())) } else { None };
        return LineVerticalGeometryStageResult::new((rd).clone(), (iod).clone(), baselines.to_vec(), tops.to_vec(), bottoms.to_vec());
    }

    pub fn line_geometry_stage_fns_line_metrics(self_: &Vec<ClusterMetricDecision>, explicit_line_height: Option<f64>, default_line_height: f64, spacing_floor: Option<f64>) -> ResolvedLineMetrics {
        let mut floor_value = 0.0f64;
        match &(spacing_floor) {
            Some(__option7) => {
                floor_value = *__option7;
            }
            None => {
            }
        }
        let floor = floor_value;
        if u32::try_from((self_.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            let mut h_value = default_line_height;
            match &(explicit_line_height) {
                Some(__option8) => {
                    h_value = *__option8;
                }
                None => {
                }
            }
            let h = h_value;
            return ResolvedLineMetrics::new(h * 0.75f64, h, Some(0 as f64));
        }
        let mut pipeline_result: Vec<ClusterMetricDecision> = Vec::new();
        for v in self_ {
            if v.layout_metrics.clone().metric_box == MetricBox::IdeographicEmBox {
                pipeline_result.push(v.clone());
            }
        }
        let mut src = (pipeline_result).clone();
        if u32::try_from((src.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            src = (*self_).clone();
        }
        let mut a = ((src[0usize]).clone().layout_metrics).clone().ascent;
        let mut d = ((src[0usize]).clone().layout_metrics).clone().descent;
        for x in &src {
            a = { let __min_a24 = a as f64; let __min_b24 = (x.layout_metrics).clone().ascent as f64; if __min_a24.is_nan() || __min_b24.is_nan() { f64::NAN } else { if __min_a24 > __min_b24 { __min_a24 } else if __min_b24 > __min_a24 { __min_b24 } else if __min_a24 == 0.0 && __min_b24 == 0.0 { if __min_a24.is_sign_negative() { __min_b24 } else { __min_a24 } } else { __min_a24 } } };
            d = { let __min_a25 = d as f64; let __min_b25 = (x.layout_metrics).clone().descent as f64; if __min_a25.is_nan() || __min_b25.is_nan() { f64::NAN } else { if __min_a25 > __min_b25 { __min_a25 } else if __min_b25 > __min_a25 { __min_b25 } else if __min_a25 == 0.0 && __min_b25 == 0.0 { if __min_a25.is_sign_negative() { __min_b25 } else { __min_a25 } } else { __min_a25 } } };
        }
        let natural = a + d;
        let mut requested_value = default_line_height;
        match &(explicit_line_height) {
            Some(__option9) => {
                requested_value = *__option9;
            }
            None => {
            }
        }
        let requested = requested_value;
        let h = { let __min_a26 = requested as f64; let __min_b26 = (natural + floor) as f64; if __min_a26.is_nan() || __min_b26.is_nan() { f64::NAN } else { if __min_a26 > __min_b26 { __min_a26 } else if __min_b26 > __min_a26 { __min_b26 } else if __min_a26 == 0.0 && __min_b26 == 0.0 { if __min_a26.is_sign_negative() { __min_b26 } else { __min_a26 } } else { __min_a26 } } };
        return ResolvedLineMetrics::new(a + (h - natural) / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0), h, Some(h - natural));
    }

    pub(crate) fn line_geometry_stage_fns_f32(v: f64) -> f64 {
        return FPHelper::i32_to_float(FPHelper::float_to_i32(v));
    }

    pub fn line_geometry_stage_fns_resolve_inline_object_line_boundary_extent(nominal_boundary_extent: f64, current_content_bottom_extent: f64, baseline_distance: f64, next_content_top_extent: f64) -> f64 {
        return { let __min_a30 = current_content_bottom_extent as f64; let __min_b30 = { let __min_a32 = nominal_boundary_extent as f64; let __min_b32 = { let __min_a33 = current_content_bottom_extent as f64; let __min_b33 = LineGeometryStageFns::line_geometry_stage_fns_f32(baseline_distance - next_content_top_extent) as f64; if __min_a33.is_nan() || __min_b33.is_nan() { f64::NAN } else { if __min_a33 > __min_b33 { __min_a33 } else if __min_b33 > __min_a33 { __min_b33 } else if __min_a33 == 0.0 && __min_b33 == 0.0 { if __min_a33.is_sign_negative() { __min_b33 } else { __min_a33 } } else { __min_a33 } } } as f64; if __min_a32.is_nan() || __min_b32.is_nan() { f64::NAN } else { if __min_a32 < __min_b32 { __min_a32 } else if __min_b32 < __min_a32 { __min_b32 } else if __min_a32 == 0.0 && __min_b32 == 0.0 { if __min_a32.is_sign_negative() { __min_a32 } else { __min_b32 } } else { __min_a32 } } } as f64; if __min_a30.is_nan() || __min_b30.is_nan() { f64::NAN } else { if __min_a30 > __min_b30 { __min_a30 } else if __min_b30 > __min_a30 { __min_b30 } else if __min_a30 == 0.0 && __min_b30 == 0.0 { if __min_a30.is_sign_negative() { __min_b30 } else { __min_a30 } } else { __min_a30 } } };
    }

    pub fn line_geometry_stage_fns_cluster_index_range_for(self_: &Vec<Cluster>, r: TextRange) -> Option<IntRange> {
        return PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&self_, (r).clone());
    }
}
