use crate::org::tiqian::protocol::paragraph_request::ParagraphRequest;
use crate::org::tiqian::protocol::paragraph_request_checks::ParagraphRequestChecks;
use crate::org::tiqian::protocol::paragraph_request_exception::NamedError;


#[derive(Clone, Copy)]
pub struct ParagraphRequestTestSupport;

impl ParagraphRequestTestSupport {
    pub fn paragraph_request_test_support_request() -> ParagraphRequest {
        return ParagraphRequest { font_session_id: "tq-font-test-1".to_string(), text: "正文一段".to_string(), max_width_px: 80.0f64, font_families: vec!["Fake CJK".to_string()], font_size_px: 16.0f64, line_height_px: 24.0f64, locale: "zh-Hans".to_string(), font_weight: 400, italic:
false, first_line_indent_ic: 0.0f64, line_length_grid_enabled: false, emphasis_dot_gap_em: None, source_boundaries: vec![], text_spans: vec![], line_break_spans: vec![], inline_boxes: vec![], inline_objects: vec![], decorations: vec![] };
    }

    pub fn paragraph_request_test_support_issue_name_of(error: NamedError) -> String {
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

    pub fn paragraph_request_test_support_issue_of(request: ParagraphRequest) -> String {
        let mut name = String::new();
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

    pub fn paragraph_request_test_support_with_text(text: &str) -> ParagraphRequest {
        let mut copy = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        copy.text = text.to_string();
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

    pub fn paragraph_request_test_support_with_families(families: &Vec<String>) -> ParagraphRequest {
        let mut copy = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        copy.font_families = (*families).clone();
        return copy;
    }
}
