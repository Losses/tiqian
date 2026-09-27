use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_number_data::UnicodeNumberData;


#[derive(Clone, Copy)]
pub struct UnicodeNumber;

impl UnicodeNumber {
    pub fn unicode_number_contains(code_point: u32) -> Result<bool, TextRangeError> {
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
        return Ok(UnicodeNumberData::unicode_number_data_contains(u32::from_ne_bytes((code_point).to_ne_bytes())));
    }
}
