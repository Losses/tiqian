use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleClassifier;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::font::unicode_emoji_presentation_data::UnicodeEmojiPresentationData;
use crate::org::tiqian::font::unicode_symbol_data::UnicodeSymbolData;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;


#[derive(Clone, PartialEq)]
pub struct CjkFontRoleClassifier {
}

impl CjkFontRoleClassifier {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn classify(&self, text: &UStr, range: TextRange, _context: Option<FontRoleContext>) -> FontRole {
    let __units = text.as_slice();
    let __count = u_string::unit_count(&text);
        let mut c = u_string::unit_at_from(__units, range.start).unwrap_or(0);
        if c as i32 >= 55296 && ((c) as i32) <= 56319 && ((u32::wrapping_add(range.start, 1)) as i32) < ((__count) as i32) {
            let lo = u_string::unit_at_from(__units, u32::wrapping_add(range.start, 1)).unwrap_or(0);
            if lo as i32 >= 56320 && ((lo) as i32) <= 57343 {
                c = u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(c, 55296)) << (10)), u32::wrapping_sub(lo, 56320));
            }
        }
        if c as i32 >= 12549 && ((c) as i32) <= 12591 || ((c) as i32) >= 12704 && ((c) as i32) <= 12735 || ((c) as i32) >= 13312 && ((c) as i32) <= 19903 || ((c) as i32) >= 19968 && ((c) as i32) <= 40959 || ((c) as i32) >= 63744 && ((c) as i32) <= 64255 || ((c) as i32) >= 131072 && ((c) as i32) <= 173791 || ((c) as i32) >= 173824 && ((c) as i32) <= 177983 || ((c) as i32) >= 177984 && ((c) as i32) <= 178207 || ((c) as i32) >= 178208 && ((c) as i32) <= 183983 || ((c) as i32) >= 196608 && ((c) as i32) <= 201551 {
            return FontRole::CjkText;
        }
        if c == 8216 || c == 8217 || c == 8220 || c == 8221 {
            let l = if range.start as i32 > (0) { u_string::unit_at_from(__units, u32::wrapping_sub(range.start, 1)) } else { None };
            let r = if range.end as i32 < ((__count) as i32) { u_string::unit_at_from(__units, range.end) } else { None };
            return if CjkFontRoleClassifier::cjk_font_role_classifier_is_latin(l) && CjkFontRoleClassifier::cjk_font_role_classifier_is_latin(r) { FontRole::LatinText } else { FontRole::CjkPunctuation };
        }
        if c as i32 >= 32 && ((c) as i32) <= 126 || ((c) as i32) >= 192 && ((c) as i32) <= 591 {
            return FontRole::LatinText;
        }
        if c as i32 >= 12288 && ((c) as i32) <= 12351 || c == 8212 || c == 8211 || c == 8252 || c == 8263 || c == 8230 || c == 8231 || c == 8943 || c == 12539 || c == 11834 || c == 183 || c == 8226 || c == 65281 || c == 65311 || c == 65292 || c == 65294 || c == 65295 || c == 65306 || c == 65307 || c == 65288 || c == 65289 || c == 65374 {
            return FontRole::CjkPunctuation;
        }
        if UnicodeEmojiPresentationData::unicode_emoji_presentation_data_contains(c) {
            return FontRole::Emoji;
        }
        if c as i32 <= 65535 && UnicodeSymbolData::unicode_symbol_data_contains(c) {
            return FontRole::Symbol;
        }
        return FontRole::Unknown;
    }

    pub(crate) fn cjk_font_role_classifier_is_latin(c: Option<u32>) -> bool {
        return match &(c) { Some(__option) => (*__option) as i32 >= 32 && ((*__option) as i32) <= 126 || ((*__option) as i32) >= 192 && ((*__option) as i32) <= 591 || *__option == 8216 || *__option == 8217 || *__option == 8220 || *__option == 8221, None => false };
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

    fn classify(&self, text: &UStr, range: TextRange, _context: Option<FontRoleContext>) -> FontRole {
    let __units1 = text.as_slice();
    let __count1 = u_string::unit_count(&text);
        let mut c = u_string::unit_at_from(__units1, range.start).unwrap_or(0);
        if c as i32 >= 55296 && ((c) as i32) <= 56319 && ((u32::wrapping_add(range.start, 1)) as i32) < ((__count1) as i32) {
            let lo = u_string::unit_at_from(__units1, u32::wrapping_add(range.start, 1)).unwrap_or(0);
            if lo as i32 >= 56320 && ((lo) as i32) <= 57343 {
                c = u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(c, 55296)) << (10)), u32::wrapping_sub(lo, 56320));
            }
        }
        if c as i32 >= 12549 && ((c) as i32) <= 12591 || ((c) as i32) >= 12704 && ((c) as i32) <= 12735 || ((c) as i32) >= 13312 && ((c) as i32) <= 19903 || ((c) as i32) >= 19968 && ((c) as i32) <= 40959 || ((c) as i32) >= 63744 && ((c) as i32) <= 64255 || ((c) as i32) >= 131072 && ((c) as i32) <= 173791 || ((c) as i32) >= 173824 && ((c) as i32) <= 177983 || ((c) as i32) >= 177984 && ((c) as i32) <= 178207 || ((c) as i32) >= 178208 && ((c) as i32) <= 183983 || ((c) as i32) >= 196608 && ((c) as i32) <= 201551 {
            return FontRole::CjkText;
        }
        if c == 8216 || c == 8217 || c == 8220 || c == 8221 {
            let l = if range.start as i32 > (0) { u_string::unit_at_from(__units1, u32::wrapping_sub(range.start, 1)) } else { None };
            let r = if range.end as i32 < ((__count1) as i32) { u_string::unit_at_from(__units1, range.end) } else { None };
            return if CjkFontRoleClassifier::cjk_font_role_classifier_is_latin(l) && CjkFontRoleClassifier::cjk_font_role_classifier_is_latin(r) { FontRole::LatinText } else { FontRole::CjkPunctuation };
        }
        if c as i32 >= 32 && ((c) as i32) <= 126 || ((c) as i32) >= 192 && ((c) as i32) <= 591 {
            return FontRole::LatinText;
        }
        if c as i32 >= 12288 && ((c) as i32) <= 12351 || c == 8212 || c == 8211 || c == 8252 || c == 8263 || c == 8230 || c == 8231 || c == 8943 || c == 12539 || c == 11834 || c == 183 || c == 8226 || c == 65281 || c == 65311 || c == 65292 || c == 65294 || c == 65295 || c == 65306 || c == 65307 || c == 65288 || c == 65289 || c == 65374 {
            return FontRole::CjkPunctuation;
        }
        if UnicodeEmojiPresentationData::unicode_emoji_presentation_data_contains(c) {
            return FontRole::Emoji;
        }
        if c as i32 <= 65535 && UnicodeSymbolData::unicode_symbol_data_contains(c) {
            return FontRole::Symbol;
        }
        return FontRole::Unknown;
    }
}
