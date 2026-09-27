#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InlineObjectPreferredStretchKind {
    PunctuationTrailing,
    Relation,
    BinaryOperator,
}

pub fn compare_inline_object_preferred_stretch_kind(a: &InlineObjectPreferredStretchKind, b: &InlineObjectPreferredStretchKind) -> i32 {
    if a == b { return 0; }
    fn rank(v: &InlineObjectPreferredStretchKind) -> i32 {
        match v {
            InlineObjectPreferredStretchKind::PunctuationTrailing => 0,
            InlineObjectPreferredStretchKind::Relation => 1,
            InlineObjectPreferredStretchKind::BinaryOperator => 2,
        }
    }
    rank(a) - rank(b)
}

impl InlineObjectPreferredStretchKind {
    pub fn to_string(&self) -> String {
        match self {
            InlineObjectPreferredStretchKind::PunctuationTrailing => "PunctuationTrailing".to_string(),
            InlineObjectPreferredStretchKind::Relation => "Relation".to_string(),
            InlineObjectPreferredStretchKind::BinaryOperator => "BinaryOperator".to_string(),
        }
    }
}

impl InlineObjectPreferredStretchKind {
    pub fn name(&self) -> &'static str {
        match self {
            InlineObjectPreferredStretchKind::PunctuationTrailing => "PunctuationTrailing",
            InlineObjectPreferredStretchKind::Relation => "Relation",
            InlineObjectPreferredStretchKind::BinaryOperator => "BinaryOperator",
        }
    }
}
