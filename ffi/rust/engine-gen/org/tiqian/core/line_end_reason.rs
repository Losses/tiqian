#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineEndReason {
    AutoWrap,
    MandatoryBreak,
    ParagraphEnd,
}

pub fn compare_line_end_reason(a: &LineEndReason, b: &LineEndReason) -> i32 {
    if a == b { return 0; }
    fn rank(v: &LineEndReason) -> i32 {
        match v {
            LineEndReason::AutoWrap => 0,
            LineEndReason::MandatoryBreak => 1,
            LineEndReason::ParagraphEnd => 2,
        }
    }
    rank(a) - rank(b)
}

impl LineEndReason {
    pub fn to_string(&self) -> String {
        match self {
            LineEndReason::AutoWrap => "AutoWrap".to_string(),
            LineEndReason::MandatoryBreak => "MandatoryBreak".to_string(),
            LineEndReason::ParagraphEnd => "ParagraphEnd".to_string(),
        }
    }
}

impl LineEndReason {
    pub fn name(&self) -> &'static str {
        match self {
            LineEndReason::AutoWrap => "AutoWrap",
            LineEndReason::MandatoryBreak => "MandatoryBreak",
            LineEndReason::ParagraphEnd => "ParagraphEnd",
        }
    }
}
