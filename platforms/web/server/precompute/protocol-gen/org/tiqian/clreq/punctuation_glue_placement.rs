#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PunctuationGluePlacement {
    MainlandSimplified,
    Traditional,
}

pub fn compare_punctuation_glue_placement(a: &PunctuationGluePlacement, b: &PunctuationGluePlacement) -> i32 {
    if a == b { return 0; }
    fn rank(v: &PunctuationGluePlacement) -> i32 {
        match v {
            PunctuationGluePlacement::MainlandSimplified => 0,
            PunctuationGluePlacement::Traditional => 1,
        }
    }
    rank(a) - rank(b)
}

impl PunctuationGluePlacement {
    pub fn to_string(&self) -> String {
        match self {
            PunctuationGluePlacement::MainlandSimplified => "MainlandSimplified".to_string(),
            PunctuationGluePlacement::Traditional => "Traditional".to_string(),
        }
    }
}

impl PunctuationGluePlacement {
    pub fn name(&self) -> &'static str {
        match self {
            PunctuationGluePlacement::MainlandSimplified => "MainlandSimplified",
            PunctuationGluePlacement::Traditional => "Traditional",
        }
    }
}
