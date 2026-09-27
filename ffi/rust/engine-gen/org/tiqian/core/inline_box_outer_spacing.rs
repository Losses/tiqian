#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InlineBoxOuterSpacing {
    Narrow,
    Source,
}

pub fn compare_inline_box_outer_spacing(a: &InlineBoxOuterSpacing, b: &InlineBoxOuterSpacing) -> i32 {
    if a == b { return 0; }
    fn rank(v: &InlineBoxOuterSpacing) -> i32 {
        match v {
            InlineBoxOuterSpacing::Narrow => 0,
            InlineBoxOuterSpacing::Source => 1,
        }
    }
    rank(a) - rank(b)
}

impl InlineBoxOuterSpacing {
    pub fn to_string(&self) -> String {
        match self {
            InlineBoxOuterSpacing::Narrow => "Narrow".to_string(),
            InlineBoxOuterSpacing::Source => "Source".to_string(),
        }
    }
}

impl InlineBoxOuterSpacing {
    pub fn name(&self) -> &'static str {
        match self {
            InlineBoxOuterSpacing::Narrow => "Narrow",
            InlineBoxOuterSpacing::Source => "Source",
        }
    }
}
