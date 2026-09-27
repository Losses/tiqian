#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaselinePolicy {
    Alphabetic,
    Ideographic,
    CenteredCjkVisual,
}

pub fn compare_baseline_policy(a: &BaselinePolicy, b: &BaselinePolicy) -> i32 {
    if a == b { return 0; }
    fn rank(v: &BaselinePolicy) -> i32 {
        match v {
            BaselinePolicy::Alphabetic => 0,
            BaselinePolicy::Ideographic => 1,
            BaselinePolicy::CenteredCjkVisual => 2,
        }
    }
    rank(a) - rank(b)
}

impl BaselinePolicy {
    pub fn to_string(&self) -> String {
        match self {
            BaselinePolicy::Alphabetic => "Alphabetic".to_string(),
            BaselinePolicy::Ideographic => "Ideographic".to_string(),
            BaselinePolicy::CenteredCjkVisual => "CenteredCjkVisual".to_string(),
        }
    }
}

impl BaselinePolicy {
    pub const ALL: [BaselinePolicy; 3] = [BaselinePolicy::Alphabetic, BaselinePolicy::Ideographic, BaselinePolicy::CenteredCjkVisual];
    pub fn name(&self) -> &'static str {
        match self {
            BaselinePolicy::Alphabetic => "Alphabetic",
            BaselinePolicy::Ideographic => "Ideographic",
            BaselinePolicy::CenteredCjkVisual => "CenteredCjkVisual",
        }
    }
}
