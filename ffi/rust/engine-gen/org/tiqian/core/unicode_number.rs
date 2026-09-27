use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_number_data::UnicodeNumberData;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct UnicodeNumber;

impl UnicodeNumber {
    pub fn unicode_number_contains(code_point: u32) -> Result<bool, TextRangeError> {
        if code_point > 2147483647 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) > (1114111) {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Not a Unicode scalar value: ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(code_point)).as_str())); __s }).as_str()) });
        }
        if i32::from_ne_bytes(((code_point) as i32).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 57343 {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Surrogate is not a Unicode scalar value: ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(code_point)).as_str())); __s }).as_str()) });
        }
        return Ok(UnicodeNumberData::unicode_number_data_contains(u32::from_ne_bytes(((code_point) as u32).to_ne_bytes())));
    }
}
