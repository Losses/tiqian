#[derive(Debug, Clone, PartialEq)]
pub struct PlanStyleDelta {
    pub font_size: Option<f64>,
    pub font_weight: Option<u32>,
    pub italic: Option<bool>,
}
