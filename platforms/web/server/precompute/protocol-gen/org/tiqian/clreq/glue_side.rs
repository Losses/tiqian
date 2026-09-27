#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GlueSide {
    LeadingOnly,
    TrailingOnly,
    BothSides,
}

pub fn compare_glue_side(a: &GlueSide, b: &GlueSide) -> i32 {
    if a == b { return 0; }
    fn rank(v: &GlueSide) -> i32 {
        match v {
            GlueSide::LeadingOnly => 0,
            GlueSide::TrailingOnly => 1,
            GlueSide::BothSides => 2,
        }
    }
    rank(a) - rank(b)
}

impl GlueSide {
    pub fn to_string(&self) -> String {
        match self {
            GlueSide::LeadingOnly => "LeadingOnly".to_string(),
            GlueSide::TrailingOnly => "TrailingOnly".to_string(),
            GlueSide::BothSides => "BothSides".to_string(),
        }
    }
}

impl GlueSide {
    pub fn name(&self) -> &'static str {
        match self {
            GlueSide::LeadingOnly => "LeadingOnly",
            GlueSide::TrailingOnly => "TrailingOnly",
            GlueSide::BothSides => "BothSides",
        }
    }
}
