#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoSpaceMode {
    Disabled,
    Replace,
    Insert,
}

pub fn compare_auto_space_mode(a: &AutoSpaceMode, b: &AutoSpaceMode) -> i32 {
    if a == b { return 0; }
    fn rank(v: &AutoSpaceMode) -> i32 {
        match v {
            AutoSpaceMode::Disabled => 0,
            AutoSpaceMode::Replace => 1,
            AutoSpaceMode::Insert => 2,
        }
    }
    rank(a) - rank(b)
}

impl AutoSpaceMode {
    pub fn to_string(&self) -> String {
        match self {
            AutoSpaceMode::Disabled => "Disabled".to_string(),
            AutoSpaceMode::Replace => "Replace".to_string(),
            AutoSpaceMode::Insert => "Insert".to_string(),
        }
    }
}

impl AutoSpaceMode {
    pub fn name(&self) -> &'static str {
        match self {
            AutoSpaceMode::Disabled => "Disabled",
            AutoSpaceMode::Replace => "Replace",
            AutoSpaceMode::Insert => "Insert",
        }
    }
}
