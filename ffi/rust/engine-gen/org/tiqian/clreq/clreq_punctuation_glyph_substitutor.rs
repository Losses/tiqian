use crate::org::tiqian::clreq::cjk_punctuation_glyph_policy::CjkPunctuationGlyphPolicy;
use crate::org::tiqian::clreq::cjk_punctuation_glyph_substitution::CjkPunctuationGlyphSubstitution;
use crate::runtime::u_string;
use crate::std::u_string_exception::UStringFault;


#[derive(Clone, PartialEq)]
pub struct ClreqPunctuationGlyphSubstitutor {
    pub(crate) policy: CjkPunctuationGlyphPolicy,
}

impl ClreqPunctuationGlyphSubstitutor {
    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDLINE_ELLIPSIS: &str = "⋯";

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_TWO_EM_DASH_SOURCE: &str = "——";

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_TWO_EM_DASH: &str = "⸺";

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_KATAKANA_MIDDLE_DOT: &str = "・";

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HYPHENATION_POINT: &str = "‧";

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_BULLET: &str = "•";

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDDLE_DOT: &str = "·";

    const CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HEX_UPPER: &str = "0123456789ABCDEF";

    pub fn new(policy: Option<CjkPunctuationGlyphPolicy>) -> Self {
        let policy = policy.unwrap_or_else(|| CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints);
        Self {
            policy: policy,
        }
    }

    pub fn substitute(&self, source_text: &str) -> Result<CjkPunctuationGlyphSubstitution, UStringFault> {
        let display_text = if self.policy == CjkPunctuationGlyphPolicy::PreserveInput { (source_text.to_string()).clone() } else { ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_to_clreq_recommended_display_text(source_text)?.to_string() };
        let reason = if display_text == source_text { format!("{}{}{}",
            "CjkPunctuationGlyphPolicy:",
            self.policy.name(),
            ":preserve"
        ).to_string() } else { format!("{}{}{}{}{}{}",
            "CjkPunctuationGlyphPolicy:",
            self.policy.name(),
            ":",
            ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_to_code_point_labels(source_text)?,
            "->",
            ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_to_code_point_labels(display_text.as_str())?
        ).to_string() };
        return Ok(CjkPunctuationGlyphSubstitution::new(source_text, display_text.as_str(), reason.as_str()));
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}",
            "ClreqPunctuationGlyphSubstitutor(policy=",
            self.policy.name(),
            ")"
        );
    }

    pub(crate) fn clreq_punctuation_glyph_substitutor_to_clreq_recommended_display_text(text: &str) -> Result<String, UStringFault> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut output = Vec::<u16>::new();
        let mut index = 0u32;
        let mut all_ellipsis = true;
        let __units1 = u_string::units(&text);
        let __count1 = u_string::unit_count(&text);
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            if !(u_string::unit_at_from(&__units1, index).as_ref().map_or(false, |v| v == &(8230))) {
                all_ellipsis = false;
                break;
            }
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDLINE_ELLIPSIS.to_string().is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            output.extend(ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDLINE_ELLIPSIS.to_string().encode_utf16());
            index = u32::wrapping_add(index, 1);
        }
        if all_ellipsis {
            return Ok(String::from_utf16(output.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(output[output.len() - 1]) })?);
        }
        if text == ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_TWO_EM_DASH_SOURCE.to_string() {
            return Ok((ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_TWO_EM_DASH.to_string()).clone());
        }
        if text == ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_KATAKANA_MIDDLE_DOT.to_string() || text == ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HYPHENATION_POINT.to_string() || text ==
ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_BULLET.to_string() {
            return Ok((ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_MIDDLE_DOT.to_string()).clone());
        }
        return Ok(text.to_string());
    }

    pub(crate) fn clreq_punctuation_glyph_substitutor_to_code_point_labels(text: &str) -> Result<String, UStringFault> {
    let __units2 = u_string::units(&text);
    let __count2 = u_string::unit_count(&text);
        let mut output = Vec::<u16>::new();
        let mut index = 0u32;
        let __units3 = u_string::units(&text);
        let __count3 = u_string::unit_count(&text);
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count2).to_ne_bytes())) {
            if i32::from_ne_bytes((index).to_ne_bytes()) > (0) {
                if let Some(&unit) = output.last() {
                    if unit >= 55296 && unit <= 56319 && !"+".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                output.extend("+".encode_utf16());
            }
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !"U+".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            output.extend("U+".encode_utf16());
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_hex_label(*(u_string::unit_at_from(&__units3, index)).as_ref().unwrap()).is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            output.extend(ClreqPunctuationGlyphSubstitutor::clreq_punctuation_glyph_substitutor_hex_label(*(u_string::unit_at_from(&__units3, index)).as_ref().unwrap()).encode_utf16());
            index = u32::wrapping_add(index, 1);
        }
        return Ok(String::from_utf16(output.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(output[output.len() - 1]) })?);
    }

    pub(crate) fn clreq_punctuation_glyph_substitutor_hex_label(unit: u32) -> String {
        return format!("{}{}{}{}",
            u_string::substring(&ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HEX_UPPER.to_string(), i32::from_ne_bytes((unit >> 12 & 15).to_ne_bytes()), i32::from_ne_bytes((u32::wrapping_add(unit >> 12 & 15, 1)).to_ne_bytes())),
            u_string::substring(&ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HEX_UPPER.to_string(), i32::from_ne_bytes((unit >> 8 & 15).to_ne_bytes()), i32::from_ne_bytes((u32::wrapping_add(unit >> 8 & 15, 1)).to_ne_bytes())),
            u_string::substring(&ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HEX_UPPER.to_string(), i32::from_ne_bytes((unit >> 4 & 15).to_ne_bytes()), i32::from_ne_bytes((u32::wrapping_add(unit >> 4 & 15, 1)).to_ne_bytes())),
            u_string::substring(&ClreqPunctuationGlyphSubstitutor::CLREQ_PUNCTUATION_GLYPH_SUBSTITUTOR_HEX_UPPER.to_string(), i32::from_ne_bytes((unit & 15).to_ne_bytes()), i32::from_ne_bytes((u32::wrapping_add(unit & 15, 1)).to_ne_bytes()))
        );
    }
}
