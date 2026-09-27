use crate::org::tiqian::protocol::named_error::NamedError;


#[derive(Clone, Copy)]
pub struct NamedErrorNames;

impl NamedErrorNames {
    pub fn named_error_names_variants() -> Vec<NamedError> {
        return vec![
    NamedError::EmptyParagraph,
    NamedError::InvalidMaximumMeasure,
    NamedError::InvalidFontSize,
    NamedError::InvalidLineHeight,
    NamedError::InvalidFirstLineIndent,
    NamedError::InvalidFontWeight,
    NamedError::InvalidEmphasisDotGapEm,
    NamedError::MissingExplicitFontFamilies,
    NamedError::InvalidTextSpanRange,
    NamedError::MissingTextSpanFontFamilies,
    NamedError::InvalidTextSpanFontSize,
    NamedError::InvalidTextSpanFontWeight,
    NamedError::InvalidTextSpanBaselineShift,
    NamedError::InvalidSourceBoundary,
    NamedError::InvalidLineBreakSpanRange,
    NamedError::InvalidInlineBoxRange,
    NamedError::InvalidInlineBoxGeometry,
    NamedError::InvalidInlineObjectRange,
    NamedError::InvalidInlineObjectAdvance,
    NamedError::InvalidInlineObjectVerticalGeometry,
    NamedError::InvalidDecorationRange,
];
    }

    pub fn named_error_names_describe(error: NamedError) -> String {
        return match error {
            NamedError::EmptyParagraph => "EmptyParagraph".to_string().to_string(),
            NamedError::InvalidMaximumMeasure => "InvalidMaximumMeasure".to_string().to_string(),
            NamedError::InvalidFontSize => "InvalidFontSize".to_string().to_string(),
            NamedError::InvalidLineHeight => "InvalidLineHeight".to_string().to_string(),
            NamedError::InvalidFirstLineIndent => "InvalidFirstLineIndent".to_string().to_string(),
            NamedError::InvalidFontWeight => "InvalidFontWeight".to_string().to_string(),
            NamedError::InvalidEmphasisDotGapEm => "InvalidEmphasisDotGapEm".to_string().to_string(),
            NamedError::MissingExplicitFontFamilies => "MissingExplicitFontFamilies".to_string().to_string(),
            NamedError::InvalidTextSpanRange => "InvalidTextSpanRange".to_string().to_string(),
            NamedError::MissingTextSpanFontFamilies => "MissingTextSpanFontFamilies".to_string().to_string(),
            NamedError::InvalidTextSpanFontSize => "InvalidTextSpanFontSize".to_string().to_string(),
            NamedError::InvalidTextSpanFontWeight => "InvalidTextSpanFontWeight".to_string().to_string(),
            NamedError::InvalidTextSpanBaselineShift => "InvalidTextSpanBaselineShift".to_string().to_string(),
            NamedError::InvalidSourceBoundary => "InvalidSourceBoundary".to_string().to_string(),
            NamedError::InvalidLineBreakSpanRange => "InvalidLineBreakSpanRange".to_string().to_string(),
            NamedError::InvalidInlineBoxRange => "InvalidInlineBoxRange".to_string().to_string(),
            NamedError::InvalidInlineBoxGeometry => "InvalidInlineBoxGeometry".to_string().to_string(),
            NamedError::InvalidInlineObjectRange => "InvalidInlineObjectRange".to_string().to_string(),
            NamedError::InvalidInlineObjectAdvance => "InvalidInlineObjectAdvance".to_string().to_string(),
            NamedError::InvalidInlineObjectVerticalGeometry => "InvalidInlineObjectVerticalGeometry".to_string().to_string(),
            NamedError::InvalidDecorationRange => "InvalidDecorationRange".to_string().to_string(),
        };
    }
}
