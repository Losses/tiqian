#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KinsokuLevel {
    None,
    Basic,
    GbStyle,
    Strict,
}

pub fn compare_kinsoku_level(a: &KinsokuLevel, b: &KinsokuLevel) -> i32 {
    if a == b { return 0; }
    fn rank(v: &KinsokuLevel) -> i32 {
        match v {
            KinsokuLevel::None => 0,
            KinsokuLevel::Basic => 1,
            KinsokuLevel::GbStyle => 2,
            KinsokuLevel::Strict => 3,
        }
    }
    rank(a) - rank(b)
}

impl KinsokuLevel {
    pub fn to_string(&self) -> String {
        match self {
            KinsokuLevel::None => "None".to_string(),
            KinsokuLevel::Basic => "Basic".to_string(),
            KinsokuLevel::GbStyle => "GbStyle".to_string(),
            KinsokuLevel::Strict => "Strict".to_string(),
        }
    }
}

impl KinsokuLevel {
    pub fn name(&self) -> &'static str {
        match self {
            KinsokuLevel::None => "None",
            KinsokuLevel::Basic => "Basic",
            KinsokuLevel::GbStyle => "GbStyle",
            KinsokuLevel::Strict => "Strict",
        }
    }
}
