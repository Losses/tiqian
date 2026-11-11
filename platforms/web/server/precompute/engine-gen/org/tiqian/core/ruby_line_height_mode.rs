#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RubyLineHeightMode {
    PerLine,
    UniformParagraph,
}

pub fn compare_ruby_line_height_mode(a: &RubyLineHeightMode, b: &RubyLineHeightMode) -> i32 {
    if a == b { return 0; }
    fn rank(v: &RubyLineHeightMode) -> i32 {
        match v {
            RubyLineHeightMode::PerLine => 0,
            RubyLineHeightMode::UniformParagraph => 1,
        }
    }
    rank(a) - rank(b)
}

impl RubyLineHeightMode {
    pub fn to_string(&self) -> String {
        match self {
            RubyLineHeightMode::PerLine => "PerLine".to_string(),
            RubyLineHeightMode::UniformParagraph => "UniformParagraph".to_string(),
        }
    }
}

impl RubyLineHeightMode {
    pub fn name(&self) -> &'static str {
        match self {
            RubyLineHeightMode::PerLine => "PerLine",
            RubyLineHeightMode::UniformParagraph => "UniformParagraph",
        }
    }
}
