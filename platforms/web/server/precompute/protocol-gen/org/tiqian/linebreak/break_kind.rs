#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BreakKind {
    Allowed,
    Forbidden,
    Required,
    Problematic,
}

pub fn compare_break_kind(a: &BreakKind, b: &BreakKind) -> i32 {
    if a == b { return 0; }
    fn rank(v: &BreakKind) -> i32 {
        match v {
            BreakKind::Allowed => 0,
            BreakKind::Forbidden => 1,
            BreakKind::Required => 2,
            BreakKind::Problematic => 3,
        }
    }
    rank(a) - rank(b)
}

impl BreakKind {
    pub fn to_string(&self) -> String {
        match self {
            BreakKind::Allowed => "Allowed".to_string(),
            BreakKind::Forbidden => "Forbidden".to_string(),
            BreakKind::Required => "Required".to_string(),
            BreakKind::Problematic => "Problematic".to_string(),
        }
    }
}

impl BreakKind {
    pub fn name(&self) -> &'static str {
        match self {
            BreakKind::Allowed => "Allowed",
            BreakKind::Forbidden => "Forbidden",
            BreakKind::Required => "Required",
            BreakKind::Problematic => "Problematic",
        }
    }
}
