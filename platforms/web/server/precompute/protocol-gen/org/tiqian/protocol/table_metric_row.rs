use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, PartialEq)]
pub struct TableMetricRow {
    pub serialized_families: UString,
    pub font_weight: f64,
    pub italic: bool,
    pub role: UString,
    pub face_selection_text: UString,
    pub values_em: Vec<Option<f64>>,
}

impl TableMetricRow {
    pub fn new(serialized_families: &UStr, font_weight: f64, italic: bool, role: &UStr, face_selection_text: &UStr, values_em: Vec<Option<f64>>) -> Self {
        Self {
            serialized_families: serialized_families.to_ustring(),
            font_weight,
            italic,
            role: role.to_ustring(),
            face_selection_text: face_selection_text.to_ustring(),
            values_em,
        }
    }
}
