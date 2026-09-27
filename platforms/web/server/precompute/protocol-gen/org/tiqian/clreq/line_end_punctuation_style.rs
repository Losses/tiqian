#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineEndPunctuationStyle {
    ForceHalfWidth,
    AllowFullWidth,
}

pub fn compare_line_end_punctuation_style(a: &LineEndPunctuationStyle, b: &LineEndPunctuationStyle) -> i32 {
    if a == b { return 0; }
    fn rank(v: &LineEndPunctuationStyle) -> i32 {
        match v {
            LineEndPunctuationStyle::ForceHalfWidth => 0,
            LineEndPunctuationStyle::AllowFullWidth => 1,
        }
    }
    rank(a) - rank(b)
}

impl LineEndPunctuationStyle {
    pub fn to_string(&self) -> String {
        match self {
            LineEndPunctuationStyle::ForceHalfWidth => "ForceHalfWidth".to_string(),
            LineEndPunctuationStyle::AllowFullWidth => "AllowFullWidth".to_string(),
        }
    }
}

impl LineEndPunctuationStyle {
    pub fn name(&self) -> &'static str {
        match self {
            LineEndPunctuationStyle::ForceHalfWidth => "ForceHalfWidth",
            LineEndPunctuationStyle::AllowFullWidth => "AllowFullWidth",
        }
    }
}
