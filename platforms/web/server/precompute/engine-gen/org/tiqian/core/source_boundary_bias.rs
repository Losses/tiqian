#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceBoundaryBias {
    Backward,
    Forward,
    Nearest,
}

pub fn compare_source_boundary_bias(a: &SourceBoundaryBias, b: &SourceBoundaryBias) -> i32 {
    if a == b { return 0; }
    fn rank(v: &SourceBoundaryBias) -> i32 {
        match v {
            SourceBoundaryBias::Backward => 0,
            SourceBoundaryBias::Forward => 1,
            SourceBoundaryBias::Nearest => 2,
        }
    }
    rank(a) - rank(b)
}

impl SourceBoundaryBias {
    pub fn to_string(&self) -> String {
        match self {
            SourceBoundaryBias::Backward => "Backward".to_string(),
            SourceBoundaryBias::Forward => "Forward".to_string(),
            SourceBoundaryBias::Nearest => "Nearest".to_string(),
        }
    }
}
