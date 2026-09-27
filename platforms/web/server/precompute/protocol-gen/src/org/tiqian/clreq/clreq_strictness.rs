#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClreqStrictness {
    Loose,
    Normal,
    Strict,
}

pub fn compare_clreq_strictness(a: &ClreqStrictness, b: &ClreqStrictness) -> i32 {
    if a == b { return 0; }
    fn rank(v: &ClreqStrictness) -> i32 {
        match v {
            ClreqStrictness::Loose => 0,
            ClreqStrictness::Normal => 1,
            ClreqStrictness::Strict => 2,
        }
    }
    rank(a) - rank(b)
}

impl ClreqStrictness {
    pub fn to_string(&self) -> String {
        match self {
            ClreqStrictness::Loose => "Loose".to_string(),
            ClreqStrictness::Normal => "Normal".to_string(),
            ClreqStrictness::Strict => "Strict".to_string(),
        }
    }
}

impl ClreqStrictness {
    pub fn name(&self) -> &'static str {
        match self {
            ClreqStrictness::Loose => "Loose",
            ClreqStrictness::Normal => "Normal",
            ClreqStrictness::Strict => "Strict",
        }
    }
}
