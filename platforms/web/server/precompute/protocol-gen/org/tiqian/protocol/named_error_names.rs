use crate::org::tiqian::protocol::paragraph_request_exception::NamedError;
use crate::runtime::u_string::UString;


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

    pub fn named_error_names_describe(error: NamedError) -> UString {
        return match error {
            NamedError::EmptyParagraph => UString::from("EmptyParagraph").to_ustring().to_ustring(),
            NamedError::InvalidMaximumMeasure => UString::from("InvalidMaximumMeasure").to_ustring().to_ustring(),
            NamedError::InvalidFontSize => UString::from("InvalidFontSize").to_ustring().to_ustring(),
            NamedError::InvalidLineHeight => UString::from("InvalidLineHeight").to_ustring().to_ustring(),
            NamedError::InvalidFirstLineIndent => UString::from("InvalidFirstLineIndent").to_ustring().to_ustring(),
            NamedError::InvalidFontWeight => UString::from("InvalidFontWeight").to_ustring().to_ustring(),
            NamedError::InvalidEmphasisDotGapEm => UString::from("InvalidEmphasisDotGapEm").to_ustring().to_ustring(),
            NamedError::MissingExplicitFontFamilies => UString::from("MissingExplicitFontFamilies").to_ustring().to_ustring(),
            NamedError::InvalidTextSpanRange => UString::from("InvalidTextSpanRange").to_ustring().to_ustring(),
            NamedError::MissingTextSpanFontFamilies => UString::from("MissingTextSpanFontFamilies").to_ustring().to_ustring(),
            NamedError::InvalidTextSpanFontSize => UString::from("InvalidTextSpanFontSize").to_ustring().to_ustring(),
            NamedError::InvalidTextSpanFontWeight => UString::from("InvalidTextSpanFontWeight").to_ustring().to_ustring(),
            NamedError::InvalidTextSpanBaselineShift => UString::from("InvalidTextSpanBaselineShift").to_ustring().to_ustring(),
            NamedError::InvalidSourceBoundary => UString::from("InvalidSourceBoundary").to_ustring().to_ustring(),
            NamedError::InvalidLineBreakSpanRange => UString::from("InvalidLineBreakSpanRange").to_ustring().to_ustring(),
            NamedError::InvalidInlineBoxRange => UString::from("InvalidInlineBoxRange").to_ustring().to_ustring(),
            NamedError::InvalidInlineBoxGeometry => UString::from("InvalidInlineBoxGeometry").to_ustring().to_ustring(),
            NamedError::InvalidInlineObjectRange => UString::from("InvalidInlineObjectRange").to_ustring().to_ustring(),
            NamedError::InvalidInlineObjectAdvance => UString::from("InvalidInlineObjectAdvance").to_ustring().to_ustring(),
            NamedError::InvalidInlineObjectVerticalGeometry => UString::from("InvalidInlineObjectVerticalGeometry").to_ustring().to_ustring(),
            NamedError::InvalidDecorationRange => UString::from("InvalidDecorationRange").to_ustring().to_ustring(),
        };
    }
}
