use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::writing_mode::WritingMode;


#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphStyle {
    pub last_line_alignment: LastLineAlignment,
    pub writing_mode: WritingMode,
    pub line_height: Option<f64>,
    pub first_line_indent: Option<Ic>,
    pub block_indent: Ic,
    pub first_line_indent_policy: MeasureAdaptiveFirstLineIndent,
    pub line_length_grid: LineLengthGrid,
    pub ruby_line_height_mode: RubyLineHeightMode,
    pub inline_object_minimum_clearance_em: f64,
    pub emphasis_dot_gap_em: f64,
}

impl ParagraphStyle {
    pub const PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM: f64 = 0.1f64;

    pub const PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM: f64 = 0.1f64;

    pub fn new(last_line_alignment: Option<LastLineAlignment>, writing_mode: Option<WritingMode>, line_height: Option<f64>, first_line_indent: Option<Ic>, block_indent: Option<Ic>, first_line_indent_policy: Option<MeasureAdaptiveFirstLineIndent>, line_length_grid:
Option<LineLengthGrid>, ruby_line_height_mode: Option<RubyLineHeightMode>, inline_object_minimum_clearance_em: Option<f64>, emphasis_dot_gap_em: Option<f64>) -> Self {
        let last_line_alignment = last_line_alignment.unwrap_or_else(|| LastLineAlignment::Start);
        let writing_mode = writing_mode.unwrap_or_else(|| WritingMode::HorizontalTb);
        let line_height = line_height.or_else(|| None);
        let first_line_indent = first_line_indent.or_else(|| None);
        let block_indent = block_indent.unwrap_or_else(|| Ic::zero());
        let first_line_indent_policy = first_line_indent_policy.unwrap_or_else(|| MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0)));
        let line_length_grid = line_length_grid.unwrap_or_else(|| LineLengthGrid::new(Some(true), None));
        let ruby_line_height_mode = ruby_line_height_mode.unwrap_or_else(|| RubyLineHeightMode::PerLine);
        let inline_object_minimum_clearance_em = inline_object_minimum_clearance_em.unwrap_or_else(|| ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM);
        let emphasis_dot_gap_em = emphasis_dot_gap_em.unwrap_or_else(|| ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM);
        Self {
            last_line_alignment: last_line_alignment,
            writing_mode: writing_mode,
            line_height: line_height,
            first_line_indent: first_line_indent,
            block_indent: block_indent,
            first_line_indent_policy: first_line_indent_policy,
            line_length_grid: line_length_grid,
            ruby_line_height_mode: ruby_line_height_mode,
            inline_object_minimum_clearance_em: inline_object_minimum_clearance_em,
            emphasis_dot_gap_em: emphasis_dot_gap_em,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "ParagraphStyle(",
            "lastLineAlignment=",
            self.last_line_alignment.name(),
            ", ",
            "writingMode=",
            self.writing_mode.name(),
            ", ",
            "lineHeight=",
            match self.line_height { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "firstLineIndent=",
            (match &(self.first_line_indent) { None => "null".to_string(), Some(__option) => __option.to_string() }),
            ", ",
            "blockIndent=",
            (self.block_indent).clone().to_string(),
            ", ",
            "firstLineIndentPolicy=",
            (self.first_line_indent_policy).clone().to_string(),
            ", ",
            "lineLengthGrid=",
            (self.line_length_grid).clone().to_string(),
            ", ",
            "rubyLineHeightMode=",
            self.ruby_line_height_mode.name(),
            ", ",
            "inlineObjectMinimumClearanceEm=",
            self.inline_object_minimum_clearance_em,
            ", ",
            "emphasisDotGapEm=",
            self.emphasis_dot_gap_em,
            ")"
        );
    }
}
