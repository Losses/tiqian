#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HangingPunctuationStyle {
    Disabled,
    PauseStops,
}

pub fn compare_hanging_punctuation_style(a: &HangingPunctuationStyle, b: &HangingPunctuationStyle) -> i32 {
    if a == b { return 0; }
    fn rank(v: &HangingPunctuationStyle) -> i32 {
        match v {
            HangingPunctuationStyle::Disabled => 0,
            HangingPunctuationStyle::PauseStops => 1,
        }
    }
    rank(a) - rank(b)
}

impl HangingPunctuationStyle {
    pub fn to_string(&self) -> String {
        match self {
            HangingPunctuationStyle::Disabled => "Disabled".to_string(),
            HangingPunctuationStyle::PauseStops => "PauseStops".to_string(),
        }
    }
}

impl HangingPunctuationStyle {
    pub fn name(&self) -> &'static str {
        match self {
            HangingPunctuationStyle::Disabled => "Disabled",
            HangingPunctuationStyle::PauseStops => "PauseStops",
        }
    }
}
