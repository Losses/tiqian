use crate::runtime::u_string;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct FontPolicyCoverageTestSupport;

impl FontPolicyCoverageTestSupport {
    pub fn font_policy_coverage_test_support_surrogate_text(codes: &Vec<u32>) -> UString {
        let mut result = UString::new();
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((codes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let unit = codes[usize::try_from(i).unwrap_or(0)];
            if ({ let v: u32 = unit; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 55296 && ({ let v: u32 = unit; i32::from_ne_bytes(v.to_ne_bytes()) }) <= 56319 && (i32::from_ne_bytes(((u32::wrapping_add(i, 1)) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((codes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) && ({ let v: u32 = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 56320 && ({ let v: u32 = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) <= 57343 {
                let low = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)];
                result += &(if u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320)) > 0xFFFF { u_string::from_units(&[0xD800 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) as u16]) });
                i = u32::wrapping_add(i, 2);
            } else {
                result += &(if unit > 0xFFFF { u_string::from_units(&[0xD800 + (((unit) - 0x10000) >> 10) as u16, 0xDC00 + (((unit) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(unit) as u16]) });
                i = u32::wrapping_add(i, 1);
            }
        }
        return result;
    }

    pub fn font_policy_coverage_test_support_copy_strings(values: &[UString]) -> Vec<UString> {
        let mut result: Vec<UString> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            result.push((values[usize::try_from(i).unwrap_or(0)]).clone());
            i = u32::wrapping_add(i, 1);
        }
        return result;
    }
}
