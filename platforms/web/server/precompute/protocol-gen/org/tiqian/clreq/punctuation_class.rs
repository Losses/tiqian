#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PunctuationClass {
    Opening,
    Closing,
    PauseOrStop,
    MiddleDot,
    Interpunct,
    Connector,
    Solidus,
    Ellipsis,
    Dash,
    Other,
}

pub fn compare_punctuation_class(a: &PunctuationClass, b: &PunctuationClass) -> i32 {
    if a == b { return 0; }
    fn rank(v: &PunctuationClass) -> i32 {
        match v {
            PunctuationClass::Opening => 0,
            PunctuationClass::Closing => 1,
            PunctuationClass::PauseOrStop => 2,
            PunctuationClass::MiddleDot => 3,
            PunctuationClass::Interpunct => 4,
            PunctuationClass::Connector => 5,
            PunctuationClass::Solidus => 6,
            PunctuationClass::Ellipsis => 7,
            PunctuationClass::Dash => 8,
            PunctuationClass::Other => 9,
        }
    }
    rank(a) - rank(b)
}

impl PunctuationClass {
    pub fn to_string(&self) -> String {
        match self {
            PunctuationClass::Opening => "Opening".to_string(),
            PunctuationClass::Closing => "Closing".to_string(),
            PunctuationClass::PauseOrStop => "PauseOrStop".to_string(),
            PunctuationClass::MiddleDot => "MiddleDot".to_string(),
            PunctuationClass::Interpunct => "Interpunct".to_string(),
            PunctuationClass::Connector => "Connector".to_string(),
            PunctuationClass::Solidus => "Solidus".to_string(),
            PunctuationClass::Ellipsis => "Ellipsis".to_string(),
            PunctuationClass::Dash => "Dash".to_string(),
            PunctuationClass::Other => "Other".to_string(),
        }
    }
}

impl PunctuationClass {
    pub fn name(&self) -> &'static str {
        match self {
            PunctuationClass::Opening => "Opening",
            PunctuationClass::Closing => "Closing",
            PunctuationClass::PauseOrStop => "PauseOrStop",
            PunctuationClass::MiddleDot => "MiddleDot",
            PunctuationClass::Interpunct => "Interpunct",
            PunctuationClass::Connector => "Connector",
            PunctuationClass::Solidus => "Solidus",
            PunctuationClass::Ellipsis => "Ellipsis",
            PunctuationClass::Dash => "Dash",
            PunctuationClass::Other => "Other",
        }
    }
}
