use crate::org::tiqian::protocol::paragraph_request::ParagraphRequest;
use crate::org::tiqian::protocol::paragraph_request_exception::ParagraphRequestError;
use crate::runtime::u_string;


#[derive(Clone, Copy)]
pub struct ParagraphRequestChecks;

impl ParagraphRequestChecks {
    pub fn paragraph_request_checks_validate(request: ParagraphRequest) -> Result<(), ParagraphRequestError> {
        if u_string::unit_count(&((request.text).to_string())) == 0 || ParagraphRequestChecks::paragraph_request_checks_is_blank_text((request.text).to_string().as_str()) {
            return Err(ParagraphRequestError::EmptyParagraph);
        }
        if !(request.max_width_px).is_finite() || (request.max_width_px) <= 0.0f64 {
            return Err(ParagraphRequestError::InvalidMaximumMeasure);
        }
        if !(request.font_size_px).is_finite() || (request.font_size_px) <= 0.0f64 {
            return Err(ParagraphRequestError::InvalidFontSize);
        }
        if !(request.line_height_px).is_finite() || (request.line_height_px) <= 0.0f64 {
            return Err(ParagraphRequestError::InvalidLineHeight);
        }
        if !(request.first_line_indent_ic).is_finite() {
            return Err(ParagraphRequestError::InvalidFirstLineIndent);
        }
        if i32::from_ne_bytes((request.font_weight).to_ne_bytes()) < (1) || (i32::from_ne_bytes((request.font_weight).to_ne_bytes())) > (1000) {
            return Err(ParagraphRequestError::InvalidFontWeight);
        }
        let gap_em = match &(request.emphasis_dot_gap_em) { None => 0.1f64, Some(__option) => *__option };
        if !(gap_em).is_finite() || (gap_em) < (0.0f64) {
            return Err(ParagraphRequestError::InvalidEmphasisDotGapEm);
        }
        if !ParagraphRequestChecks::paragraph_request_checks_has_non_blank_family(&request.font_families) {
            return Err(ParagraphRequestError::MissingExplicitFontFamilies);
        }
        let text_length = u_string::unit_count(&((request.text).to_string()));
        let mut span_index_idx = 0u32;
        while (i32::from_ne_bytes((span_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((request.text_spans.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let span = (request.text_spans[usize::try_from(span_index_idx).unwrap_or(0)]).clone();
            if !ParagraphRequestChecks::paragraph_request_checks_valid_range(span.start, span.end, text_length) {
                return Err(ParagraphRequestError::InvalidTextSpanRange);
            }
            if !ParagraphRequestChecks::paragraph_request_checks_has_non_blank_family(&span.families) {
                return Err(ParagraphRequestError::MissingTextSpanFontFamilies);
            }
            if !(span.font_size_px).is_finite() || (span.font_size_px) <= 0.0f64 {
                return Err(ParagraphRequestError::InvalidTextSpanFontSize);
            }
            if i32::from_ne_bytes((span.font_weight).to_ne_bytes()) < (1) || (i32::from_ne_bytes((span.font_weight).to_ne_bytes())) > (1000) {
                return Err(ParagraphRequestError::InvalidTextSpanFontWeight);
            }
            if !(span.baseline_shift).is_finite() {
                return Err(ParagraphRequestError::InvalidTextSpanBaselineShift);
            }
            span_index_idx = u32::wrapping_add(span_index_idx, 1);
        }
        let mut boundary_index_idx = 0u32;
        while (i32::from_ne_bytes((boundary_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((request.source_boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let boundary = request.source_boundaries[usize::try_from(boundary_index_idx).unwrap_or(0)];
            if boundary > 2147483647 || (i32::from_ne_bytes((boundary).to_ne_bytes())) > (i32::from_ne_bytes((text_length).to_ne_bytes())) {
                return Err(ParagraphRequestError::InvalidSourceBoundary);
            }
            boundary_index_idx = u32::wrapping_add(boundary_index_idx, 1);
        }
        let mut break_index_idx = 0u32;
        while (i32::from_ne_bytes((break_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((request.line_break_spans.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let span = (request.line_break_spans[usize::try_from(break_index_idx).unwrap_or(0)]).clone();
            if !ParagraphRequestChecks::paragraph_request_checks_valid_range(span.start, span.end, text_length) {
                return Err(ParagraphRequestError::InvalidLineBreakSpanRange);
            }
            break_index_idx = u32::wrapping_add(break_index_idx, 1);
        }
        let mut box_index_idx = 0u32;
        while (i32::from_ne_bytes((box_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((request.inline_boxes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let r#box = (request.inline_boxes[usize::try_from(box_index_idx).unwrap_or(0)]).clone();
            if !ParagraphRequestChecks::paragraph_request_checks_valid_range(r#box.start, r#box.end, text_length) {
                return Err(ParagraphRequestError::InvalidInlineBoxRange);
            }
            if !(r#box.inline_start).is_finite() || !(r#box.inline_end).is_finite() {
                return Err(ParagraphRequestError::InvalidInlineBoxGeometry);
            }
            box_index_idx = u32::wrapping_add(box_index_idx, 1);
        }
        let mut object_index_idx = 0u32;
        while (i32::from_ne_bytes((object_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((request.inline_objects.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let object = request.inline_objects[usize::try_from(object_index_idx).unwrap_or(0)];
            if !ParagraphRequestChecks::paragraph_request_checks_valid_range(object.start, object.end, text_length) {
                return Err(ParagraphRequestError::InvalidInlineObjectRange);
            }
            if !(object.advance).is_finite() || (object.advance) < (0.0f64) {
                return Err(ParagraphRequestError::InvalidInlineObjectAdvance);
            }
            if !(object.ascent).is_finite() || !(object.descent).is_finite() {
                return Err(ParagraphRequestError::InvalidInlineObjectVerticalGeometry);
            }
            object_index_idx = u32::wrapping_add(object_index_idx, 1);
        }
        let mut decoration_index_idx = 0u32;
        while (i32::from_ne_bytes((decoration_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((request.decorations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let decoration = (request.decorations[usize::try_from(decoration_index_idx).unwrap_or(0)]).clone();
            if !ParagraphRequestChecks::paragraph_request_checks_valid_range(decoration.start, decoration.end, text_length) {
                return Err(ParagraphRequestError::InvalidDecorationRange);
            }
            decoration_index_idx = u32::wrapping_add(decoration_index_idx, 1);
        }
        Ok(())
    }

    pub fn paragraph_request_checks_valid_range(start: u32, end: u32, text_length: u32) -> bool {
        return (start) <= 2147483647 && (i32::from_ne_bytes((start).to_ne_bytes())) < (i32::from_ne_bytes((end).to_ne_bytes())) && (i32::from_ne_bytes((end).to_ne_bytes())) <= i32::from_ne_bytes((text_length).to_ne_bytes());
    }

    pub fn paragraph_request_checks_is_kotlin_whitespace(code: u32) -> bool {
        if i32::from_ne_bytes((code).to_ne_bytes()) >= 9 && (i32::from_ne_bytes((code).to_ne_bytes())) <= 13 {
            return true;
        }
        if i32::from_ne_bytes((code).to_ne_bytes()) >= 28 && (i32::from_ne_bytes((code).to_ne_bytes())) <= 31 {
            return true;
        }
        if code == 32 || code == 133 || code == 5760 {
            return true;
        }
        if i32::from_ne_bytes((code).to_ne_bytes()) >= 8192 && (i32::from_ne_bytes((code).to_ne_bytes())) <= 8202 {
            return true;
        }
        return code == 8232 || code == 8233 || code == 8287 || code == 12288;
    }

    pub fn paragraph_request_checks_is_blank_text(text: &str) -> bool {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut index_idx = 0u32;
        let __units1 = u_string::units(&text);
        let __count1 = u_string::unit_count(&text);
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            let code = u_string::unit_at_from(&__units1, index_idx).unwrap_or(0);
            if i32::from_ne_bytes((code).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes((code).to_ne_bytes())) <= 57343 {
                return false;
            }
            if !ParagraphRequestChecks::paragraph_request_checks_is_kotlin_whitespace(code) {
                return false;
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return true;
    }

    pub fn paragraph_request_checks_has_non_blank_family(families: &Vec<String>) -> bool {
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((families.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if !ParagraphRequestChecks::paragraph_request_checks_is_blank_text((families[usize::try_from(index_idx).unwrap_or(0)]).clone().as_str()) {
                return true;
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return false;
    }
}
