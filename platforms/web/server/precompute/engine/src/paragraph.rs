//! Paragraph precompute over the engine ABI (ADR 0050 amendment
//! `PrecomputeInRust`).
//!
//! The typed request and the LayoutInput packing are the Rust port of
//! `PrecomputeWire.kt`. Domain validation is single-sourced in the
//! generated protocol model (tiqian_protocol_gen ParagraphRequestChecks,
//! Stage1-P5): this module only adapts its lane request into the
//! generated one and maps the generated issue variants to NamedError
//! through the generated Display, whose strings are the published issue
//! names. The f64 to f32 narrowing of the packing matches the Kotlin
//! toFloat() casts; the validation runs on the f64 values the caller
//! passed. The engine call exists only when build.rs linked the engine
//! archive.

use tiqian::layout_request::{InlineBoxSpec, LayoutRequest, LineBreakSpanSpec, TextSpanSpec};
use tiqian::NamedError;
use tiqian_protocol_gen::org::tiqian::protocol::inline_box_input::InlineBoxInput as GenInlineBoxInput;
use tiqian_protocol_gen::org::tiqian::protocol::line_break_span_input::LineBreakSpanInput as GenLineBreakSpanInput;
use tiqian_protocol_gen::org::tiqian::protocol::paragraph_request::ParagraphRequest as GeneratedRequest;
use tiqian_protocol_gen::org::tiqian::protocol::paragraph_request_checks::ParagraphRequestChecks;
use tiqian_protocol_gen::org::tiqian::protocol::text_span_input::TextSpanInput as GenTextSpanInput;

// The request lane keeps the code types of the packed engine ABI; callers
// building a ParagraphRequest take the codes from here.
pub use tiqian::layout_request::{InlineBoxOuterSpacingCode, LineBreakPolicyCode};

use crate::js_compat::kotlin_to_float;

