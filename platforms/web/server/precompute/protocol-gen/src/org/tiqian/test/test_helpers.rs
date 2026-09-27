use crate::runtime::fp_helper::FPHelper;


#[derive(Clone, Copy)]
pub struct TestHelpers;

impl TestHelpers {
    pub fn test_helpers_f32_literal(value: f64) -> f64 {
        return FPHelper::i32_to_float(FPHelper::float_to_i32(value));
    }

    pub fn test_helpers_f32_bits(bits: u32) -> f64 {
        return FPHelper::i32_to_float(i32::from_ne_bytes((bits).to_ne_bytes()));
    }

    pub fn test_helpers_surrogate_text(code_units: &Vec<u32>) -> String {
        let mut output = String::new();
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((code_units.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let unit = code_units[usize::try_from(index).unwrap_or(0)];
            if i32::from_ne_bytes((unit).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes((unit).to_ne_bytes())) <= 56319 && (i32::from_ne_bytes((u32::wrapping_add(index, 1)).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((code_units.len()) &
0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) && (i32::from_ne_bytes((code_units[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)]).to_ne_bytes())) >= 56320 && (i32::from_ne_bytes((code_units[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)]).to_ne_bytes())) <=
57343 {
                let low = code_units[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)];
                output += &(if u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320)) > 0xFFFF { String::from_utf16(&[0xD800 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)),
u32::wrapping_sub(low, 56320))) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) - 0x10000) & 0x3FF) as u16]).unwrap() } else {
String::from_utf16_lossy(&[(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) as u16]) });
                index = u32::wrapping_add(index, 2);
            } else {
                output += &(if unit > 0xFFFF { String::from_utf16(&[0xD800 + (((unit) - 0x10000) >> 10) as u16, 0xDC00 + (((unit) - 0x10000) & 0x3FF) as u16]).unwrap() } else { String::from_utf16_lossy(&[(unit) as u16]) });
                index = u32::wrapping_add(index, 1);
            }
        }
        return output;
    }
}
