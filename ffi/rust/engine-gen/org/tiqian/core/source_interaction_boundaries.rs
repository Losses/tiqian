use crate::org::tiqian::core::source_boundary_bias::SourceBoundaryBias;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::unicode_combining_mark_data::UnicodeCombiningMarkData;
use crate::org::tiqian::core::unicode_emoji_modifier_base_data::UnicodeEmojiModifierBaseData;
use crate::org::tiqian::core::unicode_extended_pictographic_data::UnicodeExtendedPictographicData;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;


#[derive(Clone, Copy)]
pub struct SourceInteractionBoundaries;

impl SourceInteractionBoundaries {
    const SOURCE_INTERACTION_BOUNDARIES_HIGH_SURROGATE_START: u32 = 55296;
    const SOURCE_INTERACTION_BOUNDARIES_HIGH_SURROGATE_END: u32 = 56319;
    const SOURCE_INTERACTION_BOUNDARIES_LOW_SURROGATE_START: u32 = 56320;
    const SOURCE_INTERACTION_BOUNDARIES_LOW_SURROGATE_END: u32 = 57343;
    const SOURCE_INTERACTION_BOUNDARIES_CR: u32 = 13;
    const SOURCE_INTERACTION_BOUNDARIES_LF: u32 = 10;
    const SOURCE_INTERACTION_BOUNDARIES_ZWNJ: u32 = 8204;
    const SOURCE_INTERACTION_BOUNDARIES_ZWJ: u32 = 8205;

