#[derive(Clone, PartialEq)]
pub struct ValueRow {
    pub values: Vec<Option<f64>>,
}

impl ValueRow {
    pub fn new(values: Vec<Option<f64>>) -> Self {
        Self {
            values,
        }
    }
}
