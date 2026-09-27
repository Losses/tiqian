#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BopomofoTone {
    Yinping,
    Yangping,
    Shang,
    Qu,
    Neutral,
    Ru,
}

pub fn compare_bopomofo_tone(a: &BopomofoTone, b: &BopomofoTone) -> i32 {
    if a == b { return 0; }
    fn rank(v: &BopomofoTone) -> i32 {
        match v {
            BopomofoTone::Yinping => 0,
            BopomofoTone::Yangping => 1,
            BopomofoTone::Shang => 2,
            BopomofoTone::Qu => 3,
            BopomofoTone::Neutral => 4,
            BopomofoTone::Ru => 5,
        }
    }
    rank(a) - rank(b)
}

impl BopomofoTone {
    pub fn to_string(&self) -> String {
        match self {
            BopomofoTone::Yinping => "Yinping".to_string(),
            BopomofoTone::Yangping => "Yangping".to_string(),
            BopomofoTone::Shang => "Shang".to_string(),
            BopomofoTone::Qu => "Qu".to_string(),
            BopomofoTone::Neutral => "Neutral".to_string(),
            BopomofoTone::Ru => "Ru".to_string(),
        }
    }
}

impl BopomofoTone {
    pub fn name(&self) -> &'static str {
        match self {
            BopomofoTone::Yinping => "Yinping",
            BopomofoTone::Yangping => "Yangping",
            BopomofoTone::Shang => "Shang",
            BopomofoTone::Qu => "Qu",
            BopomofoTone::Neutral => "Neutral",
            BopomofoTone::Ru => "Ru",
        }
    }
}