    pub fn source_interaction_boundaries_coerce_to_interaction_boundary(text: &UStr, offset: u32, range: TextRange, bias: SourceBoundaryBias) -> u32 {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let start = SourceInteractionBoundaries::source_interaction_boundaries_clamp(range.start, 0, __count);
        let end = SourceInteractionBoundaries::source_interaction_boundaries_clamp(range.end, start, __count);
        let target = SourceInteractionBoundaries::source_interaction_boundaries_clamp(offset, start, end);
        if target == start || target == end {
            return target;
        }
        let boundaries = SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries_by_offsets(text, start, end);
        let mut previous = start;
        let mut next = end;
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let boundary = boundaries[usize::try_from(index).unwrap_or(0)];
            if boundary == target {
                return target;
            }
            if ({ let v: u32 = boundary; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((target) as i32).to_ne_bytes())) {
                previous = boundary;
            } else {
                next = boundary;
                break;
            }
            index = u32::wrapping_add(index, 1);
        }
        if bias == SourceBoundaryBias::Backward {
            return previous;
        }
        if bias == SourceBoundaryBias::Forward {
            return next;
        }
        return if i32::from_ne_bytes(((u32::wrapping_sub(target, previous)) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((u32::wrapping_sub(next, target)) as i32).to_ne_bytes())) { previous } else { next };
    }

    pub fn source_interaction_boundaries_interaction_boundaries(text: &UStr, range: TextRange) -> Vec<u32> {
    let __units1 = u_string::units(&text);
    let __count1 = u_string::unit_count(&text);
        let start = SourceInteractionBoundaries::source_interaction_boundaries_clamp(range.start, 0, __count1);
        let end = SourceInteractionBoundaries::source_interaction_boundaries_clamp(range.end, start, __count1);
        return SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries_by_offsets(text, start, end);
    }

    pub fn source_interaction_boundaries_source_grapheme_boundaries(text: &UStr, range: TextRange) -> Vec<u32> {
        return SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(text, (range).clone());
    }

    pub fn source_interaction_boundaries_code_point_at_compat(text: &UStr, index: u32, end: u32) -> u32 {
    let __units2 = u_string::units(&text);
    let __count2 = u_string::unit_count(&text);
        let high = u_string::unit_at_from(&__units2, index).unwrap_or(0);
        if i32::from_ne_bytes(((high) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_HIGH_SURROGATE_START) as i32).to_ne_bytes())) || (i32::from_ne_bytes(((high) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_HIGH_SURROGATE_END) as i32).to_ne_bytes())) || (i32::from_ne_bytes(((u32::wrapping_add(index, 1)) as i32).to_ne_bytes())) >= i32::from_ne_bytes(((end) as i32).to_ne_bytes()) {
            return high;
        }
        let low = u_string::unit_at_from(&__units2, u32::wrapping_add(index, 1)).unwrap_or(0);
        if i32::from_ne_bytes(((low) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_LOW_SURROGATE_START) as i32).to_ne_bytes())) || (i32::from_ne_bytes(((low) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_LOW_SURROGATE_END) as i32).to_ne_bytes())) {
            return high;
        }
        return u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(high, SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_HIGH_SURROGATE_START)) << (10)), u32::wrapping_sub(low, SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_LOW_SURROGATE_START));
    }

    pub(crate) fn source_interaction_boundaries_interaction_boundaries_by_offsets(text: &UStr, start: u32, end: u32) -> Vec<u32> {
        let mut output = vec![start];
        let mut index = start;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) {
            let first = SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, index, end);
            let mut next = u32::wrapping_add(index, SourceInteractionBoundaries::source_interaction_boundaries_char_count(first));
            let mut preceding_emoji_modifier_base = UnicodeEmojiModifierBaseData::unicode_emoji_modifier_base_data_contains(first);
            let mut preceding_extended_pictographic = UnicodeExtendedPictographicData::unicode_extended_pictographic_data_contains(first);
            if first == SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_CR && (i32::from_ne_bytes(((next) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end) == SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_LF {
                next = u32::wrapping_add(next, 1);
            } else {
                if SourceInteractionBoundaries::source_interaction_boundaries_is_regional_indicator(first) && (i32::from_ne_bytes(((next) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) {
                    let following = SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end);
                    if SourceInteractionBoundaries::source_interaction_boundaries_is_regional_indicator(following) {
                        next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(following));
                    }
                } else {
                    if SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_l(first) {
                        while (i32::from_ne_bytes(((next) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_l(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                            next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                        }
                        if i32::from_ne_bytes(((next) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_v(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                            while (i32::from_ne_bytes(((next) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_v(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                                next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                            }
                            while (i32::from_ne_bytes(((next) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_t(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                                next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                            }
                        }
                    } else {
                        if SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_lv_or_lvt(first) {
                            if SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_lv(first) {
                                while (i32::from_ne_bytes(((next) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_v(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                                    next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                                }
                            }
                            while (i32::from_ne_bytes(((next) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_t(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                                next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                            }
                        }
                    }
                }
            }
            next = SourceInteractionBoundaries::source_interaction_boundaries_consume_extenders(text, next, end);
            if preceding_emoji_modifier_base && (i32::from_ne_bytes(((next) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && SourceInteractionBoundaries::source_interaction_boundaries_is_emoji_modifier(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                preceding_emoji_modifier_base = false;
                next = SourceInteractionBoundaries::source_interaction_boundaries_consume_extenders(text, next, end);
            }
            while (i32::from_ne_bytes(((next) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end) == SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_ZWJ {
                next = u32::wrapping_add(next, 1);
                if i32::from_ne_bytes(((next) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((end) as i32).to_ne_bytes()) {
                    break;
                }
                let joined = SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end);
                if !preceding_extended_pictographic || !UnicodeExtendedPictographicData::unicode_extended_pictographic_data_contains(joined) {
                    break;
                }
                next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(joined));
                preceding_emoji_modifier_base = UnicodeEmojiModifierBaseData::unicode_emoji_modifier_base_data_contains(joined);
                preceding_extended_pictographic = true;
                next = SourceInteractionBoundaries::source_interaction_boundaries_consume_extenders(text, next, end);
                if preceding_emoji_modifier_base && (i32::from_ne_bytes(((next) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && SourceInteractionBoundaries::source_interaction_boundaries_is_emoji_modifier(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                    next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                    preceding_emoji_modifier_base = false;
                    next = SourceInteractionBoundaries::source_interaction_boundaries_consume_extenders(text, next, end);
                }
            }
            index = next;
            output.push(index);
        }
        return output;
    }

    pub(crate) fn source_interaction_boundaries_consume_extenders(text: &UStr, from: u32, end: u32) -> u32 {
        let mut index = from;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) {
            let code_point = SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, index, end);
            if !SourceInteractionBoundaries::source_interaction_boundaries_is_interaction_extender(code_point) {
                break;
            }
            index = u32::wrapping_add(index, SourceInteractionBoundaries::source_interaction_boundaries_char_count(code_point));
        }
        return index;
    }

    pub(crate) fn source_interaction_boundaries_is_interaction_extender(code_point: u32) -> bool {
        return code_point == SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_ZWNJ || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 65024 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 65039 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 917760 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 917999 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 917536 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 917631 || SourceInteractionBoundaries::source_interaction_boundaries_is_combining_mark(code_point);
    }

    pub(crate) fn source_interaction_boundaries_is_combining_mark(code_point: u32) -> bool {
        return (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 65535 && UnicodeCombiningMarkData::unicode_combining_mark_data_contains(code_point);
    }

    pub(crate) fn source_interaction_boundaries_is_emoji_modifier(code_point: u32) -> bool {
        return (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 127995 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 127999;
    }

    pub(crate) fn source_interaction_boundaries_is_regional_indicator(code_point: u32) -> bool {
        return (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 127462 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 127487;
    }

    pub(crate) fn source_interaction_boundaries_is_hangul_l(code_point: u32) -> bool {
        return (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 4352 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 4447 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 43360 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 43388;
    }

    pub(crate) fn source_interaction_boundaries_is_hangul_v(code_point: u32) -> bool {
        return (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 4448 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 4519 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 55216 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 55238;
    }

    pub(crate) fn source_interaction_boundaries_is_hangul_t(code_point: u32) -> bool {
        return (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 4520 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 4607 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 55243 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 55291;
    }

    pub(crate) fn source_interaction_boundaries_is_hangul_lv_or_lvt(code_point: u32) -> bool {
        return (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 44032 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 55203;
    }

    pub(crate) fn source_interaction_boundaries_is_hangul_lv(code_point: u32) -> bool {
        return SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_lv_or_lvt(code_point) && u32::from_ne_bytes(((i32::from_ne_bytes(((u32::wrapping_sub(code_point, 44032)) as i32).to_ne_bytes()) % 28i32) as u32).to_ne_bytes()) == 0;
    }

    pub(crate) fn source_interaction_boundaries_char_count(code_point: u32) -> u32 {
        return if i32::from_ne_bytes(((code_point) as i32).to_ne_bytes()) > (65535) { 2 } else { 1 };
    }

    pub(crate) fn source_interaction_boundaries_clamp(value: u32, low: u32, high: u32) -> u32 {
        if i32::from_ne_bytes(((value) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((low) as i32).to_ne_bytes())) {
            return low;
        }
        if i32::from_ne_bytes(((value) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((high) as i32).to_ne_bytes())) {
            return high;
        }
        return value;
    }
}
