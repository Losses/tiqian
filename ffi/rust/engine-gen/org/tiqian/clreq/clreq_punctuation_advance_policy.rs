use crate::runtime::u_string;
use crate::runtime::u_string::UStr;


#[derive(Clone, Copy)]
pub struct ClreqPunctuationAdvancePolicy;

impl ClreqPunctuationAdvancePolicy {
    const CLREQ_PUNCTUATION_ADVANCE_POLICY_TWO_EM_DASH: &UStr = unsafe { &*(&[0x2E3Au16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    pub fn clreq_punctuation_advance_policy_advance_em(source_text: &UStr, display_text: &UStr) -> f64 {
        if display_text == ClreqPunctuationAdvancePolicy::CLREQ_PUNCTUATION_ADVANCE_POLICY_TWO_EM_DASH.to_ustring() {
            return 2.0f64;
        }
        if source_text == ClreqPunctuationAdvancePolicy::CLREQ_PUNCTUATION_ADVANCE_POLICY_TWO_EM_DASH.to_ustring() {
            return 2.0f64;
        }
        return ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_code_point_count(source_text);
    }

    pub(crate) fn clreq_punctuation_advance_policy_code_point_count(text: &UStr) -> f64 {
        let mut count = 0u32;
        let mut index = 0u32;
        while ((index) as i32) < ((u_string::unit_count(&(text))) as i32) {
            index = u32::wrapping_add(index, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_char_count(ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_code_point_at_compat(text, index)));
            count = u32::wrapping_add(count, 1);
        }
        return ((count) as i32) as f64;
    }

    pub(crate) fn clreq_punctuation_advance_policy_code_point_at_compat(text: &UStr, index: u32) -> u32 {
    let __units = text.as_slice();
    let __count = u_string::unit_count(&text);
        let high = u_string::unit_at_from(__units, index).unwrap_or(0);
        if high as i32 < (55296) || ((high) as i32) > (56319) || ((u32::wrapping_add(index, 1)) as i32) >= __count as i32 {
            return high;
        }
        let low = u_string::unit_at_from(__units, u32::wrapping_add(index, 1)).unwrap_or(0);
        if low as i32 < (56320) || ((low) as i32) > (57343) {
            return high;
        }
        return u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(high, 55296)) << (10)), u32::wrapping_sub(low, 56320));
    }

    pub(crate) fn clreq_punctuation_advance_policy_char_count(code_point: u32) -> u32 {
        if code_point as i32 > (65535) {
            return 2;
        }
        return 1;
    }
}
