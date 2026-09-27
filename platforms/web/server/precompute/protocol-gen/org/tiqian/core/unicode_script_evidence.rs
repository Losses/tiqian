#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnicodeScriptEvidence {
    Neutral,
    EastAsian,
    Other,
}

pub fn compare_unicode_script_evidence(a: &UnicodeScriptEvidence, b: &UnicodeScriptEvidence) -> i32 {
    if a == b { return 0; }
    fn rank(v: &UnicodeScriptEvidence) -> i32 {
        match v {
            UnicodeScriptEvidence::Neutral => 0,
            UnicodeScriptEvidence::EastAsian => 1,
            UnicodeScriptEvidence::Other => 2,
        }
    }
    rank(a) - rank(b)
}

impl UnicodeScriptEvidence {
    pub fn to_string(&self) -> String {
        match self {
            UnicodeScriptEvidence::Neutral => "Neutral".to_string(),
            UnicodeScriptEvidence::EastAsian => "EastAsian".to_string(),
            UnicodeScriptEvidence::Other => "Other".to_string(),
        }
    }
}