#[cfg(tiqian_engine_link)]
use crate::plan::Plan;

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphRequest {
    pub font_session_id: String,
    pub text: String,
    pub max_width_px: f64,
    pub font_families: Vec<String>,
    pub font_size_px: f64,
    pub line_height_px: f64,
    pub locale: String,
    pub font_weight: i32,
    pub italic: bool,
    pub first_line_indent_ic: f64,
    pub line_length_grid_enabled: bool,
    pub source_boundaries: Vec<i32>,
    pub text_spans: Vec<TextSpanInputLane>,
    pub line_break_spans: Vec<LineBreakSpanInputLane>,
    pub inline_boxes: Vec<InlineBoxInputLane>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextSpanInputLane {
    pub start: i32,
    pub end: i32,
    pub families: Vec<String>,
    pub font_size_px: f64,
    pub font_weight: i32,
    pub italic: bool,
    pub baseline_shift: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineBreakSpanInputLane {
    pub start: i32,
    pub end: i32,
    pub policy: LineBreakPolicyCode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InlineBoxInputLane {
    pub start: i32,
    pub end: i32,
    pub inline_start: f64,
    pub inline_end: f64,
    pub outer_spacing: InlineBoxOuterSpacingCode,
}

pub type TextSpanInput = TextSpanInputLane;
pub type LineBreakSpanInput = LineBreakSpanInputLane;
pub type InlineBoxInput = InlineBoxInputLane;

impl ParagraphRequest {
    /// Runs the single-sourced domain checks (generated
    /// ParagraphRequestChecks) and reports the first failure as a
    /// NamedError whose name is the generated Display string.
    pub fn validate(&self) -> Result<(), NamedError> {
        ParagraphRequestChecks::paragraph_request_checks_validate(self.to_generated())
            .map_err(|error| NamedError(error.to_string()))
    }

    /// Adapts this lane request into the generated model. The i32 to u32
    /// casts keep the check outcomes: a negative range end wraps above
    /// any text length and fails the same named range check, and an
    /// out-of-range weight fails the weight check. The generated model
    /// optional emphasis gap and its inline-object and decoration
    /// sections are fields this Rust consumer does not send; they read
    /// as absent and stay unvalidated here.
    fn to_generated(&self) -> GeneratedRequest {
        GeneratedRequest {
            font_session_id: self.font_session_id.clone(),
            text: self.text.clone(),
            max_width_px: self.max_width_px,
            font_families: self.font_families.clone(),
            font_size_px: self.font_size_px,
            line_height_px: self.line_height_px,
            locale: self.locale.clone(),
            font_weight: self.font_weight as u32,
            italic: self.italic,
            first_line_indent_ic: self.first_line_indent_ic,
            line_length_grid_enabled: self.line_length_grid_enabled,
            emphasis_dot_gap_em: None,
            source_boundaries: self
                .source_boundaries
                .iter()
                .map(|value| *value as u32)
                .collect(),
            text_spans: self
                .text_spans
                .iter()
                .map(|span| GenTextSpanInput {
                    start: span.start as u32,
                    end: span.end as u32,
                    families: span.families.clone(),
                    font_size_px: span.font_size_px,
                    font_weight: span.font_weight as u32,
                    italic: span.italic,
                    baseline_shift: span.baseline_shift,
                })
                .collect(),
            line_break_spans: self
                .line_break_spans
                .iter()
                .map(|span| GenLineBreakSpanInput {
                    start: span.start as u32,
                    end: span.end as u32,
                    policy: line_break_policy_name(span.policy),
                })
                .collect(),
            inline_boxes: self
                .inline_boxes
                .iter()
                .map(|box_input| GenInlineBoxInput {
                    start: box_input.start as u32,
                    end: box_input.end as u32,
                    inline_start: box_input.inline_start,
                    inline_end: box_input.inline_end,
                    outer_spacing: outer_spacing_name(box_input.outer_spacing),
                })
                .collect(),
            inline_objects: Vec::new(),
            decorations: Vec::new(),
        }
    }

    /// Validates, then builds the engine-level packed request.
    pub fn to_layout_request(&self) -> Result<LayoutRequest, NamedError> {
        self.validate()?;
        Ok(LayoutRequest {
            max_width_px: kotlin_to_float(self.max_width_px),
            font_size_px: kotlin_to_float(self.font_size_px),
            line_height_px: kotlin_to_float(self.line_height_px),
            first_line_indent_ic: kotlin_to_float(self.first_line_indent_ic),
            font_weight: self.font_weight,
            italic: self.italic,
            line_length_grid_enabled: self.line_length_grid_enabled,
            locale: self.locale.clone(),
            families: self.font_families.clone(),
            text: self.text.clone(),
            text_spans: self
                .text_spans
                .iter()
                .map(|span| TextSpanSpec {
                    start: span.start,
                    end: span.end,
                    font_size_px: kotlin_to_float(span.font_size_px),
                    font_weight: span.font_weight,
                    italic: span.italic,
                    baseline_shift: kotlin_to_float(span.baseline_shift),
                    families: span.families.clone(),
                })
                .collect(),
            source_boundaries: self.source_boundaries.clone(),
            line_break_spans: self
                .line_break_spans
                .iter()
                .map(|span| LineBreakSpanSpec {
                    start: span.start,
                    end: span.end,
                    policy: span.policy,
                })
                .collect(),
            inline_boxes: self
                .inline_boxes
                .iter()
                .map(|inline_box| InlineBoxSpec {
                    start: inline_box.start,
                    end: inline_box.end,
                    inline_start: kotlin_to_float(inline_box.inline_start),
                    inline_end: kotlin_to_float(inline_box.inline_end),
                    outer_spacing: inline_box.outer_spacing,
                })
                .collect(),
            font_session_id: self.font_session_id.clone(),
        })
    }
}

/// Packs, calls the engine over the ABI and deserializes the packed plan.
/// Exists only when the engine archive is linked (`TIQIAN_NATIVE_LIB_DIR` at
/// build time); the font backend must already be installed.
#[cfg(tiqian_engine_link)]
pub fn precompute_paragraph(request: &ParagraphRequest) -> Result<Plan, NamedError> {
    let packed = request.to_layout_request()?.pack()?;
    let bytes = tiqian::engine::layout_paragraph(&packed)?;
    Plan::from_packed_bytes(&bytes)
}

/// Kotlin `String.length`: UTF-16 code units. Every engine range and boundary
/// lives in this space.
pub fn utf16_length(text: &str) -> i32 {
    text.chars()
        .map(|c| {
            let code = c as u32;
            if code <= 0xffff { 1 } else { 2 }
        })
        .sum()
}

fn line_break_policy_name(policy: LineBreakPolicyCode) -> String {
    match policy {
        LineBreakPolicyCode::ProgressiveTechnical => "ProgressiveTechnical".to_string(),
    }
}

fn outer_spacing_name(outer_spacing: InlineBoxOuterSpacingCode) -> String {
    match outer_spacing {
        InlineBoxOuterSpacingCode::Narrow => "Narrow".to_string(),
        InlineBoxOuterSpacingCode::Source => "Source".to_string(),
    }
}
