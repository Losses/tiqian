use crate::org::tiqian::protocol::paragraph_request_exception::NamedError;
use crate::runtime::u_string::UString;


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

    pub fn named_error_test_support_golden() -> Vec<UString> {
        return vec![
    UString::from("EmptyParagraph").to_ustring(),
    UString::from("InvalidMaximumMeasure").to_ustring(),
    UString::from("InvalidFontSize").to_ustring(),
    UString::from("InvalidLineHeight").to_ustring(),
    UString::from("InvalidFirstLineIndent").to_ustring(),
    UString::from("InvalidFontWeight").to_ustring(),
    UString::from("InvalidEmphasisDotGapEm").to_ustring(),
    UString::from("MissingExplicitFontFamilies").to_ustring(),
    UString::from("InvalidTextSpanRange").to_ustring(),
    UString::from("MissingTextSpanFontFamilies").to_ustring(),
    UString::from("InvalidTextSpanFontSize").to_ustring(),
    UString::from("InvalidTextSpanFontWeight").to_ustring(),
    UString::from("InvalidTextSpanBaselineShift").to_ustring(),
    UString::from("InvalidSourceBoundary").to_ustring(),
    UString::from("InvalidLineBreakSpanRange").to_ustring(),
    UString::from("InvalidInlineBoxRange").to_ustring(),
    UString::from("InvalidInlineBoxGeometry").to_ustring(),
    UString::from("InvalidInlineObjectRange").to_ustring(),
    UString::from("InvalidInlineObjectAdvance").to_ustring(),
    UString::from("InvalidInlineObjectVerticalGeometry").to_ustring(),
    UString::from("InvalidDecorationRange").to_ustring(),
];
    }
}
