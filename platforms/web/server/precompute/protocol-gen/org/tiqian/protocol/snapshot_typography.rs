use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotTypography {
    pub font_families: Vec<UString>,
    pub font_size_px: f64,
    pub line_height_px: f64,
    pub locale: UString,
    pub font_weight: u32,
    pub italic: bool,
    pub first_line_indent_ic: f64,
    pub line_length_grid_enabled: bool,
    pub letter_spacing_px: f64,
    pub font_feature_settings: UString,
    pub font_variation_settings: UString,
    pub font_variant_numeric: UString,
}
