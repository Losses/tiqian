use crate::org::tiqian::protocol::paragraph_request::ParagraphRequest;
use crate::org::tiqian::protocol::paragraph_request_checks::ParagraphRequestChecks;
use crate::org::tiqian::protocol::paragraph_request_exception::NamedError;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct ParagraphRequestTestSupport;

impl ParagraphRequestTestSupport {
    pub fn paragraph_request_test_support_request() -> ParagraphRequest {
        return ParagraphRequest { font_session_id: UString::from("tq-font-test-1").to_ustring(), text: UString::from("正文一段").to_ustring(), max_width_px: 80.0f64, font_families: vec![UString::from("Fake CJK").to_ustring()], font_size_px: 16.0f64, line_height_px: 24.0f64,
locale: UString::from("zh-Hans").to_ustring(), font_weight: 400, italic: false, first_line_indent_ic: 0.0f64, line_length_grid_enabled: false, emphasis_dot_gap_em: None, source_boundaries: vec![], text_spans: vec![], line_break_spans: vec![], inline_boxes: vec![],
inline_objects: vec![], decorations: vec![] };
    }

    pub fn paragraph_request_test_support_issue_name_of(error: NamedError) -> UString {
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

    pub fn paragraph_request_test_support_issue_of(request: ParagraphRequest) -> UString {
        let mut name = UString::new();
        let __outcome: Result<(), NamedError> = (|| {
            let _ = ParagraphRequestChecks::paragraph_request_checks_validate((request).clone())?;
            Ok(())
        })();
        match __outcome {
            Ok(_) => {}
            Err(error) => {
                name = ParagraphRequestTestSupport::paragraph_request_test_support_issue_name_of(error);
            }
        }
        return name;
    }

    pub fn paragraph_request_test_support_with_text(text: &UStr) -> ParagraphRequest {
        let mut copy = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        copy.text = text.to_ustring();
        return copy;
    }

    pub fn paragraph_request_test_support_with_max_width(value: f64) -> ParagraphRequest {
        let mut copy = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        copy.max_width_px = value;
        return copy;
    }

    pub fn paragraph_request_test_support_with_font_size(value: f64) -> ParagraphRequest {
        let mut copy = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        copy.font_size_px = value;
        return copy;
    }

    pub fn paragraph_request_test_support_with_line_height(value: f64) -> ParagraphRequest {
        let mut copy = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        copy.line_height_px = value;
        return copy;
    }

    pub fn paragraph_request_test_support_with_indent(value: f64) -> ParagraphRequest {
        let mut copy = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        copy.first_line_indent_ic = value;
        return copy;
    }

    pub fn paragraph_request_test_support_with_weight(value: u32) -> ParagraphRequest {
        let mut copy = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        copy.font_weight = value;
        return copy;
    }

    pub fn paragraph_request_test_support_with_gap(value: f64) -> ParagraphRequest {
        let mut copy = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        copy.emphasis_dot_gap_em = Some(value);
        return copy;
    }

    pub fn paragraph_request_test_support_with_families(families: &Vec<UString>) -> ParagraphRequest {
        let mut copy = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        copy.font_families = (*families).clone();
        return copy;
    }
}
