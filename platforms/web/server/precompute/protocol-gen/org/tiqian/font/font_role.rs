#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontRole {
    CjkText,
    CjkPunctuation,
    LatinText,
    Symbol,
    Emoji,
    Unknown,
}

pub fn compare_font_role(a: &FontRole, b: &FontRole) -> i32 {
    if a == b { return 0; }
    fn rank(v: &FontRole) -> i32 {
        match v {
            FontRole::CjkText => 0,
            FontRole::CjkPunctuation => 1,
            FontRole::LatinText => 2,
            FontRole::Symbol => 3,
            FontRole::Emoji => 4,
            FontRole::Unknown => 5,
        }
    }
    rank(a) - rank(b)
}

impl FontRole {
    pub fn to_string(&self) -> String {
        match self {
            FontRole::CjkText => "CjkText".to_string(),
            FontRole::CjkPunctuation => "CjkPunctuation".to_string(),
            FontRole::LatinText => "LatinText".to_string(),
            FontRole::Symbol => "Symbol".to_string(),
            FontRole::Emoji => "Emoji".to_string(),
            FontRole::Unknown => "Unknown".to_string(),
        }
    }
}

impl FontRole {
    pub fn name(&self) -> &'static str {
        match self {
            FontRole::CjkText => "CjkText",
            FontRole::CjkPunctuation => "CjkPunctuation",
            FontRole::LatinText => "LatinText",
            FontRole::Symbol => "Symbol",
            FontRole::Emoji => "Emoji",
            FontRole::Unknown => "Unknown",
        }
    }
}
