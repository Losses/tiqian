use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleClassifier;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::font::unicode_emoji_presentation_data::UnicodeEmojiPresentationData;
use crate::org::tiqian::font::unicode_symbol_data::UnicodeSymbolData;
use crate::runtime::u_string;


#[derive(Clone, PartialEq)]
pub struct CjkFontRoleClassifier {
}

impl CjkFontRoleClassifier {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn classify(&self, text: &str, range: TextRange, _context: Option<FontRoleContext>) -> FontRole {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut c = u_string::unit_at_from(&__units, range.start).unwrap_or(0);
        if i32::from_ne_bytes((c).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 56319 && (i32::from_ne_bytes((u32::wrapping_add(range.start, 1)).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            let lo = u_string::unit_at_from(&__units, u32::wrapping_add(range.start, 1)).unwrap_or(0);
            if i32::from_ne_bytes((lo).to_ne_bytes()) >= 56320 && (i32::from_ne_bytes((lo).to_ne_bytes())) <= 57343 {
                c = u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(c, 55296)) << (10)), u32::wrapping_sub(lo, 56320));
            }
        }
        if i32::from_ne_bytes((c).to_ne_bytes()) >= 12549 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 12591 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 12704 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 12735 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 13312 &&
(i32::from_ne_bytes((c).to_ne_bytes())) <= 19903 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 19968 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 40959 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 63744 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 64255 ||
(i32::from_ne_bytes((c).to_ne_bytes())) >= 131072 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 173791 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 173824 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 177983 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 177984 &&
(i32::from_ne_bytes((c).to_ne_bytes())) <= 178207 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 178208 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 183983 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 196608 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 201551 {
            return FontRole::CjkText;
        }
        if c == 8216 || c == 8217 || c == 8220 || c == 8221 {
            let l = if i32::from_ne_bytes((range.start).to_ne_bytes()) > (0) { u_string::unit_at_from(&__units, u32::wrapping_sub(range.start, 1)) } else { None };
            let r = if i32::from_ne_bytes((range.end).to_ne_bytes()) < (i32::from_ne_bytes((__count).to_ne_bytes())) { u_string::unit_at_from(&__units, range.end) } else { None };
            return if CjkFontRoleClassifier::cjk_font_role_classifier_is_latin(l) && CjkFontRoleClassifier::cjk_font_role_classifier_is_latin(r) { FontRole::LatinText } else { FontRole::CjkPunctuation };
        }
        if i32::from_ne_bytes((c).to_ne_bytes()) >= 32 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 126 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 192 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 591 {
            return FontRole::LatinText;
        }
        if i32::from_ne_bytes((c).to_ne_bytes()) >= 12288 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 12351 || c == 8212 || c == 8211 || c == 8252 || c == 8263 || c == 8230 || c == 8231 || c == 8943 || c == 12539 || c == 11834 || c == 183 || c == 8226 || c == 65281 || c ==
65311 || c == 65292 || c == 65294 || c == 65295 || c == 65306 || c == 65307 || c == 65288 || c == 65289 || c == 65374 {
            return FontRole::CjkPunctuation;
        }
        if UnicodeEmojiPresentationData::unicode_emoji_presentation_data_contains(c) {
            return FontRole::Emoji;
        }
        if i32::from_ne_bytes((c).to_ne_bytes()) <= 65535 && UnicodeSymbolData::unicode_symbol_data_contains(c) {
            return FontRole::Symbol;
        }
        return FontRole::Unknown;
    }

    pub(crate) fn cjk_font_role_classifier_is_latin(c: Option<u32>) -> bool {
        return match &(c) { Some(__option) => i32::from_ne_bytes((*__option).to_ne_bytes()) >= 32 && (i32::from_ne_bytes((*__option).to_ne_bytes())) <= 126 || (i32::from_ne_bytes((*__option).to_ne_bytes())) >= 192 && (i32::from_ne_bytes((*__option).to_ne_bytes())) <= 591 ||
*__option == 8216 || *__option == 8217 || *__option == 8220 || *__option == 8221, None => false };
    }
}

impl FontRoleClassifier for CjkFontRoleClassifier {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.font.CjkFontRoleClassifier.CjkFontRoleClassifier"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn FontRoleClassifier> {
        Box::new(self.clone())
    }

    fn classify(&self, text: &str, range: TextRange, _context: Option<FontRoleContext>) -> FontRole {
    let __units1 = u_string::units(&text);
    let __count1 = u_string::unit_count(&text);
        let mut c = u_string::unit_at_from(&__units1, range.start).unwrap_or(0);
        if i32::from_ne_bytes((c).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 56319 && (i32::from_ne_bytes((u32::wrapping_add(range.start, 1)).to_ne_bytes())) < (i32::from_ne_bytes((__count1).to_ne_bytes())) {
            let lo = u_string::unit_at_from(&__units1, u32::wrapping_add(range.start, 1)).unwrap_or(0);
            if i32::from_ne_bytes((lo).to_ne_bytes()) >= 56320 && (i32::from_ne_bytes((lo).to_ne_bytes())) <= 57343 {
                c = u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(c, 55296)) << (10)), u32::wrapping_sub(lo, 56320));
            }
        }
        if i32::from_ne_bytes((c).to_ne_bytes()) >= 12549 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 12591 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 12704 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 12735 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 13312 &&
(i32::from_ne_bytes((c).to_ne_bytes())) <= 19903 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 19968 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 40959 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 63744 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 64255 ||
(i32::from_ne_bytes((c).to_ne_bytes())) >= 131072 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 173791 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 173824 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 177983 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 177984 &&
(i32::from_ne_bytes((c).to_ne_bytes())) <= 178207 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 178208 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 183983 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 196608 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 201551 {
            return FontRole::CjkText;
        }
        if c == 8216 || c == 8217 || c == 8220 || c == 8221 {
            let l = if i32::from_ne_bytes((range.start).to_ne_bytes()) > (0) { u_string::unit_at_from(&__units1, u32::wrapping_sub(range.start, 1)) } else { None };
            let r = if i32::from_ne_bytes((range.end).to_ne_bytes()) < (i32::from_ne_bytes((__count1).to_ne_bytes())) { u_string::unit_at_from(&__units1, range.end) } else { None };
            return if CjkFontRoleClassifier::cjk_font_role_classifier_is_latin(l) && CjkFontRoleClassifier::cjk_font_role_classifier_is_latin(r) { FontRole::LatinText } else { FontRole::CjkPunctuation };
        }
        if i32::from_ne_bytes((c).to_ne_bytes()) >= 32 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 126 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 192 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 591 {
            return FontRole::LatinText;
        }
        if i32::from_ne_bytes((c).to_ne_bytes()) >= 12288 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 12351 || c == 8212 || c == 8211 || c == 8252 || c == 8263 || c == 8230 || c == 8231 || c == 8943 || c == 12539 || c == 11834 || c == 183 || c == 8226 || c == 65281 || c ==
65311 || c == 65292 || c == 65294 || c == 65295 || c == 65306 || c == 65307 || c == 65288 || c == 65289 || c == 65374 {
            return FontRole::CjkPunctuation;
        }
        if UnicodeEmojiPresentationData::unicode_emoji_presentation_data_contains(c) {
            return FontRole::Emoji;
        }
        if i32::from_ne_bytes((c).to_ne_bytes()) <= 65535 && UnicodeSymbolData::unicode_symbol_data_contains(c) {
            return FontRole::Symbol;
        }
        return FontRole::Unknown;
    }
}
