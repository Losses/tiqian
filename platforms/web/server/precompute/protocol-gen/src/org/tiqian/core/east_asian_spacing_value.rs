#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EastAsianSpacingValue {
    Wide,
    Narrow,
    Other,
    Conditional,
}

pub fn compare_east_asian_spacing_value(a: &EastAsianSpacingValue, b: &EastAsianSpacingValue) -> i32 {
    if a == b { return 0; }
    fn rank(v: &EastAsianSpacingValue) -> i32 {
        match v {
            EastAsianSpacingValue::Wide => 0,
            EastAsianSpacingValue::Narrow => 1,
            EastAsianSpacingValue::Other => 2,
            EastAsianSpacingValue::Conditional => 3,
        }
    }
    rank(a) - rank(b)
}

impl EastAsianSpacingValue {
    pub fn to_string(&self) -> String {
        match self {
            EastAsianSpacingValue::Wide => "Wide".to_string(),
            EastAsianSpacingValue::Narrow => "Narrow".to_string(),
            EastAsianSpacingValue::Other => "Other".to_string(),
            EastAsianSpacingValue::Conditional => "Conditional".to_string(),
        }
    }
}

impl EastAsianSpacingValue {
    pub fn name(&self) -> &'static str {
        match self {
            EastAsianSpacingValue::Wide => "Wide",
            EastAsianSpacingValue::Narrow => "Narrow",
            EastAsianSpacingValue::Other => "Other",
            EastAsianSpacingValue::Conditional => "Conditional",
        }
    }
}
