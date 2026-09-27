#[derive(Clone, PartialEq)]
pub struct TableMetricRow {
    pub serialized_families: String,
    pub font_weight: f64,
    pub italic: bool,
    pub role: String,
    pub face_selection_text: String,
    pub values_em: Vec<Option<f64>>,
}

impl TableMetricRow {
    pub fn new(serialized_families: &str, font_weight: f64, italic: bool, role: &str, face_selection_text: &str, values_em: Vec<Option<f64>>) -> Self {
        Self {
            serialized_families: serialized_families.to_string(),
            font_weight,
            italic,
            role: role.to_string(),
            face_selection_text: face_selection_text.to_string(),
            values_em,
        }
    }
}
