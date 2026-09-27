#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphRequestError {
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

impl std::fmt::Display for ParagraphRequestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphRequestError::EmptyParagraph => write!(formatter, "{}", "EmptyParagraph"),
            ParagraphRequestError::InvalidMaximumMeasure => write!(formatter, "{}", "InvalidMaximumMeasure"),
            ParagraphRequestError::InvalidFontSize => write!(formatter, "{}", "InvalidFontSize"),
            ParagraphRequestError::InvalidLineHeight => write!(formatter, "{}", "InvalidLineHeight"),
            ParagraphRequestError::InvalidFirstLineIndent => write!(formatter, "{}", "InvalidFirstLineIndent"),
            ParagraphRequestError::InvalidFontWeight => write!(formatter, "{}", "InvalidFontWeight"),
            ParagraphRequestError::InvalidEmphasisDotGapEm => write!(formatter, "{}", "InvalidEmphasisDotGapEm"),
            ParagraphRequestError::MissingExplicitFontFamilies => write!(formatter, "{}", "MissingExplicitFontFamilies"),
            ParagraphRequestError::InvalidTextSpanRange => write!(formatter, "{}", "InvalidTextSpanRange"),
            ParagraphRequestError::MissingTextSpanFontFamilies => write!(formatter, "{}", "MissingTextSpanFontFamilies"),
            ParagraphRequestError::InvalidTextSpanFontSize => write!(formatter, "{}", "InvalidTextSpanFontSize"),
            ParagraphRequestError::InvalidTextSpanFontWeight => write!(formatter, "{}", "InvalidTextSpanFontWeight"),
            ParagraphRequestError::InvalidTextSpanBaselineShift => write!(formatter, "{}", "InvalidTextSpanBaselineShift"),
            ParagraphRequestError::InvalidSourceBoundary => write!(formatter, "{}", "InvalidSourceBoundary"),
            ParagraphRequestError::InvalidLineBreakSpanRange => write!(formatter, "{}", "InvalidLineBreakSpanRange"),
            ParagraphRequestError::InvalidInlineBoxRange => write!(formatter, "{}", "InvalidInlineBoxRange"),
            ParagraphRequestError::InvalidInlineBoxGeometry => write!(formatter, "{}", "InvalidInlineBoxGeometry"),
            ParagraphRequestError::InvalidInlineObjectRange => write!(formatter, "{}", "InvalidInlineObjectRange"),
            ParagraphRequestError::InvalidInlineObjectAdvance => write!(formatter, "{}", "InvalidInlineObjectAdvance"),
            ParagraphRequestError::InvalidInlineObjectVerticalGeometry => write!(formatter, "{}", "InvalidInlineObjectVerticalGeometry"),
            ParagraphRequestError::InvalidDecorationRange => write!(formatter, "{}", "InvalidDecorationRange"),
        }
    }
}

impl std::error::Error for ParagraphRequestError {}
