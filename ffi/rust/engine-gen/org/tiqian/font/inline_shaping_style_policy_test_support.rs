#[derive(Clone, Copy)]
pub struct InlineShapingStylePolicyTestSupport;

impl InlineShapingStylePolicyTestSupport {
    pub fn inline_shaping_style_policy_test_support_vals(n: u32) -> Vec<String> {
        let mut a: Vec<String> = vec![];
        for _ in 0..n {
            a.push("value".to_string());
        }
        return a;
    }
}
