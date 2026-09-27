use crate::org::tiqian::protocol::paragraph_request_exception::NamedError;


#[derive(Clone, Copy)]
pub struct NamedErrorTestSupport;

impl NamedErrorTestSupport {
    pub fn named_error_test_support_variant_at(index_idx: u32) -> NamedError {
        let v: NamedError;
        if index_idx == 0 {
            v = NamedError::EmptyParagraph;
        } else {
            if index_idx == 1 {
                v = NamedError::InvalidMaximumMeasure;
            } else {
                if index_idx == 2 {
                    v = NamedError::InvalidFontSize;
                } else {
                    if index_idx == 3 {
                        v = NamedError::InvalidLineHeight;
                    } else {
                        if index_idx == 4 {
                            v = NamedError::InvalidFirstLineIndent;
                        } else {
                            if index_idx == 5 {
                                v = NamedError::InvalidFontWeight;
                            } else {
                                if index_idx == 6 {
                                    v = NamedError::InvalidEmphasisDotGapEm;
                                } else {
                                    if index_idx == 7 {
                                        v = NamedError::MissingExplicitFontFamilies;
                                    } else {
                                        if index_idx == 8 {
                                            v = NamedError::InvalidTextSpanRange;
                                        } else {
                                            if index_idx == 9 {
                                                v = NamedError::MissingTextSpanFontFamilies;
                                            } else {
                                                if index_idx == 10 {
                                                    v = NamedError::InvalidTextSpanFontSize;
                                                } else {
                                                    if index_idx == 11 {
                                                        v = NamedError::InvalidTextSpanFontWeight;
                                                    } else {
                                                        if index_idx == 12 {
                                                            v = NamedError::InvalidTextSpanBaselineShift;
                                                        } else {
                                                            if index_idx == 13 {
                                                                v = NamedError::InvalidSourceBoundary;
                                                            } else {
                                                                if index_idx == 14 {
                                                                    v = NamedError::InvalidLineBreakSpanRange;
                                                                } else {
                                                                    if index_idx == 15 {
                                                                        v = NamedError::InvalidInlineBoxRange;
                                                                    } else {
                                                                        if index_idx == 16 {
                                                                            v = NamedError::InvalidInlineBoxGeometry;
                                                                        } else {
                                                                            if index_idx == 17 {
                                                                                v = NamedError::InvalidInlineObjectRange;
                                                                            } else {
                                                                                if index_idx == 18 {
                                                                                    v = NamedError::InvalidInlineObjectAdvance;
                                                                                } else {
                                                                                    if index_idx == 19 {
                                                                                        v = NamedError::InvalidInlineObjectVerticalGeometry;
                                                                                    } else {
                                                                                        if index_idx == 20 {
                                                                                            v = NamedError::InvalidDecorationRange;
                                                                                        } else {
                                                                                            v = NamedError::InvalidDecorationRange;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return v;
    }

    pub fn named_error_test_support_golden() -> Vec<String> {
        return vec![
    "EmptyParagraph".to_string(),
    "InvalidMaximumMeasure".to_string(),
    "InvalidFontSize".to_string(),
    "InvalidLineHeight".to_string(),
    "InvalidFirstLineIndent".to_string(),
    "InvalidFontWeight".to_string(),
    "InvalidEmphasisDotGapEm".to_string(),
    "MissingExplicitFontFamilies".to_string(),
    "InvalidTextSpanRange".to_string(),
    "MissingTextSpanFontFamilies".to_string(),
    "InvalidTextSpanFontSize".to_string(),
    "InvalidTextSpanFontWeight".to_string(),
    "InvalidTextSpanBaselineShift".to_string(),
    "InvalidSourceBoundary".to_string(),
    "InvalidLineBreakSpanRange".to_string(),
    "InvalidInlineBoxRange".to_string(),
    "InvalidInlineBoxGeometry".to_string(),
    "InvalidInlineObjectRange".to_string(),
    "InvalidInlineObjectAdvance".to_string(),
    "InvalidInlineObjectVerticalGeometry".to_string(),
    "InvalidDecorationRange".to_string(),
];
    }
}
