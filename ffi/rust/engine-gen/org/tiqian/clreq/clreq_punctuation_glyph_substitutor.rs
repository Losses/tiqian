use crate::org::tiqian::clreq::cjk_punctuation_glyph_policy::CjkPunctuationGlyphPolicy;
use crate::org::tiqian::clreq::cjk_punctuation_glyph_substitution::CjkPunctuationGlyphSubstitution;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[derive(Clone, PartialEq)]
pub struct ClreqPunctuationGlyphSubstitutor {
    pub(crate) policy: CjkPunctuationGlyphPolicy,
}

impl ClreqPunctuationGlyphSubstitutor {
    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDLINE_ELLIPSIS: &UStr = unsafe { &*(&[0x22EFu16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_TWO_EM_DASH_SOURCE: &UStr = unsafe { &*(&[0x2014u16, 0x2014u16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_TWO_EM_DASH: &UStr = unsafe { &*(&[0x2E3Au16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_KATAKANA_MIDDLE_DOT: &UStr = unsafe { &*(&[0x30FBu16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HYPHENATION_POINT: &UStr = unsafe { &*(&[0x2027u16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_BULLET: &UStr = unsafe { &*(&[0x2022u16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDDLE_DOT: &UStr = unsafe { &*(&[0x00B7u16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HEX_UPPER: &UStr = unsafe { &*(&[0x0030u16, 0x0031u16, 0x0032u16, 0x0033u16, 0x0034u16, 0x0035u16, 0x0036u16, 0x0037u16, 0x0038u16, 0x0039u16, 0x0041u16, 0x0042u16, 0x0043u16, 0x0044u16, 0x0045u16, 0x0046u16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    pub fn new(policy: Option<CjkPunctuationGlyphPolicy>) -> Self {
        let policy = policy.unwrap_or_else(|| CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints);
        Self {
            policy: policy,
        }
    }

    pub fn substitute(&self, source_text: &UStr) -> Result<CjkPunctuationGlyphSubstitution, UStringFault> {
        let display_text = if self.policy == CjkPunctuationGlyphPolicy::PreserveInput { (source_text.to_ustring()).clone() } else { ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_to_clreq_recommended_display_text(source_text)?.to_ustring() };
        let reason = if display_text == source_text { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("CjkPunctuationGlyphPolicy:")); __s += UString::from(self.policy.name()).as_ustr(); __s += &(UString::from(":preserve")); __s }).as_str()) } else { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("CjkPunctuationGlyphPolicy:")); __s += UString::from(self.policy.name()).as_ustr(); __s += &(UString::from(":")); __s += ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_to_code_point_labels(source_text)?.as_ustr(); __s += &(UString::from("->")); __s += ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_to_code_point_labels(display_text.as_ustr())?.as_ustr(); __s }).as_str()) };
        return Ok(CjkPunctuationGlyphSubstitution::new(source_text, display_text.as_ustr(), reason.as_ustr()));
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ClreqPunctuationGlyphSubstitutor(policy=")); __s += UString::from(self.policy.name()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }

    pub(crate) fn clreq_punctuation_glyph_substitutor_to_clreq_recommended_display_text(text: &UStr) -> Result<UString, UStringFault> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut output = Vec::<u16>::new();
        let mut index = 0u32;
        let mut all_ellipsis = true;
        let __units1 = u_string::units(&text);
        let __count1 = u_string::unit_count(&text);
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count) as i32).to_ne_bytes())) {
            if !(u_string::unit_at_from(&__units1, index).as_ref().map_or(false, |v| v == &(8230))) {
                all_ellipsis = false;
                break;
            }
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDLINE_ELLIPSIS.to_ustring().is_empty() {
                    if !ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDLINE_ELLIPSIS.to_ustring().encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            output.extend(ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDLINE_ELLIPSIS.to_ustring().encode_utf16());
            index = u32::wrapping_add(index, 1);
        }
        if all_ellipsis {
            return Ok(UString::from_utf16(output.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(output[output.len() - 1]) })?);
        }
        if text == ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_TWO_EM_DASH_SOURCE.to_ustring() {
            return Ok((ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_TWO_EM_DASH.to_ustring()).clone());
        }
        if text == ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_KATAKANA_MIDDLE_DOT.to_ustring() || text == ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HYPHENATION_POINT.to_ustring() || text == ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_BULLET.to_ustring() {
            return Ok((ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDDLE_DOT.to_ustring()).clone());
        }
        return Ok(text.to_ustring());
    }

    pub(crate) fn clreq_punctuation_glyph_substitutor_to_code_point_labels(text: &UStr) -> Result<UString, UStringFault> {
    let __units2 = u_string::units(&text);
    let __count2 = u_string::unit_count(&text);
        let mut output = Vec::<u16>::new();
        let mut index = 0u32;
        let __units3 = u_string::units(&text);
        let __count3 = u_string::unit_count(&text);
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count2) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((index) as i32).to_ne_bytes()) > (0) {
                if let Some(&unit) = output.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from("+").is_empty() {
                        if !UString::from("+").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                output.extend(UString::from("+").encode_utf16());
            }
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from("U+").is_empty() {
                    if !UString::from("U+").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            output.extend(UString::from("U+").encode_utf16());
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_hex_label(*(u_string::unit_at_from(&__units3, index)).as_ref().unwrap()).is_empty() {
                    if !ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_hex_label(*(u_string::unit_at_from(&__units3, index)).as_ref().unwrap()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            output.extend(ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_hex_label(*(u_string::unit_at_from(&__units3, index)).as_ref().unwrap()).encode_utf16());
            index = u32::wrapping_add(index, 1);
        }
        return Ok(UString::from_utf16(output.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(output[output.len() - 1]) })?);
    }

    pub(crate) fn clreq_punctuation_glyph_substitutor_hex_label(unit: u32) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += u_string::substring(&ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HEX_UPPER.to_ustring(), i32::from_ne_bytes(((unit >> 12 & 15) as i32).to_ne_bytes()), i32::from_ne_bytes(((u32::wrapping_add(unit >> 12 & 15, 1)) as i32).to_ne_bytes())).as_ustr(); __s += u_string::substring(&ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HEX_UPPER.to_ustring(), i32::from_ne_bytes(((unit >> 8 & 15) as i32).to_ne_bytes()), i32::from_ne_bytes(((u32::wrapping_add(unit >> 8 & 15, 1)) as i32).to_ne_bytes())).as_ustr(); __s += u_string::substring(&ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HEX_UPPER.to_ustring(), i32::from_ne_bytes(((unit >> 4 & 15) as i32).to_ne_bytes()), i32::from_ne_bytes(((u32::wrapping_add(unit >> 4 & 15, 1)) as i32).to_ne_bytes())).as_ustr(); __s += u_string::substring(&ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HEX_UPPER.to_ustring(), i32::from_ne_bytes(((unit & 15) as i32).to_ne_bytes()), i32::from_ne_bytes(((u32::wrapping_add(unit & 15, 1)) as i32).to_ne_bytes())).as_ustr(); __s }).as_str());
    }
}
