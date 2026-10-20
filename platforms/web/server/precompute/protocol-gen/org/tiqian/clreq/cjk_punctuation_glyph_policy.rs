#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CjkPunctuationGlyphPolicy {
    PreserveInput,
    PreferClreqRecommendedCodepoints,
    ForceClreqRecommendedCodepoints,
}

pub fn compare_cjk_punctuation_glyph_policy(a: &CjkPunctuationGlyphPolicy, b: &CjkPunctuationGlyphPolicy) -> i32 {
    if a == b { return 0; }
    fn rank(v: &CjkPunctuationGlyphPolicy) -> i32 {
        match v {
            CjkPunctuationGlyphPolicy::PreserveInput => 0,
            CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints => 1,
            CjkPunctuationGlyphPolicy::ForceClreqRecommendedCodepoints => 2,
        }
    }
    rank(a) - rank(b)
}

impl CjkPunctuationGlyphPolicy {
    pub fn to_string(&self) -> String {
        match self {
            CjkPunctuationGlyphPolicy::PreserveInput => "PreserveInput".to_string(),
            CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints => "PreferClreqRecommendedCodepoints".to_string(),
            CjkPunctuationGlyphPolicy::ForceClreqRecommendedCodepoints => "ForceClreqRecommendedCodepoints".to_string(),
        }
    }
}

impl CjkPunctuationGlyphPolicy {
    pub fn name(&self) -> &'static str {
        match self {
            CjkPunctuationGlyphPolicy::PreserveInput => "PreserveInput",
            CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints => "PreferClreqRecommendedCodepoints",
            CjkPunctuationGlyphPolicy::ForceClreqRecommendedCodepoints => "ForceClreqRecommendedCodepoints",
        }
    }
}
