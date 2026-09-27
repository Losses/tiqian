use crate::org::tiqian::protocol::plan_end_reason::PlanEndReason;
use crate::org::tiqian::protocol::plan_style_delta::PlanStyleDelta;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlanEmphasisRange {
    pub start: u32,
    pub end: u32,
}

pub fn compare_plan_emphasis_range(a: &PlanEmphasisRange, b: &PlanEmphasisRange) -> i32 {
    let cmp_start = if a.start < b.start { -1 } else if a.start > b.start { 1 } else { 0 };
    if cmp_start != 0 { return cmp_start; }
    let cmp_end = if a.end < b.end { -1 } else if a.end > b.end { 1 } else { 0 };
    if cmp_end != 0 { return cmp_end; }
    0
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanInlineEdge {
    pub offset: u32,
    pub inline_start: Option<f64>,
    pub inline_end: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanRuby {
    pub base_range_start: u32,
    pub base_range_end: u32,
    pub text: String,
    pub center_x: f64,
    pub baseline_y: f64,
    pub font_size: f64,
    pub font_weight: u32,
    pub font_families: Vec<String>,
    pub ascent: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanBopomofo {
    pub base_range_start: u32,
    pub base_range_end: u32,
    pub text: String,
    pub font_weight: u32,
    pub font_families: Vec<String>,
    pub placements: Vec<PlanBopomofoPlacement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanBopomofoPlacement {
    pub text: String,
    pub role: String,
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanDecorationSegment {
    pub kind: String,
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub source_range_start: u32,
    pub source_range_end: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanEmphasisDot {
    pub cluster_range_start: Option<f64>,
    pub anchor_x: f64,
    pub anchor_y: f64,
    pub dot_diameter: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanLine {
    pub range_start: u32,
    pub range_end: u32,
    pub top: f64,
    pub bottom: f64,
    pub baseline: f64,
    pub indent: f64,
    pub visual_width: f64,
    pub hyphen_advance: f64,
    pub end_reason: PlanEndReason,
    pub cells: Vec<PlanCell>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanCell {
    pub range_start: u32,
    pub range_end: u32,
    pub source: String,
    pub display: String,
    pub draw_x: f64,
    pub natural_width: f64,
    pub leading_layout_advance: f64,
    pub shaping_boundary: bool,
    pub open_type_features: Vec<String>,
    pub render_font_family: Option<String>,
    pub dash_strategy: Option<String>,
    pub shaping_language: Option<String>,
    pub resolved_face: Option<String>,
    pub glyph_ids: Option<String>,
    pub shaping_evidence: Option<String>,
    pub punctuation_ink_floor: Option<f64>,
    pub punctuation_body_width: Option<f64>,
    pub latin: bool,
    pub advance: Option<f64>,
    pub inline_object: Option<f64>,
    pub style_delta: Option<PlanStyleDelta>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub width: f64,
    pub height: f64,
    pub lines: Vec<PlanLine>,
    pub emphasis_ranges: Vec<PlanEmphasisRange>,
    pub inline_edges: Vec<PlanInlineEdge>,
    pub ruby_decisions: Vec<PlanRuby>,
    pub bopomofo_decisions: Vec<PlanBopomofo>,
    pub font_size: Option<f64>,
    pub overlay_width: Option<f64>,
    pub decoration_segments: Vec<PlanDecorationSegment>,
    pub emphasis_dots: Vec<PlanEmphasisDot>,
}
