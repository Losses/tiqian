use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_word_character_data::UnicodeWordCharacterData;


#[derive(Clone, Copy)]
pub struct UnicodeWordCharacter;

impl UnicodeWordCharacter {
    pub const UNICODE_WORD_CHARACTER_DATA_REVISION: &str = "17.0.0";
    pub const UNICODE_WORD_CHARACTER_DATA_SOURCE: &str = "https://www.unicode.org/Public/17.0.0/ucd/extracted/DerivedGeneralCategory.txt";
    pub const UNICODE_WORD_CHARACTER_DATA_SHA256: &str = "d62e5bab70ca74f099343f71224fa051cb1fdd61a1ab45c0488c44cfc0b6102e";

    pub fn unicode_word_character_contains(code_point: u32) -> Result<bool, TextRangeError> {
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
        return Ok(UnicodeWordCharacterData::unicode_word_character_data_contains(u32::from_ne_bytes((code_point).to_ne_bytes())));
    }
}
