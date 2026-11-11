#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LastLineAlignment {
    Start,
    Center,
    End,
}

pub fn compare_last_line_alignment(a: &LastLineAlignment, b: &LastLineAlignment) -> i32 {
    if a == b { return 0; }
    fn rank(v: &LastLineAlignment) -> i32 {
        match v {
            LastLineAlignment::Start => 0,
            LastLineAlignment::Center => 1,
            LastLineAlignment::End => 2,
        }
    }
    rank(a) - rank(b)
}

impl LastLineAlignment {
    pub fn to_string(&self) -> String {
        match self {
            LastLineAlignment::Start => "Start".to_string(),
            LastLineAlignment::Center => "Center".to_string(),
            LastLineAlignment::End => "End".to_string(),
        }
    }
}

impl LastLineAlignment {
    pub fn name(&self) -> &'static str {
        match self {
            LastLineAlignment::Start => "Start",
            LastLineAlignment::Center => "Center",
            LastLineAlignment::End => "End",
        }
    }
}
