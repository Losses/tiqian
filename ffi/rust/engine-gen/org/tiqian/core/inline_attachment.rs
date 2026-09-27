#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InlineAttachment {
    None,
    Previous,
}

pub fn compare_inline_attachment(a: &InlineAttachment, b: &InlineAttachment) -> i32 {
    if a == b { return 0; }
    fn rank(v: &InlineAttachment) -> i32 {
        match v {
            InlineAttachment::None => 0,
            InlineAttachment::Previous => 1,
        }
    }
    rank(a) - rank(b)
}

impl InlineAttachment {
    pub fn to_string(&self) -> String {
        match self {
            InlineAttachment::None => "None".to_string(),
            InlineAttachment::Previous => "Previous".to_string(),
        }
    }
}

impl InlineAttachment {
    pub fn name(&self) -> &'static str {
        match self {
            InlineAttachment::None => "None",
            InlineAttachment::Previous => "Previous",
        }
    }
}
