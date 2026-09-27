#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecorationKind {
    Emphasis,
    Mourning,
    ProperNoun,
    BookTitle,
}

pub fn compare_decoration_kind(a: &DecorationKind, b: &DecorationKind) -> i32 {
    if a == b { return 0; }
    fn rank(v: &DecorationKind) -> i32 {
        match v {
            DecorationKind::Emphasis => 0,
            DecorationKind::Mourning => 1,
            DecorationKind::ProperNoun => 2,
            DecorationKind::BookTitle => 3,
        }
    }
    rank(a) - rank(b)
}

impl DecorationKind {
    pub fn to_string(&self) -> String {
        match self {
            DecorationKind::Emphasis => "Emphasis".to_string(),
            DecorationKind::Mourning => "Mourning".to_string(),
            DecorationKind::ProperNoun => "ProperNoun".to_string(),
            DecorationKind::BookTitle => "BookTitle".to_string(),
        }
    }
}

impl DecorationKind {
    pub fn name(&self) -> &'static str {
        match self {
            DecorationKind::Emphasis => "Emphasis",
            DecorationKind::Mourning => "Mourning",
            DecorationKind::ProperNoun => "ProperNoun",
            DecorationKind::BookTitle => "BookTitle",
        }
    }
}
