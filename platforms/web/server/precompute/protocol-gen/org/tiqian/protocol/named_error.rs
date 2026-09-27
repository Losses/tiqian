#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NamedError {
    EmptyParagraph,
    InvalidMaximumMeasure,
    InvalidFontSize,
    InvalidLineHeight,
    InvalidFirstLineIndent,
    InvalidFontWeight,
    InvalidEmphasisDotGapEm,
    MissingExplicitFontFamilies,
    InvalidTextSpanRange,
    MissingTextSpanFontFamilies,
    InvalidTextSpanFontSize,
    InvalidTextSpanFontWeight,
    InvalidTextSpanBaselineShift,
    InvalidSourceBoundary,
    InvalidLineBreakSpanRange,
    InvalidInlineBoxRange,
    InvalidInlineBoxGeometry,
    InvalidInlineObjectRange,
    InvalidInlineObjectAdvance,
    InvalidInlineObjectVerticalGeometry,
    InvalidDecorationRange,
}

pub fn compare_named_error(a: &NamedError, b: &NamedError) -> i32 {
    if a == b { return 0; }
    fn rank(v: &NamedError) -> i32 {
        match v {
            NamedError::EmptyParagraph => 0,
            NamedError::InvalidMaximumMeasure => 1,
            NamedError::InvalidFontSize => 2,
            NamedError::InvalidLineHeight => 3,
            NamedError::InvalidFirstLineIndent => 4,
            NamedError::InvalidFontWeight => 5,
            NamedError::InvalidEmphasisDotGapEm => 6,
            NamedError::MissingExplicitFontFamilies => 7,
            NamedError::InvalidTextSpanRange => 8,
            NamedError::MissingTextSpanFontFamilies => 9,
            NamedError::InvalidTextSpanFontSize => 10,
            NamedError::InvalidTextSpanFontWeight => 11,
            NamedError::InvalidTextSpanBaselineShift => 12,
            NamedError::InvalidSourceBoundary => 13,
            NamedError::InvalidLineBreakSpanRange => 14,
            NamedError::InvalidInlineBoxRange => 15,
            NamedError::InvalidInlineBoxGeometry => 16,
            NamedError::InvalidInlineObjectRange => 17,
            NamedError::InvalidInlineObjectAdvance => 18,
            NamedError::InvalidInlineObjectVerticalGeometry => 19,
            NamedError::InvalidDecorationRange => 20,
        }
    }
    rank(a) - rank(b)
}

impl NamedError {
    pub fn to_string(&self) -> String {
        match self {
            NamedError::EmptyParagraph => "EmptyParagraph".to_string(),
            NamedError::InvalidMaximumMeasure => "InvalidMaximumMeasure".to_string(),
            NamedError::InvalidFontSize => "InvalidFontSize".to_string(),
            NamedError::InvalidLineHeight => "InvalidLineHeight".to_string(),
            NamedError::InvalidFirstLineIndent => "InvalidFirstLineIndent".to_string(),
            NamedError::InvalidFontWeight => "InvalidFontWeight".to_string(),
            NamedError::InvalidEmphasisDotGapEm => "InvalidEmphasisDotGapEm".to_string(),
            NamedError::MissingExplicitFontFamilies => "MissingExplicitFontFamilies".to_string(),
            NamedError::InvalidTextSpanRange => "InvalidTextSpanRange".to_string(),
            NamedError::MissingTextSpanFontFamilies => "MissingTextSpanFontFamilies".to_string(),
            NamedError::InvalidTextSpanFontSize => "InvalidTextSpanFontSize".to_string(),
            NamedError::InvalidTextSpanFontWeight => "InvalidTextSpanFontWeight".to_string(),
            NamedError::InvalidTextSpanBaselineShift => "InvalidTextSpanBaselineShift".to_string(),
            NamedError::InvalidSourceBoundary => "InvalidSourceBoundary".to_string(),
            NamedError::InvalidLineBreakSpanRange => "InvalidLineBreakSpanRange".to_string(),
            NamedError::InvalidInlineBoxRange => "InvalidInlineBoxRange".to_string(),
            NamedError::InvalidInlineBoxGeometry => "InvalidInlineBoxGeometry".to_string(),
            NamedError::InvalidInlineObjectRange => "InvalidInlineObjectRange".to_string(),
            NamedError::InvalidInlineObjectAdvance => "InvalidInlineObjectAdvance".to_string(),
            NamedError::InvalidInlineObjectVerticalGeometry => "InvalidInlineObjectVerticalGeometry".to_string(),
            NamedError::InvalidDecorationRange => "InvalidDecorationRange".to_string(),
        }
    }
}
