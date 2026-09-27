use crate::org::tiqian::protocol::decoration_input::DecorationInput;
use crate::org::tiqian::protocol::inline_box_input::InlineBoxInput;
use crate::org::tiqian::protocol::inline_object_input::InlineObjectInput;
use crate::org::tiqian::protocol::line_break_span_input::LineBreakSpanInput;
use crate::org::tiqian::protocol::text_span_input::TextSpanInput;


#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphRequest {
    pub font_session_id: String,
    pub text: String,
    pub max_width_px: f64,
    pub font_families: Vec<String>,
    pub font_size_px: f64,
    pub line_height_px: f64,
    pub locale: String,
    pub font_weight: u32,
    pub italic: bool,
    pub first_line_indent_ic: f64,
    pub line_length_grid_enabled: bool,
    pub emphasis_dot_gap_em: Option<f64>,
    pub source_boundaries: Vec<u32>,
    pub text_spans: Vec<TextSpanInput>,
    pub line_break_spans: Vec<LineBreakSpanInput>,
    pub inline_boxes: Vec<InlineBoxInput>,
    pub inline_objects: Vec<InlineObjectInput>,
    pub decorations: Vec<DecorationInput>,
}
