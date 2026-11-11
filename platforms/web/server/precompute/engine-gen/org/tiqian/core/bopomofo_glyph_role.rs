#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BopomofoGlyphRole {
    Symbol,
    Tone,
    Neutral,
}

pub fn compare_bopomofo_glyph_role(a: &BopomofoGlyphRole, b: &BopomofoGlyphRole) -> i32 {
    if a == b { return 0; }
    fn rank(v: &BopomofoGlyphRole) -> i32 {
        match v {
            BopomofoGlyphRole::Symbol => 0,
            BopomofoGlyphRole::Tone => 1,
            BopomofoGlyphRole::Neutral => 2,
        }
    }
    rank(a) - rank(b)
}

impl BopomofoGlyphRole {
    pub fn to_string(&self) -> String {
        match self {
            BopomofoGlyphRole::Symbol => "Symbol".to_string(),
            BopomofoGlyphRole::Tone => "Tone".to_string(),
            BopomofoGlyphRole::Neutral => "Neutral".to_string(),
        }
    }
}

impl BopomofoGlyphRole {
    pub fn name(&self) -> &'static str {
        match self {
            BopomofoGlyphRole::Symbol => "Symbol",
            BopomofoGlyphRole::Tone => "Tone",
            BopomofoGlyphRole::Neutral => "Neutral",
        }
    }
}
