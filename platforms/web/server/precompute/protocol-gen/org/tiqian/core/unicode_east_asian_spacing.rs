use crate::org::tiqian::core::east_asian_spacing_data::EastAsianSpacingData;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::source_interaction_boundaries::SourceInteractionBoundaries;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct UnicodeEastAsianSpacing;

impl UnicodeEastAsianSpacing {
    pub const UNICODE_EAST_ASIAN_SPACING_DATA_REVISION: &UStr = unsafe { &*(&[0x0064u16, 0x0072u16, 0x0061u16, 0x0066u16, 0x0074u16, 0x002Du16, 0x0032u16, 0x0030u16, 0x0032u16, 0x0034u16, 0x002Du16, 0x0031u16, 0x0032u16, 0x002Du16, 0x0031u16, 0x0036u16] as *const [u16] as *const crate::runtime::u_string::UStr) };
    pub const UNICODE_EAST_ASIAN_SPACING_DATA_SOURCE: &UStr = unsafe { &*(&[0x0068u16, 0x0074u16, 0x0074u16, 0x0070u16, 0x0073u16, 0x003Au16, 0x002Fu16, 0x002Fu16, 0x0077u16, 0x0077u16, 0x0077u16, 0x002Eu16, 0x0075u16, 0x006Eu16, 0x0069u16, 0x0063u16, 0x006Fu16, 0x0064u16, 0x0065u16, 0x002Eu16, 0x006Fu16, 0x0072u16, 0x0067u16, 0x002Fu16, 0x0072u16, 0x0065u16, 0x0070u16, 0x006Fu16, 0x0072u16, 0x0074u16, 0x0073u16, 0x002Fu16, 0x0074u16, 0x0072u16, 0x0035u16, 0x0039u16, 0x002Fu16, 0x0065u16, 0x0061u16, 0x0073u16, 0x0074u16, 0x002Du16, 0x0061u16, 0x0073u16, 0x0069u16, 0x0061u16, 0x006Eu16, 0x002Du16, 0x0073u16, 0x0070u16, 0x0061u16, 0x0063u16, 0x0069u16, 0x006Eu16, 0x0067u16, 0x002Eu16, 0x0074u16, 0x0078u16, 0x0074u16] as *const [u16] as *const crate::runtime::u_string::UStr) };
    pub const UNICODE_EAST_ASIAN_SPACING_DATA_SHA256: &UStr = unsafe { &*(&[0x0034u16, 0x0039u16, 0x0066u16, 0x0065u16, 0x0033u16, 0x0034u16, 0x0030u16, 0x0061u16, 0x0039u16, 0x0036u16, 0x0034u16, 0x0061u16, 0x0036u16, 0x0065u16, 0x0038u16, 0x0065u16, 0x0030u16, 0x0065u16, 0x0062u16, 0x0063u16, 0x0033u16, 0x0030u16, 0x0030u16, 0x0039u16, 0x0039u16, 0x0037u16, 0x0030u16, 0x0039u16, 0x0063u16, 0x0036u16, 0x0036u16, 0x0035u16, 0x0063u16, 0x0063u16, 0x0036u16, 0x0031u16, 0x0033u16, 0x0038u16, 0x0064u16, 0x0034u16, 0x0034u16, 0x0034u16, 0x0062u16, 0x0035u16, 0x0063u16, 0x0033u16, 0x0036u16, 0x0064u16, 0x0063u16, 0x0033u16, 0x0033u16, 0x0036u16, 0x0036u16, 0x0030u16, 0x0034u16, 0x0030u16, 0x0034u16, 0x0037u16, 0x0066u16, 0x0031u16, 0x0030u16, 0x0031u16, 0x0030u16, 0x0066u16] as *const [u16] as *const crate::runtime::u_string::UStr) };
    pub const UNICODE_EAST_ASIAN_SPACING_LANGUAGE_REGISTRY_REVISION: &UStr = unsafe { &*(&[0x0032u16, 0x0030u16, 0x0032u16, 0x0036u16, 0x002Du16, 0x0030u16, 0x0036u16, 0x002Du16, 0x0031u16, 0x0034u16] as *const [u16] as *const crate::runtime::u_string::UStr) };
    pub const UNICODE_EAST_ASIAN_SPACING_LANGUAGE_REGISTRY_SOURCE: &UStr = unsafe { &*(&[0x0068u16, 0x0074u16, 0x0074u16, 0x0070u16, 0x0073u16, 0x003Au16, 0x002Fu16, 0x002Fu16, 0x0077u16, 0x0077u16, 0x0077u16, 0x002Eu16, 0x0069u16, 0x0061u16, 0x006Eu16, 0x0061u16, 0x002Eu16, 0x006Fu16, 0x0072u16, 0x0067u16, 0x002Fu16, 0x0061u16, 0x0073u16, 0x0073u16, 0x0069u16, 0x0067u16, 0x006Eu16, 0x006Du16, 0x0065u16, 0x006Eu16, 0x0074u16, 0x0073u16, 0x002Fu16, 0x006Cu16, 0x0061u16, 0x006Eu16, 0x0067u16, 0x0075u16, 0x0061u16, 0x0067u16, 0x0065u16, 0x002Du16, 0x0073u16, 0x0075u16, 0x0062u16, 0x0074u16, 0x0061u16, 0x0067u16, 0x002Du16, 0x0072u16, 0x0065u16, 0x0067u16, 0x0069u16, 0x0073u16, 0x0074u16, 0x0072u16, 0x0079u16, 0x002Fu16, 0x006Cu16, 0x0061u16, 0x006Eu16, 0x0067u16, 0x0075u16, 0x0061u16, 0x0067u16, 0x0065u16, 0x002Du16, 0x0073u16, 0x0075u16, 0x0062u16, 0x0074u16, 0x0061u16, 0x0067u16, 0x002Du16, 0x0072u16, 0x0065u16, 0x0067u16, 0x0069u16, 0x0073u16, 0x0074u16, 0x0072u16, 0x0079u16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    pub fn unicode_east_asian_spacing_is_chinese_language_context(locale: &UStr) -> bool {
        let language = UnicodeEastAsianSpacing::unicode_east_asian_spacing_language_subtag(locale);
        if language == UString::from("zh") {
            return true;
        }
        return language == UString::from("cdo") || language == UString::from("cjy") || language == UString::from("cmn") || language == UString::from("cnp") || language == UString::from("cpx") || language == UString::from("csp") || language == UString::from("czh") || language == UString::from("czo") || language == UString::from("gan") || language == UString::from("hak") || language == UString::from("hnm") || language == UString::from("hsn") || language == UString::from("luh") || language == UString::from("lzh") || language == UString::from("mnp") || language == UString::from("nan") || language == UString::from("sjc") || language == UString::from("wuu") || language == UString::from("yue");
    }

    pub fn unicode_east_asian_spacing_property_of(code_point: u32) -> Result<EastAsianSpacingValue, TextRangeError> {
        let _ = UnicodeEastAsianSpacing::unicode_east_asian_spacing_validate_scalar(code_point)?;
        return Ok(EastAsianSpacingData::east_asian_spacing_data_lookup(code_point)?);
    }

    pub fn unicode_east_asian_spacing_resolved_for_grapheme_cluster(grapheme_cluster: &UStr, locale: &UStr) -> Result<EastAsianSpacingValue, TextRangeError> {
    let __units = u_string::units(&grapheme_cluster);
    let __count = u_string::unit_count(&grapheme_cluster);
        if __count == 0 {
            return Ok(EastAsianSpacingValue::Other);
        }
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count) as i32).to_ne_bytes())) {
            let code_point = SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(grapheme_cluster, index, __count);
            if UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_enclosing_mark(code_point) {
                return Ok(EastAsianSpacingValue::Other);
            }
            let advance = if i32::from_ne_bytes(((code_point) as i32).to_ne_bytes()) > (65535) { 2 } else { 1 };
            index = u32::wrapping_add(index, advance);
        }
        let property = UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(grapheme_cluster, 0, __count))?;
        if property == EastAsianSpacingValue::Conditional {
            return Ok(if UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(locale) { EastAsianSpacingValue::Narrow } else { EastAsianSpacingValue::Other });
        }
        return Ok(property);
    }

    pub fn unicode_east_asian_spacing_resolved_edges(text: &UStr, locale: &UStr) -> Result<EastAsianSpacingEdges, TextRangeError> {
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
        while (i32::from_ne_bytes(((u32::wrapping_add(index, 1)) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let value = UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(u_string::substring(&text, { let v: u32 = boundaries[usize::try_from(index).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }, { let v: u32 = boundaries[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }).as_ustr(), locale)?;
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
        if code_point > 2147483647 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) > (1114111) {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Not a Unicode scalar value: ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(code_point)).as_str())); __s }).as_str()) });
        }
        if i32::from_ne_bytes(((code_point) as i32).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 57343 {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Surrogate is not a Unicode scalar value: ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(code_point)).as_str())); __s }).as_str()) });
        }
        Ok(())
    }

    pub(crate) fn unicode_east_asian_spacing_language_subtag(locale: &UStr) -> UString {
    let __units2 = u_string::units(&locale);
    let __count2 = u_string::unit_count(&locale);
        let mut end = u_string::find_from(&(locale), UString::from("-").as_ustr(), 0);
        let underscore = u_string::find_from(&(locale), UString::from("_").as_ustr(), 0);
        if end < (0) || (underscore) >= 0 && (underscore) < (end) {
            end = underscore;
        }
        if end < (0) {
            end = i32::from_ne_bytes(((__count2) as i32).to_ne_bytes());
        }
        return u_string::substring(&locale, 0i32, i32::from_ne_bytes(((end) as i32).to_ne_bytes())).to_lowercase();
    }

    pub(crate) fn unicode_east_asian_spacing_is_enclosing_mark(code_point: u32) -> bool {
        return code_point == 1160 || code_point == 1161 || code_point == 6846 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 8413 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 8416 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 42608 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 42610 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 42612 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 42621;
    }
}
