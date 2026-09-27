use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::unicode_number_data::UnicodeNumberData;
use crate::runtime::u_string;


#[derive(Clone, Copy)]
pub struct NumberSymbolCohesion;

impl NumberSymbolCohesion {
    pub fn number_symbol_cohesion_unbreakable_ranges(text: &str) -> Vec<IntRange> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut result: Vec<IntRange> = vec![];
        let mut i = 0u32;
        let __units1 = u_string::units(&text);
        let __count1 = u_string::unit_count(&text);
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            if !UnicodeNumberData::unicode_number_data_contains(*(u_string::unit_at_from(&__units1, i)).as_ref().unwrap()) {
                i = u32::wrapping_add(i, 1);
                continue;
            }
            let mut end = i;
            while (i32::from_ne_bytes((u32::wrapping_add(end, 1)).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
                let unit = u_string::unit_at_from(&__units, u32::wrapping_add(end, 1)).unwrap_or(0);
                if UnicodeNumberData::unicode_number_data_contains(unit) {
                    end = u32::wrapping_add(end, 1);
                } else {
                    if (unit == 46 || unit == 44) && (i32::from_ne_bytes((u32::wrapping_add(end, 2)).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) && UnicodeNumberData::unicode_number_data_contains(*(u_string::unit_at_from(&__units, u32::wrapping_add(end,
2))).as_ref().unwrap()) {
                        end = u32::wrapping_add(end, 2);
                    } else {
                        break;
                    }
                }
            }
            let mut start = i;
            if i32::from_ne_bytes((start).to_ne_bytes()) > (0) && (NumberSymbolCohesion::number_symbol_cohesion_is_prefix_sign(*(u_string::unit_at_from(&__units, u32::wrapping_sub(start, 1))).as_ref().unwrap()) ||
NumberSymbolCohesion::number_symbol_cohesion_is_front_currency(*(u_string::unit_at_from(&__units, u32::wrapping_sub(start, 1))).as_ref().unwrap())) {
                start = u32::wrapping_sub(start, 1);
            }
            while (i32::from_ne_bytes((u32::wrapping_add(end, 1)).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) && NumberSymbolCohesion::number_symbol_cohesion_is_suffix_unit(*(u_string::unit_at_from(&__units, u32::wrapping_add(end, 1))).as_ref().unwrap()) {
                end = u32::wrapping_add(end, 1);
            }
            if i32::from_ne_bytes((u32::wrapping_add(end, 1)).to_ne_bytes()) < (i32::from_ne_bytes((__count).to_ne_bytes())) && NumberSymbolCohesion::number_symbol_cohesion_is_back_currency(*(u_string::unit_at_from(&__units, u32::wrapping_add(end, 1))).as_ref().unwrap()) {
                end = u32::wrapping_add(end, 1);
            }
            result.push(IntRange::new(start, end));
            i = u32::wrapping_add(end, 1);
        }
        return result;
    }

    pub(crate) fn number_symbol_cohesion_is_prefix_sign(unit: u32) -> bool {
        return unit == 43 || unit == 45 || unit == 177;
    }

    pub(crate) fn number_symbol_cohesion_is_suffix_unit(unit: u32) -> bool {
        return unit == 37 || unit == 8240 || unit == 176 || unit == 8451 || unit == 8457 || unit == 8242 || unit == 8243;
    }

    pub(crate) fn number_symbol_cohesion_is_front_currency(unit: u32) -> bool {
        return unit == 165 || unit == 65509 || unit == 36 || unit == 65284 || unit == 8364 || unit == 163 || unit == 8361 || unit == 8381 || unit == 8377 || unit == 3647;
    }

    pub(crate) fn number_symbol_cohesion_is_back_currency(unit: u32) -> bool {
        return unit == 8363;
    }
}
