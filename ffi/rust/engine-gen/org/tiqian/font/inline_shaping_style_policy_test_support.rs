use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct InlineShapingStylePolicyTestSupport;

impl InlineShapingStylePolicyTestSupport {
    pub fn inline_shaping_style_policy_test_support_vals(n: u32) -> Vec<UString> {
        let mut a: Vec<UString> = vec![];
        for _ in 0..n {
            a.push(UString::from("value").to_ustring());
        }
        return a;
    }
}
