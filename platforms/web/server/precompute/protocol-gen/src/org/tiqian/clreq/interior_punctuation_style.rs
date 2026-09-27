#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InteriorPunctuationStyle {
    FullWidth,
    Kaiming,
}

pub fn compare_interior_punctuation_style(a: &InteriorPunctuationStyle, b: &InteriorPunctuationStyle) -> i32 {
    if a == b { return 0; }
    fn rank(v: &InteriorPunctuationStyle) -> i32 {
        match v {
            InteriorPunctuationStyle::FullWidth => 0,
            InteriorPunctuationStyle::Kaiming => 1,
        }
    }
    rank(a) - rank(b)
}

impl InteriorPunctuationStyle {
    pub fn to_string(&self) -> String {
        match self {
            InteriorPunctuationStyle::FullWidth => "FullWidth".to_string(),
            InteriorPunctuationStyle::Kaiming => "Kaiming".to_string(),
        }
    }
}

impl InteriorPunctuationStyle {
    pub fn name(&self) -> &'static str {
        match self {
            InteriorPunctuationStyle::FullWidth => "FullWidth",
            InteriorPunctuationStyle::Kaiming => "Kaiming",
        }
    }
}
