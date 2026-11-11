#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClreqRegion {
    Mainland,
    Taiwan,
    HongKong,
    Custom,
}

pub fn compare_clreq_region(a: &ClreqRegion, b: &ClreqRegion) -> i32 {
    if a == b { return 0; }
    fn rank(v: &ClreqRegion) -> i32 {
        match v {
            ClreqRegion::Mainland => 0,
            ClreqRegion::Taiwan => 1,
            ClreqRegion::HongKong => 2,
            ClreqRegion::Custom => 3,
        }
    }
    rank(a) - rank(b)
}

impl ClreqRegion {
    pub fn to_string(&self) -> String {
        match self {
            ClreqRegion::Mainland => "Mainland".to_string(),
            ClreqRegion::Taiwan => "Taiwan".to_string(),
            ClreqRegion::HongKong => "HongKong".to_string(),
            ClreqRegion::Custom => "Custom".to_string(),
        }
    }
}

impl ClreqRegion {
    pub fn name(&self) -> &'static str {
        match self {
            ClreqRegion::Mainland => "Mainland",
            ClreqRegion::Taiwan => "Taiwan",
            ClreqRegion::HongKong => "HongKong",
            ClreqRegion::Custom => "Custom",
        }
    }
}
