use crate::org::tiqian::core::east_asian_spacing_data::EastAsianSpacingData;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::source_interaction_boundaries::SourceInteractionBoundaries;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string;


#[derive(Clone, Copy)]
pub struct UnicodeEastAsianSpacing;

impl UnicodeEastAsianSpacing {
    pub const UNICODE_EAST_ASIAN_SPACING_DATA_REVISION: &str = "draft-2024-12-16";
    pub const UNICODE_EAST_ASIAN_SPACING_DATA_SOURCE: &str = "https://www.unicode.org/reports/tr59/east-asian-spacing.txt";
    pub const UNICODE_EAST_ASIAN_SPACING_DATA_SHA256: &str = "49fe340a964a6e8e0ebc30099709c665cc6138d444b5c36dc336604047f1010f";
    pub const UNICODE_EAST_ASIAN_SPACING_LANGUAGE_REGISTRY_REVISION: &str = "2026-06-14";
    pub const UNICODE_EAST_ASIAN_SPACING_LANGUAGE_REGISTRY_SOURCE: &str = "https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry";

    pub fn unicode_east_asian_spacing_is_chinese_language_context(locale: &str) -> bool {
        let language = UnicodeEastAsianSpacing::unicode_east_asian_spacing_language_subtag(locale);
        if language == "zh" {
            return true;
        }
        return language == "cdo" || language == "cjy" || language == "cmn" || language == "cnp" || language == "cpx" || language == "csp" || language == "czh" || language == "czo" || language == "gan" || language == "hak" || language == "hnm" || language == "hsn" || language ==
"luh" || language == "lzh" || language == "mnp" || language == "nan" || language == "sjc" || language == "wuu" || language == "yue";
    }

    pub fn unicode_east_asian_spacing_property_of(code_point: u32) -> Result<EastAsianSpacingValue, TextRangeError> {
        let _ = UnicodeEastAsianSpacing::unicode_east_asian_spacing_validate_scalar(code_point)?;
        return Ok(EastAsianSpacingData::east_asian_spacing_data_lookup(code_point)?);
    }

    pub fn unicode_east_asian_spacing_resolved_for_grapheme_cluster(grapheme_cluster: &str, locale: &str) -> Result<EastAsianSpacingValue, TextRangeError> {
    let __units = u_string::units(&grapheme_cluster);
    let __count = u_string::unit_count(&grapheme_cluster);
        if __count == 0 {
            return Ok(EastAsianSpacingValue::Other);
        }
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            let code_point = SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(grapheme_cluster, index, __count);
            if UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_enclosing_mark(code_point) {
                return Ok(EastAsianSpacingValue::Other);
            }
            let advance = if i32::from_ne_bytes((code_point).to_ne_bytes()) > (65535) { 2 } else { 1 };
            index = u32::wrapping_add(index, advance);
        }
        let property = UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(grapheme_cluster, 0, __count))?;
        if property == EastAsianSpacingValue::Conditional {
            return Ok(if UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(locale) { EastAsianSpacingValue::Narrow } else { EastAsianSpacingValue::Other });
        }
        return Ok(property);
    }

    pub fn unicode_east_asian_spacing_resolved_edges(text: &str, locale: &str) -> Result<EastAsianSpacingEdges, TextRangeError> {
    let __units1 = u_string::units(&text);
    let __count1 = u_string::unit_count(&text);
        if __count1 == 0 {
            return Ok(EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false));
        }
        let boundaries = SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(text, TextRange::new(0u32, __count1)?);
        let mut index = 0u32;
        let mut leading = EastAsianSpacingValue::Other;
        let mut trailing = EastAsianSpacingValue::Other;
        let mut contains_wide = false;
        while (i32::from_ne_bytes((u32::wrapping_add(index, 1)).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let value = UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(u_string::substring(&text, { let v: u32 = boundaries[usize::try_from(index).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }, { let v: u32 =
boundaries[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }).as_str(), locale)?;
            if index == 0 {
                leading = value;
            }
            trailing = value;
            if value == EastAsianSpacingValue::Wide {
                contains_wide = true;
            }
            index = u32::wrapping_add(index, 1);
        }
        return Ok(EastAsianSpacingEdges::new(leading, trailing, contains_wide));
    }

    pub(crate) fn unicode_east_asian_spacing_validate_scalar(code_point: u32) -> Result<(), TextRangeError> {
        if code_point > 2147483647 || (i32::from_ne_bytes((code_point).to_ne_bytes())) > (1114111) {
            return Err(TextRangeError::Message { text: format!("{}{}",
            "Not a Unicode scalar value: ",
            crate::runtime::int_text::IntText::int_text(code_point)
        ).to_string() });
        }
        if i32::from_ne_bytes((code_point).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes((code_point).to_ne_bytes())) <= 57343 {
            return Err(TextRangeError::Message { text: format!("{}{}",
            "Surrogate is not a Unicode scalar value: ",
            crate::runtime::int_text::IntText::int_text(code_point)
        ).to_string() });
        }
        Ok(())
    }

    pub(crate) fn unicode_east_asian_spacing_language_subtag(locale: &str) -> String {
    let __units2 = u_string::units(&locale);
    let __count2 = u_string::unit_count(&locale);
        let mut end = u_string::find_from(&locale, "-", 0);
        let underscore = u_string::find_from(&locale, "_", 0);
        if end < (0) || (underscore) >= 0 && (underscore) < (end) {
            end = underscore;
        }
        if end < (0) {
            end = i32::from_ne_bytes((__count2).to_ne_bytes());
        }
        return u_string::substring(&locale, 0i32, i32::from_ne_bytes((end).to_ne_bytes())).to_lowercase();
    }

    pub(crate) fn unicode_east_asian_spacing_is_enclosing_mark(code_point: u32) -> bool {
        return code_point == 1160 || code_point == 1161 || code_point == 6846 || (i32::from_ne_bytes((code_point).to_ne_bytes())) >= 8413 && (i32::from_ne_bytes((code_point).to_ne_bytes())) <= 8416 || (i32::from_ne_bytes((code_point).to_ne_bytes())) >= 42608 &&
(i32::from_ne_bytes((code_point).to_ne_bytes())) <= 42610 || (i32::from_ne_bytes((code_point).to_ne_bytes())) >= 42612 && (i32::from_ne_bytes((code_point).to_ne_bytes())) <= 42621;
    }
}
