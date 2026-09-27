#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PunctuationFontPolicy {
    PreferCjkForAmbiguousPunctuation,
    PreferLatinForAscii,
    PreserveRunFont,
    CustomMap,
}

pub fn compare_punctuation_font_policy(a: &PunctuationFontPolicy, b: &PunctuationFontPolicy) -> i32 {
    if a == b { return 0; }
    fn rank(v: &PunctuationFontPolicy) -> i32 {
        match v {
            PunctuationFontPolicy::PreferCjkForAmbiguousPunctuation => 0,
            PunctuationFontPolicy::PreferLatinForAscii => 1,
            PunctuationFontPolicy::PreserveRunFont => 2,
            PunctuationFontPolicy::CustomMap => 3,
        }
    }
    rank(a) - rank(b)
}

impl PunctuationFontPolicy {
    pub fn to_string(&self) -> String {
        match self {
            PunctuationFontPolicy::PreferCjkForAmbiguousPunctuation => "PreferCjkForAmbiguousPunctuation".to_string(),
            PunctuationFontPolicy::PreferLatinForAscii => "PreferLatinForAscii".to_string(),
            PunctuationFontPolicy::PreserveRunFont => "PreserveRunFont".to_string(),
            PunctuationFontPolicy::CustomMap => "CustomMap".to_string(),
        }
    }
}

impl PunctuationFontPolicy {
    pub const ALL: [PunctuationFontPolicy; 4] = [PunctuationFontPolicy::PreferCjkForAmbiguousPunctuation, PunctuationFontPolicy::PreferLatinForAscii, PunctuationFontPolicy::PreserveRunFont, PunctuationFontPolicy::CustomMap];
    pub fn name(&self) -> &'static str {
        match self {
            PunctuationFontPolicy::PreferCjkForAmbiguousPunctuation => "PreferCjkForAmbiguousPunctuation",
            PunctuationFontPolicy::PreferLatinForAscii => "PreferLatinForAscii",
            PunctuationFontPolicy::PreserveRunFont => "PreserveRunFont",
            PunctuationFontPolicy::CustomMap => "CustomMap",
        }
    }
}
