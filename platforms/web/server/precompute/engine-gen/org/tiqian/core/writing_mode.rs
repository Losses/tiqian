#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WritingMode {
    HorizontalTb,
    VerticalRl,
}

pub fn compare_writing_mode(a: &WritingMode, b: &WritingMode) -> i32 {
    if a == b { return 0; }
    fn rank(v: &WritingMode) -> i32 {
        match v {
            WritingMode::HorizontalTb => 0,
            WritingMode::VerticalRl => 1,
        }
    }
    rank(a) - rank(b)
}

impl WritingMode {
    pub fn to_string(&self) -> String {
        match self {
            WritingMode::HorizontalTb => "HorizontalTb".to_string(),
            WritingMode::VerticalRl => "VerticalRl".to_string(),
        }
    }
}

impl WritingMode {
    pub fn name(&self) -> &'static str {
        match self {
            WritingMode::HorizontalTb => "HorizontalTb",
            WritingMode::VerticalRl => "VerticalRl",
        }
    }
}
