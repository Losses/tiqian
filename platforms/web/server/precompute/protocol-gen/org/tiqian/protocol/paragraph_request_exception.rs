#[derive(Debug, Clone, PartialEq)]
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

impl std::fmt::Display for NamedError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedError::EmptyParagraph => write!(formatter, "{}", "EmptyParagraph"),
            NamedError::InvalidMaximumMeasure => write!(formatter, "{}", "InvalidMaximumMeasure"),
            NamedError::InvalidFontSize => write!(formatter, "{}", "InvalidFontSize"),
            NamedError::InvalidLineHeight => write!(formatter, "{}", "InvalidLineHeight"),
            NamedError::InvalidFirstLineIndent => write!(formatter, "{}", "InvalidFirstLineIndent"),
            NamedError::InvalidFontWeight => write!(formatter, "{}", "InvalidFontWeight"),
            NamedError::InvalidEmphasisDotGapEm => write!(formatter, "{}", "InvalidEmphasisDotGapEm"),
            NamedError::MissingExplicitFontFamilies => write!(formatter, "{}", "MissingExplicitFontFamilies"),
            NamedError::InvalidTextSpanRange => write!(formatter, "{}", "InvalidTextSpanRange"),
            NamedError::MissingTextSpanFontFamilies => write!(formatter, "{}", "MissingTextSpanFontFamilies"),
            NamedError::InvalidTextSpanFontSize => write!(formatter, "{}", "InvalidTextSpanFontSize"),
            NamedError::InvalidTextSpanFontWeight => write!(formatter, "{}", "InvalidTextSpanFontWeight"),
            NamedError::InvalidTextSpanBaselineShift => write!(formatter, "{}", "InvalidTextSpanBaselineShift"),
            NamedError::InvalidSourceBoundary => write!(formatter, "{}", "InvalidSourceBoundary"),
            NamedError::InvalidLineBreakSpanRange => write!(formatter, "{}", "InvalidLineBreakSpanRange"),
            NamedError::InvalidInlineBoxRange => write!(formatter, "{}", "InvalidInlineBoxRange"),
            NamedError::InvalidInlineBoxGeometry => write!(formatter, "{}", "InvalidInlineBoxGeometry"),
            NamedError::InvalidInlineObjectRange => write!(formatter, "{}", "InvalidInlineObjectRange"),
            NamedError::InvalidInlineObjectAdvance => write!(formatter, "{}", "InvalidInlineObjectAdvance"),
            NamedError::InvalidInlineObjectVerticalGeometry => write!(formatter, "{}", "InvalidInlineObjectVerticalGeometry"),
            NamedError::InvalidDecorationRange => write!(formatter, "{}", "InvalidDecorationRange"),
        }
    }
}

impl std::error::Error for NamedError {}
