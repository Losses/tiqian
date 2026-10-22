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
    let __units = text.as_slice();
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
        while ((index) as i32) < ((u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32) {
            let boundary = boundaries[usize::try_from(index).unwrap_or(0)];
            if boundary == target {
                return target;
            }
            if ({ let v: u32 = boundary; v as i32 }) < ((target) as i32) {
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
        return if u32::wrapping_sub(target, previous) as i32 < ((u32::wrapping_sub(next, target)) as i32) { previous } else { next };
    }

    pub fn source_interaction_boundaries_interaction_boundaries(text: &UStr, range: TextRange) -> Vec<u32> {
    let __units1 = text.as_slice();
    let __count1 = u_string::unit_count(&text);
        let start = SourceInteractionBoundaries::source_interaction_boundaries_clamp(range.start, 0, __count1);
        let end = SourceInteractionBoundaries::source_interaction_boundaries_clamp(range.end, start, __count1);
        return SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries_by_offsets(text, start, end);
    }

    pub fn source_interaction_boundaries_source_grapheme_boundaries(text: &UStr, range: TextRange) -> Vec<u32> {
        return SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(text, (range).clone());
    }

    pub fn source_interaction_boundaries_code_point_at_compat(text: &UStr, index: u32, end: u32) -> u32 {
    let __units2 = text.as_slice();
    let __count2 = u_string::unit_count(&text);
        let high = u_string::unit_at_from(__units2, index).unwrap_or(0);
        if high as i32 < ((SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_HIGH_SURROGATE_START) as i32) || ((high) as i32) > ((SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_HIGH_SURROGATE_END) as i32) || ((u32::wrapping_add(index, 1)) as i32) >= end as i32 {
            return high;
        }
        let low = u_string::unit_at_from(__units2, u32::wrapping_add(index, 1)).unwrap_or(0);
        if low as i32 < ((SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_LOW_SURROGATE_START) as i32) || ((low) as i32) > ((SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_LOW_SURROGATE_END) as i32) {
            return high;
        }
        return u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(high, SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_HIGH_SURROGATE_START)) << (10)), u32::wrapping_sub(low, SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_LOW_SURROGATE_START));
    }

    pub(crate) fn source_interaction_boundaries_interaction_boundaries_by_offsets(text: &UStr, start: u32, end: u32) -> Vec<u32> {
        let mut output = vec![start];
        let mut index = start;
        while ((index) as i32) < ((end) as i32) {
            let first = SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, index, end);
            let mut next = u32::wrapping_add(index, SourceInteractionBoundaries::source_interaction_boundaries_char_count(first));
            let mut preceding_emoji_modifier_base = UnicodeEmojiModifierBaseData::unicode_emoji_modifier_base_data_contains(first);
            let mut preceding_extended_pictographic = UnicodeExtendedPictographicData::unicode_extended_pictographic_data_contains(first);
            if first == SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_CR && ((next) as i32) < ((end) as i32) && SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end) == SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_LF {
                next = u32::wrapping_add(next, 1);
            } else {
                if SourceInteractionBoundaries::source_interaction_boundaries_is_regional_indicator(first) && ((next) as i32) < ((end) as i32) {
                    let following = SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end);
                    if SourceInteractionBoundaries::source_interaction_boundaries_is_regional_indicator(following) {
                        next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(following));
                    }
                } else {
                    if SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_l(first) {
                        while ((next) as i32) < ((end) as i32) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_l(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                            next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                        }
                        if next as i32 < ((end) as i32) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_v(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                            while ((next) as i32) < ((end) as i32) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_v(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                                next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                            }
                            while ((next) as i32) < ((end) as i32) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_t(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                                next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                            }
                        }
                    } else {
                        if SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_lv_or_lvt(first) {
                            if SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_lv(first) {
                                while ((next) as i32) < ((end) as i32) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_v(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                                    next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                                }
                            }
                            while ((next) as i32) < ((end) as i32) && SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_t(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                                next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                            }
                        }
                    }
                }
            }
            next = SourceInteractionBoundaries::source_interaction_boundaries_consume_extenders(text, next, end);
            if preceding_emoji_modifier_base && ((next) as i32) < ((end) as i32) && SourceInteractionBoundaries::source_interaction_boundaries_is_emoji_modifier(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
                next = u32::wrapping_add(next, SourceInteractionBoundaries::source_interaction_boundaries_char_count(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)));
                preceding_emoji_modifier_base = false;
                next = SourceInteractionBoundaries::source_interaction_boundaries_consume_extenders(text, next, end);
            }
            while ((next) as i32) < ((end) as i32) && SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end) == SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_ZWJ {
                next = u32::wrapping_add(next, 1);
                if next as i32 >= end as i32 {
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
                if preceding_emoji_modifier_base && ((next) as i32) < ((end) as i32) && SourceInteractionBoundaries::source_interaction_boundaries_is_emoji_modifier(SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, next, end)) {
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
        while ((index) as i32) < ((end) as i32) {
            let code_point = SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, index, end);
            if !SourceInteractionBoundaries::source_interaction_boundaries_is_interaction_extender(code_point) {
                break;
            }
            index = u32::wrapping_add(index, SourceInteractionBoundaries::source_interaction_boundaries_char_count(code_point));
        }
        return index;
    }

    pub(crate) fn source_interaction_boundaries_is_interaction_extender(code_point: u32) -> bool {
        return code_point == SourceInteractionBoundaries::SOURCE_INTERACTION_BOUNDARIES_ZWNJ || ((code_point) as i32) >= 65024 && ((code_point) as i32) <= 65039 || ((code_point) as i32) >= 917760 && ((code_point) as i32) <= 917999 || ((code_point) as i32) >= 917536 && ((code_point) as i32) <= 917631 || SourceInteractionBoundaries::source_interaction_boundaries_is_combining_mark(code_point);
    }

    pub(crate) fn source_interaction_boundaries_is_combining_mark(code_point: u32) -> bool {
        return ((code_point) as i32) <= 65535 && UnicodeCombiningMarkData::unicode_combining_mark_data_contains(code_point);
    }

    pub(crate) fn source_interaction_boundaries_is_emoji_modifier(code_point: u32) -> bool {
        return ((code_point) as i32) >= 127995 && ((code_point) as i32) <= 127999;
    }

    pub(crate) fn source_interaction_boundaries_is_regional_indicator(code_point: u32) -> bool {
        return ((code_point) as i32) >= 127462 && ((code_point) as i32) <= 127487;
    }

    pub(crate) fn source_interaction_boundaries_is_hangul_l(code_point: u32) -> bool {
        return ((code_point) as i32) >= 4352 && ((code_point) as i32) <= 4447 || ((code_point) as i32) >= 43360 && ((code_point) as i32) <= 43388;
    }

    pub(crate) fn source_interaction_boundaries_is_hangul_v(code_point: u32) -> bool {
        return ((code_point) as i32) >= 4448 && ((code_point) as i32) <= 4519 || ((code_point) as i32) >= 55216 && ((code_point) as i32) <= 55238;
    }

    pub(crate) fn source_interaction_boundaries_is_hangul_t(code_point: u32) -> bool {
        return ((code_point) as i32) >= 4520 && ((code_point) as i32) <= 4607 || ((code_point) as i32) >= 55243 && ((code_point) as i32) <= 55291;
    }

    pub(crate) fn source_interaction_boundaries_is_hangul_lv_or_lvt(code_point: u32) -> bool {
        return ((code_point) as i32) >= 44032 && ((code_point) as i32) <= 55203;
    }

    pub(crate) fn source_interaction_boundaries_is_hangul_lv(code_point: u32) -> bool {
        return SourceInteractionBoundaries::source_interaction_boundaries_is_hangul_lv_or_lvt(code_point) && (((u32::wrapping_sub(code_point, 44032)) as i32 % 28i32) as u32) == 0;
    }

    pub(crate) fn source_interaction_boundaries_char_count(code_point: u32) -> u32 {
        return if code_point as i32 > (65535) { 2 } else { 1 };
    }

    pub(crate) fn source_interaction_boundaries_clamp(value: u32, low: u32, high: u32) -> u32 {
        if value as i32 < ((low) as i32) {
            return low;
        }
        if value as i32 > ((high) as i32) {
            return high;
        }
        return value;
    }
}
