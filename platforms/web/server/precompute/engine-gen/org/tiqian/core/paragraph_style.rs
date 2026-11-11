use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::runtime::u_string::UString;


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

    pub fn new(last_line_alignment: Option<LastLineAlignment>, writing_mode: Option<WritingMode>, line_height: Option<f64>, first_line_indent: Option<Ic>, block_indent: Option<Ic>, first_line_indent_policy: Option<MeasureAdaptiveFirstLineIndent>, line_length_grid: Option<LineLengthGrid>, ruby_line_height_mode: Option<RubyLineHeightMode>, inline_object_minimum_clearance_em: Option<f64>, emphasis_dot_gap_em: Option<f64>) -> Self {
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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ParagraphStyle(")); __s += &(UString::from("lastLineAlignment=")); __s += UString::from(self.last_line_alignment.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("writingMode=")); __s += UString::from(self.writing_mode.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineHeight=")); __s += &(match self.line_height { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("firstLineIndent=")); __s += (match &(self.first_line_indent) { None => UString::from("null"), Some(__option) => UString::from((__option).to_string().as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("blockIndent=")); __s += UString::from(((self.block_indent).clone()).to_string().as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("firstLineIndentPolicy=")); __s += UString::from(format!("{}", (self.first_line_indent_policy).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineLengthGrid=")); __s += UString::from(format!("{}", (self.line_length_grid).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("rubyLineHeightMode=")); __s += UString::from(self.ruby_line_height_mode.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineObjectMinimumClearanceEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.inline_object_minimum_clearance_em)); __s += &(UString::from(", ")); __s += &(UString::from("emphasisDotGapEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.emphasis_dot_gap_em)); __s += &(UString::from(")")); __s }).as_str());
    }
}
