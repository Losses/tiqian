#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RubyKind {
    Pinyin,
    Bopomofo,
}

pub fn compare_ruby_kind(a: &RubyKind, b: &RubyKind) -> i32 {
    if a == b { return 0; }
    fn rank(v: &RubyKind) -> i32 {
        match v {
            RubyKind::Pinyin => 0,
            RubyKind::Bopomofo => 1,
        }
    }
    rank(a) - rank(b)
}

impl RubyKind {
    pub fn to_string(&self) -> String {
        match self {
            RubyKind::Pinyin => "Pinyin".to_string(),
            RubyKind::Bopomofo => "Bopomofo".to_string(),
        }
    }
}

impl RubyKind {
    pub fn name(&self) -> &'static str {
        match self {
            RubyKind::Pinyin => "Pinyin",
            RubyKind::Bopomofo => "Bopomofo",
        }
    }
}
