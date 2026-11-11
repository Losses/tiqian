#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineBreakPolicy {
    ProgressiveTechnical,
}

pub fn compare_line_break_policy(a: &LineBreakPolicy, b: &LineBreakPolicy) -> i32 {
    if a == b { return 0; }
    fn rank(v: &LineBreakPolicy) -> i32 {
        match v {
            LineBreakPolicy::ProgressiveTechnical => 0,
        }
    }
    rank(a) - rank(b)
}

impl LineBreakPolicy {
    pub fn to_string(&self) -> String {
        match self {
            LineBreakPolicy::ProgressiveTechnical => "ProgressiveTechnical".to_string(),
        }
    }
}

impl LineBreakPolicy {
    pub fn name(&self) -> &'static str {
        match self {
            LineBreakPolicy::ProgressiveTechnical => "ProgressiveTechnical",
        }
    }
}
