#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaselineClass {
    Roman,
    IdeographicCentered,
    IdeographicLow,
    Math,
    Hanging,
}

pub fn compare_baseline_class(a: &BaselineClass, b: &BaselineClass) -> i32 {
    if a == b { return 0; }
    fn rank(v: &BaselineClass) -> i32 {
        match v {
            BaselineClass::Roman => 0,
            BaselineClass::IdeographicCentered => 1,
            BaselineClass::IdeographicLow => 2,
            BaselineClass::Math => 3,
            BaselineClass::Hanging => 4,
        }
    }
    rank(a) - rank(b)
}

impl BaselineClass {
    pub fn to_string(&self) -> String {
        match self {
            BaselineClass::Roman => "Roman".to_string(),
            BaselineClass::IdeographicCentered => "IdeographicCentered".to_string(),
            BaselineClass::IdeographicLow => "IdeographicLow".to_string(),
            BaselineClass::Math => "Math".to_string(),
            BaselineClass::Hanging => "Hanging".to_string(),
        }
    }
}

impl BaselineClass {
    pub fn name(&self) -> &'static str {
        match self {
            BaselineClass::Roman => "Roman",
            BaselineClass::IdeographicCentered => "IdeographicCentered",
            BaselineClass::IdeographicLow => "IdeographicLow",
            BaselineClass::Math => "Math",
            BaselineClass::Hanging => "Hanging",
        }
    }
}
