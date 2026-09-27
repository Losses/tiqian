use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;


#[derive(Debug, Clone, PartialEq)]
pub struct LineBreakSpan {
    pub range: TextRange,
    pub policy: LineBreakPolicy,
}

impl LineBreakSpan {
    pub fn new(range: TextRange, policy: LineBreakPolicy) -> Self {
        Self {
            range,
            policy,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "LineBreakSpan(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "policy=",
            self.policy.name(),
            ")"
        );
    }
}

fn line_break_span_policy_order(v: &LineBreakPolicy) -> i32 {
    match v {
        LineBreakPolicy::ProgressiveTechnical => 0,
    }
}
pub fn compare_line_break_span(a: &LineBreakSpan, b: &LineBreakSpan) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_policy = match line_break_span_policy_order(&a.policy).cmp(&line_break_span_policy_order(&b.policy)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_policy != 0 { return cmp_policy; }
    0
}
