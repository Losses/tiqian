#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlanEndReason {
    AutoWrap,
    MandatoryBreak,
    ParagraphEnd,
}

pub fn compare_plan_end_reason(a: &PlanEndReason, b: &PlanEndReason) -> i32 {
    if a == b { return 0; }
    fn rank(v: &PlanEndReason) -> i32 {
        match v {
            PlanEndReason::AutoWrap => 0,
            PlanEndReason::MandatoryBreak => 1,
            PlanEndReason::ParagraphEnd => 2,
        }
    }
    rank(a) - rank(b)
}

impl PlanEndReason {
    pub fn to_string(&self) -> String {
        match self {
            PlanEndReason::AutoWrap => "AutoWrap".to_string(),
            PlanEndReason::MandatoryBreak => "MandatoryBreak".to_string(),
            PlanEndReason::ParagraphEnd => "ParagraphEnd".to_string(),
        }
    }
}
