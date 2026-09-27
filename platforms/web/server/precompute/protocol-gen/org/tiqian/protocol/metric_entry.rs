#[derive(Clone, PartialEq)]
pub struct MetricEntry {
    pub families_ref: u32,
    pub weight: f64,
    pub italic: u32,
    pub role_ref: u32,
    pub face_selection_ref: u32,
    pub value_pool_ref: u32,
    pub stored: bool,
}

impl MetricEntry {
    pub fn new(families_ref: u32, weight: f64, italic: u32, role_ref: u32, face_selection_ref: u32, value_pool_ref: u32) -> Self {
        Self {
            families_ref,
            weight,
            italic,
            role_ref,
            face_selection_ref,
            value_pool_ref,
            stored: false,
        }
    }
}
