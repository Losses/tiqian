pub static RANGES: [u32; 80] = [
    0x261d, 0x261d, 0x26f9, 0x26f9, 0x270a, 0x270d, 0x1f385, 0x1f385,
    0x1f3c2, 0x1f3c4, 0x1f3c7, 0x1f3c7, 0x1f3ca, 0x1f3cc, 0x1f442, 0x1f443,
    0x1f446, 0x1f450, 0x1f466, 0x1f478, 0x1f47c, 0x1f47c, 0x1f481, 0x1f483,
    0x1f485, 0x1f487, 0x1f48f, 0x1f48f, 0x1f491, 0x1f491, 0x1f4aa, 0x1f4aa,
    0x1f574, 0x1f575, 0x1f57a, 0x1f57a, 0x1f590, 0x1f590, 0x1f595, 0x1f596,
    0x1f645, 0x1f647, 0x1f64b, 0x1f64f, 0x1f6a3, 0x1f6a3, 0x1f6b4, 0x1f6b6,
    0x1f6c0, 0x1f6c0, 0x1f6cc, 0x1f6cc, 0x1f90c, 0x1f90c, 0x1f90f, 0x1f90f,
    0x1f918, 0x1f91f, 0x1f926, 0x1f926, 0x1f930, 0x1f939, 0x1f93c, 0x1f93e,
    0x1f977, 0x1f977, 0x1f9b5, 0x1f9b6, 0x1f9b8, 0x1f9b9, 0x1f9bb, 0x1f9bb,
    0x1f9cd, 0x1f9cf, 0x1f9d1, 0x1f9dd, 0x1fac3, 0x1fac5, 0x1faf0, 0x1faf8
];

#[derive(Clone, Copy)]
pub struct UnicodeEmojiModifierBaseData;

impl UnicodeEmojiModifierBaseData {

    pub fn unicode_emoji_modifier_base_data_contains(code_point: u32) -> bool {
        let mut low = 0u32;
        let mut high = u32::wrapping_sub(u32::try_from((RANGES.len()) & 0xFFFF_FFFF).unwrap_or(0) >> 1, 1);
        while (i32::from_ne_bytes((low).to_ne_bytes())) <= i32::from_ne_bytes((high).to_ne_bytes()) {
            let middle = u32::wrapping_add(low, high) >> 1;
            let base = u32::wrapping_mul(middle, 2);
            if i32::from_ne_bytes((code_point).to_ne_bytes()) < ({ let v: u32 = RANGES[usize::try_from(base).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                high = u32::wrapping_sub(middle, 1);
            } else {
                if i32::from_ne_bytes((code_point).to_ne_bytes()) > ({ let v: u32 = RANGES[usize::try_from(u32::wrapping_add(base, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                    low = u32::wrapping_add(middle, 1);
                } else {
                    return true;
                }
            }
        }
        return false;
    }
}
