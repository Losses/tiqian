use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_policy::FontCandidate;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::runtime::u_string;


#[derive(Clone, Copy)]
pub struct TextShaperCoverageTestSupport;

impl TextShaperCoverageTestSupport {
    pub fn text_shaper_coverage_test_support_input(text: &str, role: Option<FontRole>, display_text: Option<String>, features: Option<Vec<String>>) -> Result<ShapingInput, TextRangeError> {
        let role = role.unwrap_or_else(|| FontRole::LatinText);
        let actual_role = role;
        let range = TextRange::new(0u32, u_string::unit_count(&(text)))?;
        return Ok(ShapingInput::new(text, (range).clone(), TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)), FontDecision::new((range).clone(), FontCandidate::new("test-font", "test-font",
actual_role), actual_role, "coverage-test"), Some(match &(display_text) { None => text.to_string(), Some(__option) => __option.to_string() }.to_string()), Some((match &(features) { None => vec![], Some(__option1) => (*__option1).clone() }).clone())));
    }

    pub fn text_shaper_coverage_test_support_surrogate_text(codes: &Vec<u32>) -> String {
        let mut result = String::new();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((codes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let unit = codes[usize::try_from(i).unwrap_or(0)];
            if ({ let v: u32 = unit; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 55296 && ({ let v: u32 = unit; i32::from_ne_bytes(v.to_ne_bytes()) }) <= 56319 && (i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((codes.len()) &
0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) && ({ let v: u32 = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 56320 && ({ let v: u32 = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)];
i32::from_ne_bytes(v.to_ne_bytes()) }) <= 57343 {
                let low = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)];
                result += &(if u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320)) > 0xFFFF { String::from_utf16(&[0xD800 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)),
u32::wrapping_sub(low, 56320))) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) - 0x10000) & 0x3FF) as u16]).unwrap() } else {
String::from_utf16_lossy(&[(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) as u16]) });
                i = u32::wrapping_add(i, 2);
            } else {
                result += &(if unit > 0xFFFF { String::from_utf16(&[0xD800 + (((unit) - 0x10000) >> 10) as u16, 0xDC00 + (((unit) - 0x10000) & 0x3FF) as u16]).unwrap() } else { String::from_utf16_lossy(&[(unit) as u16]) });
                i = u32::wrapping_add(i, 1);
            }
        }
        return result;
    }
}
