#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineAdjustmentStrategy {
    PushInFirst,
    PushOutFirst,
    PushOutOnly,
}

pub fn compare_line_adjustment_strategy(a: &LineAdjustmentStrategy, b: &LineAdjustmentStrategy) -> i32 {
    if a == b { return 0; }
    fn rank(v: &LineAdjustmentStrategy) -> i32 {
        match v {
            LineAdjustmentStrategy::PushInFirst => 0,
            LineAdjustmentStrategy::PushOutFirst => 1,
            LineAdjustmentStrategy::PushOutOnly => 2,
        }
    }
    rank(a) - rank(b)
}

impl LineAdjustmentStrategy {
    pub fn to_string(&self) -> String {
        match self {
            LineAdjustmentStrategy::PushInFirst => "PushInFirst".to_string(),
            LineAdjustmentStrategy::PushOutFirst => "PushOutFirst".to_string(),
            LineAdjustmentStrategy::PushOutOnly => "PushOutOnly".to_string(),
        }
    }
}

impl LineAdjustmentStrategy {
    pub fn name(&self) -> &'static str {
        match self {
            LineAdjustmentStrategy::PushInFirst => "PushInFirst",
            LineAdjustmentStrategy::PushOutFirst => "PushOutFirst",
            LineAdjustmentStrategy::PushOutOnly => "PushOutOnly",
        }
    }
}
